// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
//! UMRS Validation Engine
//!
//! Centralized regex-driven syntax validation for UMRS artifacts.
//!
//! Design goals:
//! - Single drop-in module
//! - Enum-addressable pattern registry
//! - Cached regex compilation
//! - Stateless call surface
//!
//! Example:
//!
//! ```rust
//! use umrs_core::validate::{is_valid, UmrsPattern};
//!
//! assert!(is_valid(UmrsPattern::Email, "user@agency.gov"));
//! assert!(is_valid(UmrsPattern::CuiMarking, "CUI//LEI"));
//! assert!(!is_valid(UmrsPattern::RgbHex, "blue"));
//! ```
//!
//! This engine performs syntax validation only.
//! Semantic validation should layer above this module.
//

use regex::Regex;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Canonical validation pattern registry.
///
/// Add new patterns here as UMRS validation expands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UmrsPattern {
    // -------------------------------
    // Generic data formats
    // -------------------------------
    Email,
    RgbHex,
    SafeString,

    // -------------------------------
    // CUI / Markings
    // -------------------------------
    CuiMarking,

    // -------------------------------
    // SELinux / MLS
    // -------------------------------
    SelinuxContext,
    MlsRange,
}

/// Pattern → regex mapping.
impl UmrsPattern {
    pub fn regex(&self) -> &'static str {
        match self {
            // ---------------------------
            // Generic
            // ---------------------------
            UmrsPattern::Email =>
                r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$",

            UmrsPattern::RgbHex =>
                r"^#([A-Fa-f0-9]{6})$",

            // NSA-style sanitized string concept
            // Printable ASCII, no control chars
            UmrsPattern::SafeString =>
                r"^[\x20-\x7E]+$",

            // ---------------------------
            // CUI
            // ---------------------------
            UmrsPattern::CuiMarking =>
                r"^CUI(//[A-Z]+)+$",

            // ---------------------------
            // SELinux
            // ---------------------------
            UmrsPattern::SelinuxContext =>
                r"^[^:]+:[^:]+:[^:]+:[^:]+$",

            UmrsPattern::MlsRange =>
                r"^s\d(:c\d+(,c\d+)*)?$",
        }
    }
}

//
// Regex cache
//

static REGEX_CACHE: OnceLock<Mutex<HashMap<UmrsPattern, Regex>>> =
    OnceLock::new();

fn get_regex(kind: UmrsPattern) -> Regex {
    let cache = REGEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();

    if let Some(re) = map.get(&kind) {
        return re.clone();
    }

    let compiled = Regex::new(kind.regex())
        .expect("Invalid validation regex");

    map.insert(kind, compiled.clone());
    compiled
}

/// Validate input against a registered UMRS pattern.
///
/// # Parameters
///
/// - `kind` → Pattern class
/// - `input` → String to validate
///
/// # Returns
///
/// `true` if input matches pattern, otherwise `false`.
///
/// # Example
///
/// ```rust
/// is_valid(UmrsPattern::Email, "user@agency.gov");
/// ```
pub fn is_valid(kind: UmrsPattern, input: &str) -> bool {
    let re = get_regex(kind);
    re.is_match(input)
}

