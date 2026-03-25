---
name: i18n pipeline state
description: Confirmed crate, toolchain, domain assignments, and active translation status as of 2026-03-25
type: project
---

## i18n Crate

`gettext-rs` 0.7.7 confirmed in `umrs-core/Cargo.toml` and `Cargo.lock`.
Uses system `libintl` via `gettext-sys`. Correct choice for RHEL 10 high-assurance deployment.
Do not switch to pure-Rust `gettext` crate or Mozilla `fluent`.

## Entry Point

`umrs-core::i18n` module at `umrs-core/src/i18n.rs`.
Public API: `init(domain: &'static str)` and `tr(msgid: &str) -> String`.
`tr()` calls `dgettext(dom, msgid)` — no context-qualified lookup.

**Missing function:** `tr_ctx(ctx: &str, msgid: &str) -> String` using `dcpgettext()`.
Required for msgctxt / security label policy. Developer action pending.

## Extraction Tool

`xtr` — Rust-aware xgettext companion. Install: `cargo install xtr`.
Status as of 2026-03-10: NOT CONFIRMED INSTALLED. Must verify before any extraction run.
If xtr does not recognize i18n::tr() calls, add `--keyword i18n::tr` flag.

Standard invocation:
```
xtr --package-name <domain> --output resources/i18n/<domain>/<domain>.pot <src files>
```

## Active Domains (fr_CA translation present and compiled)

| Domain | .pot | fr_CA.po | .mo location |
|---|---|---|---|
| umrs-ls | yes | yes (8 msgids, 100% complete) | fr_CA/LC_MESSAGES/umrs-ls.mo |
| umrs-state | yes | yes (8 msgids, 100% complete) | fr_CA/LC_MESSAGES/umrs-state.mo |
| umrs-logspace | yes | yes (5 msgids, 100% complete) | fr_CA/LC_MESSAGES/umrs-logspace.mo |
| umrs-platform | yes | yes (39 msgids, 100% complete) | fr_CA/LC_MESSAGES/umrs-platform.mo |
| umrs-uname | yes (hand-crafted 2026-03-23) | yes (98 msgids, ~85% complete — help text empty) | fr_CA/LC_MESSAGES/umrs-uname.mo |

## Root-level .mo files

Root-level `.mo` files (e.g., `umrs-ls/fr_CA.mo`) are obsolete artifacts from an earlier
compilation approach. They are NOT loaded at runtime. Cleanup pending Jamie authorization.

## .mo Runtime Resolution

`bindtextdomain("domain", localedir)` resolves to:
`<localedir>/<locale>/LC_MESSAGES/<domain>.mo`

Production: `/usr/share/locale/fr_CA/LC_MESSAGES/<domain>.mo`
Development: `UMRS_LOCALEDIR=resources/i18n/<domain> LANG=fr_CA.UTF-8 cargo run -p <crate>`

## Ghost Domains (empty placeholder files)

`umrs-df`, `umrs-ps`, `umrs-tester` — directories exist, all files are 0 bytes.
These crates are not in the workspace. Status (planned vs abandoned) pending Jamie decision.

## fr_FR Status

`umrs-ls/fr_FR.po` is non-empty (translated, identical to fr_CA for that crate's strings).
Other domains have empty `fr_FR.po` placeholders.
`fr_FR` is NOT in `domains.md` as a supported locale. Decision on fr_FR scope pending Jamie.

**Why:** The two locales share all strings in the current umrs-ls set. Divergence will
appear when government-specific terms or policy-accurate vocabulary begins to differ.
