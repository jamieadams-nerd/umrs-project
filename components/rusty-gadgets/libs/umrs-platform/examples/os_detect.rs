// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # os_detect — UMRS Platform OS Detection Walkthrough
//!
//! Runs the full OS detection pipeline and renders each phase of the result:
//! OS identity, label trust, confidence model, evidence chain, contradictions,
//! and a package-query demo.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p umrs-platform --example os_detect
//! RUST_LOG=debug cargo run -p umrs-platform --example os_detect
//! ```
//!
//! ## Compliance
//!
//! This example exercises the following controls for demonstration purposes:
//! - NIST SP 800-53 CM-8 — component inventory via substrate identity
//! - NIST SP 800-53 SI-7 — software integrity via label trust / T4 gate
//! - NIST SP 800-53 AU-3 — evidence chain display for audit record content

use std::io::IsTerminal;

use umrs_platform::detect::label_trust::LabelTrust;
use umrs_platform::detect::{DetectionError, OsDetector, is_installed};
use umrs_platform::evidence::{DigestAlgorithm, EvidenceRecord, SourceKind};
use umrs_platform::{ConfidenceModel, Distro, OsFamily, SubstrateIdentity, TrustLevel};

// ---------------------------------------------------------------------------
// ANSI colour helpers
// ---------------------------------------------------------------------------

struct Colours {
    reset: &'static str,
    bold: &'static str,
    cyan: &'static str,
    green: &'static str,
    yellow: &'static str,
    red: &'static str,
    dim: &'static str,
}

impl Colours {
    fn enabled() -> Self {
        Self {
            reset: "\x1b[0m",
            bold: "\x1b[1m",
            cyan: "\x1b[36m",
            green: "\x1b[32m",
            yellow: "\x1b[33m",
            red: "\x1b[31m",
            dim: "\x1b[2m",
        }
    }

    fn disabled() -> Self {
        Self {
            reset: "",
            bold: "",
            cyan: "",
            green: "",
            yellow: "",
            red: "",
            dim: "",
        }
    }
}

// ---------------------------------------------------------------------------
// Hex formatting helper
// ---------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// ---------------------------------------------------------------------------
// Section header
// ---------------------------------------------------------------------------

fn section(title: &str, c: &Colours) {
    println!();
    println!("{}{}━━━  {}  ━━━{}", c.bold, c.cyan, title, c.reset);
}

// ---------------------------------------------------------------------------
// Display helpers for enums
// ---------------------------------------------------------------------------

fn trust_level_label(level: TrustLevel) -> &'static str {
    match level {
        TrustLevel::Untrusted => "T0 — Untrusted",
        TrustLevel::KernelAnchored => "T1 — KernelAnchored",
        TrustLevel::EnvAnchored => "T2 — EnvAnchored",
        TrustLevel::SubstrateAnchored => "T3 — SubstrateAnchored",
        TrustLevel::IntegrityAnchored => "T4 — IntegrityAnchored",
    }
}

fn trust_level_description(level: TrustLevel) -> &'static str {
    match level {
        TrustLevel::Untrusted => {
            "No kernel anchor established. No claim can be made about the platform."
        }
        TrustLevel::KernelAnchored => "procfs verified via PROC_SUPER_MAGIC + PID coherence gate.",
        TrustLevel::EnvAnchored => {
            "Mount topology cross-checked (mountinfo vs statfs). Execution environment known."
        }
        TrustLevel::SubstrateAnchored => {
            "Package substrate parsed; identity derived from >= 2 independent facts."
        }
        TrustLevel::IntegrityAnchored => {
            "os-release ownership + installed digest verified against package DB."
        }
    }
}

fn source_kind_label(kind: &SourceKind) -> &'static str {
    match kind {
        SourceKind::Procfs => "procfs",
        SourceKind::RegularFile => "regular-file",
        SourceKind::PackageDb => "package-db",
        SourceKind::SymlinkTarget => "symlink-target",
        SourceKind::SysfsNode => "sysfs",
        SourceKind::StatfsResult => "statfs",
    }
}

fn distro_label(distro: &Distro) -> String {
    match distro {
        Distro::Rhel => "RHEL".to_owned(),
        Distro::Fedora => "Fedora".to_owned(),
        Distro::CentOs => "CentOS".to_owned(),
        Distro::AlmaLinux => "AlmaLinux".to_owned(),
        Distro::RockyLinux => "Rocky Linux".to_owned(),
        Distro::Debian => "Debian".to_owned(),
        Distro::Ubuntu => "Ubuntu".to_owned(),
        Distro::Kali => "Kali Linux".to_owned(),
        Distro::Other(s) => s.clone(),
    }
}

fn family_label(family: &OsFamily) -> &'static str {
    match family {
        OsFamily::RpmBased => "RPM-based",
        OsFamily::DpkgBased => "dpkg-based",
        OsFamily::PacmanBased => "pacman-based",
        OsFamily::Unknown => "unknown",
    }
}

// ---------------------------------------------------------------------------
// Rendering sections
// ---------------------------------------------------------------------------

fn print_header(c: &Colours) {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "\n{}{}  umrs-platform OS Detection  v{}{}",
        c.bold, c.cyan, version, c.reset
    );
    println!(
        "{}  Exercises the full detection pipeline and renders each phase.{}",
        c.dim, c.reset
    );
}

fn print_os_identity(result: &umrs_platform::detect::DetectionResult, c: &Colours) {
    section("OS Identity", c);

    match &result.os_release {
        None => {
            println!("  {}os-release: not available{}", c.yellow, c.reset);
        }
        Some(rel) => {
            println!("  {}ID           :{} {}", c.cyan, c.reset, rel.id.as_str());
            println!(
                "  {}NAME         :{} {}",
                c.cyan,
                c.reset,
                rel.name.as_str()
            );
            if let Some(ver) = &rel.version_id {
                println!("  {}VERSION_ID   :{} {}", c.cyan, c.reset, ver.as_str());
            }
            if let Some(pn) = &rel.pretty_name {
                println!("  {}PRETTY_NAME  :{} {}", c.cyan, c.reset, pn.as_str());
            }
            if let Some(cpe) = &rel.cpe_name {
                println!("  {}CPE_NAME     :{} {}", c.cyan, c.reset, cpe.as_str());
            }
        }
    }

    match &result.substrate_identity {
        None => {
            println!("  {}substrate-identity: not available{}", c.yellow, c.reset);
        }
        Some(sub) => {
            println!();
            println!(
                "  {}Substrate Identity{}  (package-DB derived, independent of os-release)",
                c.bold, c.reset
            );
            print_substrate_identity(sub, c);
        }
    }

    if let Some(boot) = &result.boot_id {
        println!();
        println!("  {}boot-id      :{} {}", c.cyan, c.reset, boot);
    }
}

fn print_substrate_identity(sub: &SubstrateIdentity, c: &Colours) {
    println!(
        "  {}family       :{} {}",
        c.cyan,
        c.reset,
        family_label(&sub.family)
    );
    if let Some(distro) = &sub.distro {
        println!(
            "  {}distro       :{} {}",
            c.cyan,
            c.reset,
            distro_label(distro)
        );
    }
    if let Some(ver) = &sub.version_id {
        println!("  {}version_id   :{} {}", c.cyan, c.reset, ver);
    }
    println!("  {}facts_count  :{} {}", c.cyan, c.reset, sub.facts_count);
    println!("  {}probe_used   :{} {}", c.cyan, c.reset, sub.probe_used);
}

fn print_trust_summary(label_trust: &LabelTrust, c: &Colours) {
    section("Trust Summary", c);

    match label_trust {
        LabelTrust::UntrustedLabelCandidate => {
            println!(
                "  {}UNTRUSTED CANDIDATE{}  — permissions failed or file is unowned.",
                c.red, c.reset
            );
            println!("  The label must NOT be used for policy decisions.");
        }
        LabelTrust::LabelClaim => {
            println!(
                "  {}LABEL CLAIM{}  — structurally valid; integrity unconfirmed.",
                c.yellow, c.reset
            );
            println!("  Package substrate was not probed (T3 not reached), or digest unavailable.");
            println!("  Usable for display only; do not rely on it for policy.");
        }
        LabelTrust::TrustedLabel => {
            println!(
                "  {}TRUSTED LABEL{}  — T4: ownership + digest verified against package DB.",
                c.green, c.reset
            );
            println!("  Label content corroborates substrate-derived identity.");
            println!("  Safe for policy decisions (NIST SP 800-53 SI-7, CMMC L2 SI.1.210).");
        }
        LabelTrust::IntegrityVerifiedButContradictory {
            contradiction,
        } => {
            println!(
                "  {}INTEGRITY VERIFIED BUT CONTRADICTORY{}  — anomalous.",
                c.red, c.reset
            );
            println!(
                "  Digest matched the package DB, but the label contradicts substrate identity."
            );
            // Truncate to <= 64 chars before display — NIST SP 800-53 SI-12.
            let display: String = contradiction.chars().take(64).collect();
            println!("  Contradiction: {}{}{}", c.yellow, display, c.reset);
            println!("  Must NOT be used for policy decisions.");
        }
    }
}

fn print_confidence(confidence: &ConfidenceModel, c: &Colours) {
    section("Confidence Model", c);

    let level = confidence.level();
    let colour = match level {
        TrustLevel::Untrusted => c.red,
        TrustLevel::KernelAnchored => c.yellow,
        TrustLevel::EnvAnchored => c.yellow,
        TrustLevel::SubstrateAnchored => c.green,
        TrustLevel::IntegrityAnchored => c.green,
    };
    println!(
        "  {}Current level:{} {}{}{} — {}",
        c.cyan,
        c.reset,
        colour,
        trust_level_label(level),
        c.reset,
        trust_level_description(level)
    );

    if confidence.downgrade_reasons.is_empty() {
        println!("  {}No downgrade reasons recorded.{}", c.dim, c.reset);
    } else {
        println!();
        println!("  {}Downgrade reasons:{}", c.bold, c.reset);
        for (i, reason) in confidence.downgrade_reasons.iter().enumerate() {
            println!("    [{}] {}", i.saturating_add(1), reason);
        }
    }
}

fn print_evidence_chain(records: &[EvidenceRecord], c: &Colours) {
    section("Evidence Chain", c);
    println!(
        "  {} record(s) collected during this detection run.",
        records.len()
    );

    for (i, rec) in records.iter().enumerate() {
        let idx = i.saturating_add(1);
        println!();
        println!(
            "  {}[{:02}] {}{}{}",
            c.dim, idx, c.reset, c.bold, rec.path_requested
        );
        println!(
            "{}        kind={} fd={}  parse_ok={}{}",
            c.reset,
            source_kind_label(&rec.source_kind),
            rec.opened_by_fd,
            if rec.parse_ok {
                "yes"
            } else {
                "NO"
            },
            c.reset,
        );

        if let Some(resolved) = &rec.path_resolved {
            println!("        resolved -> {}", resolved);
        }

        if let Some(magic) = rec.fs_magic {
            println!("        fs_magic = 0x{:08x}", magic);
        }

        if let Some(stat) = &rec.stat {
            let ino = stat.ino.map_or_else(|| "-".to_owned(), |v| v.to_string());
            let size = stat.size.map_or_else(|| "-".to_owned(), |v| v.to_string());
            let uid = stat.uid.map_or_else(|| "-".to_owned(), |v| v.to_string());
            println!("        stat: ino={ino}  size={size}  uid={uid}");
        }

        if let Some(sha) = &rec.sha256 {
            println!("        sha256 = {}", hex(sha));
        }

        if let Some(pkg) = &rec.pkg_digest {
            let alg = match &pkg.algorithm {
                DigestAlgorithm::Sha256 => "sha256".to_owned(),
                DigestAlgorithm::Sha512 => "sha512".to_owned(),
                DigestAlgorithm::Md5 => "md5 (WEAK)".to_owned(),
                DigestAlgorithm::Unknown(s) => format!("unknown({})", s),
            };
            println!("        pkg_digest: alg={}  value={}", alg, hex(&pkg.value));
        }

        for note in &rec.notes {
            println!("        {}note: {}{}", c.dim, note, c.reset);
        }
    }
}

fn print_contradictions(confidence: &ConfidenceModel, c: &Colours) {
    section("Contradictions", c);

    if confidence.contradictions.is_empty() {
        println!("  {}No contradictions recorded.{}", c.green, c.reset);
        return;
    }

    println!(
        "  {}{} contradiction(s) recorded:{}",
        c.red,
        confidence.contradictions.len(),
        c.reset
    );
    for (i, con) in confidence.contradictions.iter().enumerate() {
        let idx = i.saturating_add(1);
        println!();
        println!("  [{}] {} vs {}", idx, con.source_a, con.source_b);
        // Truncate description — NIST SP 800-53 SI-12.
        let display: String = con.description.chars().take(64).collect();
        println!("      {}", display);
    }
}

fn print_package_query_demo(c: &Colours) {
    section("Package Query Demo", c);
    println!("  {}is_installed(\"bash\"):{}", c.cyan, c.reset);
    print_pkg_result("bash", c);
    println!();
    println!("  {}is_installed(\"openssl\"):{}", c.cyan, c.reset);
    print_pkg_result("openssl", c);
    println!();
    println!("  {}is_installed(\"nonexistent-pkg\"):{}", c.cyan, c.reset);
    print_pkg_result("nonexistent-pkg", c);
    println!();
    println!(
        "  {}Note:{} is_installed() returns Ok(true/false) when the DB is readable.",
        c.dim, c.reset
    );
    println!("  It returns Err(DatabaseUnavailable) when the RPM DB cannot be opened.");
}

fn print_pkg_result(name: &str, c: &Colours) {
    match is_installed(name) {
        Ok(true) => {
            println!(
                "    {}installed{} — package '{}' found in the RPM database.",
                c.green, c.reset, name
            );
        }
        Ok(false) => {
            println!(
                "    {}not installed{} — package '{}' not found in the RPM database.",
                c.yellow, c.reset, name
            );
        }
        Err(e) => {
            println!("    {}query error{} — {e}", c.red, c.reset);
        }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    env_logger::init();

    let use_colour = std::io::stdout().is_terminal();
    let c = if use_colour {
        Colours::enabled()
    } else {
        Colours::disabled()
    };

    print_header(&c);

    let result = match OsDetector::default().detect() {
        Ok(r) => r,
        Err(e) => {
            match e {
                DetectionError::ProcfsNotReal => {
                    eprintln!(
                        "{}HARD GATE FAILURE:{} procfs is not real procfs — \
                         cannot establish kernel anchor.",
                        c.red, c.reset
                    );
                }
                DetectionError::PidCoherenceFailed {
                    syscall,
                    procfs,
                } => {
                    eprintln!(
                        "{}HARD GATE FAILURE:{} PID coherence broken \
                         (syscall={syscall}, procfs={procfs}).",
                        c.red, c.reset
                    );
                }
                DetectionError::KernelAnchorIo(ref io_err) => {
                    // Do not include io_err display in the user-visible message
                    // if it could contain sensitive path info; log it instead.
                    log::error!("kernel anchor I/O failure: {io_err}");
                    eprintln!(
                        "{}HARD GATE FAILURE:{} I/O error during kernel anchor phase.",
                        c.red, c.reset
                    );
                }
            }
            return;
        }
    };

    print_os_identity(&result, &c);
    print_trust_summary(&result.label_trust, &c);
    print_confidence(&result.confidence, &c);
    print_evidence_chain(result.evidence.records(), &c);
    print_contradictions(&result.confidence, &c);
    print_package_query_demo(&c);

    println!();
    println!("{}Done.{}", c.dim, c.reset);
    println!();
}
