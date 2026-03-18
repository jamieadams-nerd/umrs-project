---
name: CPU Access Controls and Mitigations Familiarization
description: Phase 1E/1F reference material — PCID, vulnerability sysfs, CET-SS, CET-IBT, UMIP, PKU, ARM PAC/BTI/MTE
type: project
---

# CPU Access Controls and Mitigations Familiarization
**Phases:** 1E (Speculative Execution Mitigations — PCID) + 1F (CPU-Enforced Access Controls)
**Files read:** pcid.md, vulnerability-sysfs-reference.md, cet-shadow-stack.md, cet-ibt.md, umip.md, pku.md, arm-access-controls.md
**Date:** 2026-03-18

---

## 1. Key Types and Detection Paths

### New IndicatorId / CpuIndicatorId Variants Implied

The reference material implies the following new signal-level concepts that do not yet exist
in `umrs-platform/src/posture/indicator.rs`:

| Concept | Suggested variant | Detection source | Priority |
|---|---|---|---|
| PCID present (PTI perf enabler) | `CpuIndicatorId::Pcid` | `/proc/cpuinfo` `pcid` flag | Medium |
| CET-SS hardware present | `CpuIndicatorId::CetShadowStack` | `/proc/cpuinfo` `shstk` flag | High |
| CET-IBT hardware present | `CpuIndicatorId::CetIbt` | `/proc/cpuinfo` `ibt` flag | High |
| UMIP hardware present | `CpuIndicatorId::Umip` | `/proc/cpuinfo` `umip` flag | Important |
| PKU hardware present | `CpuIndicatorId::Pku` | `/proc/cpuinfo` `pku` + `ospke` flags | Informational |
| ARM PAC present | `CpuIndicatorId::ArmPac` | `/proc/cpuinfo` `paca` flag | High (ARM targets) |
| ARM BTI present | `CpuIndicatorId::ArmBti` | `/proc/cpuinfo` `bti` flag | High (ARM targets) |
| ARM MTE present | `CpuIndicatorId::ArmMte` | `/proc/cpuinfo` `mte`/`mte2`/`mte3` | Informational |

Note: These are Layer 1 (hardware capability) signals. Layer 2 (active utilization) is a
separate detection concern covered by binary analysis (ELF `.note.gnu.property`) and
per-process status (`/proc/<pid>/status`).

### Vulnerability Sysfs — New Types Implied

The reference proposes `VulnerabilityStatus` and `VulnerabilityReport` structs for
`/sys/devices/system/cpu/vulnerabilities/`. These are distinct from existing posture indicators
(which check kernel cmdline parameters) and represent runtime mitigation state.

The files to enumerate:
- `spectre_v1`, `spectre_v2`, `spec_store_bypass`, `mds`, `tsx_async_abort`
- `itlb_multihit`, `mmio_stale_data`, `retbleed`, `srbds`
- `gather_data_sampling`, `reg_file_data_sampling`, `ghostwrite`
- ARM-specific: `spectre_bhb`

These must be read via `SecureReader::read_generic_text` (`SysfsText` path) — not raw
`File::open`. This is mandatory per the TOCTOU / procfs-sysfs routing rule.

---

## 2. Rust Implementation Notes

### 2.1 `VulnerabilityStatus` Enum — Evaluation

The proposed type design from the reference is sound and maps directly to a well-typed Rust enum:

```rust
pub enum VulnerabilityStatus {
    NotAffected,
    Vulnerable(String),      // includes detail text after "Vulnerable: "
    Mitigated(String),       // includes detail text after "Mitigation: "
    Unknown,                 // file missing or parse failure
}
```

**Assessment:** This design is correct. The `String` detail fields preserve the kernel's
free-form mitigation description without loss. `Unknown` is the fail-closed sentinel for
missing files (kernel version too old, or virtualized guest with no vulnerability exposure).

**TPI applicability:** The parse logic for each file is simple prefix-matching (Not affected /
Vulnerable / Mitigation:). This does not rise to the level requiring TPI — a single linear
parse is appropriate. TPI applies to security context parsing (dual `nom` + `FromStr`), not
trivial sysfs text classification.

**Implementation note for spectre_v2:** The reference documents semicolon-separated
sub-components (IBPB status, STIBP status, RSB filling, BHI). Consider a structured
`Spectre2Detail` type that parses these components rather than embedding them in the
raw `String`. This enables programmatic querying (e.g., "is IBPB conditional or disabled?").

**`VulnerabilityReport` struct:** A flat struct with one `VulnerabilityStatus` field per
vulnerability file is the correct approach. Consider making fields `Option<VulnerabilityStatus>`
so that "file not present" (older kernel, some ARM) is representable without conflating
it with `Unknown` from a failed read.

### 2.2 CET Binary Verification — `goblin` Crate

The reference describes Layer 2 binary verification: parse ELF `.note.gnu.property` to check
for `GNU_PROPERTY_X86_FEATURE_1_SHSTK` (bit 1) and `GNU_PROPERTY_X86_FEATURE_1_IBT` (bit 0).

**`goblin` crate assessment:**
- `goblin` is a widely-used pure-Rust ELF/PE/Mach-O parser.
- It supports `note` section parsing including GNU property notes.
- Supply chain: well-maintained, used by `cargo` itself indirectly via `object` crate.
- No `unsafe` in the parsing surface visible to us (internal `unsafe` is contained).
- RHEL 10 deployment: pure Rust, no system library dependency.

**Recommendation:** `goblin` is acceptable for binary CET note inspection, but requires
Jamie's approval as a new external dependency (architectural review trigger). The alternative
is shelling out to `readelf`, which introduces a subprocess dependency and is not
deterministic from a high-assurance standpoint. `goblin` is the better choice.

**ARM PAC/BTI binary detection uses the same ELF `.note.gnu.property` mechanism** —
the same `goblin` path would serve both x86 CET and ARM PAC/BTI binary analysis.

### 2.3 PKU and Crypto Key Isolation — UMRS Use Case

The reference raises `pkey_alloc` / `pkey_mprotect` as a possible mechanism for protecting
cryptographic key material within a process address space.

**Assessment for UMRS:** Limited applicability at this time.

Reasons:
1. PKU requires inline assembly or compiler intrinsics (WRPKRU) for domain transitions —
   this conflicts with `#![forbid(unsafe_code)]` unless done via a safe abstraction layer
   (e.g., a crate that wraps the syscall interface). The syscall path (`pkey_alloc`,
   `pkey_mprotect`) is accessible via `rustix` without unsafe.
2. The WRPKRU bypass caveat (any code in the process can unlock domains) limits PKU's
   value as a hard isolation boundary. It is defense-in-depth, not a hard guarantee.
3. UMRS currently uses `secrecy::Secret<T>` and `Zeroize` for key material — these provide
   memory zeroization but not domain isolation. PKU would be an additive layer.
4. Hardware availability: Xeon Scalable (2017) for Intel, Zen 3 (2020) for AMD. Not universal
   on all target systems.

**Conclusion:** Flag as an INFORMATIONAL opportunity to Jamie. Not a current implementation
priority. If UMRS develops a log-sealing or key-management pipeline, revisit PKU as a
defense-in-depth mechanism for that specific subsystem.

### 2.4 CET Rust Limitation — Critical Finding

**rust-lang/rust#93754**: Stable Rust does not support `-Z cf-protection=full`. This means:

- UMRS binaries compiled with stable Rust will NOT carry `GNU_PROPERTY_X86_FEATURE_1_SHSTK`
  or `GNU_PROPERTY_X86_FEATURE_1_IBT` ELF notes.
- On CET-capable hardware (Tiger Lake+, Zen 3+) running RHEL 10 (glibc 2.39+ with
  shadow stack activation), UMRS processes will NOT activate shadow stack.
- The dynamic linker AND-ing of library properties means a single non-CET binary in the
  process's dependency chain disables CET for the entire process.

**Audit classification:** INFORMATIONAL
**Rationale:** Rust's ownership model, borrow checker, and `#![forbid(unsafe_code)]` provide
compile-time memory safety guarantees that render ROP and JOP significantly harder to
construct. The attack surface that CET defends against (memory corruption enabling gadget
chains) is substantially reduced in safe Rust. This is the same classification as GCC-compiled
code without CET on a non-CET system.

**What to record in a posture check:** When inspecting UMRS binaries on CET-capable hardware,
the absence of CET ELF notes is an INFORMATIONAL finding with the Rust memory-safety
mitigation explanation. Do NOT classify it as HIGH (that severity is reserved for C/C++ binaries
missing CET on CET-capable systems).

**Action required when rust-lang/rust#93754 is resolved:** Revisit and add `-Z cf-protection=full`
to the UMRS build configuration. File a task at that time.

### 2.5 PCID Cross-Reference with Existing `Pti` Indicator

The reference establishes a direct dependency: PTI without PCID causes 30-50% performance
regression, creating operational pressure to add `nopti` to the kernel cmdline.

The existing `IndicatorId::Pti` tracks whether PTI has been disabled via cmdline. A new
`CpuIndicatorId::Pcid` signal would complement this: if `Pti` finds PTI active AND PCID is
absent from `/proc/cpuinfo`, that is a CAUTION finding (performance risk that may lead to
future disablement). If `Pti` finds PTI disabled AND PCID is present, that is a CRITICAL
finding (Meltdown mitigation disabled despite hardware support for the performance fix).

---

## 3. Cross-References to Existing Posture Signals

| Existing `IndicatorId` | New material connection |
|---|---|
| `Mitigations` | Umbrella indicator; `VulnerabilityReport` would provide per-file detail backing it |
| `Pti` | Cross-ref with `CpuIndicatorId::Pcid` — PCID absent + PTI active = CAUTION; PCID present + PTI disabled = CRITICAL |
| `SpectreV2Off` | `vulnerability-sysfs-reference.md` documents the spectre_v2 sysfs file — IBPB/STIBP sub-fields provide richer state than the cmdline-off indicator alone |
| `SpectreV2UserOff` | Same as above |
| `MdsOff` | `mds` vulnerability sysfs file maps directly |
| `TsxAsyncAbortOff` | `tsx_async_abort` vulnerability sysfs file maps directly |
| `RetbleedOff` | `retbleed` vulnerability sysfs file maps directly |
| `SrbdsOff` | `srbds` vulnerability sysfs file maps directly |

**Gap:** There are no existing indicators for:
- `gather_data_sampling` (Downfall, CVE-2022-40982)
- `reg_file_data_sampling` (RFDS, CVE-2023-28746)
- `mmio_stale_data` (CVE-2022-21123 series)
- `itlb_multihit` (CVE-2018-12207)
- `spectre_v1` (sysfs file — different from the cmdline off variant)

These vulnerability sysfs files have no current cmdline-off indicators because they are
mitigated by microcode or kernel internals rather than by cmdline opt-outs. They require
direct sysfs reads.

---

## 4. Critical Findings for UMRS

### Finding 1 — Rust CET Limitation (INFORMATIONAL, architectural)
- UMRS Rust binaries will NOT activate CET shadow stack or IBT on CET-capable hardware.
- This is an INFORMATIONAL finding due to Rust memory safety providing comparable protection.
- Monitor rust-lang/rust#93754 for resolution.
- When reporting binary CET status, distinguish Rust binaries from C/C++ binaries.

### Finding 2 — VulnerabilityReport type is not yet implemented
- The sysfs vulnerability interface provides authoritative runtime mitigation state.
- The existing `Mitigations` indicator only checks whether the cmdline umbrella flag is set.
- There is a gap: no UMRS code reads the individual vulnerability sysfs files.
- The `VulnerabilityReport` type would close this gap and provide per-vulnerability detail.

### Finding 3 — Gather Data Sampling and RFDS have no indicators
- CVE-2022-40982 (Downfall / `gather_data_sampling`) and CVE-2023-28746 (RFDS /
  `reg_file_data_sampling`) are post-2022 vulnerabilities.
- No existing `IndicatorId` variants cover these.
- Both require microcode updates; the sysfs files are the only check path.

### Finding 4 — PCID is missing from the CPU probe
- PCID is not currently tracked as a CPU signal.
- Its primary value is as a cross-reference with the existing `Pti` indicator.
- Absence of PCID + active PTI should produce a CAUTION finding in operator output.

### Finding 5 — ARM PAC/BTI binary detection requires `goblin` (new dependency)
- Detecting whether UMRS binaries on ARM have PAC/BTI ELF notes requires ELF parsing.
- `goblin` is the appropriate crate; requires Jamie's approval before adoption.

---

## 5. ARM vs x86 Feature Mapping

### Control-Flow Integrity

| Threat | x86 defense | ARM defense | Notes |
|---|---|---|---|
| ROP (return-oriented programming) | CET Shadow Stack (`shstk`) | PAC (`paca`/`pacg`) | Different mechanisms: x86 uses parallel stack, ARM uses crypto signing |
| JOP/COP (jump/call-oriented) | CET-IBT (`ibt`) | BTI (`bti`) | Same concept, different instruction (`ENDBR64` vs `BTI c/j/jc`) |

### PAC vs CET-SS difference

PAC signs return addresses cryptographically in the upper pointer bits — no separate memory
structure. CET-SS maintains a separate shadow stack page protected by special PTE encoding.
PAC is more robust against direct memory attacks (no shadow stack page to target) but has
a 1/128 brute-force window with 48-bit VA. CET-SS has no brute-force window but requires
shadow-stack-writable memory isolation.

**PACMAN attack (2022):** Speculative execution can be used to probe PAC values on Apple M1
without triggering faults. Mitigated by combining PAC with BTI. Relevant for UMRS ARM
deployment risk assessment.

### Memory Safety

| Threat | x86 defense | ARM defense | Notes |
|---|---|---|---|
| Memory corruption (UAF, overflow) | None at hardware level | MTE (`mte`/`mte2`/`mte3`) | MTE is unique to ARM; no x86 equivalent |

MTE is architecturally novel — it provides probabilistic hardware memory safety (4-bit tag,
1/16 collision chance). On ARM deployment targets, MTE on glibc/OpenSSL provides defense-in-depth
for C library dependencies. Rust code benefits minimally (compile-time safety), but `unsafe`
blocks could be protected.

### Detection path unification

Both x86 and ARM Layer 1 signals use `/proc/cpuinfo` flags. The detection code should use
a platform-conditional approach:

```
x86_64: shstk, ibt, umip, pku, pcid, ospke
aarch64: paca, pacg, bti, mte, mte2, mte3
```

Both platforms share the vulnerability sysfs interface (`/sys/devices/system/cpu/vulnerabilities/`)
for applicable entries — ARM populates `spectre_v1`, `spectre_v2`, `spec_store_bypass`,
`meltdown`, and the ARM-specific `spectre_bhb`.

### Compiler flag mapping

| Platform | Protection | Flag |
|---|---|---|
| x86_64 (GCC/Clang) | CET-SS + IBT | `-fcf-protection=full` |
| aarch64 (GCC/Clang) | PAC + BTI | `-mbranch-protection=standard` |
| Rust (all) | None (stable) | Awaiting rust-lang/rust#93754 |

On octopussy (ARM64 development machine), OpenSSL is compiled with `-mbranch-protection=standard`.
UMRS Rust binaries on octopussy will NOT have PAC/BTI ELF notes — same INFORMATIONAL
classification as x86 CET.

---

## 6. Implementation Checklist (Future Work)

When approved for implementation, the following items are prerequisites or co-requisites:

1. `VulnerabilityStatus` + `VulnerabilityReport` types in `umrs-platform` — reads via
   `SecureReader` (SysfsText path). No new sysfs routing needed; existing infrastructure applies.

2. New `CpuIndicatorId` variants: `Pcid`, `CetShadowStack`, `CetIbt`, `Umip`, `Pku`, `ArmPac`,
   `ArmBti`, `ArmMte` — all read from `/proc/cpuinfo` flags via `ProcfsText` path.

3. `goblin` dependency approval required for binary ELF `.note.gnu.property` inspection.
   Raises supply chain review requirement.

4. `Pti` + `Pcid` contradiction logic: cross-reference these two signals in the contradiction
   detector.

5. Integration tests for `VulnerabilityReport` should use fixture sysfs files (the same pattern
   already established for modprobe and bootcmdline tests).

6. Spectre v2 sub-field parsing (IBPB/STIBP/BHI) is optional complexity — start with
   the flat `Mitigated(String)` representation and add structured parsing in a follow-up.

---

**Why:** This material directly informs Phase 1F of the CPU extension probe design in
`umrs-hw` / `umrs-platform`. The access control signals (CET, UMIP, PKU, PAC, BTI) are
distinct from performance/crypto extension signals and occupy Category 11 in the
probe's 23-column matrix taxonomy.

**How to apply:** Reference this file when designing `VulnerabilityReport`, new `CpuIndicatorId`
variants for access controls, and the `goblin`-based binary analysis path.
