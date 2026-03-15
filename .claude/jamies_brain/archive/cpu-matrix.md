# CPU Security Feature Matrix — Research Specification (v2)

**Version:** 2 (updated from Phase 0 review findings)
**Date:** 2026-03-14
**Supersedes:** Original cpu-research.md matrix sections
**Reviews incorporated:**
- `.claude/reports/cpu-matrix-review/rust-developer-review.md`
- `.claude/reports/cpu-matrix-review/security-auditor-review.md`

**Scope:** This is a RESEARCH specification. The researcher agent uses this document to build
the CPU security extensions corpus. No Rust development begins until the kernel probe project
is complete.

---

## Purpose

Build a structured CPU feature corpus for future CPU Audit Cards. Focus on CPU features
that affect:

- high assurance posture
- cryptography acceleration and correctness
- trusted execution and confidential computing
- speculative execution mitigation
- CPU-enforced access controls
- hardware entropy quality
- memory protection and encryption
- virtualization isolation
- software utilization verification (Layer 2)

The output must support a two-layer audit model:
1. **Layer 1 — Hardware Capability:** Does the CPU offer this feature?
2. **Layer 2 — Software Utilization:** Is deployed software actually using it?

The security finding emerges at the intersection: hardware offers it, software ignores it.

---

## Matrix Schema — 23 Columns

For each CPU feature, the researcher must populate ALL of these columns.

| # | Column | Type Guidance | Notes |
|---|--------|--------------|-------|
| 1 | Feature | Canonical name + known aliases | Define one canonical name per feature |
| 2 | Vendor | Intel / AMD / ARM / Both / Any | If "Both," document per-vendor CPUID differences |
| 3 | Category | One of the 15 categories below | Assign one primary; note secondaries |
| 4 | Purpose | One-sentence description | |
| 5 | Example instructions | Up to 5 most security-relevant | Not a full ISA listing |
| 6 | CPUID leaf/subleaf/bit | Structured: leaf (hex), subleaf, register (EAX/EBX/ECX/EDX), bit | Document ALL locations if multi-CPUID (e.g., AVX-512) |
| 7 | Linux detection — `/proc/cpuinfo` flag | Exact flag token name | Kernel-filtered; informational, not authoritative for all features |
| 8 | Linux detection — authoritative path | Sysfs/devfs path with expected value format | More reliable than cpuinfo for TEE, mitigations, topology |
| 9 | Minimum CPU generations | Prose: "Sandy Bridge onward" / "Zen 2 onward" | Informational only |
| 10 | Security benefit | One sentence | |
| 11 | Performance benefit | One sentence | |
| 12 | Assurance caveats | Known limitations, attacks, controversies — do NOT sanitize | |
| 13 | Virtualization behavior | Per-hypervisor (KVM, VMware, Hyper-V): passthrough / masked / emulated | Also: can guest distinguish passthrough from emulation? |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: bool, microcode_required: bool }` | Which features are CPUID-present but firmware-disabled? |
| 15 | Audit-card relevance | Critical/Defensive, Critical/Operational, Important, Informational | See classification definitions below |
| 16 | Recommended disposition when unused | "Disable in BIOS" / "Disable via cmdline" / "Monitor only" / "Leave enabled" / "N/A" | Required for DoD audit actionability |
| 17 | Software utilization detection method | How to verify software uses this feature at runtime | Primarily `/proc/crypto`, ELF inspection, build flags |
| 18 | FIPS utilization requirement | "Required by FIPS validation certificate" / "Required by NIST SP 800-90B" / "Recommended" / "N/A" / "Unknown" | |
| 19 | Active mitigation status path | Sysfs vulnerability path if applicable | `/sys/devices/system/cpu/vulnerabilities/<name>` |
| 20 | Feature accessible vs advertised | Can CPUID say yes while firmware/OS says no? | Document the gate: BIOS, kernel config, hypervisor |
| 21 | Guest-vs-host discrepancy risk | Bool: commonly differs between host and guest? | |
| 22 | Notes | Free-form | |
| 23 | Sources | List of authoritative references | Vendor manuals, kernel docs, CVEs |

### Column Notes from Phase 0 Reviews

**Column 6 (CPUID):** Must be structured, not a single string. Many features (AVX-512, CET)
use multiple CPUID locations across different registers/bits. Document ALL locations per feature.

**Columns 7 and 8 (Linux detection):** These are intentionally split. Column 7 is the
`/proc/cpuinfo` flag (informational, kernel-filtered). Column 8 is the authoritative detection
path (sysfs, devfs, or other kernel interface). They are NOT equivalent. For TEE features,
Column 8 is the only reliable path.

**Column 13 (Virtualization):** Must name specific hypervisors, not just "can be masked."
Also document whether a guest can detect that a flag is emulated vs hardware-backed.

**Column 14 (Firmware/microcode):** For mitigation-class features, microcode VERSION matters.
Document minimum microcode revision when known. A CPU may have the CPUID bit but pre-fix
microcode that does not implement the mitigation correctly.

---

## Classification Definitions

### Critical/Defensive
Absence removes a protection that blocks a known attack class.
The system is vulnerable to a documented exploit technique without this feature.

**Examples:** SMEP (ret2usr), SMAP (userland pivot), CET (ROP chains), IBRS (Spectre v2),
NX/XD (code injection on data pages), IOMMU (DMA attacks)

### Critical/Operational
Absence requires a fallback path that introduces timing risk, audit surface, or FIPS
compliance risk. The system is not directly exploitable but its security posture degrades
in ways that matter for compliance or operational integrity.

**Examples:** AES-NI on FIPS systems (software AES has timing side-channel risk),
RDRAND/RDSEED on FIPS systems (entropy source compliance per NIST SP 800-90B),
SEV-SNP for confidential VM deployments

### Important
Materially changes performance, hardening depth, or isolation quality. Absence is worth
noting but does not directly expose a vulnerability or create a compliance gap.

**Examples:** SHA extensions, PCID (PTI performance), UMIP (KASLR bypass prevention),
Intel TXT (measured boot), SMT state, microcode version

### Informational
Useful context for understanding the platform. Not decisive for security posture.

**Examples:** SSE/SSE2 (baseline on all x86_64), AVX/AVX2 (performance context),
BMI1/BMI2 (big integer helpers), Key Locker (not widely deployed)

---

## Feature Categories

| # | Category | Features |
|---|----------|----------|
| 1 | Symmetric Cryptography Acceleration | AES-NI, VAES |
| 2 | Hash & Authentication Acceleration | SHA-NI, PCLMULQDQ/CLMUL |
| 3 | Big Integer / Public Key Acceleration | ADX, BMI1, BMI2 |
| 4 | Entropy & Random Generation | RDRAND, RDSEED, ARMv8.5-RNG |
| 5 | Vector Acceleration (crypto-relevant) | SSE family, AVX, AVX2, AVX-512 |
| 6 | Trusted Execution & Enclaves | Intel SGX, Intel TXT |
| 7 | Confidential Computing & Encrypted Virt | AMD SEV, SEV-ES, SEV-SNP, Intel TDX |
| 8 | Memory Encryption | AMD SME, AMD TMME, Intel TME |
| 9 | Key Protection | Intel Key Locker |
| 10 | Speculative Execution Mitigations | IBRS/eIBRS, IBPB, STIBP, SSBD, MDS/MD_CLEAR, L1D flush, PCID |
| 11 | CPU-Enforced Access Controls | SMEP, SMAP, CET-SS, CET-IBT, UMIP, NX/XD, PKU |
| 12 | Virtualization Security | VMX, SVM, nested paging (EPT/NPT), VT-d/AMD-Vi (IOMMU) |
| 13 | Reliability / Availability / Resilience | MCA, RAS, ECC |
| 14 | Platform Topology & Metadata | SMT state, microcode version |
| 15 | ARM/AArch64 Equivalents | ARMv8 Crypto, SHA, RNG, TrustZone, PAC, BTI, MTE |

---

## Complete Feature List (60 features)

### Category 1: Symmetric Cryptography Acceleration
1. **AES-NI** — Hardware AES rounds. Instructions: AESENC, AESENCLAST, AESDEC, AESDECLAST, AESKEYGENASSIST. CPUID: leaf 1, ECX bit 25. `/proc/crypto` driver: `aesni_intel` (x86), `aes-ce` (ARM). Classification: Critical/Operational (FIPS).
2. **VAES** — Vectorized AES on AVX-512 registers. Multiple AES blocks processed simultaneously. Classification: Informational.

### Category 2: Hash & Authentication Acceleration
3. **SHA Extensions (SHA-NI)** — Hardware SHA-1 and SHA-256. Classification: Important.
4. **PCLMULQDQ / CLMUL** — Carry-less multiplication for GF math and AES-GCM authentication. `/proc/crypto` driver: `ghash-clmulni-intel`. Classification: Important.

### Category 3: Big Integer / Public Key Acceleration
5. **ADX** — Multi-precision arithmetic for RSA/ECC. Classification: Informational.
6. **BMI1** — Bit manipulation set 1. Classification: Informational.
7. **BMI2** — Bit manipulation set 2. Classification: Informational.

### Category 4: Entropy & Random Generation
8. **RDRAND** — Hardware RNG. Classification: Critical/Operational (FIPS). Requires NIST SP 800-90B analysis.
9. **RDSEED** — Raw entropy for CSPRNG seeding. Classification: Critical/Operational (FIPS). Requires NIST SP 800-90B analysis.
10. **ARMv8.5-RNG** — ARM equivalent of RDRAND. Flag: `rng`. Classification: Critical/Operational (FIPS).

### Category 5: Vector Acceleration (Crypto-Relevant)
11. **SSE** — 128-bit SIMD. Classification: Informational (baseline).
12. **SSE2** — Integer vector ops. Classification: Informational (baseline).
13. **SSE3** — Horizontal adds. Classification: Informational.
14. **SSSE3** — Shuffle byte, alignment. Classification: Informational.
15. **SSE4.1** — Blend, min/max, dot product. Classification: Informational.
16. **SSE4.2** — CRC32, string ops. Classification: Informational.
17. **AVX** — 256-bit vector registers. Classification: Informational.
18. **AVX2** — 256-bit integer vectors. Classification: Informational.
19. **AVX-512** — 512-bit vectors, mask registers. Classification: Informational. Note: commonly disabled in cloud VMs.

### Category 6: Trusted Execution & Enclaves
20. **Intel SGX** — Enclave-based trusted execution with attestation. Classification: Important. **Caveat:** Substantial attack history (Foreshadow, SGAxe, CrossTalk, LVI, AEPIC Leak). Presence-without-use is attack surface, not assurance gain. Recommended disposition when unused: disable in BIOS. Detection: `/dev/sgx_enclave` existence (device node — different provenance model from sysfs).
21. **Intel TXT** — Measured launch environment for platform attestation. Distinct from TDX. Complements Secure Boot for measured boot chain. Classification: Important.

### Category 7: Confidential Computing & Encrypted Virtualization
22. **AMD SEV** — Per-VM memory encryption keys. Classification: Important.
23. **AMD SEV-ES** — Encrypted CPU register state. Classification: Important.
24. **AMD SEV-SNP** — VM integrity protections, attestation. Classification: Critical/Operational. Detection: `/sys/module/kvm_amd/parameters/sev_snp`.
25. **Intel TDX** — Hardware-isolated confidential VMs. Classification: Critical/Operational. Detection: `/sys/module/kvm_intel/parameters/tdx`.

### Category 8: Memory Encryption
26. **AMD SME** — Transparent memory encryption with hardware key. Classification: Important.
27. **AMD TMME** — Extended transparent memory encryption. Classification: Important.
28. **Intel TME** — Full platform memory encryption (Intel's SME equivalent). Classification: Important.

### Category 9: Key Protection
29. **Intel Key Locker** — Hardware-protected key storage preventing software access to raw keys. Classification: Informational. Firmware dependency: high.

### Category 10: Speculative Execution Mitigations
30. **IBRS** — Indirect Branch Restricted Speculation. Spectre v2 mitigation. CPUID: leaf 7, subleaf 0, EDX bit 26. Classification: Critical/Defensive.
31. **eIBRS** — Enhanced IBRS. Preferred on newer CPUs. Classification: Critical/Defensive.
32. **IBPB** — Indirect Branch Predictor Barrier. Flushes branch predictor on context switch. On SELinux MLS systems, cross-domain branch predictor contamination is a label boundary violation. Classification: Critical/Defensive.
33. **STIBP** — Single Thread Indirect Branch Predictors. Prevents SMT siblings sharing predictions. Classification: Critical/Defensive (on SMT systems).
34. **SSBD** — Speculative Store Bypass Disable. Spectre v4 mitigation. Classification: Critical/Defensive.
35. **MDS / MD_CLEAR** — Microarchitectural Data Sampling mitigation (VERW-based). Covers MDS, TAA, SRBDS. Requires microcode. Classification: Critical/Defensive.
36. **L1D flush** — L1 Terminal Fault / Foreshadow mitigation. Classification: Critical/Defensive.
37. **PCID** — Process Context Identifiers. Makes PTI (Meltdown mitigation) low-overhead. Without PCID, PTI has significant performance cost that drives operators to disable it. Classification: Important.

### Category 11: CPU-Enforced Access Controls
38. **SMEP** — Supervisor Mode Execution Prevention. Kills ret2usr exploit class. CPUID: leaf 7, subleaf 0, EBX bit 7. Classification: Critical/Defensive.
39. **SMAP** — Supervisor Mode Access Prevention. Kills userland pivot kernel exploits. CPUID: leaf 7, subleaf 0, EBX bit 20. Classification: Critical/Defensive.
40. **CET-SS** — Control-flow Enforcement: Shadow Stack. CPU-protected return address stack; any ROP chain triggers a fault. RHEL 10 ships CET-enabled. CPUID: leaf 7, subleaf 0, ECX bit 7. Binary verification: `eu-readelf -n <binary>` for `.note.gnu.property` with `SHSTK` bit. Classification: Critical/Defensive.
41. **CET-IBT** — Control-flow Enforcement: Indirect Branch Tracking. Enforces ENDBR landing pads. CPUID: leaf 7, subleaf 0, ECX bit 20. Classification: Critical/Defensive.
42. **UMIP** — User Mode Instruction Prevention. Blocks SGDT/SIDT/SLDT/SMSW/STR from userspace. Kills KASLR-bypass information leaks. CPUID: leaf 7, subleaf 0, ECX bit 2. Classification: Important.
43. **NX/XD** — No-Execute / Execute Disable. Hardware W^X prerequisite. Foundational — entire exploit mitigation stack collapses without it. Classification: Critical/Defensive.
44. **PKU** — Protection Keys for User-space. Per-page access protection without syscalls. Classification: Important.

### Category 12: Virtualization Security
45. **VMX** — Intel VT-x. Classification: Informational.
46. **SVM** — AMD-V. Classification: Informational.
47. **Nested paging (EPT / NPT)** — Hardware page table translation for VMs. Classification: Informational.
48. **VT-d / AMD-Vi (IOMMU)** — DMA isolation. Classification: Critical/Defensive. Without IOMMU, DMA attacks bypass kernel-level blacklisting entirely. Thunderbolt/FireWire blacklisting (already posture signals) depends on IOMMU being active. Detection: `/sys/class/iommu/`, `/proc/cmdline` for `intel_iommu=on`.

### Category 13: Reliability / Availability / Resilience
49. **MCA** — Machine Check Architecture. Classification: Important.
50. **RAS** — Reliability, Availability, Serviceability features. Classification: Important.
51. **ECC** — Error-correcting code memory interactions. Classification: Important.

### Category 14: Platform Topology & Metadata
52. **SMT state** — Simultaneous Multithreading / Hyperthreading. Not a feature in the traditional sense — a platform topology property with security implications. Affects STIBP, MDS exposure, isolation posture. Detection: `/sys/devices/system/cpu/smt/active`. Cmdline: `nosmt`, `mitigations=nosmt`. Classification: Important.
53. **Microcode version** — Mitigation correctness prerequisite. Detection: `/proc/cpuinfo` `microcode` field, `/sys/devices/system/cpu/cpu0/microcode/version`. For mitigation features, microcode version determines whether the fix is actually implemented. Classification: Important.

### Category 15: ARM/AArch64 Equivalents
54. **ARMv8 Cryptography Extension** — AES equivalent. Flag: `aes`. Classification: Critical/Operational (FIPS).
55. **ARMv8 SHA** — SHA equivalent. Flags: `sha2`, `sha512`. Classification: Important.
56. **ARMv8.5-RNG** — RDRAND equivalent. Flag: `rng`. Classification: Critical/Operational (FIPS).
57. **ARM TrustZone** — TEE equivalent (process-level, not enclave). Classification: Important.
58. **ARM PAC** — Pointer Authentication Codes. Return address signing. Classification: Important.
59. **ARM BTI** — Branch Target Identification. CET-IBT equivalent. Classification: Important.
60. **ARM MTE** — Memory Tagging Extension. Hardware memory safety. Classification: Important.

---

## Feature Interpretation Rules

For each feature, ask:

1. Does it improve performance only?
2. Does it improve security only?
3. Does it improve both?
4. Does it introduce new trust assumptions?
5. Does it depend on firmware, microcode, kernel, or hypervisor cooperation?
6. Can it be disabled, hidden, or virtualized away?
7. **Does it have compliance-specific behavior under FIPS 140-2/3, NIST SP 800-90B, CC, or CMMC?**
8. **If present but unused by software, does that constitute a security finding?**

Do not treat every CPU extension as automatically "good." SGX present-but-unused is attack
surface, not assurance. AES-NI present but not used by OpenSSL on a FIPS system is a finding.

---

## Mandatory Research Sections Per Feature

For each feature, write a structured note containing:

1. **Description** — what it is and how it works
2. **Why security engineers care** — threat model impact
3. **Why performance engineers care** — throughput/latency implications
4. **Trust model** — what must be trusted for this feature to deliver its claim
5. **Linux visibility** — `/proc/cpuinfo` flag AND authoritative sysfs/devfs path
6. **Hypervisor visibility** — per-hypervisor behavior (KVM, VMware, Hyper-V)
7. **Known problems or controversies** — do NOT sanitize
8. **CVE summary** — significant CVEs, CVSS range, fix mechanism (microcode/kernel/firmware)
9. **Compliance-specific requirements** — FIPS, NIST SP 800-90B, CMMC, CC implications
10. **Virtualization confidence** — can a guest reliably verify hardware-backed?
11. **Recommended disposition when unused** — BIOS/kernel action if feature exists but deployment does not use it
12. **Software fallback risk** — is the software fallback constant-time? CVE history? FIPS-validated?
13. **Recommendation for audit-card display** — what the audit card should show

---

## Linux Detection Interfaces

### Primary detection paths (Layer 1 — hardware capability)

| Interface | What it reveals | Provenance verification | Notes |
|---|---|---|---|
| `/proc/cpuinfo` flags line | Kernel-filtered CPU flags | `PROC_SUPER_MAGIC` via fstatfs | Main safe-Rust path. Flag names are kernel-assigned, not always identical to vendor names. ARM uses `Features:` instead of `flags:` |
| `/sys/devices/system/cpu/vulnerabilities/*` | Per-vulnerability mitigation status | `SYSFS_MAGIC` via fstatfs | More authoritative than `/proc/cmdline` for whether mitigations are ACTIVE. Document every file and all possible values |
| `/sys/devices/system/cpu/smt/active` | SMT topology status | `SYSFS_MAGIC` | |
| `/sys/devices/system/cpu/cpu0/microcode/version` | Installed microcode version | `SYSFS_MAGIC` | |
| `/sys/module/kvm_amd/parameters/sev*` | AMD SEV/SNP enablement | `SYSFS_MAGIC` | Document exact value format: `1`, `Y`, or other? |
| `/sys/module/kvm_intel/parameters/tdx` | Intel TDX enablement | `SYSFS_MAGIC` | |
| `/dev/sgx_enclave` | SGX availability | Device node — no fstatfs magic check | Different provenance model from sysfs |
| `/sys/class/iommu/` | IOMMU presence | `SYSFS_MAGIC` | |

### Layer 2 detection path (software utilization)

| Interface | What it reveals | Notes |
|---|---|---|
| `/proc/crypto` | Registered kernel crypto algorithm implementations | **Primary Layer 2 source.** Shows driver name (hardware vs software), priority, selftest status, fips_allowed. `PROC_SUPER_MAGIC` provenance |
| ELF `.note.gnu.property` | CET compatibility flags (IBT, SHSTK) | Binary compiled with `-fcf-protection=full`? |
| `OPENSSL_ia32cap` env var | Runtime CPU feature override for OpenSSL | Misconfigured value silently disables hardware acceleration |
| RPM/package metadata | Compile flags used for system packages | SRPM metadata, not always available at runtime |

### `/proc/crypto` format reference

Each entry in `/proc/crypto`:
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

- `driver` distinguishes hardware-accelerated from software (e.g., `aesni_intel` vs `aes-generic`)
- `priority` — higher wins when multiple implementations exist
- `selftest: passed` — required for FIPS operation
- `fips_allowed: yes/no` — directly relevant to FIPS posture

### Driver-to-feature mapping (researcher must verify and expand)

| CPU Feature | Expected hardware driver | Software fallback driver |
|---|---|---|
| AES-NI (x86) | `aesni_intel`, `aes-aesni` | `aes-generic` |
| AES (ARM) | `aes-ce` | `aes-generic` |
| PCLMULQDQ | `ghash-clmulni-intel`, `crct10dif-pclmul` | `ghash-generic`, `crct10dif-generic` |
| SHA-NI | `sha256-ni` | `sha256-generic` |
| SHA (AVX2) | `sha256-avx2` | `sha256-generic` |

---

## Authoritative Source Pack

### Primary sources (vendor manuals and kernel docs)

**Intel:**
- Intel SDM — specifically **Volume 2A, Chapter 3 ("CPUID — CPU Identification")**
- Intel Instruction Set Extensions Programming Reference
- Intel CET specification
- Intel SGX developer documentation
- Intel Product Security Center — for per-feature CVE history

**AMD:**
- AMD64 Architecture Programmer's Manual — specifically **Volume 3, Appendix E ("Obtaining Processor Information via CPUID")**
- AMD SEV developer documentation
- AMD Product Security — for SEV/SNP CVE history

**Linux kernel:**
- `/proc/cpuinfo` x86 feature flags documentation
- `Documentation/x86/cet.rst` — CET kernel support
- `Documentation/security/snp-tdx-threat-model.rst` — already acquired
- `Documentation/arch/x86/amd-memory-encryption.rst` — already acquired
- `Documentation/crypto/` — crypto subsystem, already acquired
- `/sys/devices/system/cpu/vulnerabilities/` documentation
- SGX documentation
- TDX documentation

### Government and standards sources

- **NIST SP 800-90B** — entropy source requirements. **BLOCKING for RDRAND/RDSEED classification.** URL: `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf`
- **NIST SP 800-155** — BIOS integrity measurement guidelines (measured boot, TXT)
- **NIST SP 800-193** — Platform firmware resiliency guidelines (microcode update chain)
- **NSA RHEL hardening guidance** — which CPU features NSA recommends verifying
- **DoD STIGs for RHEL 10** — STIG controls with CPU-capability prerequisites
- **CMVP Module Search** — `https://csrc.nist.gov/projects/cryptographic-module-validation-program` — RHEL 10 FIPS module validation certificates

### CVE and vulnerability research

For each feature with known attack history, document:
- Table of significant CVEs
- Impact on the feature's security claim
- Fixed by microcode, firmware, or kernel patch?
- CVSS score range

**Minimum CVE research required for:**
- SGX: Foreshadow/L1TF, SGAxe, CrossTalk, LVI, AEPIC Leak
- RDRAND: CVE-2019-11090 (AMD RDRAND bug), Intel stepping-specific issues
- AMD SEV/SEV-SNP: CacheWarp, CVE-2021-26311 and related
- Intel TDX: early implementation issues
- AES-NI: cache-timing attacks in software fallback paths (not hardware itself)

**CVE sources:**
- NVD: `https://nvd.nist.gov/`
- Intel Product Security Center: `https://www.intel.com/content/www/us/en/security-center`
- AMD Product Security: `https://www.amd.com/en/resources/product-security.html`

### Secondary sources

Use secondary sources only to enrich terminology, history, or cross-checks.
Primary claims must come from vendor manuals or kernel documentation.
Academic papers are required for side-channel concerns, SGX/SEV attack history,
and TDX threat assumptions. Separate "vendor claim" from "research community criticism."

---

## Data Quality Rules

- Prefer vendor manuals and kernel docs over blogs.
- Prefer architecture manuals over marketing pages.
- Record exact CPUID locations whenever available.
- Record when a feature requires BIOS enablement, microcode update, kernel configuration, or hypervisor support.
- Record when guest-visible support can differ from host support.
- For mitigation features, record minimum microcode revision when known.

---

## ARM/AArch64 Requirements

The project runs on AArch64 (RHEL kernel 6.12 aarch64). The corpus must document ARM
equivalents for at minimum:

| x86 Feature | ARM Equivalent | `/proc/cpuinfo` flag |
|---|---|---|
| AES-NI | ARMv8 Crypto Extension | `aes` |
| SHA-NI | ARMv8 SHA | `sha2`, `sha512` |
| RDRAND | ARMv8.5-RNG | `rng` |
| SGX / TDX | ARM TrustZone | N/A (different model) |
| CET Shadow Stack | ARM PAC (Pointer Authentication) | `paca`, `pacg` |
| CET IBT | ARM BTI (Branch Target Identification) | `bti` |
| N/A | ARM MTE (Memory Tagging Extension) | `mte` |

On AArch64, `/proc/cpuinfo` uses `Features:` instead of `flags:`.

---

## Initial Prioritization

Ordered by immediate audit-card value. Defensive features first (from security-auditor
review), then capability features.

**Tier 1 — Highest priority (defensive features tied to existing posture signals):**
1. SMEP / SMAP — directly relevant to kernel hardening posture
2. CET (Shadow Stack + IBT) — RHEL 10 first-class hardening
3. NX/XD — foundational, must confirm present
4. IBRS / IBPB / STIBP / SSBD — what `mitigations=off` actually disables
5. `/sys/devices/system/cpu/vulnerabilities/` interface — detection reference for above
6. IOMMU (VT-d / AMD-Vi) — DMA protection prerequisite

**Tier 2 — High priority (crypto and entropy with FIPS relevance):**
7. AES-NI
8. PCLMULQDQ
9. SHA extensions
10. RDRAND / RDSEED (blocked on NIST SP 800-90B)

**Tier 3 — High priority (confidential computing):**
11. SGX
12. AMD SEV / SEV-ES / SEV-SNP
13. Intel TDX
14. AMD SME / Intel TME

**Tier 4 — Medium priority (remaining features):**
15. AVX2, AVX-512, vector family
16. ADX, BMI1, BMI2
17. Intel Key Locker, Intel TXT
18. MCA, RAS, ECC, SMT, microcode version

**Tier 5 — ARM equivalents (parallel with corresponding x86 tiers):**
19. ARMv8 Crypto, SHA, RNG, PAC, BTI, MTE, TrustZone

---

## Deliverables

1. **Master CPU feature matrix** — all 60 features, all 23 columns
2. **Per-feature research notes** — mandatory sections per feature
3. **Linux detection reference sheet** — all paths, magic checks, value formats
4. **`/proc/crypto` reference** — driver mapping table, priority semantics, FIPS fields
5. **Audit-card summary recommendations** — what to show, how to classify
6. **CVE/attack history tables** — per feature where applicable
7. **ARM/AArch64 equivalents appendix**
8. **Evidence-source map** — where each conclusion is derived
9. **Knowledge index entry** — for agent pre-seeding (see below)

---

## Knowledge Index Entry (for agent pre-seeding after corpus is built)

```
Topic: CPU security-relevant extensions
Categories:
  - Symmetric crypto acceleration (AES-NI, VAES)
  - Hash & auth acceleration (SHA-NI, PCLMULQDQ)
  - Hardware entropy (RDRAND, RDSEED, ARMv8.5-RNG)
  - Speculative execution mitigations (IBRS, IBPB, STIBP, SSBD, MDS, L1D flush)
  - CPU-enforced access controls (SMEP, SMAP, CET-SS, CET-IBT, NX/XD, UMIP)
  - Trusted execution (SGX, TXT, TrustZone)
  - Confidential computing (SEV, SEV-SNP, TDX)
  - Memory encryption (SME, TMME, TME)
  - Vector crypto acceleration (SSE→AVX-512)
  - Virtualization security (VMX, SVM, IOMMU)
  - Platform topology (SMT, microcode version)
  - ARM equivalents (Crypto Ext, PAC, BTI, MTE)
Resource location: .claude/references/cpu-extensions/
Questions this corpus answers:
  - Which CPU features materially affect security posture?
  - Which features are present but not used by software? (Layer 2)
  - Which mitigations are active on this CPU?
  - Is OpenSSL using hardware acceleration on a FIPS system?
  - Which features are CPUID-present but firmware-disabled?
Consult when working on: CPU audit cards, crypto posture, FIPS compliance, mitigation verification
```
