# UMRS i18n / l10n Architecture Report

**Date:** 2026-03-25
**Author:** umrs-translator agent ("Simone")
**Status:** Research complete — for review by Jamie and tech-writer

---

## 1. Terminology Guide

This section is the canonical reference for how all UMRS agents and documentation should use
the terms i18n and l10n. These are not interchangeable and should not be treated as a
stylistic pair.

### 1.1 Internationalization (i18n)

**Definition:** The engineering work that makes software capable of supporting multiple
languages and locales without code changes. The "18" is the count of letters between "i" and
"n" in "internationalization."

**What it covers in UMRS:**
- The `umrs-core::i18n` module (`gettextrs` bindings, `init()`, `tr()`)
- The `gettext-rs` crate dependency and its workspace-wide selection
- The `xtr` extraction toolchain and the `.pot` → `.po` → `.mo` pipeline
- The Makefile `i18n-*` targets (`i18n-extract`, `i18n-merge`, `i18n-compile`, `i18n-check`)
- The text domain strategy (per-crate domain names, `bindtextdomain`, split-domain architecture)
- The `UMRS_LOCALEDIR` environment variable and runtime catalog resolution
- Locale environment variable selection (`LC_ALL`, `LC_MESSAGES`, `LANG`, `LANGUAGE`)
- The `resources/i18n/` directory and its layout
- String wrapping discipline in source (`i18n::tr()` calls, wrapping reports)
- The `OnceLock`-guarded locale initialization pattern in `i18n.rs`

**What it does NOT cover:**
- The content of any translation (that is l10n)
- The JSON label catalogs (those are a separate system — see Section 1.3)

### 1.2 Localization (l10n)

**Definition:** The work of adapting software for a specific locale: translating strings,
applying locale-appropriate formatting, and ensuring that terminology is accurate for the
target audience. The "10" is the count of letters between "l" and "n" in "localization."

**What it covers in UMRS:**
- The French Canadian (`fr_CA`) translation work: `.po` file content and decisions
- Terminology research via the GNU corpus (the `french-lookup` skill) and Termium Plus
- The `vocabulary-fr_CA.md` term registry and UMRS-specific term decisions
- PROTÉGÉ A / PROTÉGÉ B / PROTÉGÉ C as official TBS French terminology for Canadian Protected
  markings (source: Treasury Board Secretariat French-language policy documents — these terms
  come from the TBS corpus, not Termium)
- Gender agreement, plural forms, typography rules (guillemets, non-breaking space before colon)
- Width constraints for the TUI key column (150% rule, 66-character help text limit)
- The five-eyes locale coverage: `fr_CA`, `en_GB`, `en_AU`, `en_NZ` (all localized variants of `en_US` base)
- Date, time, and number formatting appropriate for the target locale (see Gap 6 below)

**What it does NOT cover:**
- The gettext pipeline machinery (that is i18n)
- The JSON label catalogs (separate system — see Section 1.3)

### 1.3 JSON Label Catalogs (distinct from gettext)

UMRS uses two separate systems that should not be confused:

| System | Purpose | Technology | Location |
|---|---|---|---|
| gettext catalogs | UI string translation (operator-facing labels, tab names, column headers) | `.pot` / `.po` / `.mo` | `resources/i18n/<domain>/` |
| JSON label catalogs | CUI category metadata, NISPOM category definitions, label palette | Rust `serde_json` | `umrs-labels` crate (`src/cui/catalog.rs`) |

The JSON catalogs are not translated strings. They are structured data representing security
classification schemas. They are loaded, validated, and displayed — but the display layer that
renders them to an operator is where gettext translation applies, if at all.

If Canadian Protected labels (PROTÉGÉ A/B/C) are displayed in a TUI, the display strings
("PROTÉGÉ A", "PROTÉGÉ B", "PROTÉGÉ C") come from the JSON catalog data, not from gettext.
These are designators, not prose — they follow flag terminology rules (retained as-is, no
gettext wrapping). The prose around them (e.g., "Niveau de classification :", "Marquage :")
is what gettext translates.

### 1.4 LOCALE Environment Variables — Priority Chain

When `setlocale(LcAll, "")` is called (as in `umrs-core::i18n::init`), glibc resolves the
locale in the following priority order:

```
LANGUAGE > LC_ALL > LC_MESSAGES > LANG > (compiled default)
```

**Practical meaning for UMRS operators:**

| Variable | Role | UMRS Relevance |
|---|---|---|
| `LANGUAGE` | Colon-separated list of preferred languages; overrides all others | Rarely set in server environments; glibc extension, not POSIX |
| `LC_ALL` | Forces all locale categories to one value; overrides everything else | The recommended way to force `fr_CA.UTF-8` for testing |
| `LC_MESSAGES` | Controls message catalog selection only; does not affect date/number formatting | Can be set independently to translate messages while keeping system locale for formatting |
| `LANG` | Sets default locale for all categories not overridden by a specific `LC_*` | Typical systemwide setting on a deployed RHEL 10 system |
| `UMRS_LOCALEDIR` | UMRS-specific override for catalog search path (not a glibc variable) | Required for development and CI testing; maps to `bindtextdomain` path |

**Development testing invocation (from `i18n.rs` doc comment):**
```bash
UMRS_LOCALEDIR=resources/i18n/umrs-uname LANG=fr_CA.UTF-8 cargo run -p umrs-uname
```

For RHEL 10 deployment, `.mo` files are installed to `/usr/share/locale/<locale>/LC_MESSAGES/<domain>.mo`
and `UMRS_LOCALEDIR` is not set; glibc resolves from `LANG` or `LC_MESSAGES`.

---

## 2. Current State Assessment

### 2.1 i18n Infrastructure

The infrastructure is in good shape and ahead of many open-source Rust projects:

- **Crate:** `gettext-rs` (0.7.7) confirmed in `umrs-core/Cargo.toml`. Used across all
  binary crates via `umrs-core::i18n`. Workspace-consistent.
- **Extraction tool:** `xtr` specified in Makefile but status unknown — it was not installed
  as of 2026-03-10. Must be verified before any extraction run.
- **Module:** `umrs-core/src/i18n.rs` provides `init(domain)` and `tr(msgid)`, with
  `OnceLock`-guarded locale initialization. Fallback-safe: returns `msgid` unchanged when no
  catalog is found.
- **Domain strategy:** Split-domain model is documented in `resources/i18n/domains.md`.
  Library crates get their own domain; binary crates bind their own domain (library strings
  are translated in the binary's `.po` at runtime via `dgettext`).
- **Makefile:** Full pipeline defined — `i18n-extract-<domain>`, `i18n-merge-<domain>`,
  `i18n-compile-<domain>`, `i18n-check`. Parameterized per domain via `I18N_DOMAIN_RULES` macro.
- **`.mo` layout:** Binary `.mo` files correctly placed at
  `resources/i18n/<domain>/<locale>/LC_MESSAGES/<domain>.mo`, which is the standard gettext
  directory structure that `bindtextdomain` expects.

**Active domains with fr_CA translations compiled:**

| Domain | .pot | fr_CA.po | fr_CA.mo | In Makefile |
|---|---|---|---|---|
| `umrs-ls` | yes | yes | yes | yes |
| `umrs-state` | yes | yes | yes | yes |
| `umrs-logspace` | yes | yes | yes | yes |
| `umrs-uname` | yes | yes | yes | yes |
| `umrs-platform` | yes | yes | yes (display.rs scope) | yes |

### 2.2 Workspace Members and i18n Coverage

The workspace has 10 members (excluding `xtask`). Coverage:

| Crate | Type | Uses i18n::tr? | Domain directory | Notes |
|---|---|---|---|---|
| `umrs-core` | library | no (provides it) | none (correct) | Source of `init()` and `tr()` |
| `umrs-ls` | binary | yes | yes | Full pipeline active |
| `umrs-state` | binary | not found | yes | Domain dir exists; need to verify init call |
| `umrs-logspace` | binary | not found | yes | Domain dir exists; need to verify init call |
| `umrs-uname` | binary | yes | yes | Full pipeline active; open developer items |
| `umrs-stat` | binary | yes | **NO** | Gap: calls `i18n::init("umrs-stat")` with no domain directory |
| `umrs-platform` | library | yes | yes | display.rs scope only; log strings out of scope |
| `umrs-ui` | library | yes | none | Gap: `data_panel.rs` and `header.rs` use `i18n::tr()` but no domain |
| `umrs-labels` | library | no | none | Not yet onboarded |
| `umrs-hw` | unknown | unknown | none | Not yet assessed |
| `umrs-selinux` | library | unknown | none | Not yet assessed |
| `umrs-cui` | binary? | unknown | none | Not yet assessed |

---

## 3. Gaps Identified

### Gap 1 — umrs-stat: domain directory missing (blocking)

`umrs-stat/src/main.rs:748` calls `i18n::init("umrs-stat")`. The domain `umrs-stat` has no
directory under `resources/i18n/`, no `.pot`, no `.po`, and no `.mo`. The binary will fall
back to returning msgids (English) for all fr_CA operators.

The Makefile does not include `umrs-stat` in `I18N_TEXT_DOMAINS` or `I18N_ACTIVE_DOMAINS`.

**Required action (translator):** Onboard `umrs-stat` to the pipeline. Create the domain
directory, run `xtr`, initialize `fr_CA.po`, translate, compile.

**Required action (Makefile):** Add `umrs-stat` to `I18N_TEXT_DOMAINS`, `I18N_ACTIVE_DOMAINS`,
define `I18N_SRC_DIR_umrs_stat`, and `I18N_ACTIVE_LOCALES_umrs_stat`.

### Gap 2 — umrs-ui: library strings use i18n::tr() with no domain binding

`umrs-ui/src/data_panel.rs` and `umrs-ui/src/header.rs` import and call `i18n::tr()`, but
`umrs-ui` has no text domain of its own and is not registered in `domains.md`.

This is architecturally ambiguous. Two valid approaches:

**Option A (current implicit behavior):** `umrs-ui` strings are resolved in the calling
binary's domain. The binary's `i18n::init("umrs-stat")` or `i18n::init("umrs-uname")` sets
the active domain, so `dgettext` in `tr()` uses the binary's domain. This means UI strings
must be present in every binary's `.po` file. The "Recommended" msgid in
`umrs-uname/fr_CA.po` is an example of this approach already in use.

**Option B (own domain):** Give `umrs-ui` its own domain `umrs-ui`, and calls from
`umrs-ui` functions use `dgettext("umrs-ui", msgid)` via a `tr_ui()` wrapper in
`umrs-core::i18n`. The binary's domain handles its own strings; `umrs-ui` handles its own.

Option A has lower overhead (fewer catalogs, simpler binding). Option B is cleaner for
reuse across many binaries. **Decision is Jamie's / developer's.** The current code drifts
toward Option A, and the "Recommended" translation precedent is already committed under Option A.

Recommendation: document the decision in `domains.md` and add `umrs-ui` to the domain table
even if Option A is chosen ("resolved in calling binary's domain, no own domain").

### Gap 3 — umrs-stat strings are not wrapped (candidate audit required)

`umrs-stat/src/main.rs` has significant `i18n::tr()` usage on UI-facing strings (Path,
Filename, File type, MIME type, Size, Mode, Immutable, IMA/EVM, SELinux Context, etc.). A
wrapping audit has not been run. Before the `.pot` can be produced for `umrs-stat`, a
complete scan of `main.rs` must identify all unwrapped strings.

This needs a wrapping report.

### Gap 4 — umrs-uname open developer items (from 2026-03-23 rescan)

Per the rescan report (`resources/i18n/reports/2026-03-23-umrs-uname-rescan.md`):
- Items 1–2: `report_name()` and `report_subject()` return `&'static str`; the trait must
  return `String` before wrapping is possible. Developer action pending.
- Items 4–8: os-release field names (VERSION, ID, NAME, etc.) — developer decides whether
  these are translated or retained as technical identifiers.
- Items 12–15: format strings requiring template substitution refactor. Developer action pending.

### Gap 5 — No locale documentation in operator or deployment docs

The locale resolution chain (`LANGUAGE > LC_ALL > LC_MESSAGES > LANG > UMRS_LOCALEDIR`),
the `.mo` installation path, and the fr_CA testing invocation are documented only in
`umrs-core/src/i18n.rs` as a Rust doc comment. This needs to surface in:
- The developer guide (how to test a translation during development)
- The deployment guide (how `.mo` files are installed on RHEL 10, what systemd locale
  settings affect UMRS output)

**Action:** Tech-writer to add an i18n/l10n section to the developer guide. Translator will
provide the content draft.

### Gap 6 — No locale-aware date/time or number formatting

UMRS currently formats dates and numbers using Rust's `Display` trait (UTC timestamps, byte
counts via `humansize`). Neither is locale-aware:

- French Canadian date format is `DD/MM/YYYY` or ISO 8601 (`YYYY-MM-DD`) — both differ from
  US conventions.
- Decimal separator in fr_CA is a comma, not a period (`1 234,56` not `1,234.56`).
- Thousands separator in fr_CA is a non-breaking space.

`humansize` produces English unit strings (`GB`, `MB`). The gettext pipeline translates
label strings but does not affect number formatting. This is a genuine l10n gap.

**Short-term:** Acceptable to keep English number formatting if byte counts are kept as
pure numbers with gettext-translated unit labels. Flag this in the developer guide as a
known limitation.

**Medium-term:** Evaluate whether `icu_locid` / `icu_datetime` (from the `icu4x` crate)
is warranted. This is a supply chain decision requiring Jamie's direction — `icu4x` is
a significant dependency. Do not introduce it without an explicit architecture decision.

### Gap 7 — PROTÉGÉ A/B/C terminology not in vocabulary-fr_CA.md

The vocabulary file does not have an entry for Canadian Protected markings. These terms
appear in `umrs-labels` (CUI/Protected label catalog) and will eventually surface in TUI
display strings. The rule:

- "PROTÉGÉ A", "PROTÉGÉ B", "PROTÉGÉ C" — these are designators from TBS policy documents,
  not Termium translations. They are retained exactly as TBS defines them, including the
  accent. They are never run through gettext.
- Prose around them ("Marquage :", "Niveau :") is translatable via gettext in the normal way.
- "Protected A / Protected B / Protected C" (English) → "Protégé A / Protégé B / Protégé C"
  (French) in prose context is the TBS canonical form.

**Action:** Add a "Canadian Protected Markings" section to `vocabulary-fr_CA.md` before
any l10n work begins on `umrs-labels`.

### Gap 8 — umrs-labels, umrs-hw, umrs-selinux, umrs-cui not assessed

These crates have not been scanned for user-facing strings or onboarded to the i18n pipeline.
Assessment is deferred until Jamie confirms scope.

---

## 4. Recommended Architecture (Full Pipeline)

The architecture is already sound. What follows is the canonical description of how it should
work, filling in the parts that are not yet documented.

### 4.1 Directory Layout

```
resources/i18n/
├── domains.md                        Domain registry (maintained by translator)
├── vocabulary-fr_CA.md               Term decisions (maintained by translator)
├── vocabulary-en_GB.md               British English spelling decisions
├── vocabulary-en_AU.md               Australian English spelling decisions
├── vocabulary-en_NZ.md               New Zealand English spelling decisions
├── reports/                          Wrapping reports and audit outputs
└── <domain>/                         One directory per text domain
    ├── <domain>.pot                  Extracted message template (source of truth)
    ├── fr_CA.po                      Canadian French translation
    ├── en_GB.po                      British English variant
    ├── en_AU.po                      Australian English variant
    ├── en_NZ.po                      New Zealand English variant
    └── <locale>/
        └── LC_MESSAGES/
            └── <domain>.mo           Compiled binary catalog (runtime artifact)
```

The `.mo` files under `<locale>/LC_MESSAGES/` are what `bindtextdomain` and glibc load at
runtime. The `fr_CA.mo` files at the domain root are artifacts of an earlier compilation
approach and should be considered obsolete — the canonical compiled output is always
inside `<locale>/LC_MESSAGES/`.

### 4.2 The .pot → .po → .mo Lifecycle

```
Source (.rs files)
       │
       ▼  xtr --package-name <domain> --output <domain>.pot <src files>
.pot (template)
       │
       ├── msginit -l fr_CA --input <domain>.pot --output fr_CA.po   (first time only)
       │
       └── msgmerge --update fr_CA.po <domain>.pot                   (after source changes)
                │
                ▼  translator fills msgstr values in fr_CA.po
         fr_CA.po (translated)
                │
                ▼  msgfmt --check fr_CA.po   (validate)
                │
                ▼  msgfmt -o fr_CA/LC_MESSAGES/<domain>.mo fr_CA.po   (compile)
         <domain>.mo (runtime binary catalog)
```

`make i18n-extract-<domain>` runs `xtr`.
`make i18n-merge-<domain>` runs `msgmerge`.
`make i18n-compile-<domain>` runs `msgfmt`.
`make i18n-check` runs `msgfmt --check` on all active `.po` files.

### 4.3 Runtime Catalog Resolution

At process startup, the binary calls `i18n::init("umrs-<toolname>")`. This:

1. Calls `setlocale(LcAll, "")` — reads `LC_ALL`, `LC_MESSAGES`, or `LANG` from the environment.
2. Reads `UMRS_LOCALEDIR` (if set) for development override; otherwise uses `/usr/share/locale`.
3. Calls `bindtextdomain("umrs-<toolname>", <localedir>)` — tells glibc where to find catalogs.

glibc then resolves to: `<localedir>/<locale>/LC_MESSAGES/umrs-<toolname>.mo`

For production RHEL 10 deployment, `.mo` files are installed to:
`/usr/share/locale/fr_CA/LC_MESSAGES/umrs-<toolname>.mo`

For development testing:
```bash
UMRS_LOCALEDIR=resources/i18n/umrs-uname LANG=fr_CA.UTF-8 cargo run -p umrs-uname
```

### 4.4 Per-Crate Domain Binding (Library Strings)

Library crates (`umrs-platform`, `umrs-ui`) use `i18n::tr()` which calls `dgettext` with the
domain set by the most recent `init()` call. This means library strings are resolved in the
**calling binary's domain**.

Consequence: when a library string (e.g., "Recommended" from `umrs-ui`) needs translation,
its msgid/msgstr pair must be present in the binary's `.po` file (e.g., `umrs-uname/fr_CA.po`),
not in a library-specific catalog.

This is the chosen architecture. It is simpler than per-library domains and appropriate for
the current number of binaries. If UMRS ships many binaries that all share heavy UI library
content, a `dgettext("umrs-ui", msgid)` pattern via a `tr_ui()` wrapper should be revisited.

### 4.5 PROTÉGÉ Marking Display Rule

When displaying Canadian Protected markings:
- The designator itself ("PROTÉGÉ A") is data from the label catalog, not a gettext msgid.
- Do not wrap designators in `i18n::tr()`.
- Surrounding prose labels (column headers, field names) are wrapped normally.
- If a prose description of the marking level is ever added (e.g., "Information dont la
  divulgation non autorisée pourrait..."), that description string is a gettext candidate.

---

## 5. Action Items

### Translator (Simone) — immediate

1. Add "Canadian Protected Markings" section to `vocabulary-fr_CA.md` covering PROTÉGÉ A/B/C
   terminology rules before any `umrs-labels` l10n work begins.

2. Run a wrapping scan on `umrs-stat/src/main.rs` and produce a wrapping report to
   `resources/i18n/reports/2026-03-25-umrs-stat-unwrapped.md`. This is a prerequisite for
   onboarding `umrs-stat` to the pipeline.

3. Add `umrs-ui` domain status to `domains.md` — document the Option A decision (library
   strings resolved in calling binary's domain, no own domain for `umrs-ui`) once confirmed.

4. Draft a "locale and l10n" section for the developer guide (hand off to tech-writer).
   Content: locale resolution chain, `UMRS_LOCALEDIR` pattern, development testing invocation,
   RHEL 10 `.mo` installation path.

### Developer — pending

1. Resolve `umrs-uname` open items from 2026-03-23 rescan: `report_name()`/`report_subject()`
   return type change, os-release field decisions, format string refactors.

2. Onboard `umrs-stat` to the Makefile i18n targets after the domain directory is created:
   add to `I18N_TEXT_DOMAINS`, `I18N_ACTIVE_DOMAINS`, define `I18N_SRC_DIR_umrs_stat`,
   `I18N_ACTIVE_LOCALES_umrs_stat := fr_CA`.

3. Confirm umrs-ui Option A vs Option B decision so the translator can document it correctly.

### Jamie — decision required

1. **umrs-ui domain decision:** Option A (library strings in calling binary's domain, no own
   domain) vs Option B (umrs-ui gets its own domain and `tr_ui()` wrapper). Current implicit
   behavior is Option A.

2. **Number/date locale formatting:** Confirm whether `fr_CA` number formatting (comma decimal,
   non-breaking-space thousands separator) is in scope. If yes, an architecture decision on
   `icu4x` or a lightweight alternative is needed before any implementation starts.

3. **Unassessed crates:** Confirm which of `umrs-labels`, `umrs-hw`, `umrs-selinux`, `umrs-cui`
   should be onboarded to the i18n pipeline and in what order.

4. **Obsolete `.mo` artifacts:** The root-level `.mo` files (e.g.,
   `resources/i18n/umrs-ls/fr_CA.mo`) predate the `<locale>/LC_MESSAGES/` layout. Confirm
   they can be removed. They are not used at runtime.

### Tech-writer

1. Add i18n/l10n terminology section to the developer guide, using the definitions in
   Section 1 of this report as the authoritative source.

2. Add locale deployment section to the operator/installation guide: how `.mo` files are
   installed on RHEL 10, what `LANG` setting produces French output, the `UMRS_LOCALEDIR`
   development override.

---

## 6. What UMRS Is Doing Well

- The crate selection (`gettext-rs`) is appropriate: it is the standard Rust gettext binding,
  uses system libintl on RHEL (avoiding a bundled copy), is workspace-consistent, and has no
  problematic transitive dependencies.

- The `OnceLock`-guarded locale initialization is correct and thread-safe. Fallback-on-failure
  is correctly implemented (msgid returned unchanged if no catalog).

- The Makefile pipeline is well-structured and parameterized. Adding a new domain requires
  exactly four lines.

- The `UMRS_LOCALEDIR` escape hatch for development is a sound design. It allows catalog
  testing without system-level installation.

- The vocabulary file is rigorous: every decision is documented with source attribution and
  rationale. The corpus-first lookup protocol ensures consistency with established GNU
  francophone conventions.

- The visual verification checklist (150% length rule, 66-char help text limit, mandatory
  post-compile check) addresses a real TUI layout risk that purely textual translation review
  misses.

- The split-domain strategy (library crates get domains, binary crates bind multiple domains)
  is the correct architectural choice for a multi-crate workspace.
