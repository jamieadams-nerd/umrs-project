// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # read_timestamp — Hardware Timestamp API Demonstration
//!
//! Demonstrates `umrs_hw::read_hw_timestamp()` and `umrs_hw::tsc_is_invariant()`.
//!
//! Run with:
//! ```text
//! cargo run -p umrs-hw --example read_timestamp
//! ```
//!
//! Expected output on x86_64:
//! ```text
//! TSC invariant: true
//! t0 = 1234567890123
//! t1 = 1234567892456
//! duration = 2333 (cycles on x86_64, nanoseconds on other arches)
//! ```

fn main() {
    let invariant = umrs_hw::tsc_is_invariant();
    println!("TSC invariant: {invariant}");
    if !invariant {
        println!(
            "WARNING: TSC is not invariant — phase durations may be unreliable \
             across core migrations or C-state transitions."
        );
    }

    let t0 = umrs_hw::read_hw_timestamp();
    // Spin briefly to create a measurable span.
    let mut sink: u64 = 0;
    for i in 0u64..100_000 {
        sink = sink.wrapping_add(i);
    }
    let t1 = umrs_hw::read_hw_timestamp();
    let _ = sink;

    let duration = t1.saturating_sub(t0);

    println!("t0 = {t0}");
    println!("t1 = {t1}");

    #[cfg(target_arch = "x86_64")]
    println!("duration = {duration} cycles");

    #[cfg(not(target_arch = "x86_64"))]
    println!("duration = {duration} ns (CLOCK_MONOTONIC_RAW fallback)");
}
