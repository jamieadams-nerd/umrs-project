// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA Trust Settings Builder
//!
//! Bridges the UMRS trust configuration (paths in `TrustConfig`) to the
//! c2pa SDK's `Settings` type, which expects PEM **content strings**, not
//! file paths.
//!
//! The primary entry point is [`build_c2pa_settings`], which reads PEM files
//! from disk at the paths configured in `[trust]` and injects their contents
//! into a `c2pa::Settings` object.  The resulting `Settings` is consumed by
//! a `c2pa::Context` that is then passed to `c2pa::Reader::from_context`, so
//! every manifest read carries real trust validation.
//!
//! ## Trust Model
//!
//! - If `verify_trust = false` in config, trust validation is disabled and
//!   `NoTrustList` remains the status for all chain entries.
//! - If no trust files are configured, the function returns default settings
//!   (no trust list loaded, no validation attempted).
//! - If any PEM file cannot be read, the error is surfaced as
//!   `InspectError::Io` — the caller must not proceed with degraded trust.
//!
//! ## Trust Anchor Combination
//!
//! When both `trust_anchors` and `user_anchors` are configured, their PEM
//! content is concatenated into a single bundle.  The c2pa SDK accepts a
//! PEM bundle (multiple certificates in one string) for `trust.trust_anchors`.
//! Separately configurable `allowed_list` and `trust_config` paths are fed
//! through their respective SDK settings keys.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — trust anchor
//!   validation is mandatory for FIPS environments; this module ensures the
//!   SDK receives verifiable CA material rather than operating with an empty
//!   trust list.
//! - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
//!   supplying an explicit trust list closes the gap between manifest
//!   signature presence and cryptographic verification against known roots.
//! - **NSA RTB RAIN**: Non-Bypassability — the `build_c2pa_settings` function
//!   is the single gate through which trust configuration passes to the SDK;
//!   all reader construction paths in `manifest.rs` call it, so trust can
//!   never be silently bypassed.

use std::path::Path;

use c2pa::settings::Settings;
use log::warn;

#[cfg(unix)]
use libc;

use crate::c2pa::{config::UmrsConfig, error::InspectError};
#[allow(unused_imports)]
use crate::verbose;

/// Count PEM certificates in a PEM bundle string.
///
/// Used for verbose diagnostic output only — does not validate the certs.
fn count_pem_certs(pem: &str) -> usize {
    pem.matches("-----BEGIN").count()
}

/// Read a PEM file from disk and return its content as a `String`.
///
/// On Unix, opens the file with `O_NOFOLLOW` to refuse symlink targets.
/// An attacker who can write to the trust anchor directory could otherwise
/// replace a trust anchor path with a symlink to an attacker-controlled file,
/// causing the SDK to validate manifests against an injected CA certificate.
///
/// On non-Unix platforms, falls back to `std::fs::read_to_string`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Access Enforcement — `O_NOFOLLOW` enforces that
///   the configured path names a regular file, not an attacker-redirected symlink.
/// - **NSA RTB**: TOCTOU defense — the fd is obtained once; all subsequent
///   I/O uses the same fd, eliminating the open-then-read race window.
///
/// # Errors
///
/// Returns `InspectError::Io` if the file cannot be opened or read, or if
/// the bytes are not valid UTF-8.
fn read_pem(path: &Path) -> Result<String, InspectError> {
    #[cfg(unix)]
    {
        use std::io::Read;
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)
            .map_err(InspectError::Io)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(InspectError::Io)?;
        Ok(contents)
    }
    #[cfg(not(unix))]
    {
        std::fs::read_to_string(path).map_err(InspectError::Io)
    }
}

/// Build c2pa SDK `Settings` from the UMRS trust configuration.
///
/// Reads PEM files from disk at the paths in `config.trust`, loads their
/// content into a `c2pa::Settings` object, and returns it so the caller can
/// construct a `c2pa::Context` with real trust validation enabled.
///
/// ## Behaviour
///
/// | Condition | Result |
/// |---|---|
/// | `verify_trust = false` | Returns default settings; trust disabled |
/// | No trust files configured | Returns default settings; no trust list |
/// | Files configured but unreadable | Returns `Err(InspectError::Io)` |
/// | Files loaded successfully | Returns settings with trust anchors set |
///
/// ## Errors
///
/// Returns `InspectError::Io` if any configured PEM file cannot be read.
/// Returns `InspectError::Config` if the c2pa SDK rejects the assembled
/// settings (e.g. malformed PEM content).
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-13**: Cryptographic Protection
/// - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity
/// - **NSA RTB RAIN**: Non-Bypassability
#[must_use = "Settings must be passed to a Context to take effect"]
pub fn build_c2pa_settings(config: &UmrsConfig) -> Result<Settings, InspectError> {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    // Gate 1: operator has disabled trust validation.
    if !config.trust.verify_trust {
        verbose!("Trust validation disabled by config (verify_trust = false)");
        log::debug!(target: "umrs", "trust_settings verify_trust=false skipping");

        #[cfg(debug_assertions)]
        log::debug!(
            target: "umrs",
            "TrustSettingsBuilder pattern completed (no-op) in {} µs",
            start.elapsed().as_micros()
        );

        return Ok(Settings::default());
    }

    // Gate 2: no trust files are configured at all.
    if !config.has_trust_config() {
        verbose!("No trust files configured — trust validation cannot be performed");
        log::debug!(target: "umrs", "trust_settings no_trust_files configured");
        warn!(
            target: "umrs",
            "No C2PA trust files configured. \
             Manifests will resolve as NO TRUST LIST. \
             Configure trust_anchors in [trust] to enable validation."
        );

        #[cfg(debug_assertions)]
        log::debug!(
            target: "umrs",
            "TrustSettingsBuilder pattern completed (no trust files) in {} µs",
            start.elapsed().as_micros()
        );

        return Ok(Settings::default());
    }

    // Build a settings instance, injecting each configured PEM file.
    let mut settings = Settings::new();

    // --- trust_anchors + user_anchors → trust.trust_anchors (combined bundle) ---
    //
    // The SDK accepts a single PEM bundle for `trust.trust_anchors`.  When
    // both `trust_anchors` and `user_anchors` are configured, we concatenate
    // them into one bundle: official C2PA roots first, operator roots second.
    let mut anchor_bundle = String::new();

    if let Some(path) = &config.trust.trust_anchors {
        verbose!("Loading trust anchors from: {}", path.display());
        let pem = read_pem(path)?;
        let cert_count = count_pem_certs(&pem);
        verbose!("  {} certificate(s) loaded from trust anchors", cert_count);
        log::debug!(target: "umrs", "trust_anchors path={} certs={}", path.display(), cert_count);
        anchor_bundle.push_str(&pem);
    }

    if let Some(path) = &config.trust.user_anchors {
        verbose!("Loading user anchors from: {}", path.display());
        let pem = read_pem(path)?;
        let cert_count = count_pem_certs(&pem);
        verbose!("  {} certificate(s) loaded from user anchors", cert_count);
        log::debug!(target: "umrs", "user_anchors path={} certs={}", path.display(), cert_count);
        if !anchor_bundle.is_empty() && !anchor_bundle.ends_with('\n') {
            anchor_bundle.push('\n');
        }
        anchor_bundle.push_str(&pem);
    }

    if !anchor_bundle.is_empty() {
        let total_certs = count_pem_certs(&anchor_bundle);
        verbose!(
            "Trust validation enabled with {} total anchor certificate(s)",
            total_certs
        );
        settings = settings
            .with_value("trust.trust_anchors", anchor_bundle)
            .map_err(|e| InspectError::Config(format!("c2pa trust_anchors settings error: {e}")))?;
    }

    // --- allowed_list → trust.allowed_list ---
    if let Some(path) = &config.trust.allowed_list {
        verbose!("Loading allowed list from: {}", path.display());
        let pem = read_pem(path)?;
        let cert_count = count_pem_certs(&pem);
        verbose!("  {} certificate(s) in allowed list", cert_count);
        log::debug!(target: "umrs", "allowed_list path={} certs={}", path.display(), cert_count);
        settings = settings
            .with_value("trust.allowed_list", pem)
            .map_err(|e| InspectError::Config(format!("c2pa allowed_list settings error: {e}")))?;
    }

    // --- trust_config → trust.trust_config ---
    if let Some(path) = &config.trust.trust_config {
        verbose!("Loading EKU OID config from: {}", path.display());
        let content = read_pem(path)?;
        log::debug!(target: "umrs", "trust_config path={}", path.display());
        settings = settings
            .with_value("trust.trust_config", content)
            .map_err(|e| InspectError::Config(format!("c2pa trust_config settings error: {e}")))?;
    }

    // Enable trust verification in the verify sub-section.
    settings = settings
        .with_value("verify.verify_trust", true)
        .map_err(|e| InspectError::Config(format!("c2pa verify_trust settings error: {e}")))?;

    #[cfg(debug_assertions)]
    log::debug!(
        target: "umrs",
        "TrustSettingsBuilder pattern completed in {} µs",
        start.elapsed().as_micros()
    );

    Ok(settings)
}
