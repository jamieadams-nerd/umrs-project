**Status:** completed
**ROADMAP:** G3 (CUI labeling), M2 (TUI tooling)

# Security Label Registry — umrs-label TUI Implementation Plan

## Overview

Read-only TUI browser for CUI/Protected label catalogs. Named "Security Label Registry."
Three modes: TUI (default), CLI (`--cli`), JSON (`--json`).

## Phases

### Phase 0: Fix catalog deserialization (BLOCKING)
- Create `LocaleText` type in `catalog.rs` — struct wrapping HashMap, custom Deserialize for both flat string and locale object
- Update `Marking`: `name`/`description` become `LocaleText`, remove `_fr` fields
- Update `CatalogMetadata`: `catalog_name` becomes `LocaleText`
- Handle CA `marking_banner`, `injury_examples`, `handling` sub-fields
- Update `main.rs` CLI output to use `.en()`
- All tests pass

### Phase 1: Marking detail renderer in umrs-ui (parallel with Phase 0)
- New `marking_detail.rs` in umrs-ui
- `MarkingDetailData` struct — all owned Strings, no dependency on umrs-label
- `render_marking_detail()` — textwrap, bilingual display, designation coloring
- Reusable by umrs-ls (Enter on marking header) and umrs-label TUI

### Phase 2: Security Label Registry TUI
- `LabelRegistryApp` implementing `ViewerApp`
- Loads US + CA catalogs into single tree
- Tree: Catalog root → Index Groups → Markings + Dissemination Controls branch
- Two-panel layout: tree (40%) | detail (60%), Tab to switch focus
- Security posture header (same as umrs-ls) + wizard logo
- Catalog-specific header row: blank for now (future: signature/integrity status)
- Enter: root → metadata, group → expand/collapse, marking → detail panel
- Custom `render_label_registry()` renderer

### Phase 3: Polish
- Search/filter, help dialog, status bar, detail scrolling

## Key Design Decisions
- `LocaleText` typed struct, not raw HashMap — path to SafeText
- Detail panel in umrs-ui via generic `MarkingDetailData` (no circular dep)
- Both catalogs in one tree, not tabbed
- Name: "Security Label Registry"
- Header: full security posture (future: catalog file signature/integrity status)
