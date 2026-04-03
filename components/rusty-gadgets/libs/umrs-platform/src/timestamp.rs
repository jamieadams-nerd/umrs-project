// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Boot-Session Timestamps — High-Resolution Monotonic Ordering for Audit Records
//!
//! This module provides [`BootSessionTimestamp`] and [`BootSessionDuration`],
//! a pair of newtypes for ordering audit events with nanosecond precision within
//! a single kernel boot session.
//!
//! ## Design Rationale
//!
//! `std::time::Instant` uses `CLOCK_MONOTONIC` on Linux, which is subject to
//! NTP frequency correction (slewing). For kernel event ordering in MLS contexts,
//! NTP adjustments can cause two adjacent timestamp readings to compare as equal
//! or out-of-order. `CLOCK_MONOTONIC_RAW` bypasses NTP entirely — it runs at the
//! raw hardware clocksource rate (TSC on x86_64, system counter on AArch64), and
//! is guaranteed to advance monotonically at a fixed rate within a boot session.
//!
//! Direct use of RDTSC/cycle counters is architecture-specific and requires
//! `unsafe` code, both of which are prohibited in this codebase. Accessing
//! `CLOCK_MONOTONIC_RAW` via `rustix::time::clock_gettime` achieves equivalent
//! precision using the kernel's own clocksource abstraction — architecture-
//! independent and entirely safe.
//!
//! ## Validity Scope
//!
//! A [`BootSessionTimestamp`] is valid **only within a single boot session**.
//! The underlying clock resets to zero at each kernel boot. Do not compare
//! timestamps across reboots or serialize them as wall-clock values. For
//! wall-clock audit records, use `std::time::SystemTime` alongside this type.
//!
//! ## Overflow Considerations
//!
//! A `u64` nanosecond counter overflows at approximately 584 years of uptime.
//! Systems rebooting within any normal operational cycle will never approach
//! this ceiling. All arithmetic operations use `checked_*` to prevent any
//! theoretical overflow from producing a silent incorrect value.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — timestamps provide the
//!   temporal ordering and precision required for complete, non-repudiable audit
//!   records in MLS environments.
//! - **NIST SP 800-53 AU-8**: Time Stamps — high-resolution monotonic source
//!   ensures audit record sequence is preserved without NTP correction interference.
//! - **NIST SP 800-53 AU-12**: Audit Record Generation — each call to
//!   [`BootSessionTimestamp::now`] generates a unique, ordered reference point
//!   suitable for event sequencing.
//! - **NSA RTB**: Deterministic Execution — fixed-size stack-allocated types
//!   with no heap allocation; arithmetic uses checked operations to prevent
//!   silent overflow.

use rustix::time::{ClockId, clock_gettime};
use std::fmt;

// ===========================================================================
// BootSessionDuration
// ===========================================================================

/// A nanosecond-precision duration within a single boot session.
///
/// Produced by [`BootSessionTimestamp::elapsed_since`]. Represents the elapsed
/// time between two timestamps taken from the same boot session.
///
/// The inner value is nanoseconds. All arithmetic uses `checked_*` operations
/// to prevent silent overflow.
///
/// NIST SP 800-53 AU-8: Time Stamps — duration values support audit record
/// interval analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BootSessionDuration(u64);

impl BootSessionDuration {
    /// Construct directly from a nanosecond count.
    ///
    /// Intended for tests and controlled construction. In production code,
    /// obtain durations via [`BootSessionTimestamp::elapsed_since`].
    #[must_use = "constructed BootSessionDuration carries the nanosecond interval — discarding it loses the duration value"]
    pub const fn from_nanos(nanos: u64) -> Self {
        Self(nanos)
    }

    /// Return the duration as a nanosecond count.
    #[must_use = "nanosecond value must be used; discarding it loses the duration"]
    pub const fn as_nanos(&self) -> u64 {
        self.0
    }

    /// Return the duration as whole microseconds (truncating sub-microsecond remainder).
    #[must_use = "microsecond value must be used; discarding it loses the duration"]
    pub const fn as_micros(&self) -> u64 {
        self.0 / 1_000
    }

    /// Return the duration as whole milliseconds (truncating sub-millisecond remainder).
    #[must_use = "millisecond value must be used; discarding it loses the duration"]
    pub const fn as_millis(&self) -> u64 {
        self.0 / 1_000_000
    }

    /// Add two durations using checked arithmetic.
    ///
    /// Returns `None` on overflow. Use this for any security-relevant duration
    /// arithmetic. NIST SP 800-218 SSDF PW.4.1 — `checked_add` prevents silent
    /// integer overflow.
    #[must_use = "checked_add result must be examined — None indicates overflow"]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }
}

impl fmt::Display for BootSessionDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let micros = self.as_micros();
        let nanos_remainder = self.0 % 1_000;
        write!(f, "{micros}.{nanos_remainder:03} µs")
    }
}

// ===========================================================================
// TimestampError
// ===========================================================================

/// Error type for timestamp construction failures.
///
/// NIST SP 800-53 AU-3: errors in timestamp acquisition must be surfaced
/// explicitly — callers must decide how to handle missing temporal context
/// rather than silently continuing with no ordering reference.
#[derive(Debug, thiserror::Error)]
pub enum TimestampError {
    /// The `tv_sec` field from `clock_gettime` was negative, which indicates
    /// a clock that has not yet been initialized or a platform inconsistency.
    /// Fail-closed: a negative seconds value cannot produce a valid boot-session
    /// timestamp.
    #[error("clock_gettime returned negative tv_sec: {0}")]
    NegativeSecs(i64),

    /// The `tv_nsec` field from `clock_gettime` was negative or out of range
    /// `[0, 999_999_999]`.
    #[error("clock_gettime returned out-of-range tv_nsec: {0}")]
    NegativeNsecs(i64),

    /// The computed nanosecond count overflowed `u64`.
    ///
    /// This cannot occur in practice: a `u64` nanosecond counter would overflow
    /// after approximately 584 years of uptime. Included to ensure arithmetic
    /// is provably correct — any overflow is an error, not undefined behavior.
    #[error("timestamp nanosecond value overflowed u64")]
    Overflow,
}

// ===========================================================================
// BootSessionTimestamp
// ===========================================================================

/// Nanosecond-precision monotonic timestamp within a single kernel boot session.
///
/// Constructed by [`BootSessionTimestamp::now`], which reads
/// `CLOCK_MONOTONIC_RAW` via `rustix::time::clock_gettime`. This clock:
///
/// - Starts at zero at kernel boot.
/// - Advances at the hardware clocksource rate (TSC on x86_64, system counter
///   on AArch64) without NTP frequency adjustment.
/// - Is guaranteed monotonically non-decreasing within a boot session.
/// - Provides nanosecond-resolution ordering suitable for MLS audit sequencing.
///
/// ## Ordering Invariant
///
/// Two [`BootSessionTimestamp`] values obtained from the same boot session
/// compare correctly with `<`, `>`, and `==`. The ordering is total and
/// deterministic. Do not compare timestamps from different boot sessions.
///
/// ## Non-Allocating
///
/// All operations on this type are stack-allocated and heap-free. The inner
/// representation is a single `u64` nanosecond count.
///
/// NIST SP 800-53 AU-3, AU-8, AU-12 — audit record completeness, time stamps,
/// and audit record generation.
/// NSA RTB Deterministic Execution; NIST SP 800-218 SSDF PW.4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BootSessionTimestamp(u64);

impl BootSessionTimestamp {
    /// Capture the current monotonic timestamp from `CLOCK_MONOTONIC_RAW`.
    ///
    /// Uses the hardware clocksource rate with no NTP correction, providing
    /// nanosecond-resolution ordering for audit event sequencing.
    ///
    /// Fails closed on any value that cannot be safely converted to a `u64`
    /// nanosecond count — negative `tv_sec`, negative `tv_nsec`, or overflow.
    ///
    /// NIST SP 800-53 AU-12: Audit Record Generation — each call generates an
    /// ordered reference point for event sequencing.
    ///
    /// # Errors
    ///
    /// Returns `TimestampError::NegativeSecs` if `tv_sec` is negative,
    /// `TimestampError::NegativeNsecs` if `tv_nsec` is negative, or
    /// `TimestampError::Overflow` if the nanosecond conversion overflows `u64`.
    #[must_use = "BootSessionTimestamp::now() returns the current monotonic timestamp — \
                  discarding it silently loses the audit ordering reference point"]
    pub fn now() -> Result<Self, TimestampError> {
        #[cfg(debug_assertions)]
        let t0 = std::time::Instant::now();

        // clock_gettime(CLOCK_MONOTONIC_RAW) returns Timespec directly (no Result).
        // CLOCK_MONOTONIC_RAW: hardware clocksource, no NTP frequency adjustment.
        // On Linux, tv_sec is non-negative for any normally running kernel.
        let ts = clock_gettime(ClockId::MonotonicRaw);

        // Fail-closed: reject negative values — they indicate a broken clock state.
        // NIST SP 800-218 SSDF PW.4.1: checked_mul and checked_add prevent overflow.
        let secs = u64::try_from(ts.tv_sec).map_err(|_| TimestampError::NegativeSecs(ts.tv_sec))?;

        let nsec =
            u64::try_from(ts.tv_nsec).map_err(|_| TimestampError::NegativeNsecs(ts.tv_nsec))?;

        let secs_as_nanos = secs.checked_mul(1_000_000_000u64).ok_or(TimestampError::Overflow)?;

        let total = secs_as_nanos.checked_add(nsec).ok_or(TimestampError::Overflow)?;

        #[cfg(debug_assertions)]
        {
            let elapsed = t0.elapsed();
            log::debug!(
                "BootSessionTimestamp::now() completed in {} µs — value: {} ns since boot",
                elapsed.as_micros(),
                total
            );
        }

        Ok(Self(total))
    }

    /// Return the raw nanosecond count since kernel boot.
    ///
    /// This value is meaningful only within the current boot session.
    #[must_use = "raw nanosecond value must be used; discarding it loses the timestamp"]
    pub const fn as_nanos(&self) -> u64 {
        self.0
    }

    /// Compute the elapsed duration from `earlier` to `self`.
    ///
    /// Returns `None` if `earlier` is after `self` (i.e., the timestamps are
    /// out of order) or if the subtraction overflows. This is fail-closed: any
    /// ambiguity returns `None` rather than a potentially wrong duration.
    ///
    /// NIST SP 800-218 SSDF PW.4.1 — checked subtraction prevents silent underflow.
    ///
    /// NIST SP 800-53 AU-8: Time Stamps — elapsed duration supports audit
    /// record interval analysis.
    #[must_use = "elapsed_since result must be examined — None indicates ordering violation or overflow"]
    pub fn elapsed_since(self, earlier: Self) -> Option<BootSessionDuration> {
        self.0.checked_sub(earlier.0).map(BootSessionDuration)
    }

    /// Return the earlier of two timestamps.
    ///
    /// Useful for building ordered event sequences without explicit comparison.
    #[must_use = "min result must be used; discarding it loses the earlier timestamp"]
    pub const fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    /// Return the later of two timestamps.
    #[must_use = "max result must be used; discarding it loses the later timestamp"]
    pub const fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }
}

impl fmt::Display for BootSessionTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0 / 1_000_000_000;
        let nanos_rem = self.0 % 1_000_000_000;
        write!(f, "T+{secs}.{nanos_rem:09}s")
    }
}
