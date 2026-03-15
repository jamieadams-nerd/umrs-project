# Widgets API Overview

<!-- source: https://docs.rs/ratatui/latest/ratatui/widgets/index.html -->
<!-- source: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html -->
<!-- source: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html -->
<!-- source: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html -->
<!-- fetched: 2026-03-15 (via WebSearch) -->

## Block

`Block` is the foundational container widget. It renders borders, titles, and padding around
other widgets and around arbitrary content areas.

```rust
// Border styles
Block::new()                    // no borders
Block::bordered()               // all four borders
Block::new().borders(Borders::TOP | Borders::BOTTOM)

// Titles
Block::bordered().title("Title")
Block::bordered()
    .title(Title::from("Left Title"))
    .title(Title::from("Right Title").alignment(Alignment::Right))

// Border style
Block::bordered()
    .border_type(BorderType::Rounded)   // rounded corners
    .border_style(Style::new().fg(Color::Blue))

// Padding
Block::bordered().padding(Padding::uniform(1))

// Inner area computation (use this to position child content)
let inner = block.inner(area); // Rect excluding borders/padding
```

### Border Types

```rust
pub enum BorderType {
    Plain,      // default — straight lines
    Rounded,    // rounded corners
    Double,     // double lines
    Thick,      // thick lines
    QuadrantInside,
    QuadrantOutside,
}
```

## Paragraph

`Paragraph` displays optionally styled and wrapped text.

```rust
Paragraph::new("Simple text")
Paragraph::new(Text::from("Styled text"))
Paragraph::new(Line::from(vec![
    Span::styled("bold ", Style::new().bold()),
    Span::raw("normal"),
]))
.block(Block::bordered().title("My Paragraph"))
.alignment(Alignment::Center)
.wrap(Wrap { trim: true })
.scroll((scroll_offset, 0))
```

For scrollable paragraphs, use `.scroll((y_offset, x_offset))` and track the offset in state.

## List

`List` displays a list of items and supports selection.

```rust
let items = vec!["Item 1", "Item 2", "Item 3"];
let list = List::new(items)
    .block(Block::bordered().title("List"))
    .highlight_style(Style::new().reversed())
    .highlight_symbol(">> ");

// Stateful rendering
let mut state = ListState::default().with_selected(Some(0));
frame.render_stateful_widget(list, area, &mut state);
```

`ListState` methods:
- `state.select(Some(i))` — select by index
- `state.selected()` — get current selection
- `state.scroll_offset()` — get scroll position

## Table

`Table` displays data in formatted columns with selection support.

```rust
let header = Row::new(["Name", "Age", "City"]).style(header_style).height(1);
let rows = data.iter().map(|d| {
    Row::new([d.name.clone(), d.age.to_string(), d.city.clone()])
        .height(1)
});
let widths = [
    Constraint::Length(20),
    Constraint::Length(5),
    Constraint::Min(10),
];
let table = Table::new(rows, widths)
    .header(header)
    .block(Block::bordered())
    .row_highlight_style(Style::new().reversed())
    .column_highlight_style(Style::new().fg(Color::Blue))
    .cell_highlight_style(Style::new().bg(Color::DarkGray))
    .highlight_spacing(HighlightSpacing::Always);

let mut state = TableState::default().with_selected(0);
frame.render_stateful_widget(table, area, &mut state);
```

`TableState` supports:
- `state.select(Some(row))` — select row
- `state.select_next_column()` / `state.select_previous_column()` — column selection
- Per-cell selection when both row and column are selected

## Gauge

```rust
Gauge::default()
    .block(Block::bordered().title("Progress"))
    .gauge_style(Style::new().blue().on_black())
    .percent(42)
    .label("42%")
```

## Sparkline

```rust
Sparkline::default()
    .block(Block::bordered().title("Data"))
    .data(&[1, 2, 3, 4, 5, 4, 3, 2, 1])
    .style(Style::new().fg(Color::Green))
    .absent_value_style(Style::new().fg(Color::DarkGray))  // v0.29+
    .absent_value_symbol(symbols::bar::HALF)                // v0.29+
```

## Scrollbar

```rust
// Horizontal or vertical scrollbar
let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
    .begin_symbol(Some("↑"))
    .end_symbol(Some("↓"))
    .thumb_symbol("█");

let mut scroll_state = ScrollbarState::new(content_length)
    .position(current_position);

frame.render_stateful_widget(
    scrollbar,
    area.inner(Margin { vertical: 1, horizontal: 0 }),
    &mut scroll_state,
);
```

## Tabs

```rust
let titles = ["Tab 1", "Tab 2", "Tab 3"];
let tabs = Tabs::new(titles)
    .block(Block::bordered().title("Tabs"))
    .select(selected_index)
    .highlight_style(Style::new().reversed())
    .divider("|")
    .padding(" ", " ");
```

## Clear

`Clear` clears the cells in a given area before rendering another widget on top. Use for
popups and overlays:

```rust
frame.render_widget(Clear, popup_area);
frame.render_widget(popup_block, popup_area);
```

## Text, Line, and Span

Ratatui's text model:

```
Text  — a collection of Lines
Line  — a collection of Spans on a single line
Span  — a piece of text with an optional Style
```

```rust
// Building styled text
let text = Text::from(vec![
    Line::from(vec![
        Span::styled("Error: ", Style::new().red().bold()),
        Span::raw("file not found"),
    ]),
    Line::from("Second line"),
]);

// Convenience
Text::from("plain text")
Text::styled("styled", Style::new().bold())
Line::from("line")
Line::styled("styled line", Style::new().italic())
Span::raw("span")
Span::styled("styled span", Style::new().fg(Color::Green))
```

`Line::centered()`, `Line::left_aligned()`, `Line::right_aligned()` set alignment inline.
