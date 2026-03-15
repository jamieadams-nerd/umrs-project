# CPU Feature Matrix — Rust Developer Review
**Phase 0 Pre-Research Review**
**Author:** rust-developer agent
**Date:** 2026-03-14
**Source specs reviewed:**
- `.claude/jamies_brain/cpu-matrix.md`
- `.claude/jamies_brain/cpu-research.md`
- `.claude/jamies_brain/cpu-post-research.md`

---

## 1. Overview

This review evaluates the 17-column CPU feature matrix specification from an implementation
perspective before the researcher agent builds the corpus. The goal is to identify columns
that are clear vs. ambiguous, flag detection method feasibility in safe Rust, recommend
missing columns, and propose where CPU feature detection integrates with the existing
`umrs-platform` codebase.

The existing codebase already has a mature pattern for kernel attribute reading:
`StaticSource` + `SecureReader` + TPI parsing, with provenance verification via `fstatfs`
magic checks. Any CPU detection implementation must integrate cleanly with this pattern.

---

## 2. Column-by-Column Analysis of the 17 Matrix Columns

### Column 1: Feature
**Type mapping:** `&'static str` or a typed enum `CpuFeatureId`

Clear. This is the canonical feature name (e.g., `"AES-NI"`, `"RDRAND"`).

**Recommendation:** The researcher should define and document a canonical name per feature
along with known aliases. The implementation will need a stable identifier — a `CpuFeatureId`
enum variant is preferred over a free-form string because it enables compile-time exhaustiveness
checks, which matters for the Signal Evaluation Path Rule (see `signal.rs`). If the corpus uses
free-form strings, the implementation will have to map them to typed variants anyway.

---

### Column 2: Vendor
**Type mapping:** `enum CpuVendor { Intel, Amd, Arm, Both, Any }`

Clear. A closed enum is the right representation. The researcher should confirm whether any
features listed as "Both" vendor entries appear under different CPUID leaf/bit addresses per
vendor — this is common and matters for detection logic.

**Concern:** "Both" is underspecified as a column value. The corpus must clarify:
- Does the feature use the same CPUID leaf/bit on both vendors?
- Or is the same capability exposed under vendor-specific CPUID structure?

AES-NI is a concrete case: Intel and AMD expose it at the same CPUID location (leaf 1, ECX
bit 25), but the researcher must document this explicitly per vendor. Do not assume symmetry.

---

### Column 3: Category
**Type mapping:** `enum CpuFeatureCategory { SymmetricCrypto, HashAuth, BigIntPki, Entropy,
TrustedExecution, ConfidentialComputing, MemoryEncryption, VectorAcceleration, BitManipulation,
PlatformAttestation, Reliability, VirtualizationSecurity }`

Clear. The 12 categories listed in the spec map directly to a closed enum. This is exactly the
right approach for audit card grouping and programmatic filtering.

**Recommendation:** The researcher must assign exactly one primary category per feature. Some
features (e.g., VAES straddles SymmetricCrypto and VectorAcceleration) could fit multiple
categories. The corpus must document the primary assignment and any secondary ones so the
implementation knows which grouping takes precedence in audit card display.

---

### Column 4: Purpose
**Type mapping:** `&'static str`

Clear. A human-readable description string. Straightforward.

---

### Column 5: Example instructions
**Type mapping:** `&'static [&'static str]` or `Option<&'static [&'static str]>`

Clear in most cases. Some features (RDRAND, RDSEED) have dedicated instructions.
For vector features (SSE, AVX), the instruction list is very large. The spec should
clarify whether this column lists a representative subset or attempts completeness.

**Recommendation for researcher:** This column needs a bounded representative set —
not a full ISA listing. Define "example instructions" as: up to 5 instructions that
are the most security-relevant or most commonly used in FIPS-relevant code paths.
This scopes the column usably for an audit card.

---

### Column 6: CPUID leaf/subleaf/bit
**Type mapping:** `struct CpuidLocation { leaf: u32, subleaf: Option<u32>, register: CpuidReg,
bit: u8 }` where `enum CpuidReg { Eax, Ebx, Ecx, Edx }`

This is the most technically demanding column and the most important for reliable
detection. It is a structured type, not a string. The researcher must document:
- The exact leaf number (hex)
- The subleaf if applicable (leaf 7 uses subleaf 0 vs. 1 for different features)
- Which output register (EAX, EBX, ECX, or EDX)
- Which bit

**Concern:** Some features span multiple CPUID leaves (e.g., AVX-512 has features spread
across leaf 7 subleaf 0 EBX, ECX, and EDX). The single "leaf/subleaf/bit" column is
insufficient for these. The corpus must document all CPUID locations per feature even
if the display column simplifies to the primary one.

**Implementation note:** Raw CPUID access from userspace Rust without unsafe code is not
directly possible from the instruction itself. However, Linux exposes CPUID data via
`/proc/cpuinfo` flags and `/dev/cpu/*/cpuid`. The `/proc/cpuinfo` route is the only
safe-Rust option without FFI. See Section 3 for the full feasibility analysis.

---

### Column 7: Linux flag or detection mechanism
**Type mapping:** `enum DetectionMethod { ProcCpuinfo(&'static str), SysfsPath(&'static str),
KernelCmdline(&'static str), ProcStatus(&'static str), NotExposed }`

This is the most implementation-critical column. The researcher must be precise:
- Which `/proc/cpuinfo` flag name (these are kernel-assigned, not always identical to
  vendor flag names)
- Which sysfs path if applicable (e.g., `/sys/module/kvm_intel/parameters/`)
- Whether the kernel exposes the feature at all in `/proc/cpuinfo` for the feature class

**Concern:** The cpu-matrix.md explicitly cites Linux documentation that
`/proc/cpuinfo` flags "are mainly for kernel debugging" and applications should prefer
`kcpuid` or `cpuid(1)`. The researcher must not treat `/proc/cpuinfo` as the primary
detection mechanism where a more authoritative sysfs or procfs path exists.

This column must clearly separate: "this is how `/proc/cpuinfo` exposes it" from
"this is the authoritative Linux detection path."

---

### Column 8: Minimum CPU generations of interest
**Type mapping:** `&'static str` or structured `struct CpuGenInfo { intel_first_gen: Option<&'static str>,
amd_first_gen: Option<&'static str> }`

Moderately complex. The spec uses prose like "Sandy Bridge onward" or "Zen 2 onward."
For a pure string column this is fine. For programmatic filtering this needs structure.

**Recommendation:** Keep this as a documentation string for Phase 0. Phase 0 is not
implementing generation filtering. Flag this column as "informational only" for the audit
card. Do not attempt to encode generation ranges as a runtime-filtered type in Phase 0.

---

### Column 9: Security benefit
**Type mapping:** `&'static str`

Clear. Human-readable prose. Keep it short — one sentence per feature suitable for
audit card display.

---

### Column 10: Performance benefit
**Type mapping:** `&'static str`

Clear. Same as above.

---

### Column 11: Assurance caveats
**Type mapping:** `&'static str`

Clear, but this may be the most important column for the security-engineer review.
This is where the corpus documents known limitations: speculative execution side
channels (AES-NI timing is not fully constant-time in all microarchitectures),
RDRAND controversies, SGX attack history, SEV-SNP limitations.

**Recommendation:** The researcher must not sanitize this column. Known attacks,
known architectural weaknesses, and known controversies belong here verbatim.
An audit card that does not surface caveats is misleading.

---

### Column 12: Virtualization behavior
**Type mapping:** `enum VirtExposure { PassThrough, GuestVisible, CanBeMasked, AlwaysMasked,
HypervisorDependent, NotApplicable }`

This is a complex column but the enum type is clean. The researcher must document:
- Whether the hypervisor passes the CPUID flag through to guests
- Whether the guest `/proc/cpuinfo` reflects hardware reality or hypervisor policy
- Which hypervisors (KVM, VMware, Hyper-V) mask or expose each feature

This column is critical for the UMRS threat model: a system that believes it has AES-NI
but is running in a VM with the flag masked will silently fall back to software AES. The
detection logic must account for this.

---

### Column 13: Firmware / BIOS dependency
**Type mapping:** `enum FirmwareDep { None, BiosEnablementRequired, MicrocodeRequired, Both,
Unknown }`

Clear enum. Important for SGX (requires BIOS enable), AMD SEV (requires firmware
configuration), and Key Locker. The researcher must document which features can be
present in CPUID hardware support but disabled at the firmware layer.

---

### Column 14: Microcode dependency
**Type mapping:** `bool` or `enum MicrocodeDep { None, Required, PartiallyRequired }`

Somewhat overlaps with Column 13. The distinction the spec intends: Column 13 is about
BIOS/firmware enabling, Column 14 is about microcode patches that enable or disable
functionality post-CPU release. The clearest example is post-Spectre microcode that
disables certain branch predictor features. Some SGX capabilities also depend on microcode.

**Concern:** Columns 13 and 14 are closely related and could be merged without loss of
information. The researcher should document both clearly but the implementation can
represent them together as `struct FirmwareDeps { bios_enable_required: bool,
microcode_required: bool }`.

---

### Column 15: Audit-card relevance
**Type mapping:** `enum AuditCardRelevance { Critical, Important, Informational }`

Clear, and already aligns with the existing `AssuranceImpact` enum in `umrs-platform`.
The implementation can reuse or extend `AssuranceImpact` for CPU features.

**Recommendation:** The researcher should use the same three-tier taxonomy: Critical,
Important, Informational. Do not introduce new tiers. The security-engineer review
(Phase 0 sibling report) will assign final tier classifications.

---

### Column 16: Notes
**Type mapping:** `Option<&'static str>`

Clear. Free-form. Anything that does not fit another column goes here.

---

### Column 17: Sources
**Type mapping:** `&'static [&'static str]`

Clear. Document source URLs or document section references per feature. This is the
evidence chain for the corpus.

---

## 3. Detection Method Feasibility in Safe Rust

### 3.1 CPUID Instruction — Not Directly Accessible Without Unsafe

The CPUID instruction cannot be executed from safe Rust. It requires inline assembly
or a dedicated crate. In this codebase, `#![forbid(unsafe_code)]` is a compile-time
constraint in every crate. This rules out any direct CPUID invocation.

**Available safe alternatives:**

**Option A: `/proc/cpuinfo` parsing** (recommended for Phase 0)
- `umrs-platform` already has the full `SecureReader` + `ProcfsText` + provenance
  verification infrastructure for `/proc/` paths.
- `/proc/cpuinfo` is parseable as structured text with the existing `ProcfsText`
  route via `SecureReader::read_generic_text`.
- The Linux kernel flag name (column 7) maps to tokens in the `flags:` line of
  `/proc/cpuinfo` for x86_64 features.
- **Limitation:** `/proc/cpuinfo` reflects kernel-filtered flags, not raw hardware
  CPUID. Some features visible in hardware CPUID are not exposed in `/proc/cpuinfo`.
  This must be documented as a limitation.

**Option B: `/dev/cpu/*/cpuid` device reads**
- Linux exposes raw CPUID data via `/dev/cpu/<cpuid>` device nodes.
- This is a legitimate read path but requires `CAP_SYS_RAWIO` in many configurations.
- Not appropriate for an unprivileged audit tool.

**Option C: Crate-based CPUID via `raw_cpuid` crate**
- The `raw_cpuid` crate provides safe-Rust CPUID access by wrapping the instruction.
- Internally it uses inline assembly, which the crate marks as `unsafe`.
- Our `#![forbid(unsafe_code)]` constraint applies **per crate root** — if
  `raw_cpuid` is added as a dependency, the unsafe lives inside that crate, not ours.
- However, adding this crate requires an architectural review trigger (new external
  dependency) and supply chain assessment. This is a decision for Jamie, not Phase 0.

**Conclusion for Phase 0 specification:** The researcher should document both the
CPUID leaf/bit location (for completeness and for potential future use of Option C)
and the `/proc/cpuinfo` flag name (for the safe implementation path). Both columns
must be populated — the CPUID data is authoritative reference material; the Linux
flag is the implementation path.

---

### 3.2 `/proc/cpuinfo` Parsing

This is already a solved pattern. The existing `ProcfsText` + `SecureReader` route
in `umrs-platform` handles `/proc/` reads with `PROC_SUPER_MAGIC` provenance
verification. A CPU feature reader would:
1. Read `/proc/cpuinfo` via `SecureReader::read_generic_text` with a `ProcfsText` wrapper.
2. Parse the `flags:` line (or `Features:` on ARM) by splitting on whitespace.
3. Check for presence/absence of specific flag tokens.

This is a `ProcfsText` read — a string split check. No TPI is required here because
the `/proc/cpuinfo` flags line is a space-separated list of presence tokens; there is
no security-classification decision being made from it. The presence of `aes` in
`/proc/cpuinfo` flags is an informational signal, not an access control decision.

**Note on TPI exemption:** The TPI rule in this codebase applies to security-relevant
input where ambiguity must fail closed. CPU feature presence is a posture signal, not
an access gate. TPI is not required. The kernel attribute parser rules (boolean/dual-boolean)
also do not apply here — these are flag-presence checks.

---

### 3.3 Sysfs Paths (`/sys/devices/system/cpu/`)

Relevant for:
- `/sys/devices/system/cpu/vulnerabilities/` — exposes CPU vulnerability mitigation
  status as text files (this is already partially covered by the `Mitigations` signal
  in the posture catalog).
- `/sys/devices/system/cpu/cpu*/topology/` — CPU topology information.

The `SysfsText` + `SecureReader` route with `SYSFS_MAGIC` verification handles this.
**This is the preferred path for vulnerability mitigation status** — it is more
authoritative than `/proc/cmdline` for detecting whether a mitigation is actually active,
versus whether it was merely requested at boot.

**This is a gap in the existing posture catalog.** The current `Mitigations` signal
only checks for `mitigations=off` in `/proc/cmdline`. It does not read the actual
per-vulnerability mitigation state from `/sys/devices/system/cpu/vulnerabilities/`.
This is an important distinction for a CPU audit card.

---

### 3.4 Confidential Computing / TEE Detection

SGX, SEV/SNP, TDX detection involves:
- `/dev/sgx_enclave` or `/dev/sgx/enclave` device existence (SGX available and enabled)
- `/sys/kernel/security/tpm0/` (SGX attestation chain)
- `/sys/bus/platform/devices/MSFT0101/` (AMD SEV in some configurations)
- For AMD SEV: `/sys/module/kvm_amd/parameters/sev` and `sev_es`, `sev_snp` parameters
- For Intel TDX: `/sys/module/kvm_intel/parameters/tdx`

These sysfs/devfs paths are the realistic Linux detection mechanism for TEE features.
They require `SysfsText` reads with `SYSFS_MAGIC` verification, which the existing
infrastructure supports. The researcher must document these paths precisely.

**CPUID is not sufficient for TEE detection** — the CPU may support SGX but the BIOS
has disabled it. A system with SGX disabled at firmware level will not expose `/dev/sgx_enclave`
even if `/proc/cpuinfo` shows the `sgx` flag. The audit card must distinguish
"CPU supports SGX" from "SGX is enabled and accessible."

---

## 4. Missing Columns for a Complete CPU Audit Card

After reviewing what `umrs-platform` already tracks, the following columns are missing
from the 17-column spec and would be needed for a complete CPU audit card:

### Missing Column A: Active Mitigation Status
- **What:** The actual mitigation status per vulnerability, not the cmdline flag
- **Source:** `/sys/devices/system/cpu/vulnerabilities/<vuln_name>` text files
- **Why missing:** The spec focuses on feature detection, not mitigation status.
  These are separate concepts. A CPU audit card needs both.
- **Type:** `enum MitigationStatus { NotAffected, Mitigated(String), Vulnerable,
  KernelNotProviding }`
- **Examples:** spectre_v1, spectre_v2, meltdown, mds, srbds, retbleed, etc.
- **Audit-card relevance:** Critical for systems where `mitigations=off` is suspected

### Missing Column B: Feature Accessible vs. Feature Present
- **What:** A Boolean distinguishing "CPU advertises feature in CPUID" vs.
  "feature is accessible to kernel and userspace under current firmware/OS config"
- **Why missing:** The spec documents CPUID location but does not distinguish hardware
  support from software access. This matters for SGX (BIOS enable), SEV (firmware key
  management), and AVX-512 (some cloud VMs disable it for scheduler fairness).
- **Type:** `struct FeatureAvailability { advertised: bool, accessible: bool }`

### Missing Column C: Guest-vs-Host Discrepancy Risk
- **What:** A flag indicating whether this feature commonly differs between host and
  guest environments in ways that affect security posture
- **Why missing:** The virtualization column documents behavior but not risk.
  A CPU audit card deployed in a VM needs to flag when it cannot verify feature presence.
- **Type:** `bool` — is this feature commonly hidden or modified by hypervisors?

---

## 5. Cross-Reference with Existing `umrs-platform` Detections

The following shows where CPU feature detection would complement or extend existing signals:

| Existing Signal | CPU Feature Intersection | Gap |
|---|---|---|
| `SignalId::Mitigations` (cmdline) | CPU vulnerability mitigations | Does not read actual mitigation status from sysfs |
| `SignalId::FipsEnabled` | AES-NI, RDRAND, RDSEED | FIPS enforcement does not verify hardware acceleration is present |
| `SignalId::RandomTrustCpu` | RDRAND/RDSEED | Checks whether kernel trusts CPU RNG, but not whether RNG hardware exists |
| `KernelLockdown` | SGX/TDX | Lockdown affects whether SGX enclaves can load — not currently cross-checked |
| None | CPU vulnerability mitigation status | No existing sysfs vulnerability reader |
| None | Confidential computing availability | No existing SEV/SGX/TDX probe |
| None | AES-NI presence | No existing crypto acceleration check |

The highest-value integration point is the vulnerability mitigation sysfs reader.
This complements the existing `Mitigations` cmdline check and provides ground truth
about whether the kernel has actually applied mitigations, regardless of what the
cmdline says.

---

## 6. Proposed Data Structure Overview (for researcher's information)

The researcher does not need to design Rust types — that is the implementation phase.
However, documenting what the corpus needs to support will help the researcher organize
the data correctly.

The corpus should be structured so that each feature entry can be read into approximately:

```
CpuFeatureEntry {
    id:                 canonical name + aliases
    vendor:             Intel | AMD | Both | Any
    category:           one of 12 categories
    purpose:            one-sentence description
    example_instrs:     bounded representative list
    cpuid:              leaf, subleaf, register, bit (per vendor if they differ)
    linux_flag:         /proc/cpuinfo flag name if applicable
    linux_sysfs:        sysfs detection path if applicable
    security_benefit:   one sentence
    performance_benefit: one sentence
    caveats:            known limitations and attack history
    virt_behavior:      passthrough | masked | hypervisor-dependent
    firmware_dep:       none | bios-enable | microcode | both
    audit_relevance:    Critical | Important | Informational
    notes:              free-form
    sources:            list of authoritative references
}
```

The CPUID location and the Linux detection path must both be documented even if the
implementation initially uses only the Linux path. The CPUID data is authoritative
reference material that future implementations may use.

---

## 7. Concerns About Implementation Feasibility

### 7.1 CPUID Without Unsafe
As noted in Section 3.1, direct CPUID access requires unsafe. The researcher should
document this limitation clearly so the implementation phase can address it without
surprises. The `/proc/cpuinfo` route is viable for Phase 0. The `raw_cpuid` crate
(Option C) requires an architectural review decision before it can be considered.

### 7.2 TEE Feature Detection Requires Privileged Paths
SGX `/dev/sgx_enclave`, AMD SEV sysfs parameters, and Intel TDX module parameters
may require elevated privileges or specific kernel configuration. The corpus must
document required capabilities per detection path so the implementation can handle
`PermissionDenied` gracefully (fail-closed, log the inaccessibility as a finding).

### 7.3 ARM Architecture
The spec focuses on x86_64. RHEL 10 supports AArch64. The existing codebase runs on
AArch64 (the development host uses RHEL kernel 6.12 aarch64 as shown in environment).
The corpus should note which columns are architecture-specific. The `flags:` line in
`/proc/cpuinfo` on AArch64 is called `Features:` and the token names differ. If UMRS
must support AArch64, the researcher needs to document ARM equivalents:
- AES: `aes` flag, covers ARMv8 Cryptography Extension
- SHA: `sha2`, `sha512`
- RNG: `rng` (ARMv8.5-RNG extension)
- The concept of SGX/SEV does not apply; ARM TrustZone is the TEE equivalent

---

## 8. Recommendations for the Researcher

In priority order:

1. **For each feature, document both CPUID location AND `/proc/cpuinfo` Linux flag name.**
   Both are needed. CPUID is the hardware truth; `/proc/cpuinfo` is the implementation path.

2. **Expand the Linux Detection column** to include sysfs paths for TEE features.
   `/sys/module/kvm_amd/parameters/sev*`, `/dev/sgx_enclave`, etc. are more reliable
   than CPUID for determining whether a TEE feature is actually accessible.

3. **Add the vulnerability mitigation sysfs paths** as a separate section.
   The files under `/sys/devices/system/cpu/vulnerabilities/` are high-value for any
   CPU audit card and complement the existing posture catalog.

4. **Document the "advertised vs. accessible" distinction** explicitly for SGX, SEV,
   SEV-SNP, TDX, and Key Locker. These require firmware enablement in addition to
   hardware support.

5. **Document ARM equivalents** for at least the crypto acceleration and entropy
   columns, since the project runs on AArch64.

6. **For VAES and AVX-512:** Document that AVX-512 availability is inconsistent in
   cloud VMs (some providers disable it). The corpus must note hypervisor masking
   behavior explicitly.

7. **For RDRAND/RDSEED:** The spec already notes the `random.trust_cpu` signal.
   The corpus should document the FIPS 140-3 perspective: RDRAND is not automatically
   a FIPS-approved entropy source. This directly connects to the existing
   `SignalId::RandomTrustCpu` signal.

8. **Separate "CPU capability" from "mitigation status."** These are two different
   data domains. The feature matrix covers CPU capabilities. Mitigation status
   (from `/sys/devices/system/cpu/vulnerabilities/`) is a separate enumeration that
   deserves its own section in the corpus.

---

## 9. Summary Assessment

The 17-column spec is well-conceived and the columns are implementable. The main issues are:

- **Column 6 (CPUID)** needs a structured representation, not a single string, because
  many features use multiple CPUID locations.
- **Column 7 (Linux detection)** must clearly separate `/proc/cpuinfo` flags from
  authoritative sysfs paths — they are not equivalent.
- **Columns 13 and 14** (firmware and microcode) overlap and can be documented together.
- **Three columns are missing:** active mitigation status, feature accessibility (vs.
  mere advertisement), and guest-vs-host discrepancy risk.
- **CPUID direct access is not possible** in safe Rust without a new crate dependency.
  `/proc/cpuinfo` is the implementation path for Phase 0. This constraint must be
  explicit in the corpus so the implementation team is not surprised.
- **ARM architecture** is not addressed. The development host runs AArch64. The corpus
  should cover at minimum the ARM crypto extension equivalents.

The researcher corpus will be actionable if it populates all 17 columns with concrete,
exact values (not prose descriptions) for columns 6, 7, 12, 13, 14, and 15.

---

## 10. Addendum: Integration with the Existing Posture Catalog Architecture

**Context:** The CPU feature matrix will directly feed a `CpuProbe` following the same
`SignalId` / `SignalDescriptor` / `SIGNALS` pattern already implemented in
`umrs-platform/src/posture/`. This addendum addresses how the 17-column matrix maps to
that architecture, and how to support the two-layer audit model (hardware capability vs.
software utilization).

---

### 10.1 Mapping Matrix Columns to the Existing Signal Architecture

The existing posture catalog uses this data model:

```
SignalId   → typed enum variant (compile-time exhaustive)
SignalClass → how/where the live value is read (Sysctl, KernelCmdline, SecurityFs, etc.)
LiveValue  → what the kernel reports right now
ConfiguredValue → what the configuration layer intends
DesiredValue → what "hardened" looks like
AssuranceImpact → Critical / High / Medium
ContradictionKind → classification when live ≠ configured
```

CPU features map as follows:

| Matrix Column | Signal Architecture Equivalent |
|---|---|
| Feature (col 1) | `SignalId` variant, e.g., `CpuAesNi` |
| Category (col 3) | Groups of `SignalId` variants; no direct field in catalog today |
| Linux flag/detection (col 7) | `live_path` in `SignalDescriptor` + `SignalClass` |
| Audit-card relevance (col 15) | `AssuranceImpact` |
| Security benefit (col 9) | `rationale` field in `SignalDescriptor` |
| NIST controls | `nist_controls` field in `SignalDescriptor` |
| Caveats (col 11) | Does not map directly — currently not in `SignalDescriptor` |
| Virtualization behavior (col 12) | Does not map directly — new field needed |

The `SignalClass` enum needs a new variant for CPU feature signals. The existing variants
are `Sysctl`, `KernelCmdline`, `SecurityFs`, `DistroManaged`, `ModprobeConfig`. CPU
features read from `/proc/cpuinfo` would be a new class. Sysfs paths for TEE features
would extend the `ModprobeConfig` pattern (sysfs-backed, `SYSFS_MAGIC`). Appropriate
new `SignalClass` variants:

```
CpuProcInfo     → /proc/cpuinfo flags line (PROC_SUPER_MAGIC)
CpuSysfs        → /sys/devices/system/cpu/ paths (SYSFS_MAGIC)
CpuVulnSysfs    → /sys/devices/system/cpu/vulnerabilities/ (SYSFS_MAGIC)
TeeDevfs        → /dev/sgx_enclave, /dev/sev (character device, no magic check)
```

**`TeeDevfs` note:** Device nodes under `/dev/` are not pseudo-filesystems with a
predictable `fstatfs` magic. Existence of `/dev/sgx_enclave` is detected via a
`stat()` or `open()` attempt, not a filesystem magic check. This is a different
provenance model than the existing KATTRS pattern and must be documented explicitly
in the corpus.

---

### 10.2 Which CPU Features Should Be `SignalId` Variants vs. Informational-Only

**Should become `SignalId` variants** (security posture signals — absence or
misconfiguration materially changes posture):

| Feature | Proposed `SignalId` | Rationale |
|---|---|---|
| AES-NI | `CpuAesNi` | FIPS crypto acceleration; absent = potential SW fallback |
| RDRAND | `CpuRdrand` | Hardware entropy; relates to `RandomTrustCpu` signal |
| RDSEED | `CpuRdseed` | Stronger entropy source; FIPS 140-3 entropy relevance |
| SGX (accessible) | `CpuSgxEnabled` | TEE capability — BIOS gate, not just CPUID |
| AMD SEV-SNP | `CpuSevSnp` | Confidential VM capability |
| Intel TDX | `CpuTdx` | Confidential VM capability |
| PCLMULQDQ | `CpuPclmulqdq` | AES-GCM authentication acceleration — TLS relevance |
| SHA Extensions | `CpuShaExt` | SHA-1/SHA-256 hardware — hash acceleration |
| CPU mitigations | (per-vuln) | Active per-vulnerability mitigation status |

**Informational only** (useful for display, not hardening signals):

| Feature | Why Informational |
|---|---|
| SSE, SSE2, SSE3, SSSE3, SSE4.x | Baseline on all x86_64 targets; absence is abnormal |
| AVX | Informational context for crypto library performance |
| AVX2 | Performance context; not a security posture signal on its own |
| AVX-512 | Performance context; VM masking is common and not a security finding |
| ADX, BMI1, BMI2 | Big-integer performance; informational |
| Intel Key Locker | Not widely deployed; firmware dependency; informational |
| AMD SME | Memory encryption; relevant if running bare-metal; informational |
| VMX / SVM | Virtualization support; informational for the CPU probe |

The `AssuranceImpact` tiers for the signals that become `SignalId` variants should
be assigned by the security-engineer review, not in this report. The researcher
should flag audit-card relevance as Critical/Important/Informational using the
taxonomy, and the security-engineer will finalize them.

---

### 10.3 Detection Method Alignment with Provenance-Verified Read Patterns

Every detection path must use the provenance-verified read route:

| Detection Source | Existing Pattern | Signal Class |
|---|---|---|
| `/proc/cpuinfo` flags | `ProcfsText` + `SecureReader` + `PROC_SUPER_MAGIC` | `CpuProcInfo` (new) |
| `/sys/devices/system/cpu/vulnerabilities/` | `SysfsText` + `SecureReader` + `SYSFS_MAGIC` | `CpuVulnSysfs` (new) |
| `/sys/module/kvm_amd/parameters/sev*` | `SysfsText` + `SecureReader` + `SYSFS_MAGIC` | `CpuSysfs` (new) |
| `/dev/sgx_enclave` existence | `stat()` attempt — no magic check | `TeeDevfs` (new) |
| Raw CPUID instruction | Not usable in safe Rust without a new dependency | N/A for Phase 0 |

The researcher must document the exact sysfs path and expected value format for
each TEE feature so the implementation can apply the `StaticSource` + `SecureReader`
pattern. If the live path is `/sys/module/kvm_amd/parameters/sev`, the implementer
needs to know: is the value `"1"`, `"Y"`, or a different format? The corpus must
document the exact byte content returned by each path.

---

### 10.4 The Two-Layer Audit Model — Hardware Capability vs. Software Utilization

The CPU probe (Layer 1) detects what the hardware offers. A separate future binary
auditor (Layer 2) detects whether software actually uses those capabilities. The
contradiction between the two layers is the highest-value finding this system can produce.

**Layer 1 — CPU Probe (what this matrix covers):**
- CPU has AES-NI: `CpuAesNi` → `LiveValue::Bool(true)`
- CPU has RDRAND: `CpuRdrand` → `LiveValue::Bool(true)`
- CPU mitigations active: per-vulnerability status from sysfs

**Layer 2 — Software Utilization Audit (future work, but the corpus must support it):**
- OpenSSL compiled with AES-NI support: check ELF binary for `-maes` compilation
  flag evidence, or detect AES-NI instruction opcodes in the `.text` section
- libgcrypt linked to hardware-accelerated routines: check `/proc/crypto` for
  hardware-backed algorithm implementations
- Crypto library seeds PRNG from RDRAND: check library source/build flags or
  `/proc/crypto` entry for `"driver" : "crct10dif-pclmul"` type entries

**Detection methods for Layer 2 that the researcher should document:**

1. **`/proc/crypto`** — The kernel exposes all registered crypto algorithm
   implementations at runtime. Each entry includes the algorithm name, driver name
   (which reveals whether it is hardware-accelerated), and priority. Example:
   `crct10dif-pclmul` indicates the PCLMULQDQ-accelerated CRC implementation is
   registered. This is a `ProcfsText` read and fits the existing pattern cleanly.

2. **ELF inspection** — The `umrs-tui` crate already reads ELF headers for display
   purposes. A deeper ELF audit could examine `.gnu.property` notes for ISA level
   requirements (ISA_LEVEL_2 = AVX-512 required) and check dynamic library
   dependencies via the NEEDED entries. This is a file-based inspection, not a
   kernel attribute read.

3. **`ldd` / dynamic linker inspection** — Reading which shared libraries an
   OpenSSL or libgcrypt binary links against can reveal whether the hardware-
   accelerated provider path is available at link time.

4. **Build metadata (`.note.gnu.build-id`, DWARF, RPATH)** — Package management
   databases (RPM) record compile flags in SRPM metadata, but this is not reliably
   accessible at runtime without querying the RPM DB with specific fields.

**Paired `SignalId` design:** The two-layer model implies paired signals for high-value
features:

```
CpuAesNi          → Layer 1: hardware present
LibraryUsesAesNi  → Layer 2: software uses it (future)
```

The contradiction detection infrastructure already supports this model: if
`CpuAesNi` is `LiveValue::Bool(true)` but `LibraryUsesAesNi` is `LiveValue::Bool(false)`,
the `ContradictionKind` for this case would be a new variant — something like
`CapabilityUnused` — meaning "hardware offers it, software ignores it." The researcher
should be aware this contradiction type needs to be defined.

**`/proc/crypto` is the most actionable Layer 2 path for Phase 1:** It does not
require ELF parsing, does not require privilege escalation, and reads from a
kernel-authoritative source via the existing `ProcfsText` + `SecureReader` route.
The corpus must document `/proc/crypto` format and which algorithm driver names
indicate hardware acceleration for each relevant feature (AES-NI → `aesni_intel` or
`aes_ce` on ARM; PCLMULQDQ → `crct10dif-pclmul`; SHA → `sha256_ssse3` or similar).

---

### 10.5 Updated `DesiredValue` Requirements

The existing `DesiredValue` enum handles numeric exact/threshold comparisons and
cmdline token presence/absence. CPU feature signals need two additional variants:

```
DesiredValue::FlagPresent(&'static str)
    → /proc/cpuinfo flags line must contain this token
    → analogous to CmdlinePresent but for the cpuinfo flags field

DesiredValue::DeviceAccessible(&'static str)
    → device node at this path must be openable (TEE present and enabled)
    → fail-closed: if open() returns PermissionDenied, TEE is present but inaccessible
```

The researcher does not need to implement these, but should document what "meets
desired" means per feature so the implementation can select the correct variant.

---

### 10.6 `SignalDescriptor` Extension Requirements

The existing `SignalDescriptor` struct does not have fields for:
- Virtualization masking behavior
- Firmware/microcode dependency
- Layer 2 detection path (software utilization)

These columns from the matrix (12, 13, 14) would need to be added to
`SignalDescriptor` or captured in a parallel `CpuFeatureDescriptor` type.
Given that CPU features have different semantics from kernel sysctl signals,
a `CpuFeatureDescriptor` type alongside the existing `SignalDescriptor` may
be cleaner than extending the existing type. This is an implementation
decision for when coding begins; the researcher should document all column
values so the implementer has the data regardless of which structure holds it.

---

### 10.7 `/proc/crypto` — Critical Addition to the Corpus

`/proc/crypto` is not mentioned in the cpu-matrix.md or cpu-research.md specs.
It must be added as a mandatory corpus topic. It exposes:
- Every registered crypto algorithm in the kernel
- Which driver handles each algorithm (software vs. hardware)
- Priority values (higher priority drivers win when multiple implementations exist)
- Algorithm properties (type, blocksize, ivsize, digestsize)

For a CPU audit card, `/proc/crypto` is the authoritative answer to "is hardware
acceleration actually registered and active?" — not just "does the CPU support it."
A system with AES-NI in hardware but no `aesni_intel` driver loaded shows AES-NI
in `/proc/cpuinfo` but has no hardware-backed AES in the kernel. `/proc/crypto`
catches this gap.

**Recommendation:** Add `/proc/crypto` documentation as a mandatory section in the
corpus, covering format, driver naming conventions, priority semantics, and the
mapping from hardware feature to expected driver name.

---

## 11. Final Recommendations (Updated)

The following supersedes or extends the recommendations in Section 8:

1. **Document both CPUID location AND `/proc/cpuinfo` flag name per feature.**
   Both layers are needed.

2. **Add `/proc/crypto` as a mandatory corpus section.** This is the Layer 2
   detection path for crypto algorithm utilization and is more authoritative than
   `/proc/cpuinfo` for determining whether hardware acceleration is actually active.

3. **Document sysfs paths for TEE features** including exact path and expected
   value format per feature, so the `StaticSource` pattern can be applied directly.

4. **Classify each feature as `SignalId` variant (posture signal) vs. informational**
   using the recommendations in Section 10.2 as a starting point. Security-engineer
   review will finalize.

5. **Document "hardware accessible" distinction for SGX, SEV, TDX, Key Locker.**
   CPUID may say supported while firmware says disabled.

6. **Document ARM equivalents** (AES, SHA, RDRAND analogues on AArch64).

7. **Expand virtualization behavior column** to include which hypervisors
   (KVM, VMware, Hyper-V) mask or expose each feature, not just "can be masked."

8. **Document the paired signal concept** for the two-layer audit model. The corpus
   should explicitly note which features have a Layer 2 utilization-check path and
   what that path is (primarily `/proc/crypto` for crypto algorithms).

9. **For RDRAND/RDSEED:** Document the FIPS 140-3 perspective on hardware RNG
   trustworthiness and the connection to the existing `SignalId::RandomTrustCpu` signal.

10. **Separate vulnerability mitigation status** from CPU feature presence. The
    `/sys/devices/system/cpu/vulnerabilities/` files are a distinct data domain
    from the feature matrix and should be a separate corpus section that feeds
    a separate probe phase.

---

*End of rust-developer review (including addendum). Ready for researcher agent to begin
corpus construction with the expanded scope above.*
