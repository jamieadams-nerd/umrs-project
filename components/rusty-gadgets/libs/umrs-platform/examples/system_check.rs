// SPDX-License-Identifier: MIT
//! # system_check — First-Day Intern Platform API Walkthrough
//!
//! Exercises four basic tasks against the `umrs-platform` public API,
//! written from scratch using only rustdoc and the existing examples as
//! reference. Every point of confusion is marked with a `// STUCK:` comment.
//!
//! ## Tasks demonstrated
//!
//! 1. Detect the operating system (name, version, kernel release).
//! 2. Check whether `openssl-libs` and `audit` are installed (RPM).
//! 3. Show the SELinux security context of `/etc/shadow` — or explain why
//!    this requires a different crate.
//! 4. Probe security posture indicators relevant to kernel integrity (as a
//!    proxy for IMA/EVM status, which is not yet a named catalog entry).
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p umrs-platform --example system_check
//! ```
//!
//! ## Compliance
//!
//! - NIST SP 800-53 CM-8 — component inventory (OS identity, package check)
//! - NIST SP 800-53 CA-7 — continuous monitoring (posture snapshot)
#![forbid(unsafe_code)]

use std::io::IsTerminal;

use umrs_platform::PackageQueryError;
use umrs_platform::detect::{DetectionError, OsDetector, is_installed};
use umrs_platform::posture::{AssuranceImpact, IndicatorId, PostureSnapshot};

// ---------------------------------------------------------------------------
// Minimal colour helper — honours NO_COLOR per NIST SP 800-53 SI-12
// ---------------------------------------------------------------------------

struct C {
    reset: &'static str,
    bold: &'static str,
    green: &'static str,
    yellow: &'static str,
    red: &'static str,
    cyan: &'static str,
}

impl C {
    fn new() -> Self {
        let on = std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal();
        if on {
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

fn header(label: &str, c: &C) {
    println!();
    println!("{}{}--- {} ---{}", c.bold, c.cyan, label, c.reset);
}

// ---------------------------------------------------------------------------
// Task 1: OS identity — name, version, kernel release
//
// STUCK (mild): The rustdoc for `OsRelease` shows `id`, `name`,
// `version_id`, `pretty_name` as fields, but does NOT show the concrete
// types clearly inline. Looking at the existing system_summary.rs example
// helped confirm they implement Display. Without that example I would have
// tried `rel.id.to_string()` and then `{}` formatting to figure out which
// worked.
//
// STUCK (mild): `OsDetector::default()` is not obvious from the struct page.
// The sidebar lists `Default` in trait implementations, so it's derivable,
// but a short note in the struct-level doc would help a first-day user skip
// the trait implementation section.
// ---------------------------------------------------------------------------

fn task1_os_identity(c: &C) {
    header("Task 1 — Operating System Identity", c);

    let result = match OsDetector::default().detect() {
        Ok(r) => r,
        Err(DetectionError::ProcfsNotReal) => {
            println!(
                "  {}ERROR: procfs is not real — cannot establish kernel anchor.{}",
                c.red, c.reset
            );
            return;
        }
        Err(DetectionError::PidCoherenceFailed {
            syscall,
            procfs,
        }) => {
            println!(
                "  {}ERROR: PID coherence failed (syscall={syscall} procfs={procfs}).{}",
                c.red, c.reset
            );
            return;
        }
        Err(e) => {
            // DetectionError implements Display via thiserror.
            println!("  {}ERROR: detection failed: {e}{}", c.red, c.reset);
            return;
        }
    };

    // OS release fields from /etc/os-release.
    match &result.os_release {
        None => println!("  {}os-release: not available{}", c.yellow, c.reset),
        Some(rel) => {
            // OsId, OsName, VersionId implement Display — use {} directly.
            println!("  {}ID        :{} {}", c.cyan, c.reset, rel.id);
            println!("  {}NAME      :{} {}", c.cyan, c.reset, rel.name);
            if let Some(ver) = &rel.version_id {
                println!("  {}VERSION_ID:{} {ver}", c.cyan, c.reset);
            }
            if let Some(pn) = &rel.pretty_name {
                println!("  {}PRETTY    :{} {pn}", c.cyan, c.reset);
            }
        }
    }

    // Kernel release from /proc/sys/kernel/osrelease.
    //
    // STUCK (mild): `DetectionResult::kernel_release` is documented as
    // `Option<KernelRelease>` but the KernelRelease struct page does not show
    // what Display looks like — or even if it implements Display. The sidebar
    // only shows Clone and Debug. I had to check the system_summary.rs example
    // to learn that `.release` is the field name (a String) to use for
    // display, and `.corroborated` is a bool. The rustdoc for KernelRelease
    // should show its fields in the main body with types and doc text, not
    // just in the sidebar TOC.
    match &result.kernel_release {
        None => println!("  {}kernel    : not available{}", c.yellow, c.reset),
        Some(kr) => {
            let note = if kr.corroborated {
                "(corroborated)"
            } else {
                "(single source)"
            };
            println!("  {}kernel    :{} {} {note}", c.cyan, c.reset, kr.release);
        }
    }

    // Confidence tier.
    let trust = result.confidence.level();
    println!("  {}trust     :{} {trust}", c.cyan, c.reset);
}

// ---------------------------------------------------------------------------
// Task 2: Check if packages are installed
//
// STUCK (none): `is_installed` is re-exported at `umrs_platform::detect` and
// the function signature is clear: `fn(pkgname: &str) -> Result<bool,
// PackageQueryError>`. The error variants `DatabaseUnavailable` and
// `QueryFailed` appear in the sidebar immediately. This was the easiest part
// of the API to use.
// ---------------------------------------------------------------------------

fn task2_package_check(c: &C) {
    header("Task 2 — Package Installation Check (RPM)", c);

    // The task asks us to check openssl-libs and audit.
    let packages = &["openssl-libs", "audit"];

    for pkg in packages {
        match is_installed(pkg) {
            Ok(true) => println!("  {}[installed]{} {pkg}", c.green, c.reset),
            Ok(false) => println!("  {}[absent]   {} {pkg}", c.yellow, c.reset),
            Err(PackageQueryError::DatabaseUnavailable) => {
                println!(
                    "  {}[db-error] {} {pkg} — RPM DB unavailable",
                    c.red, c.reset
                );
            }
            Err(PackageQueryError::QueryFailed) => {
                println!("  {}[qry-error]{} {pkg} — query failed", c.red, c.reset);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Task 3: SELinux security context of /etc/shadow
//
// STUCK (BLOCKER): There is no function or type in `umrs-platform` to read
// the SELinux security context of a file. I searched the full public API
// (every module listed in the rustdoc index — detect, posture, os_release,
// os_identity, evidence, kattrs, confidence, sealed_cache, timestamp) and
// found nothing related to xattrs, security.selinux, or file labels.
//
// The crate root re-exports do not mention anything SELinux-related. The
// `kattrs` module name is promising (kernel attributes?) but its rustdoc
// index page describes it as "High-Assurance Kernel Attribute Access" without
// mentioning xattrs or file contexts.
//
// The blog post says umrs-selinux is the SELinux crate. But the task asked
// me to do this in umrs-platform — the rustdoc gives NO cross-crate pointer
// from umrs-platform to umrs-selinux for this task.
//
// FINDING: umrs-platform's module-level doc should include a cross-crate
// pointer: "For reading file SELinux contexts (xattrs), use umrs-selinux."
// Without this, a first-day user will search every module in umrs-platform
// and give up confused.
//
// The correct call (from system_summary.rs) is:
//   use umrs_selinux::ls::{SecureDirent, ReadOptions};
//   let dirent = SecureDirent::open("/etc/shadow", &ReadOptions::default())?;
//   println!("{}", dirent.security_context());
//
// We cannot add umrs-selinux as a dependency in this example without editing
// Cargo.toml. We document the limitation and stub the section.
// ---------------------------------------------------------------------------

fn task3_selinux_context(c: &C) {
    header("Task 3 — SELinux Security Context of /etc/shadow", c);

    println!(
        "  {}NOTE:{} File SELinux context is not in umrs-platform.",
        c.cyan, c.reset
    );
    println!("  Use the `umrs-selinux` crate: umrs_selinux::ls::SecureDirent.");
    println!();
    println!("  There is no cross-crate pointer in umrs-platform rustdoc.");
    println!("  The kattrs module doc does not mention file xattrs or SELinux.");
    println!("  A new user will not discover this without reading source or examples.");
}

// ---------------------------------------------------------------------------
// Task 4: IMA/EVM status via posture catalog
//
// STUCK (BLOCKER): The task says to check IMA/EVM status. I expected an
// `IndicatorId::Ima` or `IndicatorId::Evm` variant. There is none.
//
// The full list of IndicatorId variants (from the rustdoc sidebar) is:
// BluetoothBlacklisted, CorePattern, DmesgRestrict, FipsEnabled,
// FirewireCoreBlacklisted, KexecLoadDisabled, KptrRestrict, L1tfOff,
// Lockdown, MdsOff, Mitigations, ModuleSigEnforce, ModulesDisabled,
// NfConntrackAcct, NoSmtOff, PerfEventParanoid, ProtectedFifos,
// ProtectedHardlinks, ProtectedRegular, ProtectedSymlinks, Pti,
// RandomTrustBootloader, RandomTrustCpu, RandomizeVaSpace, RetbleedOff,
// SpectreV2Off, SpectreV2UserOff, SrbdsOff, SuidDumpable, Sysrq,
// ThunderboltBlacklisted, TsxAsyncAbortOff, UnprivBpfDisabled,
// UnprivUsernsClone, UsbStorageBlacklisted, YamaPtraceScope.
//
// IMA and EVM are mentioned in the blog post as Phase 2 features, so their
// absence from the catalog is intentional. However, the posture module
// rustdoc does NOT explain this gap. A new user has no way to know whether
// IMA/EVM is missing because it's not implemented yet, or because they need
// to look in a different module.
//
// FINDING: The posture module-level doc should list planned-but-not-yet-
// implemented indicators with a note like "IMA/EVM indicators are planned
// for Phase 2 — they are not present in the current catalog."
//
// As the closest proxy, we check integrity-relevant indicators:
// FipsEnabled, ModuleSigEnforce, ModulesDisabled, Lockdown.
// ---------------------------------------------------------------------------

fn task4_posture_check(c: &C) {
    header("Task 4 — Kernel Integrity Posture (IMA/EVM proxy)", c);

    println!(
        "  {}NOTE:{} No IMA or EVM IndicatorId exists in the current catalog.",
        c.yellow, c.reset
    );
    println!("  Showing the closest proxy: integrity-relevant indicators.");
    println!();

    let snap = PostureSnapshot::collect();

    println!(
        "  Snapshot: {}/{} indicators meet desired value",
        snap.hardened_count(),
        snap.readable_count()
    );

    if let Some(boot) = &snap.boot_id {
        println!("  boot_id: {boot}");
    }

    println!();

    // Integrity-relevant indicators as a proxy for IMA/EVM readiness.
    let integrity_ids = [
        IndicatorId::ModuleSigEnforce,
        IndicatorId::ModulesDisabled,
        IndicatorId::Lockdown,
        IndicatorId::KexecLoadDisabled,
        IndicatorId::FipsEnabled,
    ];

    println!("  {:<30} {:<10} Status", "Indicator", "Live");
    println!("  {}", "-".repeat(60));

    for id in integrity_ids {
        // STUCK (mild): snap.get(id) returns Option<&IndicatorReport>. The
        // IndicatorReport struct is documented, but the doc for `live_value`
        // says it's `Option<LiveValue>` without explaining what LiveValue
        // displays as. I had to look at posture_demo.rs to see that
        // `live_value.as_ref().map_or_else(|| "<unavail>", |v| v.to_string())`
        // is the pattern. LiveValue should show a Display example in its docs.
        let Some(report) = snap.get(id) else {
            println!("  {:<30} {:<10} [not in snapshot]", id.label(), "N/A");
            continue;
        };

        let live =
            report.live_value.as_ref().map_or_else(|| "<unavail>".to_owned(), |v| v.to_string());

        let status = match report.meets_desired {
            Some(true) => format!("{}[hardened]{}", c.green, c.reset),
            Some(false) => format!("{}[FINDING] {}", c.red, c.reset),
            None => format!("{}[unread]  {}", c.yellow, c.reset),
        };

        println!("  {:<30} {:<10} {status}", id.label(), live);
    }

    println!();

    // Show any critical-impact findings for operator awareness.
    let mut found_critical = false;
    for report in snap.by_impact(AssuranceImpact::Critical) {
        if !found_critical {
            println!("  Critical-impact indicators:");
            found_critical = true;
        }
        let status = match report.meets_desired {
            Some(true) => format!("{}ok      {}", c.green, c.reset),
            Some(false) => format!("{}FINDING {}", c.red, c.reset),
            None => format!("{}unread  {}", c.yellow, c.reset),
        };
        println!("  {} {}", status, report.descriptor.label);
    }
    if !found_critical {
        println!("  (no critical-impact indicators in catalog)");
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let c = C::new();

    println!();
    println!(
        "{}{}umrs-platform system_check — first-day intern API walkthrough{}",
        c.bold, c.cyan, c.reset
    );
    println!("Tasks: OS identity | package check | SELinux context | IMA/EVM posture");

    task1_os_identity(&c);
    task2_package_check(&c);
    task3_selinux_context(&c);
    task4_posture_check(&c);

    println!();
    println!("{}Done.{}", c.bold, c.reset);
}
