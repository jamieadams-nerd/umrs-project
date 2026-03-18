# PCLMULQDQ / CLMUL (Carryless Multiplication)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | PCLMULQDQ (Carry-Less Multiplication Quadword) / CLMUL instruction set |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | CPUID detection | EAX=01H, ECX bit 1 (PCLMULQDQ); VPCLMULQDQ: EAX=07H ECX=0, ECX bit 10 |
| 4 | Linux `/proc/cpuinfo` flag | `pclmulqdq` (also shown as `pclmuldq` on some kernels) |
| 5 | Key instructions | PCLMULQDQ (XMM), VPCLMULQDQ (YMM/ZMM — AVX-512 variant) |
| 6 | Introduced | Intel Westmere (2010, same generation as AES-NI); AMD Bulldozer (2011) |
| 7 | Security relevance | Accelerates GF(2^128) polynomial multiplication used in AES-GCM's GHASH authentication tag computation. Without PCLMULQDQ, AES-GCM authentication is software-computed and significantly slower. Also accelerates CRC32 for data integrity checks. |
| 8 | Performance benefit | AES-GCM: ~10.68 cycles/byte without → ~2.47 cycles/byte with AES-NI + PCLMULQDQ. The GHASH component is often the bottleneck in AES-GCM without PCLMULQDQ. |
| 9 | Known vulnerabilities | No known hardware CVEs. The instruction performs mathematical operations in GF(2^n) — no side-channel concerns in the instruction itself. |
| 10 | Compliance mapping | NIST SP 800-53 SC-13 (Cryptographic Protection — AES-GCM authentication); NIST SP 800-38D (GCM specification) |
| 11 | Classification | **Important** |
| 12 | Classification rationale | PCLMULQDQ is the performance enabler for AES-GCM, the dominant authenticated encryption mode in TLS and disk encryption. Without it, AES-GCM's GHASH computation becomes the bottleneck, potentially pushing systems toward less-preferred cipher suites. |
| 13 | Linux kernel support | Kernel modules: `ghash-clmulni-intel`, `crct10dif-pclmul`, `crc32-pclmul`. No separate config option — built into the GHASH and CRC modules. |
| 14 | Detection method (safe Rust) | `std::is_x86_feature_detected!("pclmulqdq")` (safe); or parse `/proc/cpuinfo` for `pclmulqdq` flag |
| 15 | Virtualization confidence | Same CPUID masking risk. `/proc/crypto` showing `ghash-clmulni-intel` is best guest-side indicator. |
| 16 | ARM/AArch64 equivalent | ARMv8 PMULL/PMULL2 instructions (`pmull` flag in `/proc/cpuinfo`). Used for GCM GHASH on ARM. |
| 17 | References | Intel CLMUL White Paper; NIST SP 800-38D (GCM); Intel SDM |
| 18 | Disposition when unused | **INVESTIGATE** — If PCLMULQDQ is present but `/proc/crypto` shows `ghash-generic` instead of `ghash-clmulni-intel`, the AES-GCM authentication path is software-only. Performance impact on TLS and disk encryption. |
| 19 | Software utilization detection | `/proc/crypto`: look for `ghash-clmulni-intel` (GHASH), `crct10dif-pclmul` (CRC), `crc32-pclmul` (CRC32). If absent, `ghash-generic` is being used. |
| 20 | FIPS utilization requirement | AES-GCM is a FIPS-approved mode. PCLMULQDQ acceleration is preferred but the software GHASH path is also validated. |
| 21 | Active mitigation status | N/A |
| 22 | Feature accessible vs advertised | Not BIOS-gated. Available if CPU supports it. |
| 23 | Guest-vs-host discrepancy risk | Medium — same concerns as AES-NI. AES-GCM performance degradation on migration to non-PCLMULQDQ host. |

## `/proc/crypto` Driver Mapping

| Driver | Module | Purpose | Hardware-backed |
|--------|--------|---------|-----------------|
| `ghash-clmulni-intel` | `ghash_clmulni_intel` | AES-GCM GHASH authentication | Yes — PCLMULQDQ |
| `crct10dif-pclmul` | `crct10dif_pclmul` | CRC-T10DIF (storage integrity) | Yes — PCLMULQDQ |
| `crc32-pclmul` | `crc32_pclmul` | CRC32 (data integrity) | Yes — PCLMULQDQ |
| `ghash-generic` | `ghash_generic` | AES-GCM GHASH | No — software |

## Relationship to AES-NI

PCLMULQDQ and AES-NI are **co-dependent for AES-GCM performance**:
- AES-NI handles the AES block cipher operations
- PCLMULQDQ handles the GHASH polynomial multiplication for authentication
- Both were introduced in the same CPU generation (Westmere 2010)
- A system with AES-NI but without PCLMULQDQ gets fast encryption but slow authentication

## Sources

- [Intel CLMUL White Paper](https://cdrdv2-public.intel.com/724272/carry-less-multiplication-instruction.pdf)
- [CLMUL instruction set — Wikipedia](https://en.wikipedia.org/wiki/CLMUL_instruction_set)
- [PCLMULQDQ reference — felixcloutier](https://www.felixcloutier.com/x86/pclmulqdq)
- [NIST SP 800-38D: Recommendation for Block Cipher Modes — GCM](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
