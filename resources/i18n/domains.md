# UMRS i18n Text Domain Assignments

Maintained by the `umrs-translator` agent. One row per crate. Updated whenever a new crate
is onboarded to the i18n pipeline.

---

## Domain Assignment Table

| Crate | Crate Type | Text Domain | Domain Directory | Notes |
|---|---|---|---|---|
| `umrs-ls` | binary | `umrs-ls` | `resources/i18n/umrs-ls/` | First binary crate in pipeline; uses `gettextrs` |

---

## Supported Locales

| Locale | Description | Vocabulary Reference |
|--------|-------------|----------------------|
| `en_US` | American English (base/fallback) | — (msgid is en_US) |
| `fr_CA` | Canadian French | `vocabulary-fr_CA.md` |
| `en_GB` | British English | `vocabulary-en_GB.md` |
| `en_AU` | Australian English | `vocabulary-en_AU.md` |
| `en_NZ` | New Zealand English | `vocabulary-en_NZ.md` |

All Five Eyes nations are represented. `en_US` is the msgid baseline; the three
Commonwealth English variants (en_GB, en_AU, en_NZ) differ primarily in spelling
conventions per their respective government style guides.

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
- Target locales: `fr_CA` (Canadian French), `en_GB`, `en_AU`, `en_NZ` (Commonwealth English variants)
