# core::arch Intrinsics Map — UMRS Use Cases

Before writing raw `asm!`, verify the intrinsic is not available here.
Intrinsics are always preferred over raw ASM.

All intrinsics require `#[target_feature(enable = "...")]` or runtime
detection via `is_x86_feature_detected!()` before use.

---

## Timing and Serialization

| Need | Intrinsic | Feature flag | Raw ASM fallback |
|------|-----------|-------------|-----------------|
| Non-serializing timestamp | `_rdtsc()` | none (always available on x86_64) | `rdtsc` |
| Serializing timestamp (preferred) | None — use raw `asm!` | — | `rdtscp` |
| Load fence | `_mm_lfence()` | `sse2` | `lfence` |
| Store fence | `_mm_sfence()` | `sse` | `sfence` |
| Full memory fence | `_mm_mfence()` | `sse2` | `mfence` |

Note: RDTSCP has no `core::arch` intrinsic. Raw `asm!` is required.
Use the template in `SKILL.md` Step 3.

---

## Hardware Entropy

| Need | Intrinsic | Feature flag | Notes |
|------|-----------|-------------|-------|
| Hardware RNG (DRBG output) | `_rdrand64_step()` | `rdrand` | NIST SP 800-90B — use only as fallback |
| Hardware entropy seed | `_rdseed64_step()` | `rdseed` | Preferred per NIST SP 800-90B |

Both return a success flag — always check it and retry with backoff
on failure. Never spin without a retry limit.

```rust
use std::arch::x86_64::_rdseed64_step;

// SAFETY: [SC-13] RDSEED for key material entropy.
// Feature verified by is_x86_feature_detected!("rdseed").
unsafe fn get_seed() -> Option<u64> {
    let mut val = 0u64;
    if _rdseed64_step(&mut val) == 1 { Some(val) } else { None }
}
```

---

## CPU Feature Detection

| Need | Intrinsic | Notes |
|------|-----------|-------|
| Basic CPUID | `__cpuid(leaf)` | Returns CpuidResult {eax, ebx, ecx, edx} |
| CPUID with subleaf | `__cpuid_count(leaf, subleaf)` | Required for leaves 4, 7, 11, 13 |

```rust
use std::arch::x86_64::__cpuid;

// SAFETY: [SA-8] CPUID always available on x86_64.
let result = unsafe { __cpuid(0x80000007) };
let invariant_tsc = (result.edx & (1 << 8)) != 0;
```

Important CPUID leaves for UMRS:
- `0x00000001` — feature flags (SSE2, AES, RDRAND)
- `0x00000007` subleaf 0 — extended features (RDSEED, SHA)
- `0x80000007` — invariant TSC (EDX bit 8)
- `0x80000008` — physical/linear address sizes

---

## Cryptographic Instructions

| Need | Intrinsic | Feature flag | Control |
|------|-----------|-------------|---------|
| AES encrypt round | `_mm_aesenc_si128()` | `aes` | SC-28 |
| AES encrypt last round | `_mm_aesenclast_si128()` | `aes` | SC-28 |
| AES decrypt round | `_mm_aesdec_si128()` | `aes` | SC-28 |
| AES key expansion assist | `_mm_aeskeygenassist_si128()` | `aes` | SC-28 |
| Carry-less multiply | `_mm_clmulepi64_si128()` | `pclmulqdq` | SC-28 |
| SHA-256 message schedule | `_mm_sha256msg1_epu32()` | `sha` | SC-28 |
| SHA-256 rounds | `_mm_sha256rnds2_epu32()` | `sha` | SC-28 |

For AES-NI, always use intrinsics — never raw `asm!` for crypto.
The intrinsics allow LLVM to verify operand types and scheduling.

---

## SIMD (auto-vectorization check first)

Before reaching for SIMD intrinsics, verify LLVM is not already
auto-vectorizing your loop. Inspect with:

```bash
cargo asm <function> --rust
# or
RUSTFLAGS="--emit=asm" cargo build --release
```

If the output already contains `vmovdqu`, `vpaddd`, `vpxor` etc.,
LLVM is vectorizing — do not add manual SIMD.

Only proceed with SIMD intrinsics if:
1. Profiler shows the loop is a bottleneck
2. Compiler output confirms no auto-vectorization
3. The specific SIMD instruction provides measurable speedup

---

## MSR Access (kernel context only)

There are no `core::arch` intrinsics for MSR reads/writes (`RDMSR`,
`WRMSR`). These require raw `asm!` and are only valid in kernel/ring-0
context. In userspace they will fault.

If UMRS kernel module work requires MSR access, consult The IRS
before implementation — MSR manipulation has direct security
implications for MLS enforcement.1G
