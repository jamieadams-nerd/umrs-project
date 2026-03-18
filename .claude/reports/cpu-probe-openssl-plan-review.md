# Plan Review: CPU Extension Probe + OpenSSL Posture Module

**Date:** 2026-03-18
**Reviewer:** rust-developer (Sonnet 4.6)
**Plans reviewed:**
- `.claude/plans/cpu-extension-probe.md`
- `.claude/plans/openssl-posture-module.md`
**Reference material:** `.claude/agent-memory/rust-developer/cpu_access_controls_familiarization.md`

---

## Part 1 — CPU Extension Probe Review

### 1.1 The Vulnerability Sysfs Layer Is Missing From the Model (Critical Gap)

The plan describes a three-layer activation model (Hardware / OS / Software). The corpus
research reveals a fourth orthogonal layer that the plan does not mention: **runtime
mitigation state** as reported by
`/sys/devices/system/cpu/vulnerabilities/<filename>`.

This layer is not part of the three-layer activation model. It answers a different question:
given that the CPU has certain microarchitectural vulnerabilities, what mitigations are
the kernel and microcode currently applying at runtime?

The existing posture `Mitigations` indicator reads `/proc/cmdline` for the umbrella
`mitigations=` switch. That is a configuration intent check — it tells you whether the
user disabled all mitigations in aggregate. The vulnerability sysfs files tell you whether
each individual mitigation is actually running. These are not the same question.

**What is missing:**

The plan does not describe a `VulnerabilityReport` type or a vulnerability sysfs reader.
The corpus material explicitly proposes both, and the familiarization notes document 14+
sysfs files to read, including post-2022 CVEs that have no existing `IndicatorId` coverage:

- `gather_data_sampling` (Downfall, CVE-2022-40982)
- `reg_file_data_sampling` (RFDS, CVE-2023-28746)
- `mmio_stale_data` (CVE-2022-21123 series)
- `itlb_multihit` (CVE-2018-12207)
- `spectre_v1` (sysfs route, distinct from the cmdline-off indicator)
- `spectre_bhb` (ARM-only)

**Recommendation:** Add a "Layer 4 — Runtime Mitigation State" section to the plan and
expand the Definition of Done to include `VulnerabilityReport`. This is not a luxury
addition — it closes the gap between "mitigations are configured" and "mitigations are
active". An auditor will ask for the latter.

**Proposed type design (for the plan to capture):**

```rust
/// Per-vulnerability runtime mitigation state read from
/// `/sys/devices/system/cpu/vulnerabilities/<name>`.
///
/// NIST SP 800-53 SI-2: Flaw Remediation — runtime state is the authoritative
/// record of whether a known flaw is mitigated.
pub enum VulnerabilityStatus {
    NotAffected,
    Mitigated(String),  // free-form kernel description after "Mitigation: "
    Vulnerable(String), // free-form kernel description after "Vulnerable: "
    Unknown,            // read failure or parse mismatch; fail-closed
}

/// Structured per-vulnerability field rather than Option<VulnerabilityStatus>
/// for each entry so "file not present" (older kernel) is distinguishable from
/// `Unknown` (read failure). Use Option<VulnerabilityStatus> per field.
pub struct VulnerabilityReport {
    pub spectre_v1:              Option<VulnerabilityStatus>,
    pub spectre_v2:              Option<VulnerabilityStatus>,
    pub spec_store_bypass:       Option<VulnerabilityStatus>,
    pub mds:                     Option<VulnerabilityStatus>,
    pub tsx_async_abort:         Option<VulnerabilityStatus>,
    pub itlb_multihit:           Option<VulnerabilityStatus>,
    pub mmio_stale_data:         Option<VulnerabilityStatus>,
    pub retbleed:                Option<VulnerabilityStatus>,
    pub srbds:                   Option<VulnerabilityStatus>,
    pub gather_data_sampling:    Option<VulnerabilityStatus>,
    pub reg_file_data_sampling:  Option<VulnerabilityStatus>,
    pub spectre_bhb:             Option<VulnerabilityStatus>, // ARM only
}
```

All reads use `SysfsText` + `SecureReader::read_generic_text` — mandatory per the TOCTOU
routing rule. No new routing infrastructure is needed; the existing `SysfsText` type covers
dynamic `/sys/` paths exactly.

**TPI note:** The sysfs text prefix-matching parse (Not affected / Vulnerable / Mitigation:)
is simple enough to be a single linear parse. TPI does not apply here.

**Spectre v2 structured detail (optional follow-up):** The kernel emits semicolon-separated
sub-components (IBPB status, STIBP mode, RSB filling, BHI). A `Spectre2Detail` struct
enabling programmatic queries ("is IBPB conditional or disabled?") would be high value but
can be deferred to a follow-up pass. Start with `Mitigated(String)`.

**Where does `VulnerabilityReport` live?** The plan assumes the CPU probe lives in
`umrs-platform`. The vulnerability sysfs data is orthogonal to both the existing posture
module (which checks cmdline configuration) and the CPU extension probe (which checks
hardware capability). It belongs in `umrs-platform` but as its own submodule
`posture::vulnerability` — not inside the CPU extension probe. The probe and the
vulnerability reader are parallel subsystems that report upward to a combined
`CpuPostureSnapshot`.

### 1.2 CET Binary Verification Is a Layer 2 Signal — Not Captured in the Plan

The plan defines Layer 2 as "OS Enablement" (kernel state management). The corpus material
distinguishes a second meaning of Layer 2: **software utilization evidence** at the binary
level — specifically, whether compiled binaries carry ELF `.note.gnu.property` flags
indicating CET-SS (`GNU_PROPERTY_X86_FEATURE_1_SHSTK`, bit 1) and CET-IBT
(`GNU_PROPERTY_X86_FEATURE_1_IBT`, bit 0).

This is binary analysis, not kernel state. On ARM, the same ELF `.note.gnu.property`
mechanism carries PAC and BTI flags.

The current plan mentions "readelf/elfdump binary hardening evidence integrated" in the
Definition of Done checklist, but there is no discussion of the type design, the parsing
approach, or the fact that this requires a new external dependency (`goblin`). This is
insufficient for a plan that is meant to be implementation-ready.

**Critical finding — Rust CET limitation:** Stable Rust does not support
`-Z cf-protection=full` (rust-lang/rust#93754). UMRS Rust binaries will NOT carry
`GNU_PROPERTY_X86_FEATURE_1_SHSTK` or `GNU_PROPERTY_X86_FEATURE_1_IBT` notes. On CET-capable
hardware (Tiger Lake+, Zen 3+) with RHEL 10, UMRS processes will not activate CET shadow
stack. The dynamic linker AND-gates all library properties — a single non-CET binary in
the process disables CET for the whole process.

**Audit classification for UMRS binaries missing CET notes:** INFORMATIONAL. Rust's
ownership model and `#![forbid(unsafe_code)]` substantially reduce the attack surface
that CET defends. This must NOT be classified HIGH (that severity applies to C/C++ binaries).
The plan should document this distinction explicitly so the operator-facing output carries
the correct severity.

**Same classification applies to ARM:** UMRS Rust binaries will not carry PAC/BTI ELF notes
on aarch64. INFORMATIONAL, same rationale.

**What the plan needs:** An explicit type design for binary hardening evidence, a note on
the Rust CET limitation, and a decision on whether `goblin` is the parsing mechanism.
See the shared `ElfInspector` design proposal in Section 3.

### 1.3 `CpuSignalId` Design Needs Expansion

The plan defers the `CpuSignalId` design to "after corpus research is complete." The corpus
research is now complete. The plan should be updated with the full variant list.

**Proposed additions to `CpuSignalId` (from the Phase 1E/1F material):**

| Variant | `/proc/cpuinfo` flag | Layer | Platform |
|---|---|---|---|
| `CpuSignalId::Pcid` | `pcid` | 1 — hardware | x86_64 |
| `CpuSignalId::CetShadowStack` | `shstk` | 1 — hardware | x86_64 |
| `CpuSignalId::CetIbt` | `ibt` | 1 — hardware | x86_64 |
| `CpuSignalId::Umip` | `umip` | 1 — hardware | x86_64 |
| `CpuSignalId::Pku` | `pku` + `ospke` | 1 — hardware | x86_64 |
| `CpuSignalId::ArmPac` | `paca` (or `pacg`) | 1 — hardware | aarch64 |
| `CpuSignalId::ArmBti` | `bti` | 1 — hardware | aarch64 |
| `CpuSignalId::ArmMte` | `mte` / `mte2` / `mte3` | 1 — hardware | aarch64 |

All of these are `/proc/cpuinfo` reads — `ProcfsText` + `SecureReader::read_generic_text`.
No new routing infrastructure needed.

**PCID cross-reference with existing `Pti` indicator:** This is not just an informational
signal. The corpus establishes a direct dependency: PTI without PCID causes 30-50%
performance regression, creating operational pressure to disable PTI (i.e., remove the
Meltdown mitigation). The contradiction detector should be extended:

- `Pcid` absent + `Pti` active → CAUTION (performance risk that may lead to future
  disablement of a security mitigation)
- `Pcid` present + `Pti` disabled (existing `Pti` indicator flags this) → CRITICAL
  (Meltdown mitigation disabled despite hardware support for the perf fix)

The existing `IndicatorId::Pti` and the new `CpuSignalId::Pcid` are cross-module signals.
The contradiction detector already handles cross-signal logic — this is an extension of
that established pattern.

**PKU — low implementation priority:** PKU hardware detection is INFORMATIONAL. The
`pkey_alloc` / `pkey_mprotect` syscall path for key material isolation is an interesting
future direction but conflicts with `#![forbid(unsafe_code)]` unless done via a safe syscall
abstraction (the WRPKRU register write requires inline asm). Flag as an advisory to Jamie;
do not block the CPU probe on it.

### 1.4 ARM Detection Path — Inadequately Specified

The plan mentions ARM CPU flags in the extension table but does not describe an ARM-specific
detection path. The corpus material establishes several ARM-specific considerations:

**PAC vs CET-SS — different mechanisms:** ARM PAC signs return addresses cryptographically
in unused upper pointer bits. x86 CET-SS maintains a separate shadow stack page. These are
not equivalent implementations of the same primitive — they have different threat models.
The PACMAN attack (2022, Apple M1) demonstrates that speculative execution can probe PAC
values without triggering faults. The fix is combining PAC with BTI. UMRS should not
conflate PAC with CET-SS in operator output.

**Detection for ARM Layer 1:** `/proc/cpuinfo` flags on aarch64 — same infrastructure as
x86_64. The `ProcfsText` routing is platform-agnostic.

**Detection for ARM Layer 2 (binary):** ELF `.note.gnu.property` — same `goblin` path as
x86. The property note bit meanings differ per architecture but the parsing infrastructure
is identical.

**Vulnerability sysfs on ARM:** `/sys/devices/system/cpu/vulnerabilities/` is populated on
ARM too. Applicable files: `spectre_v1`, `spectre_v2`, `spec_store_bypass`, `meltdown`,
`spectre_bhb` (ARM-specific). `VulnerabilityReport` should make `spectre_bhb` an
`Option<VulnerabilityStatus>` field (absent on x86).

**MTE — architecturally novel:** ARM MTE provides probabilistic hardware memory safety
(4-bit tag, 1/16 collision probability) with no x86 equivalent. Rust code benefits
minimally (compile-time safety already enforces this), but C library dependencies (glibc,
OpenSSL) benefit from MTE when deployed on MTE-capable hardware. UMRS should note MTE
presence as an INFORMATIONAL capability with context explaining what it protects.

**Recommendation:** Add an "ARM Detection Path" subsection to the plan covering:
- `/proc/cpuinfo` flag names for all ARM signals
- Compiler flag mapping (`-mbranch-protection=standard` for PAC+BTI on GCC/Clang)
- PACMAN attack context (PAC alone is not sufficient; PAC+BTI is the correct combination)
- MTE on ARM as a defense-in-depth signal for C library dependencies
- ARM vulnerability sysfs file inventory

### 1.5 Existing Infrastructure Reuse — Fully Available

The plan references `SecureReader` and sysfs/procfs routing but does not inventory what is
already available. For the implementer, the relevant existing infrastructure is:

| Infrastructure | Module | Reuse for CPU probe |
|---|---|---|
| `ProcfsText` + `SecureReader::read_generic_text` | `kattrs::procfs` | `/proc/cpuinfo` reads (Layer 1) |
| `SysfsText` + `SecureReader::read_generic_text` | `kattrs::sysfs` | Vulnerability sysfs reads (Layer 4) |
| `EvidenceBundle` | `evidence` | Provenance trail for all probe results |
| `SourceKind::Procfs` / `SourceKind::SysfsNode` | `evidence` | Evidence record classification |
| `AssuranceImpact` | `posture::indicator` | Severity tier for CPU signal findings |

No new routing infrastructure is needed. The pattern established in `posture::reader.rs`
(hand-written reference + declarative macro for repetitive sysctl signals) applies here:
one hand-written `CpuSignalId::CetShadowStack` reader as auditor reference, then a
`define_cpuinfo_signal!` macro for the remaining flag-check signals.

### 1.6 Where Does `VulnerabilityReport` Live?

**Recommendation:** Not in the CPU extension probe module. The vulnerability sysfs data
is about microarchitectural flaw remediation, not about extension availability or software
utilization. It belongs in `umrs-platform` as `posture::vulnerability` — a sibling to the
existing `posture::bootcmdline` and `posture::reader` modules.

The CPU extension probe module holds extension availability signals (`CpuSignalId` variants).
The vulnerability module holds remediation state (`VulnerabilityReport`). A higher-level
`CpuPostureSnapshot` type aggregates both. This separation keeps each module's concern clear
and allows each to have independent test coverage.

---

## Part 2 — OpenSSL Posture Module Review

### 2.1 Phased Approach — Consolidation Opportunity

The five-phase breakdown is reasonable. One consolidation opportunity:

**Phases 1 and 3 can be partially merged at design time (not necessarily at implementation
time).** Phase 1 (OpenSSL identity + provider model) and Phase 3 (hardware acceleration
cross-reference) both need `/proc/cpuinfo` flags and the `OPENSSL_armcap`/`OPENSSL_ia32cap`
bitmasks. Designing `HwAccelStatus` during Phase 1 avoids retrofitting the `OpenSslPosture`
type when Phase 3 is added. The types can be designed together even if implementation is
sequential.

**No phases should be eliminated.** The existing split correctly separates identity (what is
installed) from algorithm policy (what algorithms are available) from hardware evidence
(are hardware paths active) from binary analysis (does this binary use system OpenSSL).
These are genuinely independent layers with different data sources.

### 2.2 Layer A — OpenSSL Identity Detection Path

The plan says "parse from ELF `.rodata` or config files." This is the most under-specified
section of the plan and needs clarification.

**Is ELF `.rodata` parsing feasible via `goblin`?**

Yes, but with caveats. `goblin` can parse ELF sections. The OpenSSL version string is
embedded as a `OPENSSL_VERSION_TEXT` symbol in `libcrypto.so`. It is accessible via:
- Symbol table lookup: `OPENSSL_version(OPENSSL_VERSION_STRING)` is the C API, but we
  are not calling the library directly
- The `OPENSSL_version` string is embedded in `.rodata` as a null-terminated literal
- Its format is: `"OpenSSL 3.0.13 30 Jan 2024"` — parseable with a simple `nom` rule

**However, there is a simpler and more reliable path:** The `openssl.cnf` file (at
`OPENSSLDIR/openssl.cnf`) contains the config root. The version is most reliably extracted
from the library's symbol table, not `.rodata` scanning (which requires linear search through
potentially large sections).

**Actual recommended detection path for Phase 1:**
1. Locate `libcrypto.so.3` via `/usr/lib64/libcrypto.so.3` (RHEL 10 standard path) or
   by scanning `ldconfig` cache files (read-only, no subprocess).
2. Use `goblin` to load the ELF and enumerate `.rodata`/symbol table for version string.
3. Alternatively — and this may be more robust — read `/usr/lib64/pkgconfig/libcrypto.pc`
   or `/usr/lib64/pkgconfig/openssl.pc` for version. These are plain-text files with no
   binary parsing required.

**Plan gap:** Specify the detection path more concretely. The implementer should not
discover it during implementation. A `pkg-config`-based fallback is worth having — it is
pure file read, no ELF parsing required for the version.

**Compiler flags from ELF:** Extracting the compiler flags used to build `libcrypto.so`
(e.g., `-mbranch-protection=standard`) from the binary requires reading the `.comment`
section or GCC build notes. `goblin` can access ELF sections. This is feasible but should
be explicitly stated in the plan, including which ELF section the build flags appear in.

**`OPENSSL_armcap` / `OPENSSL_ia32cap` bitmask:** These are NOT environment variables in
the detection context — they are internal bitmasks that OpenSSL uses to record which CPU
features it discovered at startup. They appear as `.data` symbols in `libcrypto.so`. Reading
them from the symbol table gives the compiled-in capability mask; the runtime value requires
actually calling the library. For a posture probe that avoids subprocess calls, reading the
compiled-in mask via ELF symbol table is the correct approach. The plan should clarify this
distinction.

### 2.3 Layer E and CET Binary Verification — Shared `ElfInspector` Proposal

Both the CPU extension probe and the OpenSSL posture module need ELF binary inspection:

- **CPU probe:** ELF `.note.gnu.property` for CET-SS, CET-IBT, PAC, BTI flags on UMRS
  binaries (and potentially system binaries of interest)
- **OpenSSL posture:** ELF NEEDED entries for `libssl.so.3`/`libcrypto.so.3`, RELRO, PIE,
  NX flags, BTI/PAC notes — on arbitrary user-specified binaries

These have substantial overlap. Implementing the same `goblin`-based parsing twice is
duplication with the attendant divergence risk. A shared `ElfInspector` type in
`umrs-platform` is warranted.

**Proposed `ElfInspector` design:**

Module path: `umrs-platform::elf_inspect` (new top-level module, not nested under posture
or the CPU probe)

```rust
/// Pure-Rust ELF binary inspector backed by `goblin`.
///
/// Parses ELF headers, section table, dynamic table (NEEDED entries), and
/// GNU property notes from a binary loaded into memory. Callers provide a
/// `Vec<u8>` (read via `SecureReader`); this type performs no I/O.
///
/// NIST SP 800-53 SI-7: binary integrity — hardening flags verify that
/// the binary was compiled with expected protections.
/// NSA RTB: non-bypassable — file read goes through SecureReader before
/// bytes reach this inspector.
pub struct ElfInspector<'a> {
    elf: goblin::elf::Elf<'a>,
}

impl<'a> ElfInspector<'a> {
    /// Construct from a byte slice (caller owns the allocation).
    pub fn parse(bytes: &'a [u8]) -> Result<Self, ElfInspectError>;

    /// NEEDED shared library entries from the dynamic section.
    pub fn needed_libs(&self) -> Vec<&str>;

    /// Binary hardening flags (RELRO, PIE, NX).
    pub fn hardening_flags(&self) -> BinaryHardeningFlags;

    /// GNU property notes (CET-SS, CET-IBT on x86_64; PAC, BTI on aarch64).
    pub fn gnu_property_flags(&self) -> GnuPropertyFlags;

    /// ELF architecture (useful for cross-platform analysis).
    pub fn architecture(&self) -> ElfArch;
}

/// Binary hardening flags extracted from ELF headers and dynamic section.
pub struct BinaryHardeningFlags {
    pub has_relro: bool,       // PT_GNU_RELRO segment present
    pub has_full_relro: bool,  // BIND_NOW in DT_FLAGS / DT_FLAGS_1
    pub is_pie: bool,          // ET_DYN ELF type
    pub has_nx_stack: bool,    // GNU_STACK segment is non-executable
}

/// GNU property note flags.
///
/// On x86_64: bits from `GNU_PROPERTY_X86_FEATURE_1_AND`.
/// On aarch64: bits from `GNU_PROPERTY_AARCH64_FEATURE_1_AND`.
pub struct GnuPropertyFlags {
    /// CET Shadow Stack (x86_64: bit 0 of `GNU_PROPERTY_X86_FEATURE_1_AND`).
    pub cet_shadow_stack: bool,
    /// CET IBT (x86_64: bit 1 of `GNU_PROPERTY_X86_FEATURE_1_AND`).
    pub cet_ibt: bool,
    /// ARM PAC (aarch64: bit 0 of `GNU_PROPERTY_AARCH64_FEATURE_1_AND`).
    pub arm_pac: bool,
    /// ARM BTI (aarch64: bit 1 of `GNU_PROPERTY_AARCH64_FEATURE_1_AND`).
    pub arm_bti: bool,
}
```

**File read pattern:** The caller reads the binary via the existing `SecureReader`-anchored
file I/O infrastructure (specifically, open the file, verify it is a regular file via `statx`,
read the bytes). The `ElfInspector` receives the bytes and does no I/O. This keeps provenance
verification in the established layer and makes `ElfInspector` unit-testable with fixture
bytes without a filesystem.

**Important:** Reading arbitrary binaries from the filesystem is NOT a `ProcfsText` or
`SysfsText` operation. It is a regular file read. The plan should specify the use of
`rustix::fs::openat2` with `ResolveFlags::NO_SYMLINKS` and a `statx` check for
`S_ISREG` before reading. This is the TOCTOU-safe file access pattern the project already
uses in the `detect` pipeline.

### 2.4 `OpenSslPosture` Dual-Audience API — Design Is Sound With One Gap

The proposed type structure maps well to the dual-audience API pattern:

Simple accessors (novice path):
```rust
impl OpenSslPosture {
    pub fn is_provider_only(&self) -> bool { ... }
    pub fn is_fips_active(&self) -> bool { ... }
    pub fn has_hw_accel(&self) -> bool { ... }
    pub fn version(&self) -> &OpenSslVersion { ... }
}
```

Full evidence chain (auditor path):
```rust
    pub fn evidence(&self) -> &EvidenceBundle { ... }
    pub fn provider_model(&self) -> ProviderModel { ... }
    pub fn algorithm_policy(&self) -> &AlgorithmPolicyCheck { ... }
    pub fn hw_acceleration(&self) -> &HwAccelStatus { ... }
```

**Gap in the plan:** The `ProviderModel` enum is not defined. The plan names the variants
in prose but does not define the type. For implementation planning it should appear as:

```rust
pub enum ProviderModel {
    ProviderOnly,               // default provider only — clean baseline
    FipsActive,                 // default + fips providers
    LegacyLoaded,               // default + legacy — flag for operator
    EngineActive,               // old engine model — warning, opaque path
    Unknown,                    // could not determine — fail-closed
}
```

**Gap in the plan:** The `BinaryLinkageReport` includes `hardening: BinaryHardening` —
but `BinaryHardening` is not defined anywhere. Once `ElfInspector` exists, this becomes
`BinaryHardening(BinaryHardeningFlags)` — a thin newtype wrapping the shared type.
Similarly `OpenSslLinkage: LinkageStatus` maps to the `ElfInspector::needed_libs()` output.

### 2.5 `goblin` Supply Chain Assessment

The plan states `goblin` is "APPROVED by Jamie." For completeness, the implementation-level
concerns:

- `goblin` is pure Rust and has no `#![forbid(unsafe_code)]` in its own root, but its
  unsafe blocks are localized and well-reviewed. The project's `#![forbid(unsafe_code)]`
  applies to `umrs-platform` source, not to its dependencies. This is consistent with the
  `rusqlite` precedent.
- `goblin` is used by `cargo` (via the `object` crate, which uses `goblin` for some
  targets) and by `miri`. Supply chain hygiene is acceptable.
- `goblin` depends on `scroll` for binary deserialization. `scroll` should be reviewed
  for any `unsafe` usage — it does contain unsafe for performance-critical byte reads. This
  is comparable to the `rustix` situation: unsafe is internal, no unsafe surface to callers.
- The dependency should be added to `umrs-platform/Cargo.toml` with the same level of
  documentation as `rusqlite` — purpose, version, feature flags used, supply chain note,
  and architectural decision reference. **Do not use default features** unless reviewed;
  `goblin` default features pull in `mach` (Mach-O) and `pe` (PE) parsers which add
  unnecessary dependency surface for a Linux-only deployment target.

**Recommended Cargo.toml entry:**
```toml
# goblin 0.8 — pure-Rust ELF parser for binary hardening analysis.
# Used in elf_inspect module: NEEDED entries, GNU property notes (CET/PAC/BTI),
# hardening flags (RELRO, PIE, NX). Linux-only features enabled; PE/Mach-O disabled.
# Supply chain: used by cargo (via object crate), well-reviewed, acceptable unsafe
# encapsulated within goblin + scroll. No unsafe surface exposed to callers.
# Reviewed and accepted by: Jamie Adams, 2026-03-XX
# NIST SP 800-53 SI-7: binary integrity verification.
goblin = { version = "0.8", default-features = false, features = ["elf32", "elf64", "endian_fd"] }
```

### 2.6 Phase 1 "No Subprocess Calls" and OpenSSL Config Discovery

The plan states provider detection reads the "provider directory." For the implementer, the
concrete path on RHEL 10 is:

- `OPENSSLDIR` = `/etc/pki/tls` (RHEL 10 default)
- Module directory: `/usr/lib64/ossl-modules/` — scan for `*.so` files
- FIPS module: `/usr/lib64/ossl-modules/fips.so`
- Legacy module: `/usr/lib64/ossl-modules/legacy.so`

The plan should specify these paths rather than leaving "provider directory" to the
implementer to discover. They are RHEL-specific and deviate from upstream OpenSSL defaults.

**Engine model detection** is harder without subprocess calls. Engines are loaded via
`openssl.cnf` directives. Reading `/etc/pki/tls/openssl.cnf` and checking for `[engines]`
sections or `ENGINE` load directives is the read-only detection path. The plan should
acknowledge this.

---

## Part 3 — Shared `ElfInspector` — Additional Design Notes

### 3.1 File Read Pattern for Binary Inspection

The `ElfInspector` receives bytes, but those bytes must arrive via a provenance-aware read.
Since binary paths are arbitrary (not `/proc/` or `/sys/`), they use the regular file read
path. The established TOCTOU-safe pattern from `detect/integrity_check.rs` is:

1. `rustix::fs::openat2(dir_fd, path, flags, mode, resolve_flags)` with `RESOLVE_NO_SYMLINKS`
2. `rustix::fs::statx(fd, ...)` — verify `S_ISREG`, record metadata in `EvidenceRecord`
3. Read all bytes via `fd`
4. Pass bytes to `ElfInspector::parse()`

A new `BinaryFileReader` helper in `elf_inspect` would encapsulate this pattern and return
`(Vec<u8>, FileStat)` — the bytes plus the provenance metadata. The `FileStat` goes into
the `EvidenceBundle`.

### 3.2 Architecture-Conditional Compilation

`GnuPropertyFlags` fields should be architecture-conditional at the type level rather than
always-present booleans that happen to always be `false` on the wrong architecture:

```rust
pub struct GnuPropertyFlags {
    #[cfg(target_arch = "x86_64")]
    pub cet_shadow_stack: bool,
    #[cfg(target_arch = "x86_64")]
    pub cet_ibt: bool,
    #[cfg(target_arch = "aarch64")]
    pub arm_pac: bool,
    #[cfg(target_arch = "aarch64")]
    pub arm_bti: bool,
}
```

Alternatively — and this may serve the dual-audience API better for cross-platform TUI
rendering — use an enum:

```rust
pub enum GnuPropertyFlags {
    X86 { cet_shadow_stack: bool, cet_ibt: bool },
    Aarch64 { arm_pac: bool, arm_bti: bool },
    Unknown,
}
```

The enum approach handles cross-compilation analysis (inspecting an aarch64 binary on an
x86_64 host, or vice versa) which is a legitimate auditor workflow. The `cfg` approach
loses that ability. **Recommendation:** use the enum.

### 3.3 Where Does `ElfInspector` Live?

The plan has no `elf_inspect` module yet. There are two options:

**Option A:** `umrs-platform::elf_inspect` — a new top-level module in `umrs-platform`,
parallel to `kattrs`, `posture`, `detect`, etc.

**Option B:** `umrs-platform::posture::binary` — a submodule of posture.

Recommendation: **Option A**. The `ElfInspector` is used by both the CPU extension probe
and the OpenSSL posture module, and potentially by future binary analysis work
(`openssl-no-vendoring.adoc` automation). A top-level module with a clear name signals
its general utility.

---

## Part 4 — Items That Must Be Added to the Plans

### Additions Required for `cpu-extension-probe.md`

1. **Add "Layer 4 — Runtime Mitigation State" section** covering vulnerability sysfs,
   the `VulnerabilityReport` type design, and the sysfs file inventory.
2. **Add `CpuSignalId` variant table** with all Phase 1E/1F signals (PCID, CET-SS,
   CET-IBT, UMIP, PKU, PAC, BTI, MTE) and their detection sources.
3. **Document the Rust CET limitation** (rust-lang/rust#93754) and specify INFORMATIONAL
   severity for UMRS binaries missing CET/PAC/BTI notes, with the Rust memory-safety
   rationale.
4. **Add ARM Detection Path subsection** covering PAC vs CET-SS mechanical differences,
   PACMAN attack context, BTI, and MTE; ARM vulnerability sysfs file inventory.
5. **Add "Existing Infrastructure Reuse" section** listing `ProcfsText`, `SysfsText`,
   `EvidenceBundle`, `AssuranceImpact`, and `SecureReader` as ready to use.
6. **Add `VulnerabilityReport` module placement decision:** `posture::vulnerability`
   (sibling to `posture::bootcmdline`), not inside the CPU extension probe.
7. **Add `goblin` dependency to Definition of Done** — requires Jamie approval as
   architectural review trigger; cross-reference openssl plan which has the same dependency.
8. **Add PCID + Pti contradiction logic** to the contradiction detector work items.

### Additions Required for `openssl-posture-module.md`

1. **Concrete OpenSSL identity detection path** — specify ELF symbol table vs
   `pkg-config` file vs `.rodata` scan, and which is preferred.
2. **Specify RHEL 10 provider directory paths** — `/usr/lib64/ossl-modules/` and
   the `openssl.cnf` engine detection approach.
3. **Define `ProviderModel` enum** with all variants in the plan.
4. **Define `BinaryHardeningFlags` / `GnuPropertyFlags`** or reference the proposed
   shared `ElfInspector`.
5. **Add `ElfInspector` proposal** — document the shared type, its module path
   (`umrs-platform::elf_inspect`), and how both this plan and the CPU probe use it.
6. **Document `goblin` features** — `default-features = false` with `elf32 elf64
   endian_fd` only. Add Cargo.toml documentation block.
7. **Document `OPENSSL_armcap`/`OPENSSL_ia32cap` bitmask read path** — symbol table
   (compiled-in mask), not environment variable (runtime override).
8. **Add `BinaryFileReader` helper** to the Phase 4 design — TOCTOU-safe regular file
   read returning `(Vec<u8>, FileStat)`.

---

## Part 5 — Summary of Gaps and Risks

| Item | Severity | Plan | Description |
|---|---|---|---|
| Vulnerability sysfs (Layer 4) absent from model | High | CPU probe | Runtime mitigation state is orthogonal to extension activation; entirely missing from plan |
| Rust CET/PAC limitation undocumented | High | CPU probe | UMRS binaries will not have CET/PAC ELF notes; needs explicit INFORMATIONAL classification |
| `CpuSignalId` variants not defined | Medium | CPU probe | Plan defers design; corpus research is complete; variants should now be listed |
| OpenSSL identity detection path underspecified | Medium | OpenSSL | "ELF `.rodata` or config files" is too vague for implementation |
| `ProviderModel` enum not defined | Medium | OpenSSL | Variants named in prose only |
| RHEL 10 provider directory paths not specified | Medium | OpenSSL | `/usr/lib64/ossl-modules/` not mentioned |
| No shared `ElfInspector` design | Medium | Both | Both plans need ELF parsing; no shared type proposed |
| `goblin` features not constrained | Low | OpenSSL | Default features pull in Mach-O/PE parsers unnecessarily |
| ARM PACMAN attack context missing | Low | CPU probe | Relevant for risk assessment on ARM targets |
| PCID/Pti contradiction logic not captured | Low | CPU probe | High-value cross-signal finding not in the plan |

---

*Review complete. No source files were modified. All recommendations are advisory pending
Jamie's decisions.*
