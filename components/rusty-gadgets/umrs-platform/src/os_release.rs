// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # OS Release — Typed Representation of `/etc/os-release`
//!
//! Strongly-typed representation of the `os-release(5)` file and all
//! newtypes for its individual fields. Every field is a validated newtype —
//! raw strings never cross the module boundary.
//!
//! Validation is enforced at construction: each newtype's `parse` method
//! rejects values that violate the field's structural constraints (length,
//! character set, required prefixes). This ensures that an `OsRelease` value,
//! once constructed, is always structurally valid.
//!
//! Two-path independent parsing of the `os-release` file itself occurs in
//! `detect/release_parse.rs`. This module provides only the types and their
//! per-field validation logic.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Information System Component Inventory —
//!   component identity fields must be accurately typed. Untyped string soup
//!   allows silent conflation of unrelated fields.
//! - **NIST SP 800-53 SI-10**: Information Input Validation — all field values
//!   are validated at construction. Callers cannot obtain an `OsId`, `VersionId`,
//!   or `CpeName` from an input that fails the field's structural rules.
//! - **NIST SP 800-53 SI-12**: Information Management and Retention — error
//!   payloads truncated to 64 characters at log call sites (not here) to
//!   prevent log flooding with user-controlled content.

use thiserror::Error;

// ===========================================================================
// OsReleaseParseError
// ===========================================================================

/// Errors that can occur when parsing individual `os-release` field values.
///
/// Each variant carries a `String` payload containing the offending input,
/// truncated to 64 characters at log and display call sites (NIST SP 800-53
/// SI-12 — callers must apply truncation before surfacing in logs or error
/// messages).
///
/// NIST SP 800-53 SI-10 — input validation failure.
#[derive(Debug, Clone, Error)]
pub enum OsReleaseParseError {
    /// `ID=` or `ID_LIKE=` value failed character/length validation.
    #[error("invalid ID field")]
    InvalidId(String),

    /// `NAME=` or `PRETTY_NAME=` value failed length/UTF-8 validation.
    #[error("invalid NAME field")]
    InvalidName(String),

    /// `VERSION_ID=` value failed character/length validation.
    #[error("invalid VERSION_ID field")]
    InvalidVersionId(String),

    /// `VERSION=` value failed length validation.
    #[error("invalid VERSION field")]
    InvalidVersion(String),

    /// `VERSION_CODENAME=` value failed character/length validation.
    #[error("invalid VERSION_CODENAME field")]
    InvalidCodename(String),

    /// `CPE_NAME=` value failed prefix/length validation.
    #[error("invalid CPE_NAME field")]
    InvalidCpe(String),

    /// `HOME_URL=` or similar URL field failed prefix/length validation.
    #[error("invalid URL field")]
    InvalidUrl(String),

    /// `VARIANT_ID=` value failed character/length validation.
    #[error("invalid VARIANT_ID field")]
    InvalidVariantId(String),

    /// `BUILD_ID=` value failed character/length validation.
    #[error("invalid BUILD_ID field")]
    InvalidBuildId(String),

    /// A key appeared more than once in the file.
    #[error("duplicate key")]
    DuplicateKey(String),

    /// File content contained non-UTF-8 bytes.
    #[error("non-UTF-8 content in os-release")]
    NonUtf8,

    /// A single line exceeded the configured maximum length.
    #[error("line too long ({0} bytes)")]
    LineTooLong(usize),

    /// A required field (`ID=`, `NAME=`) was absent.
    #[error("required field missing")]
    MissingRequired(String),
}

// ===========================================================================
// Validated newtypes
// ===========================================================================

/// Validated OS identifier from `ID=`.
///
/// Constraints: lowercase ASCII alphanumeric, hyphens, and underscores only;
/// non-empty; at most 64 characters.
///
/// NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsId(String);

impl OsId {
    /// Parse and validate an `ID=` field value.
    ///
    /// Leading/trailing whitespace and enclosing double-quotes are stripped
    /// before validation (matching `os-release(5)` quoting rules).
    ///
    /// NIST SP 800-53 SI-10 — validates input to the security-critical OS identifier field
    /// at construction.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidId(s.to_owned()));
        }
        if !s.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_'
        }) {
            return Err(OsReleaseParseError::InvalidId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated human-readable OS name from `NAME=` or `PRETTY_NAME=`.
///
/// Constraints: non-empty; at most 256 valid UTF-8 characters.
///
/// NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsName(String);

impl OsName {
    /// Parse and validate a `NAME=` or `PRETTY_NAME=` field value.
    ///
    /// NIST SP 800-53 SI-10: Input Validation — validates OS name at
    /// construction; rejects empty or oversized values.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 256 {
            return Err(OsReleaseParseError::InvalidName(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated machine-readable version identifier from `VERSION_ID=`.
///
/// Constraints: ASCII digits, dots, tildes, and hyphens only; non-empty;
/// at most 32 characters. Examples: `"10.0"`, `"22.04"`, `"9"`.
///
/// NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionId(String);

impl VersionId {
    /// Parse and validate a `VERSION_ID=` field value.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 32 {
            return Err(OsReleaseParseError::InvalidVersionId(s.to_owned()));
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == '~' || c == '-')
        {
            return Err(OsReleaseParseError::InvalidVersionId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated version string as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated human-readable version string from `VERSION=`.
///
/// Constraints: non-empty; at most 128 valid UTF-8 characters.
///
/// NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsVersion(String);

impl OsVersion {
    /// Parse and validate a `VERSION=` field value.
    ///
    /// NIST SP 800-53 SI-10: Input Validation — validates OS version string at
    /// construction; rejects empty or oversized values.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 128 {
            return Err(OsReleaseParseError::InvalidVersion(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated version string as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated distribution codename from `VERSION_CODENAME=`.
///
/// Constraints: non-empty; at most 64 valid UTF-8 characters.
///
/// NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Codename(String);

impl Codename {
    /// Parse and validate a `VERSION_CODENAME=` field value.
    ///
    /// NIST SP 800-53 SI-10: Input Validation — validates distribution codename
    /// at construction; rejects empty or oversized values.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidCodename(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated codename as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated CPE name from `CPE_NAME=`.
///
/// Constraints: must start with `"cpe:/"` (CPE 2.2) or `"cpe:2.3:"`
/// (CPE 2.3); at most 256 characters.
///
/// NIST SP 800-53 CM-8 — CPE identifiers enable mapping to the NVD
/// vulnerability database for accurate patch state assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpeName(String);

impl CpeName {
    /// Parse and validate a `CPE_NAME=` field value.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.len() > 256
            || (!s.starts_with("cpe:/") && !s.starts_with("cpe:2.3:"))
        {
            return Err(OsReleaseParseError::InvalidCpe(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated CPE string as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated URL from `HOME_URL=` or similar URL fields.
///
/// Constraints: must start with `"https://"` or `"http://"`;
/// at most 512 characters.
///
/// NIST SP 800-53 SI-10 — URL prefix validation prevents
/// data URI or file URI injection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedUrl(String);

impl ValidatedUrl {
    /// Parse and validate a URL field value.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.len() > 512
            || (!s.starts_with("https://") && !s.starts_with("http://"))
        {
            return Err(OsReleaseParseError::InvalidUrl(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated URL as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated variant identifier from `VARIANT_ID=`.
///
/// Constraints: non-empty; at most 64 valid UTF-8 characters.
///
/// NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantId(String);

impl VariantId {
    /// Parse and validate a `VARIANT_ID=` field value.
    ///
    /// NIST SP 800-53 SI-10: Input Validation — validates variant ID at
    /// construction; rejects empty or oversized values.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidVariantId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated variant ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated build identifier from `BUILD_ID=`.
///
/// Constraints: non-empty; at most 128 printable ASCII characters or spaces.
///
/// NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildId(String);

impl BuildId {
    /// Parse and validate a `BUILD_ID=` field value.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty()
            || s.len() > 128
            || !s.chars().all(|c| c.is_ascii_graphic() || c == ' ')
        {
            return Err(OsReleaseParseError::InvalidBuildId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated build ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ===========================================================================
// OsRelease
// ===========================================================================

/// Strongly-typed representation of a parsed `os-release(5)` file.
///
/// All fields are typed newtypes validated at construction — no raw strings.
/// Optional fields use `Option<T>` to make absence explicit; there is no
/// defaulting of missing fields.
///
/// Two-path independent parsing occurs in `detect/release_parse.rs`.
/// This type is the target of that parse.
///
/// NIST SP 800-53 CM-8 — typed component inventory; no string soup.
/// NIST SP 800-53 SI-10 — all field values validated at construction.
#[derive(Debug, Clone)]
pub struct OsRelease {
    /// `ID=` — machine-readable distribution identifier (e.g., `"rhel"`, `"debian"`).
    /// Required field.
    pub id: OsId,

    /// `ID_LIKE=` — space-separated parent distribution identifiers. Optional.
    pub id_like: Option<Vec<OsId>>,

    /// `NAME=` — human-readable distribution name. Required field.
    pub name: OsName,

    /// `VERSION_ID=` — machine-readable version string. Optional.
    pub version_id: Option<VersionId>,

    /// `VERSION=` — human-readable version, may include codename. Optional.
    pub version: Option<OsVersion>,

    /// `VERSION_CODENAME=` — distribution codename (e.g., `"bookworm"`). Optional.
    pub version_codename: Option<Codename>,

    /// `PRETTY_NAME=` — display string for the OS. Optional.
    pub pretty_name: Option<OsName>,

    /// `HOME_URL=` — upstream project URL. Optional.
    pub home_url: Option<ValidatedUrl>,

    /// `CPE_NAME=` — NIST NVD CPE identifier if present. Optional.
    /// Presence enables accurate NVD vulnerability mapping (NIST CM-8).
    pub cpe_name: Option<CpeName>,

    /// `VARIANT_ID=` — variant identifier (e.g., `"server"`, `"workstation"`). Optional.
    pub variant_id: Option<VariantId>,

    /// `BUILD_ID=` — immutable image build identifier. Optional.
    pub build_id: Option<BuildId>,

    /// `ANSI_COLOR=` — terminal color hint. Informational only; not validated.
    ///
    /// **MUST NOT** be used for policy or identity decisions; informational display only.
    pub ansi_color: Option<String>,
}
