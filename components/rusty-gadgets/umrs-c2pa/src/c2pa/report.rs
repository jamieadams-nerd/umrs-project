// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA Report Rendering
//!
//! Renders chain-of-custody and configuration validation results to stdout as
//! operator-readable reports. All output is plain text with Unicode box-drawing
//! characters; no ANSI color codes are used, so output is safe in any terminal
//! and in piped contexts.
//!
//! ## Key Exported Functions
//!
//! - [`print_chain`] — full chain-of-custody report after ingest
//! - [`print_chain_readonly`] — chain report for read-only inspection
//! - [`print_validation_report`] — config preflight check results
//!
//! ## Design
//!
//! Report functions accept slices of structured data types (`ChainEntry`,
//! `ValidationResult`) and render them deterministically. They do not perform
//! any I/O or validation themselves — callers own the data pipeline.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — reports surface all
//!   fields required for forensic review: signer, issuer, timestamp, algorithm,
//!   trust status, and optional security marking.
//! - **NIST SP 800-53 SI-11**: Error Handling — the report layer does not emit
//!   key material, credential paths, or classified data in any output field.

use crate::c2pa::{
    ingest::IngestResult,
    manifest::{ChainEntry, TrustStatus},
};

const SEPARATOR: &str = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━";
const THIN_SEP: &str = "────────────────────────────────────────────────────────";

/// Print the full chain-of-custody report to stdout.
///
/// Shows every entry in the chain with trust indicators, then a hash
/// consistency summary based on the ingest result.
pub fn print_chain(path: &str, sha256: &str, chain: &[ChainEntry], ingest: Option<&IngestResult>) {
    println!("\nChain of Custody — {path}");
    println!("SHA-256: {sha256}");
    println!("{SEPARATOR}");

    if chain.is_empty() {
        println!("  (no C2PA manifest found)");
    } else {
        // Calculate the maximum rendered width of all trust status tags in this
        // chain so column alignment adapts to the actual content. French
        // translations of status labels will be longer than English equivalents;
        // hardcoded constants would break the column layout.
        //
        // The tag format is "[STATUS]" for normal entries and "*[STATUS]" for
        // footnoted entries. Both width cases are measured; we use the larger.
        let max_tag_width = chain
            .iter()
            .map(|e| {
                let base = format!("[{}]", e.trust_status).len();
                // Footnoted entries prepend "*" — measure that variant too.
                let footnoted = base + 1;
                footnoted.max(base)
            })
            .max()
            .unwrap_or(14);
        // Reserve one extra column for the asterisk prefix on footnoted entries
        // (the asterisk replaces the leading space, so the total field width stays
        // consistent whether or not an entry is footnoted).
        let pad = max_tag_width + 2;

        // Collect unique footnotes keyed by their display label so identical
        // statuses across entries produce a single footnote line.
        let mut footnote_set: std::collections::BTreeMap<String, &str> =
            std::collections::BTreeMap::new();

        for (i, entry) in chain.iter().enumerate() {
            let idx = i + 1;

            // Detect self-signed: issuer == signer (cert signs itself).
            let is_self_signed = entry.signer_name == entry.issuer && entry.issuer != "Unknown";

            // Build the trust tag with asterisk for entries that need a footnote.
            let trust_tag = match (&entry.trust_status, is_self_signed) {
                (TrustStatus::Untrusted | TrustStatus::NoTrustList, true) => {
                    let label = format!("{}", entry.trust_status);
                    footnote_set
                        .entry(label)
                        .or_insert("Self-signed certificate — not issued by a trusted CA");
                    format!("*[{}]", entry.trust_status)
                }
                (TrustStatus::NoTrustList, false) => {
                    let label = format!("{}", entry.trust_status);
                    footnote_set
                        .entry(label)
                        .or_insert("No trust list configured — trust could not be evaluated");
                    format!("*[{}]", entry.trust_status)
                }
                (TrustStatus::Untrusted, false) => {
                    let label = format!("{}", entry.trust_status);
                    footnote_set
                        .entry(label)
                        .or_insert(
                            "Signature is valid but the signer's CA is not in your trust list. \
                             This does not mean the file is untrustworthy — the signer may use \
                             a CA that is not yet in the C2PA official trust list (e.g., OpenAI). \
                             To resolve, add the signer's root CA to trust_anchors or allowed_list.",
                        );
                    format!("*[{}]", entry.trust_status)
                }
                _ => format!("[{}]", entry.trust_status),
            };

            println!("  {:<3} {:<pad$}  {}", idx, trust_tag, entry.signer_name);

            match &entry.signed_at {
                Some(ts) => println!("       {:<pad$}  Signed at : {} UTC", "", ts),
                None => println!("       {:<pad$}  Signed at : no timestamp provided", ""),
            }

            // Only show Issuer if it differs from the top-level signer name.
            if entry.issuer != entry.signer_name {
                println!("       {:<pad$}  Issuer    : {}", "", entry.issuer);
            }

            println!("       {:<pad$}  Alg       : {}", "", entry.algorithm);

            // Generator + version (e.g. "ChatGPT 0.67.1")
            let gen_display = match &entry.generator_version {
                Some(v) => format!("{} {v}", entry.generator),
                None => entry.generator.clone(),
            };
            println!("       {:<pad$}  Generator : {}", "", gen_display);

            // Security label / marking, if present.
            if let Some(label) = &entry.security_label {
                println!("       {:<pad$}  Marking   : {}", "", label);
            }
            println!();
        }

        // Print deduplicated footnotes keyed by trust status label.
        if !footnote_set.is_empty() {
            println!("{THIN_SEP}");
            for (label, explanation) in &footnote_set {
                println!("  *[{label}] {explanation}");
            }
        }
    }

    println!("{SEPARATOR}");

    // Hash consistency line.
    if let Some(result) = ingest {
        if result.had_manifest {
            // We have a chain — all hashes should be consistent.
            println!("Hash consistency : PASS — file unchanged across all signing events");
        } else {
            println!("Hash consistency : N/A  — no prior manifest (first signature)");
        }
        println!("UMRS action      : {}", result.action);
        println!("UMRS output      : {}", result.output_path.display());
        if result.is_ephemeral {
            println!("UMRS identity    : ephemeral self-signed cert (test mode — UNTRUSTED)");
        }
    }

    println!();
}

/// Print the result of a read-only chain inspection (no ingest).
pub fn print_chain_readonly(path: &str, sha256: &str, chain: &[ChainEntry]) {
    print_chain(path, sha256, chain, None);
}

/// Print the config validation report.
pub fn print_validation_report(results: &[crate::c2pa::validate::ValidationResult]) {
    use crate::c2pa::validate::CheckStatus;

    println!();
    for r in results {
        let tag = match r.status {
            CheckStatus::Pass => "[OK]  ",
            CheckStatus::Warn => "[WARN]",
            CheckStatus::Fail => "[FAIL]",
            CheckStatus::Info => "[INFO]",
            CheckStatus::Skip => "[SKIP]",
        };
        println!("  {}  {}: {}", tag, r.check, r.message);
    }
    println!("{THIN_SEP}");

    let failures = results.iter().filter(|r| r.status == CheckStatus::Fail).count();
    let warnings = results.iter().filter(|r| r.status == CheckStatus::Warn).count();

    if failures == 0 {
        if warnings > 0 {
            println!("  All checks passed ({warnings} warning(s)). Configuration is ready.");
        } else {
            println!("  All checks passed. Configuration is ready.");
        }
    } else {
        println!("  {failures} check(s) failed. Configuration is NOT ready.");
    }
    println!();
}
