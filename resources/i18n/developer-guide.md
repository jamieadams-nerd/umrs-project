# UMRS i18n Developer Integration Guide

How to wire gettext-based i18n into a UMRS CLI binary using the `umrs-core::i18n` module.

The dual-mode architecture provides:

- `init(domain)` — binds the CLI's own text domain at startup
- `tr(msgid)` — translates a string via the CLI's domain
- `tr_core(msgid)` — translates a string via the `umrs-core` library domain

---

## Step 1 — Decide the CLI domain name

Pick the gettext domain for the tool. Convention is to use the crate name.

Examples:

```
umrs-ls
vaultmgr
umrs-state
```

---

## Step 2 — Initialize i18n at program start

In `<crate>/src/main.rs`, add before any output:

```rust
use umrs_core::i18n;

fn main() {
    i18n::init("vaultmgr");

    println!("{}", i18n::tr("Vault manager starting"));
}
```

---

## Step 3 — Use translations in CLI code

Anywhere in the CLI:

```rust
println!("{}", i18n::tr("Creating vault directory"));
println!("{}", i18n::tr("Vault created successfully"));
println!("{}", i18n::tr("Operation failed"));
```

These resolve from:

```
/usr/share/locale/<lang>/LC_MESSAGES/vaultmgr.mo
```

---

## Step 4 — Library messages (automatic)

Inside `umrs-core` modules, use:

```rust
use crate::i18n::tr_core;

tr_core("Audit event emitted")
```

Resolves from:

```
/usr/share/locale/<lang>/LC_MESSAGES/umrs-core.mo
```

The CLI's earlier `init()` + `ensure_locale()` covers this automatically.

---

## Runtime behavior

```
LANG=fr_CA.UTF-8 umrs-ls
```

| Call        | Domain    | Catalog        |
|-------------|-----------|----------------|
| `tr()`      | umrs-ls   | umrs-ls.mo     |
| `tr_core()` | umrs-core | umrs-core.mo   |

Both resolve simultaneously with no domain collision.

---

## Step 5 — Install catalogs

```
/usr/share/locale/fr_CA/LC_MESSAGES/umrs-ls.mo
/usr/share/locale/fr_CA/LC_MESSAGES/umrs-core.mo
```

If a catalog is missing → graceful fallback to the English msgid.

---

## Strings that must NOT be translated

**Never wrap these in `tr()` or `tr_core()`:**

- `log::debug!()` — debug strings are for developers, not users. They must stay
  in English so that upstream bug reports, log correlation, and tooling integrations
  remain consistent regardless of the operator's locale.
- `log::trace!()` — same rule as debug.
- `log::info!()`, `log::warn!()`, `log::error!()` — log strings are part of the
  audit trail and are ingested by SIEMs and log aggregators. Translating them breaks
  log parsing, correlation rules, and regex-based alerting. Keep them in English.
- Panic messages and `assert!()` strings — these are diagnostic artifacts, not user UI.
- Internal error variant names and debug representations (`{:?}` output).

**Rule summary**: Only translate strings that are displayed directly to an interactive
user in a terminal UI or report. Log output, debug output, and structured data are
always English.

---

## Important operational rule

**Only the CLI calls `i18n::init()`.** Libraries never call it.

- Prevents domain override
- Prevents locale mutation from library side
- Keeps library embedding safe

---

## Optional ergonomic alias (CLI side)

```rust
use umrs_core::i18n::tr;

println!("{}", tr("Vault created"));
```

---

## Summary

| Task                   | Location             |
|------------------------|----------------------|
| Initialize domain      | CLI `main()`         |
| Translate CLI text     | `tr()`               |
| Translate library text | `tr_core()`          |
| Locale safety          | `ensure_locale()`    |
| Catalog install path   | `/usr/share/locale`  |

See `domains.md` for the domain assignment table. See `vocabulary-*.md` for locale-specific vocabulary decisions.

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
