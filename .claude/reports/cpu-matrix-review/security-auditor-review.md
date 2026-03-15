# Security Auditor Review: CPU Feature Matrix Specification

**Review date:** 2026-03-14
**Reviewer:** security-auditor agent
**Scope:** Pre-research review of cpu-matrix.md, cpu-research.md, cpu-post-research.md
**Depth:** In-depth (spec review, not source code audit)
**Status:** Phase 0 — researcher has NOT yet built the corpus. This review precedes that work.

---

## Executive Summary

The CPU feature matrix specification (cpu-matrix.md) is architecturally sound and
substantially more rigorous than the earlier cpu-research.md seed document. The 17-column
matrix schema, category taxonomy, and feature interpretation rules are a strong foundation.

However, the specification has four material gaps that must be addressed before the researcher
begins building the corpus:

1. **The "Critical" classification definition is ambiguous** in a way that will produce
   inconsistent classifications across the feature set.
2. **An entire category of CPU security features is absent**: speculative-execution mitigation
   capabilities (IBRS, IBPB, STIBP, SSBD, MDS mitigations) and CPU-enforced access controls
   (SMEP, SMAP, CET, PKU, UMIP, NX/XD). These features are security-load-bearing on every
   deployed system and must be in the corpus.
3. **The "compliance relevance" dimension is missing** from the feature interpretation
   framework. For a FIPS-active DoD deployment, RDRAND/RDSEED classification cannot be
   completed without knowing NIST SP 800-90B's position on CPU-sourced entropy.
4. **CVE and vulnerability research guidance is absent** from the authoritative source
   requirements, leaving the researcher without direction on how to handle features with
   known attack history (SGX, SEV, RDRAND).

The sections below address each evaluation area in full.

---

## 1. Audit-Card Relevance Classification

### 1.1 Assessment of the Three-Tier Model

The spec defines three tiers (cpu-matrix.md, "Audit Card Relevance Rules"):

> - **Critical** — absence or disablement materially changes security posture.
> - **Important** — it materially changes performance, hardening, or isolation.
> - **Informational** — it is useful context but not decisive.

**The "Critical" definition has an ambiguity problem.** "Absence materially changes security
posture" is the correct intuition for a mitigations-class feature (e.g., IBRS missing → Spectre
v2 exposure). But for a capability-class feature (e.g., AES-NI missing), the system is not
*insecure* — it is *slower and more susceptible to timing side-channels*. The current definition
does not distinguish these two failure modes.

**Recommended clarification** — add a sub-distinction within Critical:

> - **Critical/Defensive**: absence removes a protection that would otherwise block a known
>   attack class. Spectre/Meltdown mitigations fall here. SMEP/SMAP fall here.
> - **Critical/Operational**: absence requires a fallback path that introduces timing risk,
>   audit surface, or FIPS compliance risk. AES-NI and RDRAND fall here on FIPS-active systems
>   because FIPS-validated software fallbacks exist, but the fallback's performance may cause
>   operational degradation that a DoD operator must know about.

Without this sub-distinction, a researcher will not know whether to classify AES-NI as Critical
(FIPS argument) or Important (pure performance argument). The classification will be
inconsistent across features.

### 1.2 Feature-Specific Classification Issues

The following features in the current corpus seed are likely to be misclassified under the
current definition:

**RDRAND and RDSEED — currently unclassified, likely to be classified Important.**
These should be **Critical** for FIPS deployments. NIST SP 800-90B requires that entropy
sources used by a FIPS module be assessed for health and quality. The existing posture catalog
already tracks `random.trust_cpu` as a posture signal (catalog.rs, SignalId::RandomTrustCpu)
and cites NIST SP 800-90B. The CPU corpus must be consistent: if the OS-level audit already
treats CPU RNG trust as a compliance finding, the CPU feature matrix must treat RDRAND/RDSEED
as Critical.

**Speculative execution mitigation features — absent from spec, would likely be Informational
if left to default.**
These must be Critical/Defensive. The posture catalog already tracks `mitigations=off` absence
as a Critical signal (SignalId::Mitigations, AssuranceImpact::Critical). The CPU corpus must
document the underlying CPU capabilities that make those mitigations possible. A CPU without
IBRS microcode support cannot implement IBRS even if the kernel requests it. This gap is
security-load-bearing.

**SGX — listed as "trusted execution," likely to be classified Critical.**
This classification requires caution. SGX has a substantial published attack history
(SGAxe, CrossTalk, LVI, Foreshadow). On a server platform running government workloads,
SGX is more likely to be a **threat surface to monitor** than a hardening capability. The
spec's feature interpretation rules (cpu-matrix.md, "Feature Interpretation Rules") ask
"Does it introduce new trust assumptions?" for each feature, which is correct — but the
audit-card relevance classification should explicitly reflect that SGX's relevance tier
depends on whether the system *uses* SGX enclaves. On a system where SGX is present but
not used, the correct posture finding is: "SGX present, ensure BIOS-disabled if not in use."
Recommend classifying SGX as **Important** with a specific note that presence without use
is an audit surface, not an assurance gain.

### 1.3 Missing "Disabled by Default" Consideration

The spec asks whether features can be "disabled, hidden, or virtualized away" but does not
ask the inverse: **should this feature be disabled if not in use?** This is an important
audit-card dimension for DoD systems. Features like SGX, IOMMU virtualization passthrough,
and SME/TMME have known misuse or misconfiguration attack vectors. The corpus should document
for each feature: "If this feature is present but not required by deployed workloads, what is
the recommended BIOS/kernel disposition?"

**Actionable addition for the spec:** Add a column or mandatory research section item:
"Recommended disposition when present but unused." This applies to at minimum: SGX, TDX,
Key Locker, VMX/SVM on non-hypervisor systems.

---

## 2. Missing Security-Relevant Extensions

This is the most significant gap in the specification. The corpus seed focuses almost
entirely on *capabilities that improve cryptographic performance or enable trusted execution*.
It is missing the category of *CPU-enforced access controls and speculative-execution
mitigations*, which are equally relevant to security posture on RHEL 10.

### 2.1 Speculative Execution Mitigations — Missing Entirely

The existing posture catalog tracks `mitigations=off` as a Critical cmdline signal. But the
*reason* that flag matters is that the underlying CPU must support specific mitigation
mechanisms. Without the CPU supporting these mechanisms in microcode, the kernel mitigation
is a no-op. The corpus must document:

**IBRS (Indirect Branch Restricted Speculation)**
- Spectre v2 mitigation
- Requires microcode support; enhanced IBRS (eIBRS) is preferred on newer CPUs
- Linux exposes as `ibrs`, `enhanced_ibrs` in `/proc/cpuinfo`
- CPUID: leaf 0x7, subleaf 0, EDX bit 26 (IBRS_AND_IBPB), EDX bit 29 (STIBP)
- Classification: **Critical/Defensive** — without this, Spectre v2 cross-process
  attacks are possible

**IBPB (Indirect Branch Predictor Barrier)**
- Flushes the branch predictor on context switch
- Required for full Spectre v2 isolation between processes with different security labels
- On SELinux MLS systems (which is the UMRS target), cross-domain branch predictor
  contamination is a label boundary violation
- Classification: **Critical/Defensive**

**STIBP (Single Thread Indirect Branch Predictors)**
- Prevents SMT siblings from sharing indirect branch predictions
- Required for isolation on hyperthreaded systems
- Classification: **Critical/Defensive** on SMT systems

**SSBD (Speculative Store Bypass Disable)**
- Spectre v4 mitigation
- Requires MSR support; exposed in Linux as `ssbd` cpuinfo flag
- Classification: **Critical/Defensive**

**MDS mitigations (Microarchitectural Data Sampling)**
- VERW-based MDS/TAA/SRBDS mitigation
- Requires microcode
- Linux tracks as `md_clear` and `taa` vulnerability status in
  `/sys/devices/system/cpu/vulnerabilities/`
- Classification: **Critical/Defensive**

**Note on the vulnerability sysfs interface:** `/sys/devices/system/cpu/vulnerabilities/`
exposes per-vulnerability mitigation status as text strings (e.g., "Mitigation: Retpoline").
This is a critical detection path that the spec does not mention anywhere. The researcher
must document this interface as an authoritative Linux detection source for all speculative
execution vulnerability status. It is more direct than `/proc/cpuinfo` for this purpose.

### 2.2 CPU-Enforced Access Controls — Missing Entirely

**SMEP (Supervisor Mode Execution Prevention)**
- Prevents the kernel from executing user-space pages
- Kills the "ret2usr" exploit class
- Linux exposes as `smep` in `/proc/cpuinfo`
- CPUID: leaf 0x7, subleaf 0, EBX bit 7
- Classification: **Critical/Defensive** — absence means kernel can be redirected
  to execute user-space shellcode

**SMAP (Supervisor Mode Access Prevention)**
- Prevents the kernel from accessing user-space memory unless explicitly unlocked
  (CLAC/STAC instructions)
- Kills an entire class of "userland pivot" kernel exploits
- Linux exposes as `smap` in `/proc/cpuinfo`
- CPUID: leaf 0x7, subleaf 0, EBX bit 20
- Classification: **Critical/Defensive**

**CET (Control-flow Enforcement Technology)**
- Two components: Shadow Stack (SS) and Indirect Branch Tracking (IBT)
- Shadow Stack: maintains a separate, CPU-protected stack of return addresses;
  any ROP chain that corrupts the call stack triggers a fault
- IBT: enforces that indirect jumps land on `ENDBR32`/`ENDBR64` instructions
- RHEL 10 ships CET-enabled kernel and user-space on supporting hardware
- This is directly relevant to UMRS's target deployment (RHEL 10)
- CPUID: leaf 0x7, subleaf 0, ECX bit 7 (CET_SS), ECX bit 20 (IBT/CET_IBT)
- Linux: `shstk`, `ibt` in `/proc/cpuinfo`; kernel compiled with
  `CONFIG_X86_SHADOW_STACK` and `CONFIG_X86_IBT`
- Classification: **Critical/Defensive** — on RHEL 10 this is a first-class
  hardening capability; its presence must be verified on all target hardware

**UMIP (User Mode Instruction Prevention)**
- Prevents user-space from executing `SGDT`, `SIDT`, `SLDT`, `SMSW`, `STR`
  instructions that expose kernel descriptor table addresses
- Kills a class of KASLR-bypass information leaks
- Linux exposes as `umip` in `/proc/cpuinfo`
- CPUID: leaf 0x7, subleaf 0, ECX bit 2
- Classification: **Important** — complements KASLR hardening

**NX/XD (No-Execute / Execute Disable)**
- Marks data pages as non-executable at the hardware level
- Prerequisite for all W^X (write XOR execute) enforcement
- On RHEL 10 this is baseline assumed-present; on any system where it is absent,
  the entire exploit mitigation stack collapses
- Linux: `nx` in `/proc/cpuinfo`
- Classification: **Critical/Defensive** — foundational; audit cards must confirm
  it is present and not disabled by firmware

**PKU (Protection Keys for User-space)**
- Allows per-page user-space access protection without system calls
- Used by some memory-safe runtimes and sandbox implementations
- CPUID: leaf 0x7, subleaf 0, ECX bit 3
- Classification: **Important** — useful context for software that leverages it

### 2.3 Platform Security Features — Partially Missing

**Intel TME (Total Memory Encryption)**
- Full platform memory encryption, unlike AMD SME which is in the spec
- Intel's answer to AMD SME
- Should be added alongside SME for vendor completeness

**AMD TMME (Transparent Machine Memory Encryption)**
- AMD extension of SME; covers more memory contexts
- Should be documented alongside SME

**Intel TXT (Trusted Execution Technology)**
- Measured launch environment; complementary to Secure Boot
- Distinct from TDX (which is in the spec); TXT is for physical-platform attestation
- On RHEL 10 government deployments, TXT and Secure Boot together form the measured
  boot attestation chain
- Classification: **Important** — relevant to boot integrity posture

**IOMMU-relevant CPU features**
- The spec mentions IOMMU/DMA-isolation in category 12 but lacks specifics
- VT-d (Intel) and AMD-Vi (AMD) are CPU/platform features that must be enabled
  both in BIOS and recognized by the kernel
- Linux: `dmar` in kernel cmdline or `/proc/cmdline`; `/sys/class/iommu/`
- Classification: **Critical/Defensive** — Thunderbolt and FireWire blacklisting
  (already in the posture catalog as High signals) are only effective if IOMMU
  is active; without IOMMU, DMA attacks bypass kernel-level blacklisting entirely

---

## 3. Feature Interpretation Framework Completeness

### 3.1 Missing Dimension: Compliance Relevance

The current interpretation framework (cpu-matrix.md, "Feature Interpretation Rules") asks:

1. Performance only?
2. Security only?
3. Both?
4. New trust assumptions?
5. Firmware/microcode dependency?
6. Can it be disabled/hidden/virtualized?

This framework is good for general-purpose analysis. For a FIPS-active DoD deployment,
it is incomplete. A seventh question must be added:

> **7. Does this feature have compliance-specific behavior, requirements, or restrictions
>    under FIPS 140-2/3, NIST SP 800-90B, CC, or CMMC?**

The implications are concrete:

- **RDRAND/RDSEED:** NIST SP 800-90B Section 2.1 requires that entropy sources be validated.
  RDRAND is not unconditionally NIST SP 800-90B validated across all CPU steppings.
  The posture catalog already flags `random.trust_cpu=on` as a Medium-impact finding
  because "trusting CPU RNG unconditionally may not satisfy NIST SP 800-90B."
  The CPU corpus must explain *why* and document which RDRAND implementations carry
  SP 800-90B-compliant health tests.

- **AES-NI:** FIPS 140-3 requires that the cryptographic module (in this case, OpenSSL
  or the kernel's crypto subsystem) claim specific algorithm implementations. On RHEL 10
  in FIPS mode, AES-NI acceleration is used by the FIPS-validated OpenSSL build.
  The corpus must document the dependency chain: AES-NI present → kernel exposes it →
  FIPS-validated OpenSSL uses it → FIPS mode is correctly accelerated.

- **SGX enclaves:** Common Criteria evaluations of SGX enclaves have specific requirements.
  A DoD system hosting an enclave-based workload must understand CC evaluation status.

### 3.2 Missing Dimension: Virtualization Masking Detection Confidence

The spec asks for "virtualization behavior" per feature, which is correct. However, it
does not ask: **how confident can a detection algorithm be that a reported flag reflects
hardware reality vs. hypervisor emulation?** This is an operational question for the
audit card.

On a VM guest, `/proc/cpuinfo` reports what the hypervisor chooses to expose. A guest may
report `aes` (AES-NI) because the hypervisor passes it through, or because the hypervisor
emulates it in software. The performance and security implications differ dramatically.

The corpus must document, for each feature:
- Whether the feature can be meaningfully forwarded by a hypervisor (passthrough)
- Whether a hypervisor can expose the flag without hardware support (emulation)
- Whether Linux provides any mechanism to distinguish passthrough from emulation
  (e.g., comparing CPUID results with hypervisor-level attestation)

This dimension directly supports the audit card's requirement to show an "evidence chain
used to validate each conclusion" (cpu-matrix.md, CPU Audit Card View Model).

### 3.3 Missing Dimension: Microcode Staleness Risk

The spec asks for "microcode dependency" per feature. This is correct but needs to go
further. For speculative-execution mitigations specifically, microcode *version* matters.
A CPU may have the correct CPUID bits set (IBRS enumerated) but the installed microcode
may be the pre-mitigation version that does not actually implement IBRS correctly.

The corpus must document:
- For each mitigation-class feature: whether the correct behavior requires a specific
  minimum microcode revision
- How Linux exposes microcode version (via `dmesg`, `/proc/cpuinfo`'s `microcode` field,
  and `/sys/devices/system/cpu/cpu0/microcode/version`)
- Whether `/sys/devices/system/cpu/vulnerabilities/` reflects the post-microcode-update
  status correctly

### 3.4 SMT (Hyperthreading) as a Cross-Cutting Concern

The spec does not mention SMT at all. SMT is not a CPU extension in the traditional sense,
but it is a security-relevant CPU topology property that affects the interpretation of
several features in the corpus:

- STIBP only matters on SMT systems
- MDS vulnerabilities (RIDL, Fallout) are significantly worse on SMT systems
- Some deployments disable SMT entirely for high-assurance isolation
- Linux exposes SMT status via `/sys/devices/system/cpu/smt/active`
- The kernel cmdline `nosmt` and `mitigations=nosmt` are related posture signals

The corpus should include SMT as a platform topology property with security implications,
not a feature in the usual sense.

---

## 4. Authoritative Source Requirements

### 4.1 Government and Standards Sources — Missing

The spec's authoritative source pack (cpu-matrix.md, "Authoritative Source Pack") covers:
- Intel SDM
- AMD Architecture Programmer's Manual
- Linux kernel documentation
- AMD SEV documentation

For a DoD/FIPS deployment, several government and standards sources are missing:

**NIST SP 800-90B — Recommendation for the Entropy Sources Used for Random Bit Generation**
- Required for RDRAND/RDSEED classification and FIPS compliance posture
- Available at: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf
- The corpus cannot correctly classify entropy features without this document

**NIST SP 800-155 — BIOS Integrity Measurement Guidelines**
- Relevant to: measured boot, TXT, UEFI Secure Boot attestation
- Informs how firmware dependency findings should be classified

**NIST SP 800-193 — Platform Firmware Resiliency Guidelines**
- Relevant to: microcode update chain, firmware dependency for mitigations
- Required for correctly classifying microcode dependency risk

**NSA RHEL Security Configuration Guides**
- NSA publishes RHEL hardening guidance that references specific CPU features
- Relevant to: which features NSA recommends verifying in a RHEL deployment
- Provides government-sourced classification support for audit card relevance

**DoD STIGs for RHEL 10**
- DISA STIGs specify which kernel parameters must be set, which implicitly depends
  on CPU capabilities
- The STIG for RHEL 10 references CPU mitigations; the corpus should note which
  STIG controls have CPU-capability prerequisites

### 4.2 CVE and Vulnerability Research — Missing

The spec's secondary source pack mentions "academic papers" for side-channel concerns and
SGX/SEV/TDX attack history. This is correct but needs to be more specific and actionable.
The researcher needs explicit direction on:

**Per-feature vulnerability research requirements:**

For each feature with known CVE history, the corpus must include:
- A table of significant CVEs affecting the feature
- The CVE's impact on the feature's security claim
- Whether the CVE was fixed by microcode, firmware, or kernel patch
- The CVSS score range observed

This applies at minimum to:
- SGX (Foreshadow/L1TF, SGAxe, CrossTalk, LVI, AEPIC Leak)
- RDRAND (CVE-2019-11090 — the AMD RDRAND bug; Intel stepping-specific issues)
- AMD SEV/SEV-SNP (CacheWarp, CVE-2021-26311 and related)
- Intel TDX (early-version implementation issues)
- AES-NI (cache-timing attacks in software fallback paths, not the hardware itself)

**Recommended source for CVE research:**
- NVD (https://nvd.nist.gov/) — search per feature name
- Intel Product Security Center (https://www.intel.com/content/www/us/en/security-center)
- AMD Product Security (https://www.amd.com/en/resources/product-security.html)

Without CVE tables, the corpus will assert security properties of features (e.g., "SGX
provides hardware-isolated execution") that the research community has substantially
qualified or refuted in specific scenarios.

### 4.3 Intel CET Documentation — Missing

Intel CET is absent from the spec and therefore absent from the source requirements.
The researcher needs:
- Intel CET specification document (available from Intel developer documentation)
- Linux CET kernel documentation (`Documentation/x86/cet.rst`)
- RHEL 10 CET userspace enablement status

### 4.4 CPUID Instruction Reference — Source Precision

The spec references Intel SDM volumes 1, 2, and 3. For CPUID specifically, the authoritative
location is **Volume 2A, Chapter 3 ("CPUID — CPU Identification")**. The spec should direct
the researcher to this specific chapter rather than leaving them to search the three-volume set.

Similarly, the AMD equivalent is **AMD64 Architecture Programmer's Manual Volume 3,
Appendix E ("Obtaining Processor Information via the CPUID Instruction")**.

---

## 5. Actionable Feedback for the Researcher Agent

The following items are structured feedback in the format the post-research review procedure
(cpu-post-research.md, Stage 6) requires.

### 5.1 New Extensions to Research

Add these extensions to the corpus, with full mandatory research section treatment:

**Category: Speculative Execution Mitigations (new category)**
- IBRS (Indirect Branch Restricted Speculation) / eIBRS
- IBPB (Indirect Branch Predictor Barrier)
- STIBP (Single Thread Indirect Branch Predictors)
- SSBD (Speculative Store Bypass Disable)
- MDS mitigation capabilities (VERW support, MD_CLEAR)
- L1TF / Foreshadow mitigation (L1D flush)
- Detection path: `/sys/devices/system/cpu/vulnerabilities/` — document every file
  in that directory and its interpretation

**Category: CPU-Enforced Access Controls (new category)**
- SMEP (Supervisor Mode Execution Prevention)
- SMAP (Supervisor Mode Access Prevention)
- CET-SS (Control-flow Enforcement: Shadow Stack)
- CET-IBT (Control-flow Enforcement: Indirect Branch Tracking)
- UMIP (User Mode Instruction Prevention)
- NX/XD (No-Execute / Execute Disable) — confirm it is exposed and enabled
- PKU (Protection Keys for User-space) — informational

**Category: Platform Attestation (additions to existing category)**
- Intel TXT (Trusted Execution Technology) — distinct from TDX
- Intel TME (Total Memory Encryption) — complement AMD SME
- VT-d / AMD-Vi IOMMU — CPU-side feature enabling IOMMU; critical for DMA protection

**Platform Topology**
- SMT (Simultaneous Multithreading) state — `/sys/devices/system/cpu/smt/active`
- Microcode version — `/proc/cpuinfo` `microcode` field,
  `/sys/devices/system/cpu/cpu0/microcode/version`

### 5.2 New Mandatory Research Sections Per Feature

Add these sections to the mandatory research format (cpu-matrix.md,
"Mandatory Research Sections Per Feature"):

- **Recommended disposition when present but unused** — BIOS/kernel action to take if
  the feature exists but the deployment does not use it
- **CVE summary** — significant CVEs, impact, fix mechanism, whether microcode or kernel
  patch required
- **Compliance-specific requirements** — FIPS, NIST SP 800-90B, CMMC, or CC implications
- **Virtualization confidence** — can a guest reliably know this is hardware-backed?

### 5.3 New Authoritative Sources to Ingest

- NIST SP 800-90B (entropy source requirements) — required before RDRAND/RDSEED are classified
- Intel CET specification + Linux `Documentation/x86/cet.rst`
- Linux `/sys/devices/system/cpu/vulnerabilities/` documentation
- NSA RHEL hardening guidance (most recent available version)
- Intel Product Security Center — for per-feature CVE history
- AMD Product Security page — for SEV/SEV-SNP CVE history
- Intel SDM Volume 2A, Chapter 3 (CPUID reference) — replace generic SDM citation with
  specific location
- AMD APM Volume 3, Appendix E (CPUID reference) — same

### 5.4 Classification Corrections for Initial Priority List

The initial prioritization list (cpu-matrix.md, "Initial Prioritization") should be
supplemented. The prioritized features are all capability-class features (crypto
acceleration, TEE). Add the following to the top of the priority list because they
are the most directly tied to posture signals already in the system:

1. SMEP / SMAP — directly relevant to mitigations posture signal
2. CET (Shadow Stack + IBT) — RHEL 10 first-class hardening feature on target hardware
3. IBRS / IBPB / STIBP / SSBD — required to understand what `mitigations=off` actually
   disables on the target CPU
4. `/sys/devices/system/cpu/vulnerabilities/` interface — detection reference for all
   of the above
5. NX/XD — foundational; must be confirmed present before assurance posture can be stated

### 5.5 Matrix Column Addition

Add one column to the 17-column schema:

**Column 18: Recommended disposition when present but unused**
- Values: "Disable in BIOS", "Disable via kernel cmdline", "Monitor only",
  "Leave enabled — no attack surface when idle", "N/A (not a discrete feature)"

This column is required for audit-card actionability on DoD systems.

---

## 6. Consistency With Existing Posture System

The following observations connect the CPU corpus spec to the existing posture catalog
and signal taxonomy, which is the integration target for this work.

### 6.1 Mitigations Signal Already Exists — Corpus Must Support It

`catalog.rs` line 281-291: `SignalId::Mitigations` is a Critical cmdline signal that checks
`mitigations=off` is absent. Its rationale is "CPU vulnerability mitigations (Spectre,
Meltdown, MDS, etc.) must not be disabled." The corpus must explain what CPU capabilities
make each of those mitigations possible. This is the primary integration point.

### 6.2 PTI Signal Already Exists — Corpus Must Explain It

`catalog.rs` line 293-303: `SignalId::Pti` checks that `pti=off` is absent. PTI is the
Meltdown mitigation. The corpus must document the CPU PCID feature (Process Context
Identifiers) because `PCID` is what makes PTI low-overhead — without PCID, PTI has a
significant performance cost that drove some operators to disable it incorrectly.

### 6.3 RandomTrustCpu Signal Already Exists — RDRAND Must Be Critical

`catalog.rs` line 305-318: `SignalId::RandomTrustCpu` flags `random.trust_cpu=on` as
Medium-impact and cites NIST SP 800-90B. The corpus must treat RDRAND classification with
full NIST SP 800-90B awareness. This directly determines whether RDRAND is Important or
Critical on FIPS systems.

### 6.4 FIPS Signal Already Exists — AES-NI Dependency Chain Must Be Documented

`catalog.rs` line 332-341: `SignalId::FipsEnabled` is Critical, cites SC-13, FIPS 140-2/3,
and CMMC SC.L2-3.13.10. On FIPS systems, AES-NI presence is not merely a performance
benefit — it determines which validated code path OpenSSL takes. The corpus must document
this dependency chain so the audit card can correctly present AES-NI as compliance-relevant.

---

## 7. Summary of Findings

### Area 1: Classification Definitions

| Finding | Severity | Action Required |
|---------|----------|-----------------|
| "Critical" definition does not distinguish defensive vs. operational failure modes | HIGH — will produce inconsistent classifications | Add sub-distinction; revise definition before researcher begins |
| RDRAND/RDSEED classification guidance absent; will default to Important | HIGH — inconsistent with posture catalog which already treats CPU RNG trust as NIST SP 800-90B compliance matter | Add NIST SP 800-90B to source requirements; specify Critical for FIPS deployments |
| SGX classification likely to be Critical without caveat about attack history | MEDIUM — misleads audit card consumers | Add explicit caveat: presence-without-use is an attack surface finding, not an assurance gain |
| "Recommended disposition when unused" dimension missing | MEDIUM — audit card will lack actionable guidance for features that should be disabled | Add as Column 18 and mandatory research section |

### Area 2: Missing Extensions

| Extension | Security Relevance | Proposed Classification | Priority |
|-----------|-------------------|------------------------|----------|
| SMEP | Kills ret2usr exploit class | Critical/Defensive | Highest |
| SMAP | Kills userland pivot kernel exploits | Critical/Defensive | Highest |
| CET Shadow Stack | ROP chain prevention; RHEL 10 active | Critical/Defensive | Highest |
| CET IBT | Indirect branch hardening; RHEL 10 active | Critical/Defensive | Highest |
| IBRS/eIBRS | Spectre v2 mitigation | Critical/Defensive | Highest |
| IBPB | Spectre v2 cross-process isolation | Critical/Defensive | Highest |
| STIBP | SMT sibling isolation | Critical/Defensive | High |
| SSBD | Spectre v4 (store bypass) mitigation | Critical/Defensive | High |
| MDS / MD_CLEAR | MDS/TAA/SRBDS mitigation | Critical/Defensive | High |
| NX/XD | Non-executable pages; foundational W^X | Critical/Defensive | Highest |
| UMIP | KASLR-bypass information leak prevention | Important | Medium |
| PKU | Userspace protection keys | Important | Low |
| Intel TXT | Measured boot / platform attestation | Important | Medium |
| Intel TME | Full platform memory encryption | Important | Medium |
| VT-d / AMD-Vi | IOMMU; enables DMA protection | Critical/Defensive | High |
| SMT topology | Affects STIBP and MDS exposure | Important | Medium |
| Microcode version | Mitigation correctness prerequisite | Important | High |
| `/sys/devices/system/cpu/vulnerabilities/` | Primary mitigation detection path | N/A (detection method) | Highest |

### Area 3: Framework Gaps

| Gap | Finding | Action |
|-----|---------|--------|
| Compliance dimension missing | FIPS/NIST SP 800-90B/CMMC implications unresearched | Add "compliance-specific requirements" to mandatory sections |
| Virtualization confidence missing | Cannot distinguish hardware passthrough from emulation | Add confidence level to virtualization column |
| Microcode staleness risk underdocumented | Microcode version required for mitigation correctness | Add microcode version to mandatory sections |
| SMT cross-cutting concern absent | SMT affects STIBP, MDS, and isolation posture | Add SMT as platform topology entry |

### Area 4: Source Gaps

| Missing Source | Required For | Priority |
|----------------|-------------|----------|
| NIST SP 800-90B | RDRAND/RDSEED compliance classification | Critical — without this RDRAND classification is guesswork |
| Intel CET specification | CET research | High |
| Linux `Documentation/x86/cet.rst` | CET detection paths | High |
| NSA RHEL hardening guidance | Audit card relevance alignment | Medium |
| Intel Product Security Center | SGX, TME CVE history | High |
| AMD Product Security | SEV/SNP CVE history | High |
| NIST SP 800-155 | Firmware/measured boot | Medium |
| NIST SP 800-193 | Microcode update chain resilience | Medium |

---

## 8. The Library Utilization Audit Layer — A Material Gap in Scope

This section addresses a dimension of the CPU feature matrix that the specification does not
contemplate at all, but which is the most important operational use case for this corpus:
**auditing whether deployed software actually uses the CPU features that are available.**

A CPU feature being present is a necessary but not sufficient condition for security posture.
The security finding emerges at the intersection of hardware capability and software
utilization. The corpus must support both layers.

### 8.1 Why This Changes Feature Classification

The two-layer model changes how features must be classified:

**A feature's audit-card relevance is higher when its non-use by software constitutes a
security or compliance finding.** Under the single-layer model (hardware only), AES-NI might
be classified Important (performance benefit). Under the two-layer model, if the system is
in FIPS mode and OpenSSL was compiled without AES-NI support, the fallback is a software
AES implementation that:

1. Has measurably worse timing characteristics (timing side-channel exposure)
2. May or may not be FIPS 140-3 validated in its software form
3. Will be significantly slower, potentially causing operational degradation that
   operators "fix" by disabling FIPS mode

This makes AES-NI **Critical** on FIPS systems — not because AES-NI is a security control
itself, but because its absence in software on a FIPS-active system creates a compliance
integrity problem.

The same logic applies across the feature set:

| Hardware Feature | Software Utilization Finding |
|-----------------|------------------------------|
| AES-NI present | OpenSSL compiled without `-DOPENSSL_IA32_SSE2` / no `aesni` in /proc/crypto → HIGH finding |
| RDRAND present | Kernel or crypto library ignoring it; only using /dev/urandom → MEDIUM finding |
| SHA extensions | TLS stack (OpenSSL) not using SHA-NI; CPU has it but software uses software SHA → MEDIUM finding |
| PCLMULQDQ | AES-GCM authentication tag computation falling back to software GF multiply → MEDIUM finding |
| AVX-512/VAES | High-throughput crypto workload using SSE2 path when VAES available → LOW-MEDIUM finding |
| CET (Shadow Stack) | Binary compiled without `-fcf-protection=full`; CET enabled in kernel but binary opts out → HIGH finding |
| SMEP/SMAP | Kernel build without `CONFIG_X86_SMEP` / `CONFIG_X86_SMAP` on capable hardware → HIGH finding (this is a build-time decision) |

### 8.2 Authoritative Sources for Utilization Verification

The spec's source pack does not address library compilation and utilization verification.
The researcher must ingest:

**OpenSSL compilation and capability verification:**
- OpenSSL `OPENSSL_ia32cap` environment variable documentation — controls which CPU
  extensions OpenSSL uses at runtime; a misconfigured value can silently disable hardware
  acceleration on FIPS systems
- OpenSSL engine and provider documentation — on RHEL 10 in FIPS mode, the FIPS provider
  must use validated code paths; confirm which code paths use AES-NI
- `/proc/crypto` — the Linux kernel crypto API registration table. Each registered
  algorithm entry shows driver name, module, type, and priority. Entries like `aes-aesni`
  confirm hardware-backed AES; entries like `aes-generic` indicate software fallback.
  This is a first-class detection source that the spec does not mention anywhere.
- OpenSSL FIPS validation certificate (from CMVP/NIST) — specifies which algorithm
  implementations are validated; this determines whether the software-AES fallback is
  itself FIPS-validated or not

**Linux kernel crypto subsystem:**
- `Documentation/crypto/` in the kernel source tree — explains the crypto API,
  algorithm registration, and priority system
- `crypto_find_alg()` semantics — higher-priority (hardware) implementations
  preempt lower-priority (software) ones; the corpus must document how to verify
  that the hardware implementation won the priority race

**FIPS validation certificate specifics:**
- CMVP Module Search: https://csrc.nist.gov/projects/cryptographic-module-validation-program
- Search for "Red Hat Enterprise Linux 10" — the FIPS module certificate specifies
  exactly which algorithms and implementations are validated
- This is the authoritative answer to "is the software-AES fallback FIPS-validated?"

**CET utilization verification:**
- `/proc/self/status` — `MemSigArm` field (or equivalent) for userspace CET state
- `checksec` tool or equivalent — checks ELF binary headers for CET compatibility flags
  (`IBT`, `SHSTK` in `.note.gnu.property`)
- RHEL 10 security advisories on which system libraries were rebuilt with CET support

### 8.3 New Matrix Columns for Utilization Audit

The 17-column schema must be extended to support the utilization audit layer. Add:

**Column 18: Recommended disposition when present but unused** (as previously proposed)

**Column 19: Linux software utilization detection method**
- How to verify that software is actually using this feature at runtime
- Sources: `/proc/crypto`, `openssl speed`, `lstopo`, kernel dmesg on boot,
  `/sys/kernel/debug/` (if debugfs mounted), `lscpu` flags

**Column 20: FIPS utilization requirement**
- Whether a FIPS-active system has a compliance obligation to use this feature when present
- Values: "Required by FIPS validation certificate", "Required by NIST SP 800-90B",
  "Recommended but not mandated", "Not applicable", "Unknown — research required"

### 8.4 The /proc/crypto Detection Interface

`/proc/crypto` is a critical Linux interface that the spec does not mention. It must be
added to the Linux Detection Research Requirements section.

Each entry in `/proc/crypto` shows:
```
name         : sha256
driver       : sha256-avx2
module       : sha256_ssse3
priority     : 170
refcnt       : 1
selftest     : passed
fips_allowed : yes
type         : shash
blocksize    : 64
digestsize   : 32
```

The `driver` field distinguishes hardware-accelerated from software implementations.
The `fips_allowed` field is directly relevant to the FIPS posture signal.
The `selftest` field indicates whether the kernel's built-in crypto self-test passed,
which is required for FIPS operation.

For the audit card, the correct posture check is: for each CPU extension that provides
hardware-accelerated cryptography, verify that the corresponding hardware driver appears
in `/proc/crypto` with `priority` higher than the software fallback, and `selftest: passed`.

This transforms the CPU audit card from "does the hardware have AES-NI?" to "does the
hardware have AES-NI AND is the system actually using it via a FIPS-allowed driver?"

### 8.5 CET Binary Audit Dimension

CET deserves special attention in the utilization context. Intel CET requires:
1. CPU hardware support (CPUID)
2. Kernel support (compiled with `CONFIG_X86_SHADOW_STACK`, `CONFIG_X86_IBT`)
3. Per-binary opt-in (compiled with `-fcf-protection=full`)

A binary compiled without CET support that loads on a CET-capable system will have CET
disabled for that process's execution domain. The kernel logs this for each executable that
opts out. On RHEL 10, this is visible in the audit log and via `/proc/<pid>/status`.

For the audit card, the utilization check is: UMRS itself and all linked libraries must
carry the CET-compatible ELF notes. If UMRS is the security audit tool, it must not itself
be an example of the vulnerability it is auditing for.

**Actionable recommendation:** When the researcher documents CET, include the verification
method for checking whether a specific binary (or all system libraries) carry CET-compatible
ELF headers. The `eu-readelf -n <binary>` or `objdump -p <binary>` commands expose the
`.note.gnu.property` section that carries `GNU_PROPERTY_X86_FEATURE_1_AND` with the
`IBT` and `SHSTK` bits.

### 8.6 "Compiled Without Hardware Acceleration" as an Attack Surface

The spec's feature interpretation framework asks whether a feature introduces new attack
surface. The utilization audit layer adds the inverse question: **does a missing or
unclaimed feature in software create an attack surface?**

For security-relevant primitives:
- A software AES implementation is vulnerable to cache-timing attacks that AES-NI
  eliminates. If the library compiled without AES-NI support falls back to software AES
  on a FIPS system, the system is potentially exposing a timing oracle that a hardware
  implementation would not.
- A software SHA-256 implementation on a CPU with SHA extensions is slower but also
  potentially less constant-time depending on the implementation. Library authors rely
  on hardware SHA being inherently constant-time; the software fallback may not be.

This is not a hypothetical concern — it is documented in OpenSSL security advisories and
in academic literature on timing attacks against TLS.

**Recommended addition to the mandatory research sections per feature:**
Add a "Software fallback risk" section that documents:
- Whether the software fallback for this feature is constant-time
- Whether the software fallback has CVE history related to timing
- Whether FIPS validation applies to the software fallback

---

## 9. Recommendation to Researcher Agent

Do not begin corpus ingestion until the spec is updated to:

1. Revise the Critical/Important classification definitions with the sub-distinction
   between defensive and operational failure modes.
2. Add the speculative-execution mitigation features and CPU-enforced access controls
   listed in Section 5.1 as a mandatory research category.
3. Add NIST SP 800-90B to the authoritative source pack. This is blocking for RDRAND.
4. Add the "compliance-specific requirements" and "CVE summary" mandatory sections to
   the per-feature research format.
5. Add three columns to the matrix schema: Column 18 (recommended disposition when
   unused), Column 19 (software utilization detection method), Column 20 (FIPS
   utilization requirement).
6. Add `/proc/crypto` as a primary Linux detection source throughout the spec.
7. Add the library utilization source pack from Section 8.2 to the authoritative
   source requirements: CMVP module search, OpenSSL FIPS provider documentation,
   Linux crypto subsystem documentation, CET binary verification methods.
8. Add "software fallback risk" to the mandatory research section format.

Items 1, 5, and 6 can be completed by the spec owner before research begins. Items 2, 3,
4, 7, and 8 will expand the researcher's scope but are required for the corpus to support
both the hardware posture audit and the library utilization audit use cases.

The corpus as currently specified would produce a high-quality document about cryptographic
acceleration and TEE features but would be materially incomplete as a CPU security posture
reference for a RHEL 10 DoD deployment. It would answer "what does the CPU support?" but
not "is the system actually using what the CPU supports?" — and the latter is the question
that produces security findings with remediation actions.
