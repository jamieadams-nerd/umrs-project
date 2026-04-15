# String Inventory Report — umrs-uname (2026-04-15 update)
**Date:** 2026-04-15
**Crate:** umrs-uname
**Domain:** umrs-uname
**Type:** binary

## Status

Existing `.po` was nearly complete from March 2026 pass. Three gaps found and fixed
in this session.

## Gaps Found and Fixed

| msgid | Status | Action |
|-------|--------|--------|
| `"configuration/kernel disagreements detected"` | MISSING from .pot and .po | Added to both |
| `"vs"` | MISSING from .pot and .po | Added to both |
| `"baseline"` | MISSING from .pot and .po | Added to both |
| `"is newer than catalog baseline"` | MISSING from .pot and .po | Added to both |
| `"is older than catalog baseline"` | MISSING from .pot and .po | Added to both |
| `"some indicators may have changed"` | MISSING from .pot and .po | Added to both |
| `"update your kernel"` | MISSING from .pot and .po | Added to both |

## Stale Entries Still in .po (not removed — harmless)

The March 2026 `.po` contained combined template strings with `{r}` and `{b}` placeholders
that do NOT correspond to any actual `i18n::tr()` call in the source. These have `, fuzzy`
flags and won't be used at runtime. They remain in the file for historical reference.
If Rusty implements the template refactor (items 12-13 from March rescan), these will
become the correct msgids. Otherwise they can be removed in a future cleanup pass.

Affected entries:
- `"{r} is newer than catalog baseline ({b}) — some indicators may have changed"`
- `"{r} is older than catalog baseline ({b}) — update your kernel"`
- `"{readable} readable — all hardened ✓"`
- `"{readable} readable — {hardened} hardened, {not_hardened} not hardened ({pct}%)"`

## fr_CA Coverage
Approximately 98% — all active source strings now have translations.
The stale template strings above will not load at runtime; at runtime the source will
call the individual fragment msgids that ARE now translated.

## Makefile Status
- Already in `I18N_ACTIVE_DOMAINS` ✓
- `I18N_ACTIVE_LOCALES_umrs_uname := fr_CA` ✓

## Policy-Critical Terms — Flagged for Henri
None new in this update pass.
