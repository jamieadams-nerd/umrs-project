// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA Configuration
//!
//! Defines the TOML-driven configuration model for the UMRS C2PA tool.
//! Configuration is loaded once at startup and treated as immutable for the
//! duration of a run. All fields have safe defaults so the binary can operate
//! in ephemeral test mode without any configuration file present.
//!
//! ## Key Exported Types
//!
//! - [`UmrsConfig`] — top-level configuration struct
//! - [`IdentityConfig`] — signing identity and credential paths
//! - [`TrustConfig`] — C2PA trust list configuration
//! - [`PolicyConfig`] — ingest action labels and reason strings
//! - [`LoggingConfig`] — log level and enable/disable
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-6**: Configuration Settings — all operational
//!   parameters are explicit, documented, and loaded from a versioned file.
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — the `algorithm`
//!   field is validated against the FIPS-safe allow-list at signer construction
//!   time; config loading does not validate it to allow fail-closed reporting.
//! - **NSA RTB RAIN**: Non-Bypassability — credential paths are not accessed
//!   until explicitly requested by the caller; the config struct does not
//!   perform I/O at load time, enforcing the Trust Gate pattern (NIST SP
//!   800-53 CM-6) and ensuring callers cannot bypass the validation step.

use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use libc;

use crate::c2pa::error::InspectError;

/// Top-level UMRS configuration loaded from `umrs-c2pa.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UmrsConfig {
    #[serde(default)]
    pub identity: IdentityConfig,

    #[serde(default)]
    pub timestamp: TimestampConfig,

    #[serde(default)]
    pub policy: PolicyConfig,

    #[serde(default)]
    pub trust: TrustConfig,

    #[serde(default)]
    pub logging: LoggingConfig,
}

/// Identity and signing credentials.
///
/// ## Fields:
///
/// - `claim_generator` — human-readable name embedded in every manifest produced by this system.
/// - `organization` — organization name for display in chain-of-custody reports.
/// - `cert_chain` — path to PEM-encoded certificate chain (leaf first, root last); if absent,
///   an ephemeral self-signed cert is generated at runtime (test mode).
/// - `private_key` — path to PEM-encoded private key corresponding to the leaf certificate; if
///   absent, an ephemeral self-signed cert is generated at runtime (test mode).
/// - `algorithm` — signing algorithm; must be in the FIPS-safe set: `es256 | es384 | es512 |
///   ps256 | ps384 | ps512`; `ed25519` is intentionally excluded — unreliable on FIPS RHEL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    #[serde(default = "default_claim_generator")]
    pub claim_generator: String,

    #[serde(default = "default_organization")]
    pub organization: String,

    pub cert_chain: Option<PathBuf>,

    pub private_key: Option<PathBuf>,

    #[serde(default = "default_algorithm")]
    pub algorithm: String,
}

/// Optional Time Stamp Authority configuration.
///
/// ## Fields:
///
/// - `tsa_url` — TSA URL for trusted signing timestamps (e.g., `"http://timestamp.digicert.com"`);
///   omit to sign without a TSA timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimestampConfig {
    pub tsa_url: Option<String>,
}

/// Ingest policy — action labels and reason strings.
///
/// ## Fields:
///
/// - `unsigned_action` — C2PA action label for files arriving without an existing manifest.
/// - `unsigned_reason` — reason string embedded in the action assertion for unsigned files.
/// - `signed_action` — C2PA action label for files arriving with an existing manifest.
/// - `signed_reason` — reason string embedded in the action assertion for signed files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    #[serde(default = "default_unsigned_action")]
    pub unsigned_action: String,

    #[serde(default = "default_unsigned_reason")]
    pub unsigned_reason: String,

    #[serde(default = "default_signed_action")]
    pub signed_action: String,

    #[serde(default = "default_signed_reason")]
    pub signed_reason: String,
}

/// Trust list configuration for C2PA signature validation.
///
/// All paths are fully configurable — no hardcoded default location.
/// See `docs/trust-maintenance.md` for update procedures.
///
/// ## Fields:
///
/// - `trust_anchors` — path to PEM bundle of root CA certificates (C2PA official or
///   org-specific); operator updates this file manually.
/// - `user_anchors` — path to PEM bundle of additional user/org root CAs.
/// - `allowed_list` — path to end-entity certificate allowlist (PEM or base64 SHA-256 hashes).
/// - `trust_config` — path to EKU OID configuration file (one OID per line, `//` comments).
/// - `verify_trust` — enable trust validation; defaults to `true` when any trust file is
///   configured.
/// - `ocsp_responder` — OCSP responder URL (skeleton — not fully implemented yet); organizations
///   can point this to their own OCSP server when ready.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrustConfig {
    pub trust_anchors: Option<PathBuf>,
    pub user_anchors: Option<PathBuf>,
    pub allowed_list: Option<PathBuf>,
    pub trust_config: Option<PathBuf>,
    #[serde(default = "default_verify_trust")]
    pub verify_trust: bool,
    pub ocsp_responder: Option<String>,
}

/// Logging configuration.
///
/// ## Fields:
///
/// - `enabled` — enable or disable all logging output.
/// - `level` — minimum log level: `off | error | warn | info | debug | trace`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_logging_enabled")]
    pub enabled: bool,
    #[serde(default = "default_log_level")]
    pub level: String,
}

impl UmrsConfig {
    /// Load configuration from a TOML file at `path`.
    ///
    /// On Unix, opens the file with `O_NOFOLLOW` to refuse symlink targets.
    /// The configuration file drives algorithm selection, credential paths, and
    /// trust list paths; a symlink substitution attack against it could redirect
    /// any of these to attacker-controlled values.
    ///
    /// On non-Unix platforms, falls back to `std::fs::read_to_string`.
    ///
    /// ## Compliance
    ///
    /// - **NIST SP 800-53 CM-6**: Configuration Settings — `O_NOFOLLOW` ensures
    ///   the config file is the operator-controlled file at the named path, not
    ///   an attacker-supplied symlink target.
    /// - **NSA RTB**: TOCTOU defense — single fd open; no re-open by path.
    ///
    /// # Errors
    ///
    /// Returns `InspectError::Io` if the file cannot be read, or
    /// `InspectError::Config` if the TOML is malformed.
    pub fn load(path: &Path) -> Result<Self, InspectError> {
        #[cfg(unix)]
        let text = {
            use std::io::Read;
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NOFOLLOW)
                .open(path)
                .map_err(InspectError::Io)?;
            let mut buf = String::new();
            file.read_to_string(&mut buf).map_err(InspectError::Io)?;
            buf
        };
        #[cfg(not(unix))]
        let text = std::fs::read_to_string(path).map_err(InspectError::Io)?;

        toml::from_str(&text).map_err(|e| InspectError::Config(e.to_string()))
    }

    /// Return the effective `LevelFilter` for the logging backend.
    #[must_use = "Log level filter determines audit log verbosity; \
                  discarding it means the logging backend is not configured"]
    pub fn log_level_filter(&self) -> LevelFilter {
        if !self.logging.enabled {
            return LevelFilter::Off;
        }
        match self.logging.level.to_lowercase().as_str() {
            "off" => LevelFilter::Off,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }

    /// Returns `true` if the identity config has cert + key configured (production mode).
    #[must_use = "Credential presence drives ephemeral-vs-production mode selection; \
                  discarding it would silently skip the mode check"]
    pub const fn has_credentials(&self) -> bool {
        self.identity.cert_chain.is_some() && self.identity.private_key.is_some()
    }

    /// Returns `true` if any trust list files are configured.
    #[must_use = "Trust config presence gates trust validation and warning emission; \
                  discarding it would silently skip trust configuration checks"]
    pub const fn has_trust_config(&self) -> bool {
        self.trust.trust_anchors.is_some()
            || self.trust.user_anchors.is_some()
            || self.trust.allowed_list.is_some()
    }
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            claim_generator: default_claim_generator(),
            organization: default_organization(),
            cert_chain: None,
            private_key: None,
            algorithm: default_algorithm(),
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            unsigned_action: default_unsigned_action(),
            unsigned_reason: default_unsigned_reason(),
            signed_action: default_signed_action(),
            signed_reason: default_signed_reason(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: default_logging_enabled(),
            level: default_log_level(),
        }
    }
}

// --- serde defaults ---

fn default_claim_generator() -> String {
    "UMRS Reference System/1.0".into()
}
fn default_organization() -> String {
    "UMRS".into()
}
fn default_algorithm() -> String {
    "es256".into()
}
fn default_unsigned_action() -> String {
    "c2pa.acquired".into()
}
fn default_signed_action() -> String {
    "c2pa.published".into()
}
const fn default_logging_enabled() -> bool {
    true
}
fn default_log_level() -> String {
    "info".into()
}

const fn default_verify_trust() -> bool {
    true
}

fn default_unsigned_reason() -> String {
    "Received at UMRS trusted ingest dropbox. Origin unknown. No modifications made.".into()
}

fn default_signed_reason() -> String {
    "Received at UMRS trusted ingest dropbox with existing provenance. No modifications made."
        .into()
}
