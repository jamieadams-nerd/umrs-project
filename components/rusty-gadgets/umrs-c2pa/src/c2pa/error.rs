// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA Error Types
//!
//! Defines [`InspectError`], the unified error type for all fallible operations
//! in the `umrs-c2pa` library. Each variant corresponds to a distinct failure
//! class, enabling callers to match, log, and escalate errors programmatically
//! without parsing message strings.
//!
//! ## Display and i18n
//!
//! [`InspectError`] implements [`std::fmt::Display`] manually rather than via
//! thiserror's `#[error]` macro. thiserror bakes string literals into the
//! generated code at compile time, making them invisible to gettext at runtime.
//! A manual `impl Display` allows each arm's message to be passed through
//! `gettext()` (or equivalent) when the i18n layer is wired in — without any
//! further structural changes to this file.
//!
//! ## Key Exported Types
//!
//! - [`InspectError`] — all errors produced by this library
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — error variants carry
//!   structured context (e.g., `UnsafeAlgorithm(String)` names the offending
//!   algorithm) so audit consumers can act on findings without string parsing.
//! - **NIST SP 800-53 SI-10**: Information Input Validation — `UnsafeAlgorithm`
//!   and `AlreadySigned` variants enforce fail-closed policy at the type level.
//! - **NIST SP 800-53 SI-11**: Error Handling — error messages do not expose
//!   key material, signing credentials, or classified content.

use std::fmt;

use umrs_core::i18n;

/// All errors produced by the UMRS c2pa library.
#[derive(Debug)]
pub enum InspectError {
    Io(std::io::Error),
    C2pa(c2pa::Error),
    Config(String),
    Signing(String),
    Hash(String),
    UnsafeAlgorithm(String),
    AlreadySigned(String),
}

impl fmt::Display for InspectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Each arm uses a plain string literal so gettext() can be inserted here
        // later without any structural change. The dynamic values are formatted
        // with standard Rust interpolation after the translatable prefix.
        match self {
            Self::Io(e) => write!(f, "{} {e}", i18n::tr("IO error:")),
            Self::C2pa(e) => write!(f, "{} {e}", i18n::tr("C2PA error:")),
            Self::Config(msg) => write!(f, "{} {msg}", i18n::tr("Config error:")),
            Self::Signing(msg) => write!(f, "{} {msg}", i18n::tr("Signing error:")),
            Self::Hash(msg) => write!(f, "{} {msg}", i18n::tr("Hash error:")),
            Self::UnsafeAlgorithm(alg) => {
                write!(
                    f,
                    "'{alg}' {}",
                    i18n::tr("is not in the FIPS-safe allowed set")
                )
            }
            Self::AlreadySigned(path) => {
                write!(
                    f,
                    "{} {path}",
                    i18n::tr("Refusing to overwrite previously signed file:")
                )
            }
        }
    }
}

impl std::error::Error for InspectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::C2pa(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for InspectError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<c2pa::Error> for InspectError {
    fn from(e: c2pa::Error) -> Self {
        Self::C2pa(e)
    }
}
