# Ratatui — Architecture

Source: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md
Retrieved: 2026-03-15
Note: Content synthesized from ARCHITECTURE.md and official crate documentation.

## Overview

Starting with version 0.30.0, Ratatui is organized as a modular Cargo workspace. The `ratatui`
crate is the main entry point for most applications — it re-exports everything from the member
crates. The workspace was modularized to improve compilation times, API stability, and dependency
management for third-party widget libraries.

## Workspace Crate Layout

```
ratatui/
├── ratatui-core          # core traits and primitive types
├── ratatui-widgets       # built-in widget implementations  → ratatui-core
├── ratatui-crossterm     # Crossterm backend               → ratatui-core
├── ratatui-termion       # Termion backend                 → ratatui-core
├── ratatui-termwiz       # Termwiz backend                 → ratatui-core
└── ratatui-macros        # proc-macro helpers
```

## Crate Descriptions

### `ratatui-core`

Provides the foundational types and traits that all other crates depend on.

Contents:
- Widget traits: `Widget`, `StatefulWidget`, `WidgetRef`
- Text types: `Span`, `Line`, `Text`
- Layout types: `Rect`, `Layout`, `Constraint`, `Direction`, `Margin`, `HorizontalAlignment`
- Buffer: `Buffer`, `Cell`
- Style types: `Style`, `Color`, `Modifier`, `Stylize`
- Symbol sets: border, braille, block, marker

Target audience: widget library authors who want minimal dependencies.

### `ratatui-widgets`

Contains all built-in widget implementations that were previously part of the main `ratatui`
crate. Most application authors do not depend on this directly — `ratatui` re-exports everything.

Widget library authors who use built-in widgets internally may depend on this crate directly
for finer-grained dependency control.

Built-in widgets include: `Block`, `Paragraph`, `List`, `Table`, `Tabs`, `Gauge`,
`LineGauge`, `BarChart`, `Sparkline`, `Chart`, `Canvas`, `Scrollbar`, `Calendar`.

### `ratatui-crossterm`

Provides the `CrosstermBackend` for Ratatui. Manages the Crossterm version via feature flags
(`crossterm_0_28`, `crossterm_0_29`, etc.) — the highest enabled flag wins.

The selected Crossterm crate is re-exported as `ratatui_crossterm::crossterm`. Widget library
authors should use this re-export rather than adding their own `crossterm` dependency, to
avoid version conflicts with the version Ratatui uses.

### `ratatui-termion` / `ratatui-termwiz`

Alternative backends for Unix-only (termion) and cross-platform with advanced features
(termwiz). Most UMRS work uses `ratatui-crossterm`.

### `ratatui-macros`

Procedural macro helpers. Provides `ratatui::layout` shorthand macros and similar ergonomics.

## Who Should Use What

**Application authors** — use the main `ratatui` crate. It re-exports everything; no need to
add `ratatui-core` or `ratatui-widgets` separately.

```toml
[dependencies]
ratatui = "0.29"
```

**Widget library authors** — depend only on `ratatui-core` to avoid pulling in backend code
or built-in widgets, and to remain compatible as the workspace evolves.

```toml
[dependencies]
ratatui-core = "0.1"
```

## Key Architectural Decisions

### `DefaultTerminal` Type Alias

`ratatui` provides `DefaultTerminal` as a type alias for `Terminal<CrosstermBackend<Stdout>>`.
Application code should use this alias rather than spelling out the generic:

```rust
fn run(mut terminal: ratatui::DefaultTerminal) -> color_eyre::Result<()> { ... }
```

### `init()` and `restore()` Helpers

`ratatui::init()` is the recommended entry point. It:
1. Enables raw mode
2. Enters the alternate screen
3. Installs a panic hook that restores the terminal before printing panic messages
4. Returns a `DefaultTerminal`

`ratatui::restore()` is the paired cleanup call:
1. Leaves the alternate screen
2. Disables raw mode

```rust
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}
```

### `Frame` and `draw()`

The primary rendering API: `terminal.draw(|frame| { ... })`. The closure receives a `&mut Frame`
which provides:
- `frame.area()` — terminal dimensions as `Rect`
- `frame.render_widget(widget, area)` — render a widget into a region
- `frame.render_stateful_widget(widget, area, state)` — render with external state
- `frame.set_cursor_position(Position)` — show cursor at a position

### Layout System

```rust
use ratatui::layout::{Layout, Constraint, Direction};

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(0)])
    .split(frame.area());
```

`Layout` results are cached internally (LRU cache, configurable with `Layout::init_cache`).
As of v0.30.0, the cache is behind the `layout-cache` feature (enabled by default).

### `HorizontalAlignment` (renamed from `Alignment` in v0.30.0)

```rust
use ratatui::layout::HorizontalAlignment;
// Type alias `Alignment` still available for backwards compatibility
```

## Versioning

All workspace crates are currently versioned together for simplicity. Independent versioning
may be adopted in a future release once the split API stabilizes.

## MSRV History

| Version | MSRV   |
|---------|--------|
| v0.31.0 | 1.88.0 |
| v0.30.0 | 1.86.0 |
| v0.24.0 | 1.70.0 |
| v0.23.0 | 1.67.0 |
| v0.21.0 | 1.65.0 |
| v0.20.0 | 1.63.0 |
