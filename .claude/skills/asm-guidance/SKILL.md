---
name: asm-guidance
description: >
  Provides detailed guidance on when and how to write inline assembly in
  Rust for the UMRS project. Use this skill whenever the rust-developer
  agent is considering using asm!, global_asm!, or core::arch intrinsics,
  or when reviewing existing ASM code. Trigger when the user or agent
  mentions "asm", "inline assembly", "intrinsics", "RDTSC", "CPUID",
  "RDSEED", "AES-NI", "memory barrier", "mfence", "lfence", or asks
  about low-level hardware instructions in Rust. Also trigger when the
  agent is about to write unsafe code involving direct hardware access.
  Never write ASM without consulting this skill first.
---

# ASM Guidance Skill — UMRS Rust Developer Agent

Detailed implementation guidance for the cases where ASM passes the
three-gate test defined in `CLAUDE.md`. If you have not already applied
the three-gate test, do that first before reading further.

---

## Step 1 — Check core::arch first

Before writing raw `asm!`, check whether `core::arch` already wraps
the instruction you need. These are stable, safe-to-call (within
`unsafe`), and compiler-verified:

```rust
use std::arch::x86_64::{
    __cpuid,           // CPUID
    _rdtsc,            // RDTSC (not serializing)
    _mm_aesenc_si128,  // AES-NI round
    _rdseed64_step,    // RDSEED
    _rdrand64_step,    // RDRAND (fallback only)
    _mm_lfence,        // LFENCE
    _mm_mfence,        // MFENCE
    _mm_sfence,        // SFENCE
};
```

If your instruction is in `core::arch` — use the intrinsic, not raw
`asm!`. The intrinsic is safer, more portable across compiler versions,
and allows LLVM to reason about it during optimization.

See `references/intrinsics-map.md` for the full mapping of UMRS use
cases to their `core::arch` equivalents.

---

## Step 2 — Verify CPU feature availability

Any code path using hardware-specific instructions must be guarded.
Never assume the feature is present.

**Static dispatch (preferred for hot paths):**
```rust
#[target_feature(enable = "aes")]
unsafe fn encrypt_block_aesni(block: __m128i, key: __m128i) -> __m128i {
    _mm_aesenc_si128(block, key)
}
```

**Runtime dispatch (for paths that must run on all hardware):**
```rust
if is_x86_feature_detected!("aes") {
    // SAFETY: [SC-28] AES-NI availability confirmed by
    // is_x86_feature_detected at runtime before this branch.
    unsafe { encrypt_block_aesni(block, key) }
} else {
    encrypt_block_software(block, key)
}
```

**For CPUID verification at startup (SC-13, SA-8):**
```rust
// SAFETY: [SA-8] CPUID is always available on x86_64.
// Used to verify hardware features before enabling enforcement paths.
// No alternative exists for direct feature enumeration.
let result = unsafe { __cpuid(0x80000007) };
let tsc_invariant = (result.edx & (1 << 8)) != 0;
```

---

## Step 3 — When raw asm! is genuinely required

Use this only after confirming `core::arch` does not cover your case.

### RDTSCP (serializing timestamp — preferred over RDTSC for AU-8)

```rust
/// Returns a serialized CPU timestamp counter value.
/// RDTSCP is preferred over RDTSC because it serializes instruction
/// retirement before reading, preventing reordering across the barrier.
///
/// # Safety
/// Caller must verify TSC invariance via CPUID leaf 0x80000007 EDX[8]
/// before relying on cross-core comparability of values.
// SAFETY: [AU-8] Serialized cycle-accurate timestamp for audit record
// ordering. std::time::Instant lacks sufficient resolution and
// monotonicity guarantees for kernel event sequencing.
// TSC invariance verified at startup via cpuid_check().
// Requires: x86_64, invariant TSC (verified).
// Alternative considered: std::time::Instant — insufficient resolution.
#[inline]
pub unsafe fn rdtscp() -> u64 {
    let low: u32;
    let high: u32;
    asm!(
        "rdtscp",
        out("eax") low,
        out("edx") high,
        out("ecx") _,      // TSC_AUX (processor ID) — discarded
        options(nomem, nostack, preserves_flags)
    );
    ((high as u64) << 32) | (low as u64)
}
```

### RDSEED (hardware entropy — SC-13)

```rust
/// Reads one 64-bit hardware random seed value via RDSEED.
/// Returns None if the hardware entropy pool is not ready.
/// Caller must retry with backoff on None — do not spin.
///
/// RDSEED is preferred over RDRAND per NIST SP 800-90B because it
/// draws from the raw entropy source rather than a DRBG output.
// SAFETY: [SC-13] Direct hardware entropy for key material seeding.
// NIST SP 800-90B Section 2.2 prefers non-DRBG entropy sources.
// Requires: RDSEED instruction (verified via is_x86_feature_detected).
// Alternative considered: _rdseed64_step intrinsic — used where
// available; this path is fallback for older toolchain versions.
#[inline]
pub unsafe fn rdseed64() -> Option<u64> {
    let mut val: u64 = 0;
    let mut success: u8;
    asm!(
        "rdseed {val}",
        "setc {ok}",
        val = out(reg) val,
        ok = out(reg_byte) success,
        options(nomem, nostack)
    );
    if success != 0 { Some(val) } else { None }
}
```

### Precise memory barriers (SC-28, MLS enforcement)

```rust
// SAFETY: [SC-28] Full memory fence required at MLS classification
// boundary to prevent speculative reads across sensitivity levels.
// _mm_mfence() from core::arch is preferred — use raw asm only if
// the intrinsic is unavailable in the current build configuration.
#[inline]
pub unsafe fn classification_barrier() {
    asm!(
        "mfence",
        options(nostack, preserves_flags)
    );
}
```

---

## Step 4 — options() selection guide

Always specify the most restrictive options that are correct.
Wrong options cause miscompilation — LLVM relies on these for
optimization decisions.

| Option | Meaning | Use when |
|--------|---------|----------|
| `nomem` | ASM does not read or write memory | Pure register operations (RDTSC, CPUID, arithmetic) |
| `nostack` | ASM does not use the stack | Almost always — unless you explicitly push/pop |
| `pure` | Same inputs always produce same outputs | Deterministic computations only — NOT for RDTSC, RDSEED |
| `readonly` | ASM reads memory but does not write | Load-only operations |
| `preserves_flags` | ASM does not modify EFLAGS | When you need flags to remain valid after the block |
| `att_syntax` | Use AT&T syntax instead of Intel | Never in UMRS — Intel syntax only per coding standards |

For RDTSC/RDTSCP: `nomem, nostack` — not `pure` (not deterministic)  
For RDSEED/RDRAND: `nomem, nostack` — not `pure`, not `readonly`  
For MFENCE: `nostack, preserves_flags` — not `nomem` (it is a memory operation)  
For CPUID: `nomem, nostack` — not `pure` (depends on implicit state)

---

## Step 5 — Verification before committing

1. **Inspect compiler output** — confirm the instruction appears and
   LLVM has not optimized it away or reordered it unexpectedly:
   ```bash
   cargo asm umrs_platform::hw::rdtscp --rust
   ```

2. **Run under ASAN/MSAN in CI** — ASM blocks bypass Rust's safety
   checks; sanitizers catch memory errors the compiler cannot:
   ```bash
   RUSTFLAGS="-Z sanitizer=address" cargo test --target x86_64-unknown-linux-gnu
   ```

3. **Verify on target hardware** — RHEL 10 on your Parallels VM may
   not expose all CPU features the production target will have.
   Check feature flags explicitly, do not assume.

4. **The IRS review** — any new `asm!` block must be flagged for
   security-auditor review before merge. Add a `// REVIEW: ASM`
   comment to make it grep-able:
   ```bash
   rg "REVIEW: ASM" --type rust
   ```

---

## Reference files

- `references/intrinsics-map.md` — UMRS use cases mapped to
  core::arch intrinsics
- `references/asm-templates.md` — copy-paste templates for all
  permitted UMRS ASM patterns with annotations pre-filled
