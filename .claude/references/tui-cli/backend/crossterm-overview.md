# crossterm — Crate Overview

Source: https://docs.rs/crossterm/latest/crossterm/
Retrieved: 2026-03-15

## Overview

`crossterm` is a pure-Rust, cross-platform terminal manipulation library. It supports all UNIX
and Windows terminals down to Windows 7. Tested terminals include Console Host (Windows 10/8.1),
Windows Terminal, Ubuntu Desktop Terminal, KDE Konsole, Kitty, Alacritty, macOS Monterey, and
macOS Sonoma.

Notable users: Broot, Cursive, Ratatui, Rust-sloth, Rusty-rain.

License: MIT

## Adding to Cargo.toml

```toml
[dependencies]
crossterm = "0.29"
```

## Feature Flags

| Feature            | Default | Description                                          |
|--------------------|---------|------------------------------------------------------|
| `bracketed-paste`  | yes     | Enables `Event::Paste` when pasting text             |
| `events`           | yes     | Enables reading input/events from the system         |
| `windows`          | yes     | Enables Windows-specific crates                      |
| `event-stream`     | no      | Enables `EventStream` for async event reading        |
| `serde`            | no      | Serialize/deserialize events                         |
| `derive_more`      | yes     | Adds `is_*` helper functions for event types         |
| `base64` / `osc52` | no      | Clipboard data encoding for OSC52 sequences          |
| `futures-core`     | no      | Async stream of events (requires `event-stream`)     |

## The Command API

The Command API makes crossterm much easier to use and provides fine-grained control over when
and how a command is executed. A command is an action you can perform on the terminal (e.g.,
cursor movement). Commands implement the `Command` trait, which abstracts platform-specific
details: ANSI sequences on Unix/Windows 10, direct WinAPI calls on older Windows.

Benefits: better performance, complete control over flush timing, control over which writer
receives commands, cleaner API.

### `execute!` Macro

Executes one or more commands immediately and flushes.

```rust
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};

execute!(std::io::stdout(), MoveTo(5, 5), Clear(ClearType::All))?;
```

Multiple commands execute left to right. Method chaining is also supported:

```rust
stdout.execute(MoveTo(5, 5))?.execute(Clear(ClearType::All))?;
```

### `queue!` Macro

Queues one or more commands for deferred execution. Commands run when you call `flush()` on the
writer, when the buffer is full, or (for stdout) at each newline.

```rust
use crossterm::{queue, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::Write;

queue!(stdout, MoveTo(5, 5), Clear(ClearType::All))?;
stdout.flush()?;
```

### When to Use Which

- `execute!` — simpler, flushes immediately; suitable for most applications
- `queue!` + manual `flush()` — preferred for real-time TUI editors where batching matters

## Module Summary

| Module      | Purpose                                                |
|-------------|--------------------------------------------------------|
| `event`     | Read keyboard, mouse, focus, paste, and resize events  |
| `terminal`  | Raw mode, alternate screen, size queries, clear        |
| `style`     | Colors, attributes, styled content                     |
| `cursor`    | Cursor movement, visibility, shape                     |
| `clipboard` | OSC52 clipboard access (feature-gated)                 |
