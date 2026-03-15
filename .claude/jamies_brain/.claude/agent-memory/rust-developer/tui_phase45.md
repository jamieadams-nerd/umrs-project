---
name: umrs-tui TUI Phases 4 and 5
description: GroupTitle theme wiring, group_title Style field, indicator_unavailable color change, tests added
type: project
---

## Phase 4 — Group Title Rendering (complete 2026-03-15)

- `DataRow::GroupTitle` variant and `group_title()` constructor were already present from Phase 3
- `build_row_line` match arm in `data_panel.rs` now uses `theme.group_title` (was `theme.data_key`)
- Render: flush-left, single `Span::styled`, no padding, no border decoration
- Indentation is a caller convention: prepend `"  "` to subsequent key strings
- Doc comments on `GroupTitle` variant and `group_title()` constructor cleaned up (removed Phase 4 forward-ref wording)

## Phase 5 — Theme Styling (complete 2026-03-15)

- `Theme` struct gained `group_title: Style` field — default: bold white (`Color::White + BOLD`)
- `indicator_unavailable` default changed from `Color::DarkGray + DIM` to `Color::Yellow`
  - Reason: security-auditor finding O-5 (MEDIUM) — unavailable (failed probe) must be
    visually distinct from inactive (known-off); gray-on-gray conflates them
  - Control: NIST SP 800-53 CA-7 (continuous monitoring)
- `indicator_active`, `indicator_inactive`, `indicator_style()` helper already existed from Phase 1

## Tests added

- `tests/data_types_tests.rs`: `data_row_group_title_stores_string`, `data_row_group_title_accepts_owned_string`
- `tests/theme_tests.rs`: `theme_default_has_group_title_style`, `theme_default_indicator_unavailable_is_yellow`,
  `theme_default_indicator_inactive_and_unavailable_differ`, `theme_default_all_indicator_styles_differ`

## Security auditor findings addressed

- O-5 (MEDIUM): indicator_unavailable now Yellow, distinct from indicator_inactive DarkGray
- All other findings from tui-plan-security-review.md are future-phase or plan-level corrections (not code)

## Key files

- `components/rusty-gadgets/umrs-tui/src/theme.rs` — Theme struct, group_title field, unavailable color
- `components/rusty-gadgets/umrs-tui/src/data_panel.rs` — GroupTitle match arm
- `components/rusty-gadgets/umrs-tui/src/app.rs` — DataRow::GroupTitle doc, group_title() constructor doc
