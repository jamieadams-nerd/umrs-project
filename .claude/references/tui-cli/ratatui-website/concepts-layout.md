# Layout Concepts

<!-- source: https://ratatui.rs/concepts/layout/ -->
<!-- fetched: 2026-03-15 (via WebSearch) -->

Layouts and widgets form the basis of the UI in Ratatui. Layouts dictate the structure of the
interface, dividing the screen into various sections using constraints, while widgets fill these
sections with content.

## Constraint-Based Layout

Ratatui uses the **Cassowary** constraint solver to determine rectangle sizes. When not all
constraints can be simultaneously satisfied, the solver returns an approximation.

You can specify a rectangle as an absolute position and size, or use the `Layout` struct to
divide the terminal window dynamically based on constraints.

### Constraint Types

| Constraint | Behavior |
|---|---|
| `Length(u16)` | Absolute size in rows or columns; not responsive to terminal size |
| `Percentage(u16)` | Relative to parent — `Percentage(50)` takes half of parent |
| `Ratio(u16, u16)` | Fine-grained proportional split |
| `Min(u16)` | Minimum size; grows to absorb excess space in all Flex modes except Legacy |
| `Max(u16)` | Maximum size |
| `Fill(u16)` | Fills excess space proportionally; lower priority than spacers |

### The Fill Constraint

`Fill(u16)` grows to allocate excess space, scaling proportionally with other `Fill` variants.
`Fill(1)` and `Fill(2)` side by side: the second takes twice the remaining space.

`Fill(0)` collapses when other non-zero `Fill` constraints are present.

## Flex Layouts

`Flex` determines how elements are positioned when constraints do not perfectly fill the
available area. The API is loosely based on CSS flexbox.

```rust
Layout::horizontal([Constraint::Length(10), Constraint::Fill(1)])
    .flex(Flex::Center)
    .split(area)
```

| Flex Variant | Behavior |
|---|---|
| `Flex::Legacy` | Excess space at end (default behavior before v0.26) |
| `Flex::Start` | Items at start, excess at end |
| `Flex::End` | Items at end, excess at start |
| `Flex::Center` | Items centered, excess split equally at both ends |
| `Flex::SpaceEvenly` | Equal space between items and at edges |
| `Flex::SpaceBetween` | Space only between items, none at edges |
| `Flex::SpaceAround` | Space around each item |

## Negative Spacing

`Layout::spacing(-1)` allows overlapping segments. Zero or positive spacing works as before.
Negative spacing causes all segments to be adjacent with overlapping pixels.

## Nested Layouts

Layouts can be nested. An outer layout produces rectangles; pass one of those rectangles
to an inner `Layout` for subdivided regions. This enables complex, responsive UI designs.

```rust
fn render(frame: &mut Frame) {
    let outer = Layout::vertical([
        Constraint::Length(3),  // header
        Constraint::Min(0),     // body
        Constraint::Length(1),  // footer
    ]);
    let [header, body, footer] = frame.area().layout(&outer);

    // Subdivide the body horizontally
    let inner = Layout::horizontal([
        Constraint::Percentage(30),  // sidebar
        Constraint::Fill(1),         // main content
    ]);
    let [sidebar, content] = body.layout(&inner);
}
```

## Practical Centering with Flex

A common recipe for centering popups or content areas:

```rust
fn centered_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = area.layout(&vertical);
    let [area] = area.layout(&horizontal);
    area
}
```

## Destructuring Layout Results

The `layout()` method on `Rect` returns an array. Use destructuring for clarity:

```rust
let layout = Layout::vertical([
    Constraint::Length(1),
    Constraint::Min(0),
    Constraint::Length(1),
]);
let [title_bar, content, footer] = area.layout(&layout);
```

## Interactive Tools

- **Constraint Explorer** (`ratatui.rs/examples/layout/constraint-explorer/`): Visualizes
  constraint interactions live.
- **Flex example** (`ratatui.rs/examples/layout/flex/`): Demonstrates all Flex variants.
- **Constraints example** (`ratatui.rs/examples/layout/constraints/`): Shows how constraint
  types interact.
