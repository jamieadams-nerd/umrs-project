# AVX and AVX2 — CPU Security Corpus Reference

**Phase:** 1C — Vector Extensions (Crypto-Relevant)
**Date:** 2026-03-18
**Features covered:** AVX (Feature #17), AVX2 (Feature #18)
**Classification of all entries:** Informational
**Schema version:** 23-column (cpu-matrix.md v3)

---

## Overview: AVX and AVX2 for Crypto

AVX and AVX2 are general-purpose SIMD extensions. Their crypto relevance is indirect but
substantial: they widen the register file and extend integer vector operations, enabling
higher-throughput crypto paths that the Linux kernel and userspace libraries expose via
well-defined driver names in `/proc/crypto`.

### Why AVX matters for security engineers

1. **Non-destructive 3-operand encoding (VEX prefix):** AVX introduces VEX-encoded forms of all
   XMM instructions. Critically, AES-NI instructions can be encoded in VEX form — this means
   pipelined AES that does not stall waiting for destination register write-back. On Sandy Bridge
   and later, `VAESENC` (VEX-encoded AES round) has no instruction latency dependency on the
   destination register, enabling pipelined AES for CBC/CTR mode without scheduling overhead.

2. **256-bit YMM register file:** The YMM register extension doubles the register width. For
   crypto operations, this enables SHA-256 multi-buffer processing: two 128-bit SHA-256 state
   vectors fit in one YMM register, allowing two independent hash computations to be interleaved
   at the instruction level without branching overhead.

3. **VPAES at 256-bit:** The vector-permutation AES constant-time technique (see sse-family.md)
   can operate on 256-bit YMM registers. Two AES block operations per VPSHUFB call rather than
   one, doubling throughput at the same timing-safety guarantee.

### Why AVX2 matters for security engineers

AVX2 extends 256-bit integer operations to match what SSE2 did for XMM:

1. **SHA-256 multi-buffer (4 parallel lanes):** The Linux kernel's `sha256-avx2` driver and
   OpenSSL's multi-buffer SHA engine process 4 independent SHA-256 computations in parallel
   in a single core using 256-bit integer lanes. On HTTPS servers processing concurrent TLS
   handshakes, this provides near-4x SHA-256 throughput — relevant for certificate verification
   at scale.

2. **AES-256 key schedule parallelism:** AES-256 requires 14 rounds (vs 10 for AES-128). With
   AVX2 integer operations, the key schedule expansion can be parallelized across multiple keys.
   OpenSSL uses this for multi-key bulk encryption scenarios.

3. **VPSHUFB on 256-bit operands (VPAES at full width):** AVX2 extends `VPSHUFB` to operate on
   full 256-bit YMM registers (with the two 128-bit lanes treated independently). This doubles
   VPAES throughput on AVX2 vs AVX hardware.

4. **Gather instructions (VPGATHERDD etc.):** Allow loading non-contiguous memory elements
   in a single instruction. Used in some P-256/P-384 ECC scalar multiplication implementations
   for constant-time table lookups without sequential access pattern leakage.

---

## Feature #17: AVX (Advanced Vector Extensions)

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | AVX (Advanced Vector Extensions) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | Extends XMM to 256-bit YMM registers; introduces 3-operand VEX encoding; enables non-destructive AES pipelining and 256-bit SIMD |
| 5 | Example instructions (security-relevant) | VXORPD (256-bit XOR — key whitening), VAESENC / VAESENCLAST (VEX-encoded AES round — 3-operand, pipelined), VAESDEC / VAESDECLAST (VEX-encoded AES decrypt), VPSHUFB (256-bit byte shuffle — VPAES foundation), VMOVDQU (unaligned 256-bit move for crypto input buffers) |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x01, subleaf 0, ECX bit 28 (AVX). Also requires OSXSAVE (ECX bit 27) and XCR0 bit 2 (YMM state) to be enabled by OS |
| 7 | `/proc/cpuinfo` flag | `avx` |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line; OSXSAVE + XCR0 check required before using YMM in userspace (kernel handles this for privileged code) |
| 9 | Minimum CPU generations | Intel Sandy Bridge (2011), AMD Bulldozer (2011). Present on all x86_64 server CPUs from 2012 onward |
| 10 | Security benefit | VEX-encoded AES-NI (VAESENC) enables 3-operand pipelined AES without register stalls. Critical for high-throughput AES-GCM on FIPS systems — the same AES-NI hardware with VEX encoding provides ~50% higher throughput at identical timing safety |
| 11 | Performance benefit | 256-bit SIMD halves loop count for bulk data operations; VEX encoding removes read-modify-write hazards; VPSHUFB on 256-bit operands doubles VPAES throughput vs SSSE3 |
| 12 | Assurance caveats | AVX requires OS XSAVE support (XCR0 bit 2 set). If OS has not enabled YMM state save/restore (unlikely on modern Linux but possible in minimal/embedded kernels), executing YMM instructions causes #UD fault. CPUID may report AVX available while the OS has not enabled it — the CPUID bit alone is insufficient detection. Must also verify OSXSAVE (ECX bit 27) and XCR0 bits at runtime. Transitioning between VEX and non-VEX code (AVX-SSE transition penalty) on Sandy Bridge/Ivy Bridge adds latency — not a security issue but relevant for crypto library perf |
| 13 | Virtualization behavior | **KVM:** Passthrough if host CPU supports AVX; KVM named model `SandyBridge` and later include AVX. `cpu host` passes all host flags including AVX. **VMware EVC:** AVX is included in EVC modes from Sandy Bridge baseline onward; masked only when EVC baseline is Core 2 or earlier — rare in practice. **Hyper-V:** Passes through AVX; Windows 7+ guest supports AVX save/restore |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }`. BIOS XSAVE support must be present but this is universal on AVX-capable platforms. No microcode required for AVX itself |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled — Informational; no security downside; disabling is not practical via CPUID masking on bare metal |
| 17 | Software utilization detection | `/proc/crypto`: `sha256-avx` (from `sha256_ssse3` module with runtime AVX dispatch) indicates AVX SHA path active. Absence of AVX SHA driver while AVX is present means the kernel module was not loaded |
| 18 | FIPS utilization requirement | N/A directly. AVX enables higher-throughput AES-NI pipelining; FIPS posture benefits from AVX presence because OpenSSL selects faster and still constant-time paths |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Requires OSXSAVE + XCR0 YMM bit in addition to CPUID. A VM may show `avx` in `/proc/cpuinfo` but if the hypervisor has not correctly plumbed XSAVE passthrough, YMM instructions will fault. In practice this is correctly handled in all modern hypervisors |
| 21 | Guest-vs-host discrepancy risk | Low for AVX itself. Elevated for VAES (AVX-512 variant) — see avx512.md |
| 22 | Notes | The VEX prefix is the architectural change that matters for crypto, not just the 256-bit width. VAESENC (VEX AES-NI) is more important than VXORPD for security purposes. The `sha256-avx` driver is registered by the same kernel module as `sha256-ssse3`; runtime CPU dispatch selects the fastest available path |
| 23 | Sources | Intel SDM Vol 2A Ch 3; AMD APM Vol 3 App E; Linux kernel `arch/x86/crypto/sha256_ssse3_glue.c`; Intel Software Developer Manual Vol 1 Ch 14 (AVX programming); OpenSSL assembly optimization sources |

### `/proc/crypto` Driver Mapping for AVX

| Driver | Module | Algorithm | AVX-accelerated path |
|--------|--------|-----------|---------------------|
| `sha256-avx` | `sha256_ssse3` | SHA-256 | Yes — AVX VPSHUFB + message schedule |
| `sha1-avx` | `sha1_ssse3` | SHA-1 | Yes |
| `sha512-avx` | `sha512_ssse3` | SHA-512 | Yes |

Note: `sha256-avx` and `sha256-ssse3` are registered by the same kernel module
(`sha256_ssse3`). The module probes at load time and registers the best available
driver name. On an AVX-capable system, both `sha256-ssse3` and `sha256-avx` entries
typically appear in `/proc/crypto` with different priorities; `sha256-avx` has the
higher priority and is selected for new crypto operations.

---

## Feature #18: AVX2 (Advanced Vector Extensions 2)

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | AVX2 (Advanced Vector Extensions 2) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Vector Acceleration (crypto-relevant) |
| 4 | Purpose | Extends all SSE2 integer operations to 256-bit YMM registers; enables 4-lane SHA-256 multi-buffer and full-width VPAES |
| 5 | Example instructions (security-relevant) | VPXOR (256-bit XOR — key expansion and state mixing), VPSHUFB on YMM (256-bit VPAES byte shuffle — constant-time AES at full width), VPGATHERDD (gather — constant-time table lookup for ECC scalar multiply), VPCMPEQB (256-bit byte compare — MAC verification), VPBROADCASTD (broadcast — key schedule distribution across lanes) |
| 6 | CPUID leaf/subleaf/bit | Leaf 0x07, subleaf 0, EBX bit 5 |
| 7 | `/proc/cpuinfo` flag | `avx2` |
| 8 | Authoritative detection path | `/proc/cpuinfo` flags line for `avx2`. Also requires `avx` (OSXSAVE + XCR0 YMM) as prerequisite |
| 9 | Minimum CPU generations | Intel Haswell (2013), AMD Excavator (2015) / Zen (2017). Present on virtually all x86_64 server CPUs from 2015 onward. Note: Intel Atom (Silvermont) and some low-power CPUs lack AVX2 |
| 10 | Security benefit | **Most significant vector feature for crypto after AES-NI and SHA-NI.** Enables: (1) `sha256-avx2` driver — 4 independent SHA-256 computations in parallel; (2) full-width VPAES for constant-time AES without AES-NI; (3) gather instructions for constant-time ECC table lookups. On FIPS systems, AVX2 presence significantly improves the quality of the software fallback crypto stack |
| 11 | Performance benefit | `sha256-avx2`: ~4x throughput over `sha256-generic` for multi-buffer workloads (TLS handshake certificate verification); AES-GCM with AVX2 VPAES: ~2x vs SSSE3 VPAES; gather-based ECC is constant-time without sequential access patterns |
| 12 | Assurance caveats | Gather instructions (`VPGATHERDD` etc.) have been the subject of side-channel research. On certain microarchitectures, gather operations may leak information through L1 cache access patterns if the indices are not aligned to cache-line boundaries (a developer error, not an inherent hardware flaw). Constant-time ECC implementations that use gather must ensure index values are cache-aligned. No known CVEs against AVX2 instructions themselves |
| 13 | Virtualization behavior | **KVM:** Included in `Haswell` and later named CPU models. `cpu host` passes through. **VMware EVC:** AVX2 included from Haswell EVC baseline onward; masked for clusters with Sandy Bridge nodes (a practical concern in heterogeneous clusters). **Hyper-V:** Passes through on Server 2012 R2+ guests. **Cloud VMs (AWS/GCP/Azure):** AVX2 is widely available in cloud VMs on modern instance types. Intel Cascade Lake and later server CPUs all include AVX2. Masking is rare except in older instance families |
| 14 | Firmware/BIOS/microcode dependency | `{ bios_enable_required: false, microcode_required: false }`. Inherits the OSXSAVE + XCR0 requirement from AVX |
| 15 | Audit-card relevance | Informational |
| 16 | Recommended disposition when unused | Leave enabled. Note: if `sha256-avx2` is not loaded in `/proc/crypto` despite `avx2` flag being present, check whether the `sha256_ssse3` kernel module is loaded (`lsmod | grep sha256_ssse3`) |
| 17 | Software utilization detection | `/proc/crypto`: `sha256-avx2` driver (from `sha256_ssse3` module, AVX2 dispatch path) — **this is the most important AVX2 crypto utilization indicator.** Priority is higher than `sha256-avx` and `sha256-ssse3`. Also: `aes-avx` for VPAES on AVX2-capable systems without AES-NI |
| 18 | FIPS utilization requirement | N/A — AVX2 is a performance optimizer for underlying FIPS-required primitives. However: on a FIPS system without AES-NI, verify that OpenSSL is using AVX2 VPAES (`aes-avx` in `/proc/crypto`) rather than T-table AES |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Requires AVX (OSXSAVE + XCR0) as prerequisite. Same accessibility caveats as AVX apply. On properly configured Linux kernel (RHEL 10 default), accessible if advertised |
| 21 | Guest-vs-host discrepancy risk | Low for modern clouds. Medium for heterogeneous on-premise VMware clusters with EVC enabled at Sandy Bridge baseline |
| 22 | Notes | AVX2 is the practical ceiling for most current Linux crypto acceleration. Above AVX2, the next step is AVX-512 + VAES, which introduces frequency scaling trade-offs (see avx512.md). For SHA specifically, the progression stops at `sha256-avx2` (4 lanes) without AVX-512; with AVX-512 + SHA extensions, the `sha256-mb` multi-buffer engine can process 8 lanes |
| 23 | Sources | Intel SDM Vol 2A Ch 3; Linux kernel `arch/x86/crypto/sha256_ssse3_glue.c`; minio/sha256-simd benchmark data; OpenSSL `aes-x86_64.pl` assembly source; Käsper-Schwabe VPAES paper |

### `/proc/crypto` Driver Mapping for AVX2

| Driver | Module | Algorithm | Notes |
|--------|--------|-----------|-------|
| `sha256-avx2` | `sha256_ssse3` | SHA-256 | Highest priority SHA-256 path below SHA-NI; 4-lane multi-buffer |
| `sha512-avx2` | `sha512_ssse3` | SHA-512 | Available on AVX2-capable systems |
| `sha1-avx2` | `sha1_ssse3` | SHA-1 (legacy) | Highest priority SHA-1 path below SHA-NI |
| `aes-avx` | (OpenSSL/kernel) | AES | VPAES on AVX hardware; constant-time software AES |

The `sha256-avx2` driver is the most important Layer 2 indicator for AVX2 crypto utilization.
Its presence confirms that: (a) AVX2 is present and accessible, (b) the `sha256_ssse3` kernel
module is loaded and running, and (c) SHA-256 computations are using 4-lane vectorization.

### AES-256 Throughput Context for FIPS Posture

AES-256 requires 14 rounds, making it ~40% slower than AES-128 (10 rounds) on the same hardware.
With AVX2 and AES-NI, the throughput progression is:

| Configuration | AES mode | Approximate throughput |
|---|---|---|
| No AES-NI, no SSSE3 | T-table AES (timing-unsafe) | ~200 MB/s |
| SSSE3 only | VPAES (timing-safe) | ~350 MB/s |
| AVX only | VPAES on YMM (timing-safe) | ~500 MB/s |
| AVX2 only | Full VPAES 256-bit (timing-safe) | ~700 MB/s |
| AES-NI + AVX | Pipelined AES-NI | ~1.4 GB/s |
| AES-NI + AVX2 | Multi-key AES-NI | ~2+ GB/s |
| VAES + AVX-512 | 4-block parallel VAES | ~5–8 GB/s |

These are approximate single-core values for AES-256-CTR. Exact values vary by CPU generation
and memory bandwidth. The table illustrates the security posture value of each feature level:
even without AES-NI, AVX2 + VPAES provides a timing-safe fallback at reasonable throughput.

---

## CVE / Vulnerability Table (AVX and AVX2)

| CVE | Year | Feature affected | Impact | Resolution |
|-----|------|-----------------|--------|------------|
| No dedicated CVEs against AVX/AVX2 instructions | — | — | — | — |

No CVEs target the AVX or AVX2 instruction sets as hardware primitives. Security risks are
confined to:

1. **Gather side-channel research (theoretical):** Research by Schwarz et al. (2019) identified
   that gather instructions on certain Intel microarchitectures can leak information through cache
   access timing if indices map to the same cache set. This affects ECC implementations that use
   `VPGATHERDD` for table lookups without cache-line alignment. Not a CVE; a developer concern.

2. **AVX-SSE transition penalty exploitation:** On Sandy Bridge, transitioning between legacy SSE
   and AVX code paths incurs a ~70-cycle penalty. An adversary with timing precision could
   potentially observe whether a victim transitions from AVX to SSE code. Not a practical attack
   vector but worth noting in high-assurance contexts.

3. **XSAVE state not restored:** Kernel bugs in XSAVE/XRESTORE could expose YMM register content
   across process boundaries. Modern Linux kernels (including RHEL 10's 6.12 series) handle this
   correctly. Not an AVX vulnerability per se — an OS obligation.

---

## Posture Check Specifications

### AVX Utilization Check
1. Verify `avx` flag in `/proc/cpuinfo` (Layer 1)
2. Check `/proc/crypto` for `sha256-avx` driver (Layer 2 indicator)
3. If `sha256-avx` absent while `avx` present: check if `sha256_ssse3` module is loaded
   — the module may simply not have been loaded yet (module is loaded on first crypto use
   or at boot if compiled as built-in)
4. Absence of AVX SHA drivers while the flag is present is a configuration advisory, not a
   security finding

### AVX2 VPAES Fallback Quality Check
1. On FIPS systems without AES-NI: verify `avx2` flag present in `/proc/cpuinfo`
2. Check `/proc/crypto` for `aes-avx` driver (VPAES on AVX2)
3. If absent: OpenSSL may still use VPAES internally (user-space, not reflected in `/proc/crypto`)
   — this check is advisory only for user-space paths
4. For kernel crypto (dm-crypt, kernel TLS): `/proc/crypto` is authoritative

### SHA-256 Acceleration Tier (complete ladder)
```
sha256-ni     (SHA-NI) — highest priority
sha256-avx2   (AVX2)   — 4-lane multi-buffer
sha256-avx    (AVX)    — 2-lane improvement over SSSE3
sha256-ssse3  (SSSE3)  — lowest hardware-accelerated tier
sha256-generic          — pure software fallback
```
Record the highest-priority driver with `selftest: passed` in the posture report.

---

## Sources

- [Intel SDM Vol 2A: CPUID — CPU Identification](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [AMD APM Vol 3 Appendix E: CPUID](https://www.amd.com/en/search/documentation/hub.html)
- [Linux kernel sha256_ssse3 glue code](https://cateee.net/lkddb/web-lkddb/CRYPTO_SHA256_SSSE3.html)
- [minio/sha256-simd: AVX512 SHA256 acceleration benchmarks](https://github.com/minio/sha256-simd)
- [OpenSSL aesni-sha256-x86_64.pl assembly source](https://github.com/openssl/openssl/blob/master/crypto/aes/asm/aesni-sha256-x86_64.pl)
- [Käsper-Schwabe: Faster and timing-attack resistant AES-GCM (CHES 2009)](https://eprint.iacr.org/2009/129.pdf)
- [Advanced Vector Extensions — Wikipedia](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions)
- [AVX-512 — Wikipedia](https://en.wikipedia.org/wiki/AVX-512)
