# umrs-translator Agent Memory — "Simone"
# Alias: Simone. No "The". Named for Simone de Beauvoir.

## i18n Crate
- Confirmed: `gettextrs` (gettext-rs) — workspace-wide
- Extraction tool: `xtr` — `cargo install xtr`
- xtr was NOT installed as of 2026-03-10 — check before extraction runs

## Domain Assignments

| Crate | Type | Domain | Directory |
|---|---|---|---|
| `umrs-ls` | binary | `umrs-ls` | `resources/i18n/umrs-ls/` |
| `umrs-state` | binary | `umrs-state` | `resources/i18n/umrs-state/` |
| `umrs-logspace` | binary | `umrs-logspace` | `resources/i18n/umrs-logspace/` |
| `umrs-uname` | binary | `umrs-uname` | `resources/i18n/umrs-uname/` |
| `umrs-core` | library | `umrs-core` | `resources/i18n/umrs-core/` |
| `umrs-platform` | library | `umrs-platform` | `resources/i18n/umrs-platform/` |

Full registry: `resources/i18n/domains.md`

## Pipeline Status

| Domain | .pot | fr_CA.po | fr_CA.mo | Notes |
|---|---|---|---|---|
| umrs-ls | hand-crafted 2026-03-10 | translated | compiled | re-run xtr when installed |
| umrs-state | xtr 2026-03-10 | translated 2026-03-11 | compiled | |
| umrs-logspace | xtr 2026-03-10 | translated 2026-03-10 | compiled | msgid colon moved to format string |
| umrs-uname | hand-crafted 2026-03-23 | rescan 2026-03-23 | compiled clean | 16 new gaps found; 4 wrapped, 8 need dev refactor |
| umrs-platform | hand-crafted 2026-03-23 | translated 2026-03-23 | NOT compiled | display.rs scope only |

## umrs-uname BUG (open — developer action required)
- `main.rs:1785` calls `i18n::init("umrs-uname")` — CONFIRMED FIXED in current source
- `report_name()` and `report_subject()` return `&'static str` — trait must change to `String` before wrapping
- See rescan report: `resources/i18n/reports/2026-03-23-umrs-uname-rescan.md`

## umrs-uname Rescan (2026-03-23) — Developer Action Pending
16 unwrapped strings found. Status:
- Items 1–3, 9–10, 16: straightforward wraps — msgids/msgstrs added to .po
- Items 4–8 (os-release field names): developer decides translate vs. retain as identifiers
- Items 12–15 (format strings): developer must refactor to template substitution pattern
- Items 1–2 (`report_name`, `report_subject`): trait return type must change `&'static str` → `String`
Report: `resources/i18n/reports/2026-03-23-umrs-uname-rescan.md`

## Key File Paths
- Vocabulary: `resources/i18n/vocabulary-fr_CA.md`
- Domain registry: `resources/i18n/domains.md`
- Makefile: `/media/psf/repos/umrs-project/Makefile`
- Corpus skill: invoke `french-lookup` Skill tool — do NOT grep corpus files directly
- Corpus files: `.claude/references/corpus/*.po` (coreutils, grep, sed, tar, findutils, bash, cryptsetup)

## Cargo.toml Entry (confirmed)
- `gettext-rs = { version = "0.7", features = ["gettext-system"] }`
- Use `gettext()` function; `gettext!()` macro does NOT exist in gettext-rs 0.7
- Primary locale: `fr_CA`

## French Typography Rules
See [french_typography.md](french_typography.md).
- Use « » with interior spaces; never English " "
- Non-breaking space (U+00A0) before colon in French (e.g., "Configuré\u00a0:")
- No guillemets for: acronyms, proper names, code, file paths, commands

## Canonical "translate for [crate]" Workflow
See detailed steps in [workflow.md](workflow.md). Summary:
1. Check xtr installed
2. Confirm domain in domains.md
3. Run french-lookup skill for each primary term
4. Extract strings (xtr or hand-craft .pot)
5. Initialize fr_CA.po (msginit)
6. Translate with UMRS standard header
7. Validate (msgfmt --check) and compile (.mo)
8. Update vocabulary-fr_CA.md
9. Update Makefile (I18N_TEXT_DOMAINS, I18N_SRC_DIR_, I18N_ACTIVE_LOCALES_)

## umrs-core i18n Pattern (canonical reference)
File: `components/rusty-gadgets/libs/umrs-core/src/i18n.rs`
- `init(domain)`, `tr(msgid)`, `tr_core(msgid)` via `dgettext("umrs-core", msgid)`
- `OnceLock` for locale init; `DEFAULT_LOCALEDIR = "/usr/share/locale"`
- For umrs-platform: `tr_platform()` with `dgettext("umrs-platform", msgid)`

## thiserror Wrapping Decision
`#[error("...")]` strings: wrap at binary display boundary, not at annotation site.
These are recorded in .pot for inventory; developer acts at the display call site.

## umrs-stat: MISSING DOMAIN (blocking gap — 2026-03-25)
- `umrs-stat/src/main.rs:748` calls `i18n::init("umrs-stat")` — NO domain directory exists
- No .pot, no .po, no .mo. Operators on fr_CA systems get raw English msgids.
- Not in Makefile I18N_TEXT_DOMAINS or I18N_ACTIVE_DOMAINS
- Action: wrapping scan → wrapping report → onboard to pipeline
- Full gap documentation: `.claude/reports/i18n-l10n-architecture.md` §Gap 1 and §Gap 3

## umrs-ui: No Own Domain (architecture decision pending — 2026-03-25)
- `umrs-ui/src/data_panel.rs` and `umrs-ui/src/header.rs` call `i18n::tr()` directly
- No domain registered in domains.md for umrs-ui
- Current implicit behavior (Option A): library strings resolved in calling binary's domain
- Option B: give umrs-ui its own domain with `tr_ui()` wrapper
- Precedent: "Recommended" (data_panel.rs:694) lives in umrs-uname fr_CA.po under Option A
- Jamie must decide. Do not onboard umrs-ui until decision is made.

## umrs-platform Scope Decision Pending
Should `warn!`/`error!` log strings in library crates be wrapped at runtime?
See `resources/i18n/reports/2026-03-11-umrs-platform-unwrapped.md`.
Current .pot and .po cover display.rs annotation strings only (2026-03-23 scope).

## Visual Verification Rule (established 2026-03-23 — MANDATORY)
After every translation pass, run the checklist at:
`resources/i18n/reports/translation-visual-verification-checklist.md`
Three rules enforced:
1. Key column strings: French must not exceed 150% of English length
2. Help text lines: max 66 chars per line
3. Single-line display strings: max 150% of English length (context exceptions noted in checklist)
Run `make i18n-compile-umrs-uname` last — must exit clean.

## umrs-uname Key Column Fixes (2026-03-23 — live TUI test)
Four keys shortened after live TUI testing showed column jamming:
- "Label Trust" → "Confiance" (was "Confiance de l'étiquette", 25 chars)
- "Trust Tier" → "Palier" (was "Palier de confiance", 19 chars)
- "Downgrade Reasons" → "Motifs" (was "Motifs de déclassement", 22 chars)
- "Evidence Records" → "Preuves" (was "Enregistrements de preuves", 26 chars)

## umrs-ui Library Strings in umrs-uname Domain
"Recommended" (from umrs-ui/src/data_panel.rs:693) lives in the umrs-uname
.po because that is the binary domain at runtime. Translation: "Recommandé"
(Termium Plus attests "recommandé/e" as fr_CA form). Added 2026-03-23.

## Vocabulary Highlights (key decisions — full list in vocabulary-fr_CA.md)
- "enabled/disabled" → "activé/désactivé" — corpus:coreutils confirmed
- "restricted" → "restreint" — corpus:bash confirmed
- "loading" → "chargement" — corpus:bash confirmed
- "hard gate" → "verrou dur" — UMRS decision
- "tampering" → "falsification" — UMRS/ANSSI
- "trust tier" → "palier de confiance" — UMRS decision
- "downgrade (trust)" → "déclassement" — UMRS/ANSSI
- "hardening" → "durcissement" — UMRS/ANSSI
- "drift (config)" → "dérive" — UMRS/ANSSI
- "core dump" → "vidage mémoire" — UMRS decision
- "digest (crypto)" → "condensé" — UMRS/ANSSI/ISO
- "accounting (netfilter)" → "comptabilisation" — UMRS decision (feminine gender)
- "profiling" → "profilage" — UMRS decision
- "indicator (posture)" → "indicateur" — UMRS decision
- "Configured:" prefix → "Configuré\u00a0:" — non-breaking space before colon required
- "OS Detection" (report name) → "Détection OS" — UMRS decision
- "Platform Identity and Integrity" → "Identité et intégrité de plateforme" — UMRS decision
- "readable" (indicators, plural) → "lisibles" — masculine plural, agrees with "indicateurs"
- "hardened" (indicators, plural) → "durcis" — masculine plural
- "update your kernel" → "mettez à jour votre noyau" — formal imperative register

## Width Constraints for Help Text (CRITICAL)
Multi-line TUI help strings have a 66-character line limit. Two flagged items in
umrs-uname/fr_CA.po require developer review:
1. "DURCISSEMENT DU SYSTÈME DE FICHIERS" (group title) — 38 chars, within limit
2. Curated note in build_kernel_security_summary_rows — longest translated line
   is 67 chars. Propose abbreviation if developer confirms hard limit is 66.

## Makefile Targets Added (2026-03-23)
- umrs-uname and umrs-platform added to I18N_TEXT_DOMAINS, I18N_ACTIVE_DOMAINS
- I18N_SRC_DIR_umrs_uname and I18N_SRC_DIR_umrs_platform defined
- I18N_ACTIVE_LOCALES_umrs_uname := fr_CA
- I18N_ACTIVE_LOCALES_umrs_platform := fr_CA
- Per-domain targets generated automatically by I18N_DOMAIN_RULES macro
