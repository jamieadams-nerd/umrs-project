# Style, Color, and Modifier API

<!-- source: https://docs.rs/ratatui/latest/ratatui/style/struct.Style.html -->
<!-- source: https://ratatui.rs/examples/style/modifiers/ -->
<!-- fetched: 2026-03-15 (via WebSearch) -->

## The `style` Module

The `style` module provides types that represent various styling options. The most important
type is `Style`, which represents the foreground and background colors and the text attributes
of a `Span`.

## Style

`Style` represents incremental changes to a cell's appearance. Applying styles S1, S2, S3 to
a cell produces the merge of all three — not just S3.

### Construction

```rust
use ratatui::style::{Color, Modifier, Style};

// Explicit builder
let style = Style::default()
    .fg(Color::Black)
    .bg(Color::Green)
    .add_modifier(Modifier::ITALIC | Modifier::BOLD);

// Shorthand via Stylize trait
let style = Style::new().black().on_green().italic().bold();
```

### Resetting Styles

```rust
// Reset all styling back to terminal default
let style = Style::reset();
```

## Color

```rust
pub enum Color {
    Reset,
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray,
    DarkGray, LightRed, LightGreen, LightYellow, LightBlue,
    LightMagenta, LightCyan, White,
    Rgb(u8, u8, u8),          // 24-bit true color
    Indexed(u8),               // 256-color palette
}
```

Ratatui includes the Tailwind palette at `ratatui::style::palette::tailwind`:

```rust
use ratatui::style::palette::tailwind;

let header_bg = tailwind::BLUE.c900;
let header_fg = tailwind::SLATE.c200;
```

## Modifier

`Modifier` is a bitflag set of text attributes:

```rust
pub struct Modifier: u16 {
    const BOLD          = 0b0000_0000_0001;
    const DIM           = 0b0000_0000_0010;
    const ITALIC        = 0b0000_0000_0100;
    const UNDERLINED    = 0b0000_0000_1000;
    const SLOW_BLINK    = 0b0000_0001_0000;
    const RAPID_BLINK   = 0b0000_0010_0000;
    const REVERSED      = 0b0000_0100_0000;
    const HIDDEN        = 0b0000_1000_0000;
    const CROSSED_OUT   = 0b0001_0000_0000;
}
```

Combine modifiers with `|`:

```rust
let bold_italic = Modifier::BOLD | Modifier::ITALIC;
```

## Stylize Trait

`Stylize` provides shorthand methods for all style operations on widgets and text types.
It is implemented for `Widget`, `Text`, `Line`, `Span`, and more.

```rust
use ratatui::style::Stylize;

// On widgets:
Paragraph::new("hello").bold().on_blue().italic()

// On text:
Line::from("error").red().bold()
Span::raw("warning").yellow()

// Color shortcuts:
widget.black()     .on_black()
widget.red()       .on_red()
widget.green()     .on_green()
widget.yellow()    .on_yellow()
widget.blue()      .on_blue()
widget.magenta()   .on_magenta()
widget.cyan()      .on_cyan()
widget.white()     .on_white()
widget.gray()      .on_gray()
widget.dark_gray() .on_dark_gray()

// Modifier shortcuts:
widget.bold()
widget.dim()
widget.italic()
widget.underlined()
widget.slow_blink()
widget.rapid_blink()
widget.reversed()
widget.hidden()
widget.crossed_out()
widget.reset()
```

## Underline Color

The underline color uses a non-standard ANSI escape sequence. It is supported by most terminal
emulators but is only implemented in the crossterm backend. Enable with the `underline-color`
feature flag:

```toml
ratatui = { version = "0.30", features = ["underline-color"] }
```

```rust
let style = Style::default()
    .underline_color(Color::Green)
    .add_modifier(Modifier::UNDERLINED);
```

## Style Application in Practice

```rust
// Row colors in a table
let normal_row = Style::new().fg(tailwind::SLATE.c200).bg(tailwind::SLATE.c950);
let alt_row    = Style::new().fg(tailwind::SLATE.c200).bg(tailwind::SLATE.c900);
let selected   = Style::default()
    .add_modifier(Modifier::REVERSED)
    .fg(tailwind::BLUE.c400);

// Key bindings display
let key_style  = Style::new().fg(Color::White).bg(Color::DarkGray);
let desc_style = Style::new().fg(Color::Gray);
```

## Color Conversions

Ratatui implements `Into<Style>` for `Color` and `Modifier`, allowing direct use where
`Into<Style>` is accepted:

```rust
// These are equivalent:
Line::styled("hello", Color::Red)
Line::styled("hello", Style::new().fg(Color::Red))

// And:
Line::styled("bold", Modifier::BOLD)
Line::styled("bold", Style::new().add_modifier(Modifier::BOLD))
```
