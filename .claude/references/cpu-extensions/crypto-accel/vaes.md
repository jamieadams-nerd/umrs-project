# VAES (Vector AES on AVX-512/AVX2)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | VAES (Vectorized AES) |
| 2 | Vendor | Intel (Ice Lake+), AMD (Zen 3+ — VAES without AVX-512) |
| 3 | CPUID detection | EAX=07H ECX=0, ECX bit 9 (VAES); requires AVX512F + AVX512VL for 512-bit; AVX2 sufficient for 256-bit on Zen 3 |
| 4 | Linux `/proc/cpuinfo` flag | `vaes` |
| 5 | Key instructions | VAESENC, VAESENCLAST, VAESDEC, VAESDECLAST (YMM/ZMM register variants of AES-NI) |
| 6 | Introduced | Intel Ice Lake (2019); AMD Zen 3 (2020, VAES+AVX2 without AVX-512) |
| 7 | Security relevance | Performance optimization for bulk AES operations. Processes 2 (YMM) or 4 (ZMM) AES blocks per instruction. Same constant-time guarantees as AES-NI. |
| 8 | Performance benefit | Up to 162% improvement on AES-GCM over scalar AES-NI; ~14 GB/s on Intel Emerald Rapids at 16 KB blocks |
| 9 | Known vulnerabilities | Inherits AES-NI security properties. AVX-512 variant subject to frequency throttling on some CPUs (performance, not security). |
| 10 | Compliance mapping | NIST SP 800-53 SC-13 (same AES primitive); no additional compliance requirement beyond AES-NI |
| 11 | Classification | **Informational** |
| 12 | Classification rationale | Performance optimization only. Not required for FIPS compliance. AES-NI is the baseline requirement; VAES is a throughput multiplier. |
| 13 | Linux kernel support | Included in `aesni_intel` kernel module. VAES+AVX-512 AES-GCM kernel patch merged 2024. |
| 14 | Detection method (safe Rust) | `std::is_x86_feature_detected!("vaes")` (safe); or parse `/proc/cpuinfo` for `vaes` flag |
| 15 | Virtualization confidence | Same risks as AES-NI plus AVX-512 masking — cloud providers frequently mask AVX-512 for power management. Guest may see `vaes` flag but not AVX-512, getting only 256-bit VAES. |
| 16 | ARM/AArch64 equivalent | No direct equivalent. ARM NEON AES instructions operate on 128-bit vectors only. SVE/SVE2 may offer wider crypto in future. |
| 17 | References | WikiChip VAES article; Phoronix AVX-512 AES-GCM benchmarks; Intel Architecture ISE Programming Reference (Dec 2024) |
| 18 | Disposition when unused | No action needed — Informational classification. Absence has no security impact. |
| 19 | Software utilization detection | `/proc/crypto`: `aesni_intel` module handles VAES automatically when available. No separate driver name. Check kernel log for VAES-optimized path selection. |
| 20 | FIPS utilization requirement | None beyond AES-NI baseline. VAES uses the same validated AES primitive at wider vector widths. |
| 21 | Active mitigation status | N/A |
| 22 | Feature accessible vs advertised | VAES requires OS XSAVE support for YMM/ZMM state. If OS hasn't enabled XSAVE (XCR0 bits), VAES instructions will fault even if CPUID advertises support. AVX-512 variant additionally requires AVX-512 OS support. |
| 23 | Guest-vs-host discrepancy risk | Medium — VM migration may silently drop from 512-bit to 256-bit VAES (or to scalar AES-NI). Performance regression, not security regression. |

## Sources

- [WikiChip: VAES](https://en.wikichip.org/wiki/x86/vaes)
- [Phoronix: Up to 162% faster AES-GCM with VAES/AVX-512](https://www.phoronix.com/news/AES-GCM-Faster-AVX-VAES)
- [Intel Architecture ISE Programming Reference (Dec 2024)](https://cdrdv2-public.intel.com/843860/architecture-instruction-set-extensions-programming-reference-dec-24.pdf)
- [golang/go issue #43925 — VAES/VPCLMULQDQ detection](https://github.com/golang/go/issues/43925)
