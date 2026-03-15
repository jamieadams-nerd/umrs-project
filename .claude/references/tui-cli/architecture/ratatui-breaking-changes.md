# Ratatui — Breaking Changes

Source: https://raw.githubusercontent.com/ratatui/ratatui/main/BREAKING-CHANGES.md
Retrieved: 2026-03-15

This document contains breaking changes per version and migration notes. Compiled from the
commit history and changelog; PRs are tagged with the [breaking change] label on GitHub.

---

## Summary

- [v0.31.0](#v0310) — MSRV 1.88.0
- [v0.30.1](#v0301) — `AsRef` impls for widgets may affect type inference
- [v0.30.0](#v0300) — Major: workspace modularization, `block::Title` removed, `Alignment`
  renamed, `Backend` associated Error type, `Flex::SpaceAround` semantics, MSRV 1.86.0
- [v0.29.0](#v0290) — `Sparkline::data` API, `Color::from_hsl` gated, `Tabs::select`
- [v0.28.0](#v0280) — `Backend::size` returns `Size`, crossterm 0.28, `Frame::size` → `area`
- [v0.27.0](#v0270) — List index clamping, prelude changes, `Rect::inner` by value
- [v0.26.0](#v0260) — `Flex::Start` default, `Table::new` and `Tabs::new` accept iterators
- [v0.25.0](#v0250) — `Table::new` requires widths, `List::new` iterator
- [v0.24.0](#v0240) — `Frame` no longer generic over Backend, `Spans` removed
- [v0.23.0](#v0230) — Scrollbar symbol moves, MSRV 1.67.0
- [v0.22.0](#v0220) — `bitflags` 2.3 serde representation change
- [v0.21.0](#v0210) — MSRV 1.65.0, `ViewPort` enum
- [v0.20.0](#v0200) — First Ratatui release (fork of tui-rs)

---

## v0.31.0

### MSRV is now 1.88.0

---

## v0.30.1

### `AsRef` impls for widgets may affect type inference

Adding `AsRef<Self>` for built-in widgets can change type inference in rare cases where `AsRef`
is part of a trait bound. Add explicit type annotations or specify the concrete widget type.
Remove any redundant `AsRef` impls.

---

## v0.30.0

### `Marker` is now `#[non_exhaustive]`

Add a wildcard arm to exhaustive `match` on `Marker`:
```diff
  match marker {
      Marker::Dot => { /* ... */ }
+     _ => { /* ... */ }
  }
```

### `Flex::SpaceAround` semantics changed

Now mirrors flexbox: space between items is twice the outer gap. Old behavior is now
`Flex::SpaceEvenly`:
```diff
- .flex(Flex::SpaceAround)
+ .flex(Flex::SpaceEvenly)
```

### `block::Title` removed

```diff
- use ratatui::widgets::{Block, block::{Title, Position}};
+ use ratatui::widgets::{Block, TitlePosition};

- .title(Title::from("Hello"))
+ .title(Line::from("Hello"))

- .title(Title::from("Hello").position(Position::Bottom).alignment(Alignment::Center))
+ .title_bottom(Line::from("Hello").centered())

- use ratatui::widgets::block::BlockExt;
+ use ratatui::widgets::BlockExt;
```

### `Style` no longer implements `Styled`

Methods are now defined directly on `Style`. Remove `Stylize` import if no longer needed:
```diff
- use ratatui::style::Stylize;
  let style = Style::new().red();
```

`Style::reset()` replaces the trait method.

### `Backend` now requires associated `Error` type and `clear_region` method

Custom `Backend` implementations must define `type Error` and implement `clear_region`. Update
function signatures:
```diff
- fn run<B: Backend>(terminal: Terminal<B>) -> io::Result<()>
+ fn run<B: Backend<Error = io::Error>>(terminal: Terminal<B>) -> io::Result<()>
// or use the concrete alias:
+ fn run(terminal: DefaultTerminal) -> io::Result<()>
```

### `TestBackend` uses `Infallible` for errors

Test cases using `TestBackend` may need updates.

### `layout::Alignment` renamed to `layout::HorizontalAlignment`

Type alias available for backwards compatibility; update imports when convenient:
```diff
- use ratatui::layout::Alignment;
+ use ratatui::layout::HorizontalAlignment;
```

### `List::highlight_symbol` accepts `Into<Line>`

Previously `&str`. Const context callers must adjust.

### `FrameExt` trait required for `unstable-widget-ref` feature

Import `ratatui::widgets::FrameExt` and enable `unstable-widget-ref` feature to use
`Frame::render_widget_ref()`.

### `WidgetRef` blanket impl reversed

```diff
- impl WidgetRef for Foo {
-     fn render_ref(&self, area: Rect, buf: &mut Buffer)
+ impl Widget for &Foo {
+     fn render(self, area: Rect, buf: &mut Buffer)
  }
```

### `From` impls for backend types replaced with specific traits

```diff
+ use ratatui::backend::crossterm::{FromCrossterm, IntoCrossterm};
- let ratatui_color = ratatui::style::Color::from(crossterm_color);
+ let ratatui_color = ratatui::style::Color::from_crossterm(crossterm_color);
- let crossterm_color = ratatui_color.into();
+ let crossterm_color = ratatui_color.into_crossterm();
```

### `layout-cache` feature now required for cache APIs

`Layout::init_cache` and `Layout::DEFAULT_CACHE_SIZE` are gated behind `layout-cache`
(enabled by default). Explicitly re-enable if you disable `default-features`.

### MSRV is now 1.86.0

---

## v0.29.0

### `Sparkline::data` takes `IntoIterator<Item = SparklineBar>` instead of `&[u64]`

No longer `const fn`. Provide explicit types for single-value slices:
```diff
- Sparkline::default().data(&[value.into()]);
+ Sparkline::default().data(&[u64::from(value)]);
```

### `Color::from_hsl` now behind `palette` feature, accepts `palette::Hsl`

```diff
- Color::from_hsl(360.0, 100.0, 100.0)
+ Color::from_hsl(Hsl::new(360.0, 100.0, 100.0))
```

### `Rect::area()` returns `u32` instead of `u16`

### `Line` implements `From<Cow<str>>`

May cause ambiguous inferred type compilation errors:
```diff
  let foo = Foo { ... }; // implements both From<String> and From<Cow<str>>
- let line = Line::from(foo);
+ let line = Line::from(String::from(foo));
```

### `Tabs::select()` accepts `Into<Option<usize>>`

```diff
  let selected = 1u8;
- let tabs = Tabs::new(["A", "B"]).select(selected.into())
+ let tabs = Tabs::new(["A", "B"]).select(selected as usize)
```

### `Table::highlight_style` renamed to `Table::row_highlight_style`

---

## v0.28.0

### `Backend::size` returns `Size` instead of `Rect`

### `Backend` migrates to `get/set_cursor_position`

### Ratatui now requires Crossterm 0.28.0

Re-exported under `ratatui::crossterm` to avoid version conflicts.

### `Axis::labels()` accepts `IntoIterator<Into<Line>>`

```diff
- Axis::default().labels(vec!["a".into(), "b".into()])
+ Axis::default().labels(["a", "b"])
```

### `Layout::init_cache` takes `NonZeroUsize`

```diff
- Layout::init_cache(100);
+ Layout::init_cache(NonZeroUsize::new(100).unwrap());
```

### `ratatui::terminal` module is now private

```diff
- use ratatui::terminal::{Frame, Terminal, TerminalOptions, ViewPort};
+ use ratatui::{Frame, Terminal, TerminalOptions, ViewPort};
```

### `Frame::size` deprecated, renamed to `Frame::area`

---

## v0.27.0

### List clamps selected index

`first`, `last`, `previous`, `next`, and `select` now clamp to list bounds.

### Prelude changes

Removed: `style::Styled`, `symbols::Marker`, `terminal::{CompletedFrame, TerminalOptions, Viewport}`
Added: `layout::{Position, Size}`

### `Rect::inner` takes `Margin` by value

```diff
- area.inner(&Margin { vertical: 0, horizontal: 2 })
+ area.inner(Margin { vertical: 0, horizontal: 2 })
```

### `Buffer::filled` takes `Cell` by value

```diff
- Buffer::filled(area, &Cell::new("X"))
+ Buffer::filled(area, Cell::new("X"))
```

### `List::start_corner` / `layout::Corner` removed

```diff
- list.start_corner(Corner::TopLeft)
+ list.direction(ListDirection::TopToBottom)
```

---

## v0.26.0

### `Flex::Start` is the new default flex mode

Old stretch-to-fill behavior available via `Flex::Legacy`:
```diff
- Layout::horizontal([Length(1), Length(2)]).split(area)
+ Layout::horizontal([Length(1), Length(2)]).flex(Flex::Legacy).split(area)
```

### `Table::new()` accepts `IntoIterator<Item: Into<Row>>`

Empty containers need explicit types:
```diff
- Table::new(vec![], widths)
+ Table::default().widths(widths)
```

### `patch_style` / `reset_style` now consume `Self`

```diff
- let mut line = Line::from("foobar"); line.patch_style(style);
+ let line = Line::from("foobar").patch_style(style);
```

### `Line` has a new `style` field

Struct initializers must add `..Default::default()` or use a constructor.

---

## v0.25.0

### `Table::new()` requires widths

```diff
- Table::new(rows).widths(widths)
+ Table::new(rows, widths)
```

### `Table::widths()` accepts `IntoIterator<Item = AsRef<Constraint>>`

```diff
- Table::new(rows).widths(&[Constraint::Length(1)])
+ Table::new(rows, [Constraint::Length(1)])
```

### `List::new()` accepts `IntoIterator<Item = Into<ListItem>>`

```diff
- List::new(vec![])
+ List::default()
```

---

## v0.24.0

### `Frame` is no longer generic over `Backend`

```diff
- fn ui<B: Backend>(frame: &mut Frame<B>) { ... }
+ fn ui(frame: &mut Frame) { ... }
```

### `ScrollbarState` fields changed from `u16` to `usize`

### `Spans` removed (replaced with `Line` since v0.21.0)

```diff
- Buffer::set_spans(0, 0, spans, 10)
+ Buffer::set_line(0, 0, line, 10)
```

---

## v0.23.0

### `Scrollbar::track_symbol` takes `Option<&str>`

```diff
- Scrollbar::default().track_symbol("|")
+ Scrollbar::default().track_symbol(Some("|"))
```

### Scrollbar symbols moved to `symbols::scrollbar`

```diff
- use ratatui::widgets::scrollbar::Set
+ use ratatui::symbols::scrollbar::Set
```

### MSRV 1.67.0

---

## v0.22.0

### `bitflags` updated to 2.3 — serde representation changed

Any existing serialized `Borders` or `Modifiers` values must be re-serialized.

---

## v0.21.0

### MSRV 1.65.0

### `ViewPort` changed from struct to enum

```diff
- viewport: Viewport::fixed(area)
+ viewport: Viewport::Fixed(area)
```

---

## v0.20.0

First release of Ratatui (forked from tui-rs). MSRV 1.63.0.

### `List` no longer ignores empty strings

Items with empty strings now render as blank lines.
