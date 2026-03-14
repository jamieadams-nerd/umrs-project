// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Kernel Security Posture Probe — demonstration example.
//!
//! Collects all kernel security posture signals and displays a formatted
//! summary report to stdout. Demonstrates both the novice-friendly
//! `PostureSnapshot` interface and the expert path (individual signal lookup,
//! catalog iteration).
//!
//! # Running
//!
//! ```bash
//! cargo run -p umrs-platform --example posture_demo
//! ```
//!
//! # Note
//!
//! Many signals require read access to `/proc/sys/` nodes. On systems where
//! access is restricted or the node is absent (containers, minimal kernels),
//! the signal will show `live=None` — this is expected graceful degradation.

use umrs_platform::posture::{
    AssuranceImpact, ContradictionKind, PostureSnapshot, SignalId,
    catalog::SIGNALS,
};

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug"),
    )
    .init();

    println!(
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    );
    println!(" UMRS Kernel Security Posture Probe — Phase 1");
    println!(
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    );

    // ── Collect snapshot ────────────────────────────────────────────────────
    let snap = PostureSnapshot::collect();

    // ── Summary ─────────────────────────────────────────────────────────────
    let boot_id = snap.boot_id.as_deref().unwrap_or("<unavailable>");
    println!("Boot ID   : {boot_id}");
    println!(
        "Hardened  : {}/{} signals meet their desired value",
        snap.hardened_count(),
        snap.readable_count()
    );
    println!("Catalog   : {} total signals", snap.reports.len());
    println!(
        "Findings  : {} signal(s) not meeting desired value",
        snap.findings().count()
    );
    println!(
        "Contradictions: {} signal(s) with live/configured disagreement",
        snap.contradictions().count()
    );
    println!();

    // ── Per-signal table ────────────────────────────────────────────────────
    println!(
        "{:<35} {:<10} {:<12} {:<8} Status",
        "Signal", "Live", "Meets", "Impact"
    );
    println!("{}", "─".repeat(78));

    for report in snap.iter() {
        let live_str = report
            .live_value
            .as_ref()
            .map_or_else(|| "<unavail>".to_owned(), |v| v.to_string());

        let meets_str = match report.meets_desired {
            Some(true) => "PASS",
            Some(false) => "FAIL",
            None => "N/A",
        };

        let impact_str = match report.descriptor.impact {
            AssuranceImpact::Critical => "CRITICAL",
            AssuranceImpact::High => "HIGH    ",
            AssuranceImpact::Medium => "MEDIUM  ",
        };

        let status = if let Some(kind) = report.contradiction {
            match kind {
                ContradictionKind::EphemeralHotfix => "[EphemeralHotfix]",
                ContradictionKind::BootDrift => "[BootDrift]",
                ContradictionKind::SourceUnavailable => "[SourceUnavailable]",
            }
        } else if report.live_value.is_none() {
            "[node absent]"
        } else {
            ""
        };

        println!(
            "{:<35} {:<10} {:<12} {:<8} {}",
            report.descriptor.id.to_string(),
            live_str,
            meets_str,
            impact_str,
            status
        );
    }

    println!();

    // ── Findings detail ─────────────────────────────────────────────────────
    let findings: Vec<_> = snap.findings().collect();
    if findings.is_empty() {
        println!(
            "No findings — all readable signals meet their desired value."
        );
    } else {
        println!(
            "── Findings ──────────────────────────────────────────────────────────────────"
        );
        for report in &findings {
            println!();
            println!("  Signal  : {}", report.descriptor.id);
            println!("  Impact  : {:?}", report.descriptor.impact);
            println!("  Live    : {:?}", report.live_value);
            println!("  Desired : {:?}", report.descriptor.desired);
            println!("  Rationale: {}", report.descriptor.rationale);
            println!("  Controls: {}", report.descriptor.nist_controls);
        }
    }

    println!();

    // ── Contradictions ───────────────────────────────────────────────────────
    let contradictions: Vec<_> = snap.contradictions().collect();
    if !contradictions.is_empty() {
        println!(
            "── Contradictions ────────────────────────────────────────────────────────────"
        );
        for report in &contradictions {
            // contradictions() only yields reports where contradiction.is_some(),
            // so this match is exhaustive in practice. Pattern-matching is used
            // instead of .expect() to comply with the no-unwrap/expect rule.
            if let Some(kind) = report.contradiction {
                println!(
                    "  {:?}: {:?}  configured={:?}",
                    report.descriptor.id,
                    kind,
                    report.configured_value.as_ref().map(|c| &c.raw)
                );
            }
        }
        println!();
    }

    // ── Expert path: individual signal lookup ────────────────────────────────
    println!(
        "── Individual signal lookup (KptrRestrict) ───────────────────────────────────"
    );
    if let Some(r) = snap.get(SignalId::KptrRestrict) {
        println!("  live={:?}  meets={:?}", r.live_value, r.meets_desired);
    } else {
        println!("  KptrRestrict not found in snapshot");
    }

    println!();

    // ── Expert path: catalog iteration ──────────────────────────────────────
    println!(
        "── Static catalog (first 5 entries) ─────────────────────────────────────────"
    );
    for desc in SIGNALS.iter().take(5) {
        println!(
            "  {:?}: desired={:?} impact={:?}",
            desc.id, desc.desired, desc.impact
        );
    }

    println!();
    println!(
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    );
    println!(" Posture probe complete.");
    println!(
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    );
}
