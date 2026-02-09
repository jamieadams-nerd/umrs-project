// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! UMRS Core Internationalization Utilities
//!
//! Minimal localization helpers for UMRS user-facing messages.
//!
//! Guarantees:
//! - Deterministic mapping of message keys to localized strings
//! - No side effects beyond returning formatted message text
//! - Stable API surface for CLI and console integration
//!
//! Non-goals:
//! - Dynamic locale discovery or environment probing
//! - Full gettext or ICU-style localization frameworks
//! - Runtime translation loading from external sources
//

use gettextrs::{LocaleCategory, bindtextdomain, dgettext, setlocale};
use std::sync::OnceLock;

static INIT_LOCALE: OnceLock<()> = OnceLock::new();
static DOMAIN: OnceLock<&'static str> = OnceLock::new();

const DEFAULT_LOCALEDIR: &str = "/usr/share/locale";
const FALLBACK_DOMAIN: &str = "umrs";

/// Initialize the UMRS internationalization subsystem.
///
/// Registers the gettext domain used for subsequent message lookups and
/// configures the runtime translation catalog path.
///
/// # Parameters
///
/// - `domain`: Static gettext domain name (e.g., `"umrs-tool"`).  
///   This must remain valid for the lifetime of the process.
///
/// # Behavior
///
/// - Sets the active translation domain for all future `tr()` calls.
/// - Attempts to load locale data from the configured resource directory.
/// - If no catalog is found, translation calls will gracefully fall back
///   to returning the original message identifiers.
///
/// # Side Effects
///
/// - Modifies global translation state.
/// - Affects all subsequent internationalized output.
///
/// # Panics
///
/// This function does not intentionally panic.  
/// Failures to locate or load locale data are handled internally and result
/// in fallback behavior rather than process termination.
pub fn init(domain: &'static str) {
    let _ = DOMAIN.set(domain);

    INIT_LOCALE.get_or_init(|| {
        let _ = setlocale(LocaleCategory::LcAll, "");

        let dom = *DOMAIN.get().unwrap_or(&FALLBACK_DOMAIN);
        let _ = bindtextdomain(dom, DEFAULT_LOCALEDIR);
    });
}

/// Translate a message identifier using the active UMRS locale catalog.
///
/// Looks up the provided message identifier in the currently initialized
/// gettext catalog and returns the localized string if available. If no
/// translation is found or if the i18n subsystem has not been initialized,
/// this function gracefully falls back to returning the original identifier.
///
/// # Parameters
///
/// - `msgid`: Message identifier key to translate.  
///   This should be a stable, human-readable English string used as the
///   canonical lookup key in translation catalogs.
///
/// # Returns
///
/// A localized string corresponding to `msgid` if a translation is available;  
/// otherwise, the original `msgid` value as a `String`.
///
/// # Behavior
///
/// - Performs a catalog lookup using the currently active gettext domain.
/// - Returns a UTF-8 owned string suitable for display or logging.
/// - Falls back to returning `msgid` unchanged if no translation exists.
///
/// # Side Effects
///
/// - None.  
///   This function does not modify global state or perform I/O.
///
/// # Panics
///
/// This function does not intentionally panic.  
/// All lookup failures and uninitialized-state conditions are handled
/// gracefully via fallback behavior.
pub fn tr(msgid: &str) -> String {
    let dom = *DOMAIN.get().unwrap_or(&FALLBACK_DOMAIN);

    dgettext(dom, msgid)
}
