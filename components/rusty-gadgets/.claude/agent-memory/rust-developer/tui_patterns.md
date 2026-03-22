---
name: umrs-ui architecture and patterns
description: Architecture decisions, trait patterns, and coding conventions for the umrs-ui crate (renamed from umrs-tui 2026-03-22)
type: project
---

# umrs-ui Patterns (renamed from umrs-tui 2026-03-22)

## Trait Design

`AuditCardApp` is a data-provider trait (never mutates state). `AuditCardState` is the
mutable counterpart owned by the calling binary. This separation means `render_audit_card`
takes `&dyn AuditCardApp` (object-safe) and `&AuditCardState`.

New methods on `AuditCardApp` MUST have defaults — the trait has two consumers
(`umrs-ui/src/main.rs` = `umrs-uname`, `umrs-stat/src/main.rs` = `umrs-stat`) and
potentially external crates. Breaking changes to the trait require updating both consumers.

## ColumnLayout Pattern

When adding a new tab layout mode:
1. Add variant to `ColumnLayout` enum in `app.rs`
2. Add default trait methods to `AuditCardApp` (returns empty / `Full`)
3. Add dispatch arm in `render_data_panel()` in `data_panel.rs`
4. Add render function (e.g., `render_three_column_pane()`)
5. Override trait methods in the relevant binary
6. Add tests in `tests/column_layout_tests.rs` or a new test file

## Data Row Hierarchy

- `GroupTitle` — section header, flush left, no border
- `KeyValue` with non-empty key — standard k/v row
- `KeyValue` with empty key — word-wrapped description (italic)
- `Separator` — blank line
- `IndicatorRow` — multi-line: key+value, description, contradiction, configured, recommendation
- `TwoColumn` — two k/v pairs on one line (different from two-column layout mode)
- `TableRow` / `TableHeader` — fixed 3-column evidence table

## `render_scrollable_pane` Sticky Header

When the first row in the scrollable section is `DataRow::TableHeader`, it is extracted
and rendered as a non-scrolling sticky bar at the top of the pane (bold + reversed).
The `has_pinned` argument was originally used to set the title but is now ignored — kept
for backward compatibility.

## TableWidths

`TableWidths::from_rows()` scans all rows to compute dynamic column widths. Must be called
once per render pass and passed down the call chain. Prevents truncation when source paths
or indicator names are longer than the minimum constants.

## Pinned Panel Height Calculation

Pinned height = sum of `expanded_row_line_count()` for all pinned rows + 2 (borders).
Clamped to max 40% of total area. Minimum 4 lines. `expanded_row_line_count` must be
kept in exact sync with `expand_row_lines` or pinned area will be wrong-sized.

## Compliance Annotations

At module level (`//!` block): always include `## Compliance` section.
At type/function level: only for security-critical items (not accessors).
Citation form: `NIST SP 800-53 XX-N` (not `NIST 800-53`), `NSA RTB RAIN`, etc.
