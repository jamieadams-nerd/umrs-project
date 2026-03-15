---
name: umrs-tui TUI Phase 2 — HeaderField Extensibility
description: What was implemented in Phase 2, key design decisions, and available slots for fields
type: project
---

Phase 2 of the TUI enhancement plan (`.claude/plans/tui-enhancement-plan.md`) is complete.

**Why:** Callers need to inject supplemental identification fields into the header (e.g.,
tool version, run timestamp) so audit cards can serve as standalone SP 800-53A Examine objects.
The `header_fields()` default method approach is backward-compatible — existing callers compile unchanged.

**What was delivered:**

- `app.rs`: `HeaderField` struct with `label: String`, `value: String`, `style_hint: StyleHint`;
  constructors `HeaderField::new(label, value, hint)` and `HeaderField::normal(label, value)`;
  both carry `#[must_use]` with message strings; AU-3 annotation at struct level.
- `app.rs`: `header_fields(&self) -> &[HeaderField]` default method on `AuditCardApp` trait;
  default returns `&[]` (backward-compatible); AU-3 annotation on method.
- `header.rs`: `render_header()` computes available supplemental slots as
  `(area.height as usize).saturating_sub(2).saturating_sub(4)` (interior minus 4 fixed rows);
  calls `append_header_fields()` which truncates with "…" marker if fields overflow;
  uses `style_hint_color()` free fn from theme to convert `StyleHint` to ratatui `Style`.
- `lib.rs`: `HeaderField` added to convenience re-exports.
- `tests/trait_impl_tests.rs`: 4 new tests: `header_fields_default_returns_empty_slice`,
  `header_fields_override_returns_custom_fields`, `header_field_new_sets_all_fields`,
  `header_field_normal_sets_normal_hint`.

**Header height arithmetic:**
- `WIZARD_SMALL.height = 7`; `HEADER_HEIGHT = 7 + 2 + 1 = 10`
- Interior rows = 8; fixed rows (report/host/subject/indicators) = 4
- Available for HeaderField = 4 slots before truncation

**Trust boundary note from security auditor:**
`value` must never contain security labels, credentials, or classified data.
`HeaderField` is for identification metadata only (tool version, timestamp, assessment ID).

**How to apply:** Each binary's `header_fields()` impl should return at most 3-4 fields.
Recommended minimum: `HeaderField::normal("Version", env!("CARGO_PKG_VERSION"))`.
