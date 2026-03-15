---
name: umrs-tui TUI Phase 3
description: DataRow struct replaced by enum; TwoColumn variant; data_panel match dispatch
type: project
---

Phase 3 complete as of 2026-03-15.

**Why:** DataRow was a struct with public fields. The plan required a `TwoColumn` variant and a
`GroupTitle` stub. An enum is the correct model — all variants are matchable, rendering stays
stateless, no builder state needed.

**What changed:**
- `DataRow` is now a `pub enum` in `src/app.rs` with variants:
  - `KeyValue { key, value, style_hint }` — standard single-column row
  - `TwoColumn { left_key, left_value, left_hint, right_key, right_value, right_hint }` — side-by-side
  - `GroupTitle(String)` — stub; full styling in Phase 4
  - `Separator` — blank line; `const fn separator()` returns this variant
- `DataRow::new()` kept as alias for `key_value()` — no breaking change at call sites
- `DataRow::separator()` is `const fn` (clippy `missing_const_for_fn` requires it; Separator has no String fields)
- `data_panel.rs` `build_row_line` now has signature `fn build_row_line<'a>(row: &'a DataRow, theme: &'a Theme) -> Line<'a>` — both lifetimes required by rustc (Line borrows from both)
- `TwoColumn` rendering: left key padded to `HALF_KEY_COL_WIDTH`, left value padded to fill remaining `KEY_COL_WIDTH` budget, then right key + right value
- `GroupTitle` renders as dim key-style label (placeholder until Phase 4 adds `group_title` theme field)

**Tests updated:**
- `tests/data_types_tests.rs` — all field access replaced with match arms; separator test changed to `matches!(row, DataRow::Separator)`; two new tests: `data_row_two_column_sets_all_fields`, `data_row_separator_variant_is_separator`
- `tests/trait_impl_tests.rs` — field access tests updated; separator test renamed to `data_rows_tab_zero_separator_is_separator_variant`

**How to apply:**
- Any new test touching `DataRow` must use `match` or `matches!()` — no direct field access
- `separator()` is `const fn`; keep it that way
- Phase 4 will add `group_title` theme field and wire up `GroupTitle` rendering
