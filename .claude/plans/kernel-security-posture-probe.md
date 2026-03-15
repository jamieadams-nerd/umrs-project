---
name: Kernel Security Posture Probe
path: components/rusty-gadgets/umrs-platform
agent: rust-developer
status: phase-2a-reviewed — security-engineer review complete (7 findings, report in .claude/reports/), tech-writer comment review done, rust-developer applied 11 comment fixes; docs deferred to post-2b; ready for Phase 2b planning
depends-on: umrs-platform-expansion.md
---

# Kernel Security Posture Probe — Implementation Plan

## Overview

Add a `posture` module to `umrs-platform` that reads, categorizes, and reports on
Linux kernel security hardening signals. The probe gives callers a typed, iterable
view of the system's runtime security posture — answering questions like "is ASLR
fully enabled?" or "are unprivileged user namespaces blocked?" — with the same
provenance-verification guarantees already established in the `kattrs` engine.

Two audiences:

1. **Experienced callers** — access individual `PostureSignal` records, inspect
   live vs. configured values, examine contradictions, build custom policy checks.
2. **Novice / intermediate callers** — call `PostureSnapshot::collect()` and iterate
   over results or ask simple yes/no questions via convenience methods.

---

## 1. Module Location and Layout

```
umrs-platform/src/
  posture/
    mod.rs              ← module doc, re-exports
    catalog.rs          ← static signal catalog (const array of SignalDescriptor)
    signal.rs           ← SignalId enum, SignalClass, AssuranceImpact, DesiredValue
    reader.rs           ← live-value reading (delegates to kattrs SecureReader engine)
    configured.rs       ← configured-value reading (sysctl.d merge, /proc/cmdline parse)
    snapshot.rs         ← PostureSnapshot: collect all signals, contradiction detection
    contradiction.rs    ← Contradiction classification types
```

New re-exports in `lib.rs`:

```rust
pub mod posture;
pub use posture::{PostureSnapshot, SignalId, SignalReport, AssuranceImpact};
```

No new workspace dependencies are anticipated. Reads from `/proc/sys/` use
`PROC_SUPER_MAGIC` via the existing `SecureReader` engine. Reads from
`/proc/cmdline` use `ProcfsText`. Reads from `/sys/` use `SYSFS_MAGIC`.

---

## 2. Signal Catalog — Static Rust, Not External JSON

### Decision

Use a **static Rust catalog** (`const` array of `SignalDescriptor` structs) rather
than an external JSON file.

### Rationale

- **Compile-time binding** — the catalog is checked by the compiler; no runtime
  deserialization, no I/O error paths, no supply-chain risk from a JSON file on disk.
  Aligns with the Compile-Time Path Binding Rule.
- **No serde dependency** — `umrs-platform` currently has no serde; adding it for a
  single static file would violate supply-chain minimality.
- **Auditability** — a reviewer can `grep` for every signal in Rust source; there is
  no hidden data file to locate.
- **The catalog is stable** — kernel sysctl semantics change very rarely. When a new
  signal is added, it is a code change that goes through review, not a data file edit
  that could be silently swapped.

### Future consideration

If the catalog grows beyond ~80 signals or if runtime extensibility is needed (e.g.,
site-local policy signals), a `build.rs` code-generation step from a checked-in TOML
or JSON source could be added later. The generated output would still be a `const`
array — the public API would not change.

---

## 3. Core Types

### 3.1 `SignalId` — Enum, Not String

```rust
/// Unique identifier for each kernel security posture signal.
///
/// Variants are ordered by catalog number for stable iteration.
/// NIST 800-53 AU-3: signal identity for audit records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignalId {
    // ── Kernel Self-Protection ──
    KptrRestrict,          // kernel.kptr_restrict
    RandomizeVaSpace,      // kernel.randomize_va_space
    UnprivBpfDisabled,     // kernel.unprivileged_bpf_disabled
    PerfEventParanoid,     // kernel.perf_event_paranoid
    YamaPtraceScope,       // kernel.yama.ptrace_scope
    DmesgRestrict,         // kernel.dmesg_restrict
    KexecLoadDisabled,     // kernel.kexec_load_disabled
    Sysrq,                 // kernel.sysrq

    // ── Kernel Integrity ──
    ModulesDisabled,       // kernel.modules_disabled  (already exists as ModuleLoadLatch)

    // ── Process Isolation ──
    UnprivUsernsClone,     // kernel.unprivileged_userns_clone

    // ── Filesystem Safety ──
    ProtectedSymlinks,     // fs.protected_symlinks
    ProtectedHardlinks,    // fs.protected_hardlinks
    ProtectedFifos,        // fs.protected_fifos
    ProtectedRegular,      // fs.protected_regular
    SuidDumpable,          // fs.suid_dumpable

    // ── Boot-time / cmdline ──
    Lockdown,              // lockdown=  (already exists as KernelLockdown)
    ModuleSigEnforce,      // module.sig_enforce
    Mitigations,           // mitigations=
    Pti,                   // pti=
    RandomTrustCpu,        // random.trust_cpu
    RandomTrustBootloader, // random.trust_bootloader

    // ── Special ──
    FipsEnabled,           // /proc/sys/crypto/fips_enabled  (already exists as ProcFips)
}
```

Design decisions (resolved):
- `Sysrq` is included with `DesiredValue::Custom` and a dedicated validator.
  Sysrq bitmask semantics are kernel-version-dependent and site-policy-dependent;
  a `Custom` handler with explicit documentation is more auditor-honest than
  encoding bitmask logic in a generic enum. The default hardened check is
  `value == 0` (fully disabled). Sites permitting restricted sysrq (e.g., 176 =
  sync + remount + reboot) can override via a policy parameter.
- `core_pattern` is deferred to Phase 2. Its "safe" value is string-based and
  distro-dependent — needs high-assurance parsing design before inclusion.
- CPU mitigations use a single umbrella `Mitigations` signal that parses
  `/proc/cmdline` once. Checks for `mitigations=off` (fail) and absence of
  weakening flags. Easiest to maintain; individual sub-signals (spectre_v2,
  pti, nosmt, tsx) can be broken out in Phase 2 if audit requires granularity.

### 3.2 `SignalClass`

```rust
/// How the signal is persisted and where its live value is read from.
///
/// NIST 800-53 CM-6: Configuration Settings — distinguishes between
/// runtime-effective and boot-persistent configuration sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalClass {
    /// Runtime sysctl: live from /proc/sys/*, configured from sysctl.d merge.
    Sysctl,
    /// Kernel command line: live from /proc/cmdline, configured from bootloader.
    KernelCmdline,
    /// Distro-managed: live from a kernel interface, configured via distro tooling.
    DistroManaged,
}
```

### 3.3 `AssuranceImpact`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssuranceImpact {
    Medium,
    High,
    Critical,
}
```

### 3.4 `DesiredValue`

```rust
/// The recommended value for a hardened system.
///
/// Expressed as a simple enum so that comparison logic is type-safe
/// and does not rely on string matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesiredValue {
    /// Exact integer match (e.g., kptr_restrict = 2).
    Exact(u32),
    /// Minimum integer threshold (e.g., perf_event_paranoid >= 2).
    AtLeast(u32),
    /// Maximum integer threshold (e.g., sysrq <= 0).
    AtMost(u32),
    /// Boolean flag present in /proc/cmdline (e.g., module.sig_enforce=1).
    CmdlinePresent(&'static str),
    /// Boolean flag must NOT be present (e.g., mitigations=off).
    CmdlineAbsent(&'static str),
    /// Special: handled by custom logic (e.g., FIPS mode cross-check).
    Custom,
}
```

### 3.5 `SignalDescriptor` — The Catalog Entry

```rust
/// Compile-time catalog entry describing one kernel security signal.
///
/// All instances are `const` and live in `catalog::SIGNALS`.
pub struct SignalDescriptor {
    pub id: SignalId,
    pub class: SignalClass,
    pub live_path: &'static str,
    pub sysctl_key: Option<&'static str>,
    pub desired: DesiredValue,
    pub impact: AssuranceImpact,
    pub rationale: &'static str,
    pub nist_controls: &'static str,
}
```

### 3.6 `SignalReport` — A Single Signal's Read Result

```rust
/// The result of reading one security posture signal.
///
/// Contains both the live (kernel) value and the configured (sysctl.d /
/// cmdline) value, plus the contradiction classification if they disagree.
///
/// NIST 800-53 AU-3, CM-6.
#[must_use = "signal reports carry security posture findings — do not discard"]
pub struct SignalReport {
    pub descriptor: &'static SignalDescriptor,
    pub live_value: Option<LiveValue>,
    pub configured_value: Option<ConfiguredValue>,
    pub meets_desired: Option<bool>,
    pub contradiction: Option<ContradictionKind>,
}
```

### 3.7 `PostureSnapshot` — The Aggregate

```rust
/// A point-in-time snapshot of all kernel security posture signals.
///
/// Constructed via `PostureSnapshot::collect()`, which reads every signal
/// in the catalog and produces a `SignalReport` for each.
///
/// Provides an iterator over reports, filtering by impact level,
/// and summary statistics.
///
/// NIST 800-53 CA-7: Continuous Monitoring — the snapshot is the
/// atomic unit of posture assessment.
#[must_use = "posture snapshots contain security findings — do not discard"]
pub struct PostureSnapshot {
    pub reports: Vec<SignalReport>,
    pub collected_at: std::time::SystemTime,
    pub boot_id: Option<String>,
}

impl PostureSnapshot {
    /// Read all signals and produce a snapshot.
    pub fn collect() -> Self { ... }

    /// Iterator over all signal reports.
    pub fn iter(&self) -> impl Iterator<Item = &SignalReport> { ... }

    /// Filter to signals that do NOT meet their desired value.
    pub fn findings(&self) -> impl Iterator<Item = &SignalReport> { ... }

    /// Filter to signals with live/configured contradictions.
    pub fn contradictions(&self) -> impl Iterator<Item = &SignalReport> { ... }

    /// Filter by minimum impact level.
    pub fn by_impact(&self, min: AssuranceImpact) -> impl Iterator<Item = &SignalReport> { ... }

    /// How many signals were successfully read.
    pub fn readable_count(&self) -> usize { ... }

    /// How many signals meet their desired hardened value.
    pub fn hardened_count(&self) -> usize { ... }
}
```

---

## 4. Reading Strategy

### 4.1 Live Values (Runtime Kernel State)

All live reads go through the existing `SecureReader` engine:

- **Sysctl signals** (`/proc/sys/*`): Each signal gets a zero-cost newtype that
  implements `KernelFileSource + StaticSource` with `EXPECTED_MAGIC = PROC_SUPER_MAGIC`.
  The parse function handles the integer-with-newline format that all sysctl nodes use.

  **Reuse opportunity**: `ProcFips` and `ModuleLoadLatch` already implement this exact
  pattern. A generic `ProcSysctl<const PATH: &str>` might reduce boilerplate, but
  const generics on `&str` are not stable. Instead, a macro
  `define_sysctl_signal!(KptrRestrict, "/proc/sys/kernel/kptr_restrict", "kernel")` can
  stamp out the trait impls. Each expansion is ~15 lines — manageable for ~15 signals.

- **Cmdline signals** (`/proc/cmdline`): Read once via `ProcfsText`, parse the full
  line, extract relevant `key=value` pairs. Cache the parsed cmdline for the duration
  of a `PostureSnapshot::collect()` call so it is not re-read per signal.

- **Securityfs signals** (`/sys/kernel/security/lockdown`): `KernelLockdown` already
  exists. The posture module reuses it directly — no duplication.

### 4.2 Configured Values (Intended State)

Configured values come from persistence sources. These are **best-effort** — if the
files cannot be read (permissions, container, etc.), the configured value is `None`
and no error is raised.

- **sysctl.d merge**: Read files from `/usr/lib/sysctl.d/`, `/run/sysctl.d/`,
  `/etc/sysctl.d/`, and `/etc/sysctl.conf` in precedence order (last writer wins,
  lexicographic within each directory). Extract key=value pairs for the signals we
  care about.

  These paths are NOT under `/proc/` or `/sys/`, so they do not use `SecureReader`.
  They are regular files on a regular filesystem. The Trust Gate Rule does not apply
  here — we are reading the *intended* configuration, not the *effective* state.
  The configured value is advisory; the live value is authoritative.

- **Kernel cmdline (configured)**: Read from bootloader config (e.g.,
  `/boot/grub2/grub.cfg` or `/boot/loader/entries/*.conf`). This is more complex
  and distro-specific. **Phase 1 will skip configured-cmdline reads** and mark
  them as `None`. This avoids scope creep and keeps the first delivery focused on
  live values and sysctl.d configured values.

### 4.3 Contradiction Detection

When both live and configured values are available:

| Live | Configured | Classification |
|------|-----------|----------------|
| Hardened | Not hardened | `EphemeralHotfix` — manual runtime override, may not survive reboot |
| Not hardened | Hardened | `BootDrift` — config says hardened but runtime disagrees |
| Unknown | Present | `SourceUnavailable` — kernel may lack feature or read failed |
| Both hardened | Both hardened | No contradiction |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContradictionKind {
    /// Live is hardened, configured is not — ephemeral hotfix.
    EphemeralHotfix,
    /// Configured is hardened, live is not — boot drift or failed application.
    BootDrift,
    /// Live value could not be read but configured value exists.
    SourceUnavailable,
}
```

---

## 5. Relationship to Existing kattrs Types

Several signals already have full `KernelFileSource + StaticSource` implementations:

| Signal | Existing Type | Reuse Strategy |
|--------|--------------|----------------|
| `FipsEnabled` | `ProcFips` | Call `ProcFips::read()` directly |
| `ModulesDisabled` | `ModuleLoadLatch` | Call `ModuleLoadLatch::read()` directly |
| `Lockdown` | `KernelLockdown` | Call `KernelLockdown::read()` directly |

New sysctl signals (kptr_restrict, dmesg_restrict, etc.) will follow the same
`KernelFileSource + StaticSource` pattern but will live in `posture/reader.rs`
(or a `posture/sysctl_types.rs` submodule) rather than `kattrs/procfs.rs`, because:

1. They are posture-specific — not general-purpose kernel attribute readers.
2. Keeping them in `posture/` makes the module self-contained.
3. They still use the `SecureReader` engine from `kattrs/traits.rs` — the
   verification path is shared, only the type definitions are localized.

**No duplication**: the new types implement the same traits and route through
the same `SecureReader::execute_read` code path.

---

## 6. Easy-to-Use Public Interface

For novice/intermediate callers who just want answers:

```rust
// One-liner: collect all signals
let snapshot = PostureSnapshot::collect();

// How many signals are hardened?
println!("{}/{} signals meet hardened baseline",
    snapshot.hardened_count(), snapshot.readable_count());

// Show only findings (signals that don't meet desired)
for report in snapshot.findings() {
    println!("{}: live={:?}, desired={:?}",
        report.descriptor.id,
        report.live_value,
        report.descriptor.desired);
}

// Check a specific signal
if let Some(report) = snapshot.get(SignalId::KptrRestrict) {
    if report.meets_desired == Some(true) {
        println!("Kernel pointer restriction: OK");
    }
}
```

For experienced callers:

```rust
// Read a single signal without collecting all
let live = KptrRestrict::read()?;  // u32 via SecureReader

// Access the static catalog
for desc in posture::catalog::SIGNALS {
    println!("{:?}: impact={:?}, desired={:?}",
        desc.id, desc.impact, desc.desired);
}

// Inspect contradictions
for report in snapshot.contradictions() {
    println!("{:?}: {:?}", report.descriptor.id, report.contradiction);
}
```

---

## 7. Logging

All signal reads emit `log::debug!()`:

```
DEBUG posture: reading signal KptrRestrict from /proc/sys/kernel/kptr_restrict
DEBUG posture: KptrRestrict live=2 desired=Exact(2) meets=true
DEBUG posture: signal UnprivUsernsClone: source unavailable (IoError: No such file or directory)
DEBUG posture: sysctl.d merge: 3 files, 47 keys extracted
DEBUG posture: PostureSnapshot collected 19/22 signals in 1.2 ms
```

---

## 8. Testing Strategy

All tests in `umrs-platform/tests/posture_tests.rs`:

1. **Catalog completeness** — every `SignalId` variant has a corresponding entry
   in `catalog::SIGNALS`. Compile-time exhaustive match.
2. **Parse unit tests** — each sysctl parser handles `"0\n"`, `"1\n"`, `"2\n"`,
   empty, non-numeric, and multi-digit values correctly.
3. **DesiredValue comparison** — `Exact`, `AtLeast`, `AtMost` logic tested with
   edge cases.
4. **Contradiction classification** — all four contradiction scenarios tested.
5. **sysctl.d merge precedence** — mock file trees with conflicting keys, verify
   last-writer-wins.
6. **Cmdline parsing** — sample `/proc/cmdline` strings with and without target
   parameters.
7. **Integration** — `PostureSnapshot::collect()` on the dev machine (will succeed
   or gracefully degrade depending on available procfs nodes).

---

## 9. Documentation

- Rust API docs on all public types (with NIST/RTB citations on module and
  security-critical items).
- Developer guide section in `docs/modules/devel/pages/` explaining the posture
  probe architecture, the dual-check model, and how to add new signals.
- Pattern documentation update: the posture probe is a concrete application of
  the Compile-Time Path Binding and Trust Gate patterns.
- An example: `examples/posture_demo.rs` — collects and prints a full snapshot.

---

## 10. Phasing

### Phase 1 — COMPLETE
- `SignalId` enum, `SignalDescriptor`, static catalog
- Live-value readers for all sysctl signals (via `SecureReader`)
- `/proc/cmdline` parser for boot-time signals
- `PostureSnapshot::collect()` with iterator interface
- Configured-value reading for sysctl.d (merge logic)
- Contradiction detection (live vs. configured sysctl only)
- Tests (60/60 pass), example (`posture_demo.rs`), lib.rs re-exports
- Developer guide documentation: pending

### Phase 2a — modprobe.d Cross-Check + FIPS Distro-Managed Cross-Check

**Goal**: Extend the posture probe with two new configured-value sources that
follow the same merge-tree and contradiction-detection patterns proven in Phase 1.
Both features strengthen the configured-vs-live comparison model and give the
security-engineer new trust boundary surface to audit.

**Checkpoint**: After 2a is complete and tests pass, save progress and hand off
to `security-engineer` for a full review of the Phase 1 + 2a posture module.

---

#### 2a.1 — modprobe.d Configured-Value Cross-Check

**What**: Read kernel module parameter configuration from the modprobe.d merge
tree and cross-check against live values from `/sys/module/*/parameters/`.

**Why**: An adversary who modifies `/etc/modprobe.d/` can alter module behaviour
at next load without changing the running kernel — a persistence vector that
escapes live-only monitoring. Conversely, a runtime `sysfs` write can override
configured parameters — an ephemeral change that escapes config-only auditing.
The same contradiction-detection model that Phase 1 uses for sysctl.d applies.

##### New Files

```
posture/
  modprobe.rs    ← modprobe.d merge-tree reader and parameter cross-check
```

##### modprobe.d Merge Tree — Precedence Order

Follows `modprobe.d(5)` precedence (last-writer-wins, lexicographic within dir):

1. `/usr/lib/modprobe.d/*.conf` — distro/vendor defaults (lowest)
2. `/run/modprobe.d/*.conf` — transient overrides
3. `/etc/modprobe.d/*.conf` — admin overrides (highest)

##### Line Format (read-only — this probe reads config files, never modifies them)

modprobe.d config files use several directive types. The probe reads and
parses these to determine the **intended** module configuration:

```
options <module_name> <param1>=<value1> [<param2>=<value2> ...]
blacklist <module_name>                  # soft blacklist
install <module_name> /bin/true          # sysadmin blacklist technique (see below)
```

**Phase 2a** parses `options` and `blacklist` directives — both have
well-defined, deterministic formats.

**Phase 2b** adds parsing for `install` directives. In modprobe.d, an
`install` line does not install software — it is a redirection: when the
kernel requests a module load, `modprobe` executes the specified command
**instead of** loading the module. Sysadmins commonly write
`install usb_storage /bin/true` to prevent a module from loading — the
kernel's load request is silently swallowed by `/bin/true`. Detecting
whether an `install` directive is a blacklist-equivalent requires parsing
the command string (`/bin/true`, `/bin/false`, `/sbin/modprobe --ignore-install`, etc.),
which is string analysis deferred to 2b for high-assurance parsing design.

##### Live Value Source

`/sys/module/<module_name>/parameters/<param>` — readable via `SysfsText`
with `SYSFS_MAGIC` provenance verification.

##### Applicable High-Assurance Patterns

| Pattern | Application | Control |
|---------|-------------|---------|
| **Compile-Time Path Binding** | Module parameter paths bound as associated constants on newtype; `SYSFS_MAGIC` verified at read time. No runtime path construction from user input. | NSA RTB RAIN |
| **Provenance Verification** | `/sys/module/` reads go through `SecureReader` with `fstatfs` against `SYSFS_MAGIC`. `/etc/modprobe.d/` reads are regular-filesystem best-effort (advisory, not authoritative). | NIST 800-53 SI-7 |
| **Trust Gate** | Only read `/sys/module/<mod>/parameters/` if the module is currently loaded (check `/sys/module/<mod>/` existence first). If the module is absent, the live value is `None` — do not guess from config. | NIST 800-53 CM-6 |
| **Security Findings as Data** | Module parameter contradictions are `ContradictionKind` enum variants, not log strings. Callers match, filter, count programmatically. | NIST 800-53 AU-3 |
| **Fail-Closed Parsing** | Lines that fail to parse are rejected and logged — never silently skipped. Unrecognised directives (e.g., `softdep`) are logged at `debug` and excluded from the parameter map rather than causing errors. | NIST 800-53 SI-10 |
| **Layered Separation** | The merge-tree reader (`modprobe.rs`) is a data-collection layer. It does not format, display, or make remediation decisions. Presentation layers consume `SignalReport` downstream. | NSA RTB / NIST 800-53 SC-3 |
| **Pattern Execution Measurement** | `#[cfg(debug_assertions)]` timing on merge-tree load, per-module parameter read, and cross-check comparison. `log::debug!("posture: modprobe.d merge completed in {} µs: {} files, {} modules", ...)` | NIST 800-218 SSDF PW.4 |
| **Must-Use Contract** | All public functions returning `Result` or `Option` carry `#[must_use = "..."]` with an explanation. `ModprobeConfig` type carries `#[must_use]` at the type level. | NIST 800-53 SI-10, SA-11 |
| **Validate at Construction** | `ModprobeConfig::load()` returns a validated, immutable config. Downstream code receives the type, not a `Result` — the error was handled at the boundary. | NIST 800-218 SSDF PW.4.1 |

##### Signals to Add

Phase 2a adds cross-check for **security-critical module parameters only** —
modules whose parameter configuration has a direct security impact:

| Module | Parameter | Desired | Rationale | Impact |
|--------|-----------|---------|-----------|--------|
| `nf_conntrack` | `acct` | `1` | Connection tracking accounting for audit trails | Medium |
| `bluetooth` | (blacklisted) | blacklisted | Bluetooth stack is an attack surface on servers | High |
| `usb_storage` | (blacklisted) | blacklisted | USB mass storage is a data exfiltration vector | High |
| `firewire_core` | (blacklisted) | blacklisted | FireWire DMA attacks bypass memory protection | High |
| `thunderbolt` | (blacklisted) | blacklisted | Thunderbolt DMA attacks bypass memory protection | High |

Blacklist signals check both `blacklist <mod>` entries and `install <mod> /bin/true`
or `/bin/false` patterns in the modprobe.d merge tree.

##### New Types

```rust
/// Merged modprobe.d configuration.
///
/// NIST 800-53 CM-6: modprobe.d persistence layer for module parameter
/// and blacklist state.
#[must_use = "modprobe config carries module parameter findings — do not discard"]
pub struct ModprobeConfig {
    /// module_name → { param_name → (value, source_file) }
    options: HashMap<String, HashMap<String, (String, String)>>,
    /// module_name → source_file (blacklisted modules)
    blacklisted: HashMap<String, String>,
}
```

##### Debug Logging Requirements

Every significant operation must emit `log::debug!()`:

```
DEBUG posture: modprobe.d merge: scanning /usr/lib/modprobe.d/ (12 files)
DEBUG posture: modprobe.d merge: /etc/modprobe.d/blacklist-bluetooth.conf:3 blacklist bluetooth
DEBUG posture: modprobe.d merge: /etc/modprobe.d/nf_conntrack.conf:1 options nf_conntrack acct=1
DEBUG posture: modprobe.d merge: completed in 340 µs: 18 files, 4 modules, 2 blacklisted
DEBUG posture: modprobe cross-check: bluetooth blacklisted=true, /sys/module/bluetooth absent → live confirms blacklist
DEBUG posture: modprobe cross-check: nf_conntrack acct: configured=1, live=1 → PASS
DEBUG posture: modprobe cross-check: usb_storage blacklisted=true, /sys/module/usb_storage present → CONTRADICTION: BootDrift
```

##### Evidence Chain

`SignalReport` for modprobe signals must capture:
- `configured_value`: the raw value from the highest-precedence modprobe.d file, with source file path
- `live_value`: the value from `/sys/module/<mod>/parameters/<param>`, or `Bool(false)` for blacklist checks where the module is loaded
- `contradiction`: `EphemeralHotfix` (module unloaded at runtime but config allows it), `BootDrift` (config blacklists but module is loaded), `SourceUnavailable` (module absent, cannot verify parameters)

##### Testing Strategy

All tests in `umrs-platform/tests/posture_modprobe_tests.rs`:

1. **Line parser unit tests** — `options`, `blacklist`, comments, malformed lines
2. **Merge precedence** — later directories override earlier ones
3. **Live cross-check** — mock-free integration against `/sys/module/` (degrade gracefully if modules absent)
4. **Contradiction classification** — all scenarios: blacklisted+loaded, blacklisted+absent, options match, options mismatch
5. **ModprobeConfig construction** — validate-at-construction guarantees
6. **Integration** — modprobe signals appear in `PostureSnapshot::collect()`

---

#### 2a.2 — FIPS Distro-Managed Cross-Check

**What**: Extend the existing `FipsEnabled` signal with configured-value discovery
from RHEL 10's FIPS persistence layer, enabling contradiction detection between
the kernel's live FIPS state and the distro's intended FIPS configuration.

**Why**: FIPS mode on RHEL is a system-wide posture decision with three sources
of truth that can disagree:

1. **Kernel live**: `/proc/sys/crypto/fips_enabled` (already read by `ProcFips`)
2. **Dracut initramfs**: `fips=1` on the kernel cmdline (already checked by Phase 1's cmdline parser)
3. **Distro persistent state**: `/etc/system-fips` (marker file) and the output of `fips-mode-setup --check`

If (1) says enabled but (3) says not configured, the system is in an ephemeral
FIPS state that will not survive a dracut rebuild. If (3) says configured but
(1) says disabled, FIPS was intended but is not active — a compliance gap.

##### RHEL 10 FIPS State Discovery

On RHEL 10, FIPS persistent state is determined by:

- **Marker file**: `/etc/system-fips` — presence indicates FIPS was configured via
  `fips-mode-setup --enable`. This is a simple existence check, not a content parse.
- **Kernel cmdline**: `fips=1` in `/proc/cmdline` — already covered by Phase 1's
  cmdline parser. Phase 2a cross-references this.
- **Crypto policy**: `/etc/crypto-policies/state/current` — should read `FIPS` or
  `FIPS:*` when FIPS is configured. This is a secondary indicator.

##### Applicable High-Assurance Patterns

| Pattern | Application | Control |
|---------|-------------|---------|
| **Trust Gate** | Only read `/etc/system-fips` and crypto-policy state if `/proc/sys/crypto/fips_enabled` is accessible. If the kernel cannot confirm the crypto subsystem state, config reads are meaningless. | NIST 800-53 CM-6 |
| **Provenance Verification** | `/proc/sys/crypto/fips_enabled` already verified via `PROC_SUPER_MAGIC` (Phase 1). `/etc/system-fips` and `/etc/crypto-policies/` are regular-filesystem reads — best-effort, not authoritative. | NIST 800-53 SI-7 |
| **Fail-Closed Parsing** | If `/etc/system-fips` is absent, the configured state is `None` (not "FIPS disabled"). If crypto-policy file is unreadable, degrade to `None` — never assume a default. | NIST 800-53 SI-10 / RTB Fail Secure |
| **Security Findings as Data** | FIPS contradiction is `ContradictionKind::BootDrift` (configured but not active) or `ContradictionKind::EphemeralHotfix` (active but not configured persistently). Programmatically matchable. | NIST 800-53 AU-3 |
| **Error Information Discipline** | FIPS state errors must not reveal the specific crypto-policy string in error messages — it could indicate the system's cryptographic posture to an adversary. Log at `debug`, not `warn`. | NIST 800-53 SI-11 / RTB Error Discipline |
| **Pattern Execution Measurement** | `#[cfg(debug_assertions)]` timing on FIPS cross-check. | NIST 800-218 SSDF PW.4 |
| **Must-Use Contract** | `FipsCrossCheck::evaluate()` returns a `#[must_use]` result. | NIST 800-53 SI-10, SA-11 |
| **Non-Bypassability** | The cross-check is invoked unconditionally during `PostureSnapshot::collect()` — there is no code path that skips it. The `collect_one` dispatch for `SignalId::FipsEnabled` calls the cross-check as part of configured-value resolution. | NSA RTB RAIN |

##### New Types

```rust
/// FIPS persistent configuration state from RHEL distro tooling.
///
/// Aggregates multiple FIPS configuration indicators into a single
/// typed assessment. Each indicator is independently resolved and
/// recorded for audit evidence.
///
/// NIST 800-53 SC-13: Cryptographic Protection — FIPS configuration
/// state determines which cryptographic modules are permitted.
/// FIPS 140-2/140-3: system-wide FIPS mode enforcement.
#[must_use = "FIPS cross-check results carry compliance findings — do not discard"]
pub struct FipsCrossCheck {
    /// Presence of `/etc/system-fips` marker file.
    pub marker_present: Option<bool>,
    /// `fips=1` found in `/proc/cmdline` (from Phase 1 CmdlineReader).
    pub cmdline_fips: Option<bool>,
    /// Content of `/etc/crypto-policies/state/current`.
    pub crypto_policy: Option<String>,
    /// Overall assessment: configured FIPS state agrees with live kernel state.
    pub configured_meets_desired: Option<bool>,
}
```

##### Integration with Existing Posture Architecture

The `FipsCrossCheck` is not a new signal — it enhances the existing
`SignalId::FipsEnabled` by providing a `configured_value` that was `None`
in Phase 1. The enhancement flows through the existing `read_configured()`
dispatch in `snapshot.rs`:

- `read_configured()` for `SignalClass::DistroManaged` + `SignalId::FipsEnabled`
  now invokes `FipsCrossCheck::evaluate()` instead of returning `None`.
- The `ConfiguredValue.raw` field contains a structured summary (e.g.,
  `"marker=present cmdline=fips=1 policy=FIPS"`) for audit display.
- The `ConfiguredValue.source_file` field records whichever indicator was
  the primary source (e.g., `/etc/system-fips`).
- Contradiction detection uses the existing `classify()` function — no changes
  to the contradiction engine.

##### Debug Logging Requirements

```
DEBUG posture: FIPS cross-check: /etc/system-fips exists=true
DEBUG posture: FIPS cross-check: /proc/cmdline fips=1 present=true
DEBUG posture: FIPS cross-check: /etc/crypto-policies/state/current = "FIPS"
DEBUG posture: FIPS cross-check: completed in 95 µs — marker=true cmdline=true policy=FIPS
DEBUG posture: FIPS cross-check: configured_meets_desired=true (all indicators agree)
```

Or on a system with a contradiction:

```
DEBUG posture: FIPS cross-check: /etc/system-fips exists=false
DEBUG posture: FIPS cross-check: /proc/cmdline fips=1 present=false
DEBUG posture: FIPS cross-check: live /proc/sys/crypto/fips_enabled=1
DEBUG posture: FIPS cross-check: CONTRADICTION — live FIPS active but not persistently configured
DEBUG posture: FIPS cross-check: classified as EphemeralHotfix
```

##### Testing Strategy

All tests in `umrs-platform/tests/posture_fips_tests.rs`:

1. **Marker file detection** — present, absent, unreadable (permissions)
2. **Crypto-policy parsing** — `FIPS`, `FIPS:OSPP`, `DEFAULT`, empty, absent
3. **Cross-check evaluation** — all contradiction scenarios
4. **Integration** — `FipsEnabled` signal in snapshot now has `configured_value`
5. **Trust gate** — if `/proc/sys/crypto/fips_enabled` is unreadable, configured
   checks return `None` (not a false positive)

---

### Model Assignments

| Phase / Work Item | Agent | Model | Rationale |
|---|---|---|---|
| Phase 2b — bootloader cmdline | rust-developer | **sonnet** | Follows established file-parsing patterns from Phase 1 |
| Phase 2b — CPU mitigation sub-signals | rust-developer | **opus** | Catalog schema change, ripples through exhaustive matches and tests |
| Phase 2b — core_pattern | rust-developer | **opus** | TPI candidate, high-assurance string parsing design |
| Phase 2b — SEC caching | rust-developer | **opus** | TTL + invalidation design, HMAC integration |
| Phase 2b — modprobe install directive | rust-developer | **sonnet** | Extends established 2a patterns |
| Phase 2b — security review | security-engineer | **opus** | Full posture module review, trust boundary analysis |
| Phase 2c — documentation | tech-writer | **sonnet** | Developer guide from established API |
| Phase 3 — CPU extension detection | rust-developer | **opus** | New signal classes, three-layer model, major scope |

---

### Phase 2b (future — after security-engineer review of 2a)
- Configured-value reading for **bootloader cmdline** (grub2, systemd-boot, BLS entries)
- **CPU mitigation sub-signals** (break umbrella `Mitigations` into spectre_v2, mds, tsx, etc.)
- **core_pattern** signal (requires high-assurance string parsing design — TPI candidate)
- **SEC caching** for posture signals (short TTL + invalidation design)
- **modprobe.d `install` directive** read-only parsing — detect blacklist-equivalent
  `install <module> /bin/true` patterns by analysing the command string (not executing it)

### Phase 3 (future — from expansion plan)
- CPU extension detection (security/crypto extensions)
- ELF binary audit for extension linkage
- Integration with `DetectionResult` and SEC cache

---

## 11. Resolved Decisions

1. **Macro + hand-written reference**: Use a declarative macro to stamp out sysctl
   signal types (~3 lines each), with one fully hand-written example (`KptrRestrict`)
   for auditor reference. All expansions route through existing `SecureReader` and
   `KernelFileSource + StaticSource` traits — no new read paths.

2. **SEC caching**: Deferred to Phase 2b. Posture signals can change at runtime
   (sysctl writes), so caching needs a short TTL and careful invalidation design.

3. **No dependency on `detect`**: `PostureSnapshot::collect()` reads `boot_id`
   independently via `ProcfsText` — lightweight, no coupling to the detection pipeline.

4. **Sysrq**: `DesiredValue::Custom` with a dedicated validator. Default hardened
   check is `value == 0`. Explicit, auditor-friendly, site-policy-overridable.

5. **core_pattern**: Deferred to Phase 2b. Requires high-assurance string parsing
   design before inclusion. Candidate for TPI dual-path parsing.

6. **CPU mitigations**: Single umbrella signal parsing `/proc/cmdline`. Individual
   sub-signals deferred to Phase 2b — requires catalog schema change that ripples
   through exhaustive matches and tests.

7. **Phase 1 scope**: Live values + sysctl.d configured values + contradiction
   detection. Bootloader cmdline configured values deferred to Phase 2b.

8. **Phase 2a/2b split rationale** (2026-03-14): Phase 2a (modprobe.d + FIPS
   cross-check) builds on proven Phase 1 patterns — merge-tree reading,
   contradiction detection, existing `SignalClass::DistroManaged`. Phase 2b
   items (bootloader cmdline, CPU sub-signals, core_pattern, SEC caching)
   introduce new architectural patterns or require catalog schema changes.
   The split minimises risk for the developer and creates a clean checkpoint
   for security-engineer review before expanding the attack surface further.

---

## DO NOT CHANGE ANY CODE RIGHT NOW

This is a design document for review and discussion. No implementation until approved.
