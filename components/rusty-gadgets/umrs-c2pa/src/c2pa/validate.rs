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

use gettextrs::ngettext;
use umrs_core::i18n;

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
            &i18n::tr("Skipped — requires both cert and key files to be readable"),
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
            &i18n::tr(
                "No certificate configured — ephemeral self-signed cert will be used (test mode). \
                 Manifests will be marked UNTRUSTED by external validators.",
            ),
        ));
    }

    results
}

fn check_required_fields(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    if config.identity.claim_generator.is_empty() {
        out.push(ValidationResult::fail(
            "claim_generator",
            &i18n::tr("Field is empty"),
        ));
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
    match std::fs::read(path) {
        Err(e) => {
            let msg = if e.kind() == std::io::ErrorKind::NotFound {
                format!("{} {}", i18n::tr("File not found:"), path.display())
            } else {
                format!("{} {e}", i18n::tr("Cannot read:"))
            };
            out.push(ValidationResult::fail("cert_chain", &msg));
            false
        }
        Ok(bytes) => {
            if is_valid_pem(&bytes) {
                out.push(ValidationResult::pass(
                    "cert_chain",
                    &format!("{} {}", i18n::tr("Valid PEM at"), path.display()),
                ));
                true
            } else {
                out.push(ValidationResult::fail(
                    "cert_chain",
                    &format!("{} {}", i18n::tr("File is not valid PEM:"), path.display()),
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
                            "{} {:04o} — {} {}",
                            i18n::tr("Private key has unsafe permissions"),
                            mode,
                            i18n::tr("expected 0600 or 0400. Run: chmod 0600"),
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
                        &format!(
                            "{} {mode:04o} ({})",
                            i18n::tr("Permissions"),
                            i18n::tr("secure")
                        ),
                    ));
                }
                // Check owner matches effective uid (direct syscall, no procfs).
                let euid = get_euid();
                if meta.uid() != euid {
                    out.push(ValidationResult::warn(
                        "key_owner",
                        &format!(
                            "{} uid={} {} euid={}",
                            i18n::tr("Private key owner"),
                            meta.uid(),
                            i18n::tr("does not match process"),
                            euid
                        ),
                    ));
                }
            }
            Err(e) => {
                out.push(ValidationResult::warn(
                    "key_permissions",
                    &format!("{} {e}", i18n::tr("Cannot stat key file:")),
                ));
            }
        }
    }

    match std::fs::read(path) {
        Err(e) => {
            let msg = if e.kind() == std::io::ErrorKind::NotFound {
                format!("{} {}", i18n::tr("File not found:"), path.display())
            } else {
                format!("{} {e}", i18n::tr("Cannot read:"))
            };
            out.push(ValidationResult::fail("private_key", &msg));
            false
        }
        Ok(bytes) => {
            if is_valid_pem(&bytes) {
                out.push(ValidationResult::pass(
                    "private_key",
                    &format!("{} {}", i18n::tr("Valid PEM at"), path.display()),
                ));
                true
            } else {
                out.push(ValidationResult::fail(
                    "private_key",
                    &format!("{} {}", i18n::tr("File is not valid PEM:"), path.display()),
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
            out.push(ValidationResult::pass(
                "key_cert_match",
                "Private key matches certificate",
            ));
        }
        Err(e) => out.push(ValidationResult::fail("key_cert_match", &e.to_string())),
    }
}

fn check_algorithm(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    let alg = &config.identity.algorithm;
    if alg == "ed25519" {
        out.push(ValidationResult::warn(
            "algorithm",
            &i18n::tr(
                "ed25519 is not reliably available on FIPS-enabled systems. \
                 Recommended: es256, es384, or es512.",
            ),
        ));
    } else if ALLOWED_ALGORITHMS.contains(&alg.as_str()) {
        let desc = crate::c2pa::signer::describe_algorithm(alg);
        out.push(ValidationResult::pass("algorithm", &desc));
    } else {
        out.push(ValidationResult::fail(
            "algorithm",
            &format!(
                "'{alg}' {} {}",
                i18n::tr("is not allowed. Use one of:"),
                ALLOWED_ALGORITHMS.join(", ")
            ),
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
            &format!("{} {url}", i18n::tr("TSA endpoint reachable:")),
        )),
        Err(e) => out.push(ValidationResult::warn(
            "tsa_reachable",
            &format!("{} {url} ({e})", i18n::tr("TSA endpoint did not respond:")),
        )),
    }

    #[cfg(not(feature = "internet"))]
    out.push(ValidationResult::warn(
        "tsa_reachable",
        &format!(
            "TSA {} ({url}) {} — {}",
            i18n::tr("configured"),
            i18n::tr("but network feature is disabled"),
            i18n::tr("timestamps will be unsigned")
        ),
    ));
}

fn check_trust_config(config: &UmrsConfig, out: &mut Vec<ValidationResult>) {
    if !config.has_trust_config() {
        out.push(ValidationResult::info(
            "trust_config",
            &i18n::tr(
                "No trust lists configured — all manifests will show NO TRUST LIST. \
                 See docs/trust-maintenance.md to set up trust anchors.",
            ),
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
        match std::fs::metadata(path) {
            Ok(_) => {
                out.push(ValidationResult::pass(
                    "allowed_list",
                    &format!("{} {}", i18n::tr("File exists:"), path.display()),
                ));
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                out.push(ValidationResult::fail(
                    "allowed_list",
                    &format!("{} {}", i18n::tr("File not found:"), path.display()),
                ));
            }
            Err(e) => {
                out.push(ValidationResult::fail(
                    "allowed_list",
                    &format!("{} {e}", i18n::tr("Cannot stat:")),
                ));
            }
        }
        #[cfg(unix)]
        check_trust_file_permissions("allowed_list", path, out);
    }
    if let Some(path) = &config.trust.trust_config {
        match std::fs::metadata(path) {
            Ok(_) => {
                out.push(ValidationResult::pass(
                    "trust_eku_config",
                    &format!("{} {}", i18n::tr("EKU config found:"), path.display()),
                ));
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                out.push(ValidationResult::fail(
                    "trust_eku_config",
                    &format!("{} {}", i18n::tr("File not found:"), path.display()),
                ));
            }
            Err(e) => {
                out.push(ValidationResult::fail(
                    "trust_eku_config",
                    &format!("{} {e}", i18n::tr("Cannot stat:")),
                ));
            }
        }
    }

    if let Some(url) = &config.trust.ocsp_responder {
        out.push(ValidationResult::info(
            "ocsp_responder",
            &format!(
                "{} {url} ({})",
                i18n::tr("OCSP responder configured:"),
                i18n::tr("not yet implemented — skeleton only")
            ),
        ));
    }
}

fn check_pem_file(name: &str, path: &std::path::Path, out: &mut Vec<ValidationResult>) {
    match std::fs::read(path) {
        Err(e) => {
            let msg = if e.kind() == std::io::ErrorKind::NotFound {
                format!("{} {}", i18n::tr("File not found:"), path.display())
            } else {
                format!("{} {e}", i18n::tr("Cannot read:"))
            };
            out.push(ValidationResult::fail(name, &msg));
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
                let cert_str = ngettext(
                    "Valid PEM at {} ({} certificate)",
                    "Valid PEM at {} ({} certificates)",
                    u32::try_from(cert_count).unwrap_or(u32::MAX),
                )
                .replacen("{}", &path.display().to_string(), 1)
                .replacen("{}", &cert_count.to_string(), 1);
                out.push(ValidationResult::pass(name, &cert_str));
            } else {
                out.push(ValidationResult::fail(
                    name,
                    &format!("{} {}", i18n::tr("File is not valid PEM:"), path.display()),
                ));
            }
        }
    }
}

/// Check that a trust anchor file is not world-writable.
///
/// Trust anchor files are the root of trust for all C2PA signature validation.
/// A world-writable trust anchor file allows any local user to inject arbitrary
/// CA certificates and bypass signature trust decisions. This is a hard failure —
/// operating with a world-writable trust anchor is not a degraded mode, it is a
/// broken trust model.
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
                // World-writable — anyone can inject CA certificates. Hard failure.
                out.push(ValidationResult::fail(
                    &format!("{name}_permissions"),
                    &format!(
                        "{} ({:04o}) — {}. {} {}",
                        i18n::tr("Trust file is world-writable (mode)"),
                        mode,
                        i18n::tr("any local user could inject CA certificates"),
                        i18n::tr("Run: chmod o-w"),
                        path.display()
                    ),
                ));
                log::error!(
                    target: "umrs",
                    "trust file is world-writable mode={:04o} file={}",
                    mode,
                    path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default()
                );
            }
        }
    }
}

/// Return the effective user ID via a direct syscall (no `/proc` parsing).
///
/// Uses `rustix::process::geteuid()` — a safe wrapper around the `geteuid(2)`
/// syscall. No file I/O, no procfs parsing, no `unsafe`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Access Enforcement — EUID is compared against
///   the private key file owner to detect ownership mismatches.
/// - **NSA RTB RAIN**: Non-bypassable — direct syscall eliminates the procfs
///   read that previously deviated from the SecureReader rule.
#[cfg(unix)]
fn get_euid() -> u32 {
    rustix::process::geteuid().as_raw()
}

/// Naively check whether bytes look like PEM (contains "-----BEGIN").
fn is_valid_pem(bytes: &[u8]) -> bool {
    bytes.windows(11).any(|w| w == b"-----BEGIN ")
}
