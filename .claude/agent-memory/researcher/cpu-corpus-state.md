---
name: CPU security corpus progress
description: Phase completion status and key findings for CPU security corpus plan
type: project
---

# CPU Security Corpus — Phase Status

**Plan:** `.claude/plans/cpu-security-corpus-plan.md`
**Last updated:** 2026-03-18 (Phase 1D complete)

## Phase Completion

| Phase | Status | Files | Location |
|-------|--------|-------|----------|
| 0 (reviews) | DONE | 2 review reports | `.claude/reports/cpu-matrix-review/` |
| 0.5 (spec update) | DONE | cpu-matrix.md v3 | `.claude/references/cpu-extensions/` |
| 1A (crypto accel) | DONE 2026-03-18 | 6 files | `.claude/references/cpu-extensions/crypto-accel/` |
| 1B (entropy/bigint) | DONE 2026-03-18 | 5 files (rdrand, rdseed, arm-rng, adx-bmi, summary) | `.claude/references/cpu-extensions/entropy-bigint/` |
| 1C (vector ext) | DONE 2026-03-18 | 4 files (sse-family, avx-family, avx512, phase-1c-summary) | `.claude/references/cpu-extensions/vector/` |
| 1D (TEE/confidential) | DONE 2026-03-18 | 12 files (intel-sgx, intel-txt, intel-tdx, intel-tme, intel-key-locker, amd-sev, amd-sev-es, amd-sev-snp, amd-sme, arm-trustzone, cve-summary, phase-1d-summary) | `.claude/references/cpu-extensions/confidential-computing/` |
| 1E (spec mitigations) | Research done; files not yet written | 11 files needed | `.claude/references/cpu-extensions/mitigations/` |
| 1F (access controls) | NOT STARTED; CET docs acquired | — | `.claude/references/cpu-extensions/access-controls/` |
| 1G (virt/reliability) | DONE 2026-03-18 | 6 files (iommu, nested-paging, mca-ras-ecc, smt-topology, microcode-tracking, phase-1g-summary) | `.claude/references/cpu-extensions/virt-reliability/` |
| 1H (/proc/crypto) | DONE 2026-03-18 | 5 files (proc-crypto-format, driver-feature-mapping, openssl-fips-chain, software-fallback-risk, phase-1h-summary) | `.claude/references/cpu-extensions/proc-crypto/` |
| 1I (synthesis) | NOT STARTED | — | — |
| 1J/1K (review/refinement) | NOT STARTED | — | — |

## Key Phase 1B Findings

- AMD-SB-7055 / CVE-2025-62626: Zen 5 RDSEED 16/32-bit returns 0 with CF=1; AGESA fix Nov 2025
- SRBDS (CVE-2020-0543): applies to both RDRAND and RDSEED; Intel microcode fix June 2020
- ARMv8.5-RNG: RNDR=RDRAND analog, RNDRRS=RDSEED analog; `HWCAP2_RNG` via `getauxval(AT_HWCAP2)`
- ADX+MULX+ADCX+ADOX = ~20-25% RSA/ECC improvement; no CVEs; Informational classification
- BMI2 PDEP/PEXT: 18-cycle latency on AMD pre-Zen 3 (microcode); timing risk in constant-time code

## Key Phase 1C Findings

- SSSE3 PSHUFB enables VPAES (constant-time software AES); absence on a FIPS system without AES-NI = HIGH finding
- SSE3 `/proc/cpuinfo` flag is `pni` NOT `sse3` — common detection bug
- AVX2 + `sha256-avx2` driver = 4-lane SHA-256; practical ceiling for cloud-deployed UMRS
- AVX-512 VAES+VPCLMULQDQ: 4 AES blocks/instr + 4-wide GHASH; 5–8 GB/s AES-GCM per core
- Skylake-X frequency penalty: 2.5% AVX-512 workload → all-core freq drops 400 MHz (Cloudflare 2018)
- Ice Lake+, AMD Zen 4+: no frequency penalty; cloud VMs routinely mask `avx512f` entirely
- VAES operates on both YMM (AMD Zen 3+) and ZMM (Ice Lake+); not exclusively AVX-512
- KVM: `cpu host` passes all flags; named models must be `Cascadelake-Server` or later for VAES+VPCLMULQDQ
- VMware EVC with Broadwell baseline: AVX-512 entirely masked; mixed Skylake/Ice Lake clusters lose VAES+ZMM
- `sha256_ssse3` kernel module registers SSSE3/AVX/AVX2 drivers at load based on CPUID; all can appear simultaneously in `/proc/crypto`

## New Signals Proposed (for future CPU probe)

- SRBDS mitigation status: `/sys/devices/system/cpu/vulnerabilities/srbds`
- AMD Zen 5 RDSEED firmware: `/sys/class/dmi/id/bios_date` vs AGESA fix date
- RNDR kernel config: `CONFIG_ARCH_RANDOM=y` in kernel build config
- ADX present: `/proc/cpuinfo: adx` (Informational)

## Key Phase 1D Findings

- SEV base (without SNP) is architecturally broken for integrity — SEVered (2018) extracts full VM plaintext; only SEV-SNP is acceptable for production
- Intel SGX has 7+ significant CVEs (2018-2022); presence-without-use is attack surface; deprecated on consumer CPUs; DISABLE IN BIOS when unused
- Intel TDX: Google Project Zero found CVSS 9.3 ACM bug (1.0) and full compromise CVE-2025-30513 (1.5); all fixed pre/post-release; heavy audit scrutiny
- CVE-2024-56161 (AMD microcode sig bypass): CRC32 used for microcode signature verification; local admin loads arbitrary microcode; undermines ALL CPU security guarantees
- BIOS gates are ubiquitous: every Phase 1D technology except TrustZone has a BIOS/firmware gate between CPUID and actual availability
- TME/SME: near-zero performance cost, should be enabled on all capable systems; TSME preferred for simplicity
- Intel Key Locker: Linux kernel support NOT YET MAINLINED (v9 patches from March 2024); Informational classification
- ARM TrustZone: hardware is sound; vulnerabilities concentrate in vendor TEE implementations (QSEE, Kinibi); OP-TEE has better track record
- TDX host support merged in Linux 6.16 (KVM); guest support since 5.19
- Detection requires multi-path: CPUID (capability) + sysfs/module param (OS activation) + device node (functional) + dmesg (boot activation)
- WeSee attack (2024): #VC interrupt injection bypasses SEV-SNP GHCB handler; guest kernel hardening required
- CacheWarp (CVE-2023-20592): affects EPYC 1st-3rd gen only; Genoa (4th gen)+ has architectural fix

## New Signals Proposed from Phase 1D

- `SevSnpActive` (Critical/Operational): `/sys/module/kvm_amd/parameters/sev_snp` = `Y`
- `TdxActive` (Critical/Operational): `/sys/module/kvm_intel/parameters/tdx` = `Y`
- `SmeActive` (Important): dmesg SME/TSME active
- `TmeActive` (Important): dmesg TME active
- `SgxPresent` (Important): `/dev/sgx_enclave` exists
- `SgxUnusedAttackSurface` (HIGH compound): SGX present + no active enclaves + not BIOS-disabled
- `SevWithoutSnp` (MEDIUM compound): SEV active + SNP not active
- `TxtMeasuredBoot` (Important): TPM PCR[17] non-zero
- `TrustZoneTeeAvailable` (Important, AArch64): `/dev/tee0` exists

## Key Phase 1G Findings

- IOMMU is hardware prerequisite for Thunderbolt/FireWire blacklisting signals; without IOMMU, kernel module blacklisting can be bypassed via DMA
- IOMMU detection: `/sys/kernel/iommu_groups/` (primary); `/sys/class/iommu/` (device list); `intel_iommu=on` required in cmdline for Intel
- SMT is not a vulnerability but determines whether MDS/L1TF/STIBP mitigations provide full isolation; must be factored into Phase 1E audit logic
- Microcode staleness is invisible from `/sys/devices/system/cpu/vulnerabilities/` — MD_CLEAR is the clearest example: pre-fix microcode + kernel VERW = does nothing
- `mcelog` is ABSENT on RHEL 9/10; `rasdaemon` is the supported RAS tool; Secure Boot confidentiality lockdown breaks rasdaemon (use integrity mode)
- EPT/NPT: Intel EPT supports execute-only pages; AMD NPT does not — asymmetric hypervisor hardening capability
- Nested VMX/SVM: enabled by default OFF in KVM; enabling it on non-hypervisor hosts is a HIGH finding
- AArch64 specifics: no SMT on Neoverse N1/N2; IOMMU = ARM SMMU v3; microcode = TF-A/UEFI firmware version (not /proc/cpuinfo field)

## New Signals Proposed from Phase 1G

- `IOMMU_ACTIVE` (Critical/Defensive): `/sys/kernel/iommu_groups/` non-empty; pairs with Thunderbolt/FireWire blacklist signals
- `MICROCODE_CURRENT` (Important): `microcode_ctl` RPM at latest RHEL version; microcode staleness = invisible mitigation gap
- `ECC_ACTIVE` (Important): `dmidecode -t 17` ECC type + `rasdaemon` active + EDAC loaded
- `SMT_MITIGATIONS_COMPLETE` (Important, compound): SMT active → verify MDS/L1TF/STIBP all confirmed

## Key Phase 1H Findings

- `/proc/crypto` is the authoritative Layer 2 interface for software utilization of crypto extensions
- T-table AES (`aes-generic`) is FIPS-validated but timing-vulnerable — treat as CRITICAL finding when hardware AES-NI is present but unused
- SHA-2 generic is inherently constant-time (no table lookups, no key-dependent branches) — SHA-NI absence is a LOW/performance-only finding
- VPAES (SSSE3-based) is a constant-time software AES fallback; on modern RHEL 10 targets, AES-NI absent → VPAES (not T-table)
- RHEL 10 OpenSSL FIPS provider (openssl-fips-provider-3.0.7-6.el10) is in "Pending OE update" — same binary as RHEL 9 CMVP #4857
- `OPENSSL_ia32cap` controls OpenSSL hardware feature selection independently from `/proc/crypto`; its presence masking AES-NI in production is a Critical config finding
- `fips_allowed: yes` on hardware drivers (`aesni_intel`, `ghash-clmulni-intel`) required explicit kernel patches; present in RHEL 9/10 kernels
- GHASH (`ghash-clmulni-intel`) is PCLMULQDQ-dependent; absent GHASH = software GCM authentication = MEDIUM finding
- ARM equivalents: `aes-ce` (priority 200), `sha256-ce` (200), `ghash-ce` (200) — same structural role, lower priority numbers than x86

## New Signals Proposed from Phase 1H

- `CryptoDriverAesNi` (Critical/Operational Layer 2): `/proc/crypto` aesni_intel present+passed+fips_allowed
- `CryptoDriverAesGcmNi` (Critical/Operational Layer 2): generic-gcm-aesni + ghash-clmulni-intel both present
- `CryptoDriverSha256Ni` (Important Layer 2): sha256-ni present+passed
- `CryptoSelftestAllPassed` (Critical Layer 2): no selftest:failed entries in /proc/crypto
- `CryptoFipsConsistent` (Critical): fips_enabled=1 ↔ hardware drivers fips_allowed=yes aligned
- New contradiction kind proposed: `ContradictionKind::CapabilityUnused` (Layer 1 YES, Layer 2 NO)

## CET Docs Acquired

- 4 files in `.claude/references/cpu-extensions/cet-docs/` — prerequisite for Phase 1F
