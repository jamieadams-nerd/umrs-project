# crossterm::terminal — Terminal Module

Source: https://docs.rs/crossterm/latest/crossterm/terminal/index.html
Retrieved: 2026-03-15

## Overview

The `terminal` module provides functionality to work with the terminal: raw mode, alternate
screen, terminal size queries, and clearing. Most operations are issued as commands via the
`execute!` or `queue!` macros.

## Raw Mode

Raw mode bypasses the terminal driver's normal line-buffered, canonical processing. In raw mode:
- Input is delivered character-by-character (no Enter required)
- Special characters (Ctrl+C, backspace) are not processed by the driver
- Output is not line-buffered

### `enable_raw_mode() -> Result<()>`

Enables raw mode. On Unix, stores original `termios` state in a `Mutex<Option<Termios>>` for
later restoration. On Windows, uses Console API bit-masking.

### `disable_raw_mode() -> Result<()>`

Restores the terminal to its previous canonical state.

### Best Practices

Always pair `enable_raw_mode` with `disable_raw_mode` — use a cleanup guard or `Drop` impl to
ensure restoration even on panic or early return. In raw mode, `println!` may not behave
correctly (no CR/LF translation); use `write!` with explicit `\r\n`.

```rust
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    // ... run TUI ...
    disable_raw_mode()?;
    Ok(())
}
```

Ratatui's `init()` helper calls `enable_raw_mode` for you and installs a panic hook that
restores the terminal before printing the panic message.

## `size() -> Result<(u16, u16)>`

Returns the current terminal size as `(columns, rows)`.

```rust
use crossterm::terminal::size;

let (cols, rows) = size()?;
```

## `Clear` Command

Clears portions of the terminal screen buffer. Used with `ClearType` variants:

```rust
use crossterm::{execute, terminal::{Clear, ClearType}};

execute!(std::io::stdout(), Clear(ClearType::All))?;
```

`ClearType` variants:

| Variant            | Effect                                                |
|--------------------|-------------------------------------------------------|
| `All`              | Clear entire screen, move cursor to top-left          |
| `Purge`            | Clear entire screen including scrollback              |
| `FromCursorDown`   | Clear from cursor to end of screen                    |
| `FromCursorUp`     | Clear from cursor to beginning of screen              |
| `CurrentLine`      | Clear entire current line                             |
| `UntilNewLine`     | Clear from cursor to end of current line              |

## Alternate Screen

The terminal has two screen buffers: the **main screen** (with scrollback) and the **alternate
screen** (exact terminal dimensions, no scrollback). Switching to the alternate screen is how
programs like Vim leave the shell buffer intact and restore it cleanly on exit.

### `EnterAlternateScreen` / `LeaveAlternateScreen`

```rust
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

fn main() -> std::io::Result<()> {
    execute!(std::io::stdout(), EnterAlternateScreen)?;

    // run TUI on alternate screen ...

    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
```

The main screen content is preserved while the alternate screen is active. All drawing
operations happen on the alternate screen; leaving it restores the original shell output.

## `SetSize(columns, rows)` / `ScrollUp(n)` / `ScrollDown(n)`

```rust
use crossterm::{execute, terminal::{ScrollUp, SetSize, size}};

let (cols, rows) = size()?;
execute!(std::io::stdout(), SetSize(80, 24), ScrollUp(5))?;
// restore original size when done
execute!(std::io::stdout(), SetSize(cols, rows))?;
```

## `BeginSynchronizedUpdate` / `EndSynchronizedUpdate`

Wraps a batch of drawing commands in a synchronized update (suppresses intermediate redraws on
supporting terminals). Ratatui issues these automatically around each `draw()` call.

## Common Import Block for TUI Setup

```rust
use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
        size, Clear, ClearType,
    },
};
```
