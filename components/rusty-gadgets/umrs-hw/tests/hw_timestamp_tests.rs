// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for `umrs_hw::hw_timestamp`.
//!
//! These tests verify the behavioral contract of the public API without
//! requiring architecture-specific hardware details. They run on any target.

use umrs_hw::{read_hw_timestamp, tsc_is_invariant};

// ===========================================================================
// read_hw_timestamp
// ===========================================================================

/// Two successive reads must be non-decreasing.
///
/// This is the fundamental monotonicity contract of the timestamp source.
/// On x86_64, RDTSCP is guaranteed monotonic within a core. On other
/// architectures, CLOCK_MONOTONIC_RAW is guaranteed monotonic.
///
/// NOTE: This test can theoretically fail on a non-invariant TSC if the
/// thread migrates between cores between the two reads and the receiving
/// core's TSC is behind. In practice this does not occur on any modern
/// RHEL 10 x86_64 system because the TSC is invariant and synchronized
/// at boot.
#[test]
fn read_hw_timestamp_is_non_decreasing() {
    let t0 = read_hw_timestamp();
    let t1 = read_hw_timestamp();
    assert!(t1 >= t0, "second timestamp ({t1}) must be >= first ({t0})");
}

/// The timestamp must be non-zero on any running system.
///
/// On x86_64 the TSC starts at 0 at power-on and has advanced by the time
/// any userspace code runs. On the fallback path, CLOCK_MONOTONIC_RAW
/// started at boot and has advanced. Zero would indicate a broken clock.
#[test]
fn read_hw_timestamp_is_nonzero() {
    let t = read_hw_timestamp();
    assert_ne!(
        t, 0,
        "hardware timestamp must be non-zero on a running system"
    );
}

/// Consecutive reads show measurable elapsed time.
///
/// Two reads separated by at least one spin iteration must differ. This
/// verifies the clock is actually advancing, not frozen.
#[test]
fn read_hw_timestamp_advances() {
    let t0 = read_hw_timestamp();
    // Spin briefly to ensure measurable elapsed time without sleeping.
    let mut sink: u64 = 0;
    for i in 0u64..10_000 {
        sink = sink.wrapping_add(i);
    }
    let t1 = read_hw_timestamp();
    // Use sink to prevent the loop from being optimized away.
    let _ = sink;
    assert!(t1 > t0, "timestamp must advance: t0={t0} t1={t1}");
}

/// saturating_sub produces the expected duration value.
///
/// Phase duration is always computed as `end.saturating_sub(start)`.
/// Verify the arithmetic is correct for the typical case.
#[test]
fn saturating_sub_duration_is_correct() {
    let start: u64 = 1_000_000;
    let end: u64 = 2_500_000;
    let duration = end.saturating_sub(start);
    assert_eq!(duration, 1_500_000);
}

/// saturating_sub does not underflow when end < start.
///
/// This is the anomaly path Jamie requested: if the clock returns end < start
/// (e.g., a non-invariant TSC after core migration), saturating_sub returns 0
/// rather than wrapping. The detection pipeline records a downgrade reason in
/// this case.
#[test]
fn saturating_sub_does_not_underflow() {
    let start: u64 = 2_000_000;
    let end: u64 = 1_000_000; // anomalous — end < start
    let duration = end.saturating_sub(start);
    assert_eq!(
        duration, 0,
        "saturating_sub must return 0 on underflow, not wrap"
    );
}

// ===========================================================================
// tsc_is_invariant
// ===========================================================================

/// tsc_is_invariant must return without panicking.
///
/// The return value is architecture-dependent and cannot be asserted to a
/// specific value in a portable test. We only verify that the call completes.
#[test]
fn tsc_is_invariant_does_not_panic() {
    let _ = tsc_is_invariant();
}

/// On non-x86_64 targets, tsc_is_invariant always returns true.
#[test]
#[cfg(not(target_arch = "x86_64"))]
fn tsc_is_invariant_always_true_on_non_x86_64() {
    assert!(
        tsc_is_invariant(),
        "fallback path must always report invariant TSC"
    );
}
