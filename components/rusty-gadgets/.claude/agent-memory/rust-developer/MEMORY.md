# Rust Developer Agent Memory

<!-- Keep under 200 lines — this file is loaded into every session context. -->

## Key Memory Files

- [tui_patterns.md](tui_patterns.md) — umrs-ui architecture, trait patterns, clippy rules

## Workspace Quick Reference

- Workspace root: `components/rusty-gadgets/`
- Run all checks from that directory: `cargo xtask clippy && cargo xtask test`
- Format: `cargo xtask fmt`
- UI library crate: `umrs-ui` (renamed from `umrs-tui` 2026-03-22; pure lib — no binary targets)
- `umrs-uname` is a standalone binary crate depending on `umrs-ui` (extracted 2026-03-22)

## Critical Rules (from CLAUDE.md)

- `#![forbid(unsafe_code)]` in every crate root — no exceptions
- `#![deny(clippy::unwrap_used)]` — use `?`, `match`, or `if let`
- All tests in `tests/` — never inline `#[cfg(test)]` or `mod tests`
- All public items need NIST SP 800-53 / CMMC / NSA RTB annotations
- Clippy must be clean with zero warnings — fix findings, no `#[allow]` without Jamie approval
- 100-char max line width, 4-space indent, Unix newlines

## umrs-ui Architecture (renamed from umrs-tui 2026-03-22)

### Key source files
- `umrs-ui/src/app.rs` — `AuditCardApp` trait, `AuditCardState`, `DataRow`, `ColumnLayout`, all enums
- `umrs-ui/src/data_panel.rs` — `render_data_panel()`, `render_scrollable_pane()`, row expansion
- `umrs-ui/src/layout.rs` — master `render_audit_card()` entry point
- `umrs-uname/src/main.rs` — `OsDetectApp` implementation, binary = `umrs-uname` (own crate, extracted 2026-03-22)
- `umrs-stat/src/main.rs` — `FileStatApp` implementation, binary = `umrs-stat` (own crate)
- `umrs-ui/tests/data_types_tests.rs` — DataRow, TabDef, StatusMessage tests
- `umrs-ui/tests/column_layout_tests.rs` — ColumnLayout, two-column trait method tests

### Tab conventions
- Trust/Evidence is always the rightmost (last) tab — UMRS convention
- Tab 0 (OS Information) uses `ColumnLayout::TwoColumn`
- Tabs 1–2 use `ColumnLayout::Full` (full-width for IndicatorRow / TableRow)

### Two-column layout (added 2026-03-20)
- `ColumnLayout` enum: `Full` (default) | `TwoColumn`
- Three new trait methods: `column_layout()`, `data_rows_left()`, `data_rows_right()`
- `render_data_panel()` dispatches on `column_layout(active_tab)`
- `render_two_column_pane()` splits area 50/50, calls `render_scrollable_pane()` twice
- Both columns share the single `scroll_offset` from `AuditCardState`
- Pinned rows (if any) remain full-width above both columns

### DataRow variants
`KeyValue`, `TwoColumn` (same-line pair), `GroupTitle`, `Separator`, `IndicatorRow`
(multi-line kernel indicator), `TableRow` (3-col evidence), `TableHeader` (sticky header)

### Existing clippy suppressions (all in lib.rs and bin files)
`doc_markdown`, `missing_errors_doc`, `missing_panics_doc`, `module_name_repetitions`,
`option_if_let_else`, `redundant_closure`, `unreadable_literal`
- `too_many_arguments` allowed on `expand_indicator_row` (10-arg kernel render helper)
- `too_many_lines` allowed on `indicator_description` and `indicator_recommended`
  (37-entry lookup tables — splitting reduces readability)
- `cast_possible_truncation` allowed on u16 casts when the value is provably in range

## umrs-platform Key Types

### KernelVersion (added 2026-03-21)
- Lives in `umrs-platform/src/os_identity.rs`, re-exported from `umrs-platform` root
- Parses `MAJOR.MINOR.PATCH` from any kernel release string (strips distro suffix at first `-`)
- `FromStr`, `Display`, `PartialOrd`, `Ord` derived — comparison is lexicographic on `(major, minor, patch)`
- `KernelVersionParseError` is the associated error type (also re-exported)
- `CATALOG_KERNEL_BASELINE: &str` in `umrs-platform/src/posture/catalog.rs`, re-exported via `posture::CATALOG_KERNEL_BASELINE`
- Parse `CATALOG_KERNEL_BASELINE` as `KernelVersion` at call site — the constant is `&str` for compile-time binding

### KernelRelease (pre-existing)
- Also in `os_identity.rs` — holds the raw release string + `corroborated: bool`
- Two-source corroboration: `uname(2)` vs `/proc/sys/kernel/osrelease`
- Does NOT parse the version triple — use `KernelVersion` for version comparison

### vec_init_then_push pattern
Clippy fires `vec_init_then_push` when the first pushes after `Vec::new()` are unconditional.
Fix: use `vec![item1, item2, ...]` for the initial unconditional items, then use `push()`
for conditional items. See `build_os_info_right()` in `main.rs` for the canonical example.
