# Backends

<!-- source: https://ratatui.rs/concepts/backends/ -->
<!-- source: https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html -->
<!-- fetched: 2026-03-15 (via WebSearch) -->

Ratatui delegates all terminal I/O to a pluggable backend. The backend abstracts the platform
and terminal library differences so the rest of the codebase is portable.

## Available Backends

| Backend | Crate | Platforms |
|---|---|---|
| `CrosstermBackend` | `ratatui-crossterm` (default) | Linux, macOS, Windows |
| `TermionBackend` | `ratatui-termion` | Unix only |
| `TermwizBackend` | `ratatui-termwiz` | Cross-platform |

**Crossterm is the default** and the correct choice for most applications. It is supported
on Linux, macOS, and Windows.

## CrosstermBackend

`CrosstermBackend` is a wrapper around a writer implementing `Write`, which is used to send
commands to the terminal. It provides methods for drawing content, manipulating the cursor,
and clearing the terminal screen.

Most applications should not call methods on `CrosstermBackend` directly. Use the `Terminal`
struct instead.

### Terminal Setup Pattern

```rust
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::execute;
use std::io::stdout;

// Preferred: use the convenience functions (v0.28+)
let terminal = ratatui::init();
// ... run app ...
ratatui::restore();

// Manual setup (older style, still valid):
enable_raw_mode()?;
execute!(stdout(), EnterAlternateScreen)?;
let backend = CrosstermBackend::new(stdout());
let mut terminal = Terminal::new(backend)?;
```

## Terminal Struct

`Terminal` is the main entry point for Ratatui. It is responsible for:
- Drawing and maintaining state of the buffers
- Cursor management
- Viewport management

It is generic over a `Backend` implementation.

### Double Buffering

The `Terminal` struct maintains **two buffers**: the current and the previous. When widgets
are drawn, changes accumulate in the current buffer. At the end of each draw pass, the two
buffers are compared, and only the changes between them are written to the terminal, avoiding
redundant operations. After flushing, the buffers are swapped.

### DefaultTerminal

`DefaultTerminal` is a type alias for `Terminal<CrosstermBackend<Stdout>>`. It is the return
type of `ratatui::init()`.

```rust
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame))?;
        if handle_events()? { break; }
    }
    Ok(())
}
```

## Raw Mode and Alternate Screen

Ratatui applications require two terminal features:

**Raw mode**: Disables line buffering and echo. Key presses are delivered immediately and are
not displayed automatically.

**Alternate screen**: A separate screen buffer. Exiting restores the terminal's previous content.

`ratatui::init()` enables both automatically and registers a panic hook to restore the terminal
even if the application panics. Always call `ratatui::restore()` or use `ratatui::init()`
to ensure the terminal is left in a usable state.

## Frame

`Frame` provides a consistent view into the terminal state for rendering. It is passed as
`&mut Frame` to the closure given to `Terminal::draw`.

Key methods:
- `frame.area()` — the full terminal area as `Rect`
- `frame.render_widget(widget, area)` — render a `Widget`
- `frame.render_stateful_widget(widget, area, &mut state)` — render a `StatefulWidget`
- `frame.render_widget_ref(widget, area)` — render using `WidgetRef` (shared reference)
- `frame.set_cursor_position(pos)` — position the cursor (for text input widgets)

## Modular Backends (v0.30.0+)

In v0.30.0, backends were split into separate workspace crates (`ratatui-crossterm`,
`ratatui-termion`, `ratatui-termwiz`). This allows backend changes to evolve independently
of the core library. The umbrella `ratatui` crate re-exports the default backend for convenience.

Widget library authors should depend on `ratatui-core` only and not on any backend crate.
