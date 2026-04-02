// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for [`BootSessionTimestamp`] and [`BootSessionDuration`].
//!
//! These tests verify:
//! - `now()` returns a valid, non-zero timestamp on a running kernel.
//! - Monotonic ordering: two sequential calls produce a non-decreasing sequence.
//! - `elapsed_since` correctly computes duration and rejects inverted pairs.
//! - `BootSessionDuration` arithmetic and conversions are correct.
//! - Display formatting is well-formed.
//! - Fail-closed arithmetic on synthetic overflow edge cases.
//!
//! NIST SP 800-53 AU-3, AU-8, AU-12 — timestamp correctness is a prerequisite
//! for non-repudiable audit record ordering.

use umrs_platform::timestamp::{BootSessionDuration, BootSessionTimestamp};

// ---------------------------------------------------------------------------
// now() — basic validity
// ---------------------------------------------------------------------------

/// A timestamp obtained from a running kernel must be non-zero.
///
/// `CLOCK_MONOTONIC_RAW` starts at approximately zero at kernel boot and
/// increases monotonically. Any system that has been up for even a millisecond
/// will produce a non-zero value.
#[test]
fn now_returns_nonzero() {
    let ts = BootSessionTimestamp::now().expect("clock_gettime(CLOCK_MONOTONIC_RAW) failed");
    assert!(
        ts.as_nanos() > 0,
        "expected non-zero nanoseconds from a running kernel"
    );
}

/// Two sequential calls to `now()` must produce a non-decreasing sequence.
///
/// `CLOCK_MONOTONIC_RAW` is guaranteed monotonically non-decreasing. If
/// `t2 < t1`, the kernel's clocksource is broken — flag loudly.
#[test]
fn now_is_monotonically_non_decreasing() {
    let t1 = BootSessionTimestamp::now().expect("first now() failed");
    let t2 = BootSessionTimestamp::now().expect("second now() failed");
    assert!(
        t2 >= t1,
        "CLOCK_MONOTONIC_RAW regression: t2 ({}) < t1 ({})",
        t2.as_nanos(),
        t1.as_nanos()
    );
}

/// Many sequential readings must all be non-decreasing.
#[test]
fn now_never_regresses_over_many_reads() {
    let mut prev = BootSessionTimestamp::now().expect("initial now() failed");
    for i in 0..100 {
        let curr = BootSessionTimestamp::now().unwrap_or_else(|e| {
            panic!("now() failed on iteration {i}: {e}");
        });
        assert!(
            curr >= prev,
            "clock regression at iteration {i}: curr={} prev={}",
            curr.as_nanos(),
            prev.as_nanos()
        );
        prev = curr;
    }
}

// ---------------------------------------------------------------------------
// elapsed_since — correct ordering and fail-closed
// ---------------------------------------------------------------------------

/// `elapsed_since` returns the non-negative delta between two ordered readings.
#[test]
fn elapsed_since_returns_nonnegative_delta() {
    let t1 = BootSessionTimestamp::now().expect("t1 failed");
    let t2 = BootSessionTimestamp::now().expect("t2 failed");
    let elapsed = t2.elapsed_since(t1);
    assert!(
        elapsed.is_some(),
        "elapsed_since returned None for ordered timestamps t1={} t2={}",
        t1.as_nanos(),
        t2.as_nanos()
    );
}

/// `elapsed_since` returns `None` when `earlier > self` (inverted pair).
///
/// This is the fail-closed behaviour: an inverted timestamp pair indicates a
/// programming error or clock anomaly. Returning `None` forces the caller to
/// handle the anomaly explicitly rather than producing a silent wrong value.
#[test]
fn elapsed_since_inverted_pair_returns_none() {
    // Construct two synthetic timestamps where t_large > t_small.
    // Use from-nanos construction via BootSessionDuration::from_nanos as a
    // workaround — timestamps cannot be fabricated directly (private constructor),
    // but we can obtain real timestamps and check the relation.
    let t1 = BootSessionTimestamp::now().expect("t1 failed");
    let t2 = BootSessionTimestamp::now().expect("t2 failed");
    // t1.elapsed_since(t2): asking "how long since t2 happened before t1?"
    // If t2 >= t1 (both obtained in order), this is None or zero.
    // We want the case where t2 > t1 strictly.
    if t2.as_nanos() > t1.as_nanos() {
        // t1.elapsed_since(t2): t1 < t2 so this is an inverted pair — must be None.
        let inverted = t1.elapsed_since(t2);
        assert!(
            inverted.is_none(),
            "elapsed_since must return None for inverted pair: self={} earlier={}",
            t1.as_nanos(),
            t2.as_nanos()
        );
    }
    // If t1 == t2, elapsed_since returns Some(0) — which is correct.
}

/// `elapsed_since` returns `Some(0)` for equal timestamps.
#[test]
fn elapsed_since_equal_timestamps_returns_zero() {
    let t = BootSessionTimestamp::now().expect("now() failed");
    let dur = t.elapsed_since(t);
    assert_eq!(
        dur,
        Some(BootSessionDuration::from_nanos(0)),
        "elapsed_since of a timestamp with itself must be zero"
    );
}

// ---------------------------------------------------------------------------
// BootSessionDuration — arithmetic and conversion
// ---------------------------------------------------------------------------

/// `from_nanos` and `as_nanos` are inverse operations.
#[test]
fn duration_roundtrip_nanos() {
    let nanos = 123_456_789u64;
    let d = BootSessionDuration::from_nanos(nanos);
    assert_eq!(d.as_nanos(), nanos);
}

/// `as_micros` truncates correctly.
#[test]
fn duration_as_micros_truncates() {
    let d = BootSessionDuration::from_nanos(1_500); // 1 µs + 500 ns
    assert_eq!(d.as_micros(), 1);
}

/// `as_millis` truncates correctly.
#[test]
fn duration_as_millis_truncates() {
    let d = BootSessionDuration::from_nanos(1_500_000); // 1 ms + 500 µs
    assert_eq!(d.as_millis(), 1);
}

/// `checked_add` succeeds for normal values.
#[test]
fn duration_checked_add_normal() {
    let a = BootSessionDuration::from_nanos(1_000_000);
    let b = BootSessionDuration::from_nanos(2_000_000);
    let result = a.checked_add(b);
    assert_eq!(result, Some(BootSessionDuration::from_nanos(3_000_000)));
}

/// `checked_add` returns `None` on overflow — fail-closed arithmetic.
#[test]
fn duration_checked_add_overflow_returns_none() {
    let a = BootSessionDuration::from_nanos(u64::MAX);
    let b = BootSessionDuration::from_nanos(1);
    assert!(
        a.checked_add(b).is_none(),
        "checked_add must return None on u64 overflow"
    );
}

// ---------------------------------------------------------------------------
// Ordering — Ord/PartialOrd correctness
// ---------------------------------------------------------------------------

/// Ordering of timestamps matches ordering of nanosecond values.
#[test]
fn timestamp_ordering_matches_nanos() {
    let t1 = BootSessionTimestamp::now().expect("t1 failed");
    let t2 = BootSessionTimestamp::now().expect("t2 failed");
    // t1 was sampled first, so t1 <= t2 must hold.
    assert!(
        t1 <= t2,
        "timestamp ordering invariant violated: t1={} t2={}",
        t1.as_nanos(),
        t2.as_nanos()
    );
}

/// Duration ordering: larger nanosecond count compares greater.
#[test]
fn duration_ordering() {
    let small = BootSessionDuration::from_nanos(100);
    let large = BootSessionDuration::from_nanos(200);
    assert!(small < large);
    assert!(large > small);
    assert_eq!(small, BootSessionDuration::from_nanos(100));
}

// ---------------------------------------------------------------------------
// min / max
// ---------------------------------------------------------------------------

#[test]
fn timestamp_min_returns_earlier() {
    let t1 = BootSessionTimestamp::now().expect("t1 failed");
    let t2 = BootSessionTimestamp::now().expect("t2 failed");
    let earliest = t1.min(t2);
    assert_eq!(earliest, t1.min(t2));
    assert!(earliest <= t1);
    assert!(earliest <= t2);
}

#[test]
fn timestamp_max_returns_later() {
    let t1 = BootSessionTimestamp::now().expect("t1 failed");
    let t2 = BootSessionTimestamp::now().expect("t2 failed");
    let latest = t1.max(t2);
    assert!(latest >= t1);
    assert!(latest >= t2);
}

// ---------------------------------------------------------------------------
// Display formatting
// ---------------------------------------------------------------------------

/// BootSessionTimestamp display format is "T+{secs}.{nanos_rem}s".
#[test]
fn timestamp_display_format() {
    let t = BootSessionTimestamp::now().expect("now() failed");
    let s = t.to_string();
    assert!(
        s.starts_with("T+"),
        "BootSessionTimestamp display must start with 'T+': got '{s}'"
    );
    assert!(
        s.ends_with('s'),
        "BootSessionTimestamp display must end with 's': got '{s}'"
    );
    assert!(
        s.contains('.'),
        "BootSessionTimestamp display must contain '.': got '{s}'"
    );
}

/// BootSessionDuration display format ends with "µs".
#[test]
fn duration_display_format() {
    let d = BootSessionDuration::from_nanos(1_234_567);
    let s = d.to_string();
    assert!(
        s.contains("µs"),
        "BootSessionDuration display must contain 'µs': got '{s}'"
    );
    // 1_234_567 ns = 1234 µs + 567 ns → "1234.567 µs"
    assert!(
        s.starts_with("1234.567"),
        "expected '1234.567 µs', got '{s}'"
    );
}
