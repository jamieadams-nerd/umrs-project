---
name: workspace-structure
description: Workspace layout, crate purposes, and key module paths as of 2026-03-25
type: project
---

# Workspace Structure

Working directory: `/DEVELOPMENT/umrs-project/components/rusty-gadgets/`

## Workspace Members

| Crate | Purpose |
|---|---|
| `umrs-core` | Foundational primitives: console macros, audit, i18n, human formatting, robots, timed_result, validate (Email/RgbHex/SafeString) |
| `umrs-labels` | CUI label abstraction lib+bin: `cui/catalog.rs`, `cui/palette.rs`, `validate.rs` (CuiMarking pattern) |
| `umrs-selinux` | SELinux modeling: SecurityContext, MLS, MCS, categories, status, xattrs, posix, observations, validate (SelinuxContext/MlsRange) |
| `umrs-platform` | OS detection, FIPS, platform-level primitives |
| `umrs-hw` | Hardware introspection |
| `umrs-ls` | Security-enriched directory listing tool |
| `umrs-stat` | File security stat tool |
| `umrs-uname` | TUI security posture tool (tabs: Trust, Kernel Security, Platform, Evidence, etc.) |
| `umrs-ui` | Shared TUI/UI primitives |
| `xtask` | Workspace task runner: `cargo xtask fmt`, `cargo xtask clippy`, `cargo xtask test` |

## Key Paths

- `umrs-labels/cui-labels.json` — CUI catalog fixture (used by catalog_tests via CARGO_MANIFEST_DIR)
- `umrs-labels/data/us/` — US CUI JSON data files
- `umrs-labels/data/ca/` — Canadian Protected JSON data files
- `umrs-labels/tests/` — catalog_tests.rs, validate_tests.rs
- `umrs-selinux/tests/` — validate_tests.rs plus many others
- `umrs-uname/src/main.rs` — large TUI file, ~1900 lines; has helper extraction pattern for clippy line-count compliance
- `umrs-ui/src/viewer/` — ViewerApp pattern: tree.rs, detail.rs, layout.rs, mod.rs
- `umrs-ui/src/config/` — ConfigApp pattern: fields.rs, diff.rs, layout.rs, mod.rs
- `umrs-ui/tests/viewer_tests.rs` — 42 integration tests for viewer pattern
- `umrs-ui/tests/config_tests.rs` — 53 integration tests for config pattern
- `umrs-ui/examples/viewer_catalog.rs` — ViewerApp example (catalog browser, no TTY)
- `umrs-ui/examples/config_selinux.rs` — ConfigApp example (SELinux config editor, no TTY)

## umrs-ui Module Summary

Three TUI patterns:
1. **AuditCardApp** — read-only audit card display (original)
2. **ViewerApp** — read-only hierarchical browser (tree expand/collapse, search, detail panel)
3. **ConfigApp** — interactive config editor (fields, validation, diff, save gate, discard confirm)

`Action` enum: includes `Expand`, `Collapse`, `Search`, `Back`, `Save`, `Discard`, `ToggleEdit`
(added 2026-03-25 alongside new patterns)

## Removed Modules (2026-03-25)

- `umrs-core::cui` — moved to `umrs-labels::cui`
- `umrs-core::selinux` (mcs.rs) — deleted; superseded by `umrs-selinux::mcs::translator`
- `umrs-core::validate::CuiMarking` — moved to `umrs-labels::validate`
- `umrs-core::validate::SelinuxContext/MlsRange` — moved to `umrs-selinux::validate`

## Validate Pattern Split

Each domain owns its pattern:
- `umrs-core::validate::UmrsPattern` — Email, RgbHex, SafeString
- `umrs-labels::validate::CuiPattern` — CuiMarking
- `umrs-selinux::validate::SelinuxPattern` — SelinuxContext, MlsRange

All three use identical `OnceLock<Mutex<HashMap<_, Regex>>>` cache design.
