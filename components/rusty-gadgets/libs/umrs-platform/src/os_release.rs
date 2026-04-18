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
#![doc = include_str!("../docs/compliance-os_release.md")]

use thiserror::Error;

// ===========================================================================
// OsReleaseParseError
// ===========================================================================

/// Errors that can occur when parsing individual `os-release` field values.
///
/// Each variant carries a `String` payload containing the offending input,
/// truncated to 64 characters at log and display call sites
/// (NIST SP 800-53 SI-12 — callers must apply truncation before surfacing in logs).
///
/// ## Variants:
///
/// - `InvalidId(String)` — `ID=` or `ID_LIKE=` value failed character/length validation.
/// - `InvalidName(String)` — `NAME=` or `PRETTY_NAME=` value failed length/UTF-8 validation.
/// - `InvalidVersionId(String)` — `VERSION_ID=` value failed character/length validation.
/// - `InvalidVersion(String)` — `VERSION=` value failed length validation.
/// - `InvalidCodename(String)` — `VERSION_CODENAME=` value failed character/length validation.
/// - `InvalidCpe(String)` — `CPE_NAME=` value failed prefix/length validation.
/// - `InvalidUrl(String)` — `HOME_URL=` or similar URL field failed prefix/length validation.
/// - `InvalidVariantId(String)` — `VARIANT_ID=` value failed character/length validation.
/// - `InvalidBuildId(String)` — `BUILD_ID=` value failed character/length validation.
/// - `DuplicateKey(String)` — a key appeared more than once in the file.
/// - `NonUtf8` — file content contained non-UTF-8 bytes.
/// - `LineTooLong(usize)` — a single line exceeded the configured maximum length.
/// - `MissingRequired(String)` — a required field (`ID=`, `NAME=`) was absent.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: input validation failure.
#[derive(Debug, Clone, Error)]
pub enum OsReleaseParseError {
    #[error("invalid ID field")]
    InvalidId(String),

    #[error("invalid NAME field")]
    InvalidName(String),

    #[error("invalid VERSION_ID field")]
    InvalidVersionId(String),

    #[error("invalid VERSION field")]
    InvalidVersion(String),

    #[error("invalid VERSION_CODENAME field")]
    InvalidCodename(String),

    #[error("invalid CPE_NAME field")]
    InvalidCpe(String),

    #[error("invalid URL field")]
    InvalidUrl(String),

    #[error("invalid VARIANT_ID field")]
    InvalidVariantId(String),

    #[error("invalid BUILD_ID field")]
    InvalidBuildId(String),

    #[error("duplicate key")]
    DuplicateKey(String),

    #[error("non-UTF-8 content in os-release")]
    NonUtf8,

    #[error("line too long ({0} bytes)")]
    LineTooLong(usize),

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
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsId(String);

impl OsId {
    /// Parse and validate an `ID=` field value.
    ///
    /// Leading/trailing whitespace and enclosing double-quotes are stripped
    /// before validation (matching `os-release(5)` quoting rules).
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SI-10 — validates input to the security-critical OS identifier field
    ///   at construction.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value is empty, exceeds the length limit, or contains invalid characters.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidId(s.to_owned()));
        }
        if !s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        {
            return Err(OsReleaseParseError::InvalidId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated identifier as a string slice.
    #[must_use = "pure accessor — returns the validated OS identifier string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for OsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validated human-readable OS name from `NAME=` or `PRETTY_NAME=`.
///
/// Constraints: non-empty; at most 256 valid UTF-8 characters.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsName(String);

impl OsName {
    /// Parse and validate a `NAME=` or `PRETTY_NAME=` field value.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SI-10: Input Validation — validates OS name at
    ///   construction; rejects empty or oversized values.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value is empty or exceeds 256 bytes.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 256 {
            return Err(OsReleaseParseError::InvalidName(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated name as a string slice.
    #[must_use = "pure accessor — returns the validated OS name string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for OsName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validated machine-readable version identifier from `VERSION_ID=`.
///
/// Constraints: ASCII digits, dots, tildes, and hyphens only; non-empty;
/// at most 32 characters. Examples: `"10.0"`, `"22.04"`, `"9"`.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionId(String);

impl VersionId {
    /// Parse and validate a `VERSION_ID=` field value.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value is empty, exceeds 32 bytes, or contains non-version characters.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 32 {
            return Err(OsReleaseParseError::InvalidVersionId(s.to_owned()));
        }
        if !s.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '~' || c == '-') {
            return Err(OsReleaseParseError::InvalidVersionId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated version string as a string slice.
    #[must_use = "pure accessor — returns the validated VERSION_ID string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validated human-readable version string from `VERSION=`.
///
/// Constraints: non-empty; at most 128 valid UTF-8 characters.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsVersion(String);

impl OsVersion {
    /// Parse and validate a `VERSION=` field value.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SI-10: Input Validation — validates OS version string at
    ///   construction; rejects empty or oversized values.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value is empty or exceeds 128 bytes.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 128 {
            return Err(OsReleaseParseError::InvalidVersion(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated version string as a string slice.
    #[must_use = "pure accessor — returns the validated VERSION string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated distribution codename from `VERSION_CODENAME=`.
///
/// Constraints: non-empty; at most 64 valid UTF-8 characters.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Codename(String);

impl Codename {
    /// Parse and validate a `VERSION_CODENAME=` field value.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SI-10: Input Validation — validates distribution codename
    ///   at construction; rejects empty or oversized values.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value is empty or exceeds 64 bytes.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidCodename(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated codename as a string slice.
    #[must_use = "pure accessor — returns the validated VERSION_CODENAME string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated CPE name from `CPE_NAME=`.
///
/// Constraints: must start with `"cpe:/"` (CPE 2.2) or `"cpe:2.3:"`
/// (CPE 2.3); at most 256 characters.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8 — CPE identifiers enable mapping to the NVD
///   vulnerability database for accurate patch state assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpeName(String);

impl CpeName {
    /// Parse and validate a `CPE_NAME=` field value.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value exceeds 256 bytes or does not start with a valid CPE prefix.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.len() > 256 || (!s.starts_with("cpe:/") && !s.starts_with("cpe:2.3:")) {
            return Err(OsReleaseParseError::InvalidCpe(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated CPE string as a string slice.
    #[must_use = "pure accessor — returns the validated CPE_NAME string used for NVD vulnerability mapping"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated URL from `HOME_URL=` or similar URL fields.
///
/// Constraints: must start with `"https://"` or `"http://"`;
/// at most 512 characters.
///
/// ## Compliance
///
/// - NIST SP 800-53 SI-10 — URL prefix validation prevents
///   data URI or file URI injection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedUrl(String);

impl ValidatedUrl {
    /// Parse and validate a URL field value.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value exceeds 512 bytes or does not use an HTTP(S) scheme.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.len() > 512 || (!s.starts_with("https://") && !s.starts_with("http://")) {
            return Err(OsReleaseParseError::InvalidUrl(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated URL as a string slice.
    #[must_use = "pure accessor — returns the validated HTTP(S) URL string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated variant identifier from `VARIANT_ID=`.
///
/// Constraints: non-empty; at most 64 valid UTF-8 characters.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantId(String);

impl VariantId {
    /// Parse and validate a `VARIANT_ID=` field value.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SI-10: Input Validation — validates variant ID at
    ///   construction; rejects empty or oversized values.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value is empty or exceeds 64 bytes.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidVariantId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated variant ID as a string slice.
    #[must_use = "pure accessor — returns the validated VARIANT_ID string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated build identifier from `BUILD_ID=`.
///
/// Constraints: non-empty; at most 128 printable ASCII characters or spaces.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SI-10.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildId(String);

impl BuildId {
    /// Parse and validate a `BUILD_ID=` field value.
    ///
    /// # Errors
    ///
    /// Returns [`OsReleaseParseError`] if the value is empty, exceeds 128 bytes, or contains non-printable characters.
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 128 || !s.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
            return Err(OsReleaseParseError::InvalidBuildId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }

    /// Return the validated build ID as a string slice.
    #[must_use = "pure accessor — returns the validated BUILD_ID string"]
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
/// ## Fields:
///
/// - `id` — `ID=`: machine-readable distribution identifier (e.g., `"rhel"`, `"debian"`);
///   required field.
/// - `id_like` — `ID_LIKE=`: space-separated parent distribution identifiers; optional.
/// - `name` — `NAME=`: human-readable distribution name; required field.
/// - `version_id` — `VERSION_ID=`: machine-readable version string; optional.
/// - `version` — `VERSION=`: human-readable version, may include codename; optional.
/// - `version_codename` — `VERSION_CODENAME=`: distribution codename (e.g., `"bookworm"`);
///   optional.
/// - `pretty_name` — `PRETTY_NAME=`: display string for the OS; optional.
/// - `home_url` — `HOME_URL=`: upstream project URL; optional.
/// - `cpe_name` — `CPE_NAME=`: NIST NVD CPE identifier if present; optional. Presence
///   enables accurate NVD vulnerability mapping. (NIST SP 800-53 CM-8)
/// - `variant_id` — `VARIANT_ID=`: variant identifier (e.g., `"server"`, `"workstation"`);
///   optional.
/// - `build_id` — `BUILD_ID=`: immutable image build identifier; optional.
/// - `ansi_color` — `ANSI_COLOR=`: terminal color hint; informational only, not validated.
///   **MUST NOT** be used for policy or identity decisions.
///
/// ## Compliance
///
/// - **NIST SP 800-53 CM-8**: typed component inventory; no raw string soup.
/// - **NIST SP 800-53 SI-10**: all field values validated at construction.
#[derive(Debug, Clone)]
pub struct OsRelease {
    pub id: OsId,
    pub id_like: Option<Vec<OsId>>,
    pub name: OsName,
    pub version_id: Option<VersionId>,
    pub version: Option<OsVersion>,
    pub version_codename: Option<Codename>,
    pub pretty_name: Option<OsName>,
    pub home_url: Option<ValidatedUrl>,
    pub cpe_name: Option<CpeName>,
    pub variant_id: Option<VariantId>,
    pub build_id: Option<BuildId>,
    pub ansi_color: Option<String>,
}
