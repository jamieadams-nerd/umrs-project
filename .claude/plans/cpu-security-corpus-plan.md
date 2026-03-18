# CPU Security Extensions Research & Corpus Plan (v2)

**Created:** 2026-03-14
**Status:** Phases 0/0.5/1A/1B/1C/1G COMPLETE. 1D/1E research done — files need writing. 1F unblocked (CET docs acquired). Next: write 1D/1E files, then 1F, then 1H.
**Source reviews:** `reports/cpu-matrix-review/rust-developer-review.md`, `reports/cpu-matrix-review/security-auditor-review.md`
**Supersedes:** Track 1 of the original research corpus expansion plan

**IMPORTANT:** This plan is research and corpus-building ONLY. No Rust development or
implementation work begins until the kernel probe project (`umrs-platform` posture work)
is complete. The researcher gathers the corpus; reviewers validate it; it sits ready for
future CPU probe development.

---

## Design Decisions Accepted (Phase 0 Output)

All rust-developer and security-auditor recommendations are accepted. Key decisions:

1. **Classification sub-tiers:** Critical/Defensive (blocks known attack class) vs Critical/Operational (fallback creates compliance/timing risk)
2. **20-column matrix** (original 17 + disposition-when-unused + software-utilization-detection + FIPS-utilization-requirement)
3. **3 additional columns from rust-developer:** active-mitigation-status, feature-accessible-vs-advertised, guest-vs-host-discrepancy-risk — total: **23 columns**
4. **18 missing extensions added:** speculative execution mitigations, CPU-enforced access controls, platform attestation additions, SMT topology
5. **Two-layer audit model** baked in: Layer 1 = hardware capability, Layer 2 = software utilization
6. **`/proc/crypto`** as primary Layer 2 detection interface
7. **CPUID detection via `/proc/cpuinfo`** (safe Rust path); `raw_cpuid` crate deferred pending Jamie's architectural review
8. **ARM/AArch64 equivalents** required for crypto and entropy features
9. **CVE tables** required per feature with known attack history
10. **NIST SP 800-90B** blocking for RDRAND/RDSEED classification

---

## What Already Exists (Do NOT Re-acquire)

| Material | Location | Status |
|---|---|---|
| NIST SP 800-53 Rev 5 | `refs/nist/sp800-53r5.pdf` | Done |
| NIST SP 800-171 Rev 2+3 | `refs/nist/` | Done |
| CMMC L2 Assessment Guide | `refs/dod/cmmc-assessment-guide-l2.pdf` | Done |
| SELinux Notebook | `.claude/references/selinux-notebook/` | Done |
| Linux kernel docs (full tree) | `.claude/references/kernel-docs/` | Done |
| — incl. SNP/TDX threat model | `kernel-docs/security/snp-tdx-threat-model.rst` | Done |
| — incl. AMD memory encryption | `kernel-docs/arch/x86/amd-memory-encryption.rst` | Done |
| — incl. crypto subsystem docs | `kernel-docs/crypto/` | Done |
| NIST SSDF (800-218) | `refs/nist/sp800-218-ssdf.pdf` | Done |
| Phase 0 reviews | `.claude/reports/cpu-matrix-review/` | Done |

---

## Complete Feature & Category Inventory

This is the full list of CPU extensions and platform properties to be researched.
Each entry will be documented with all 23 matrix columns.

### Category 1: Symmetric Cryptography Acceleration
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 1 | AES-NI (AESNI) | Both | 1A | Critical/Operational (FIPS) |
| 2 | VAES (Vector AES on AVX-512) | Intel | 1A | Informational |

### Category 2: Hash & Authentication Acceleration
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 3 | SHA Extensions (SHA-NI) | Both | 1A | Important |
| 4 | PCLMULQDQ / CLMUL (carryless multiply) | Both | 1A | Important |

### Category 3: Big Integer / Public Key Acceleration
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 5 | ADX (multi-precision add-carry) | Both | 1B | Informational |
| 6 | BMI1 (bit manipulation set 1) | Both | 1B | Informational |
| 7 | BMI2 (bit manipulation set 2) | Both | 1B | Informational |

### Category 4: Entropy & Random Generation
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 8 | RDRAND (hardware RNG) | Both | 1B | Critical/Operational (FIPS) |
| 9 | RDSEED (raw entropy source) | Both | 1B | Critical/Operational (FIPS) |
| 10 | ARMv8.5-RNG (`rng` flag) | ARM | 1B | Critical/Operational (FIPS) |

### Category 5: Vector Acceleration (Crypto-Relevant)
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 11 | SSE | Both | 1C | Informational |
| 12 | SSE2 | Both | 1C | Informational |
| 13 | SSE3 | Both | 1C | Informational |
| 14 | SSSE3 | Both | 1C | Informational |
| 15 | SSE4.1 | Both | 1C | Informational |
| 16 | SSE4.2 | Both | 1C | Informational |
| 17 | AVX | Both | 1C | Informational |
| 18 | AVX2 | Both | 1C | Informational |
| 19 | AVX-512 (foundation + crypto-relevant subsets) | Intel | 1C | Informational |

### Category 6: Trusted Execution & Enclaves
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 20 | Intel SGX (Software Guard Extensions) | Intel | 1D | Important (caveat: attack surface if unused) |
| 21 | Intel TXT (Trusted Execution Technology) | Intel | 1D | Important |

### Category 7: Confidential Computing & Encrypted Virtualization
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 22 | AMD SEV (Secure Encrypted Virtualization) | AMD | 1D | Important |
| 23 | AMD SEV-ES (Encrypted State) | AMD | 1D | Important |
| 24 | AMD SEV-SNP (Secure Nested Paging) | AMD | 1D | Critical/Operational |
| 25 | Intel TDX (Trust Domain Extensions) | Intel | 1D | Critical/Operational |

### Category 8: Memory Encryption
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 26 | AMD SME (Secure Memory Encryption) | AMD | 1D | Important |
| 27 | AMD TMME (Transparent Machine Memory Encryption) | AMD | 1D | Important |
| 28 | Intel TME (Total Memory Encryption) | Intel | 1D | Important |

### Category 9: Key Protection
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 29 | Intel Key Locker | Intel | 1D | Informational |

### Category 10: Speculative Execution Mitigations (NEW — from security-auditor review)
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 30 | IBRS (Indirect Branch Restricted Speculation) | Both | 1E | Critical/Defensive |
| 31 | eIBRS (Enhanced IBRS) | Both | 1E | Critical/Defensive |
| 32 | IBPB (Indirect Branch Predictor Barrier) | Both | 1E | Critical/Defensive |
| 33 | STIBP (Single Thread Indirect Branch Predictors) | Both | 1E | Critical/Defensive |
| 34 | SSBD (Speculative Store Bypass Disable) | Both | 1E | Critical/Defensive |
| 35 | MDS mitigations / MD_CLEAR (VERW-based) | Both | 1E | Critical/Defensive |
| 36 | L1D flush (L1TF / Foreshadow mitigation) | Both | 1E | Critical/Defensive |
| 37 | PCID (Process Context Identifiers) | Intel | 1E | Important (PTI performance) |

### Category 11: CPU-Enforced Access Controls (NEW — from security-auditor review)
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 38 | SMEP (Supervisor Mode Execution Prevention) | Both | 1F | Critical/Defensive |
| 39 | SMAP (Supervisor Mode Access Prevention) | Both | 1F | Critical/Defensive |
| 40 | CET-SS (Control-flow Enforcement: Shadow Stack) | Intel | 1F | Critical/Defensive |
| 41 | CET-IBT (Control-flow Enforcement: Indirect Branch Tracking) | Intel | 1F | Critical/Defensive |
| 42 | UMIP (User Mode Instruction Prevention) | Both | 1F | Important |
| 43 | NX/XD (No-Execute / Execute Disable) | Both | 1F | Critical/Defensive |
| 44 | PKU (Protection Keys for User-space) | Intel | 1F | Important |

### Category 12: Virtualization Security
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 45 | VMX (Intel VT-x) | Intel | 1G | Informational |
| 46 | SVM (AMD-V) | AMD | 1G | Informational |
| 47 | Nested paging (EPT / NPT) | Both | 1G | Informational |
| 48 | VT-d / AMD-Vi (IOMMU / DMA isolation) | Both | 1G | Critical/Defensive |

### Category 13: Reliability / Availability / Resilience
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 49 | MCA (Machine Check Architecture) | Both | 1G | Important |
| 50 | RAS features (Reliability, Availability, Serviceability) | Both | 1G | Important |
| 51 | ECC-related platform interactions | Both | 1G | Important |

### Category 14: Platform Topology & Metadata (cross-cutting)
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 52 | SMT state (Simultaneous Multithreading / Hyperthreading) | Both | 1G | Important (security cross-cut) |
| 53 | Microcode version | Both | 1G | Important (mitigation prerequisite) |

### Category 15: ARM/AArch64 Equivalents
| # | Feature | Vendor | Phase | Classification |
|---|---------|--------|-------|---------------|
| 54 | ARMv8 Cryptography Extension (`aes` flag) | ARM | 1A | Critical/Operational (FIPS) |
| 55 | ARMv8 SHA (`sha2`, `sha512` flags) | ARM | 1A | Important |
| 56 | ARMv8.5-RNG (`rng` flag) | ARM | 1B | Critical/Operational (FIPS) |
| 57 | ARM TrustZone (TEE equivalent) | ARM | 1D | Important |
| 58 | ARM Pointer Authentication (PAC) | ARM | 1F | Important |
| 59 | ARM Branch Target Identification (BTI) | ARM | 1F | Important |
| 60 | ARM Memory Tagging Extension (MTE) | ARM | 1F | Important |

### Detection Interfaces (not features — documented as reference)
| Interface | Purpose | Phase |
|-----------|---------|-------|
| `/proc/cpuinfo` flags/Features line | Primary safe-Rust detection path | All |
| `/sys/devices/system/cpu/vulnerabilities/*` | Per-vulnerability mitigation status | 1E |
| `/sys/devices/system/cpu/smt/active` | SMT topology | 1G |
| `/sys/devices/system/cpu/cpu0/microcode/version` | Microcode version | 1G |
| `/sys/module/kvm_amd/parameters/sev*` | AMD SEV/SNP enablement | 1D |
| `/sys/module/kvm_intel/parameters/tdx` | Intel TDX enablement | 1D |
| `/dev/sgx_enclave` | SGX availability (device node) | 1D |
| `/sys/class/iommu/` | IOMMU presence | 1G |
| `/proc/crypto` | Kernel crypto driver registration (Layer 2) | 1H |

**Total: 60 features across 15 categories + 9 detection interfaces**

---

## Model Assignments

| Phase | Agent | Model | Rationale |
|---|---|---|---|
| Phase 0.5 — Spec Update | main orchestrator | **sonnet** | Editing existing spec, not original research |
| Phase 1A — Crypto Acceleration | researcher | **sonnet** | Structured document research and extraction |
| Phase 1B — Entropy & Big Integer | researcher | **sonnet** | Structured research, 800-90B analysis |
| Phase 1C — Vector Extensions | researcher | **sonnet** | Compilation and classification |
| Phase 1D — TEE & Confidential Computing | researcher | **opus** | Complex threat models, CVE analysis, multiple detection paths |
| Phase 1E — Speculative Mitigations | researcher | **opus** | Critical/Defensive, complex sysfs interface analysis, microcode dependencies |
| Phase 1F — CPU Access Controls | researcher | **opus** | CET deep dive, binary verification, ELF analysis |
| Phase 1G — Virt, Reliability, Topology | researcher | **sonnet** | Research compilation, cross-cutting SMT analysis |
| Phase 1H — /proc/crypto & Software Util | researcher | **sonnet** | Interface documentation, driver mapping |
| Phase 1I — Matrix Synthesis | researcher | **opus** | Cross-phase synthesis, 23-column master matrix, knowledge index |
| Phase 1J — Post-Research Review | rust-developer | **opus** | Data structure proposals, SignalClass design |
| Phase 1J — Post-Research Review | security-engineer | **opus** | Classification finalization, detection path verification |
| Phase 1K — Corpus Refinement | researcher | **sonnet** | Gap filling from review feedback |

---

## Phase 0.5 — Spec Update (BLOCKING)

**Scope:** Update `cpu-matrix.md` to incorporate all Phase 0 findings before research begins
**Agent:** Main orchestrator (no subagent needed — spec editing)
**Status:** COMPLETE (2026-03-17)
**Deliverable:** `.claude/references/cpu-extensions/cpu-matrix.md` (v3)
**Verification:** All 10 actions confirmed incorporated; spec bumped to v3 and placed in researcher-consumption location

**Actions:**
1. Update classification definitions — add Critical/Defensive vs Critical/Operational sub-tiers
2. Expand matrix to 23 columns:
   - Original 17
   - Col 18: Recommended disposition when present but unused
   - Col 19: Software utilization detection method
   - Col 20: FIPS utilization requirement
   - Col 21: Active mitigation status (sysfs vulnerability path)
   - Col 22: Feature accessible vs advertised (BIOS/firmware gate)
   - Col 23: Guest-vs-host discrepancy risk flag
3. Add new feature categories:
   - **Category 13: Speculative Execution Mitigations** — IBRS/eIBRS, IBPB, STIBP, SSBD, MDS/MD_CLEAR, L1D flush
   - **Category 14: CPU-Enforced Access Controls** — SMEP, SMAP, CET-SS, CET-IBT, UMIP, NX/XD, PKU
4. Add new mandatory research sections per feature:
   - CVE summary (significant CVEs, impact, fix mechanism)
   - Compliance-specific requirements (FIPS, 800-90B, CMMC, CC)
   - Virtualization confidence (can guest verify hardware-backed?)
   - Recommended disposition when unused
   - Software fallback risk (constant-time? CVE history? FIPS-validated?)
5. Add new authoritative sources to the source pack
6. Add `/proc/crypto` as a mandatory detection interface
7. Add ARM/AArch64 equivalents requirement
8. Update feature interpretation framework — add compliance dimension (question 7)
9. Add SMT as cross-cutting platform topology property
10. Update initial prioritization — defensive features at top

**Deliverable:** Updated `cpu-matrix.md` ready for researcher consumption

---

## Phase 1A — Crypto Acceleration Extensions

**Scope:** AES-NI, VAES, SHA extensions, PCLMULQDQ/CLMUL
**Agent:** researcher
**Status:** COMPLETE (2026-03-18) — 6 files in `.claude/references/cpu-extensions/crypto-accel/`

**Actions:**
- Research each extension: description, instructions, CPUID detection, Linux flags
- Document security relevance, performance benefit, known vulnerabilities
- Document ARM/AArch64 equivalents (ARMv8 Crypto Extension: `aes`, `sha2`, `sha512`)
- For each: populate all 23 matrix columns
- Document `/proc/crypto` driver names that indicate hardware acceleration:
  - AES-NI → `aesni_intel` (x86), `aes-ce` (ARM)
  - PCLMULQDQ → `crct10dif-pclmul`, `ghash-clmulni-intel`
  - SHA → `sha256-ni`, `sha256_ssse3`, `sha256-avx2`
- CVE tables for AES-NI (cache-timing in software fallback paths)
- FIPS utilization requirements — document OpenSSL dependency chain
- Software fallback risk analysis per feature

**Sources:** Intel SDM Vol 2A Ch 3, AMD APM Vol 3 App E, Linux kernel crypto docs, `/proc/crypto` format
**Save to:** `.claude/references/cpu-extensions/crypto-accel/`
**Deliverable:** 4 structured reference files + ARM equivalents appendix

---

## Phase 1B — Entropy & Big Integer Extensions

**Scope:** RDRAND, RDSEED, ADX, BMI1, BMI2
**Agent:** researcher
**Status:** COMPLETE (2026-03-18) — 5 files in `.claude/references/cpu-extensions/entropy-bigint/`

**Actions:**
- RDRAND/RDSEED: full NIST SP 800-90B analysis
  - Entropy quality assessment requirements
  - Which CPU steppings carry 800-90B-compliant health tests
  - Connection to existing `SignalId::RandomTrustCpu` signal
  - FIPS 140-3 perspective on hardware RNG trustworthiness
  - CVE history (CVE-2019-11090 AMD RDRAND bug, Intel stepping issues)
  - ARM equivalent: ARMv8.5-RNG extension (`rng` flag)
- ADX/BMI: RSA/ECC acceleration significance, performance context
- Populate all 23 columns per feature
- RDRAND/RDSEED classified as **Critical/Operational** on FIPS systems

**Sources:** NIST SP 800-90B (must acquire first), Intel SDM, AMD APM
**Save to:** `.claude/references/cpu-extensions/entropy-bigint/`
**Deliverable:** 5 structured reference files

**Pre-requisite action:** Researcher must acquire NIST SP 800-90B before starting RDRAND/RDSEED work:
- URL: `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf`
- Save to: `refs/nist/sp800-90B.pdf`
- Update `refs/manifest.md`

---

## Phase 1C — Vector Extensions (Crypto-Relevant)

**Scope:** SSE family, AVX, AVX2, AVX-512, VAES-on-AVX-512
**Agent:** researcher
**Status:** COMPLETE (2026-03-18) — 4 files in `.claude/references/cpu-extensions/vector/`

**Actions:**
- Focus on crypto library usage, not general SIMD
- Document which crypto operations benefit from each level
- Note AVX-512 power/frequency trade-offs and cloud VM masking
- Document hypervisor-specific behavior: KVM, VMware, Hyper-V
- `/proc/crypto` driver name patterns (e.g., `sha256-avx2`, `aes-avx`)
- Column 5 (example instructions): bounded to 5 most security-relevant per feature
- Informational classification for all vector features

**Save to:** `.claude/references/cpu-extensions/vector/`
**Deliverable:** Consolidated vector extensions reference file

---

## Phase 1D — Trusted Execution & Confidential Computing

**Scope:** Intel SGX, AMD SEV/SEV-ES/SEV-SNP, Intel TDX, AMD SME, Intel Key Locker, Intel TME, Intel TXT
**Agent:** researcher
**Status:** COMPLETE (2026-03-18) — 12 files written in `.claude/references/cpu-extensions/confidential-computing/`

**Actions:**
- Research each technology: architecture, attestation model, threat model
- Document Linux kernel support status and detection methods
- **Sysfs detection paths with exact value formats:**
  - SGX: `/dev/sgx_enclave` existence (device node, no magic check — different provenance model)
  - SEV: `/sys/module/kvm_amd/parameters/sev` (value: `1` or `Y`?)
  - SEV-ES: `/sys/module/kvm_amd/parameters/sev_es`
  - SEV-SNP: `/sys/module/kvm_amd/parameters/sev_snp`
  - TDX: `/sys/module/kvm_intel/parameters/tdx`
- Leverage existing kernel docs (`snp-tdx-threat-model.rst`, `amd-memory-encryption.rst`)
- **CVE tables required:**
  - SGX: Foreshadow/L1TF, SGAxe, CrossTalk, LVI, AEPIC Leak
  - SEV/SNP: CacheWarp, CVE-2021-26311 and related
  - TDX: early implementation issues
- SGX classification: **Important** with caveat — presence-without-use is attack surface, not assurance gain
- Document "recommended disposition when unused" for all TEE features
- Document feature-accessible vs feature-advertised (BIOS gates)
- Add Intel TXT (measured boot attestation) — distinct from TDX
- Add Intel TME (complement to AMD SME)

**Sources:** Intel SDM, AMD SEV developer docs, kernel docs, Intel/AMD Product Security Centers
**Save to:** `.claude/references/cpu-extensions/confidential-computing/`
**Deliverable:** Per-technology reference files + CVE/attack history summary

---

## Phase 1E — Speculative Execution Mitigations (NEW)

**Scope:** IBRS/eIBRS, IBPB, STIBP, SSBD, MDS/MD_CLEAR, L1D flush, PCID
**Agent:** researcher
**Status:** Research COMPLETE (2026-03-18) — files need writing. 11 files for `.claude/references/cpu-extensions/mitigations/`

**Actions:**
- Research each mitigation: CPUID detection, microcode requirements, Linux exposure
- **Primary detection interface:** `/sys/devices/system/cpu/vulnerabilities/`
  - Document every file in that directory and its interpretation
  - Document all possible text values and their meanings
  - This is more authoritative than `/proc/cpuinfo` for mitigation status
- Document connection to existing posture signals:
  - `SignalId::Mitigations` (Critical) — what CPU capabilities make each mitigation possible
  - `SignalId::Pti` (High) — document PCID and its role in making PTI low-overhead
- Microcode staleness risk: minimum microcode revision per mitigation
- Microcode version detection: `/proc/cpuinfo` `microcode` field, `/sys/devices/system/cpu/cpu0/microcode/version`
- All features classified **Critical/Defensive**
- Cross-reference with SMT: which mitigations are SMT-dependent

**Sources:** Intel SDM, AMD APM, Linux kernel vulnerability docs, Intel/AMD security advisories
**Save to:** `.claude/references/cpu-extensions/mitigations/`
**Deliverable:** Per-mitigation reference files + vulnerability sysfs interface reference

---

## Phase 1F — CPU-Enforced Access Controls (NEW)

**Scope:** SMEP, SMAP, CET-SS, CET-IBT, UMIP, NX/XD, PKU
**Agent:** researcher
**Status:** UNBLOCKED (2026-03-18) — CET docs acquired at `.claude/references/cpu-extensions/cet-docs/`. Not started.

**Actions:**
- Research each control: what attack class it blocks, CPUID location, Linux detection
- **CET deep dive:**
  - Shadow Stack + IBT architecture
  - RHEL 10 enablement status (kernel + userspace)
  - Binary verification method: `eu-readelf -n <binary>` for `.note.gnu.property`
  - `GNU_PROPERTY_X86_FEATURE_1_AND` with IBT and SHSTK bits
  - Per-process CET status via `/proc/<pid>/status`
  - Software utilization audit: binary compiled without `-fcf-protection=full` on CET-capable system = HIGH finding
- NX/XD: foundational W^X prerequisite — must be confirmed present
- SMEP/SMAP: document `CONFIG_X86_SMEP`/`CONFIG_X86_SMAP` kernel build dependency
- Classification: SMEP, SMAP, CET, NX/XD = **Critical/Defensive**; UMIP = **Important**; PKU = **Important**

**Sources:** Intel SDM, Intel CET specification, Linux `Documentation/x86/cet.rst`, RHEL 10 security advisories
**Save to:** `.claude/references/cpu-extensions/access-controls/`
**Deliverable:** Per-control reference files + CET binary verification guide

**Pre-requisite action:** Researcher must acquire Intel CET specification and Linux CET docs before starting.

---

## Phase 1G — Virtualization, Reliability & Platform Topology

**Scope:** VMX, SVM, nested paging, VT-d/AMD-Vi IOMMU, MCA, RAS, ECC, SMT topology
**Agent:** researcher
**Status:** COMPLETE (2026-03-18) — 6 files in `.claude/references/cpu-extensions/virt-reliability/`

**Actions:**
- Research virtualization security features and their CPU exposure
- **IOMMU (VT-d / AMD-Vi):**
  - Critical/Defensive — DMA attacks bypass kernel blacklisting without IOMMU
  - Connect to existing posture signals for Thunderbolt/FireWire blacklisting
  - Detection: `/sys/class/iommu/`, `/proc/cmdline` for `intel_iommu=on`
- Reliability/availability features (MCA, RAS, ECC) relevant to assurance
- **SMT as cross-cutting property:**
  - `/sys/devices/system/cpu/smt/active`
  - `nosmt` and `mitigations=nosmt` cmdline parameters
  - Security implications: affects STIBP, MDS, isolation posture
  - Not a feature in the traditional sense — platform topology with security impact
- **Microcode version tracking:**
  - `/proc/cpuinfo` `microcode` field
  - `/sys/devices/system/cpu/cpu0/microcode/version`
  - Mitigation correctness prerequisite

**Save to:** `.claude/references/cpu-extensions/virt-reliability/`
**Deliverable:** Consolidated reference files + SMT security assessment + IOMMU detection guide

---

## Phase 1H — `/proc/crypto` & Software Utilization Reference (NEW)

**Scope:** Document `/proc/crypto` interface, driver-to-feature mappings, OpenSSL FIPS chain
**Agent:** researcher
**Status:** COMPLETE (2026-03-18) — 4 files in `.claude/references/cpu-extensions/proc-crypto/`

**Actions:**
- **`/proc/crypto` format documentation:**
  - Entry structure (name, driver, module, priority, refcnt, selftest, fips_allowed, type, etc.)
  - How priority determines which implementation wins (hardware > software)
  - `selftest: passed` requirement for FIPS operation
  - `fips_allowed: yes/no` field semantics
- **Driver-to-feature mapping table:**
  - AES-NI → `aesni_intel` / `aes-ce`
  - PCLMULQDQ → `crct10dif-pclmul`, `ghash-clmulni-intel`
  - SHA → `sha256-ni`, `sha256_ssse3`, `sha256-avx2`
  - Generic fallbacks → `aes-generic`, `sha256-generic`
- **OpenSSL FIPS dependency chain on RHEL 10:**
  - `OPENSSL_ia32cap` environment variable — controls runtime feature use
  - FIPS provider validated code paths
  - CMVP module search for RHEL 10 validation certificate
- **Software fallback risk analysis:**
  - Software AES: cache-timing vulnerability, constant-time properties
  - Software SHA: timing characteristics
  - Per-feature: is the software fallback FIPS-validated?
- **Posture check specification:**
  - For each crypto extension: verify hardware driver in `/proc/crypto` with priority > software, selftest passed

**Sources:** Linux kernel `Documentation/crypto/`, CMVP Module Search, OpenSSL FIPS docs
**Save to:** `.claude/references/cpu-extensions/proc-crypto/`
**Deliverable:** `/proc/crypto` reference + driver mapping table + OpenSSL FIPS chain doc + software fallback risk matrix

---

## Phase 1I — CPU Feature Matrix Synthesis

**Scope:** Synthesize all Phase 1A-1H material into the 23-column master matrix
**Agent:** researcher
**Status:** Not started — requires 1A-1H complete

**Actions:**
- Build master matrix with all 23 columns
- Assign audit-card relevance per feature (Critical/Defensive, Critical/Operational, Important, Informational)
- Produce Linux detection reference sheet (all paths, all magic checks, all value formats)
- Produce audit-card summary recommendations
- Produce paired-signal reference (Layer 1 hardware signal → Layer 2 utilization signal)
- Produce ARM/AArch64 equivalents appendix
- Knowledge index entry for agent pre-seeding

**Input:** All Phase 1A-1H deliverables
**Save to:** `.claude/references/cpu-extensions/cpu-feature-matrix.md`
**Deliverable:** Master matrix + detection sheet + audit-card recommendations + knowledge index

---

## Phase 1J — Post-Research Review

**Scope:** cpu-post-research.md stages 1-9
**Agents:** security-engineer, rust-developer (parallel)
**Status:** Not started — requires 1I complete

**Actions:**
- Each agent reviews the complete corpus (stages 1-5)
- Gap analysis and feedback (stages 4-6)
- rust-developer:
  - Propose `CpuProbe` data structures compatible with existing `SignalId`/posture catalog
  - Propose `CpuFeatureDescriptor` type (or `SignalDescriptor` extension)
  - Propose new `SignalClass` variants: `CpuProcInfo`, `CpuSysfs`, `CpuVulnSysfs`, `TeeDevfs`
  - Propose new `DesiredValue` variants: `FlagPresent`, `DeviceAccessible`
  - Propose new `ContradictionKind` variant: `CapabilityUnused` (Layer 1 yes, Layer 2 no)
  - Stage 7: CPU audit card layout
- security-engineer:
  - Finalize Critical/Defensive vs Critical/Operational classifications (stage 8)
  - Verify `/proc/crypto` posture check specifications
  - Verify CET binary verification method
  - Review software fallback risk assessments

**Input:** All Phase 1A-1I deliverables
**Deliverable:** Review reports + gap analysis + data structure proposals + classification finalization

---

## Phase 1K — Corpus Refinement

**Scope:** Address gaps identified in Phase 1J
**Agent:** researcher
**Status:** Not started — requires 1J feedback

**Actions:**
- Fill gaps identified by reviewing agents
- Update matrix with corrections and classification finalization
- RAG ingest the finalized corpus
- Update knowledge index

**Deliverable:** Final corpus + RAG ingestion complete

---

## Authoritative Sources to Acquire

Sources the researcher must fetch that are NOT already in the repo:

| Source | Required For | Priority | URL |
|---|---|---|---|
| NIST SP 800-90B | RDRAND/RDSEED classification | **BLOCKING Phase 1B** | `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf` |
| Intel CET specification | CET research (Phase 1F) | High | Intel developer docs |
| Linux `Documentation/x86/cet.rst` | CET detection paths | High | kernel source tree |
| Intel SDM Vol 2A Ch 3 | CPUID reference (all phases) | High | Intel developer portal |
| AMD APM Vol 3 App E | CPUID reference (all phases) | High | AMD developer portal |
| NSA RHEL hardening guidance | Audit card alignment | Medium | NSA/CNSS |
| NIST SP 800-155 | Firmware/measured boot | Medium | NIST |
| NIST SP 800-193 | Microcode update resilience | Medium | NIST |
| Intel Product Security Center | SGX/TME CVE history | High | Intel security center |
| AMD Product Security | SEV/SNP CVE history | High | AMD security page |
| CMVP Module Search (RHEL 10) | FIPS validation certificate | Medium | NIST CMVP |

**Note:** DoD STIGs for RHEL 10 may require manual browser download (DoD portals block curl). Flag to Jamie if needed.

---

## Execution Order

| Priority | Phase | Description | Dependencies | Parallelizable With |
|---|---|---|---|---|
| **0 (done)** | Phase 0 | rust-developer + security-auditor spec review | None | — |
| **1 (next)** | Phase 0.5 | Update cpu-matrix.md spec | Phase 0 | — |
| **2** | Phase 1A | Crypto acceleration | Phase 0.5 | Phase 1B (if 800-90B acquired), 1C |
| **2** | Phase 1B | Entropy & big integer | Phase 0.5 + 800-90B | Phase 1A, 1C |
| **2** | Phase 1C | Vector extensions | Phase 0.5 | Phase 1A, 1B |
| **3** | Phase 1D | TEE & confidential computing | Phase 0.5 | Phase 1E, 1F |
| **3** | Phase 1E | Speculative execution mitigations | Phase 0.5 | Phase 1D, 1F |
| **3** | Phase 1F | CPU-enforced access controls | Phase 0.5 + CET docs | Phase 1D, 1E |
| **4** | Phase 1G | Virt, reliability, topology | Phase 0.5 | Phase 1H |
| **4** | Phase 1H | `/proc/crypto` & software utilization | Phase 0.5 | Phase 1G |
| **5** | Phase 1I | Matrix synthesis | 1A-1H complete | — |
| **6** | Phase 1J | Post-research review | 1I complete | — |
| **7** | Phase 1K | Corpus refinement | 1J feedback | — |

**Within each priority level, phases can run in parallel across sessions.**

---

## Post-Phase Hygiene

After each phase completes:
- Researcher logs to `.claude/logs/task-log.md`
- Update this plan with phase status
- Move completed source files to `.claude/jamies_brain/archive/` when all their phases finish
- Notify Jamie of anything requiring manual download (DoD portals, etc.)
- Update `.claude/references/` tracking if new collections created
