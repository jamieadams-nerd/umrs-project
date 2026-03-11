# UMRS i18n Text Domain Assignments

Maintained by the `umrs-translator` agent. One row per crate. Updated whenever a new crate
is onboarded to the i18n pipeline.

---

## Domain Assignment Table

| Crate | Crate Type | Text Domain | Domain Directory | Notes |
|---|---|---|---|---|
| `umrs-ls` | binary | `umrs-ls` | `resources/i18n/umrs-ls/` | First binary crate in pipeline; uses `gettextrs` |

---

## Domain Strategy

**Library crates** (`umrs-selinux`, `umrs-core`, etc.) — each gets its own text domain named
after the crate. Library strings are extracted separately so they remain reusable across
multiple binaries.

**Binary crates** (CLI tools) — each binary gets its own text domain. Binaries bind their
own domain plus the domains of the libraries they consume.

---

## Toolchain

- Extraction: `xtr` (Rust-aware; install with `cargo install xtr`)
- Init: `msginit`
- Merge: `msgmerge`
- Validate: `msgfmt --check`
- Compile: `msgfmt`
- Target locale: `fr_CA` (Canadian French)
