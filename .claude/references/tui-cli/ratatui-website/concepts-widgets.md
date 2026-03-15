# Introduction to Widgets

<!-- source: https://ratatui.rs/concepts/widgets/ -->
<!-- source: https://docs.rs/ratatui/latest/ratatui/widgets/index.html -->
<!-- fetched: 2026-03-15 (via WebSearch) -->

Widgets are the building blocks of user interfaces in Ratatui. They are used to create and
manage the layout and style of the terminal interface. Widgets can be combined and nested to
create complex UIs and can be easily customized.

## Built-in Widgets

All available in `ratatui::widgets`:

| Widget | Purpose |
|---|---|
| `Block` | Borders, titles, and padding; wraps other widgets |
| `Paragraph` | Displays optionally styled and wrapped text |
| `List` | Scrollable list of items with selection support |
| `Table` | Multi-column grid with row/column/cell selection |
| `Gauge` | Progress percentage using block characters |
| `Scrollbar` | Scrollbar indicator |
| `Sparkline` | Single dataset as a sparkline chart |
| `BarChart` | Bar chart |
| `Chart` | Line and scatter charts |
| `Tabs` | Tab bar with selection |
| `Canvas` | Free-form drawing with shapes, lines, maps |
| `Calendar` | Calendar view |

Primitive text types also implement `Widget`: `String`, `&str`, `Span`, `Line`, `Text`.
Prefer `Paragraph` for complex text rendering.

## The Widget Traits

### `Widget` — Consuming

```rust
pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}
```

The widget is consumed on render. Suitable for stateless widgets created fresh each frame.

### `StatefulWidget` — With External State

```rust
pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

Used for widgets that need to track state across renders, such as scroll position or selection.
The state is owned by the application, not the widget. Built-in examples: `List`/`ListState`,
`Table`/`TableState`, `Scrollbar`/`ScrollbarState`.

### `WidgetRef` — Shared Reference (Recommended)

Implement `Widget` on `&YourWidget` rather than `YourWidget`. This is the recommended pattern:

```rust
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // render using &self — does not consume App
    }
}
```

This allows the widget to be rendered without being consumed, enabling reuse across frames and
avoiding unnecessary clones.

## Rendering Widgets

Drawing the UI is done from the closure passed to `Terminal::draw`:

```rust
terminal.draw(|frame| {
    frame.render_widget(widget, area);
    // or:
    frame.render_stateful_widget(widget, area, &mut state);
})?;
```

A common compositional pattern: have a single root widget (`App`) that is passed to
`frame.render_widget()`. Within its `render` method, it calls `render` directly on child
widgets, passing sub-areas from a layout.

## State Management

### Widget-Owned State
State is embedded in the widget. Simple but couples the widget to its own history.

### External State (Recommended for interactive widgets)
State is owned by the application and passed as `&mut State` to `render_stateful_widget`.
This is what built-in widgets like `List` and `Table` use.

```rust
struct App {
    list_state: ListState,
    items: Vec<String>,
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list = List::new(self.items.clone())
            .highlight_symbol(">> ");
        StatefulWidget::render(list, area, buf, &mut self.list_state);
    }
}
```

## Block — The Container Widget

`Block` creates visual containers by drawing borders around an area. Most built-in widgets
accept an optional `Block` via a `.block()` builder method.

```rust
let paragraph = Paragraph::new("Hello, world!")
    .block(Block::bordered().title("My Paragraph"));
```

When a widget renders with a block:
1. The widget's style is applied first.
2. The block's style is applied.
3. The widget's content is rendered within the inner area.

`Block::inner(area)` computes the inner area after accounting for borders, titles, and padding.

## Custom Widgets

Implement `Widget` on your type:

```rust
struct Button {
    label: String,
    style: Style,
}

impl Widget for &Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().style(self.style);
        let inner = block.inner(area);
        block.render(area, buf);
        Line::from(self.label.as_str())
            .centered()
            .render(inner, buf);
    }
}
```

## Styling Widgets

All widgets accept `Style` via the `Stylize` trait shorthand or explicit `Style`:

```rust
// Explicit
Paragraph::new("text").style(Style::default().fg(Color::Red).bg(Color::Blue));

// Stylize shorthand
Paragraph::new("text").red().on_blue().bold();
```

Styles are incremental: applying S1, S2, S3 produces a merge of all three.
