// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # Example: RPM Package Substrate Probe
//!
//! Demonstrates the RPM probe interface:
//!
//! 1. Run the probe phase to detect the RPM substrate.
//! 2. Check package installation by name.
//! 3. Query file ownership for `/usr/lib/os-release`.
//! 4. Query the installed digest for `/usr/lib/os-release`.
//!
//! Run on an RHEL 10 system with:
//! ```bash
//! cargo run -p umrs-platform --example rpm_probe
//! ```
//!
//! On non-RHEL systems the probe will report the database as absent and
//! all queries will return `None`.

use std::path::Path;

use umrs_platform::detect::is_installed;
use umrs_platform::detect::substrate::PackageProbe;
use umrs_platform::detect::substrate::rpm::RpmProbe;
use umrs_platform::evidence::EvidenceBundle;

fn main() {
    // Initialise a simple stderr logger so debug messages are visible.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== UMRS Platform — RPM Probe Example ===\n");

    // ── Phase: probe ──────────────────────────────────────────────────────────
    let probe = RpmProbe::new();
    let mut bundle = EvidenceBundle::new();
    let result = probe.probe(&mut bundle);

    println!("Probe:            {}", result.probe_name);
    println!("parse_ok:         {}", result.parse_ok);
    println!("can_query_owner:  {}", result.can_query_ownership);
    println!("can_verify_digest: {}", result.can_verify_digest);

    if let Some(ref id) = result.identity {
        println!("OS family:        {:?}", id.family);
        println!("Distro:           {:?}", id.distro);
        println!("Facts collected:  {}", id.facts_count);
    }

    println!("\nEvidence records: {}", bundle.len());
    for rec in bundle.iter() {
        println!(
            "  [{:?}] {} — parse_ok={}",
            rec.source_kind, rec.path_requested, rec.parse_ok
        );
        for note in &rec.notes {
            println!("    note: {note}");
        }
    }

    // ── Package installation check ────────────────────────────────────────────
    println!();
    for pkg in &["bash", "coreutils", "selinux-policy", "nonexistent-xyzzy"] {
        match is_installed(pkg) {
            Ok(true) => println!("is_installed({pkg:<28}) = installed"),
            Ok(false) => println!("is_installed({pkg:<28}) = absent"),
            Err(e) => println!("is_installed({pkg:<28}) = error: {e}"),
        }
    }

    // ── File ownership query ──────────────────────────────────────────────────
    println!();
    let target = Path::new("/usr/lib/os-release");

    // We need (dev, ino) to call query_ownership — get them via stat.
    match nix::sys::stat::stat(target) {
        Ok(stat) => {
            let dev = stat.st_dev;
            let ino = stat.st_ino;
            match probe.query_ownership(dev, ino, target) {
                Some(ownership) => {
                    println!("Ownership of {}:", target.display());
                    println!("  package: {}", ownership.package_name);
                    println!("  version: {}", ownership.package_version);
                    for trail in &ownership.evidence_trail {
                        println!("  trail:   {trail}");
                    }
                }
                None => {
                    println!("Ownership of {}: not found in RPM DB", target.display());
                }
            }
        }
        Err(e) => {
            println!("Could not stat {}: {e}", target.display());
        }
    }

    // ── File digest query ─────────────────────────────────────────────────────
    println!();
    match probe.installed_digest(target) {
        Some(digest) => {
            println!("Installed digest for {}:", target.display());
            println!("  algorithm: {:?}", digest.algorithm);
            println!(
                "  bytes:     {} bytes (first 8: {:02x?}...)",
                digest.value.len(),
                &digest.value[..digest.value.len().min(8)]
            );
        }
        None => {
            println!("No installed digest found for {}", target.display());
        }
    }

    println!("\nDone.");
}
