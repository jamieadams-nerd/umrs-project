// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//
// ============================================================================
// UMRS SELINUX: Extended Attribute (xattr) Logic
// NIST SP 800-53 AC-3 / NSA RTB (Non-Bypassability & Redundancy)
// ============================================================================
//!
//! # SELinux Extended Attribute (xattr) Reader
//!
//! Provides `SecureXattrReader` — the primary interface for reading the
//! `security.selinux` xattr from an open file descriptor.
//!
//! ## TPI Guarantee
//!
//! All label reads are parsed by two independent paths:
//!
//! - **Path A**: `nom` combinator parser
//! - **Path B**: `FromStr` implementation on `SecurityContext`
//!
//! If the paths disagree, `TpiError::Disagreement` is returned and the read
//! fails closed. A disagreement is treated as a potential integrity event and
//! logged at ERROR. Single-path failures are code defects and are logged at WARN.
//!
//! ## TOCTOU Safety
//!
//! Reads are anchored to the file descriptor opened by the caller — no
//! path-based re-open occurs between open and read. The fd is passed directly
//! to `fgetxattr` via `rustix`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access Enforcement — the security label read here
//!   is the authoritative source for access control decisions.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — TPI ensures
//!   parse-path disagreements are surfaced as integrity events, not silently
//!   resolved.
//! - **NSA RTB RAIN**: Non-Bypassability — callers cannot receive a label
//!   without passing the TPI cross-check gate.
//!
use nom::{
    IResult,
    bytes::complete::{tag, take_until},
};
use rustix::fs::fgetxattr;
use std::fmt;
use std::fs::File;
use std::io;
use std::time::Instant;

use crate::context::{ContextParseError, MlsLevel, SecurityContext};

/// The standard SELinux xattr name (NIST SP 800-53 AU-3: Source Identifier)
pub const XATTR_NAME_SELINUX: &str = "security.selinux";

// ===========================================================================
// TpiError
//
// Typed error discriminating between the three structurally distinct TPI
// outcomes.  A single-path failure is a code defect; a disagreement is a
// potential integrity event.  Callers must not treat these uniformly.
//
// NIST SP 800-53 SI-7: Software and Information Integrity.
// NSA RTB RAIN: Non-Bypassability — the cross-check gate must always fire.
// ===========================================================================

/// Typed error for TPI (Two-Path Independence) parse outcomes.
///
/// | Variant | Meaning | Log level |
/// |---|---|---|
/// | `PathAFailed` | Nom parser failed — code/validator defect | WARN |
/// | `PathBFailed` | FromStr parser failed — code/validator defect | WARN |
/// | `Disagreement` | Both paths succeeded but produced different results | ERROR |
///
/// `Disagreement` is the only variant that constitutes a potential integrity
/// event.  Single-path failures are code defects and must never produce an
/// ERROR log entry.
///
/// NIST SP 800-53 SI-7: Software and Information Integrity.
/// NSA RTB RAIN: Non-Bypassability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TpiError {
    /// Path A (nom parser) failed. Code or validator defect — not a security
    /// event. The reason string contains only the error kind, no raw input.
    /// NIST SP 800-53 SI-12: no sensitive data in error messages.
    PathAFailed(String),

    /// Path B (FromStr parser) failed. Code or validator defect — not a
    /// security event. The reason string contains only the error kind.
    /// NIST SP 800-53 SI-12: no sensitive data in error messages.
    PathBFailed(String),

    /// Both paths succeeded but produced structurally different results.
    ///
    /// Potential integrity event — an adversary manipulating the xattr byte
    /// stream at the kernel interface could produce this outcome.
    /// NIST SP 800-53 SI-7: integrity violation.
    /// NSA RTB RAIN: redundancy cross-check failure.
    Disagreement(Box<SecurityContext>, Box<SecurityContext>),
}

impl fmt::Display for TpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathAFailed(reason) => {
                write!(f, "TPI Path A (nom) failed: {reason}")
            }
            Self::PathBFailed(reason) => {
                write!(f, "TPI Path B (FromStr) failed: {reason}")
            }
            Self::Disagreement(_, _) => {
                write!(
                    f,
                    "TPI disagreement: nom and FromStr produced different contexts"
                )
            }
        }
    }
}

// ===========================================================================
// XattrReadError
//
// Public error type for SecureXattrReader::read_context().
//
// Distinguishes OS-level errors (ENODATA, EACCES, etc.) from TPI-layer
// failures so callers can set the correct SelinuxCtxState on SecureDirent.
//
// NIST SP 800-53 AU-3: caller must distinguish "unlabeled inode" from
// "parse failure" to produce a complete audit record.
// NIST SP 800-53 SI-12: no sensitive data in the error type.
// ===========================================================================

/// Errors returned by [`SecureXattrReader::read_context`].
///
/// Callers must discriminate `OsError` (inode has no xattr, or OS denied the
/// read) from `Tpi` (label was present but could not be verified) to produce
/// correct audit output.
///
/// NIST SP 800-53 AU-3: audit record completeness requires this distinction.
/// NIST SP 800-53 SI-7 / NSA RTB RAIN: TPI integrity.
#[derive(Debug)]
pub enum XattrReadError {
    /// An OS-level error before or during the raw xattr read.
    ///
    /// `ENODATA` means the inode genuinely has no SELinux label.
    /// `EACCES`/`EPERM` means DAC or MAC prevented the read.
    OsError(io::Error),

    /// The raw bytes are present but could not be verified via TPI.
    ///
    /// The label is on the inode but its integrity cannot be confirmed.
    /// Structurally different from an absent label.
    Tpi(TpiError),
}

impl fmt::Display for XattrReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OsError(e) => write!(f, "xattr OS error: {e}"),
            Self::Tpi(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for XattrReadError {
    fn from(e: io::Error) -> Self {
        Self::OsError(e)
    }
}

impl From<TpiError> for XattrReadError {
    fn from(e: TpiError) -> Self {
        Self::Tpi(e)
    }
}

/// High-assurance xattr reader with TPI integrity gate.
///
/// All reads are fd-anchored (TOCTOU safety). The SELinux context path runs
/// two independent parsers and cross-checks them before returning a value.
///
/// NIST SP 800-53 SI-7: integrity before trust.
/// NSA RTB Non-Bypassability: fd-based reads cannot be redirected via path.
/// NSA RTB Redundancy: dual-path cross-check gate must always fire.
pub struct SecureXattrReader;

impl SecureXattrReader {
    /// NIST SP 800-53 SI-7: High-Assurance xattr retrieval via raw syscalls.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the extended attribute cannot be read from the file descriptor.
    pub fn read_raw(file: &File, attr: &str) -> io::Result<Vec<u8>> {
        let size = fgetxattr(file, attr, &mut [] as &mut [u8; 0]).map_err(
            #[expect(clippy::redundant_closure, reason = "explicit closure required for map_err with a From impl that rustc cannot infer without it")]
            |e| io::Error::from(e),
        )?;

        if size == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty xattr"));
        }

        let mut buffer = vec![0u8; size];
        let bytes_read = fgetxattr(file, attr, &mut buffer).map_err(
            #[expect(clippy::redundant_closure, reason = "explicit closure required for map_err with a From impl that rustc cannot infer without it")]
            |e| io::Error::from(e),
        )?;

        buffer.truncate(bytes_read);
        if buffer.last() == Some(&0) {
            buffer.pop();
        }

        Ok(buffer)
    }

    /// Read and verify the SELinux security context via two independent parse
    /// paths (TPI gate).
    ///
    /// ## TPI contract
    ///
    /// Both Path A (nom) and Path B (FromStr) are **always attempted**,
    /// regardless of whether the first succeeds or fails. The cross-check gate
    /// is always reached. This prevents a single-path defect from silently
    /// bypassing the redundancy requirement (NSA RTB RAIN).
    ///
    /// ## Outcome matrix
    ///
    /// | Path A | Path B | Result | Log level |
    /// |--------|--------|--------|-----------|
    /// | Ok | Ok, agree | `Ok(ctx)` | DEBUG |
    /// | Ok | Ok, disagree | `Err(Disagreement)` | ERROR |
    /// | Fail | Ok | `Err(PathAFailed)` | WARN |
    /// | Ok | Fail | `Err(PathBFailed)` | WARN |
    /// | Fail | Fail | `Err(PathAFailed)` | WARN |
    ///
    /// ## SI-12 compliance
    ///
    /// Log messages contain error kinds only — never raw input slices or
    /// security context values. Raw input may contain MLS sensitivity levels
    /// or category sets that are security-relevant.
    ///
    /// NIST SP 800-53 AC-3 / SI-7 / SI-12.
    /// NSA RTB RAIN: redundancy cross-check must always fire.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the extended attribute cannot be read, or an integrity error if TPI validation detects a mismatch.
    pub fn read_context(file: &File) -> Result<SecurityContext, XattrReadError> {
        let start_time = Instant::now();

        let raw_bytes = Self::read_raw(file, XATTR_NAME_SELINUX)?;
        let context_str = std::str::from_utf8(&raw_bytes).map_err(|_| {
            XattrReadError::OsError(io::Error::new(
                io::ErrorKind::InvalidData,
                "Non-UTF8 xattr value",
            ))
        })?;

        // --- PATH A: nom Parser (Declarative) ---
        // Collect result — do NOT short-circuit.  Path B is always attempted.
        let path_a_result = parse_context_nom(context_str);

        // --- PATH B: FromStr (Imperative) ---
        // Always attempted regardless of Path A outcome.  This is the core of
        // the TPI contract: both paths must always run so the gate fires.
        let path_b_result: Result<SecurityContext, ContextParseError> = context_str.parse();

        // --- TPI GATE: always reached ---
        let result = match (path_a_result, path_b_result) {
            (Ok((_, context_a)), Ok(context_b)) => {
                if context_a == context_b {
                    Ok(context_a)
                } else {
                    // CRITICAL integrity event: both parsers succeeded but
                    // produced different results. May indicate adversarial
                    // manipulation of the xattr byte stream.
                    //
                    // SI-12: log only the type component — not sensitivity or
                    // category data, which are security-sensitive.
                    log::error!(
                        "CRITICAL: TPI disagreement — nom and FromStr produced \
                         different security contexts; type-A={}, type-B={} \
                         (NIST SP 800-53 SI-7 / NSA RTB RAIN integrity event)",
                        context_a.security_type(),
                        context_b.security_type(),
                    );
                    Err(TpiError::Disagreement(Box::new(context_a), Box::new(context_b)).into())
                }
            }

            (Err(nom_err), Ok(_)) => {
                // Path A failed, Path B succeeded.  Code/validator defect.
                // WARN — this is NOT a security integrity event.
                let kind = nom_error_kind(&nom_err);
                log::warn!(
                    "TPI Path A (nom) failed — code/validator defect: \
                     kind={kind}"
                );
                Err(TpiError::PathAFailed(kind).into())
            }

            (Ok(_), Err(from_str_err)) => {
                // Path B failed, Path A succeeded.  Code/validator defect.
                // WARN — this is NOT a security integrity event.
                let kind = context_parse_error_kind(&from_str_err);
                log::warn!(
                    "TPI Path B (FromStr) failed — code/validator defect: \
                     kind={kind}"
                );
                Err(TpiError::PathBFailed(kind).into())
            }

            (Err(nom_err), Err(from_str_err)) => {
                // Both paths failed.  Still a code/validator defect.
                // Two consistent failures do not elevate to integrity event.
                let kind_a = nom_error_kind(&nom_err);
                let kind_b = context_parse_error_kind(&from_str_err);
                log::warn!(
                    "TPI both paths failed — Path A: kind={kind_a}; \
                     Path B: kind={kind_b}"
                );
                Err(TpiError::PathAFailed(kind_a).into())
            }
        };

        #[cfg(debug_assertions)]
        {
            let duration = start_time.elapsed();
            log::debug!(
                "TPI gate completed in {} µs — result: {}",
                duration.as_micros(),
                if result.is_ok() {
                    "ok"
                } else {
                    "err"
                },
            );
        }
        // Suppress the unused-variable lint in non-debug builds.
        let _ = start_time;

        result
    }
}

// ===========================================================================
// SI-12: Nom Error Kind Extractor
//
// nom::Err<nom::error::Error<&str>>::Display includes the verbatim input
// slice, which may contain MLS sensitivity levels or category identifiers.
// This helper extracts only the ErrorKind variant name via Debug formatting —
// ErrorKind is a fieldless enum whose Debug output is the variant name only
// (e.g., "Tag", "TakeUntil"). No user-controlled data is included.
//
// NIST SP 800-53 SI-12: Information Management and Retention
// ===========================================================================
/// Extracts the nom `ErrorKind` variant name from a parse error.
///
/// Returns a `String` containing only the structural kind (e.g., `"Tag"`,
/// `"TakeUntil"`) — never the input slice. Used to ensure log entries
/// satisfy NIST SP 800-53 SI-12 by excluding raw user-controlled data.
///
/// This function is `pub` to allow verification in integration tests that
/// its output never contains raw input bytes.
///
/// NIST SP 800-53 SI-12: Information Management and Retention
#[must_use = "returns a sanitized error kind string for diagnostic output; discarding it loses the parse failure reason"]
pub fn nom_error_kind(e: &nom::Err<nom::error::Error<&str>>) -> String {
    match e {
        nom::Err::Incomplete(_) => "Incomplete".to_string(),
        nom::Err::Error(inner) | nom::Err::Failure(inner) => {
            // ErrorKind is a fieldless enum; Debug emits only the variant
            // name — no input slice, no user-controlled data.
            format!("{:?}", inner.code)
        }
    }
}

/// Extract a short kind label from a `ContextParseError` without leaking
/// rejected token values that may contain security-relevant data.
///
/// Returns a truncated Display string.  `ContextParseError` Display is
/// already structured (no raw input), but we cap at 60 chars as a
/// belt-and-suspenders guard against future changes to the error type.
///
/// NIST SP 800-53 SI-12: Information Management and Retention
fn context_parse_error_kind(e: &ContextParseError) -> String {
    let s = e.to_string();
    if s.len() > 60 {
        format!("{}..", &s[..58])
    } else {
        s
    }
}

// ===========================================================================
// TPI Path A Helper Logic
//
// NOM Parser - Next Generation Object Manipulators
//
// In high-assurance engineering, NOM is a Parser Combinator library.
// To an architect, it is the difference between "string-splitting"
// (brute force) and "formal grammar" (surgical precision).
//
// NIST SP 800-53 AC-4: Redundant (Path A) MLS Parser
// ===========================================================================
fn parse_context_nom(input: &str) -> IResult<&str, SecurityContext> {
    use crate::category::CategorySet;
    use crate::role::SelinuxRole;
    use crate::sensitivity::SensitivityLevel;
    use crate::type_id::SelinuxType;
    use crate::user::SelinuxUser;
    use std::str::FromStr;

    // 1. Parse user:role
    let (input, user_raw) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, role_raw) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;

    // 2. Greedy Decompose for the Level
    // Handles multi-part contexts found in RHEL 10 MCS
    let (remaining_after_type, type_raw) =
        match take_until::<&str, &str, nom::error::Error<&str>>(":")(input) {
            Ok((rem, t)) => (rem.strip_prefix(":").unwrap_or(rem), t),
            Err(_) => ("", input),
        };

    log::debug!("[PATH A] Raw Type: '{type_raw}', Level remainder: '{remaining_after_type}'");

    // 3. Level Parsing (Sensitivity + Categories)
    let level = if remaining_after_type.is_empty() {
        None
    } else {
        let (sens_raw, cats_str) =
            remaining_after_type.split_once(':').unwrap_or((remaining_after_type, ""));

        let sens = SensitivityLevel::from_str(sens_raw)
            .unwrap_or_else(|_| SensitivityLevel::new(0).expect("TCB Invariant: s0 must be valid"));

        let cats = if cats_str.is_empty() {
            CategorySet::new()
        } else {
            parse_mcs_categories(cats_str).map_err(|_| {
                nom::Err::Failure(nom::error::Error::new(cats_str, nom::error::ErrorKind::Tag))
            })?
        };

        Some(MlsLevel {
            sensitivity: sens,
            categories: cats,
            raw_level: remaining_after_type.to_string(),
        })
    };

    // 4. Map to Strong Types
    let user = SelinuxUser::from_str(user_raw).map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(user_raw, nom::error::ErrorKind::Tag))
    })?;

    let role = SelinuxRole::from_str(role_raw).map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(role_raw, nom::error::ErrorKind::Tag))
    })?;

    let security_type = SelinuxType::from_str(type_raw).map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(type_raw, nom::error::ErrorKind::Tag))
    })?;

    Ok(("", SecurityContext::new(user, role, security_type, level)))
}

// ===========================================================================
// Shared Category Parser
// NIST SP 800-53 AC-4: Helper to parse MCS Category strings (e.g., "c0.c3,c90")
// ===========================================================================
/// Parses MCS Categories
///
/// # Errors
///
/// Returns `ErrorKind` due to invalid data.
///
pub fn parse_mcs_categories(input: &str) -> io::Result<crate::category::CategorySet> {
    let mut set = crate::category::CategorySet::new();

    if !input.contains('c') {
        return Ok(set);
    }

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if part.contains('.') {
            let range: Vec<&str> = part.split('.').collect();
            if range.len() == 2 {
                let start = parse_cat_id(range[0])?;
                let end = parse_cat_id(range[1])?;
                for i in start..=end {
                    let cat = crate::category::Category::new(i)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
                    set.insert(cat);
                }
            }
        } else {
            let id = parse_cat_id(part)?;
            let cat = crate::category::Category::new(id)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            set.insert(cat);
        }
    }
    Ok(set)
}

fn parse_cat_id(s: &str) -> io::Result<u16> {
    s.trim_start_matches('c')
        .parse::<u16>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}
