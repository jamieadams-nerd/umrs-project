// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Demonstrates [`BootSessionTimestamp`] and [`BootSessionDuration`] for
//! audit event ordering with nanosecond precision within a boot session.
//!
//! Run with:
//!   cargo run -p umrs-platform --example timestamp_demo

use umrs_platform::timestamp::{BootSessionDuration, BootSessionTimestamp};

fn main() -> std::process::ExitCode {
    println!("=== BootSessionTimestamp Demo ===");
    println!();
    println!("Clock source: CLOCK_MONOTONIC_RAW (hardware rate, no NTP slewing)");
    println!("Precision:    nanosecond");
    println!("Scope:        valid within this boot session only");
    println!();

    // --- Capture a sequence of audit event timestamps ---
    let (t0, t1, t2, t3) = match (
        capture("event 0: start of audit window"),
        capture("event 1: kernel attribute read"),
        capture("event 2: policy decision"),
        capture("event 3: end of audit window"),
    ) {
        (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
        _ => return std::process::ExitCode::from(1),
    };

    println!();
    println!("--- Interval Analysis ---");
    print_interval("event 0 → event 3 (total window)", t0, t3);
    print_interval("event 0 → event 1", t0, t1);
    print_interval("event 1 → event 2", t1, t2);
    print_interval("event 2 → event 3", t2, t3);

    println!();
    println!("--- Ordering Verification ---");
    let events = [t0, t1, t2, t3];
    for i in 0..events.len() - 1 {
        let ok = events[i] <= events[i + 1];
        println!(
            "  event {} <= event {}: {}",
            i,
            i + 1,
            if ok {
                "OK"
            } else {
                "FAIL (clock regression!)"
            }
        );
    }

    println!();
    println!("--- Duration Arithmetic ---");
    let total = t0.elapsed_since(t3);
    let forward = t3.elapsed_since(t0);
    println!(
        "  t3.elapsed_since(t0) = {:?}",
        forward.map(|d| format!("{}", d))
    );
    println!(
        "  t0.elapsed_since(t3) = {:?} (inverted pair: expect None)",
        total.map(|d| format!("{}", d))
    );

    println!();
    println!("--- BootSessionDuration Conversions ---");
    if let Some(dur) = forward {
        println!("  nanoseconds:  {}", dur.as_nanos());
        println!("  microseconds: {}", dur.as_micros());
        println!("  milliseconds: {}", dur.as_millis());
        println!("  display:      {}", dur);
    }

    println!();
    println!("--- checked_add Example ---");
    let a = BootSessionDuration::from_nanos(500_000);
    let b = BootSessionDuration::from_nanos(750_000);
    match a.checked_add(b) {
        Some(sum) => println!("  500 µs + 750 µs = {}", sum),
        None => println!("  overflow (unexpected)"),
    }

    let overflow_a = BootSessionDuration::from_nanos(u64::MAX);
    let overflow_b = BootSessionDuration::from_nanos(1);
    match overflow_a.checked_add(overflow_b) {
        Some(_) => println!("  overflow check FAILED (should not happen)"),
        None => println!("  u64::MAX + 1 overflow correctly returns None"),
    }
    std::process::ExitCode::SUCCESS
}

fn capture(label: &str) -> Option<BootSessionTimestamp> {
    match BootSessionTimestamp::now() {
        Ok(ts) => {
            println!("  [{label}]  {ts}  ({} ns)", ts.as_nanos());
            Some(ts)
        }
        Err(e) => {
            eprintln!("  [{label}]  ERROR: {e}");
            None
        }
    }
}

fn print_interval(label: &str, earlier: BootSessionTimestamp, later: BootSessionTimestamp) {
    match later.elapsed_since(earlier) {
        Some(dur) => println!("  {label}: {dur}  ({} ns)", dur.as_nanos()),
        None => println!("  {label}: INVERTED PAIR (clock anomaly)"),
    }
}
