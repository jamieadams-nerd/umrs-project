// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
//! # SELinux Input Validation
//!
//! Regex-driven syntax validation for SELinux security context strings and
//! MLS range notation.
//!
//! This module owns the `SelinuxContext` and `MlsRange` validation patterns.
//! Co-locating these validators with the `umrs-selinux` crate makes the trust
//! boundary explicit: SELinux label syntax is validated at the point closest
//! to the data types that consume it.
//!
//! Each pattern uses the same `OnceLock<Mutex<HashMap>>` regex-cache design
//! established in `umrs-core::validate`, ensuring regexes are compiled once
//! and reused safely across threads.
//!
//! ## Key Exported Types
//!
//! - [`SelinuxPattern`] — enum of SELinux-specific validation patterns
//! - [`is_valid`] — validate an input string against a `SelinuxPattern`
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-10**: Information Input Validation — all SELinux
//!   context and MLS range strings are validated before use in label objects.
//! - **NIST SP 800-53 AC-3**: Access Enforcement — only syntactically valid
//!   labels enter the system; malformed strings are rejected fail-closed.
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — MLS range
//!   syntax is validated as a prerequisite for dominance comparisons.
//! - **NSA RTB RAIN**: Non-Bypassability — validation occurs at construction;
//!   callers cannot construct label types with unvalidated strings.

use regex::Regex;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// SELinux-specific validation pattern registry.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation
/// - **NIST SP 800-53 AC-3**: Access Enforcement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelinuxPattern {
    /// Validates a SELinux security context string.
    ///
    /// Accepted form: three colon separators producing four fields. The first
    /// three fields (user, role, type) may not contain colons. The fourth
    /// field (level) may contain colons, as MLS ranges use colon notation
    /// (e.g., `s0:c0,c5`, `s0-s0:c0.c1023`).
    ///
    /// Example: `"system_u:system_r:httpd_t:s0"`,
    /// `"unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023"`.
    ///
    /// This is a structural check only; it does not verify that the user,
    /// role, type, or level components are defined in the active policy.
    SelinuxContext,

    /// Validates an MLS sensitivity range string.
    ///
    /// Accepted form: `sN` optionally followed by `:cN` category components.
    /// Examples: `"s0"`, `"s0:c0"`, `"s1:c0,c5"`.
    ///
    /// This is a structural check only; it does not verify that the
    /// sensitivity level or categories are within the active policy's bounds.
    MlsRange,
}

impl SelinuxPattern {
    /// Return the compiled regex pattern string for this variant.
    #[must_use = "the regex string is required to compile the pattern"]
    pub const fn regex(self) -> &'static str {
        match self {
            // The level field (4th) can itself contain colons (e.g., s0:c0,c5
            // or s0-s0:c0.c1023), so match it with .+ rather than [^:]+.
            Self::SelinuxContext => r"^[^:]+:[^:]+:[^:]+:.+$",
            Self::MlsRange => r"^s\d+(:c\d+(,c\d+)*)?(-s\d+(:c\d+(,c\d+)*)?)?$",
        }
    }
}

// ---------------------------------------------------------------------------
// Regex cache
// ---------------------------------------------------------------------------

static REGEX_CACHE: OnceLock<Mutex<HashMap<SelinuxPattern, Regex>>> = OnceLock::new();

fn get_regex(kind: SelinuxPattern) -> Regex {
    let cache = REGEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    let mut map = cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);

    if let Some(re) = map.get(&kind) {
        return re.clone();
    }

    // SAFETY of expect: pattern literals are authored at compile time and
    // are known-valid. A panic here indicates a programmer error, not a
    // runtime condition.
    let compiled = Regex::new(kind.regex()).expect("SelinuxPattern regex failed to compile");

    map.insert(kind, compiled.clone());
    compiled
}

/// Validate `input` against a registered `SelinuxPattern`.
///
/// Returns `true` if `input` matches the pattern; `false` otherwise.
/// This is a syntax-only check — semantic validation (e.g., verifying that
/// the context components are defined in the active policy) must be performed
/// separately.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Information Input Validation — fail-closed;
///   any string that does not match is rejected.
/// - **NIST SP 800-53 AC-3**: Access Enforcement — prevents malformed label
///   strings from entering the type system.
#[must_use = "the validation result must be checked; ignoring it defeats the purpose"]
pub fn is_valid(kind: SelinuxPattern, input: &str) -> bool {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    let result = get_regex(kind).is_match(input);

    #[cfg(debug_assertions)]
    log::debug!(
        "SELinux validation pattern {:?} completed in {} µs — result: {}",
        kind,
        start.elapsed().as_micros(),
        result
    );

    result
}
