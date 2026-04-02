// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
//! # CUI Marking Validation
//!
//! Regex-driven syntax validation for CUI marking strings as defined by the
//! NARA CUI registry.
//!
//! This module owns the `CuiMarking` validation pattern. It follows the same
//! `OnceLock<Mutex<HashMap>>` regex-cache design used in `umrs-core::validate`,
//! keeping each pattern co-located with the data type it validates.
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
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

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
    /// Accepted form: `CUI` optionally followed by one or more `//SEGMENT`
    /// components, where each segment is one or more uppercase ASCII letters.
    ///
    /// Examples: `"CUI"`, `"CUI//LEI"`, `"CUI//SP-CTI"`, `"CUI//LEI/JUV"`.
    CuiMarking,
}

impl CuiPattern {
    /// Return the compiled regex pattern string for this variant.
    #[must_use = "the regex string is required to compile the pattern"]
    pub const fn regex(self) -> &'static str {
        match self {
            Self::CuiMarking => r"^CUI(//[A-Z][-A-Z]*)(/[A-Z][-A-Z]*)*$",
        }
    }
}

// ---------------------------------------------------------------------------
// Regex cache
// ---------------------------------------------------------------------------

static REGEX_CACHE: OnceLock<Mutex<HashMap<CuiPattern, Regex>>> = OnceLock::new();

fn get_regex(kind: CuiPattern) -> Regex {
    let cache = REGEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    let mut map = cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);

    if let Some(re) = map.get(&kind) {
        return re.clone();
    }

    // SAFETY of expect: the pattern literals are authored at compile time and
    // are known-valid. A panic here indicates a programmer error, not a
    // runtime condition.
    let compiled = Regex::new(kind.regex()).expect("CuiPattern regex failed to compile");

    map.insert(kind, compiled.clone());
    compiled
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
