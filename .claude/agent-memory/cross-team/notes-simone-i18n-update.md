# i18n Pipeline Changes — 2026-03-23

**To:** Simone (umrs-translator)
**From:** The Hand (Henry)
**Status:** open

Three changes to the i18n pipeline that affect your workflow:

## 1. `UMRS_LOCALEDIR` environment variable

`umrs-core/src/i18n.rs` now reads `UMRS_LOCALEDIR` at runtime to override the default
locale directory (`/usr/share/locale`). This enables development-time testing without
installing `.mo` files system-wide.

Usage:
```bash
UMRS_LOCALEDIR=resources/i18n/umrs-uname LANG=fr_CA.UTF-8 cargo run -p umrs-uname
```

This variable is documented as a future member of the environment scrub allowlist
(umrs-tool-init plan, sub-phase 1e). When `SanitizedEnv` is implemented, the raw
`std::env::var` read will be replaced by a validated accessor.

## 2. Makefile `.mo` output path changed

The `i18n-compile-<domain>` targets now output to the gettext-standard directory layout:

**Old:** `resources/i18n/<domain>/<locale>.mo`
**New:** `resources/i18n/<domain>/<locale>/LC_MESSAGES/<domain>.mo`

This matches what `gettextrs::bindtextdomain` expects. No changes needed to `.po` files
— only the compiled `.mo` output path changed.

The `i18n-build` target now delegates to per-domain `i18n-compile-<domain>` targets
(previously it used the generic `I18N_LOCALES` list and missed `fr_CA`).

## 3. `.po` file quality standard

**Your `.po` files must compile clean via `make i18n-build` before deliverables are
considered complete.** Same standard as every other agent — work isn't done until the
build system accepts it.

Issues found in the first `fr_CA.po` delivery:
- Unicode escapes (`\u2014`, `\u00a0`) — `.po` files require literal UTF-8 characters,
  not Python/JSON-style escape notation
- Duplicate `msgid` entries ("T0 — Untrusted" appeared twice)
- Both caused `msgfmt` to fail with 16 fatal errors

**Verification step before declaring done:**
```bash
make i18n-build    # must exit 0 with no errors
```
