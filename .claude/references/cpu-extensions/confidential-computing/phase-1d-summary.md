# Phase 1D Summary — Trusted Execution & Confidential Computing

## Scope

Phase 1D covers 12 technologies across 4 categories:
- **Trusted Execution & Enclaves:** Intel SGX, Intel TXT
- **Confidential Computing & Encrypted Virtualization:** AMD SEV, AMD SEV-ES, AMD SEV-SNP, Intel TDX
- **Memory Encryption:** AMD SME/TSME, Intel TME/MKTME
- **Key Protection:** Intel Key Locker
- **ARM Equivalent:** ARM TrustZone
- **Cross-cutting:** CVE/attack history consolidation

## Files Produced

| File | Technology | Classification |
|------|-----------|---------------|
| `intel-sgx.md` | Intel SGX | Important (attack-surface caveat) |
| `intel-txt.md` | Intel TXT | Important |
| `intel-tdx.md` | Intel TDX | Critical/Operational |
| `intel-tme.md` | Intel TME/MKTME | Important |
| `intel-key-locker.md` | Intel Key Locker | Informational |
| `amd-sev.md` | AMD SEV | Important |
| `amd-sev-es.md` | AMD SEV-ES | Important |
| `amd-sev-snp.md` | AMD SEV-SNP | Critical/Operational |
| `amd-sme.md` | AMD SME/TSME | Important |
| `arm-trustzone.md` | ARM TrustZone | Important |
| `cve-summary.md` | Consolidated CVE table | N/A (reference) |
| `phase-1d-summary.md` | This file | N/A (summary) |

---

## Detection Paths Summary

### Host-Side Detection

| Technology | Primary Detection Path | Value Format | Authoritative? |
|------------|----------------------|--------------|----------------|
| Intel SGX | `/dev/sgx_enclave` device node | exists/absent | Yes (device node) |
| Intel TXT | `/proc/cpuinfo` `smx` flag + `dmesg` tboot messages | flag + boot log | Partial (SMX=capability, dmesg=active) |
| Intel TDX | `/sys/module/kvm_intel/parameters/tdx` | `Y`/`N` | Yes (module parameter) |
| Intel TME | `dmesg` for "x86/tme: enabled" | boot log | Partial (no sysfs) |
| Intel Key Locker | `/proc/cpuinfo` `kl`, `aeskle` | flags | Layer 1 only (no kernel driver yet) |
| AMD SEV | `/sys/module/kvm_amd/parameters/sev` | `Y`/`N` | Yes (module parameter) |
| AMD SEV-ES | `/sys/module/kvm_amd/parameters/sev_es` | `Y`/`N` | Yes (module parameter) |
| AMD SEV-SNP | `/sys/module/kvm_amd/parameters/sev_snp` | `Y`/`N` | Yes (module parameter) |
| AMD SME | `/proc/cpuinfo` `sme` + `dmesg` + `/proc/cmdline` `mem_encrypt=` | flag + log + cmdline | Combined |
| ARM TrustZone | `/dev/tee0` device node | exists/absent | TEE presence (TZ always present) |

### Guest-Side Detection (Inside Confidential VM)

| Technology | Guest Detection | Attestation Mechanism |
|------------|----------------|----------------------|
| Intel SGX (enclave) | `/dev/sgx_enclave` in guest | DCAP remote attestation |
| Intel TDX (TD) | CPUID leaf 0x21, `/dev/tdx_guest` | DCAP (Intel infrastructure) |
| AMD SEV (VM) | `dmesg`, MSR 0xc0010131 bit 0 | LAUNCH_MEASURE (initial only) |
| AMD SEV-ES (VM) | `dmesg`, MSR 0xc0010131 bit 1 | LAUNCH_MEASURE (initial only) |
| AMD SEV-SNP (VM) | `/dev/sev-guest`, MSR bit 2 | VCEK/VLEK signed attestation report |
| ARM TrustZone | N/A (TZ is not per-VM) | Platform-specific (PSA) |

---

## Classification Rationale Summary

### Critical/Operational (2)

| Technology | Rationale |
|------------|-----------|
| **AMD SEV-SNP** | Hardware root of trust for AMD confidential VMs. Provides encryption + integrity (RMP) + attestation (VCEK/VLEK). Absence breaks the deployment model. |
| **Intel TDX** | Hardware root of trust for Intel confidential VMs. Trust Domains with hardware encryption + register protection + attestation (DCAP). Absence breaks the deployment model. |

### Important (7)

| Technology | Rationale |
|------------|-----------|
| **Intel SGX** | Strong isolation in theory but extensive CVE history. Deprecated on consumer. Presence-without-use = attack surface. |
| **Intel TXT** | DRTM attestation strengthens platform integrity but is not required (Secure Boot provides baseline). |
| **Intel TME** | Transparent memory encryption against physical attacks. Near-zero cost, defense-in-depth. |
| **AMD SEV** | Foundation for SEV-ES/SNP but lacks integrity protection. SEVered demonstrates insufficiency. |
| **AMD SEV-ES** | Register state protection but retains SEV's memory integrity weakness. |
| **AMD SME/TSME** | Transparent memory encryption, Intel TME equivalent. Defense-in-depth. |
| **ARM TrustZone** | Architectural TEE, always present on Cortex-A. Security depends on TEE OS quality. |

### Informational (1)

| Technology | Rationale |
|------------|-----------|
| **Intel Key Locker** | AES key handle protection. No kernel support yet. Not widely deployed. No compliance requirement. |

---

## Disposition When Unused — Recommendations

| Technology | Recommendation | Priority | Rationale |
|------------|---------------|----------|-----------|
| Intel SGX | **DISABLE IN BIOS** | HIGH | Presence-without-use is attack surface (Foreshadow, SGAxe, AEPIC Leak target SGX-specific state). |
| Intel TXT | Leave enabled | LOW | Minimal attack surface when dormant. |
| Intel TDX | Leave enabled | LOW | Dormant without `kvm_intel.tdx=1`. No attack surface. |
| Intel TME | **ENABLE IN BIOS** | MEDIUM | Near-zero cost, defense-in-depth against physical attacks. |
| Intel Key Locker | Leave enabled | NONE | Dormant without OS support. |
| AMD SEV | Leave enabled | LOW | Dormant without KVM activation. |
| AMD SEV-ES | Leave enabled | LOW | Same as SEV. |
| AMD SEV-SNP | Leave enabled | LOW | Dormant without KVM RMP initialization. |
| AMD SME/TSME | **ENABLE IN BIOS (TSME)** | MEDIUM | Near-zero cost, defense-in-depth. |
| ARM TrustZone | N/A (cannot disable) | N/A | Architectural feature. |

---

## BIOS Gate Analysis

Every technology in Phase 1D has a BIOS/firmware gate between CPUID capability and actual availability:

| Technology | CPUID Says Yes But... | Gate |
|------------|----------------------|------|
| Intel SGX | BIOS must enable + allocate EPC + set FLC MSRs | BIOS: "SGX" enable + PRMRR config |
| Intel TXT | Chipset must be TXT-capable + BIOS enable + TPM present | BIOS: "TXT" enable + TPM |
| Intel TDX | BIOS must load TDX module + allocate CMR | BIOS: "TDX" enable + firmware load |
| Intel TME | BIOS must set IA32_TME_ACTIVATE MSR | BIOS: "TME" enable |
| Intel Key Locker | OS must set CR4.KL + LOADIWKEY | OS gate (no BIOS gate) |
| AMD SEV/ES/SNP | BIOS must enable in AMD-SP firmware + (SNP: allocate RMP) | BIOS: "SEV"/"SNP" enable + RMP reservation |
| AMD SME | BIOS must set MSR_AMD64_SYSCFG bit 23 | BIOS: "Memory Encryption" enable |
| ARM TrustZone | Always present (no gate) | TEE OS must be loaded by secure boot chain |

**Implication for UMRS:** A CPUID flag alone is insufficient to determine active capability. The audit card must verify the BIOS gate for every technology. This requires a combination of:
1. CPUID check (capability)
2. Sysfs/module parameter check (OS-level activation)
3. Device node check (functional availability)
4. dmesg/boot log check (actual activation)

---

## New Signal Proposals

Based on Phase 1D research, the following signals are proposed for the future CPU probe:

### Critical/Operational Signals

| Signal ID (proposed) | Detection Path | Classification | Rationale |
|---------------------|---------------|---------------|-----------|
| `SevSnpActive` | `/sys/module/kvm_amd/parameters/sev_snp` = `Y` | Critical/Operational | Required for AMD confidential VM deployments |
| `TdxActive` | `/sys/module/kvm_intel/parameters/tdx` = `Y` | Critical/Operational | Required for Intel confidential VM deployments |

### Important Signals

| Signal ID (proposed) | Detection Path | Classification | Rationale |
|---------------------|---------------|---------------|-----------|
| `SmeActive` | `dmesg` SME/TSME active | Important | Physical attack mitigation |
| `TmeActive` | `dmesg` TME active | Important | Physical attack mitigation |
| `SgxPresent` | `/dev/sgx_enclave` exists | Important | Track for disposition recommendation |
| `TxtMeasuredBoot` | TPM PCR[17] non-zero | Important | DRTM attestation active |
| `TrustZoneTeeAvailable` | `/dev/tee0` exists | Important | TEE services available (AArch64) |

### Informational Signals

| Signal ID (proposed) | Detection Path | Classification | Rationale |
|---------------------|---------------|---------------|-----------|
| `KeyLockerCapable` | `/proc/cpuinfo` `kl` flag | Informational | Future capability tracking |

### Compound/Contradiction Signals

| Signal ID (proposed) | Condition | Finding Level | Rationale |
|---------------------|-----------|---------------|-----------|
| `SgxUnusedAttackSurface` | SGX present + no active enclaves + not disabled in BIOS | HIGH | SGX unused = attack surface |
| `SevWithoutSnp` | SEV active + SNP not active | MEDIUM | SEV alone lacks integrity protection |
| `ConfidentialVmWithoutAttestation` | SEV-SNP or TDX active + no attestation infrastructure | MEDIUM | Confidential computing without verification |

---

## Cross-Phase Dependencies

### From Phase 1D to other phases:

| Dependency | Target Phase | Description |
|-----------|-------------|-------------|
| L1TF/Foreshadow mitigation | 1E (Mitigations) | SGX CVE-2018-3615 is addressed by L1D flush, which is a Phase 1E mitigation |
| SRBDS/CrossTalk mitigation | 1E (Mitigations) | SGX CVE-2020-0543 is addressed by VERW-based MD_CLEAR |
| IOMMU for DMA protection | 1G (Virt/Reliability) | TrustZone TZASC + IOMMU for DMA protection |
| `/proc/crypto` Layer 2 | 1H (proc/crypto) | Key Locker `aeskl-intel` driver (future) |
| SMT impact on isolation | 1G (Topology) | SMT affects side-channel exposure for SGX/SEV |

### From other phases to Phase 1D:

| Source Phase | Dependency | Description |
|-------------|-----------|-------------|
| 1A (Crypto) | AES-NI for TME/SME | TME and SME use AES-128 in memory controller |
| 1B (Entropy) | RDRAND for key gen | TME, SGX, Key Locker IWK all depend on hardware RNG |
| 1G (Microcode) | Microcode currency | CacheWarp, CVE-2024-56161 fixes require microcode updates |

---

## Key Findings

1. **Integrity protection is non-negotiable.** The SEV base attack history (SEVered, Undeadattack) proves that memory encryption without integrity is insufficient. Any confidential computing assessment must verify integrity mechanisms: RMP (SEV-SNP) or TDX module integrity checks.

2. **SGX should be disabled when unused.** Unlike other TEE technologies, SGX increases attack surface when present but not used. This is unique among Phase 1D technologies and should be a HIGH finding on audit cards.

3. **BIOS gates are ubiquitous.** Every Phase 1D technology except TrustZone has a BIOS gate. CPUID alone never confirms active capability. Multi-path detection (CPUID + sysfs + device node + dmesg) is required.

4. **CVE-2024-56161 (AMD microcode) is cross-cutting.** It undermines ALL AMD CPU security guarantees, not just SEV-SNP. Microcode integrity verification should be elevated to a platform-level signal.

5. **TDX audit results are a double-edged sword.** The willingness to submit to external audit is positive; the severity of findings (CVSS 9.3 in 1.0, full compromise in 1.5) indicates that the TDX module is complex and error-prone. Ongoing scrutiny is required.

6. **ARM TrustZone security is implementation-dependent.** The hardware is architecturally sound. Vendor TEE implementations (QSEE, Kinibi) are the primary vulnerability surface. OP-TEE (open source) has a better track record.

7. **Memory encryption (TME/SME) is low-cost, high-value.** Near-zero performance overhead, defense-in-depth against physical attacks. Should be enabled on all capable systems.

## Sources

See individual technology files for detailed per-technology source lists.
