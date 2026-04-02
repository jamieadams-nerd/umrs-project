---
name: Workspace project structure
description: Key file locations, build commands, and conventions for the rusty-gadgets workspace
type: project
---

## Build commands (all from components/rusty-gadgets/)

- `cargo xtask clippy` — must be zero warnings before any commit
- `cargo xtask test` — full workspace tests
- `cargo xtask fmt` — format
- `cargo test -p umrs-c2pa --test c2pa_tests` — single crate tests

## Known pre-existing test failures (as of 2026-04-02)

- `umrs-label` setrans_tests — 7 failures related to CUI catalog category range mismatches.
  Not a regression to investigate unless Jamie assigns it. Unrelated to umrs-c2pa or umrs-selinux work.

## Key conventions

- `#![forbid(unsafe_code)]` in every crate root — enforced at compile time
- `#[deny(clippy::unwrap_used)]` — use `?`, `map_err`, or explicit match
- All tests in `tests/` — never inline `#[cfg(test)]` modules
- Public functions returning Result/Option/security type: `#[must_use]` with message string
- Module-level `//!` blocks required: purpose, key types, `## Compliance` section
- NIST citations: `NIST SP 800-53` (not `NIST 800-53`)

## crate roots

- `umrs-c2pa/src/lib.rs` and `src/main.rs`
- `umrs-c2pa/src/c2pa/mod.rs` — primary re-export point for library API
- `umrs-selinux` is under `libs/umrs-selinux/`

## i18n pattern (gettext wrapping)

- Binaries use `umrs_core::i18n::tr()` — call `i18n::init("domain-name")` at top of `main()`.
- For plural forms: import `gettextrs::ngettext` directly; add `gettext-rs = { version = "0.7", features = ["gettext-system"] }` to the crate's Cargo.toml.
- `ngettext(singular, plural, n: u32)` — always use `u32::try_from(n).unwrap_or(u32::MAX)` for usize counts.
- `describe_algorithm()` was changed from `&'static str` to `String` return type to support `i18n::tr()`.
- Status tags ([OK], [WARN], [FAIL], [INFO], [SKIP]) and TrustStatus labels (TRUSTED, UNVERIFIED, etc.) stay English — per D3/D4 design decisions in Simone's string inventory.
- `thiserror` is incompatible with gettext — use manual `Display` impl so each arm can call `i18n::tr()`.
- Library code can call `i18n::tr()` directly once `umrs-core` is in the deps; no separate `i18n::init` needed in lib (only in main).
- For `ngettext` pluralization patterns in format strings: `.replace("{}", &n.to_string())` since `ngettext` returns a plain String.
