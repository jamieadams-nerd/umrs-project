# crossterm::style — Style Module

Source: https://docs.rs/crossterm/latest/crossterm/style/index.html
Retrieved: 2026-03-15

## Overview

The `style` module provides types and commands for controlling terminal text appearance:
foreground/background colors, text attributes (bold, italic, underline, etc.), and styled
content output. Crossterm uses ANSI sequences on Unix and Windows 10+.

## Color Type

```rust
pub enum Color {
    Reset,
    Black, DarkGrey,
    Red, DarkRed,
    Green, DarkGreen,
    Yellow, DarkYellow,
    Blue, DarkBlue,
    Magenta, DarkMagenta,
    Cyan, DarkCyan,
    White, Grey,
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),           // 256-color palette index
}
```

## Attribute Type

```rust
pub enum Attribute {
    Reset,
    Bold, Dim,
    Italic, Underlined,
    DoubleUnderlined, Undercurled, Underdotted, Underdashed,
    SlowBlink, RapidBlink,
    Reverse, Hidden, CrossedOut,
    Fraktur, NoBold,
    NormalIntensity, NoItalic, NoUnderline, NoBlink,
    NoReverse, NoHidden, NotCrossedOut,
    Framed, Encircled, OverLined,
    NotFramedOrEncircled, NotOverLined,
}
```

## Style Commands

These are issued via `execute!` / `queue!`:

```rust
use crossterm::{execute, style::{
    SetForegroundColor, SetBackgroundColor, SetAttribute, ResetColor, Color, Attribute
}};

execute!(
    stdout,
    SetForegroundColor(Color::Red),
    SetBackgroundColor(Color::Black),
    SetAttribute(Attribute::Bold),
)?;
// write text ...
execute!(stdout, ResetColor)?;
```

### Available Commands

| Command                           | Effect                                       |
|-----------------------------------|----------------------------------------------|
| `SetForegroundColor(Color)`       | Set text foreground color                    |
| `SetBackgroundColor(Color)`       | Set text background color                    |
| `SetUnderlineColor(Color)`        | Set underline color                          |
| `SetAttribute(Attribute)`         | Set a single text attribute                  |
| `SetAttributes(Attributes)`       | Set multiple text attributes                 |
| `ResetColor`                      | Reset foreground and background to default   |
| `Print(value)`                    | Print a value to the terminal                |

## ContentStyle and StyledContent

`ContentStyle` groups foreground color, background color, underline color, and attributes into
one value. `StyledContent<D>` pairs a `ContentStyle` with displayable content.

```rust
use crossterm::style::{ContentStyle, StyledContent, Color, Attribute, Attributes};

let style = ContentStyle {
    foreground_color: Some(Color::Green),
    background_color: None,
    underline_color: None,
    attributes: Attribute::Bold.into(),
};
let styled = StyledContent::new(style, "hello");
execute!(stdout, crossterm::style::PrintStyledContent(styled))?;
```

## `Stylize` Trait

The `Stylize` trait adds fluent builder methods to `&str`, `String`, and other displayable
types, returning `StyledContent`. This is the most ergonomic API for one-off styled output.

```rust
use crossterm::style::Stylize;

let styled = "Error".red().bold();
let styled = "Warning".yellow().on_black().italic();
let styled = "OK".green();
execute!(stdout, crossterm::style::Print(styled))?;
```

Available `Stylize` methods (selected):

```
.red() .green() .yellow() .blue() .magenta() .cyan() .white() .black()
.dark_red() .dark_green() .dark_yellow() .dark_blue() .dark_magenta() .dark_cyan() .grey()
.on_red() .on_green() ... (background variants)
.bold() .dim() .italic() .underlined() .slow_blink() .rapid_blink()
.reverse() .hidden() .crossed_out()
.with(Color) .on(Color) .underline(Color)
.attribute(Attribute)
.reset()
```

## NO_COLOR Compliance

`crossterm` respects the `NO_COLOR` environment variable. When `NO_COLOR` is set to any
non-empty value, color output is suppressed. UMRS TUI code must honor this unconditionally —
check `std::env::var("NO_COLOR").is_ok()` before applying styles if using lower-level APIs,
or rely on crossterm's built-in suppression.

See: https://no-color.org/
