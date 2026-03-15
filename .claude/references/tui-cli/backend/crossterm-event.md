# crossterm::event — Event Module

Source: https://docs.rs/crossterm/latest/crossterm/event/index.html
Retrieved: 2026-03-15

## Overview

The `event` module provides functions and types for reading keyboard, mouse, focus, paste, and
resize events from the terminal. Raw mode must be enabled for keyboard events to work correctly.

## Core Functions

### `poll(timeout: Duration) -> Result<bool>`

Checks whether an `Event` is available within the given time period. Returns `true` if a
subsequent `read()` call will not block. Use `Duration::from_secs(0)` for a non-blocking check.

### `read() -> Result<Event>`

Reads the next `Event`. Blocks until an event is available. Guaranteed not to block when
`poll()` returned `true`.

### Typical Event Loop Pattern

```rust
use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

fn run() -> io::Result<()> {
    loop {
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => break,
                Event::Key(key) => handle_key(key),
                Event::Resize(w, h) => handle_resize(w, h),
                _ => {}
            }
        }
    }
    Ok(())
}
```

## The `Event` Enum

```rust
pub enum Event {
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Resize(u16, u16),  // (columns, rows)
}
```

Convenience accessors:
- `as_key_event()` — returns `Option<KeyEvent>` for press events only (not release/repeat)
- `as_key_repeat_event()` — returns `Option<KeyEvent>` for repeat events
- `as_mouse_event()` — returns `Option<&MouseEvent>` if this is a mouse event

Note: `Mouse` and `FocusGained`/`FocusLost` events are not enabled by default. Enable them
with the `EnableMouseCapture` and `EnableFocusChange` commands. `Paste` requires
`bracketed-paste` feature and `EnableBracketedPaste` command.

## `KeyEvent` Struct

```rust
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
    pub state: KeyEventState,
}
```

- `code` — The key itself.
- `modifiers` — Bitflags: `NONE`, `SHIFT`, `CONTROL`, `ALT`, `SUPER`, `HYPER`, `META`.
- `kind` — `Press`, `Release`, or `Repeat`. Release/Repeat only available with keyboard
  enhancement flags enabled.
- `state` — `KEYPAD`, `CAPS_LOCK`, `NUM_LOCK` (requires enhancement flags).

Helper methods: `is_press()`, `is_release()`, `is_repeat()`.

### Control Character Mapping

Control characters (0x01–0x1A, 0x1C–0x1F) are automatically converted. For example, Ctrl+C
(0x03) becomes `KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. }`.

## `KeyCode` Enum (selected variants)

```
Backspace, Enter, Left, Right, Up, Down, Home, End, PageUp, PageDown,
Tab, BackTab, Delete, Insert, Null, Esc, CapsLock, ScrollLock, NumLock,
PrintScreen, Pause, Menu, KeypadBegin,
F(u8),                   // F1–F35
Char(char),              // any printable character
Media(MediaKeyCode),     // Play, Pause, Stop, etc.
Modifier(ModifierKeyCode)// standalone modifier key press (with enhancement)
```

## `MouseEvent` Struct

```rust
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}
```

`MouseEventKind` variants: `Down(MouseButton)`, `Up(MouseButton)`, `Drag(MouseButton)`,
`Moved`, `ScrollDown`, `ScrollUp`, `ScrollLeft`, `ScrollRight`.

## Keyboard Enhancement (Kitty Protocol)

For access to key release events, repeat events, and keypad disambiguation:

```rust
use crossterm::{execute, event::{PushKeyboardEnhancementFlags, KeyboardEnhancementFlags}};

execute!(
    stdout,
    PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
    )
)?;

// Restore on exit:
execute!(stdout, PopKeyboardEnhancementFlags)?;
```

## EnableMouseCapture / DisableMouseCapture

```rust
use crossterm::{execute, event::{EnableMouseCapture, DisableMouseCapture}};

execute!(stdout, EnableMouseCapture)?;
// ... run event loop ...
execute!(stdout, DisableMouseCapture)?;
```
