# String Inventory Report — umrs-stat
**Date:** 2026-04-15
**Crate:** umrs-stat
**Domain:** umrs-stat
**Type:** binary

## Summary

24 unique msgids across 2 source files (main.rs, lib.rs). All wrapped with `i18n::tr()`.

## Source Files Scanned
- `src/main.rs`
- `src/lib.rs`

## Makefile Status
- `umrs-stat` is NOT in `I18N_TEXT_DOMAINS` or `I18N_ACTIVE_DOMAINS`
- No `I18N_SRC_DIR_umrs_stat` defined
- No `I18N_ACTIVE_LOCALES_umrs_stat` defined
- Directory `resources/i18n/umrs-stat/` does not exist

**Makefile changes needed (not performed — propose for developer):**
```makefile
# In I18N_TEXT_DOMAINS:
I18N_TEXT_DOMAINS   := umrs-logspace umrs-ps umrs-df umrs-ls umrs-state umrs-uname umrs-platform umrs-c2pa umrs-stat

# In I18N_ACTIVE_DOMAINS:
I18N_ACTIVE_DOMAINS := umrs-ls umrs-state umrs-logspace umrs-uname umrs-platform umrs-c2pa umrs-stat

# New per-domain entries:
I18N_SRC_DIR_umrs_stat     := components/rusty-gadgets/umrs-stat/src
I18N_ACTIVE_LOCALES_umrs_stat := fr_CA
```

## String Inventory

### src/main.rs
| Line | msgid | Notes |
|------|-------|-------|
| 78 | `File Security Audit` | Page/report title |

### src/lib.rs
| Line | msgid | Notes |
|------|-------|-------|
| 133 | `No security concerns` | Status message |
| 282 | `Path` | Row label |
| 284 | `Filename` | Row label |
| 288 | `File type` | Row label |
| 309 | `MIME type` | Row label |
| 329 | `Size` | Row label — corpus: "Taille" |
| 389 | `Marking` | Section group title fragment |
| 406 | `Encryption` | Section group title fragment — Termium: "Chiffrement" |
| 461 | `IMA/EVM` | Section group title — specification acronym, keep English |
| 534 | `Count` | Column header |
| 541 | `Findings` | Column/section header |
| 542 | `No security observations` | Status message |
| 569 | `Failed to read file` | Error message |
| 578 | `No data available` | Status message |
| 662/706 | `Identity` | Tab label — Termium: "Identité" |
| 663/707 | `Security` | Tab label — Termium: "Sécurité" |
| 664/708 | `Observations` | Tab label |
| 718 | `Failed to read file attributes` | Error message |

## Policy-Critical Terms — Flagged for Henri
None — no Canadian government classification markings in this crate's strings.
