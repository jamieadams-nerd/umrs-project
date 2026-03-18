# Phase 1C Summary — Vector Extensions (Crypto-Relevant)

**Phase:** 1C — Vector Extensions (Crypto-Relevant)
**Date:** 2026-03-18
**Status:** COMPLETE
**Files produced:**
- `sse-family.md` — Features #11–16: SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2
- `avx-family.md` — Features #17–18: AVX, AVX2
- `avx512.md` — Feature #19: AVX-512 (foundation + crypto subsets)
- `phase-1c-summary.md` — This file

---

## The SSE → AVX → AVX-512 Crypto Progression

The vector extension family forms a layered stack where each level enables better or safer
crypto. The security engineer's concern is not performance benchmarks but two specific
questions:

1. **Is the constant-time fallback path available?** (no AES-NI → need SSSE3 for VPAES)
2. **Is the highest-throughput accelerated path active?** (is it showing up in `/proc/crypto`?)

### Progression Table

| Level | Key feature | Crypto significance | Classification |
|-------|------------|--------------------|-|
| SSE | XMM register file | Foundation for AES-NI, SHA-NI instructions | Informational |
| SSE2 | Integer XMM ops | Baseline bitsliced crypto; mandatory x86_64 ABI | Informational |
| SSE3 | Horizontal adds, LDDQU | Minimal crypto benefit; unaligned load efficiency | Informational |
| SSSE3 | PSHUFB | **Enables VPAES** — constant-time AES without AES-NI | Informational |
| SSE4.1 | BLENDV | Branchless conditional select for constant-time ECC | Informational |
| SSE4.2 | CRC32C, PCMPESTR* | Storage integrity acceleration; `crc32c-intel` driver | Informational |
| AVX | YMM + VEX encoding | Non-destructive AES-NI pipelining; VPAES at 256-bit | Informational |
| AVX2 | 256-bit integer | `sha256-avx2` 4-lane SHA; full VPAES; gather for ECC | Informational |
| AVX-512F | ZMM + mask regs | Foundation; enables VAES+ZMM and VPCLMULQDQ | Informational |
| VAES (YMM/ZMM) | Multi-block AES | 2× (YMM) or 4× (ZMM) AES blocks/instruction | Informational |
| VPCLMULQDQ | Multi-block GHASH | 4× GCM authentication throughput | Informational |

All vector features are classified **Informational**. The Critical/Operational and
Critical/Defensive classifications belong to the primitives they accelerate (AES-NI, SHA-NI,
PCLMULQDQ), not to the vector infrastructure.

---

## Security-Critical Decision Tree: AES Fallback Quality

When evaluating a system's AES acceleration stack, proceed through this decision tree:

```
AES-NI present? (check 'aes' flag in /proc/cpuinfo)
├── YES → Use AES-NI path. Check /proc/crypto for 'aesni_intel' driver.
│         Is VAES present? (check 'vaes' flag)
│         ├── YES + AVX-512 → VAES+ZMM: 4 blocks/instr; see avx512.md for caveats
│         ├── YES + AVX2    → VAES+YMM: 2 blocks/instr; no frequency penalty
│         └── NO            → Scalar AES-NI: 1 block/instr; fine for FIPS
└── NO → Fallback path. What is the constant-time option?
         SSSE3 present? (check 'ssse3' flag)
         ├── YES → VPAES available. Check /proc/crypto for 'aes-avx' or verify
         │         OpenSSL uses VPAES. Timing-safe. [Advisory note on FIPS system]
         └── NO  → Only bitsliced SSE2 AES or T-table AES available.
                   T-table AES = TIMING VULNERABLE. [HIGH finding on FIPS/CUI system]
```

On a FIPS system without AES-NI:
- SSSE3 present: acceptable (VPAES is timing-safe and OpenSSL selects it automatically)
- SSSE3 absent: HIGH finding — software AES is cache-timing vulnerable

---

## CPUID Detection Reference Table

### Flags and CPUID locations for all 9 vector features in Phase 1C

| Feature | CPUID Leaf | Register | Bit | `/proc/cpuinfo` flag |
|---------|-----------|----------|-----|---------------------|
| SSE | 01H | EDX | 25 | `sse` |
| SSE2 | 01H | EDX | 26 | `sse2` |
| SSE3 | 01H | ECX | 0 | `pni` (not `sse3`) |
| SSSE3 | 01H | ECX | 9 | `ssse3` |
| SSE4.1 | 01H | ECX | 19 | `sse4_1` |
| SSE4.2 | 01H | ECX | 20 | `sse4_2` |
| POPCNT (companion to SSE4.2) | 01H | ECX | 23 | `popcnt` |
| AVX | 01H | ECX | 28 | `avx` |
| AVX2 | 07H, subleaf 0 | EBX | 5 | `avx2` |
| AVX-512F | 07H, subleaf 0 | EBX | 16 | `avx512f` |
| AVX-512BW | 07H, subleaf 0 | EBX | 30 | `avx512bw` |
| AVX-512VL | 07H, subleaf 0 | EBX | 31 | `avx512vl` |
| AVX-512DQ | 07H, subleaf 0 | EBX | 17 | `avx512dq` |
| VAES | 07H, subleaf 0 | ECX | 9 | `vaes` |
| VPCLMULQDQ | 07H, subleaf 0 | ECX | 10 | `vpclmulqdq` |
| GFNI | 07H, subleaf 0 | ECX | 8 | `gfni` |

### Important flag name anomaly
The SSE3 flag in `/proc/cpuinfo` is `pni` (Prescott New Instructions), NOT `sse3`. This is
a common source of bugs in detection scripts. When searching `/proc/cpuinfo` for SSE3 support,
search for `pni`, not `sse3`.

### AVX OS enablement check
For AVX and above (YMM register usage), CPUID bit alone is insufficient. The OS must have
enabled XSAVE and set XCR0 bits for YMM (bit 2) and ZMM (bits 5–7) state. On RHEL 10 with
a standard kernel, this is done correctly at boot. In custom container environments or with
experimental kernels, verify that the OS has enabled XSAVE.

Safe detection pattern in Rust via `/proc/cpuinfo`:
```rust
// Read /proc/cpuinfo and search the flags line — no XSAVE check needed because
// Linux only sets the cpuinfo flags when XSAVE is correctly configured
let flags = read_cpuinfo_flags()?;
let has_avx = flags.contains("avx");
let has_avx2 = flags.contains("avx2");
let has_vaes = flags.contains("vaes");
```
Reading `/proc/cpuinfo` is the safe-Rust detection path. The kernel only populates these flags
when the feature is both hardware-supported and OS-enabled. The `avx` flag in `/proc/cpuinfo`
implies XSAVE is configured correctly.

---

## `/proc/crypto` Driver Pattern Table

This table maps CPU vector features to their expected Linux kernel crypto drivers.
Use this for Layer 2 software utilization verification.

| CPU features required | `/proc/crypto` driver name | Module | Algorithm | Notes |
|-----------------------|--------------------------|--------|-----------|-------|
| SSE4.2 | `crc32c-intel` | `crc32c_intel` | CRC32C | Storage integrity; NOT a crypto hash |
| SSSE3 | `sha256-ssse3` | `sha256_ssse3` | SHA-256 | Lowest SIMD SHA tier |
| SSSE3 | `sha1-ssse3` | `sha1_ssse3` | SHA-1 | Legacy |
| SSSE3 | `sha512-ssse3` | `sha512_ssse3` | SHA-512 | — |
| AVX | `sha256-avx` | `sha256_ssse3` | SHA-256 | Runtime dispatch from same module |
| AVX | `sha1-avx` | `sha1_ssse3` | SHA-1 | — |
| AVX2 | `sha256-avx2` | `sha256_ssse3` | SHA-256 | 4-lane multi-buffer; highest non-NI SHA tier |
| AVX2 | `sha512-avx2` | `sha512_ssse3` | SHA-512 | — |
| SSSE3 or AVX | `aes-avx` | (OpenSSL/kernel) | AES | VPAES constant-time path |
| AES-NI | `aesni_intel` | `aesni_intel` | AES family | Covers AES, AES-GCM, CTR, CBC, XTS |
| AES-NI + PCLMULQDQ | `__gcm-aes-aesni` | `aesni_intel` | AES-GCM | Scalar pipelined GCM |
| VAES + VPCLMULQDQ | (via `aesni_intel`) | `aesni_intel` | AES-GCM | AVX-512 path in same module |

### Priority semantics in `/proc/crypto`

Higher numeric `priority` wins. When multiple implementations are registered for the same
algorithm, the kernel selects the highest-priority one for new operations. Hardware-accelerated
drivers always have higher priority than generic software implementations.

Typical priority ordering for SHA-256:
```
sha256-ni     priority 300  (SHA-NI hardware)
sha256-avx2   priority 170  (AVX2 multi-buffer)
sha256-avx    priority 160  (AVX single-buffer)
sha256-ssse3  priority 150  (SSSE3 baseline)
sha256-generic priority 100 (pure software)
```

If `sha256-ni` is present with `priority: 300` and `selftest: passed`, SHA-NI hardware is
active and no further investigation is needed.

### The `sha256_ssse3` module: one module, multiple drivers

The kernel module `sha256_ssse3` registers multiple driver names at load time based on
CPUID probing:
1. If SSSE3 present → register `sha256-ssse3` (priority 150)
2. Additionally, if AVX present → register `sha256-avx` (priority 160)
3. Additionally, if AVX2 present → register `sha256-avx2` (priority 170)

All three can appear simultaneously in `/proc/crypto`. The highest-priority one is used.
If only `sha256-ssse3` appears despite AVX/AVX2 being in `/proc/cpuinfo`, the module may
have been loaded before the CPU flags were fully initialized (rare) or the kernel was built
without AVX/AVX2 SHA support compiled in.

---

## Hypervisor Masking Summary

| Feature | KVM default | VMware EVC | Hyper-V | Cloud risk |
|---------|------------|------------|---------|------------|
| SSE / SSE2 | Always exposed | Always exposed | Always exposed | None — mandatory ABI |
| SSSE3 | Exposed (Penryn+) | Exposed (Penryn+) | Exposed | Very low |
| SSE4.1 / SSE4.2 | Exposed (Penryn+) | Exposed (Penryn+) | Exposed | Very low |
| AVX | Exposed (Sandy Bridge+) | Exposed from Sandy Bridge EVC | Exposed | Low |
| AVX2 | Exposed (Haswell+) | Exposed from Haswell EVC | Exposed | Low-medium (older clouds) |
| AVX-512F | Exposed with `cpu host` or Skylake-Server model | Exposed from Skylake-SP EVC baseline | Exposed | **High** — many clouds mask |
| VAES (YMM) | Exposed if host has it | Depends on EVC baseline | Exposed | Medium — Ice Lake+ only |
| VAES (ZMM) | Exposed with `cpu host` | Often masked in mixed clusters | Exposed | High — Skylake clouds lack it |
| VPCLMULQDQ | Exposed with `cpu host` or Cascadelake+ model | Often masked | Exposed | High — same as VAES(ZMM) |

**Key takeaway:** For UMRS instances in cloud or VMware environments, the crypto acceleration
ceiling is likely AVX2 (for SHA) and AES-NI (for AES-GCM) rather than AVX-512 VAES. This is
expected and acceptable — the AES-NI + PCLMULQDQ path is FIPS-compliant and provides
sufficient throughput for all current UMRS workloads.

---

## ARM/AArch64 Equivalents Note

ARM NEON (Arm Advanced SIMD) is the ARM equivalent of SSE/AVX for general vector operations.
For crypto-specific purposes:

- **ARM NEON:** 128-bit SIMD registers (equivalent to SSE/SSE2 width). Present on all ARMv8
  and later including the RHEL 10 aarch64 deployment target.
- **ARM Crypto Extension (`aes`, `sha2`, `sha512` flags):** Separate from NEON; provides
  direct hardware crypto acceleration. See `crypto-accel/arm-crypto-equivalents.md`.
- **SVE / SVE2:** ARM Scalable Vector Extension — variable-width (128–2048 bits). Future
  RHEL aarch64 kernels may expose SVE on appropriate hardware. Not yet covered in this corpus.

NEON does not have the same frequency penalty concerns as AVX-512 on Intel. ARM's NEON and
SVE are designed with lower power profiles and the frequency-reduction behavior documented
for Intel Skylake AVX-512 does not apply to ARM implementations.

---

## Key Findings for UMRS

1. **SSSE3 is the minimum constant-time AES fallback gate.** If a FIPS system lacks AES-NI,
   the absence of SSSE3 is a HIGH finding. The presence of SSSE3 makes VPAES available and
   resolves the timing vulnerability concern.

2. **AVX2 is the practical crypto acceleration ceiling for cloud-deployed UMRS.** AVX2 provides
   `sha256-avx2` (4-lane SHA-256) and VPAES at full 256-bit width. This is sufficient for
   all UMRS audit and crypto workloads.

3. **AVX-512 VAES + VPCLMULQDQ is a bonus, not a baseline.** On bare metal Sapphire Rapids or
   AMD Zen 4+, it is available with no penalties. In cloud VMs, expect it to be masked.

4. **The Skylake-X AVX-512 frequency penalty is real but microarchitecture-specific.** RHEL 10
   on Ice Lake (Intel's current-generation Xeon Scalable) and AMD Zen 4 do not exhibit this
   behavior. Document the penalty only when CPU microarchitecture is confirmed Skylake-X or
   Cascade Lake with early steppings.

5. **`/proc/crypto` is the authoritative Layer 2 indicator** for kernel crypto path selection.
   User-space OpenSSL paths are not reflected in `/proc/crypto` — only kernel-internal crypto
   (dm-crypt, kernel TLS, IPsec, kernel HMAC) appears there. For user-space posture, rely on
   OpenSSL's `openssl speed` or `OPENSSL_ia32cap` introspection.

6. **The SSE3 flag anomaly:** The `/proc/cpuinfo` flag for SSE3 is `pni`, not `sse3`. Any
   detection code or posture check that looks for the string `sse3` in `/proc/cpuinfo` will
   fail silently.

---

## References

- `sse-family.md` — SSE through SSE4.2 (Features #11–16)
- `avx-family.md` — AVX and AVX2 (Features #17–18)
- `avx512.md` — AVX-512 foundation + crypto subsets (Feature #19)
- `../crypto-accel/vaes.md` — VAES classification and CPUID details
- `../crypto-accel/pclmulqdq.md` — PCLMULQDQ and VPCLMULQDQ
- `../cpu-matrix.md` — 23-column matrix schema and classification definitions
- [Travis Downs: Gathering Intel on Intel AVX-512 Transitions](https://travisdowns.github.io/blog/2020/01/17/avxfreq1.html)
- [Travis Downs: Ice Lake AVX-512 Downclocking](https://travisdowns.github.io/blog/2020/08/19/icl-avx512-freq.html)
- [Cloudflare: On the dangers of Intel's frequency scaling](https://blog.cloudflare.com/on-the-dangers-of-intels-frequency-scaling/)
- [Linux kernel x86 feature flags documentation](https://www.kernel.org/doc/html/v5.12/x86/cpuinfo.html)
- [Käsper-Schwabe: Faster and timing-attack resistant AES-GCM (CHES 2009)](https://eprint.iacr.org/2009/129.pdf)
