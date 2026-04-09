// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//! # Locale Text
//!
//! [`LocaleText`] is a bilingual text container that maps locale codes
//! (e.g., `"en_US"`, `"fr_CA"`) to translated string values.
//!
//! The type is designed to bridge two JSON representations that appear in the
//! UMRS label catalogs:
//!
//! - **Legacy flat string** — `"name": "Law Enforcement Information"`. Stored
//!   internally as `{"en_US": value}` so all access paths remain uniform.
//! - **Locale object** — `"name": {"en_US": "...", "fr_CA": "..."}`. Stored
//!   as-is.
//!
//! The custom [`serde::Deserialize`] implementation handles both forms and
//! fails closed: if the JSON value is neither a string nor an object with
//! string values, deserialization returns an error rather than silently
//! producing an empty or partial value.
//!
//! ## Key Exported Types
//!
//! - [`LocaleText`] — bilingual text container
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — label text must be
//!   accurate and consistent in all locale renderings; this type enforces
//!   that fidelity at the deserialization boundary.
//! - **NIST SP 800-53 SI-10**: Information Input Validation — the custom
//!   deserializer validates that locale values are strings, rejecting
//!   malformed inputs at the trust boundary before they influence any
//!   security label display path.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — markings rendered from
//!   this type appear in audit-visible output; correctness of the underlying
//!   locale text is a prerequisite for audit record integrity.

use std::collections::HashMap;
use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

// ===========================================================================
// LocaleText
// ===========================================================================

/// Bilingual text container with locale-keyed string values.
///
/// Stores one translated string per locale code (e.g., `"en_US"`, `"fr_CA"`).
/// The canonical English accessor is [`en`](LocaleText::en) and the canonical
/// Canadian French accessor is [`fr`](LocaleText::fr).
///
/// Deserialization handles both a legacy flat string (stored as `en_US` only)
/// and a locale-keyed object. Serialization always writes the locale object
/// form to enable round-trip fidelity.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Label text accuracy at the deserialization boundary.
/// - **NIST SP 800-53 SI-10**: Input validation — non-string locale values are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocaleText {
    inner: HashMap<String, String>,
}

impl LocaleText {
    /// Construct a `LocaleText` with a single English (`en_US`) value.
    ///
    /// This constructor is provided for tests and programmatic construction.
    /// Catalog data should arrive via the `Deserialize` implementation.
    #[must_use = "the constructed LocaleText should be used or stored"]
    pub fn from_en(value: impl Into<String>) -> Self {
        let mut inner = HashMap::with_capacity(1);
        inner.insert("en_US".to_string(), value.into());
        Self {
            inner,
        }
    }

    /// Construct a `LocaleText` from individual locale strings.
    #[must_use = "the constructed LocaleText should be used or stored"]
    pub fn from_parts(en_us: impl Into<String>, fr_ca: impl Into<String>) -> Self {
        let mut inner = HashMap::with_capacity(2);
        inner.insert("en_US".to_string(), en_us.into());
        inner.insert("fr_CA".to_string(), fr_ca.into());
        Self {
            inner,
        }
    }

    /// Returns the English (`en_US`) text, or the first available value, or
    /// an empty string if no values are present.
    ///
    /// This is the primary accessor for English-locale display paths.
    #[must_use = "locale text is consumed by display and audit paths; use the return value"]
    pub fn en(&self) -> &str {
        self.inner
            .get("en_US")
            .map(String::as_str)
            .or_else(|| self.inner.values().next().map(String::as_str))
            .unwrap_or("")
    }

    /// Returns the Canadian French (`fr_CA`) text, or an empty string if
    /// no French translation is present.
    #[must_use = "locale text is consumed by display and audit paths; use the return value"]
    pub fn fr(&self) -> &str {
        self.inner.get("fr_CA").map_or("", String::as_str)
    }

    /// Returns the text for the given locale code, or `None` if absent.
    ///
    /// Use this for dynamic locale selection (e.g., from a `--locale` flag).
    #[must_use = "returns None if the locale is absent; check before use"]
    pub fn get(&self, locale: &str) -> Option<&str> {
        self.inner.get(locale).map(String::as_str)
    }

    /// Returns `true` if no locale entries are present, or all values are empty.
    #[must_use = "check return value to determine if text content is absent"]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty() || self.inner.values().all(String::is_empty)
    }

    /// Returns `true` if the English value (or first available value) is
    /// non-empty after trimming whitespace.
    ///
    /// This mirrors the behaviour of the former `has_description()` predicate
    /// that operated on `String` fields.
    #[must_use = "check return value to determine if displayable text is present"]
    pub fn has_content(&self) -> bool {
        !self.en().trim().is_empty()
    }
}

// ---------------------------------------------------------------------------
// Display — shows the en_US value
// ---------------------------------------------------------------------------

impl fmt::Display for LocaleText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.en())
    }
}

// ---------------------------------------------------------------------------
// Custom Deserialize
// ---------------------------------------------------------------------------

/// Serde visitor for `LocaleText`.
///
/// Accepts either:
/// - A JSON string — wrapped as `{"en_US": value}`.
/// - A JSON object with string values — stored as-is.
///
/// Any other JSON type (number, array, boolean, null) causes deserialization
/// to fail closed with an informative error.
struct LocaleTextVisitor;

impl<'de> Visitor<'de> for LocaleTextVisitor {
    type Value = LocaleText;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string or a locale-keyed object with string values")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<LocaleText, E> {
        // Flat legacy string — store under en_US.
        let mut inner = HashMap::with_capacity(1);
        inner.insert("en_US".to_string(), value.to_string());
        Ok(LocaleText {
            inner,
        })
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<LocaleText, M::Error> {
        let mut inner: HashMap<String, String> =
            HashMap::with_capacity(access.size_hint().unwrap_or(2));
        while let Some((key, value)) = access.next_entry::<String, String>()? {
            inner.insert(key, value);
        }
        Ok(LocaleText {
            inner,
        })
    }
}

impl<'de> Deserialize<'de> for LocaleText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(LocaleTextVisitor)
    }
}

// ---------------------------------------------------------------------------
// Serialize — always write the locale object form
// ---------------------------------------------------------------------------

impl Serialize for LocaleText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}
