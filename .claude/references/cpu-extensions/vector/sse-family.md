# SSE Family — CPU Security Corpus Reference

**Phase:** 1C — Vector Extensions (Crypto-Relevant)
**Date:** 2026-03-18
**Features covered:** SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2 (6 features, features #11–16)
**Classification of all entries:** Informational
**Schema version:** 23-column (cpu-matrix.md v3)

---

## Overview: Why the SSE Family Matters for Crypto

The SSE family is not itself a crypto feature set. These are general-purpose SIMD instruction
sets that crypto libraries exploit as building blocks. The security relevance is indirect:

1. **Constant-time software AES:** VPAES (vector-permutation AES) uses SSSE3 shuffle instructions
   to implement a bitsliced AES that is immune to cache-timing attacks. Without SSSE3, the safest
   software AES fallback is T-table AES, which is timing-vulnerable.

2. **SHA acceleration via SIMD:** The Linux kernel's `sha256_ssse3` module implements SHA-256 using
   SSSE3, AVX, and AVX2 instructions (one compiled module, runtime dispatch). This is the first
   acceleration layer below SHA-NI and above pure software SHA.

3. **CRC32C acceleration (SSE4.2):** The CRC32 instruction in SSE4.2 is used for storage integrity
   verification (Btrfs, iSCSI checksums, ZFS). Mapped to `crc32c-intel` kernel driver.

4. **Baseline guarantee:** SSE and SSE2 are mandatory on all x86_64 CPUs. Any x86_64 binary
   may use these without a CPUID check. This simplifies constant-time library design.

### The SSE → AVX → AVX-512 Crypto Throughput Progression

| Level | Register width | AES blocks/instruction | SHA-256 parallel | Constant-time AES path |
|-------|---------------|----------------------|------------------|------------------------|
| SSE2 baseline | 128-bit XMM | 1 (with AES-NI) | 1 | VPAES requires SSSE3 |
| SSSE3 | 128-bit XMM | 1 | 1 | VPAES available |
| AVX | 256-bit YMM (128 VEX-encoded) | 1 AES-NI (non-destructive) | 1 | VPAES on 256-bit |
| AVX2 | 256-bit YMM integer | 1 AES-NI | 4 (SHA-256 multi-buffer) | Full VPAES |
| AVX-512 + VAES | 512-bit ZMM | 4 (VAES) | 8 (SHA512-mb) | Same constant-time |

The progression matters for FIPS posture: if AES-NI is absent, the fallback path's security
depends on whether SSSE3 is available for VPAES. Without SSSE3, the only fallback is T-table
AES, which leaks key bits through cache timing.

---

## Feature #11: SSE (Streaming SIMD Extensions)

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SSE (Streaming SIMD Extensions) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | 128-bit floating-point SIMD; 8 × XMM registers (XMM0–XMM7); first vector ISA on x86 |
| 5 | Example instructions (security-relevant) | MOVAPS (aligned move), MOVUPS (unaligned move), XORPS (XOR for key whitening), ANDPS (masking), ORPS (merging) |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x01, subleaf 0, EDX bit 25 |
| 7 | `/proc/cpuinfo` flag | `sse` |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line; `PROC_SUPER_MAGIC` provenance via fstatfs |
| 9 | Minimum CPU generations | Intel Pentium III (1999), AMD Athlon XP (2001). Present on all x86_64 CPUs |
| 10 | Security benefit | Baseline XMM register file that AES-NI, SHA-NI, and VAES operate on. Required for any 128-bit vectorized crypto |
| 11 | Performance benefit | Enables SIMD-parallelized crypto; floating-point XMM operations used in some ECDH scalar multiply implementations |
| 12 | Assurance caveats | SSE introduces the XMM register file that is not zeroed on context switch by default (pre-XSAVE era). Modern kernels save/restore XMM via XSAVE/XSAVEOPT. Before Linux 2.6, XMM registers could leak across context switches on buggy kernels |
| 13 | Virtualization behavior | **KVM:** Passes through unconditionally (mandatory for x86_64 ABI). **VMware:** Same. **Hyper-V:** Same. Not maskable on x86_64 VMs — it is part of the ABI |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — SSE is a core ISA feature, always enabled |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled — non-maskable baseline; "unused" is not applicable |
| 17 | Software utilization detection | Implicit — any x86_64 kernel/userspace uses XMM registers. Not meaningful to check specifically |
| 18 | FIPS utilization requirement | N/A — SSE itself is not a crypto primitive. AES-NI (which uses XMM) has FIPS relevance |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Always accessible if advertised on x86_64. No BIOS gate, no hypervisor mask |
| 21 | Guest-vs-host discrepancy risk | False (no meaningful discrepancy possible on x86_64) |
| 22 | Notes | SSE introduced the XMM register file. SSE2 extended it to integers. The XMM file is shared across SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, and AES-NI/SHA-NI instructions |
| 23 | Sources | Intel SDM Vol 2A Ch 3; AMD APM Vol 3 App E; Linux x86_64 ABI documentation |

---

## Feature #12: SSE2

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SSE2 (Streaming SIMD Extensions 2) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | Extends XMM registers to integer operations (8-bit through 64-bit lanes); mandatory x86_64 ISA baseline |
| 5 | Example instructions (security-relevant) | PXOR (XOR for key schedule), PSRLDQ / PSLLDQ (byte shift — key expansion), PUNPCKLBW/PUNPCKHBW (byte interleave — bitsliced crypto), PSHUFB precursor operations via PSHUFD/PSHUFHW/PSHUFLW |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x01, subleaf 0, EDX bit 26 |
| 7 | `/proc/cpuinfo` flag | `sse2` |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line; always present on x86_64 |
| 9 | Minimum CPU generations | Intel Pentium 4 (2001), AMD Opteron / Athlon 64 (2003). **Mandatory on all x86_64 CPUs per ABI** |
| 10 | Security benefit | Enables integer vector operations used in constant-time bitsliced AES and SHA implementations. Without SSE2, bitsliced constant-time crypto is impractical on x86_64 |
| 11 | Performance benefit | Integer SIMD is the foundation of all x86_64 vectorized software; compilers auto-vectorize using SSE2 |
| 12 | Assurance caveats | SSE2 is the minimum x86_64 ABI baseline. Libraries compiled for x86_64 assume SSE2 unconditionally. No known timing or data-leakage vulnerabilities in the instructions themselves |
| 13 | Virtualization behavior | **KVM/VMware/Hyper-V:** Unconditionally passed through — required by x86_64 ABI. Cannot be masked |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }` |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled — baseline ABI requirement |
| 17 | Software utilization detection | N/A — implicitly used by all x86_64 software |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Always accessible |
| 21 | Guest-vs-host discrepancy risk | False |
| 22 | Notes | SSE2 integer operations (especially PXOR, PSRLDQ, byte shuffles) are the building blocks of bitsliced cryptography. The PSHUFB instruction (added in SSSE3) completes the shuffle capability needed for VPAES |
| 23 | Sources | Intel SDM Vol 2A Ch 3; AMD APM Vol 3 App E; System V AMD64 ABI specification |

### Security-Relevant Detail: SSE2 and Constant-Time AES

SSE2 is sufficient for a bitsliced AES implementation (e.g., the Käsper-Schwabe approach), but
not for VPAES (which requires SSSE3 PSHUFB). Bitsliced AES with SSE2 is constant-time and
cache-neutral but processes 8 AES blocks in parallel, making it unsuited for applications that
process one block at a time (e.g., counter mode with small buffers). On a FIPS system without
AES-NI and without SSSE3, the available constant-time paths are limited.

---

## Feature #13: SSE3

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SSE3 (Streaming SIMD Extensions 3) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | Adds horizontal arithmetic and complex-number operations to XMM registers |
| 5 | Example instructions (security-relevant) | LDDQU (unaligned load, cache-line crossing — efficient for unaligned crypto input buffers), MOVSHDUP / MOVSLDUP (complex arithmetic in some key schedule patterns) |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x01, subleaf 0, ECX bit 0 |
| 7 | `/proc/cpuinfo` flag | `pni` (Prescott New Instructions — Intel's original marketing name; the flag name is `pni` not `sse3`) |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line for `pni` |
| 9 | Minimum CPU generations | Intel Prescott (2004), AMD Athlon 64 (later steppings). Present on virtually all modern x86_64 CPUs |
| 10 | Security benefit | Minor: LDDQU improves efficiency on unaligned input in block cipher implementations. Horizontal adds used in some integrity check kernels |
| 11 | Performance benefit | Horizontal arithmetic used in FFT, DSP; limited direct crypto benefit beyond unaligned load improvement |
| 12 | Assurance caveats | Minimal security surface. No known vulnerabilities |
| 13 | Virtualization behavior | **KVM:** Passes through if host supports it. Named model `Nehalem` and later include SSE3. **VMware/Hyper-V:** Same — always present in modern VM CPU configurations |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }` |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled |
| 17 | Software utilization detection | N/A for security posture |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Always accessible if advertised |
| 21 | Guest-vs-host discrepancy risk | False — rarely masked, always present on modern hardware |
| 22 | Notes | The `/proc/cpuinfo` flag name `pni` (not `sse3`) is a common source of confusion. SSSE3 is a much more significant step than SSE3 for crypto |
| 23 | Sources | Intel SDM Vol 2A Ch 3; AMD APM Vol 3 App E |

---

## Feature #14: SSSE3 (Supplemental SSE3)

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SSSE3 (Supplemental Streaming SIMD Extensions 3) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | Adds byte shuffle and sign/absolute operations to XMM; enables VPAES (vector-permutation AES) and SSSE3-based SHA implementations |
| 5 | Example instructions (security-relevant) | PSHUFB (byte-granularity shuffle — core of VPAES/bitsliced AES), PALIGNR (byte-level concatenation rotate — SHA key schedule), PSIGNB / PABSB (absolute value variants), PMADDUBSW (dot product used in some hash primitives) |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x01, subleaf 0, ECX bit 9 |
| 7 | `/proc/cpuinfo` flag | `ssse3` |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line |
| 9 | Minimum CPU generations | Intel Core 2 (2007), AMD Bobcat/Bulldozer family (2011). Present on all modern x86_64 server CPUs |
| 10 | Security benefit | **Significant:** PSHUFB enables VPAES — a constant-time, cache-neutral AES implementation that does not require AES-NI. On systems without AES-NI, SSSE3+VPAES is the preferred constant-time fallback over T-table AES |
| 11 | Performance benefit | VPAES achieves ~300–400 MB/s on SSSE3 hardware, far below AES-NI (~1.4 GB/s) but acceptable and timing-safe. SHA-SSSE3 implementation provides ~1.5x over pure software SHA |
| 12 | Assurance caveats | VPAES (the constant-time AES path) depends on SSSE3. Without SSSE3, software AES on x86_64 defaults to T-table implementations, which are cache-timing vulnerable. The SHA Linux kernel module `sha256_ssse3` uses SSSE3 for the lowest-level vectorized SHA — the same module also detects AVX and AVX2 at runtime for higher performance |
| 13 | Virtualization behavior | **KVM:** Included in `Penryn` and all later named CPU models. **VMware/Hyper-V:** Present in all modern VM configurations. Not masked in practice |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }` |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled |
| 17 | Software utilization detection | `/proc/crypto`: presence of `sha256-ssse3` driver (from `sha256_ssse3` module) indicates SSSE3 SHA optimization is active |
| 18 | FIPS utilization requirement | N/A directly. Relevant indirectly: if AES-NI absent, SSSE3+VPAES is the recommended constant-time fallback. OpenSSL uses VPAES automatically on SSSE3 systems without AES-NI |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Always accessible if advertised |
| 21 | Guest-vs-host discrepancy risk | Low — masked only in very old or unusual VM configurations |
| 22 | Notes | SSSE3 is a larger security step than SSE3 for crypto purposes. The PSHUFB instruction alone enables an entire class of constant-time implementations. Check `/proc/crypto` for `sha256-ssse3` as a proxy for SSSE3 utilization |
| 23 | Sources | Intel SDM Vol 2A Ch 3; Käsper-Schwabe VPAES paper; Linux kernel `arch/x86/crypto/sha256_ssse3_glue.c` |

### VPAES and SSSE3 Security Significance

The VPAES (vector-permutation AES) technique, described by Käsper and Schwabe (2009), uses PSHUFB
to implement AES S-box lookups using byte shuffles rather than table lookups. This eliminates the
cache-set dependency that makes T-table AES timing-vulnerable. VPAES is the recommended constant-time
AES fallback for x86_64 CPUs with SSSE3 but without AES-NI.

OpenSSL automatically selects VPAES over T-table AES when SSSE3 is present and AES-NI is absent.
A FIPS auditor examining a system without AES-NI should verify that SSSE3 is present to confirm
that OpenSSL is using VPAES rather than the timing-vulnerable T-table path.

### `/proc/crypto` Driver Mapping for SSSE3

| Driver | Module | Algorithm | SSSE3-accelerated |
|--------|--------|-----------|-------------------|
| `sha256-ssse3` | `sha256_ssse3` | SHA-256 | Yes — lowest-level SSSE3 path; module also includes AVX/AVX2 variants with runtime dispatch |
| `sha1-ssse3` | `sha1_ssse3` | SHA-1 | Yes |
| `sha512-ssse3` | `sha512_ssse3` | SHA-512 | Yes |

Note: The `sha256_ssse3` kernel module is a single compiled object that probes CPUID at runtime
and dispatches to SSSE3, AVX, or AVX2 code paths. The `sha256-ssse3` driver name in `/proc/crypto`
indicates the registered name; the actual execution path may use AVX2 on capable hardware.

---

## Feature #15: SSE4.1

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SSE4.1 |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | Adds blend, min/max, dot product, and gather instructions to XMM; improves vectorization of conditional logic |
| 5 | Example instructions (security-relevant) | BLENDVPD / BLENDVPS (constant-time conditional select — branchless crypto), PBLENDVB (byte-granularity blend), PMOVZXBW through PMOVZXDQ (zero-extend — padding alignment in hash input preparation), PCMPEQQ (64-bit compare used in MAC verification) |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x01, subleaf 0, ECX bit 19 |
| 7 | `/proc/cpuinfo` flag | `sse4_1` |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line |
| 9 | Minimum CPU generations | Intel Penryn / Nehalem (2007–2008), AMD Bulldozer (2011). Present on all modern server CPUs |
| 10 | Security benefit | BLENDV instructions enable branchless conditional operations in constant-time code. Software ECC scalar multiplication implementations use blend for constant-time point selection |
| 11 | Performance benefit | Improved vectorization of conditional data selection; significant in multimedia but modest in most crypto paths |
| 12 | Assurance caveats | BLENDV instructions are data-independent only when the control operand is also data-independent. Improper use can reintroduce timing channels. The instruction itself is not inherently constant-time — the usage pattern must be |
| 13 | Virtualization behavior | **KVM:** Included in `Penryn` and later models. **VMware/Hyper-V:** Present in modern VM configurations |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }` |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled |
| 17 | Software utilization detection | N/A for security posture — indirect use in compiler-generated code |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Always accessible if advertised |
| 21 | Guest-vs-host discrepancy risk | Low |
| 22 | Notes | SSE4.1 is the prerequisite for SSE4.2. The two are often referred to together but were introduced at the same time (Penryn) on Intel; AMD added both simultaneously in Bulldozer |
| 23 | Sources | Intel SDM Vol 2A Ch 3; AMD APM Vol 3 App E |

---

## Feature #16: SSE4.2

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SSE4.2 |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | Adds hardware CRC32C and string comparison instructions; CRC32 is used in storage integrity paths that interact with security data flows |
| 5 | Example instructions (security-relevant) | CRC32 (hardware CRC32C — Castagnoli polynomial; used by Btrfs, iSCSI, ZFS, NVMe checksums), PCMPESTRI / PCMPESTRM (string search — pattern matching in intrusion detection), PCMPISTRI / PCMPISTRM (implicit-length string compare), POPCNT (population count — Hamming weight used in some crypto primitives) |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x01, subleaf 0, ECX bit 20 (SSE4.2); POPCNT has separate CPUID: Leaf 0x01, ECX bit 23 |
| 7 | `/proc/cpuinfo` flag | `sse4_2` (and `popcnt` separately) |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line |
| 9 | Minimum CPU generations | Intel Nehalem (2008), AMD Bulldozer (2011) |
| 10 | Security benefit | Hardware CRC32C accelerates storage integrity verification for filesystems and protocols that protect data blocks. In security-sensitive deployments, CRC32C is used as a framing integrity check (not a MAC, but detects accidental corruption) |
| 11 | Performance benefit | ~10–20x over software CRC32C; enables Gigabit-speed checksumming in iSCSI, NVMe, and Btrfs without CPU overhead |
| 12 | Assurance caveats | CRC32C is NOT a cryptographic hash and must not be used as a MAC or collision-resistant hash. It detects accidental bit errors only. The CRC32 instruction should never appear in a security-critical integrity path without an accompanying cryptographic MAC |
| 13 | Virtualization behavior | **KVM:** Included in `Nehalem` and later named models. **VMware/Hyper-V:** Present. **Note:** POPCNT has its own CPUID bit and can be queried separately |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }` |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled — CRC32C acceleration benefits filesystem performance; no security downside |
| 17 | Software utilization detection | `/proc/crypto`: `crc32c-intel` driver (from `crc32c_intel` module) indicates SSE4.2 CRC32C hardware is in use. Compare against `crc32c-generic` |
| 18 | FIPS utilization requirement | N/A — CRC32C is not a FIPS-relevant cryptographic primitive |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Always accessible if advertised |
| 21 | Guest-vs-host discrepancy risk | Low — KVM passes SSE4.2 in all current named models from Nehalem onward |
| 22 | Notes | The distinction between SSE4.1 and SSE4.2 is small; they were introduced together on Nehalem. CRC32C (SSE4.2) and PCLMULQDQ-based GHASH (a separate feature) are both used for integrity but are algorithmically different. Do not conflate them |
| 23 | Sources | Intel SDM Vol 2A Ch 3; Linux `crc32c_intel` driver source; Btrfs CRC documentation |

### `/proc/crypto` Driver Mapping for SSE4.2

| Driver | Module | Algorithm | Hardware-backed |
|--------|--------|-----------|-----------------|
| `crc32c-intel` | `crc32c_intel` | CRC32C | Yes — SSE4.2 CRC32 instruction |
| `crc32c-generic` | `crc32c_generic` | CRC32C | No — software |
| `crct10dif-pclmul` | `crct10dif_pclmul` | CRC-T10DIF | Yes — PCLMULQDQ (not SSE4.2) |

The `crc32c-intel` driver is a reliable proxy indicator that SSE4.2 is available and being used
for checksum acceleration. Its presence with `selftest: passed` confirms the SSE4.2 CRC32C path
is functional.

---

## CVE / Vulnerability Table (SSE Family)

| CVE | Year | Feature affected | Impact | Resolution |
|-----|------|-----------------|--------|------------|
| No dedicated CVEs against SSE instructions themselves | — | — | — | — |

The SSE family has no known CVEs against the instructions as hardware primitives. Security risks
arise from:

1. **Incorrect software use:** T-table AES on SSE2 systems (not a hardware vulnerability)
2. **Register state leakage:** Pre-XSAVE kernel XMM state management bugs (historical, resolved)
3. **BLENDV misuse:** Using SSE4.1 blend with secret-dependent control input (developer error)

The most significant SSE-related security consideration is the **absence** of SSSE3: without it,
constant-time software AES (VPAES) is unavailable, leaving T-table AES as the fallback.

---

## Posture Check Specifications

### AES Fallback Quality Check (when AES-NI absent)
1. Verify `ssse3` flag in `/proc/cpuinfo` (Layer 1)
2. If present: OpenSSL will use VPAES — timing-safe
3. If absent: OpenSSL will use T-table AES — **HIGH finding on FIPS/CUI systems**
4. Document: "System has no AES-NI and no SSSE3; software AES is timing-vulnerable"

### SHA Acceleration Tier Verification
1. Check `/proc/crypto` for `sha256-ni` (SHA-NI hardware)
2. If absent, check for `sha256-avx2` or `sha256-ssse3` (SIMD-accelerated software)
3. If only `sha256-generic` present — note as performance advisory (not a security finding)

### CRC32C Hardware Check
1. Check `/proc/crypto` for `crc32c-intel` driver
2. If `crc32c-generic` has higher priority — note as performance advisory only
3. CRC32C is NOT a security finding regardless of hardware/software status

---

## Sources

- [Intel SDM Vol 2A: CPUID — CPU Identification](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [AMD APM Vol 3 Appendix E: CPUID](https://www.amd.com/en/search/documentation/hub.html)
- [Linux Kernel Driver DataBase: CONFIG_CRYPTO_SHA256_SSSE3](https://cateee.net/lkddb/web-lkddb/CRYPTO_SHA256_SSSE3.html)
- [Linux Kernel Driver DataBase: CONFIG_CRYPTO_CRC32C_INTEL](https://cateee.net/lkddb/web-lkddb/CRYPTO_CRC32C_INTEL.html)
- [Käsper-Schwabe: Faster and timing-attack resistant AES-GCM (CHES 2009)](https://eprint.iacr.org/2009/129.pdf)
- [OpenSSL VPAES implementation](https://github.com/openssl/openssl/blob/master/crypto/aes/asm/vpaes-x86_64.pl)
- [Linux crypto architecture docs](https://docs.kernel.org/crypto/architecture.html)
