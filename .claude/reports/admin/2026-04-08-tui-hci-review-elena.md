# HCI and Information Architecture Review — UMRS TUI Tools
**Reviewer**: Elena (Senior Technical Writer / Information Architecture)
**Date**: 2026-04-08
**Scope**: `umrs-label`, `umrs-ls`, `umrs-stat`, and the shared `umrs-ui` library
**Purpose**: Pre-integration review gating the embedding of `umrs-stat` and `umrs-label` as views inside `umrs-ls`

---

## Summary Table

| Category | Count |
|---|---|
| ACCURATE | 9 |
| CONCERN | 11 |
| ERROR | 2 |

---

## Findings

### ACCURATE items

**A-1 — Shared theme is the single source of truth**
`umrs-ui::theme::Theme` centralizes every color, style, and semantic color meaning. All three tools construct a `Theme` once and pass it into every render function. No tool hard-codes a color outside a renderer, with the sole exception of the group header row in `umrs-ls` (see C-1). The `list_selection` style — the aged-parchment warm-yellow background used as a cursor row — propagates automatically to all three tools without any per-tool override. This is the correct architecture.

**A-2 — Icon catalog is genuinely shared**
`umrs-ui::icons` is the single catalog for every glyph. All three tools import from it; none define their own `\u{...}` literals outside the catalog. The `ICON_BANNER`, `ICON_SIBLING`, `CHEVRON_OPEN/CLOSED`, `ICON_FLAG`, `ICON_PLACEHOLDER`, and filesystem-object icons are all correctly centralized. This guarantees that "encrypted directory" renders identically in `umrs-ls` and in any future embedded file view.

**A-3 — Header layout is consistent across umrs-ls and umrs-label**
Both tools implement a `9-row / LOGO_PANEL_WIDTH=17` header with the same two-column split: security posture (55%) left, session info (45%) right, directory or tool-identity rows below. The `render_kv_rows` / `render_label_value_rows` helpers are functionally equivalent — they both compute `max_label` width dynamically, pad with ` {label:<max} : `, and right-truncate values. This is the correct shared pattern.

**A-4 — Panel focus is visually signalled in umrs-label**
The tree and detail panels in `umrs-label` use full-brightness cyan (`BOLD`) border when active and `DarkGray` when inactive. An operator scanning the screen can immediately determine where keyboard input will land. No ambiguity exists between "tree has focus" and "detail has focus" states.

**A-5 — IOV column design is sound**
The IOV (Integrity / Observation / Verified) column in `umrs-ls` uses a middle-dot placeholder (`·`) for clear slots, preserving column alignment without visual noise. The `I` in red, `⚑` in red or yellow, and `V` in green carry distinct color-and-character identity — each marker is independently readable by character alone when color is unavailable. This is a strong accessibility choice.

**A-6 — Scroll state is preserved in umrs-label detail panel**
The `detail_scroll: u16` state variable in `umrs-label` is reset to zero on panel-switch (`Tab`) and on node change. This prevents the operator from seeing a stale scroll position when selecting a new marking.

**A-7 — Designation color coding in the detail panel is semantically correct**
`specified` → yellow (caution), `basic` → green (standard). Yellow signals additional statutory obligations. This follows the established UMRS color semantics (green = OK, yellow = attention) and will read correctly when the detail panel is embedded in `umrs-ls`.

**A-8 — Dynamic key-width alignment in the detail panel**
`build_detail_lines` measures only the keys that will actually appear and sizes the key column to that width. A marking with only four fields does not waste horizontal space on a 16-character key column. This is especially valuable in a narrow embedded popup.

**A-9 — The listing render path is bounded by visible rows**
`render_listing` only processes `display.display_list` entries for the visible window. The hot path (called every frame) does not iterate the full directory tree on each draw call. This is the correct approach for a tool that may browse large directories.

---

### CONCERN items

**C-1 — Group header colors are hard-coded, not theme-sourced**
`build_group_header_item` in `umrs-ls/src/tui_render.rs` (lines 820–821) hard-codes:
```rust
let type_bg = Color::Rgb(70, 80, 90);
let marking_bg = Color::Rgb(55, 65, 75);
```
A comment notes these are "Placeholder gray tones — will be replaced by umrs-labels palette in Phase 5." The concern is not the palette placeholder itself — it is that these constants bypass the `Theme` struct entirely. When `Theme::light()` is implemented, the group header will remain dark regardless of the active theme.

**Recommendation**: Add `group_header_type_bg` and `group_header_marking_bg` fields to `Theme`. Set them to the current gray values in `default_dark()`. Set them to appropriate light-mode values in the eventual `light()` impl. The hard-coded `Color::Rgb(...)` literals in the renderer are then replaced by `theme.group_header_type_bg`.

---

**C-2 — `highlight_symbol` inconsistency between umrs-ls and umrs-label**
`umrs-ls/src/tui_render.rs:728` uses `.highlight_symbol("► ")` (a 2-character prefix on the selected row). `umrs-label/src/tui/render.rs:386` uses no `highlight_symbol` — selection is color-only.

This is not wrong in either tool taken alone. But it becomes a problem during integration. When the label detail panel is embedded as a popup in `umrs-ls`, the two tools will use visually different selection indicators on adjacent or overlapping lists. An operator will see a `►` cursor in the `ls` listing and a color-only cursor in the label tree. These read as different UI paradigms.

**Recommendation**: Choose one approach for the entire embedded experience and apply it consistently. The color-only approach from `umrs-label` is the better choice — it was a deliberate decision to eliminate the 2-char blank prefix that pushed icons away from the left edge. Remove `highlight_symbol("► ")` from `umrs-ls` to match.

---

**C-3 — NO_COLOR check is present but inert in umrs-stat and umrs-label**
Both tools execute:
```rust
let _no_color = std::env::var("NO_COLOR").is_ok();
let theme = Theme::default();
```
The `_no_color` flag is detected but not used. The theme is always the dark color theme regardless. The TODO comment acknowledges this: "Theme::no_color() variant needed; currently falls back to default." The check is architecturally correct but the behavior is non-compliant with the `NO_COLOR` standard until `Theme::no_color()` is implemented.

A security operator running with `NO_COLOR=1` in a restricted terminal environment will still receive color output from these two tools. `umrs-ls` handles NO_COLOR correctly in CLI mode (`use_color = args.contains(&"--color") && std::env::var("NO_COLOR").is_err()`), but its TUI mode has the same inert-check pattern.

**Recommendation**: Before shipping the integrated view, implement `Theme::no_color()` — a palette where all `Style` values use only `Modifier::BOLD`, `Modifier::UNDERLINED`, and `Modifier::DIM` with no `Color::*` assignments. Wire all three tools to use it when `NO_COLOR` is set. This is not a cosmetic concern; it is an accessibility and compliance requirement.

---

**C-4 — Status bar key legends are inconsistent between tools**
`umrs-label`: `"  ↑↓:nav  Enter:show  Tab:panel  /:search  ?:help  q:quit"`
`umrs-ls`:    `"  ↑↓:nav Enter:open ?:help q:quit "`

The divergence in formatting (double-space separator vs. single-space), the action labels ("show" vs. "open"), and the visible key set (umrs-ls omits `/` and `Tab`) will be visible side-by-side in the integrated view. An operator reading both status bars in the same screen will see two different conventions for the same actions.

**Recommendation**: Standardize the key legend format. Move legend rendering into `umrs-ui` as a shared helper that accepts a `&[(key, action)]` slice. The format should be: two leading spaces, then `key:action` pairs separated by two spaces, with no trailing space. Agreed canonical labels: `nav`, `open`, `search`, `panel`, `help`, `quit`.

---

**C-5 — The `render_kv_rows` helper exists in two places**
`umrs-label/src/tui/render.rs` defines `render_kv_rows`. `umrs-ls/src/tui_render.rs` defines `render_label_value_rows`. Both perform the same operation: compute max label width, format ` {label:<max} : `, right-truncate value, emit a `Line`. The implementations differ only in the name.

This duplication will diverge. A change to one (padding, truncation behavior, value style) will not propagate to the other unless both are edited in the same commit.

**Recommendation**: Move the canonical implementation to `umrs-ui` under a name like `render_kv_block`. Both tools import it. The function signature is already compatible: `(frame, area, &[(label, value, style)], theme)`.

---

**C-6 — The wizard logo render function is duplicated three times**
`umrs-ui/src/layout.rs:render_logo`, `umrs-label/src/tui/render.rs:render_wizard_logo`, and `umrs-ls/src/tui_render.rs:render_wizard_logo` are all identical: iterate `WIZARD_SMALL.lines`, style each with `theme.wizard`, wrap in a rounded bordered block. The `umrs-label` and `umrs-ls` versions even carry a comment "Mirrors the render_logo function from umrs-ui/src/layout.rs."

The comment documents the problem. The solution is to resolve it, not annotate it.

**Recommendation**: Export `render_logo` from `umrs-ui::layout` with `pub` visibility. Delete the local copies in `umrs-label` and `umrs-ls`. The function signature is already identical.

---

**C-7 — `umrs-stat` tab naming does not follow UMRS convention**
`umrs-stat` uses tab labels "Identity", "Security", "Observations". These are wrapped through `i18n::tr()`, which is correct. The concern is structural: "Observations" is the third tab, but it contains the most security-critical content — the typed `SecurityObservation` findings. Security auditors will navigate to the third tab every time.

**Recommendation**: Reorder to: "Security" (tab 0, security context and inode flags), "Observations" (tab 1, findings), "Identity" (tab 2, file metadata). Alternatively, or additionally, show a count badge on the Observations tab when findings are present (e.g., `"Observations (3)"`). The badge can be constructed in `AuditCardApp::tabs()` dynamically based on `build_status()`. Neither change is integration-blocking, but the ordering concern is worth resolving before embedding.

---

**C-8 — The header "Level" row label is ambiguous across tools**
Both `umrs-ls` and `umrs-label` render a `Level : s0-s0:c0.c1023` row in the session info panel. This is the running process's sensitivity level from `/proc/self/attr/current` — not the operator's clearance ceiling. The UMRS memory notes this disambiguation is still pending ("TUI Level = process, not user").

When embedded, this label will appear in the `umrs-ls` frame while a `umrs-label` popup is open. An operator unfamiliar with the distinction may mistake the process-level display for their clearance level.

**Recommendation**: Rename the row from `Level` to `Process Level` in all three tools. A planned popup to explain this further is noted in project memory — that popup will close the remaining gap. The label change can be made in the `render_session_lines` / `render_user_session_lines` helpers and will propagate everywhere these helpers are used.

---

**C-9 — Group header fill line is a commented-out placeholder**
`umrs-ls/src/tui_render.rs` lines 855–862:
```rust
//let fill_width = 80usize.saturating_sub(2 + 22 + 1 + 20 + 1 + 1);
//let fill_span = Span::styled(
    //" ".repeat(fill_width),
    //Style::default().add_modifier(Modifier::UNDERLINED),
//);
let fill_span = Span::styled(" ", Style::default());
```
The underlined fill that should extend across the header row to the terminal edge is disabled. The group header currently ends with a single space rather than a visual rule. On wider terminals, the group header floats in the middle of the row.

**Recommendation**: Implement a terminal-width-aware fill using the `area.width` available at render time instead of the hard-coded 80-char calculation. The renderer has `area` in scope. Use `area.width as usize` minus the sum of the preceding spans' display widths (chevron 2 + type block 22 + marker 1 + marking block 21 + marker 1 = 47 chars) to compute fill width.

---

**C-10 — `umrs-stat` has no file path search or navigation — integration implies a workflow gap**
`umrs-stat` is invoked with a single path argument. In the proposed integration, pressing Enter on a file in `umrs-ls` would open a `umrs-stat` embedded view. But `umrs-stat` has no "go back" affordance. The operator presses `q`/`Esc` to exit. If that closes the embedding rather than returning to the `umrs-ls` listing, the operator loses their scroll position and browsing context.

This is not an error in `umrs-stat` as a standalone tool. It becomes an architectural requirement for the embedding.

**Recommendation**: Before implementing the Enter-key integration, define explicitly whether `Esc` in the embedded view returns to `umrs-ls` or closes the entire tool. Document the decision in `umrs-ls/src/main.rs` before coding starts. The `umrs-label` integration is simpler (it opens as a popup modal, not a full-screen swap) — start there.

---

**C-11 — Detail panel placeholder text reads as an instruction rather than a state**
`render_marking_detail` renders "Select a marking to view details." when `data` is `None`. This is grammatically correct but slightly imperative. In an embedded context where the operator has just arrived at the panel, the instruction is fine. In an integration where the panel may be pre-loaded with context, it will flash briefly before the data renders.

**Recommendation**: Change the placeholder to "No marking selected." This is a state description, not an instruction, and it degrades more gracefully in edge cases. This is a cosmetic change but contributes to consistent tone across the tool.

---

### ERROR items

**E-1 — `umrs-stat` JSON mode is accepted but silently does nothing**

Severity: Medium — operator-visible broken promise

`umrs-stat` accepts `--json` via clap, logs a debug message, and then runs the TUI regardless:
```rust
log::debug!("umrs-stat: json={} (JSON output not yet implemented)", args.json);
```
The debug log is not visible to the operator. An operator who runs `umrs-stat /etc/shadow --json` to pipe results into a downstream tool will receive TUI output on their terminal with no warning that JSON mode is unavailable.

The `tui_cli_rules.md` rule is clear: "Provide `--json` output mode for all commands that return structured data." Accepting a flag and ignoring it violates the operator contract.

**Recommended fix**: Either (a) emit a clear error message to stderr and exit non-zero when `--json` is passed before the implementation is ready, or (b) remove the `--json` flag from the clap definition until it is implemented. Do not silently fall through to TUI mode. Option (a) is preferred — it communicates the gap honestly and makes operator scripts fail loudly rather than silently.

---

**E-2 — `umrs-label` event loop passes `_keymap` as an unused parameter**

Severity: Low — dead code indicates architectural confusion

`run_event_loop` in `umrs-label/src/main.rs` accepts `_keymap: &KeyMap` (note the underscore prefix, confirming it is unused). The event loop then implements its own raw `match key.code` dispatch instead of calling `keymap.lookup()`. This means the shared `KeyMap` defaults (`q`/`Esc` → Quit, `j`/`k` → scroll, `r` → Refresh) are not honored — the event loop re-implements them directly.

As a result, `umrs-label` does not use the canonical keymap at all. If the default bindings in `KeyMap::new()` change (for example, to add a new action), `umrs-label` will not inherit those changes.

`umrs-ls` and `umrs-stat` both call `keymap.lookup(key)` correctly.

**Recommended fix**: Remove `_keymap` from the signature. Restructure the `umrs-label` event loop to call `keymap.lookup(key)` for common actions (quit, scroll, search activation) and handle only the umrs-label-specific cases (`Tab` for panel switching, `Left`/`Right` for expand/collapse, `Enter` for node selection) directly. This aligns `umrs-label` with the pattern already established in `umrs-ls` and `umrs-stat`.

---

## Integration Readiness Assessment

This section addresses the specific question: what concerns exist about embedding `umrs-stat` and `umrs-label` as views inside `umrs-ls`?

### umrs-label → umrs-ls (popup/modal)

The `MarkingDetailData` and `render_marking_detail` components in `umrs-ui` were explicitly designed for this integration. The module-level comment in `umrs-ui/src/marking_detail.rs` states: "This module lives in `umrs-ui` so that both `umrs-label` … and `umrs-ls` … can show the same detail panel without any circular dependency."

The architecture is ready. The popup can be triggered by pressing Enter on a marking label in the `umrs-ls` group header or a file row. Resolve C-2 (highlight_symbol), C-4 (status bar legend), and C-6 (duplicated wizard logo) first. These are cheap fixes that prevent visual dissonance from appearing in the integration.

**Layout constraint**: The detail panel needs at minimum 45 columns and 20 rows to be readable. On a 80-column terminal, a 48/52 split would produce a 38-column detail panel — insufficient for the marking name and key-value rows. Recommend: trigger the marking detail as a floating overlay (using ratatui's `Clear` widget at a centered rect) rather than a fixed horizontal split. The `umrs-ls` listing already imports `Clear` — see the `render_permission_denied` usage.

### umrs-stat → umrs-ls (Enter-key drill-down)

The embedding of `umrs-stat` is more complex than `umrs-label`. `umrs-stat` is a full-screen audit card with its own header, tab bar, data panel, and status bar. It uses `umrs-ui::layout::render_audit_card`, which expects to own the full terminal area.

Two paths forward:

**Option A — Full-screen swap**: `umrs-ls` suspends its render loop, launches a sub-view using `render_audit_card` in the same terminal, and returns to its render loop when the operator presses `Esc`/`q`. The `umrs-ls` state (scroll position, selected entry) is preserved in memory during the sub-view. The operator sees `umrs-stat` full-screen, then returns to `umrs-ls` at the same position.

This is architecturally cleaner. The state management is a simple stack: push `umrs-ls` state, render `umrs-stat` sub-view, pop when done. The "frozen child handling" concern (noted in project memory) is addressed by bounded timeouts and Ctrl-C escape, not by structural changes to the render loop.

**Option B — Embedded panel**: `umrs-ls` reserves a right-side panel for the `umrs-stat` data, triggered on Enter. This requires splitting the listing area horizontally, which will reduce the `NAME` column width significantly. Given the IOV + MODE + OWNER:GROUP + MODIFIED + NAME columns already compete for space, a 50/50 split on a 120-column terminal would leave only about 25 characters for the stat panel data values — not enough for meaningful display.

**Recommendation**: Implement Option A for `umrs-stat`. Implement Option B (as a floating overlay) for `umrs-label`. Define a `SubView` enum (`None | StatView(FileStatApp, AuditCardState) | LabelView(MarkingDetailData, scroll)`) in `umrs-ls/src/main.rs` to manage the state.

### Navigation flow — explicit requirement before coding

Resolve the navigation contract before writing code. Two decisions must be made and documented:

1. In the `umrs-stat` sub-view, does `Esc` return to `umrs-ls`, or does it quit the entire tool? The current `umrs-stat` standalone behavior maps `Esc` to quit. The embedded behavior must override this.

2. In the `umrs-label` popup, does `Tab` cycle the popup's internal panel focus, or does it dismiss the popup and return focus to `umrs-ls`? `umrs-label` uses `Tab` for panel switching. `umrs-ls` uses `Tab` (via the default `KeyMap`) for `NextTab` — which in `umrs-ls` maps to `NextTab` but there is only one tab, so it is effectively a no-op today. If both tools are active simultaneously, the keybinding semantics must be explicitly assigned to one context.

These are not difficult problems, but they must be answered explicitly before the integration loop is written.

---

## Strengths Worth Preserving

**Shared theme propagation works.** The aged-parchment `list_selection` color, the semantic indicator colors (green/yellow/red for enabled/unavailable/disabled), and the dialog-severity border colors are all centralized and will propagate automatically to the integrated view. Do not fragment this — keep theme modifications in `Theme::default_dark()` and the eventual `light()`.

**`MarkingDetailData` is integration-ready.** The data struct is decoupled from both the `umrs-label` catalog and any `umrs-ls` rendering path. It carries owned strings, renders with a shared `render_marking_detail` function, and handles empty-field suppression gracefully. This will embed with minimal friction.

**The IOV column is scannable at 3 seconds.** Red `I`, flag `⚑` in red or yellow, and green `V` against dim middle-dot placeholders create a compact security posture column that an experienced operator can read without focusing on it. Preserve this column layout exactly in any integrated view.

**Dynamic key-width computation in the detail panel.** The `max_label_width` calculation in `build_detail_lines` means the key column stays tight in a narrow popup context. A panel showing only four fields will not waste 16 characters of horizontal space on an empty key column.

**Wizard logo and security posture header are already consistent.** Both tools render an identical 9-row header with the same split, same row labels, same indicator colors. An operator switching between the two tools in the integrated experience will not need to re-orient to find the SELinux status indicator.

**`render_marking_detail` handles `Wrap { trim: false }`.** Long description fields, injury examples, and required warning statements will word-wrap inside whatever panel width the embedding provides. This is essential for a popup that may be narrower than the full terminal.

---

## Open Items

- E-1 must be resolved before integration. An inoperative `--json` flag in a tool that will be embedded in a larger system is a broken promise that will become an integration test failure.
- E-2 (`_keymap` unused in `umrs-label`) should be resolved before integration to ensure keybinding behavior in the embedded context is predictable.
- C-3 (NO_COLOR inert) is not integration-blocking today, but it must be resolved before any release. Flag it as a P1 pre-release item.
- C-10 (navigation contract for embedded views) must be answered in a written design note before coding starts. The integration work should not begin without this decision.
