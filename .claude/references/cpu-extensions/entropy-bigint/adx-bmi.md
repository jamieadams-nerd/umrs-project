# ADX, BMI1, BMI2 (Big Integer & Bit Manipulation Extensions)

**Category:** Big Integer / Public Key Acceleration (Category 3)
**Classification:** Informational
**Phase:** 1B
**Date:** 2026-03-18

---

## Overview

ADX, BMI1, and BMI2 are x86 instruction set extensions that accelerate big integer arithmetic and bit manipulation. They are consolidated here because they function as a cohesive acceleration suite for public-key cryptography: ADX provides multi-precision carry arithmetic; MULX (part of BMI2) provides carry-free 64-bit multiplication; BMI1 and BMI2 provide bit manipulation primitives useful in constant-time cryptographic code.

All three are **Informational** classification — their absence does not directly expose a vulnerability, but their presence enables significant performance improvements in RSA, ECC, and post-quantum cryptographic implementations.

---

## Feature 1: ADX (Multi-Precision Add-Carry Extension)

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ADX (Intel ADX, Multi-Precision Add-Carry Instruction Extensions) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Big Integer / Public Key Acceleration |
| 4 | Purpose | Enables parallel carry chains for multi-precision integer addition. Provides two variants of add-with-carry that each modify only one flag register (CF or OF), allowing two independent carry chains to be maintained simultaneously. Used with MULX (BMI2) to parallelise large integer multiplication. |
| 5 | Example instructions | `ADCX r64, r/m64` — add with carry from CF flag, writes result to CF only; `ADOX r64, r/m64` — add with overflow from OF flag, writes result to OF only; `MULX r64, r64, r/m64` — multiply 64-bit × 64-bit → 128-bit result, no flag modification (used with ADCX/ADOX) |
| 6 | CPUID detection | EAX=07H, ECX=0: EBX bit 19 (ADX) |
| 7 | Linux `/proc/cpuinfo` flag | `adx` |
| 8 | Linux authoritative path | `/proc/cpuinfo` flag `adx` |
| 9 | Minimum CPU generations | Intel: Broadwell (2014, 5th gen Core); AMD: Ryzen (Zen, 2017) |
| 10 | Security benefit | Enables higher-performance RSA and ECC operations without timing side-channel risk. ADX-based implementations can achieve constant-time multi-precision multiplication more efficiently than classic ADC loops, reducing the temptation to use non-constant-time shortcuts for performance. |
| 11 | Performance benefit | ~20–25% improvement in big-integer multiplication throughput over classic ADC loop. Enables parallelism in 256-bit, 384-bit, and 512-bit modular multiplication. OpenSSL's ECDSA and RSA operations on RHEL 10 use ADX when available. |
| 12 | Assurance caveats | (1) No CVEs against ADX itself — the instructions perform arithmetic correctly. (2) ADX does not inherently provide constant-time guarantees — the programmer must still avoid secret-dependent branches and memory accesses. ADX enables efficient constant-time implementations but does not enforce them. (3) MULX from BMI2 is commonly paired with ADCX/ADOX; a system supporting ADX without BMI2 (unusual after Broadwell) cannot use the full parallel multiply pattern. |
| 13 | Virtualization behavior | Passed through by KVM, VMware, and Hyper-V when the host supports ADX. No special behavior in virtual environments. |
| 14 | Firmware / BIOS / microcode dependency | Not firmware-gated. Available if CPU supports it. |
| 15 | Audit-card relevance | **Informational** |
| 16 | Recommended disposition when unused | Monitor only. If ADX is present but OpenSSL is not using it (check `/proc/crypto` for preferred ECC/RSA paths — though these are not exposed there; check with `openssl speed ecdsa` to verify expected throughput), investigate whether the FIPS provider is correctly detecting CPU capabilities. |
| 17 | Software utilization detection | No `/proc/crypto` entry — ADX is used within library code (OpenSSL, BoringSSL, libgcrypt) for RSA/ECC implementations, not exposed as a kernel crypto driver. Indirect check: `openssl speed rsa2048` throughput — a significant drop vs expected may indicate ADX is not being used. OpenSSL uses `CPUID` to detect ADX at initialization. |
| 18 | FIPS utilization requirement | N/A — ADX accelerates the big-integer arithmetic underlying RSA/ECC but is not itself a validated cryptographic primitive. The FIPS module validation covers the algorithm (RSA, ECDSA), not the arithmetic acceleration layer. |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Generally not BIOS-gated. Available if CPU supports it. |
| 21 | Guest-vs-host discrepancy risk | LOW — ADX passthrough is standard and functionally transparent. No security implication of the feature being present in guest but absent on host (hypervisor would mask it). |
| 22 | Notes | ADX was specifically designed in response to DJB and Lange's request for instruction set support that would enable efficient constant-time big-integer arithmetic. The ADCX/ADOX split across CF and OF allows the processor to execute two parallel carry chains, enabling the Schoolbook multiplication algorithm to be fully pipelined. Intel cited RSA and ECDH as the primary motivating use cases. |
| 23 | Sources | Intel ADX introduction blog; Intel SDM Vol 2A (ADCX, ADOX instructions); Wikipedia: Intel ADX; DJB blog post on Intel instruction set extensions (2014-05-17); OpenSSL source: `crypto/bn/asm/x86_64-mont.pl` |

---

## Feature 2: BMI1 (Bit Manipulation Instruction Set 1)

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | BMI1 (Bit Manipulation Instruction Set 1) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Big Integer / Public Key Acceleration |
| 4 | Purpose | Provides bit manipulation instructions that improve efficiency of integer arithmetic kernels. Key instructions: ANDN (AND-NOT), BEXTR (bit field extract), BLSI (extract lowest set bit), BLSMSK (mask up to lowest set bit), BLSR (reset lowest set bit), TZCNT (count trailing zeros). |
| 5 | Example instructions | `ANDN r64, r64, r/m64` — AND-NOT; `TZCNT r64, r/m64` — count trailing zeros (defined for zero input, unlike BSF); `BLSR r64, r/m64` — reset lowest set bit; `BEXTR r64, r/m64, r64` — bit field extract |
| 6 | CPUID detection | EAX=07H, ECX=0: EBX bit 3 (BMI1). Note: LZCNT is separate — CPUID 0x80000001, ECX bit 5 |
| 7 | Linux `/proc/cpuinfo` flag | `bmi1` (LZCNT appears separately as `abm` or `lzcnt`) |
| 8 | Linux authoritative path | `/proc/cpuinfo` flag `bmi1` |
| 9 | Minimum CPU generations | Intel: Haswell (2013); AMD: Piledriver (2012); Jaguar (2013) |
| 10 | Security benefit | TZCNT and LZCNT provide defined behavior on zero input (unlike BSF/BSR which have undefined output for zero), improving correctness of constant-time code that avoids undefined behavior. BLSR and related instructions reduce loop iterations in bit-scan-heavy cryptographic code (e.g., prime sieve algorithms used in key generation). |
| 11 | Performance benefit | Moderate — primarily eliminates multi-instruction sequences for common bit manipulation patterns. Most useful in conjunction with BMI2 for crypto library inner loops. |
| 12 | Assurance caveats | (1) TZCNT and LZCNT have well-defined behavior for zero input, unlike BSF/BSR. On processors that do not support BMI1/LZCNT, the instruction encoding falls through to BSF/BSR — which have **undefined output for zero input**. Code that relies on the zero-defined behavior must verify BMI1 support before use. (2) BEXTR is not inherently constant-time — the extract width is a runtime parameter and some microarchitectures implement it in microcode with operand-dependent timing. Avoid using BEXTR in secret-dependent bit extraction paths without microarchitectural timing analysis. |
| 13 | Virtualization behavior | Passed through by all major hypervisors when host supports. No special behavior. |
| 14 | Firmware / BIOS / microcode dependency | Not firmware-gated. |
| 15 | Audit-card relevance | **Informational** |
| 16 | Recommended disposition when unused | Monitor only. No security finding if absent. |
| 17 | Software utilization detection | No `/proc/crypto` entry. Used within compiler-generated code and crypto libraries. Compiler flag `-mbmi` enables BMI1 code generation in GCC/Clang. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Not firmware-gated. |
| 21 | Guest-vs-host discrepancy risk | LOW |
| 22 | Notes | BMI1 is almost always present together with BMI2 on modern hardware — Intel and AMD introduced them simultaneously in the same CPU generations. Exceptions: AMD supports BMI1 without BMI2 on Piledriver/Steamroller; this combination is rare in server deployments. Rust `std::is_x86_feature_detected!("bmi1")` is available but the feature is primarily used by compiler auto-vectorization and library code, not directly in Rust user code. |
| 23 | Sources | Intel SDM Vol 2A (BMI1 instructions); Wikipedia: x86 Bit Manipulation Instruction Sets; Mozilla CPUID detection bug 1488726 |

---

## Feature 3: BMI2 (Bit Manipulation Instruction Set 2)

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | BMI2 (Bit Manipulation Instruction Set 2) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Big Integer / Public Key Acceleration |
| 4 | Purpose | Provides second-generation bit manipulation instructions. Key additions over BMI1: MULX (64-bit multiply without flag modification — essential for ADX parallel multiplication), PEXT (parallel bits extract), PDEP (parallel bits deposit), BZHI (zero high bits), RORX/SARX/SHRX/SHLX (shift without flag modification). |
| 5 | Example instructions | `MULX r64, r64, r/m64` — unsigned 64-bit multiply producing 128-bit result, does not modify flags (enables use alongside ADCX/ADOX); `PEXT r64, r64, r/m64` — extract bits at positions specified by mask to contiguous low-order bits; `PDEP r64, r64, r/m64` — scatter contiguous low-order bits to positions specified by mask; `BZHI r64, r/m64, r64` — zero high bits above a specified index |
| 6 | CPUID detection | EAX=07H, ECX=0: EBX bit 8 (BMI2) |
| 7 | Linux `/proc/cpuinfo` flag | `bmi2` |
| 8 | Linux authoritative path | `/proc/cpuinfo` flag `bmi2` |
| 9 | Minimum CPU generations | Intel: Haswell (2013); AMD: Excavator (2015) — AMD added BMI2 later than BMI1; Zen 1 (2017) has full support |
| 10 | Security benefit | MULX is the keystone of ADX's parallel multiplication pattern — without MULX, the ADCX/ADOX dual-carry-chain technique cannot be used to its full potential. PEXT/PDEP enable efficient, potentially constant-time implementations of certain GF(2^n) operations and permutation layers in symmetric cryptography. |
| 11 | Performance benefit | MULX: enables ADX parallel multiplication, ~20–25% improvement in RSA/ECC as noted above. PEXT/PDEP: theoretically enable fast bit permutation for DES-like operations and GCM's field arithmetic. **CRITICAL CAVEAT:** On pre-Zen 3 AMD CPUs, PDEP and PEXT are implemented in microcode with 18-cycle latency vs. 3 cycles on Zen 3 and Intel Haswell+. On affected AMD CPUs, the software fallback (loop-based) is often faster. |
| 12 | Assurance caveats | (1) **PDEP/PEXT AMD microcode latency:** On AMD Zen 1, Zen 2, PDEP and PEXT have 18-cycle latency (microcode implementation) vs. 3 cycles on Intel and AMD Zen 3+. Code that assumes constant fast PDEP/PEXT will be unexpectedly slow on affected AMD CPUs. More critically: if a constant-time algorithm is designed around fast PDEP/PEXT and falls back to a loop on slow CPUs, the loop may have variable timing, reintroducing the side channel the hardware path was meant to prevent. (2) PEXT/PDEP are not inherently constant-time with respect to the mask argument. On some microarchitectures, execution time may depend on the mask value (popcount-dependent paths). Careful analysis is required before using PEXT/PDEP in secret-dependent code. |
| 13 | Virtualization behavior | Passed through by all major hypervisors. KVM on Intel passes MULX/PEXT/PDEP; KVM on pre-Zen 3 AMD exposes BMI2 flags but underlying PDEP/PEXT have high microcode latency. |
| 14 | Firmware / BIOS / microcode dependency | Not firmware-gated. PDEP/PEXT performance varies by microcode version on AMD pre-Zen 3 (microcode version affects whether hardware or microcode path is used). |
| 15 | Audit-card relevance | **Informational** |
| 16 | Recommended disposition when unused | Monitor only. No security finding if absent. |
| 17 | Software utilization detection | No `/proc/crypto` entry. MULX usage is embedded in OpenSSL's big-integer assembly (`x86_64-mont.pl`). PEXT/PDEP usage is rare in mainstream crypto libraries due to the AMD latency issue. `is_x86_feature_detected!("bmi2")` in Rust; `-mbmi2` compiler flag for GCC/Clang. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Not firmware-gated. PDEP/PEXT are present and functional on pre-Zen 3 AMD; they are slow (18 cycles), not absent. The feature is accurately advertised; performance is the issue. |
| 21 | Guest-vs-host discrepancy risk | LOW for security. Medium for performance — a guest migrated from Intel to pre-Zen 3 AMD will see the same BMI2 flag but significantly reduced PDEP/PEXT throughput. |
| 22 | Notes | MULX was specifically designed to work with ADCX/ADOX: since MULX does not modify any flags, it can be interleaved freely with ADCX/ADOX carry chains without breaking the carry propagation. This trio (MULX + ADCX + ADOX) is the foundation of all modern high-performance public-key cryptography on x86. DJB's original feedback to Intel (published 2014-05-17) directly influenced the design of these instructions. |
| 23 | Sources | Intel SDM Vol 2A (BMI2 instructions); Wikipedia: x86 Bit Manipulation Instruction Sets; DJB blog 2014-05-17 (small suggestions for Intel instruction set); Bit manipulations using BMI2 (randombit.net); has_fast_pdep crate (seancroach); AMD APM Vol 3 App E |

---

## ADX + BMI2 Combined: The RSA/ECC Acceleration Pattern

The most important use of ADX and BMI2 together is multi-precision modular multiplication, which underlies RSA key operations, ECDH key agreement, and ECDSA signing/verification.

### Classic ADC loop (pre-ADX)

```
; Multiply 4-limb (256-bit) integers using ADC
; Each limb multiply: MUL + ADC
; Serial carry chain: each iteration depends on previous
; Cannot be pipelined — latency-bound
```

### ADX parallel pattern (MULX + ADCX + ADOX)

```
; MULX: multiply without touching flags
; ADCX: carry-add using CF only
; ADOX: carry-add using OF only
; Two independent carry chains run simultaneously
; Can be interleaved: processor executes both in parallel
; Result: ~2x improvement in 256-bit modular multiplication throughput
```

### UMRS Relevance

On RHEL 10 with OpenSSL FIPS provider:
- OpenSSL detects ADX at initialization via CPUID
- RSA and ECDSA operations use the ADX-optimized code path when available
- A system lacking ADX will use the classic ADC loop — functionally identical, but slower
- For UMRS, this is an Informational signal: if ADX is absent on a FIPS system, performance of key generation and signing operations will be measurably lower

---

## PEXT/PDEP and Constant-Time Code

PEXT and PDEP (BMI2) have potential cryptographic applications in GF(2^n) arithmetic and permutation-heavy algorithms (DES round function, McBits code-based cryptography). However, two concerns arise for UMRS deployments:

1. **AMD pre-Zen 3 latency:** The 18-cycle microcode latency means PEXT/PDEP are rarely beneficial on AMD hardware prior to Zen 3. Code paths designed around fast PEXT/PDEP may inadvertently use slower loop fallbacks on AMD that have variable timing.

2. **Mask-dependent timing:** On some microarchitectures, PEXT/PDEP execution time may correlate with the population count of the mask argument. If the mask derives from secret data, this is a timing side channel. Intel's guidance on timing-safe cryptographic implementations warns against this pattern.

For UMRS: do not use PEXT/PDEP in constant-time code paths without explicit microarchitectural timing analysis. The safe approach is to avoid these instructions in secret-dependent paths entirely.

---

## CVE / Vulnerability Table

No CVEs exist against ADX, BMI1, or BMI2 themselves. These are arithmetic/bit manipulation instructions with no known hardware security vulnerabilities.

The notable security-relevant behaviors are:

| Issue | Feature | Nature | Impact |
|-------|---------|--------|--------|
| BSF/BSR undefined behavior for zero input | BMI1 fallback | Undefined C/C++ behavior when BMI1 absent | Correctness defect, not CVE |
| PDEP/PEXT slow on pre-Zen 3 AMD | BMI2 | Performance degradation | No security CVE; timing concern in constant-time code |
| BEXTR operand-dependent timing (some μarchs) | BMI1 | Potential timing side channel on secret-dependent use | No CVE; code review required |

---

## Sources

- [Intel ADX — Wikipedia](https://en.wikipedia.org/wiki/Intel_ADX)
- [x86 Bit Manipulation Instruction Sets — Wikipedia](https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set)
- [DJB: Some small suggestions for the Intel instruction set (2014-05-17)](https://blog.cr.yp.to/20140517-insns.html)
- [Bit manipulations using BMI2 — randombit.net](https://randombit.net/bitbashing/posts/haswell_bit_permutations.html)
- [Intel SDM Vol 2A: ADCX, ADOX, MULX, PEXT, PDEP, BZHI](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [ADCX — felixcloutier.com](https://www.felixcloutier.com/x86/adcx)
- [ADOX — felixcloutier.com](https://www.felixcloutier.com/x86/adox)
- [has_fast_pdep crate — seancroach](https://github.com/seancroach/has_fast_pdep)
- [Intel: Guidelines for Mitigating Timing Side Channels Against Cryptographic Implementations](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/secure-coding/mitigate-timing-side-channel-crypto-implementation.html)
- [Crypto++: MULX, ADCX, ADOX issue](https://github.com/weidai11/cryptopp/issues/463)
