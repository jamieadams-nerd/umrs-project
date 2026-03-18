# ARM/AArch64 Cryptography Equivalents

## Overview

ARM's cryptography extensions are optional features in the ARMv8-A architecture, detected
via the `AT_HWCAP` and `AT_HWCAP2` auxiliary vectors (not CPUID). Linux exposes these as
flags in `/proc/cpuinfo` under the `Features` line.

## ARMv8 AES Extension — Critical/Operational (FIPS)

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ARMv8 Cryptography Extension — AES |
| 2 | Vendor | ARM |
| 3 | Detection | `ID_AA64ISAR0_EL1` register, AES field (bits [7:4]): 0b0001 = AESE/AESD/AESMC/AESIMC; 0b0010 = also PMULL/PMULL2 |
| 4 | Linux `/proc/cpuinfo` flag | `aes` (AES), `pmull` (polynomial multiply for GHASH) |
| 5 | Key instructions | AESE, AESD, AESMC, AESIMC, PMULL, PMULL2 |
| 6 | Introduced | ARMv8.0-A (optional), 2011 specification; widely available from Cortex-A53 (2012) onward |
| 7 | Security relevance | Same as x86 AES-NI — eliminates cache-timing side channels in software AES. PMULL provides GHASH acceleration equivalent to x86 PCLMULQDQ. |
| 8 | Performance benefit | Similar magnitude to AES-NI: 3–8x over software AES depending on implementation |
| 9 | Known vulnerabilities | No known hardware CVEs against the ARM AES instructions |
| 10 | Compliance mapping | NIST SP 800-53 SC-13; FIPS 140-2/3 |
| 11 | Classification | **Critical/Operational (FIPS)** |
| 12 | Classification rationale | Same rationale as AES-NI — FIPS crypto path dependency |
| 13 | Linux kernel support | Kernel module: `aes-ce` (cipher), `aes-ce-ccm`, `aes-ce-blk`. Config: `CONFIG_CRYPTO_AES_ARM64_CE`. |
| 14 | Detection method (safe Rust) | `std::is_aarch64_feature_detected!("aes")` (safe); or parse `/proc/cpuinfo` Features line for `aes` |
| 15 | Virtualization confidence | Similar masking risk — QEMU/KVM can expose or hide hwcaps |
| 16 | x86 equivalent | AES-NI + PCLMULQDQ |
| 17 | References | ARM Architecture Reference Manual; ARM Developer ID_AA64ISAR0_EL1 documentation |
| 18 | Disposition when unused | Same as AES-NI: INVESTIGATE if `aes-generic` wins priority in `/proc/crypto` |
| 19 | Software utilization detection | `/proc/crypto`: look for driver `aes-ce` with priority > `aes-generic` |
| 20 | FIPS utilization requirement | Same as x86 — preferred hardware path for OpenSSL FIPS provider on ARM |

## ARMv8 SHA Extensions — Important

| Feature | Flag | Detection Register | Instructions | Kernel Module |
|---------|------|-------------------|--------------|---------------|
| SHA-1 + SHA-256 | `sha1`, `sha2` | ID_AA64ISAR0_EL1 SHA1/SHA2 fields | SHA1C, SHA1H, SHA1M, SHA1P, SHA1SU0, SHA1SU1, SHA256H, SHA256H2, SHA256SU0, SHA256SU1 | `sha1-ce`, `sha256-ce` |
| SHA-512 | `sha512` | ID_AA64ISAR0_EL1 SHA3 field (ARMv8.2+) | SHA512H, SHA512H2, SHA512SU0, SHA512SU1 | `sha512-ce` |
| SHA-3 | `sha3` | ID_AA64ISAR0_EL1 SHA3 field (ARMv8.2+) | EOR3, RAX1, XAR, BCAX | `sha3-ce` |

**Note:** ARM has dedicated SHA-512 instructions (ARMv8.2+), unlike x86 which has no dedicated SHA-512 hardware and relies on AVX2 software implementations.

### Detection (safe Rust)
```
std::is_aarch64_feature_detected!("sha2")  // SHA-1 + SHA-256
std::is_aarch64_feature_detected!("sha3")  // SHA-512 + SHA-3 (ARMv8.2+)
```

### `/proc/crypto` Driver Mapping (ARM)

| Driver | Hash | Hardware-backed |
|--------|------|-----------------|
| `sha1-ce` | SHA-1 | Yes |
| `sha256-ce` | SHA-256 | Yes |
| `sha512-ce` | SHA-512 | Yes (ARMv8.2+) |
| `sha3-ce` | SHA-3 | Yes (ARMv8.2+) |
| `sha256-generic` | SHA-256 | No |

## ARMv8.5 RNG Extension

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ARMv8.5-RNG |
| 4 | Linux `/proc/cpuinfo` flag | `rng` |
| 5 | Key instructions | RNDR, RNDRRS |
| 7 | Security relevance | Hardware random number generation equivalent to x86 RDRAND/RDSEED |
| 11 | Classification | **Critical/Operational (FIPS)** — same rationale as RDRAND/RDSEED |

**Note:** ARMv8.5-RNG is covered in detail in Phase 1B (Entropy & Big Integer).

## Cross-Platform Detection Summary

| Capability | x86 Flag | ARM Flag | x86 Kernel Module | ARM Kernel Module |
|-----------|----------|----------|-------------------|-------------------|
| AES acceleration | `aes` | `aes` | `aesni_intel` | `aes-ce` |
| GCM GHASH | `pclmulqdq` | `pmull` | `ghash-clmulni-intel` | `ghash-ce` |
| SHA-1 | `sha_ni` | `sha1` | `sha1-ni` | `sha1-ce` |
| SHA-256 | `sha_ni` | `sha2` | `sha256-ni` | `sha256-ce` |
| SHA-512 | (no dedicated) | `sha512` | `sha512-avx2` (SW) | `sha512-ce` |
| Hardware RNG | `rdrand`/`rdseed` | `rng` | N/A | N/A |

## Sources

- [ARM64 CPU Feature Registers — Linux Kernel](https://docs.kernel.org/arch/arm64/cpu-feature-registers.html)
- [ARM64 ELF hwcaps — kernel.org](https://dri.freedesktop.org/docs/drm/arm64/elf_hwcaps.html)
- [ID_AA64ISAR0_EL1 — ARM Developer](https://developer.arm.com/documentation/ddi0601/latest/AArch64-Registers/ID-AA64ISAR0-EL1)
- [cpufeatures crate — RustCrypto](https://docs.rs/cpufeatures/latest/cpufeatures/)
