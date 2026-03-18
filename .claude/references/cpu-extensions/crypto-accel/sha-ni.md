# SHA Extensions (SHA-NI)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SHA Extensions (SHA-NI) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | CPUID detection | EAX=07H ECX=0, EBX bit 29 |
| 4 | Linux `/proc/cpuinfo` flag | `sha_ni` |
| 5 | Key instructions | SHA1RNDS4, SHA1NEXTE, SHA1MSG1, SHA1MSG2, SHA256RNDS2, SHA256MSG1, SHA256MSG2 |
| 6 | Introduced | Intel Goldmont (2016), AMD Ryzen / Zen (2017) |
| 7 | Security relevance | Hardware SHA-1 and SHA-256 acceleration. Provides constant-time hash computation eliminating timing side channels in software implementations. Critical for HMAC, TLS, code signing, IMA. |
| 8 | Performance benefit | ~3–4x speedup over software SHA-256; ~2–3.2 GB/s vs 500–800 MB/s. minio/sha256-simd benchmarks show 3.28 GB/s with SHA-NI vs 0.91 GB/s AVX2. |
| 9 | Known vulnerabilities | Early Intel Goldmont (2016–2017) had implementation bugs in SHA-NI — firmware workaround and OpenSSL patch released. No CVEs against the instruction set itself. |
| 10 | Compliance mapping | NIST SP 800-53 SC-13, SI-7 (Software Integrity); FIPS 180-4 (SHA standard); CMMC SC.L2-3.13.11 |
| 11 | Classification | **Important** |
| 12 | Classification rationale | SHA-256 is ubiquitous in security operations (IMA, dm-verity, TLS, HMAC). Hardware acceleration is not strictly required for FIPS compliance but significantly improves performance of integrity-critical operations. |
| 13 | Linux kernel support | `CONFIG_CRYPTO_SHA256_NI` — kernel modules `sha256-ni`, `sha1-ni`. Supported since Linux 4.4+. |
| 14 | Detection method (safe Rust) | `std::is_x86_feature_detected!("sha")` (safe); or parse `/proc/cpuinfo` for `sha_ni` flag |
| 15 | Virtualization confidence | Same CPUID masking risk as other features. `/proc/crypto` showing `sha256-ni` driver is best guest-side indicator. |
| 16 | ARM/AArch64 equivalent | ARMv8 SHA extensions: `sha1`, `sha2` flags (SHA-1, SHA-256); `sha512` flag for SHA-512 (ARMv8.2). Kernel modules: `sha1-ce`, `sha256-ce`. |
| 17 | References | Intel SHA Extensions whitepaper; minio/sha256-simd benchmarks; Linux kernel crypto docs |
| 18 | Disposition when unused | **MONITOR** — If SHA-NI is present but `/proc/crypto` shows `sha256-generic` at higher priority, the system is missing hardware acceleration for integrity-critical operations. Not a vulnerability per se, but a performance and assurance concern. |
| 19 | Software utilization detection | `/proc/crypto`: look for drivers `sha256-ni`, `sha1-ni` with priority > `sha256-generic`, `sha1-generic`. Also check for AVX2 variants: `sha256-avx2`, `sha256-ssse3`. |
| 20 | FIPS utilization requirement | SHA-256 is required by FIPS; hardware acceleration is preferred but not mandated. The FIPS module validates both hardware and software paths. |
| 21 | Active mitigation status | N/A |
| 22 | Feature accessible vs advertised | Not typically BIOS-gated. Available if CPU supports it. |
| 23 | Guest-vs-host discrepancy risk | Medium — same CPUID masking concerns. Performance impact on integrity-heavy workloads (IMA, dm-verity). |

## Coverage Note

SHA-NI covers **SHA-1 and SHA-256 only**. SHA-512 requires either:
- AVX2-based software implementation (`sha512-avx2`)
- ARM SHA-512 extension (`sha512` flag, ARMv8.2+)
- No dedicated x86 instruction exists for SHA-512

## `/proc/crypto` Driver Mapping

| Driver | Module | Hash | Hardware-backed |
|--------|--------|------|-----------------|
| `sha256-ni` | `sha256_ni` | SHA-256 | Yes — SHA-NI |
| `sha1-ni` | `sha1_ni` | SHA-1 | Yes — SHA-NI |
| `sha256-avx2` | `sha256_ssse3` | SHA-256 | Partial — SIMD-accelerated software |
| `sha256-ssse3` | `sha256_ssse3` | SHA-256 | Partial — SIMD-accelerated software |
| `sha256-generic` | `sha256_generic` | SHA-256 | No — pure software |

## Sources

- [Intel SHA Extensions](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sha-extensions.html)
- [SHA instruction set — Wikipedia](https://en.wikipedia.org/wiki/Intel_SHA_extensions)
- [minio/sha256-simd benchmarks](https://github.com/minio/sha256-simd)
- [Linux kernel crypto subsystem docs](https://docs.kernel.org/crypto/)
