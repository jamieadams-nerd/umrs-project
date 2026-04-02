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
//! ## Locale directory resolution
//!
//! The catalog directory defaults to `/usr/share/locale` (the system standard).
//! For development and testing, set `UMRS_LOCALEDIR` to override:
//!
//! ```bash
//! UMRS_LOCALEDIR=resources/i18n/umrs-uname LANG=fr_CA.UTF-8 cargo run -p umrs-uname
//! ```
//!
//! `UMRS_LOCALEDIR` is on the environment scrub allowlist defined in the
//! `umrs-core::init` tool initialization plan (sub-phase 1e). When the
//! `SanitizedEnv` pipeline is implemented, this variable will be read from
//! the validated environment snapshot rather than raw `std::env`.
//!
//! ## Non-goals
//!
//! - Full gettext or ICU-style localization frameworks
//! - Runtime translation loading from external sources
//

use gettextrs::{LocaleCategory, bindtextdomain, dgettext, setlocale};
use std::sync::OnceLock;

static INIT_LOCALE: OnceLock<()> = OnceLock::new();
static DOMAIN: OnceLock<&'static str> = OnceLock::new();

const DEFAULT_LOCALEDIR: &str = "/usr/share/locale";
const LOCALEDIR_ENV: &str = "UMRS_LOCALEDIR";
const FALLBACK_DOMAIN: &str = "umrs";

/// Ensure locale subsystem is initialized.
///
/// This provides a safety net for library consumers that do not explicitly
/// call `init()`. It initializes process locale once without rebinding
/// translation domains.
///
/// Behavior:
/// - No effect if `init()` already executed.
/// - Does not override caller domain bindings.
/// - Enables gettext catalog resolution.
fn ensure_locale() {
    INIT_LOCALE.get_or_init(|| {
        let _ = setlocale(LocaleCategory::LcAll, "");
    });
}

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
/// # Locale directory
///
/// If `UMRS_LOCALEDIR` is set, its value is used as the catalog search
/// path instead of the compiled-in default (`/usr/share/locale`). This
/// allows development-time testing without installing `.mo` files
/// system-wide. When the `SanitizedEnv` pipeline (umrs-tool-init plan,
/// sub-phase 1e) is implemented, this read will be replaced by a
/// validated accessor.
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
        let locale_dir =
            std::env::var(LOCALEDIR_ENV).unwrap_or_else(|_| DEFAULT_LOCALEDIR.to_string());
        let _ = bindtextdomain(dom, &locale_dir);
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
    ensure_locale();

    let dom = *DOMAIN.get().unwrap_or(&FALLBACK_DOMAIN);

    dgettext(dom, msgid)
}
