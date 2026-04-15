# String Inventory Report — umrs-ls
**Date:** 2026-04-15
**Crate:** umrs-ls
**Domain:** umrs-ls
**Type:** binary

## Summary

8 unique msgids. All are wrapped with `i18n::tr()`. Existing translations in
`resources/i18n/umrs-ls/fr_CA.po` are complete and correct.

**Action taken:** `.pot` updated with corrected line numbers (2026-04-15).
`fr_CA.po` updated with corrected line numbers and dates.

## Source Files Scanned
- `src/main.rs`

## Makefile Status
- Already in `I18N_ACTIVE_DOMAINS` ✓
- `I18N_SRC_DIR_umrs_ls` defined ✓
- `I18N_ACTIVE_LOCALES_umrs_ls := fr_CA fr_FR en_GB en_AU en_NZ` ✓

## Line Number Changes (from 2026-03-11 to 2026-04-15)

| msgid | Old line | New line |
|-------|----------|----------|
| "access denied" | 175 | 458 |
| "MODE" | 257 | 1459 |
| "MARKING" | 258 | 1460 |
| "OWNER:GROUP" | 259 | 1461 |
| "SIZE" | 260 | 1462 |
| "MODIFIED" | 261 | 1463 |
| "NAME" | 262 | 1464 |
| "<restricted>" | 467 | 1656 |

These changes are cosmetic — msgfmt uses msgid matching, not line numbers.
The `.po` translations are valid regardless of line number drift.

## fr_CA Coverage: 8/8 (100%)

## Policy-Critical Terms — Flagged for Henri
None.
