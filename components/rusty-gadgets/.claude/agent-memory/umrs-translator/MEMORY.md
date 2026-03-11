# umrs-translator Agent Memory

## i18n Crate

- **Crate**: `gettextrs = "0.7.7"` (pinned in `umrs-core/Cargo.toml`)
- **Extraction tool**: `xtr` (Rust-aware; `cargo install xtr`)
- Consistent across workspace — one crate, one approach

## Domain Assignments

| Crate | Type | Domain | Directory |
|---|---|---|---|
| `umrs-ls` | binary | `umrs-ls` | `resources/i18n/umrs-ls/` |
| `umrs-core` | library | `umrs-core` | `resources/i18n/umrs-core/` |
| `umrs-platform` | library | `umrs-platform` | `resources/i18n/umrs-platform/` |

Full registry: `resources/i18n/domains.md`

## umrs-core i18n Pattern (canonical reference)

File: `components/rusty-gadgets/umrs-core/src/i18n.rs`

- Two public functions: `init(domain: &'static str)` and `tr(msgid: &str) -> String`
- Library-scoped function: `tr_core(msgid)` — uses `dgettext("umrs-core", msgid)`
- `OnceLock<()>` for locale init guard; `OnceLock<&'static str>` for domain
- `DEFAULT_LOCALEDIR = "/usr/share/locale"`; `FALLBACK_DOMAIN = "umrs"`
- No `build.rs` in `umrs-core` — `.mo` compilation is a manual pipeline step

For `umrs-platform`, the library-scoped function should be named `tr_platform()`
with `dgettext("umrs-platform", msgid)`.

## xtr Invocation Pattern

```bash
xtr --package-name umrs-platform \
    --output resources/i18n/umrs-platform/umrs-platform.pot \
    components/rusty-gadgets/umrs-platform/src/**/*.rs
```

Run from repo root. Adjust glob expansion per shell. Requires `xtr` to be
installed and `gettextrs` wrapping to be applied first (xtr finds wrapped calls).

## thiserror Wrapping Decision

`#[error("...")]` attributes on `thiserror`-derived enums cannot be wrapped
with `tr_platform()` at the annotation site. Agreed approach: leave `#[error]`
strings in English; wrap at the binary display boundary where errors are surfaced
to users. Record these strings in the `.pot` for translation inventory purposes.

## Scope Decision Pending

Before library log string wrapping is implemented, the developer and architect
must confirm: should `warn!`/`error!` strings in library crates be wrapped at
runtime (adds `gettextrs` lookup per call) or only at the binary display layer?
See `resources/i18n/reports/2026-03-11-umrs-platform-unwrapped.md`.

## Locales Supported

- `en_US` — msgid baseline (American English)
- `fr_CA` — Canadian French (primary translation target)
- `en_GB`, `en_AU`, `en_NZ` — Commonwealth English variants

## Key Files

- `resources/i18n/domains.md` — domain registry
- `resources/i18n/vocabulary-fr_CA.md` — UMRS-specific French term decisions
- `resources/i18n/reports/` — wrapping reports (developer work instructions)
- `resources/i18n/umrs-ls/umrs-ls.pot` — reference .pot for format/style
