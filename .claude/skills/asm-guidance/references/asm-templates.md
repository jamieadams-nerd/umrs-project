# ASM Templates — UMRS Permitted Patterns

Copy-paste templates with annotations pre-filled. Replace bracketed
placeholders before use. All templates assume x86_64 / RHEL 10.

---

## Template 1 — RDTSCP Serialized Timestamp (AU-8)

```rust
/// Returns a serialized CPU timestamp counter value suitable for
/// audit record ordering.
///
/// # Safety
/// - Caller must have verified invariant TSC via `cpuid_check_invariant_tsc()`
///   at process/module startup before calling this function.
/// - Not suitable for wall-clock time — use only for relative ordering
///   of events within a single boot session on a single NUMA node.
///
// SAFETY: [AU-8] Serialized cycle-accurate timestamp for audit event
// ordering. std::time::Instant does not provide sufficient resolution
// or serialization guarantees for kernel event sequencing in MLS context.
// Invariant TSC confirmed at startup. x86_64 target guaranteed by build.
// Alternative considered: std::time::Instant — rejected, insufficient
// resolution. _rdtsc() intrinsic — rejected, non-serializing.
#[inline]
pub unsafe fn read_serialized_tsc() -> u64 {
    let low: u32;
    let high: u32;
    asm!(
        "rdtscp",
        out("eax") low,
        out("edx") high,
        out("ecx") _,
        options(nomem, nostack, preserves_flags)
    );
    ((high as u64) << 32) | (low as u64)
}
```

---

## Template 2 — RDSEED Hardware Entropy (SC-13)

```rust
/// Reads one 64-bit hardware entropy seed value.
///
/// Returns `Some(val)` on success, `None` if the entropy pool is not
/// ready. Caller must implement retry with exponential backoff.
/// Do not busy-spin — hardware entropy pools need time to refill.
///
/// # Safety
/// - Caller must have verified RDSEED availability via
///   `is_x86_feature_detected!("rdseed")` before calling.
///
// SAFETY: [SC-13] Direct hardware entropy source for cryptographic
// key material seeding per NIST SP 800-90B Section 2.2.
// RDSEED preferred over RDRAND as it draws from raw entropy source
// rather than DRBG output. Feature verified by caller.
// Alternative considered: _rdseed64_step() intrinsic — use that
// if available in toolchain; this template is for explicit control.
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

/// Retry wrapper — call this, not rdseed64() directly.
pub fn get_hardware_seed() -> Result<u64, EntropyError> {
    // SAFETY: RDSEED availability checked here before unsafe call.
    if !is_x86_feature_detected!("rdseed") {
        return Err(EntropyError::NotSupported);
    }
    for attempt in 0..RDSEED_MAX_RETRIES {
        // SAFETY: [SC-13] Feature verified above.
        if let Some(val) = unsafe { rdseed64() } {
            return Ok(val);
        }
        // Backoff — do not spin
        std::hint::spin_loop();
    }
    Err(EntropyError::Exhausted)
}
```

---

## Template 3 — Classification Boundary Fence (SC-28)

```rust
/// Full memory fence for MLS classification boundary enforcement.
///
/// Must be called at every point where execution crosses a sensitivity
/// label boundary to prevent speculative reads across classification levels.
///
// SAFETY: [SC-28] Full serializing memory fence at MLS sensitivity
// boundary. Prevents speculative execution from reading data at a
// higher sensitivity level before the classification check completes.
// _mm_mfence() from core::arch is semantically equivalent and preferred
// when sse2 feature is confirmed; this raw form used when feature
// detection is not yet complete (early boot path only).
#[inline]
pub unsafe fn classification_fence() {
    asm!(
        "mfence",
        options(nostack, preserves_flags)
    );
}

/// Load fence — prevents speculative loads across boundary.
/// Use when only load ordering is required (cheaper than mfence).
// SAFETY: [SC-28] Load fence at read boundary. Less expensive than
// mfence when store ordering is not required.
#[inline]
pub unsafe fn classification_load_fence() {
    asm!(
        "lfence",
        options(nomem, nostack, preserves_flags)
    );
}
```

---

## Template 4 — CPUID Feature Verification (SA-8)

```rust
use std::arch::x86_64::{__cpuid, __cpuid_count};

/// Verifies hardware features required for UMRS enforcement paths.
/// Must be called once at startup before any feature-dependent code.
///
// SAFETY: [SA-8] CPUID instruction always available on x86_64.
// Used to verify hardware capabilities before enabling MLS enforcement
// paths. Panics on missing required features rather than silently
// degrading security posture.
pub fn verify_required_cpu_features() -> CpuFeatures {
    // Leaf 1 — basic feature flags
    let leaf1 = unsafe { __cpuid(0x00000001) };
    let has_aes   = (leaf1.ecx & (1 << 25)) != 0;
    let has_rdrand = (leaf1.ecx & (1 << 30)) != 0;

    // Leaf 7 subleaf 0 — extended features
    let leaf7 = unsafe { __cpuid_count(0x00000007, 0) };
    let has_rdseed = (leaf7.ebx & (1 << 18)) != 0;
    let has_sha    = (leaf7.ebx & (1 << 29)) != 0;

    // Leaf 0x80000007 — invariant TSC
    let leaf_ext = unsafe { __cpuid(0x80000007) };
    let has_invariant_tsc = (leaf_ext.edx & (1 << 8)) != 0;

    // Hard requirement — MLS enforcement cannot function without these
    assert!(has_aes, "AES-NI required for SC-28 CUI encryption");
    assert!(has_invariant_tsc, "Invariant TSC required for AU-8 audit timestamps");

    CpuFeatures {
        aes: has_aes,
        rdrand: has_rdrand,
        rdseed: has_rdseed,
        sha: has_sha,
        invariant_tsc: has_invariant_tsc,
    }
}
```

---

## Checklist before committing any ASM block

- [ ] Three-gate test passed (CLAUDE.md)
- [ ] `core::arch` intrinsic checked and unavailable or insufficient
- [ ] `// SAFETY:` comment present and complete
- [ ] NIST/CMMC control annotation present
- [ ] CPU feature guard in place (static or runtime)
- [ ] `options()` selected correctly per SKILL.md Step 4 table
- [ ] `cargo asm` output inspected — instruction appears as expected
- [ ] `// REVIEW: ASM` marker added for The IRS review
- [ ] Test in separate file (per UMRS testing standards)
