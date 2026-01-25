// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Jamie Adams
//
// Unclassified MLS Reference System Project (UMRS)
// MIT licensed—use, modify, and redistribute per LICENSE.
//
// i18n: Internationalization
//
// Notes:
//   - Uses system gettext via the gettext-rs crate (gettextrs API).
//   - Call init("your-domain") once early in main().

use std::sync::OnceLock;

use gettextrs::{bindtextdomain, dgettext, setlocale, LocaleCategory};

static INIT_LOCALE: OnceLock<()> = OnceLock::new();
static DOMAIN: OnceLock<&'static str> = OnceLock::new();

const DEFAULT_LOCALEDIR: &str = "/usr/share/locale";
const FALLBACK_DOMAIN: &str = "umrs";

pub fn init(domain: &'static str) {
    // Record the domain once. If called multiple times, keep the first.
    let _ = DOMAIN.set(domain);

    // Locale init should only happen once per process.
    INIT_LOCALE.get_or_init(|| {
        // Respect LANG/LC_ALL/LANGUAGE from the environment.
        let _ = setlocale(LocaleCategory::LcAll, "");

        // Bind the chosen domain to the locale directory.
        // Note: this is safe to call even if translations are not installed yet.
        let dom = *DOMAIN.get().unwrap_or(&FALLBACK_DOMAIN);
        let _ = bindtextdomain(dom, DEFAULT_LOCALEDIR);
    });
}

pub fn tr(msgid: &str) -> String {
    // If init() was never called, fall back to a safe default domain.
    // (You’ll still get English msgids unless .mo files are installed.)
    let dom = *DOMAIN.get().unwrap_or(&FALLBACK_DOMAIN);

    // dgettext returns an owned String in gettextrs.
    dgettext(dom, msgid)
}

