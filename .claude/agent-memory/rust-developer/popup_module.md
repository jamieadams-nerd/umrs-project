---
name: Popup module
description: umrs-ui popup.rs shared popup infrastructure — replaces duplicated rendering in umrs-ls
type: project
---

## Popup Module (libs/umrs-ui/src/popup.rs)

Added 2026-04-09. Eliminates duplicated popup rendering code from umrs-ls.

### Key Exports

- `PopupConfig` — geometry spec (title, hint, width/height pct + clamp bounds)
- `PopupCardData` trait — `tab_names() -> &[&'static str]` + `rows_for_tab(usize) -> &[DataRow]`
- `render_popup_frame(frame, area, config, theme) -> Rect` — Clear + Double border + hint line, returns content Rect
- `render_marking_detail_popup(frame, area, data, scroll, theme)` — label detail popup (70%, 40-80×12-30)
- `render_audit_card_popup(frame, area, app, active_tab, scroll, theme)` — tabbed file audit (70%, 60-90×18-35)
- `data_row_to_line(row, col_width, val_width, theme) -> Line` — authoritative DataRow → Line for popups
- `pub(crate) centered_rect(w, h, area) -> Rect` — same geometry helper as dialog.rs

### PopupCardData impl

`FileStatApp` implements `PopupCardData` in `umrs-stat/src/lib.rs`.
Static tab names: `static FILE_STAT_TAB_NAMES: &[&str] = &["Identity", "Security", "Observations"];`
`rows_for_tab` delegates to `Self::rows_for_tab(self, tab)` (bounds-checked).

### Deleted from umrs-ls main.rs

- `render_label_popup` → `umrs_ui::popup::render_marking_detail_popup`
- `stat_row_to_line` → `umrs_ui::popup::data_row_to_line`
- `render_stat_popup` → `umrs_ui::popup::render_audit_card_popup`

Call site in umrs-ls:
```rust
render_audit_card_popup(f, f.area(), &sp.app, sp.active_tab, sp.scroll[sp.active_tab], &theme);
```

### Side effects

- `build_detail_lines` in marking_detail.rs got `#[expect(clippy::too_many_lines)]` — cargo fmt split its
  signature across lines, pushing line count from exactly 100 to 145. Rationale: field ordering structure.
- Removed stale `#[expect(too_many_lines)]` from `tui_render::build_file_entry_item` — formatter
  compressed it below 100 lines so the expect was unfulfilled.
