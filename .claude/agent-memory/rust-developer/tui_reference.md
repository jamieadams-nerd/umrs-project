# TUI Implementation Reference

Moved from MEMORY.md 2026-03-18. Detailed TUI implementation notes.
See also: tui_phase1.md through tui_phase45.md for phase-specific context.

## umrs-tui Architecture (2026-03-12)

- Library crate `umrs_tui` (lib.rs) + binary `umrs-tui` (main.rs)
- Modules: `app`, `theme`, `keymap`, `layout`, `header`, `tabs`, `data_panel`, `status_bar`
- Entry trait: `AuditCardApp` (object-safe). `report_name`/`report_subject` return `&'static str`.
- State: `AuditCardState::new(tab_count)` — owns active_tab, scroll_offset, should_quit
- Render: `render_audit_card(frame, f.area(), &dyn AuditCardApp, &state, &theme)`
- Layout: Vertical[header, 1 tab, Min(0) data, 1 status]
- Deps: `crossterm = "0.28"`, `rustix` with `system`, `systemd-journal-logger = "2"`
- Journald init: best-effort; TUI never writes to stderr

## Binaries

- `umrs-os-detect-tui` — `src/main.rs` (Tab 0 = OS, Tab 1 = Trust/Evidence)
- `umrs-file-stat` — `src/bin/file_stat.rs` (Identity / Security / Observations tabs)
  Deps: `umrs-selinux`, `umrs-platform`, `tree_magic_mini`

## AuditCardApp `report_subject()` Pattern

Returns `&'static str`. Runtime string: `Box::leak(runtime_string.into_boxed_str())`

## tree_magic_mini Supply Chain

`tree_magic_mini = "3"` in `umrs-tui` only. Pure-Rust MIME detection.
Path-based API (not fd-based) — documented in source. Display-only.

## /proc/mounts Lookup Pattern

ProcfsText + SecureReader, walk lines, longest-prefix match on mount_point.

## ELF Header Read Pattern

`File::open` + `read_exact` 20 bytes. Display-only (not trust-relevant).

## i18n Integration

- `i18n::init("umrs-tui")` — first line of `main()` in BOTH binaries
- `i18n::tr("msgid")` returns `String`; compatible with `impl Into<String>`
- `card_title()` returns `String` (not `&'static str`); `report_name()`/`report_subject()` stay `&'static str`
- Header field padding: `format!("{:<8} : ", i18n::tr("Report"))`

## Test Suite

4 files, 92 tests: audit_card_state (27), keymap (18), data_types (18), theme (11), trait_impl (18).
Key: `handle_action` takes `&Action`; `key()` helper must be `const fn`; mock fails-closed.

## Clippy too_many_lines

Fix: extract blocks into helpers (e.g., `build_inode_flag_rows`), NOT suppress.
