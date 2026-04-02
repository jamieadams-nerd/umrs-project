// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Hardware Timestamp — RDTSCP (x86_64) / CLOCK_MONOTONIC_RAW (other arches)
//!
//! Provides `read_hw_timestamp()` and `tsc_is_invariant()` as the only public
//! surface of this module. All unsafe code is in the `rdtscp_raw()` function
//! below, annotated with a `// REVIEW: ASM` marker for audit tooling.
//!
//! ## x86_64 Path — RDTSCP
//!
//! RDTSCP reads the Time Stamp Counter with instruction-retirement serialization.
//! It is preferred over RDTSC because it prevents reordering across the barrier:
//! all preceding instructions have retired before the counter is sampled.
//!
//! The raw TSC value is a cycle count, not nanoseconds. For phase duration
//! comparisons the unit is consistent within a single call to `detect()`, so
//! the exact unit does not matter — `end.saturating_sub(start)` yields a
//! cycle-domain duration. Callers store this as `duration_ns: u64` with the
//! understanding that on this architecture the value is in CPU cycles, which
//! approximates nanoseconds on a 1 GHz+ processor.
//!
//! TSC invariance is verified via CPUID leaf `0x80000007` EDX bit 8. A
//! non-invariant TSC produces values that are not monotonically comparable
//! across cores or after C-state transitions. `tsc_is_invariant()` exposes
//! this check so that `detect/mod.rs` can record a downgrade reason when
//! the TSC is not invariant.
//!
//! ## aarch64 / fallback path — CLOCK_MONOTONIC_RAW
//!
//! On non-x86_64 targets, `rustix::time::clock_gettime(CLOCK_MONOTONIC_RAW)`
//! provides a nanosecond-precision monotonic value without requiring unsafe.
//! The unit is true nanoseconds. `tsc_is_invariant()` always returns `true`
//! on this path — the system counter on AArch64 is always invariant.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-8**: Time Stamps — cycle-accurate or nanosecond-
//!   precise ordering for per-phase detection durations in audit records.
//! - **NIST SP 800-218 SSDF PW.4**: Unsafe isolation — unsafe is confined to
//!   `rdtscp_raw()` in this single module.

// ===========================================================================
// x86_64 implementation
// ===========================================================================

#[cfg(target_arch = "x86_64")]
mod x86_64_impl {
    use std::arch::x86_64::__cpuid;

    /// Read the TSC via RDTSCP and return the raw cycle count.
    ///
    /// RDTSCP serializes instruction retirement before sampling the TSC,
    /// preventing the counter read from being reordered ahead of earlier
    /// instructions by the CPU's out-of-order execution engine.
    ///
    /// The `ecx` output (TSC_AUX — processor ID) is discarded. We are
    /// interested only in the counter value for phase duration measurement.
    ///
    /// # Safety
    ///
    /// RDTSCP is available on all x86_64 processors that implement the
    /// `rdtscp` CPUID feature bit (CPUID.80000001H:EDX[27]). On RHEL 10 and
    /// any modern x86_64 hardware, this is universally present. The instruction
    /// reads only CPU registers — it does not access memory and cannot fault.
    ///
    /// // REVIEW: ASM — this block must be reviewed by the security-auditor
    /// // before any modification. See .claude/rules for the grep marker convention.
    ///
    /// NIST SP 800-53 AU-8 — serialized cycle-accurate timestamp for audit
    /// record phase ordering. std::time::Instant uses CLOCK_MONOTONIC which is
    /// subject to NTP slew; RDTSCP bypasses the kernel clocksource layer entirely.
    #[inline]
    pub(super) unsafe fn rdtscp_raw() -> u64 {
        // REVIEW: ASM
        let low: u32;
        let high: u32;
        // SAFETY: [AU-8] Serialized cycle-accurate timestamp for phase duration
        // measurement. RDTSCP is available on all x86_64 targets supported by
        // RHEL 10. The instruction reads CPU registers only (nomem, nostack).
        // TSC_AUX output (ecx) is discarded — we need only the counter value.
        // No core::arch intrinsic for RDTSCP exists (see intrinsics-map.md).
        // options: nomem (pure register op), nostack (no stack use),
        //          preserves_flags (does not modify EFLAGS).
        core::arch::asm!(
            "rdtscp",
            out("eax") low,
            out("edx") high,
            out("ecx") _,
            options(nomem, nostack, preserves_flags)
        );
        ((high as u64) << 32) | (low as u64)
    }

    /// Verify TSC invariance via CPUID leaf `0x80000007` EDX bit 8.
    ///
    /// An invariant TSC advances at a constant rate regardless of CPU frequency
    /// scaling or C-state transitions, and is comparable across cores on the
    /// same physical processor. A non-invariant TSC produces unreliable phase
    /// duration measurements.
    ///
    /// Returns `true` if the invariant TSC feature bit is set.
    ///
    /// # Safety
    ///
    /// CPUID is always available on x86_64; it is a non-faulting instruction.
    ///
    /// NIST SP 800-53 AU-8 — TSC invariance is a prerequisite for reliable
    /// monotonic timestamp ordering across pipeline phases.
    pub fn tsc_is_invariant() -> bool {
        // SAFETY: [SA-8] CPUID is always available on x86_64.
        // Leaf 0x80000007: Advanced Power Management Information.
        // EDX bit 8: Invariant TSC — TSC runs at a constant rate in all
        // ACPI P-, C-, and T-states.
        let result = unsafe { __cpuid(0x8000_0007) };
        (result.edx & (1 << 8)) != 0
    }

    /// Read a high-resolution hardware timestamp.
    ///
    /// Returns a `u64` cycle count from RDTSCP. The value is monotonically
    /// non-decreasing within a single core but may not be comparable across
    /// cores when the TSC is not invariant. Use `tsc_is_invariant()` to check.
    ///
    /// NIST SP 800-53 AU-8 — high-resolution phase timing for audit records.
    #[must_use = "hardware timestamp must be used to compute phase duration; discarding it loses the measurement"]
    pub fn read_hw_timestamp() -> u64 {
        #[cfg(debug_assertions)]
        let t0 = std::time::Instant::now();

        // SAFETY: rdtscp_raw() reads CPU registers only; see its Safety comment.
        let ts = unsafe { rdtscp_raw() };

        #[cfg(debug_assertions)]
        {
            let elapsed = t0.elapsed();
            log::debug!(
                "read_hw_timestamp (RDTSCP) completed in {} µs — value: {ts}",
                elapsed.as_micros()
            );
        }

        ts
    }
}

// ===========================================================================
// Non-x86_64 fallback (aarch64 and others)
// ===========================================================================

#[cfg(not(target_arch = "x86_64"))]
mod fallback_impl {
    use rustix::time::{ClockId, clock_gettime};

    /// TSC invariance is always considered true on the fallback path.
    ///
    /// The system counter on AArch64 (`CNTVCT_EL0`) is specified by the
    /// Arm architecture to be invariant — it runs at a fixed frequency
    /// regardless of CPU frequency scaling. No runtime check is required.
    ///
    /// NIST SP 800-53 AU-8 — monotonic timestamp ordering.
    pub const fn tsc_is_invariant() -> bool {
        true
    }

    /// Read CLOCK_MONOTONIC_RAW and return nanoseconds since boot.
    ///
    /// CLOCK_MONOTONIC_RAW advances at the hardware counter rate with no
    /// NTP frequency adjustment — equivalent to the RDTSCP path in terms of
    /// NTP independence. The value is nanoseconds, not CPU cycles.
    ///
    /// Returns `0` on arithmetic overflow (theoretical — u64 overflows after
    /// ~584 years of uptime) or on a clock returning negative values.
    /// Saturating at 0 is the correct fail-safe: a zero duration is visible
    /// and auditable; a wrapped value would be misleading.
    ///
    /// NIST SP 800-53 AU-8 — nanosecond-precision phase timing for audit records.
    #[must_use = "hardware timestamp must be used to compute phase duration; discarding it loses the measurement"]
    pub fn read_hw_timestamp() -> u64 {
        #[cfg(debug_assertions)]
        let t0 = std::time::Instant::now();

        let ts = clock_gettime(ClockId::MonotonicRaw);

        // Fail-closed on negative values; saturate on overflow.
        let nanos: u64 = u64::try_from(ts.tv_sec)
            .ok()
            .and_then(|s| s.checked_mul(1_000_000_000u64))
            .and_then(|s_ns| u64::try_from(ts.tv_nsec).ok().and_then(|ns| s_ns.checked_add(ns)))
            .unwrap_or(0);

        #[cfg(debug_assertions)]
        {
            let elapsed = t0.elapsed();
            log::debug!(
                "read_hw_timestamp (CLOCK_MONOTONIC_RAW fallback) completed in {} µs — value: {nanos} ns",
                elapsed.as_micros()
            );
        }

        nanos
    }
}

// ===========================================================================
// Public API — dispatch to the correct arch implementation
// ===========================================================================

/// Verify that the hardware timestamp counter is invariant.
///
/// On x86_64: queries CPUID leaf `0x80000007` EDX bit 8 (Invariant TSC).
/// On other architectures: always returns `true` (system counter is
/// architecturally invariant).
///
/// When this returns `false`, phase duration values obtained from
/// `read_hw_timestamp()` may be unreliable across core migrations or
/// C-state transitions. The detection pipeline records a downgrade reason
/// in `ConfidenceModel` when this condition is detected.
///
/// NIST SP 800-53 AU-8 — TSC reliability assertion for audit record ordering.
#[must_use = "TSC invariance check result must be examined to decide whether to record a downgrade reason"]
pub const fn tsc_is_invariant() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        x86_64_impl::tsc_is_invariant()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback_impl::tsc_is_invariant()
    }
}

/// Read a high-resolution hardware timestamp.
///
/// Returns a `u64` value suitable for computing phase durations via
/// `end.saturating_sub(start)`. The unit is:
///
/// - **x86_64**: CPU cycles (from RDTSCP — serializing, non-NTP-adjusted).
/// - **other architectures**: nanoseconds (from `CLOCK_MONOTONIC_RAW`).
///
/// Within a single detection run on a single architecture, the unit is
/// consistent and the difference is a valid duration measurement.
///
/// Never panics. Returns `0` only in the theoretical case of arithmetic
/// overflow (non-x86_64 fallback only — ~584 years of uptime required).
///
/// NIST SP 800-53 AU-8 — high-resolution timestamps for phase duration
/// recording in `PhaseDuration::duration_ns` audit fields.
#[must_use = "hardware timestamp must be used to compute phase duration; discarding it loses the measurement"]
pub fn read_hw_timestamp() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        x86_64_impl::read_hw_timestamp()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback_impl::read_hw_timestamp()
    }
}
