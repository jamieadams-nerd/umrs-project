---
name: umrs-label TUI module
description: LabelRegistryApp, tui/app.rs, tui/render.rs, DisseminationControl, main.rs dispatch
type: project
---

## umrs-label KeyMap wiring (2026-04-08, Elena E-2)

`Action::PanelSwitch` added to `libs/umrs-ui/src/keymap.rs` (after `Back` variant in ViewerApp section).
When adding a new `Action` variant, three exhaustive match sites in umrs-ui must be updated:
- `libs/umrs-ui/src/app.rs` — `AuditCardState::handle_action` (add to the "not relevant" arm)
- `libs/umrs-ui/src/config/mod.rs` — `ConfigState::handle_action` (add to the "not relevant" arm)
- `libs/umrs-ui/src/viewer/mod.rs` — `ViewerState::handle_action` (add to the "not relevant" arm)

`main.rs` event loop now uses `keymap.lookup(&key)` in normal mode instead of `match key.code`.
Keymap overrides added in `run_tui` after `KeyMap::default()`:
- Tab/BackTab → `PanelSwitch` (overrides default NextTab/PrevTab)
- Right/Left → `Expand`/`Collapse` (overrides default NextTab/PrevTab)
- `l`/`h` → `Expand`/`Collapse`
- Space → `DialogConfirm` (overrides default `Expand`; Space+Enter both call `handle_enter()`)
- `Q`/`Q+SHIFT` → `Quit` (uppercase Q in addition to lowercase)

Key semantic distinction preserved:
- `DialogConfirm` (Enter, Space) → `handle_enter()` — expands branches AND loads leaf details
- `Expand` (Right, l) → `expand_selected()` — purely expands, no detail loading

Ctrl+C handled in `_` arm with direct key.code check (not in keymap — modifier pattern overlap).

## umrs-label review fixes (2026-04-08)

- `validate.rs`: `OnceLock<Mutex<HashMap>>` replaced with per-variant `static CUI_MARKING_RE: OnceLock<Regex>`. `get_regex()` now returns `&'static Regex`.
- `validate.rs` regex updated to `^CUI(//(SP-)?[A-Z][-A-Z]*(/(SP-)?[A-Z][-A-Z]*)*(//[A-Z][-A-Z /,]*)?)?$` — supports plain `CUI`, LDC banners (`CUI//LEI//NOFORN`, `CUI//LEI//FED ONLY`, `CUI//SP-CTI//REL TO USA, CAN`).
- `cui/catalog.rs`: `policy_aware_description(description: &str) -> String` — replaces enforcement language under targeted policy; preserves under MLS. Import: `use crate::cui::catalog::policy_aware_description;`
- `tui/app.rs`: description_en/fr now call `policy_aware_description()` in both `marking_to_detail()` and `dc_to_detail()`.
- `main.rs` (umrs-label, umrs-stat): `NO_COLOR` check present; `Theme::no_color()` TODO pending umrs-ui implementation.
- `umrs-ls/main.rs`: NO_COLOR gates `--color` flag: `args.contains("--color") && std::env::var("NO_COLOR").is_err()`

## umrs-label TUI (Phase 2, 2026-04-07)

New files:
- `src/tui/mod.rs` — declares `pub mod app; pub mod render;`
- `src/tui/app.rs` — `LabelRegistryApp`, `Panel` enum (`Tree`/`Detail`), `DetailContent` enum
- `src/tui/render.rs` — `render_label_registry()` master entry point (same pattern as umrs-ls)

`lib.rs` has `pub mod tui;` between `pub mod cui;` and `pub mod validate;`.

### LabelRegistryApp API

```rust
LabelRegistryApp::new(us_catalog: Catalog, ca_catalog: Option<Catalog>) -> Self
LabelRegistryApp::build_tree() -> TreeModel
LabelRegistryApp::total_markings() -> usize
LabelRegistryApp::us_catalog() -> &Catalog
LabelRegistryApp::ca_catalog() -> Option<&Catalog>
LabelRegistryApp::marking_detail_us(key: &str) -> Option<MarkingDetailData>
LabelRegistryApp::marking_detail_ca(key: &str) -> Option<MarkingDetailData>
LabelRegistryApp::dissemination_detail(key: &str) -> Option<MarkingDetailData>
LabelRegistryApp::us_catalog_metadata() -> Vec<(String, String)>
LabelRegistryApp::ca_catalog_metadata() -> Vec<(String, String)>
```

Tree node `kind` metadata values: `catalog_root`, `group`, `dc_branch`, `marking_leaf`, `dc_leaf`.
`is_us` metadata: `"1"` = US catalog, `"0"` = CA catalog.

### DisseminationControl struct (in cui/catalog.rs)

New type for US LDC entries. Fields:
- `name: LocaleText`, `description: LocaleText`
- `banner_marking: Option<String>`, `portion_marking: Option<String>`
- `parameterized: bool`, `category_restriction: Option<String>`
- `mutually_exclusive_with: Vec<String>`

`Catalog` has `dissemination_controls: HashMap<String, DisseminationControl>` with `#[serde(default)]`.
Methods: `iter_dissemination_controls()`, `has_dissemination_controls()`.

### main.rs dispatch

`main()` → `()`. Mode resolution:
1. `--json` → stub (reserved)
2. `--cli` or non-TTY → `run_cli(&us_catalog, ca_catalog.as_ref())`
3. Default → `run_tui(us_catalog, ca_catalog)`

CLI flags: `--us-catalog <path>` (default: `config/us/US-CUI-LABELS.json`), `--ca-catalog <path>` (default: `config/ca/CANADIAN-PROTECTED.json`).

### Cargo.toml additions

Added: `umrs-ui`, `umrs-selinux`, `ratatui = "0.30.0"`, `crossterm = "0.28"`, `chrono = "0.4"`, `nix = { version = "0.27", features = ["user", "fs"] }`, `env_logger = "0.11"`.

### Test status

37 tests pass. 7 pre-existing `setrans_tests` failures (CUI catalog rebuild in progress, unrelated).

### Example

`examples/label_registry.rs` — demonstrates `LabelRegistryApp::new()`, `build_tree()`, `marking_detail_us()`, `dissemination_detail()`, `us_catalog_metadata()`.
