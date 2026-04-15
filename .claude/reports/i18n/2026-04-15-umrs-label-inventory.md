# String Inventory Report — umrs-label
**Date:** 2026-04-15
**Crate:** umrs-label
**Domain:** umrs-label (not yet registered)
**Type:** binary

## Status: BLOCKED — No gettext wrapping in source

`umrs-label` has zero `i18n::tr()` calls anywhere in its source tree.
`umrs_core::i18n` is not imported. No `i18n::init()` call exists.

This crate is **not i18n-enabled**. String extraction cannot proceed.

## Makefile Status
- `umrs-label` is NOT in `I18N_TEXT_DOMAINS` or `I18N_ACTIVE_DOMAINS`
- No `I18N_SRC_DIR_umrs_label` defined
- Locale directories `components/rusty-gadgets/umrs-label/locale/fr_CA/LC_MESSAGES/`
  exist but are empty

## User-Facing Strings Identified (unwrapped — for Rusty)

These are the strings that need wrapping. All are in `src/main.rs`.

### Help text strings (TUI key bindings popup)
| Line | String | Macro to use |
|------|--------|--------------|
| 668 | `"Move up"` | `i18n::tr("Move up")` |
| 672 | `"Move down"` | `i18n::tr("Move down")` |
| 676 | `"Collapse node"` | `i18n::tr("Collapse node")` |
| 680 | `"Expand node"` | `i18n::tr("Expand node")` |
| 684 | `"Scroll active panel"` | `i18n::tr("Scroll active panel")` |
| 689 | `"Show details / toggle branch"` | `i18n::tr("Show details / toggle branch")` |
| 693 | `"Switch focus (Tree ↔ Detail)"` | `i18n::tr("Switch focus (Tree ↔ Detail)")` |
| 697 | `"Search / filter catalog"` | `i18n::tr("Search / filter catalog")` |
| 701 | `"Cancel search"` | `i18n::tr("Cancel search")` |
| 706 | `"Toggle this help"` | `i18n::tr("Toggle this help")` |
| 710 | `"Quit"` | `i18n::tr("Quit")` |

### Infrastructure required before wrapping can occur
1. Add `use umrs_core::i18n;` to `src/main.rs`
2. Add `i18n::init("umrs-label");` in `main()` before TUI initialization
3. Wrap strings listed above with `i18n::tr()`

### Strings intentionally NOT wrapped
- `log::warn!("Canadian catalog unavailable: {e}")` — log messages stay English (cross-locale SOC debugging)
- `log::error!("terminal draw error: {e}")` — same reason
- clap `about` / `long_about` strings (lines 82-83) — clap help text i18n requires
  separate clap-derived approach; deferred

## Makefile Changes Required (propose to developer/Jamie)
```makefile
# Add to I18N_TEXT_DOMAINS:
I18N_TEXT_DOMAINS   := ... umrs-label

# Add to I18N_ACTIVE_DOMAINS:
I18N_ACTIVE_DOMAINS := ... umrs-label

# New entries:
I18N_SRC_DIR_umrs_label    := components/rusty-gadgets/umrs-label/src
I18N_ACTIVE_LOCALES_umrs_label := fr_CA
```

## Blocker Resolution Path
1. Rusty wraps the 11 strings listed above and adds i18n infrastructure
2. Simone runs `make i18n-extract-umrs-label` (requires xtr)
3. Simone initializes `fr_CA.po` and translates
4. .mo compiled

## Note on Scope
`umrs-label` catalog content (CUI marking names and descriptions) is already
translated in `US-CUI-LABELS.json` (completed 2026-04-07). The strings in this
report are only the TUI navigation/help strings in the Rust binary itself.
