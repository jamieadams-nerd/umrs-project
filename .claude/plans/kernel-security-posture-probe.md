---
name: Kernel Security Posture Probe
path: components/rusty-gadgets/umrs-platform
agent: rust-developer
status: approved — decisions locked, ready for implementation
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

### Phase 1 (this plan)
- `SignalId` enum, `SignalDescriptor`, static catalog
- Live-value readers for all sysctl signals (via `SecureReader`)
- `/proc/cmdline` parser for boot-time signals
- `PostureSnapshot::collect()` with iterator interface
- Configured-value reading for sysctl.d (merge logic)
- Contradiction detection (live vs. configured sysctl only)
- Tests, example, docs

### Phase 2 (future)
- Configured-value reading for bootloader cmdline
- `modprobe.d` parameter cross-check
- FIPS distro-managed cross-check (fips-mode-setup state)
- CPU mitigation sub-signals (individual spectre/meltdown knobs)

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

2. **SEC caching**: Deferred to Phase 2. Posture signals can change at runtime
   (sysctl writes), so caching needs a short TTL and careful invalidation design.

3. **No dependency on `detect`**: `PostureSnapshot::collect()` reads `boot_id`
   independently via `ProcfsText` — lightweight, no coupling to the detection pipeline.

4. **Sysrq**: `DesiredValue::Custom` with a dedicated validator. Default hardened
   check is `value == 0`. Explicit, auditor-friendly, site-policy-overridable.

5. **core_pattern**: Deferred to Phase 2. Requires high-assurance string parsing
   design before inclusion.

6. **CPU mitigations**: Single umbrella signal parsing `/proc/cmdline`. Individual
   sub-signals deferred to Phase 2 if audit requires granularity.

7. **Phase 1 scope**: Live values + sysctl.d configured values + contradiction
   detection. Bootloader cmdline configured values deferred to Phase 2.

---

## DO NOT CHANGE ANY CODE RIGHT NOW

This is a design document for review and discussion. No implementation until approved.
