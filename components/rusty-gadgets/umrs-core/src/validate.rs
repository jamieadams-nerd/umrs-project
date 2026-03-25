// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
//! # UMRS Core Validation Engine
//!
//! Regex-driven syntax validation for generic data formats used across
//! UMRS tools.
//!
//! This module owns patterns for general-purpose formats: email addresses,
//! RGB hex colors, and safe printable strings. Domain-specific patterns
//! live with their owning crates:
//!
//! - CUI marking patterns → `umrs-labels::validate`
//! - SELinux context and MLS range patterns → `umrs-selinux::validate`
//!
//! ## Design
//!
//! - Single drop-in module
//! - Enum-addressable pattern registry
//! - Cached regex compilation (`OnceLock<Mutex<HashMap>>`)
//! - Stateless call surface
//!
//! ## Key Exported Types
//!
//! - [`UmrsPattern`] — enum of generic validation patterns
//! - [`is_valid`] — validate an input string against a `UmrsPattern`
//!
//! ## Example
//!
//! ```rust
//! use umrs_core::validate::{is_valid, UmrsPattern};
//!
//! assert!(is_valid(UmrsPattern::Email, "user@agency.gov"));
//! assert!(!is_valid(UmrsPattern::RgbHex, "blue"));
//! assert!(is_valid(UmrsPattern::SafeString, "hello world"));
//! ```
//!
//! This engine performs syntax validation only.
//! Semantic validation should layer above this module.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-10**: Information Input Validation — all input
//!   strings are validated against registered patterns before use.
//! - **NSA RTB RAIN**: Non-Bypassability — callers cannot bypass validation
//!   without explicitly ignoring the return value, which `#[must_use]` flags.

use regex::Regex;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Generic validation pattern registry for UMRS core data formats.
///
/// Domain-specific patterns (CUI markings, SELinux contexts, MLS ranges)
/// live in their respective crates.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UmrsPattern {
    /// RFC 5322 simplified email address.
    Email,
    /// CSS-style RGB hex color (`#RRGGBB`).
    RgbHex,
    /// Printable ASCII string with no control characters.
    SafeString,
}

impl UmrsPattern {
    /// Return the regex pattern string for this variant.
    #[must_use = "the regex string is required to compile the pattern"]
    pub const fn regex(self) -> &'static str {
        match self {
            Self::Email => r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$",
            Self::RgbHex => r"^#([A-Fa-f0-9]{6})$",
            // Printable ASCII only; no control characters.
            Self::SafeString => r"^[\x20-\x7E]+$",
        }
    }
}

// ---------------------------------------------------------------------------
// Regex cache
// ---------------------------------------------------------------------------

static REGEX_CACHE: OnceLock<Mutex<HashMap<UmrsPattern, Regex>>> =
    OnceLock::new();

fn get_regex(kind: UmrsPattern) -> Regex {
    let cache = REGEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    let mut map =
        cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);

    if let Some(re) = map.get(&kind) {
        return re.clone();
    }

    // SAFETY of expect: pattern literals are authored at compile time and
    // are known-valid. A panic here indicates a programmer error, not a
    // runtime condition.
    let compiled =
        Regex::new(kind.regex()).expect("UmrsPattern regex failed to compile");

    map.insert(kind, compiled.clone());
    compiled
}

/// Validate `input` against a registered `UmrsPattern`.
///
/// Returns `true` if `input` matches the pattern; `false` otherwise.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation — fail-closed;
///   any string that does not match is rejected.
#[must_use = "the validation result must be checked; ignoring it defeats the purpose"]
pub fn is_valid(kind: UmrsPattern, input: &str) -> bool {
    let re = get_regex(kind);
    re.is_match(input)
}
