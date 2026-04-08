// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
//! # CUI Marking Validation
//!
//! Regex-driven syntax validation for CUI marking strings as defined by the
//! NARA CUI registry.
//!
//! This module owns the `CuiMarking` validation pattern. Each `CuiPattern`
//! variant has its own `static OnceLock<Regex>` — one atomic load after first
//! initialisation, no `Mutex`, no `HashMap` clone. This follows the
//! `OnceLock<T>` per-variant pattern appropriate when the key set is bounded
//! and enumerable at compile time.
//!
//! ## Key Exported Types
//!
//! - [`CuiPattern`] — enum of CUI-specific validation patterns
//! - [`is_valid`] — validate an input string against a `CuiPattern`
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-10**: Information Input Validation — all CUI marking
//!   strings entering the system are validated against the canonical pattern
//!   before use.
//! - **NIST SP 800-53 AC-16**: Security Attributes — only syntactically valid
//!   markings are accepted; malformed strings are rejected fail-closed.
//! - **CMMC AC.L2-3.1.3**: Control CUI flow in accordance with approved
//!   authorizations.

use regex::Regex;
use std::sync::OnceLock;

/// CUI-specific validation pattern registry.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation
/// - **NIST SP 800-53 AC-16**: Security Attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CuiPattern {
    /// Validates a CUI regulatory marking string.
    ///
    /// Accepts the full NARA banner syntax:
    ///
    /// ```text
    /// CUI
    /// CUI//NOFORN
    /// CUI//FED ONLY
    /// CUI//CATEGORY
    /// CUI//SP-CATEGORY
    /// CUI//CAT1/CAT2
    /// CUI//SP-CAT1/CAT2
    /// CUI//CAT1//LDC
    /// CUI//SP-CTI/EXPT//NOFORN
    /// CUI//LEI//FED ONLY
    /// CUI//SP-CTI//REL TO USA, CAN, GBR
    /// ```
    ///
    /// Structure: `CUI` optionally followed by `//CATEGORIES` where categories
    /// are `(SP-)?[A-Z][-A-Z]*` separated by single `/`, then optionally
    /// followed by `//LDC` where the LDC portion permits spaces and commas
    /// (needed for `FED ONLY`, `REL TO USA, CAN`).
    ///
    /// All category tokens must be uppercase ASCII. The SP- prefix is permitted
    /// on any category. Plain `CUI` with no categories is valid. Bare `CUI`
    /// with a direct LDC and no category (`CUI//NOFORN`) is also valid per
    /// 32 CFR Part 2002.
    CuiMarking,
}

impl CuiPattern {
    /// Return the compiled regex pattern string for this variant.
    #[must_use = "the regex string is required to compile the pattern"]
    pub const fn regex(self) -> &'static str {
        match self {
            // Pattern anatomy (two alternatives after CUI):
            //   ^CUI                        — literal prefix
            //   (                           — optional block (one of two alternatives):
            //     //                        — double-slash separator
            //     (SP-)?[A-Z][-A-Z]*        — first category, SP- prefix optional
            //     (/(SP-)?[A-Z][-A-Z]*)*    — additional categories, single-slash
            //     (//[A-Z][-A-Z /,]*)?      — optional LDC block (spaces/commas)
            //   |                           — OR: bare CUI with direct LDC, no categories
            //     //[A-Z][-A-Z /,]*         — LDC directly (e.g., CUI//NOFORN, CUI//FED ONLY)
            //   )?$
            Self::CuiMarking => {
                r"^CUI(//(SP-)?[A-Z][-A-Z]*(/(SP-)?[A-Z][-A-Z]*)*(//[A-Z][A-Z ,\-]*)?|//[A-Z][A-Z ,\-]*)?$"
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Regex cache — one OnceLock per variant; no Mutex, no HashMap
// ---------------------------------------------------------------------------

static CUI_MARKING_RE: OnceLock<Regex> = OnceLock::new();

/// Return a reference to the compiled `Regex` for `kind`.
///
/// After the first call for a given variant the regex is cached in a
/// `static OnceLock<Regex>`. Subsequent calls are a single atomic load with
/// no lock acquisition and no allocation.
fn get_regex(kind: CuiPattern) -> &'static Regex {
    match kind {
        CuiPattern::CuiMarking => CUI_MARKING_RE.get_or_init(|| {
            // The pattern literal is authored at compile time and is
            // known-valid. A panic here indicates a programmer error, not a
            // runtime condition.
            Regex::new(kind.regex()).expect("CuiPattern::CuiMarking regex failed to compile")
        }),
    }
}

/// Validate `input` against a registered `CuiPattern`.
///
/// Returns `true` if `input` matches the pattern; `false` otherwise.
/// This is a syntax-only check — semantic validation (e.g., whether the
/// marking is in the catalog) must be performed separately.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation — fail-closed;
///   any string that does not match is rejected.
#[must_use = "the validation result must be checked; ignoring it defeats the purpose"]
pub fn is_valid(kind: CuiPattern, input: &str) -> bool {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    let result = get_regex(kind).is_match(input);

    #[cfg(debug_assertions)]
    log::debug!(
        "CUI validation pattern {:?} completed in {} µs — result: {}",
        kind,
        start.elapsed().as_micros(),
        result
    );

    result
}
