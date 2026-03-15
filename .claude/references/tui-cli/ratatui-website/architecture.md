# Ratatui Architecture

<!-- source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/ARCHITECTURE.md -->
<!-- source: https://github.com/ratatui/ratatui/blob/ratatui-v0.30.0/ARCHITECTURE.md -->
<!-- fetched: 2026-03-15 -->

## Overview

Ratatui underwent a significant restructuring in version 0.30.0, transitioning from a single
crate into a modular workspace.

## Core Structure

The project now consists of specialized crates serving different purposes:

**Foundation**: `ratatui-core` provides "foundational types and traits for the Ratatui ecosystem,"
making it ideal for widget authors who need stability.

**Main Entry Point**: The `ratatui` crate "re-exports everything from other crates for
convenience," remaining the standard choice for most application developers.

**Widgets and Backends**: Separate crates handle widgets (Block, Paragraph, List, Chart) and
backends (crossterm, termion, termwiz), each depending on the core foundation.

## Crate Layout

```
ratatui-core       — foundational types and traits (Buffer, Cell, Rect, Color, Style, Text, Widget, ...)
ratatui-widgets    — built-in widgets (Block, Paragraph, List, Table, Chart, Gauge, Sparkline, ...)
ratatui-crossterm  — crossterm backend
ratatui-termion    — termion backend
ratatui-termwiz    — termwiz backend
ratatui            — umbrella crate; re-exports everything for application convenience
```

## Key Benefits of Modularization

1. **Compilation speed**: Widget libraries only compile core types, not the full stack.
2. **Stability**: `ratatui-core` is designed for maximum stability to minimize breaking changes
   for widget libraries.
3. **Flexibility**: Applications can selectively include only needed components.

## Migration Impact

All existing code using the `ratatui` crate will continue to work unchanged. Widget library
authors should consider migrating to `ratatui-core` for enhanced stability and independence
from broader ecosystem changes.

## Rendering Model

Ratatui uses **immediate rendering with intermediate buffers**. Each frame, the application must
render all widgets that should be part of the UI. This is in contrast to retained-mode rendering
where widgets are updated and automatically redrawn.

The `Terminal` struct maintains two buffers: the current and the previous. When widgets are drawn,
changes accumulate in the current buffer. At the end of each draw pass, the two buffers are
compared, and only the changes between them are written to the terminal. After flushing, the
buffers are swapped for the next draw cycle.

## Layout System

Ratatui uses the **Cassowary** constraint solver algorithm to determine rectangle sizes. When not
all constraints can be simultaneously satisfied, the solver returns a close approximation.

Layout types:
- `Layout::vertical([...])` — divide an area vertically
- `Layout::horizontal([...])` — divide an area horizontally
- Layouts can be nested arbitrarily

Constraint variants:
- `Constraint::Length(u16)` — absolute size in rows/columns
- `Constraint::Percentage(u16)` — relative to parent
- `Constraint::Ratio(u16, u16)` — fractional proportion
- `Constraint::Min(u16)` — minimum size, grows to absorb excess space
- `Constraint::Max(u16)` — maximum size
- `Constraint::Fill(u16)` — fills excess space proportionally (v0.26+)

Flex layout modes (v0.26+):
- `Flex::Legacy` — all excess space at the end (previous default behavior)
- `Flex::Start` — items at start, excess at end
- `Flex::End` — items at end, excess at start
- `Flex::Center` — items centered
- `Flex::SpaceEvenly` — space distributed evenly between items and edges
- `Flex::SpaceBetween` — space between items only
- `Flex::SpaceAround` — space around each item

## Widget System

Widgets implement one of two traits:

```rust
pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}

pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

Recommended pattern for new widgets: implement `Widget` on `&YourWidget` (shared reference)
rather than consuming `self`. This allows reuse across frames.

## no_std Support

Added in v0.30.0. `ratatui-core` and `ratatui-widgets` support `no_std` targets, enabling
use in embedded environments.
