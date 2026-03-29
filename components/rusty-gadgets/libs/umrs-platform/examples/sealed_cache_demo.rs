// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Sealed Evidence Cache (SEC) — demonstration example.
//!
//! Shows how to wrap `OsDetector` in a `SealedCache` to cache the result
//! of the expensive OS detection pipeline with a cryptographic HMAC seal.
//!
//! Run with:
//!   cargo run -p umrs-platform --example sealed_cache_demo

use umrs_platform::{SealedCache, TrustLevel, detect::OsDetector};

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug"),
    )
    .init();

    println!("=== Sealed Evidence Cache (SEC) Demo ===\n");

    // ── Construct the cache ──────────────────────────────────────────────────
    // SealedCache::new reads /proc/sys/crypto/fips_enabled at construction
    // time (TOCTOU safety — FIPS state is captured once, not at each seal).
    // If FIPS is active, caching_enabled() returns false and every query
    // re-runs the pipeline.
    let mut cache = SealedCache::new(OsDetector::default());

    println!("Cache initialized:");
    println!("  TTL              : {} seconds", cache.ttl().as_secs());
    println!("  Caching enabled  : {}", cache.caching_enabled());
    println!("  Status           : {:?}\n", cache.status());

    if !cache.caching_enabled() {
        println!(
            "Note: FIPS mode is active — HMAC-SHA-256 (hmac + sha2) is not"
        );
        println!("FIPS 140-3 validated. Sealed cache is disabled. Every query");
        println!("re-runs the full pipeline.\n");
    }

    // ── First query (pipeline run) ───────────────────────────────────────────
    println!("--- First query (pipeline run or cache miss) ---");
    let t0 = std::time::Instant::now();
    let result1 = match cache.query() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Detection pipeline failed (hard gate): {e}");
            std::process::exit(1);
        }
    };
    let elapsed1 = t0.elapsed();

    println!("  Trust level  : {:?}", result1.confidence.level());
    println!(
        "  Boot ID      : {}",
        result1.boot_id.as_deref().unwrap_or("<not available>")
    );
    println!(
        "  OS ID        : {}",
        result1
            .os_release
            .as_ref()
            .map(|r| r.id.as_str())
            .unwrap_or("<not available>")
    );
    println!("  Evidence recs: {}", result1.evidence.len());
    println!("  Elapsed      : {} µs", elapsed1.as_micros());
    println!("  Status after : {:?}\n", cache.status());

    // Trust level interpretation
    let tier = match result1.confidence.level() {
        TrustLevel::Untrusted => "T0 — no kernel anchor",
        TrustLevel::KernelAnchored => "T1 — procfs verified",
        TrustLevel::EnvAnchored => "T2 — mount topology checked",
        TrustLevel::SubstrateAnchored => "T3 — package substrate confirmed",
        TrustLevel::IntegrityAnchored => "T4 — digest integrity verified",
    };
    println!("  Trust tier   : {tier}");

    if !result1.confidence.downgrade_reasons.is_empty() {
        println!("  Downgrade reasons:");
        for reason in &result1.confidence.downgrade_reasons {
            println!("    - {reason}");
        }
    }
    println!();

    // ── Second query (cache hit or re-run) ───────────────────────────────────
    println!("--- Second query (cache hit path, if entry Live) ---");
    let status_before = cache.status();
    let t1 = std::time::Instant::now();
    let result2 = match cache.query() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Second query failed: {e}");
            std::process::exit(1);
        }
    };
    let elapsed2 = t1.elapsed();

    println!("  Status before query : {status_before:?}");
    println!("  Trust level         : {:?}", result2.confidence.level());
    println!("  Elapsed             : {} µs", elapsed2.as_micros());
    println!("  Status after query  : {:?}\n", cache.status());

    // ── Security properties summary ──────────────────────────────────────────
    println!("--- Security properties ---");
    println!("  Seal algorithm   : HMAC-SHA-256");
    println!(
        "  Key lifetime     : ephemeral (derived from boot_id + wall-clock nanos)"
    );
    println!("  Key zeroized on  : SealedCache::drop (ZeroizeOnDrop)");
    println!(
        "  Seal covers      : TrustLevel byte + SHA-256(EvidenceBundle) + identifiers"
    );
    println!("  Seal failure     : discard, re-run pipeline, log::warn!");
    println!(
        "  FIPS gate        : /proc/sys/crypto/fips_enabled (read at init, not at seal)"
    );
    println!(
        "  TTL ceiling      : {} seconds",
        umrs_platform::MAX_TTL_SECS
    );
    println!();
    println!("  Relevant controls:");
    println!(
        "    NIST SP 800-53 SC-28 — integrity protection for cached result"
    );
    println!("    NIST SP 800-53 SC-12 — ephemeral key, zeroized on drop");
    println!(
        "    NIST SP 800-53 SI-7  — seal failure → re-verify, not stale data"
    );
    println!("    NIST SP 800-53 AU-3  — seal failures logged with log::warn!");
}
