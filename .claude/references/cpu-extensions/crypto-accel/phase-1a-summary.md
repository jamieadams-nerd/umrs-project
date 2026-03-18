# Phase 1A Summary — Crypto Acceleration Extensions

**Completed:** 2026-03-18
**Agent:** researcher (research), main orchestrator (file writing)
**Scope:** AES-NI, VAES, SHA-NI, PCLMULQDQ/CLMUL + ARM equivalents

## Features Documented

| Feature | Classification | Key Finding |
|---------|---------------|-------------|
| AES-NI | Critical/Operational (FIPS) | No hardware CVEs; eliminates cache-timing side channels in software fallback |
| VAES | Informational | Up to 162% AES-GCM improvement; performance optimization only |
| SHA-NI | Important | 3–4x SHA-256 speedup; covers SHA-1 and SHA-256 only (no SHA-512) |
| PCLMULQDQ | Important | Co-dependent with AES-NI for AES-GCM; GHASH bottleneck without it |
| ARM AES/PMULL | Critical/Operational (FIPS) | Equivalent to AES-NI + PCLMULQDQ combined |
| ARM SHA | Important | ARM has dedicated SHA-512 instructions; x86 does not |

## Cross-Cutting Findings

### 1. The AES-NI + PCLMULQDQ Pair
AES-NI and PCLMULQDQ are **co-dependent for AES-GCM**. A posture check must verify BOTH are present and utilized — AES-NI alone gives fast encryption but slow GHASH authentication.

### 2. `/proc/crypto` as Layer 2 Detection
The `/proc/crypto` interface is the authoritative source for verifying hardware acceleration is actually in use:
- Check driver names: `aesni_intel`, `ghash-clmulni-intel`, `sha256-ni`
- Check priority: hardware driver must have higher priority than `*-generic`
- Check `selftest: passed` for FIPS compliance

### 3. Virtualization CPUID Masking
ALL crypto extension CPUID bits can be masked by hypervisors (KVM, VMware, VirtualBox). A guest cannot independently verify hardware-backed crypto without either:
- Checking `/proc/crypto` driver names (best available heuristic)
- TEE attestation (Phase 1D)

### 4. Software Fallback Risk Hierarchy
| Fallback | Timing-safe? | FIPS? | Risk Level |
|----------|-------------|-------|------------|
| AES T-table (`aes-generic`) | NO | Yes (weak) | **HIGH** |
| AES VPAES/BSAES | Yes | Yes | Low |
| SHA-256 generic | Yes (no timing dependency) | Yes | Low |
| GHASH generic | Yes | Yes | Low (but slow) |

AES software fallback is the only **HIGH** risk — T-table implementations leak key material via cache timing.

### 5. ARM Advantage: SHA-512
ARM has dedicated SHA-512 hardware instructions (ARMv8.2+) while x86 relies on AVX2 software implementations. This is relevant for UMRS's cross-platform posture assessment.

### 6. Rust Detection Path
All features detectable via safe `std::is_x86_feature_detected!()` / `std::is_aarch64_feature_detected!()` macros. No unsafe code or raw CPUID needed. The `cpufeatures` crate from RustCrypto is a well-maintained alternative.

**Note:** `raw-cpuid` crate has a known soundness issue (RUSTSEC-2021-0013). Prefer `std` macros or `cpufeatures` crate.

## Posture Check Specifications (for future CPU probe)

### AES Hardware Acceleration Check
1. Verify `aes` flag in `/proc/cpuinfo` (Layer 1)
2. Verify `aesni_intel` (x86) or `aes-ce` (ARM) in `/proc/crypto` with priority > `aes-generic` (Layer 2)
3. Verify `selftest: passed` on the hardware driver entry
4. If Layer 1 = present but Layer 2 = software → **HIGH finding** (hardware present but unused)

### GHASH Acceleration Check
1. Verify `pclmulqdq` flag (x86) or `pmull` flag (ARM) in `/proc/cpuinfo`
2. Verify `ghash-clmulni-intel` (x86) or `ghash-ce` (ARM) in `/proc/crypto`
3. If AES-NI present but GHASH acceleration absent → **MEDIUM finding** (AES-GCM partially hardware-accelerated)

### SHA Acceleration Check
1. Verify `sha_ni` (x86) or `sha2` (ARM) in `/proc/cpuinfo`
2. Verify `sha256-ni` (x86) or `sha256-ce` (ARM) in `/proc/crypto`
3. Absence is INFORMATIONAL — software SHA-256 is not timing-vulnerable

## Files Produced

1. `aes-ni.md` — Full 23-column profile + CVE table + `/proc/crypto` mapping
2. `vaes.md` — Full 23-column profile
3. `sha-ni.md` — Full 23-column profile + coverage note (no SHA-512)
4. `pclmulqdq.md` — Full 23-column profile + AES-NI co-dependency analysis
5. `arm-crypto-equivalents.md` — ARM AES/SHA/RNG + cross-platform detection summary
6. `phase-1a-summary.md` — This file

## Sources

All source citations are in individual feature files. Key sources:
- Intel AES-NI White Paper
- Intel CLMUL White Paper
- Intel SHA Extensions documentation
- ARM Architecture Reference Manual (ID_AA64ISAR0_EL1)
- Bernstein cache-timing paper (2005)
- minio/sha256-simd benchmarks
- Linux kernel crypto subsystem documentation
- RHEL 10 FIPS mode documentation
- RustCrypto cpufeatures crate
- RUSTSEC-2021-0013 (raw-cpuid soundness advisory)
