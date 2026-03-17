# ASM Usage Policy (Rust Developer Agent)

## Standing Rule

Inline assembly (`asm!`, `global_asm!`) is **not prohibited** but is
**strictly governed**. It is distinct from FFI and does not violate the
FFI prohibition. However, it must never be used for convenience,
familiarity, or speculative performance gains.

### The three-gate test — ALL three must pass before writing any ASM

**Gate 1 — No safe alternative exists**
Check in this order before considering ASM:
1. Does `core::arch` provide an intrinsic for this instruction?
   (e.g., `_mm_aesenc_si128`, `_rdtsc`, `__cpuid`)
2. Does a well-maintained crate expose it safely?
   (e.g., `aes`, `rand`, `raw-cpuid`)
3. Can LLVM emit the target instruction via normal Rust with
   the right hints (`#[target_feature]`, `#[repr]`, volatile)?

If yes to any of the above — use that instead. Do not write ASM.

**Gate 2 — Measurable, significant performance benefit**
"Significant" means demonstrable via benchmark, not intuition.
Acceptable justifications:
- Specific hardware instruction with no compiler-emittable equivalent
  (RDTSC, RDTSCP, RDSEED, CPUID leaves, MSR reads, precise barriers)
- SIMD path the auto-vectorizer provably does not produce
  (verified by examining compiler output with `cargo-asm` or godbolt)
- Cryptographic primitive requiring AES-NI, SHA-NI, or CLMUL
  where the compiler cannot guarantee the instruction is used

Not acceptable:
- "I think this will be faster"
- "I know assembly well"
- Replacing arithmetic the compiler already optimizes
- Loop bodies without profiler evidence of a bottleneck

**Gate 3 — Safety and correctness can be fully documented**
Every `unsafe { asm!(...) }` block requires:
- A `// SAFETY:` comment explaining why the invariants hold
- A NIST SP 800-53 or CMMC control annotation justifying the need
- A note on which CPU features are required and how they are verified
  before the code path is reached

If you cannot write these comments completely and correctly,
you do not understand the ASM well enough to include it.

---

### Permitted use cases in UMRS

| Use case | Instruction(s) | Justification |
|----------|---------------|---------------|
| Serialized cycle timestamps for audit records | RDTSCP | AU-8 — std::time insufficient resolution for kernel event ordering |
| Hardware entropy for key material | RDSEED | SC-13 — NIST SP 800-90B prefers RDSEED over RDRAND |
| CPU feature detection | CPUID | SA-8 — must verify hardware capabilities before enabling enforcement paths |
| Precise memory ordering in MLS enforcement | MFENCE / LFENCE | SC-28 — prevent speculative execution across classification boundaries |
| AES-NI for CUI encryption paths | Via core::arch intrinsics preferred | SC-28 — use intrinsics first, raw asm only if intrinsics unavailable |

---

### Prohibited ASM patterns

- ASM in safe code — always `unsafe`, always justified
- ASM replacing arithmetic, comparisons, or branching
- ASM without a `// SAFETY:` block
- ASM without a NIST/CMMC annotation
- `global_asm!` defining symbols called via FFI — this combines
  two restricted patterns and requires explicit senior review
- AT&T syntax — use Intel syntax (default) for readability

---

### Required template for every ASM block

```rust
// SAFETY: [CONTROL-ID] <one sentence: why this is safe and correct>
// Requires: <CPU feature, e.g., "SSE2, verified by cpuid_check() at startup">
// Alternative considered: <what was checked and why it was insufficient>
unsafe {
    asm!(
        // instruction(s)
        options(/* nomem | nostack | pure | readonly | preserves_flags */)
    );
}
```

---

### Before writing ASM — mandatory check

Run `cargo-asm` or inspect godbolt output to confirm the compiler is
not already emitting the target instruction. Many "obvious" ASM
optimizations are already performed by LLVM. If the compiler output
already contains the instruction you were going to write — do not add ASM.

```bash
cargo asm <crate>::<module>::<function>
```
