// SPDX-License-Identifier: MIT
//! # system_summary — Platform API Demonstration
//!
//! Demonstrates how a consumer of `umrs-platform` approaches four basic tasks
//! using the public API:
//!
//! 1. Detect the operating system (name, version, id, kernel release).
//! 2. Check whether named RPM packages are installed.
//! 3. Show where to find SELinux file security contexts (cross-crate pointer).
//! 4. Check kernel security posture via the indicator catalog.
//!
//! This example was originally written by a first-day guest coder and updated
//! to use the ergonomic Display impls and accessors added in response to their
//! findings. The `// STUCK:` comments from the original have been replaced with
//! `// NOTE:` annotations explaining how the API now works.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p umrs-platform --example system_summary
//! ```
//!
//! ## Compliance
//!
//! - NIST SP 800-53 CM-8 — component inventory via OS detection and substrate
//! - NIST SP 800-53 CA-7 — continuous monitoring via posture snapshot
#![forbid(unsafe_code)]

use std::io::IsTerminal;

use umrs_platform::PackageQueryError;
use umrs_platform::detect::{DetectionError, OsDetector, is_installed};
use umrs_platform::posture::{AssuranceImpact, IndicatorId, PostureSnapshot};

// ---------------------------------------------------------------------------
// Simple terminal colour helpers — honour NO_COLOR
// ---------------------------------------------------------------------------

struct Colours {
    reset: &'static str,
    bold: &'static str,
    green: &'static str,
    yellow: &'static str,
    red: &'static str,
    cyan: &'static str,
}

impl Colours {
    fn new() -> Self {
        // Honour NO_COLOR and non-terminal output (NIST SP 800-53 SI-12 —
        // output must not embed terminal-specific sequences in structured contexts).
        let use_color = std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal();
        if use_color {
            Self {
                reset: "\x1b[0m",
                bold: "\x1b[1m",
                green: "\x1b[32m",
                yellow: "\x1b[33m",
                red: "\x1b[31m",
                cyan: "\x1b[36m",
            }
        } else {
            Self {
                reset: "",
                bold: "",
                green: "",
                yellow: "",
                red: "",
                cyan: "",
            }
        }
    }
}

fn section(label: &str, c: &Colours) {
    println!();
    println!("{}{}=== {} ==={}", c.bold, c.cyan, label, c.reset);
}

// ---------------------------------------------------------------------------
// Task 1: OS identity
// ---------------------------------------------------------------------------

fn print_os_identity(c: &Colours) {
    section("Task 1 — Operating System Identity", c);

    let detector = OsDetector::default();
    let result = match detector.detect() {
        Ok(r) => r,
        Err(DetectionError::ProcfsNotReal) => {
            println!(
                "  {}ERROR: procfs is not real procfs — kernel channel unusable.{}",
                c.red, c.reset
            );
            return;
        }
        Err(DetectionError::PidCoherenceFailed {
            syscall,
            procfs,
        }) => {
            println!(
                "  {}ERROR: PID coherence check failed (syscall={syscall} procfs={procfs}).{}",
                c.red, c.reset
            );
            return;
        }
        Err(e) => {
            // NOTE: DetectionError implements Display via thiserror — use {} not {:?}.
            println!(
                "  {}ERROR: detection pipeline failed: {e}{}",
                c.red, c.reset
            );
            return;
        }
    };

    // NOTE: TrustLevel now implements Display — no manual match arm needed.
    let trust = result.confidence.level();
    println!("  {}confidence  :{} {trust}", c.cyan, c.reset);

    // OS name, id, version from /etc/os-release (if parsed and trusted).
    match &result.os_release {
        None => {
            println!("  {}os-release  : not available{}", c.yellow, c.reset);
        }
        Some(rel) => {
            // NOTE: OsId, OsName, VersionId now implement Display — use {} directly.
            println!("  {}id          :{} {}", c.cyan, c.reset, rel.id);
            println!("  {}name        :{} {}", c.cyan, c.reset, rel.name);
            if let Some(ver) = &rel.version_id {
                println!("  {}version_id  :{} {ver}", c.cyan, c.reset);
            }
            if let Some(pn) = &rel.pretty_name {
                println!("  {}pretty_name :{} {pn}", c.cyan, c.reset);
            }
        }
    }

    // NOTE: KernelRelease is now accessible via DetectionResult::kernel_release.
    // The original intern was blocked here — this field was missing from the
    // public API. It is now populated from /proc/sys/kernel/osrelease in
    // the kernel anchor phase.
    if let Some(kr) = &result.kernel_release {
        let corroboration = if kr.corroborated {
            "(corroborated)"
        } else {
            "(single source)"
        };
        println!(
            "  {}kernel      :{} {} {corroboration}",
            c.cyan, c.reset, kr.release
        );
    } else {
        println!("  {}kernel      :{} not available", c.yellow, c.reset);
    }

    if let Some(boot) = &result.boot_id {
        println!("  {}boot_id     :{} {boot}", c.cyan, c.reset);
    }

    // Show confidence tier with integer value for operator clarity.
    let tier_num = trust as u8;
    println!("  {}trust tier  :{} T{tier_num} ({trust})", c.cyan, c.reset);
}

// ---------------------------------------------------------------------------
// Task 2: Package installation check
// ---------------------------------------------------------------------------

fn print_package_check(c: &Colours) {
    section("Task 2 — Package Installation Check (RPM)", c);

    // NOTE: is_installed now returns Result<bool, PackageQueryError> so callers
    // can distinguish "package absent" from "database unreadable." The intern
    // reported this as a gap when is_installed returned a bare bool.
    let packages = &["openssl-libs", "audit", "policycoreutils", "selinux-policy"];

    for pkg in packages {
        match is_installed(pkg) {
            Ok(true) => {
                println!("  {}[installed]{} {pkg}", c.green, c.reset);
            }
            Ok(false) => {
                println!("  {}[absent]   {} {pkg}", c.yellow, c.reset);
            }
            Err(PackageQueryError::DatabaseUnavailable) => {
                println!(
                    "  {}[db-unavail]{} {pkg} — RPM DB not readable",
                    c.red, c.reset
                );
            }
            Err(PackageQueryError::QueryFailed) => {
                println!("  {}[query-err]{} {pkg} — query failed", c.red, c.reset);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Task 3: SELinux security context of a file
// ---------------------------------------------------------------------------

fn print_selinux_context(c: &Colours) {
    section("Task 3 — SELinux Security Context of /etc/shadow", c);

    // NOTE: The intern was correctly blocked here — file SELinux context is not
    // part of umrs-platform. The umrs-platform module-level docs now include a
    // cross-crate pointer directing users to umrs-selinux for this task.
    //
    // To read the security context of a file, use umrs-selinux:
    //
    //   use umrs_selinux::ls::{SecureDirent, ReadOptions};
    //   let dirent = SecureDirent::open("/etc/shadow", &ReadOptions::default())?;
    //   println!("{}", dirent.security_context());
    //
    // umrs-platform handles kernel attribute access and OS detection.
    // umrs-selinux handles SELinux policy, label parsing, and file contexts.
    println!(
        "  {}NOTE:{} File SELinux context requires the `umrs-selinux` crate.",
        c.cyan, c.reset
    );
    println!(
        "  umrs-platform's module doc now includes a cross-crate pointer for discoverability."
    );
    println!("  See: umrs_selinux::ls::SecureDirent for per-file context reading.");
}

// ---------------------------------------------------------------------------
// Task 4: Kernel security posture
// ---------------------------------------------------------------------------

fn print_posture_summary(c: &Colours) {
    section("Task 4 — Kernel Security Posture", c);

    let snap = PostureSnapshot::collect();

    println!(
        "  posture snapshot: {}/{} indicators meet hardened baseline",
        snap.hardened_count(),
        snap.readable_count()
    );

    if let Some(boot) = &snap.boot_id {
        println!("  boot_id: {boot}");
    }

    // NOTE: IndicatorDescriptor now has a short `label` field suitable for
    // column headers and compact display. The intern noted that only `rationale`
    // (a full sentence) was available, requiring manual truncation.
    println!();
    println!("  Integrity-relevant indicators (label / id / status):");
    println!();

    let integrity_indicators = [
        IndicatorId::ModuleSigEnforce,
        IndicatorId::ModulesDisabled,
        IndicatorId::Lockdown,
        IndicatorId::KexecLoadDisabled,
        IndicatorId::FipsEnabled,
    ];

    for id in integrity_indicators {
        if let Some(report) = snap.get(id) {
            let status = match report.meets_desired {
                Some(true) => format!("{}[hardened]{}", c.green, c.reset),
                Some(false) => format!("{}[finding] {}", c.red, c.reset),
                None => format!("{}[unread]  {}", c.yellow, c.reset),
            };
            // Use the new short label field from IndicatorDescriptor.
            let label = report.descriptor.label;
            // IndicatorId implements Display (sysctl key / cmdline path).
            println!("  {status} {label:<25} ({id})");
        }
    }

    // Show critical-impact findings — useful for operator triage.
    println!();
    println!("  Critical-impact indicator status:");
    let mut found_critical = false;
    for report in snap.by_impact(AssuranceImpact::Critical) {
        found_critical = true;
        let status = match report.meets_desired {
            Some(true) => format!("{}[ok]     {}", c.green, c.reset),
            Some(false) => format!("{}[FINDING]{}", c.red, c.reset),
            None => format!("{}[unread] {}", c.yellow, c.reset),
        };
        // Use label for compact column display.
        let label = report.descriptor.label;
        println!("  {} {label:<25}", status);
    }
    if !found_critical {
        println!("  (none in catalog)");
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let c = Colours::new();

    println!();
    println!(
        "{}{}umrs-platform system_summary — API Demonstration{}",
        c.bold, c.cyan, c.reset
    );
    println!("Exercises: Display impls, KernelRelease accessor, Result-based is_installed,");
    println!("           IndicatorDescriptor.label, cross-crate discovery pointer.");

    print_os_identity(&c);
    print_package_check(&c);
    print_selinux_context(&c);
    print_posture_summary(&c);

    println!();
    println!("{}Done.{}", c.bold, c.reset);
}
