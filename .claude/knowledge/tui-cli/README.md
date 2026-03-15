# tui-cli Knowledge Collection

Collection: `tui-cli`
Familiarization pass: 2026-03-15
Ratatui version: v0.30.0
Document count: 26 files across 6 subdirectories

---

## Coverage Summary

This collection covers TUI and CLI development reference material for the UMRS project. It
provides authoritative guidance on:

- **Ratatui** (v0.30.0): application architecture, widget system, layout constraints, styling,
  backends, and breaking change history from v0.20.0 through v0.31.0
- **crossterm**: event handling (keyboard, mouse, focus, resize), terminal control (raw mode,
  alternate screen), and styling with NO_COLOR compliance
- **clap** (v4 derive API): argument parsing, subcommands, `ValueEnum`, validation, `--json` pattern
- **color-eyre**: error handling for TUI binaries, `WrapErr`, section helpers, panic hook integration
- **CLIG** (CLI Guidelines): UX requirements for output design, error messages, flags, exit codes
- **NO_COLOR standard**: the unconditional color suppression contract
- **Ratatui examples**: five working Rust programs demonstrating multi-tab apps, popups, tables,
  scrollbars, and flex layout exploration
- **Ecosystem catalog**: awesome-ratatui and awesome-tuis for design inspiration and third-party crate discovery

---

## Artifact Files

| File | Description |
|---|---|
| `concept-index.md` | One entry per document: what it covers, key terms, what it governs, cross-references |
| `cross-reference-map.md` | Agreements, tensions, chains, and gaps across the corpus |
| `style-decision-record.md` | Project-specific rulings on corpus tensions; open placeholders for Jamie decisions |
| `term-glossary.md` | Canonical terminology from the domain with source citations and usage notes |

---

## Top Findings

### 1. `WidgetRef` blanket impl was reversed in v0.30.0

The correct pattern for new widgets is `impl Widget for &YourType`, NOT `impl WidgetRef for YourType`.
Any existing UMRS code using the old form will fail to compile against ratatui 0.30+. The breaking
changes document (ARCH-2) provides the migration diff. This affects `umrs-tui` if it has custom widgets.

### 2. v0.30.0 breaking changes are numerous and require active migration checking

`block::Title` removed, `Alignment` renamed, `Flex::SpaceAround` semantics changed, `Style` no
longer implements `Styled`, `Backend` requires associated `Error` type. Any UMRS crate targeting
ratatui 0.30.0 should be audited against the full breaking change list in `cross-reference-map.md`.

### 3. The corpus confirms: `poll(timeout)` not blocking `read()` for UMRS tools

All example code for tick-based applications uses `event::poll(timeout)?` before `event::read()`.
This is the correct pattern for UMRS security posture tools that must refresh independently of
user input. The style decision record (SDR-4) codifies this as a project ruling.

### 4. NO_COLOR compliance is a two-layer concern

crossterm suppresses color automatically for TUI rendering paths. However, any UMRS code that
produces ANSI-colored output outside of ratatui (e.g., direct println! with escape codes) must
perform an explicit `NO_COLOR` check. SDR-3 captures this distinction.

### 5. Supply chain gap: third-party widget crates need hygiene review before adoption

The awesome-ratatui catalog (AWE-1) lists useful third-party widgets (`tui-logger`, `tui-popup`,
`ratatui-textarea`). Before any of these are added as dependencies, the project's architectural
review trigger for new external crates applies: supply chain hygiene assessment, security-engineer
approval, Jamie notification. These are not pre-approved.

---

## Open Questions for Jamie

Three style decisions require project owner input — see `style-decision-record.md`:

- **PH-1:** Should UMRS tools adopt async tokio event handling?
- **PH-2:** Should any UMRS TUI enable mouse capture (`EnableMouseCapture`)?
- **PH-3:** Should UMRS ship shell completion scripts via `clap_complete`?
