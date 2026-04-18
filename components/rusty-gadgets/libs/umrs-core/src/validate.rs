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
//! - Cached regex compilation (`OnceLock<Regex>` per variant — lock-free warm path)
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
use std::sync::OnceLock;

/// Generic validation pattern registry for UMRS core data formats.
///
/// Domain-specific patterns (CUI markings, SELinux contexts, MLS ranges)
/// live in their respective crates.
///
/// ## Variants:
///
/// - `Email` — RFC 5322 simplified email address.
/// - `RgbHex` — CSS-style RGB hex color (`#RRGGBB`).
/// - `SafeString` — Printable ASCII string with no control characters.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UmrsPattern {
    Email,
    RgbHex,
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
// Regex cache — one OnceLock<Regex> per variant
//
// Warm-path cost: one atomic load per call — no Mutex, no HashMap lookup,
// no Regex clone. Each variant is compiled at most once across the process
// lifetime. Thread safety is provided by OnceLock's internal Once.
//
// NIST SP 800-53 SI-10: compiled patterns are immutable once initialised;
// callers cannot substitute a different pattern after first use.
// ---------------------------------------------------------------------------

static RE_EMAIL: OnceLock<Regex> = OnceLock::new();
static RE_RGB_HEX: OnceLock<Regex> = OnceLock::new();
static RE_SAFE_STRING: OnceLock<Regex> = OnceLock::new();

/// Return a reference to the compiled `Regex` for `kind`.
///
/// On first call per variant the pattern is compiled and stored; subsequent
/// calls perform only an atomic load. The `expect` is unreachable in practice:
/// all pattern strings are compile-time literals validated by the test suite.
fn get_regex(kind: UmrsPattern) -> &'static Regex {
    let cell = match kind {
        UmrsPattern::Email => &RE_EMAIL,
        UmrsPattern::RgbHex => &RE_RGB_HEX,
        UmrsPattern::SafeString => &RE_SAFE_STRING,
    };
    cell.get_or_init(|| Regex::new(kind.regex()).expect("UmrsPattern regex failed to compile"))
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
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = get_regex(kind).is_match(input);

    #[cfg(debug_assertions)]
    log::debug!(
        "is_valid pattern={kind:?} result={result} elapsed={}µs",
        t0.elapsed().as_micros()
    );

    result
}
