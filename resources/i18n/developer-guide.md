# UMRS i18n Developer Integration Guide

How to wire gettext-based i18n into a UMRS CLI binary using the `umrs-core::i18n` module.

The architecture is intentionally simple:

- `init(domain)` — binds the CLI's own text domain at startup
- `tr(msgid)` — translates a string via the CLI's domain

**Rule: only tool binaries (CLI crates) translate strings. Library crates return typed
values and structured errors — they never call `tr()` or `init()`.**

---

## Step 1 — Decide the CLI domain name

Pick the gettext domain for the tool. Convention is the crate name.

Examples:

```
umrs-ls
umrs-state
umrs-logspace
```

---

## Step 2 — Initialize i18n at program start

In `<crate>/src/main.rs`, add before any output:

```rust
use umrs_core::i18n;

fn main() {
    i18n::init("umrs-state");

    println!("{}", i18n::tr("Starting umrs-state"));
}
```

---

## Step 3 — Wrap user-facing strings with `tr()`

Wrap any string that appears directly in terminal output:

```rust
println!("{}", i18n::tr("Creating vault directory"));
println!("{}", i18n::tr("Vault created successfully"));
eprintln!("{}", i18n::tr("Operation failed"));
```

These resolve from:

```
/usr/share/locale/<lang>/LC_MESSAGES/<domain>.mo
```

For strings with runtime values, format after translation:

```rust
// Correct — translate the template, then format
let msg = format!("{} {path}", i18n::tr("State file not found:"));

// Wrong — the format string is never extracted
let msg = i18n::tr(&format!("State file {path} not found"));
```

---

## Step 4 — Strings that must NOT be translated

**Never wrap these in `tr()`:**

- `log::debug!()`, `log::trace!()` — developer-facing; must stay English for log correlation
- `log::info!()`, `log::warn!()`, `log::error!()` — audit trail strings ingested by SIEMs;
  translating them breaks log parsing and regex-based alerting
- `panic!()`, `.expect()`, `assert!()` — diagnostic artifacts, not user UI
- Internal error variant names and `{:?}` debug representations

**Rule summary**: Only translate strings displayed directly to an interactive user in a
terminal or report. Log output, audit output, and structured data are always English.

---

## Step 5 — Add to the i18n pipeline (Makefile)

Add three lines to the Makefile i18n configuration block:

```makefile
I18N_ACTIVE_DOMAINS := umrs-ls umrs-state umrs-logspace   # add domain here

I18N_SRC_DIR_umrs_state    := components/rusty-gadgets/umrs-state/src
I18N_SRC_DIR_umrs_logspace := components/rusty-gadgets/umrs-logspace/src

I18N_ACTIVE_LOCALES_umrs_state    := fr_CA
I18N_ACTIVE_LOCALES_umrs_logspace := fr_CA
```

Then run:

```bash
make i18n-setup          # scaffold domain directory if new
make i18n-extract-<domain>   # generate .pot
# msginit to create initial .po if fr_CA.po does not exist
make i18n-merge-<domain>     # merge .pot into .po
make i18n-compile-<domain>   # compile .mo
```

---

## Step 6 — Install catalogs

```
/usr/share/locale/fr_CA/LC_MESSAGES/umrs-ls.mo
/usr/share/locale/fr_CA/LC_MESSAGES/umrs-state.mo
/usr/share/locale/fr_CA/LC_MESSAGES/umrs-logspace.mo
```

If a catalog is missing → graceful fallback to the English msgid.

---

## Runtime behavior

```
LANG=fr_CA.UTF-8 umrs-ls
```

| Call   | Domain   | Catalog    |
|--------|----------|------------|
| `tr()` | umrs-ls  | umrs-ls.mo |

Each tool has exactly one domain. Library code (umrs-core, umrs-selinux, umrs-platform)
produces no translated output.

---

## Summary

| Task                  | Location            |
|-----------------------|---------------------|
| Initialize domain     | CLI `main()`        |
| Translate CLI text    | `tr()`              |
| Locale initialization | `ensure_locale()`   |
| Catalog install path  | `/usr/share/locale` |

See `domains.md` for the domain assignment table. See `vocabulary-*.md` for locale-specific
vocabulary decisions.

---

## Translator workflow: RAG queries

When making a translation decision for a term that is:
- A technical security or OS concept (SELinux, MLS, FIPS, kernel attributes)
- A NIST control or framework term
- A Five Eyes classification or government term

Use the `rag-query` skill (Claude Code skill) to search the UMRS reference library
before committing a translation. The RAG library contains authoritative NIST SPs,
STIGs, and related standards — searching it ensures translations align with how
the source standards express the concept.

Example workflow for a new French term:
1. Run `/rag-query <English term>` to check if the term appears in the reference corpus
2. Note how the source standard expresses the concept
3. Consult the GNU francophone corpus via `/french-lookup <term>` for established translations
4. Record the decision in the appropriate `vocabulary-*.md` file with its source

For Commonwealth English variants (en_GB, en_AU, en_NZ), RAG queries are less
critical since the differences are spelling conventions rather than concept translations.
Use the `vocabulary-*.md` spelling tables as the primary reference for these locales.
