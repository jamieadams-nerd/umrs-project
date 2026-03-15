# tui-cli Reference Collection — Source Index

Collection: tui-cli
Retrieved: 2026-03-15
Purpose: RAG augmentation for TUI/CLI development guidance in the UMRS project.

---

## backend/

### crossterm-overview.md
- Source: https://docs.rs/crossterm/latest/crossterm/
- Content: Crate overview, feature flags, Command API, `execute!` and `queue!` macros,
  module summary.

### crossterm-event.md
- Source: https://docs.rs/crossterm/latest/crossterm/event/index.html
- Content: `poll()`, `read()`, `Event` enum, `KeyEvent`, `KeyCode`, `MouseEvent`,
  `KeyEventKind`/`KeyEventState`, Kitty keyboard enhancement protocol, `EnableMouseCapture`.

### crossterm-terminal.md
- Source: https://docs.rs/crossterm/latest/crossterm/terminal/index.html
- Content: `enable_raw_mode`, `disable_raw_mode`, `size()`, `Clear`/`ClearType`,
  `EnterAlternateScreen`, `LeaveAlternateScreen`, `SetSize`, `ScrollUp`,
  `BeginSynchronizedUpdate`.

### crossterm-style.md
- Source: https://docs.rs/crossterm/latest/crossterm/style/index.html
- Content: `Color`, `Attribute`, style commands (`SetForegroundColor`, `SetBackgroundColor`,
  `ResetColor`), `ContentStyle`, `StyledContent`, `Stylize` trait fluent API, NO_COLOR
  compliance.

### color-eyre.md
- Source: https://docs.rs/color-eyre/latest/color_eyre/
- Content: `install()` placement, `Report` type, `eyre::Result` alias, feature flags,
  `WrapErr`, section helpers (`.with_note()`, `.with_suggestion()`), `bail!`, `ensure!`,
  typical TUI binary error-handling pattern.

### clap.md
- Source: https://docs.rs/clap/latest/clap/
- Source: https://docs.rs/clap/latest/clap/_derive/index.html
- Source: https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
- Content: Derive API (`Parser`, `Args`, `Subcommand`, `ValueEnum`), subcommand patterns,
  `#[command(flatten)]`, nested subcommands, `ValueEnum` restricted values, `value_parser`
  range validation, custom validation functions, env var fallback, help text formatting,
  `arg_required_else_help`, `--json` output pattern, cookbook examples.

---

## architecture/

### ratatui-architecture.md
- Source: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md
- Content: Modular workspace layout (ratatui-core, ratatui-widgets, ratatui-crossterm,
  ratatui-termion, ratatui-termwiz, ratatui-macros), per-crate descriptions, audience
  guidance (app authors vs widget library authors), `DefaultTerminal`, `init()`/`restore()`
  helpers, `Frame`/`draw()` API, `Layout` system, MSRV history.

### ratatui-breaking-changes.md
- Source: https://raw.githubusercontent.com/ratatui/ratatui/main/BREAKING-CHANGES.md
- Content: Full breaking-change log from v0.20.0 through v0.31.0 with diff examples.
  Key migrations: `block::Title` removal, `Alignment` → `HorizontalAlignment`, `Backend`
  associated Error, `Frame` generic removal, `WidgetRef` impl reversal, `Flex::Legacy`.

---

## ratatui-website/  (added 2026-03-15, ratatui v0.30.0)

Content synthesized from ratatui.rs and docs.rs via WebSearch + direct GitHub fetches.
Note: ratatui.rs and docs.rs are not in the WebFetch allowlist; add them to
`.claude/settings.json` to enable verbatim fetches in future update passes.

### architecture.md
- Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/ARCHITECTURE.md
- Content: v0.30.0 modular workspace structure, crate roles, rendering model (immediate
  mode, double buffering), layout system (Cassowary), constraint types, Flex modes,
  widget traits (Widget / StatefulWidget / WidgetRef), no_std support.

### concepts-layout.md
- Source: https://ratatui.rs/concepts/layout/
- Content: Constraint types (Length, Percentage, Ratio, Min, Max, Fill), Flex variants,
  negative spacing, nested layouts, centered_area recipe, destructuring syntax.

### concepts-widgets.md
- Source: https://ratatui.rs/concepts/widgets/ + docs.rs widget index
- Content: Built-in widget catalog, Widget vs StatefulWidget vs WidgetRef traits,
  rendering methods (render_widget, render_stateful_widget), Block container,
  custom widget implementation pattern, styling.

### concepts-backends.md
- Source: https://ratatui.rs/concepts/backends/ + CrosstermBackend docs
- Content: Backend comparison table, CrosstermBackend usage, Terminal struct,
  double-buffering model, DefaultTerminal alias, Frame API (area, render_widget,
  render_stateful_widget, set_cursor_position), raw mode, alternate screen.

### concepts-application-architecture.md
- Source: https://ratatui.rs/concepts/application-patterns/the-elm-architecture/
          https://ratatui.rs/tutorials/counter-app/basic-app/
- Content: Immediate-mode rendering philosophy, basic event loop, App struct pattern,
  event handling (blocking, channel, async/tokio), Elm Architecture (TEA) pattern,
  component decomposition, tick-based timing, architecture decision table.

### api-style.md
- Source: https://docs.rs/ratatui/latest/ratatui/style/struct.Style.html
- Content: Style construction (explicit and Stylize shorthand), Color enum (named,
  Rgb, Indexed, Tailwind palette), Modifier bitflags, Stylize trait methods,
  underline color feature flag, Into<Style> conversions.

### api-widgets-overview.md
- Source: https://docs.rs/ratatui/latest/ratatui/widgets/index.html + struct pages
- Content: Block (borders, border types, padding, inner area), Paragraph (wrapping,
  scroll, alignment), List + ListState, Table + TableState (row/column/cell selection),
  Gauge, Sparkline (absent_value_style v0.29), Scrollbar + ScrollbarState, Tabs,
  Clear (for overlays), Text/Line/Span model with builder methods.

---

## ratatui-examples/  (added 2026-03-15, ratatui v0.30.0)

Raw Rust source files fetched from github.com/ratatui/ratatui tag ratatui-v0.30.0.

### demo2_app.rs
- Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/examples/apps/demo2/src/app.rs
- Tags: tabs, multi-tab, mode-state-machine, App-as-Widget, event-handling, Layout

### popup.rs
- Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/examples/apps/popup/src/main.rs
- Tags: popup, overlay, Clear, Flex::Center, centered_area helper, ratatui::run()

### table.rs
- Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/examples/apps/table/src/main.rs
- Tags: Table, TableState, Scrollbar, row/column/cell selection, Tailwind palette

### scrollbar.rs
- Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/examples/apps/scrollbar/src/main.rs
- Tags: Scrollbar, ScrollbarState, tick-rate, vertical/horizontal, Masked, Margin

### flex_layouts.rs
- Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/examples/apps/flex/src/main.rs
- Tags: Flex, all Constraint variants, split_with_spacers, Buffer::empty, scrollable demo

---

## awesome-ratatui/  (added 2026-03-15)

### README.md
- Source: https://raw.githubusercontent.com/ratatui/awesome-ratatui/main/README.md
- Content: Curated list of ratatui libraries (frameworks, widgets, utilities) and
  applications (dev tools, sysadmin, networking, productivity, games, etc.)

---

## Update Check

To check for newer content:
- crossterm: https://crates.io/crates/crossterm — compare published version against fetched docs
- color-eyre: https://crates.io/crates/color-eyre
- clap: https://crates.io/crates/clap
- ratatui releases: https://github.com/ratatui/ratatui/releases
- ratatui BREAKING-CHANGES: https://raw.githubusercontent.com/ratatui/ratatui/main/BREAKING-CHANGES.md
- ratatui ARCHITECTURE: https://raw.githubusercontent.com/ratatui/ratatui/main/ARCHITECTURE.md
- awesome-ratatui: https://raw.githubusercontent.com/ratatui/awesome-ratatui/main/README.md
