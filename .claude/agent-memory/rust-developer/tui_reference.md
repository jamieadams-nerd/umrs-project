# TUI Implementation Reference

Moved from MEMORY.md 2026-03-18. Detailed TUI implementation notes.
See also: tui_phase1.md through tui_phase45.md for phase-specific context.

## tui-cli RAG Collection (familiarized 2026-03-28)

Knowledge artifacts at `.claude/knowledge/tui-cli/`. Full re-read confirmed unchanged from
2026-03-15 ingestion. Covers:
- ratatui v0.30.0 API: workspace structure, Widget/StatefulWidget, Layout/Constraint/Flex,
  Frame::area(), DefaultTerminal, init()/restore(), breaking changes v0.20–v0.31
- crossterm: event poll/read pattern, KeyCode, KeyModifiers, NO_COLOR (built-in suppression)
- clap v4 derive API: Parser/Subcommand/Args/ValueEnum, --json pattern (UMRS convention)
- color-eyre: install() first in main(), WrapErr, Section helpers
- CLIG guidelines: stdout/stderr discipline, --json, error message format, NO_COLOR
- 5 example Rust programs: demo2 (multi-tab), popup, table, scrollbar, flex_layouts

Key rulings (see style-decision-record.md):
- SDR-1: manual init()/restore() for production binaries; ratatui::run() for examples only
- SDR-2: impl Widget for &App (not WidgetRef)
- SDR-3: crossterm handles NO_COLOR for TUI paths; manual check for direct println! paths
- SDR-4: always poll(timeout) — UMRS tools need periodic refresh for live status
- SDR-8: always use array destructuring let [header, body, footer] = area.layout(&layout)
- SDR-9: use Line::from("...") for block titles (block::Title removed in v0.30.0)

Open: PH-4 — whether #[expect(clippy::...)] requires same Jamie approval as #[allow].

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
