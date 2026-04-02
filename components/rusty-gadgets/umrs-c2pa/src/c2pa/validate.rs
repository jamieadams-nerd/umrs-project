// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA Configuration Preflight Validation
//!
//! Runs preflight checks against a loaded [`UmrsConfig`] to verify that
//! signing credentials, algorithms, trust lists, and operational parameters
//! are correctly configured before any signing or inspection operation.
//!
//! ## Key Exported Types
//!
//! - [`CheckStatus`] — pass/fail/warn/info/skip status for a single check
//! - [`ValidationResult`] — result of a single preflight check
//! - [`validate_config`] — run all preflight checks; returns one result per check
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-6**: Configuration Settings — all operational
//!   parameters (algorithm, credential paths, trust list paths, log level)
//!   are verified against expected values before use.
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — the algorithm check
//!   verifies that the configured algorithm is in the FIPS-safe allow-list;
//!   a `CheckStatus::Fail` on the algorithm check prevents signing.
//! - **NIST SP 800-53 AC-3**: Access Enforcement — private key permission
//!   checks detect world-readable or group-readable key files and surface them
//!   as `CheckStatus::Warn` results so operators can correct them.
//! - **NSA RTB Fail Secure**: a `CheckStatus::Fail` on any required check
//!   causes the caller to exit non-zero, preventing operation with an invalid
//!   configuration.

use crate::c2pa::{config::UmrsConfig, signer::ALLOWED_ALGORITHMS};
#[allow(unused_imports)]
use crate::verbose;

/// Status of a single preflight check.
///
/// `Fail` drives the process exit code in `cmd_config_validate` — any
/// `Fail` result causes exit(1), preventing operation with an invalid
/// configuration. This is the NSA RTB Fail Secure property for preflight.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation — `Fail` status
///   is the structured representation of an invalid configuration value.
/// - **NIST SP 800-53 SC-13**: the `Fail` variant on the algorithm check
///   is the enforcement point for the FIPS algorithm allow-list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
    Info,
    Skip,
}

/// Result of a single preflight check.
///
/// Carries the check name, status, and a human-readable message. Callers
/// should inspect `status` programmatically; the `message` field is for
/// operator display only.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: structured result type enables programmatic
///   filtering and counting of failures before any operation proceeds.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub check: String,
    pub status: CheckStatus,
    pub message: String,
}

impl ValidationResult {
    fn pass(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            status: CheckStatus::Pass,
            message: msg.into(),
        }
    }
    fn warn(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            status: CheckStatus::Warn,
            message: msg.into(),
        }
    }
    fn fail(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            status: CheckStatus::Fail,
            message: msg.into(),
        }
    }
    fn info(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            status: CheckStatus::Info,
            message: msg.into(),
        }
    }
    fn skip(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            status: CheckStatus::Skip,
            message: msg.into(),
        }
    }
}

/// Run all preflight checks against the loaded configuration.
///
/// Returns a list of results — one per check. The caller must inspect the
/// results: any `CheckStatus::Fail` means the configuration is invalid and
/// the tool should not proceed. This function is the primary operator
/// preflight gate before any signing or inspection operation.
///
/// ## Compliance
///
/// - **NIST SP 800-53 CM-6**: all configuration values are checked.
/// - **NSA RTB Fail Secure**: fail results must cause exit(1) in callers.
#[must_use = "Preflight results must be inspected; discarding them means a \
              misconfigured FIPS algorithm, missing trust anchors, or \
              world-readable private key would go unreported"]
pub fn validate_config(config: &UmrsConfig) -> Vec<ValidationResult> {
    let mut results = Vec::new();

    // Identity: required fields.
    verbose!("Checking required identity fields...");
    check_required_fields(config, &mut results);

    // Cert + key file checks (includes permission checks on key file).
    verbose!("Checking certificate and key files...");
    let cert_ok = check_cert_file(config, &mut results);
    let key_ok = check_key_file(config, &mut results);

    // Key/cert pair match — only if both files are readable.
    if cert_ok && key_ok {
        check_key_cert_match(config, &mut results);
    } else if config.identity.cert_chain.is_some() || config.identity.private_key.is_some() {
        results.push(ValidationResult::skip(
            "key_cert_match",
            "Skipped — requires both cert and key files to be readable",
        ));
    }

    // Algorithm.
    verbose!("Checking signing algorithm...");
    check_algorithm(config, &mut results);

    // TSA reachability.
    verbose!("Checking TSA configuration...");
    check_tsa(config, &mut results);

    // Trust list checks (includes world-writable checks on trust anchor files).
    verbose!("Checking trust list configuration...");
    check_trust_config(config, &mut results);

    // Ephemeral mode notice.
    if config.identity.cert_chain.is_none() && config.identity.private_key.is_none() {
        results.push(ValidationResult::info(
            "credential_mode",
            "No certificate configured — ephemeral self-signed cert will be used (test mode). \
             Manifests will be marked UNTRUSTED by external validators.",
        ));
    }

    results
}

fn check_required_fields(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    if config.identity.claim_generator.is_empty() {
        out.push(ValidationResult::fail("claim_generator", "Field is empty"));
    } else {
        out.push(ValidationResult::pass(
            "claim_generator",
            &format!("\"{}\"", config.identity.claim_generator),
        ));
    }
}

fn check_cert_file(config: &UmrsConfig, out: &mut Vec<ValidationResult>) -> bool {
    let Some(path) = &config.identity.cert_chain else {
        return false;
    };
    if !path.exists() {
        out.push(ValidationResult::fail(
            "cert_chain",
            &format!("File not found: {}", path.display()),
        ));
        return false;
    }
    match std::fs::read(path) {
        Err(e) => {
            out.push(ValidationResult::fail("cert_chain", &format!("Cannot read: {e}")));
            false
        }
        Ok(bytes) => {
            if is_valid_pem(&bytes) {
                out.push(ValidationResult::pass(
                    "cert_chain",
                    &format!("Valid PEM at {}", path.display()),
                ));
                true
            } else {
                out.push(ValidationResult::fail(
                    "cert_chain",
                    &format!("File is not valid PEM: {}", path.display()),
                ));
                false
            }
        }
    }
}

fn check_key_file(config: &UmrsConfig, out: &mut Vec<ValidationResult>) -> bool {
    let Some(path) = &config.identity.private_key else {
        return false;
    };
    if !path.exists() {
        out.push(ValidationResult::fail(
            "private_key",
            &format!("File not found: {}", path.display()),
        ));
        return false;
    }

    // Unix permission check — private key must not be world-readable.
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        match std::fs::metadata(path) {
            Ok(meta) => {
                let mode = meta.mode() & 0o777;
                if mode & 0o077 != 0 {
                    // This is a FAIL — a world-readable private key is a hard error.
                    out.push(ValidationResult::fail(
                        "key_permissions",
                        &format!(
                            "Private key has unsafe permissions {:04o} — expected 0600 or 0400. \
                             Run: chmod 0600 {}",
                            mode,
                            path.display()
                        ),
                    ));
                    log::warn!(
                        target: "umrs",
                        "private key has unsafe permissions mode={:04o} file={}",
                        mode,
                        path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default()
                    );
                } else {
                    out.push(ValidationResult::pass(
                        "key_permissions",
                        &format!("Permissions {mode:04o} (secure)"),
                    ));
                }
                // Check owner matches effective uid.
                // Read euid from /proc/self/status to avoid unsafe code.
                if let Some(euid) = read_euid_from_proc()
                    && meta.uid() != euid
                {
                    out.push(ValidationResult::warn(
                        "key_owner",
                        &format!(
                            "Private key owner uid={} does not match process euid={}",
                            meta.uid(),
                            euid
                        ),
                    ));
                }
            }
            Err(e) => {
                out.push(ValidationResult::warn(
                    "key_permissions",
                    &format!("Cannot stat key file: {e}"),
                ));
            }
        }
    }

    match std::fs::read(path) {
        Err(e) => {
            out.push(ValidationResult::fail("private_key", &format!("Cannot read: {e}")));
            false
        }
        Ok(bytes) => {
            if is_valid_pem(&bytes) {
                out.push(ValidationResult::pass(
                    "private_key",
                    &format!("Valid PEM at {}", path.display()),
                ));
                true
            } else {
                out.push(ValidationResult::fail(
                    "private_key",
                    &format!("File is not valid PEM: {}", path.display()),
                ));
                false
            }
        }
    }
}

fn check_key_cert_match(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    // Delegate actual key/cert matching to the signer builder — if it succeeds, they match.
    match crate::c2pa::signer::resolve_signer_mode(&config.identity, None)
        .and_then(|mode| crate::c2pa::signer::build_signer(&mode).map(|_| ()))
    {
        Ok(()) => {
            out.push(ValidationResult::pass("key_cert_match", "Private key matches certificate"));
        }
        Err(e) => out.push(ValidationResult::fail("key_cert_match", &e.to_string())),
    }
}

fn check_algorithm(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    let alg = &config.identity.algorithm;
    if alg == "ed25519" {
        out.push(ValidationResult::warn(
            "algorithm",
            "ed25519 is not reliably available on FIPS-enabled systems. \
             Recommended: es256, es384, or es512.",
        ));
    } else if ALLOWED_ALGORITHMS.contains(&alg.as_str()) {
        let desc = crate::c2pa::signer::describe_algorithm(alg);
        out.push(ValidationResult::pass("algorithm", desc));
    } else {
        out.push(ValidationResult::fail(
            "algorithm",
            &format!("'{alg}' is not allowed. Use one of: {}", ALLOWED_ALGORITHMS.join(", ")),
        ));
    }
}

fn check_tsa(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    let Some(url) = &config.timestamp.tsa_url else {
        return;
    };

    #[cfg(feature = "internet")]
    match ureq::head(url).call() {
        Ok(_) => out.push(ValidationResult::pass(
            "tsa_reachable",
            &format!("TSA endpoint reachable: {url}"),
        )),
        Err(e) => out.push(ValidationResult::warn(
            "tsa_reachable",
            &format!("TSA endpoint did not respond: {url} ({e})"),
        )),
    }

    #[cfg(not(feature = "internet"))]
    out.push(ValidationResult::warn(
        "tsa_reachable",
        &format!(
            "TSA configured ({url}) but network feature is disabled — timestamps will be unsigned"
        ),
    ));
}

fn check_trust_config(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    if !config.has_trust_config() {
        out.push(ValidationResult::info(
            "trust_config",
            "No trust lists configured — all manifests will show NO TRUST LIST. \
             See docs/trust-maintenance.md to set up trust anchors.",
        ));
        return;
    }

    // Check each trust file.
    if let Some(path) = &config.trust.trust_anchors {
        check_pem_file("trust_anchors", path, out);
        // Trust anchor files must not be world-writable — anyone who can modify
        // them can inject arbitrary CA certificates and subvert trust validation.
        #[cfg(unix)]
        check_trust_file_permissions("trust_anchors", path, out);
    }
    if let Some(path) = &config.trust.user_anchors {
        check_pem_file("user_anchors", path, out);
        #[cfg(unix)]
        check_trust_file_permissions("user_anchors", path, out);
    }
    if let Some(path) = &config.trust.allowed_list {
        if path.exists() {
            out.push(ValidationResult::pass(
                "allowed_list",
                &format!("File exists: {}", path.display()),
            ));
        } else {
            out.push(ValidationResult::fail(
                "allowed_list",
                &format!("File not found: {}", path.display()),
            ));
        }
        #[cfg(unix)]
        check_trust_file_permissions("allowed_list", path, out);
    }
    if let Some(path) = &config.trust.trust_config {
        if path.exists() {
            out.push(ValidationResult::pass(
                "trust_eku_config",
                &format!("EKU config found: {}", path.display()),
            ));
        } else {
            out.push(ValidationResult::fail(
                "trust_eku_config",
                &format!("File not found: {}", path.display()),
            ));
        }
    }

    if let Some(url) = &config.trust.ocsp_responder {
        out.push(ValidationResult::info(
            "ocsp_responder",
            &format!("OCSP responder configured: {url} (not yet implemented — skeleton only)"),
        ));
    }
}

fn check_pem_file(name: &str, path: &std::path::Path, out: &mut Vec<ValidationResult>) {
    if !path.exists() {
        out.push(ValidationResult::fail(name, &format!("File not found: {}", path.display())));
        return;
    }
    match std::fs::read(path) {
        Err(e) => {
            out.push(ValidationResult::fail(name, &format!("Cannot read: {e}")));
        }
        Ok(bytes) => {
            if is_valid_pem(&bytes) {
                let cert_count =
                    bytes.windows(17).filter(|w| w == b"-----BEGIN CERT-").count().max(
                        // Fallback: count full BEGIN CERTIFICATE markers
                        String::from_utf8_lossy(&bytes)
                            .matches("-----BEGIN CERTIFICATE-----")
                            .count(),
                    );
                out.push(ValidationResult::pass(
                    name,
                    &format!("Valid PEM at {} ({} certificate(s))", path.display(), cert_count),
                ));
            } else {
                out.push(ValidationResult::fail(
                    name,
                    &format!("File is not valid PEM: {}", path.display()),
                ));
            }
        }
    }
}

/// Check that a trust anchor file is not world-writable.
///
/// Trust anchor files are the root of trust for all C2PA signature validation.
/// A world-writable trust anchor file allows any local user to inject arbitrary
/// CA certificates and bypass signature trust decisions.
///
/// This emits a `Warn` rather than a `Fail` because the file may still be
/// functionally usable, but the operator must be informed.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: Integrity — trust anchor files must not be
///   modifiable by untrusted principals.
/// - **NIST SP 800-53 CM-5**: Access Restrictions for Change — world-writable
///   trust files violate change access control requirements.
#[cfg(unix)]
fn check_trust_file_permissions(
    name: &str,
    path: &std::path::Path,
    out: &mut Vec<ValidationResult>,
) {
    use std::os::unix::fs::MetadataExt;
    match std::fs::metadata(path) {
        Err(_) => {} // existence already checked by check_pem_file
        Ok(meta) => {
            let mode = meta.mode() & 0o777;
            if mode & 0o002 != 0 {
                // World-writable — anyone can inject CA certificates.
                out.push(ValidationResult::warn(
                    &format!("{name}_permissions"),
                    &format!(
                        "Trust file is world-writable (mode {:04o}) — \
                         any local user could inject CA certificates. \
                         Run: chmod o-w {}",
                        mode,
                        path.display()
                    ),
                ));
                log::warn!(
                    target: "umrs",
                    "trust file is world-writable mode={:04o} file={}",
                    mode,
                    path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default()
                );
            }
        }
    }
}

/// Read the effective user ID from `/proc/self/status` without unsafe code.
///
/// The `Uid:` line has the form: `Uid:\t<real> <effective> <saved> <fs>`
/// Returns `None` if the file cannot be read or the line is not found.
#[cfg(unix)]
fn read_euid_from_proc() -> Option<u32> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("Uid:") {
            let mut fields = rest.split_whitespace();
            let _real = fields.next()?;
            let effective = fields.next()?;
            return effective.parse::<u32>().ok();
        }
    }
    None
}

/// Naively check whether bytes look like PEM (contains "-----BEGIN").
fn is_valid_pem(bytes: &[u8]) -> bool {
    bytes.windows(11).any(|w| w == b"-----BEGIN ")
}
