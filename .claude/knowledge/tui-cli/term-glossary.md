# tui-cli Collection — Term Glossary

Generated: 2026-03-15
Ratatui version covered: v0.30.0

Priority: ratatui docs → crossterm docs → CLIG → style guides.

---

## alternate screen

**Definition:** A secondary terminal screen buffer distinct from the main screen. When a program
enters the alternate screen, the main screen content (including scrollback) is preserved. On exit,
the original content is restored. Used by full-screen TUI programs (Vim, htop, ratatui apps).

**Source:** crossterm-terminal.md; ratatui-website/concepts-backends.md

**Commands:** `EnterAlternateScreen` / `LeaveAlternateScreen`

**Canonical spelling:** alternate screen (two words, lowercase)

**Synonyms/deprecated:** alternate buffer, secondary screen

---

## App struct

**Definition:** The central state container for a ratatui application. Holds all business logic,
UI state, and mutable data. Conventionally implements `Widget for &App` for rendering and exposes
methods for state transitions.

**Source:** ratatui-website/concepts-application-architecture.md

**Canonical spelling:** `App` (when referring to the type); "App struct" or "app struct" in prose

---

## Buffer

**Definition:** A two-dimensional array of `Cell` values representing the terminal screen content.
`Terminal` maintains two buffers (current and previous); diffs are computed at frame end and only
changed cells are written to the terminal.

**Source:** ratatui-website/architecture.md; ratatui-website/concepts-backends.md

**Canonical spelling:** `Buffer` (type name); "buffer" in prose

---

## Cassowary

**Definition:** The constraint solver algorithm used by ratatui's `Layout` system. Determines
rectangle sizes from a set of `Constraint` expressions. When constraints cannot all be satisfied
simultaneously, returns a close approximation.

**Source:** ratatui-website/concepts-layout.md; ratatui-website/architecture.md

**Usage:** "ratatui uses the Cassowary constraint solver" — do not call it "Cassowary algorithm"
or "Cassowary solver" inconsistently; pick one form per document.

---

## Cell

**Definition:** A single terminal character position. Contains a symbol (grapheme cluster),
foreground color, background color, and text modifiers. `Buffer` is a collection of `Cell`s.

**Source:** ratatui-website/architecture.md (implicitly via Buffer)

**Canonical spelling:** `Cell` (type name)

---

## Constraint

**Definition:** An expression that determines the size of a layout region. Variants:
- `Length(u16)` — absolute size in rows/columns
- `Percentage(u16)` — relative to parent (0–100)
- `Ratio(u16, u16)` — fractional proportion
- `Min(u16)` — minimum size; grows to absorb excess
- `Max(u16)` — maximum size
- `Fill(u16)` — fills remaining space proportionally (lowest priority)

**Source:** ratatui-website/concepts-layout.md; ratatui-website/architecture.md

**Canonical spelling:** `Constraint` (type name); constraint variants use their variant spelling

**Priority ordering (highest to lowest):** `Min`/`Max` > `Length` > `Percentage` > `Ratio` > `Fill`

---

## CrosstermBackend

**Definition:** The default ratatui backend. Wraps a `Write` implementor and uses the crossterm
library for terminal I/O. Supports Linux, macOS, and Windows.

**Source:** ratatui-website/concepts-backends.md; ratatui-architecture.md

**Canonical spelling:** `CrosstermBackend` (type name, no space)

**Related:** `DefaultTerminal` (type alias that uses `CrosstermBackend<Stdout>`)

---

## DefaultTerminal

**Definition:** A type alias: `Terminal<CrosstermBackend<Stdout>>`. Returned by `ratatui::init()`.
Application code should use this alias rather than spelling out the generic.

**Source:** ratatui-architecture.md; ratatui-website/concepts-backends.md

**Canonical spelling:** `DefaultTerminal`

---

## double buffering

**Definition:** The rendering technique used by ratatui's `Terminal`. Two `Buffer`s are maintained:
current and previous. Widgets draw into the current buffer each frame. At frame end, only the
diff between current and previous is written to the terminal, then the buffers are swapped.

**Source:** ratatui-website/concepts-backends.md; ratatui-website/architecture.md

**Canonical spelling:** double buffering (two words, lowercase)

---

## Event (crossterm)

**Definition:** An input event from the terminal. Variants: `Key(KeyEvent)`, `Mouse(MouseEvent)`,
`Resize(u16, u16)`, `Paste(String)`, `FocusGained`, `FocusLost`. Obtained via `event::read()`.

**Source:** crossterm-event.md

**Canonical spelling:** `Event` (type name)

---

## Flex

**Definition:** Controls how layout elements are positioned when constraints do not perfectly fill
the available area. Loosely based on CSS flexbox.

Variants:
- `Flex::Legacy` — excess space at end (default before v0.26.0)
- `Flex::Start` — items at start, excess at end (current default since v0.26.0)
- `Flex::End` — items at end, excess at start
- `Flex::Center` — items centered
- `Flex::SpaceEvenly` — equal space between items and at edges
- `Flex::SpaceBetween` — space between items only, none at edges
- `Flex::SpaceAround` — space around each item (since v0.30.0 semantics match CSS)

**Source:** ratatui-website/concepts-layout.md; ratatui-breaking-changes.md

**Canonical spelling:** `Flex` (type name); variant names use `Flex::` prefix

**Breaking change note:** `Flex::SpaceAround` semantics changed in v0.30.0. Old `SpaceAround`
behavior is now `Flex::SpaceEvenly`.

---

## FORCE_COLOR

**Definition:** An informal companion to `NO_COLOR`. When set to a non-empty value, instructs
programs to add ANSI color even when they would otherwise not (e.g., stdout is not a TTY). Used
in CI systems that want colored logs.

**Source:** no-color.md

**Canonical spelling:** `FORCE_COLOR` (all caps, as an environment variable name)

**Status:** Informal standard (not as widely supported as `NO_COLOR`)

---

## Frame

**Definition:** The rendering context provided to the closure passed to `terminal.draw()`. Provides
the terminal area, widget rendering methods, and cursor positioning. Not generic over Backend since
v0.24.0.

**Source:** ratatui-architecture.md; ratatui-website/concepts-backends.md

**Canonical spelling:** `Frame` (type name)

**Key methods:** `frame.area()`, `frame.render_widget()`, `frame.render_stateful_widget()`,
`frame.set_cursor_position()`

**Breaking change:** `frame.size()` deprecated in v0.28.0 — use `frame.area()` instead.

---

## HorizontalAlignment

**Definition:** Controls horizontal text alignment within a widget. Renamed from `Alignment` in
v0.30.0. A `type Alignment = HorizontalAlignment` alias is provided for backwards compatibility.

**Source:** ratatui-architecture.md; ratatui-breaking-changes.md

**Canonical spelling:** `HorizontalAlignment` (current); `Alignment` (alias, acceptable in
existing code)

---

## immediate rendering

**Definition:** Ratatui's rendering model. The application must render all visible widgets on
every frame. No widget state is retained between frames by the framework — all state is owned
by the application. Contrasts with retained-mode rendering.

**Source:** ratatui-website/architecture.md; ratatui-website/concepts-application-architecture.md

**Canonical spelling:** immediate rendering (or immediate-mode rendering); not "immediate mode GUI"
(ratatui is TUI, not GUI)

---

## KeyCode

**Definition:** The physical key pressed in a `KeyEvent`. Variants include `Char(char)`, `F(u8)`,
`Backspace`, `Enter`, `Left`, `Right`, `Up`, `Down`, `Home`, `End`, `PageUp`, `PageDown`,
`Tab`, `Esc`, `Delete`, `Insert`, `Media(MediaKeyCode)`, `Modifier(ModifierKeyCode)`.

**Source:** crossterm-event.md

**Canonical spelling:** `KeyCode` (type name)

---

## KeyEvent

**Definition:** A keyboard event from crossterm. Contains `code: KeyCode`, `modifiers: KeyModifiers`,
`kind: KeyEventKind` (Press/Release/Repeat), and `state: KeyEventState`.

**Source:** crossterm-event.md

**Canonical spelling:** `KeyEvent`

**Helper:** `as_key_press_event()` on `Event` — returns `Option<KeyEvent>` for Press events only.

---

## Kitty keyboard protocol

**Definition:** A keyboard enhancement protocol (originating from the Kitty terminal emulator)
that enables access to key release events, repeat events, and keypad disambiguation. Enabled via
`PushKeyboardEnhancementFlags` in crossterm.

**Source:** crossterm-event.md

**Canonical spelling:** Kitty keyboard protocol (or Kitty protocol); not "kitty protocol" (capitalize Kitty)

---

## Layout

**Definition:** A ratatui type that divides a rectangular area into sub-rectangles based on a list
of `Constraint` expressions and a `Flex` mode. Uses the Cassowary constraint solver internally.
Results are cached by default.

**Source:** ratatui-architecture.md; ratatui-website/concepts-layout.md

**Canonical spelling:** `Layout` (type name)

**Key methods:** `Layout::vertical([...])`, `Layout::horizontal([...])`, `.flex(Flex::Center)`,
`.spacing(n)`, `.split(area)`, `area.layout(&layout)` (preferred destructuring form)

---

## Line

**Definition:** A collection of `Span`s on a single terminal line. Part of ratatui's text model:
`Text` → `Line` → `Span`. Supports alignment via `Line::centered()`, `Line::left_aligned()`,
`Line::right_aligned()`.

**Source:** ratatui-website/api-widgets-overview.md; ratatui-breaking-changes.md

**Canonical spelling:** `Line` (type name)

**Replaced:** `Spans` was removed in v0.24.0 and replaced by `Line`.

---

## Modifier (ratatui)

**Definition:** A bitflag set of text rendering attributes applied via `Style`. Values include
`BOLD`, `DIM`, `ITALIC`, `UNDERLINED`, `SLOW_BLINK`, `RAPID_BLINK`, `REVERSED`, `HIDDEN`,
`CROSSED_OUT`.

**Source:** ratatui-website/api-style.md

**Canonical spelling:** `Modifier` (type name); individual flags: `Modifier::BOLD`, etc.

**Note:** This is the ratatui `Modifier`. crossterm has a separate `Attribute` enum; do not
conflate them.

---

## NO_COLOR

**Definition:** An informal standard environment variable. When set to any non-empty value,
instructs programs to suppress all ANSI color output. Widely supported (crossterm, ratatui,
cargo, rustc, and hundreds of others).

**Source:** no-color.md; crossterm-style.md; clig-guidelines.md

**Canonical spelling:** `NO_COLOR` (all caps, as an environment variable name)

**Detection (Rust):** `std::env::var("NO_COLOR").map_or(false, |v| !v.is_empty())`

**Companion:** `FORCE_COLOR` — forces color when stdout is not a TTY.

---

## Paragraph

**Definition:** A ratatui widget that displays optionally styled and wrapped text. Accepts `Text`,
`Line`, or `&str` content. Supports scrolling, wrapping, and alignment.

**Source:** ratatui-website/api-widgets-overview.md

**Canonical spelling:** `Paragraph`

---

## raw mode

**Definition:** A terminal mode that bypasses the driver's normal line-buffered, canonical
processing. In raw mode, input is delivered character-by-character without waiting for Enter, and
special characters (Ctrl+C, backspace) are not processed by the driver.

**Source:** crossterm-terminal.md; ratatui-website/concepts-backends.md

**Canonical spelling:** raw mode (two words, lowercase)

**Commands:** `enable_raw_mode()` / `disable_raw_mode()`

---

## Rect

**Definition:** A rectangular area in ratatui, defined by position (`x`, `y`) and size (`width`,
`height`). All rendering and layout operations use `Rect` for positioning. `Rect::area()` returns
`u32` (changed from `u16` in v0.29.0).

**Source:** ratatui-architecture.md (implicitly); ratatui-breaking-changes.md

**Canonical spelling:** `Rect`

---

## ratatui-core

**Definition:** The foundational crate in the ratatui modular workspace (v0.30.0+). Provides
core traits (`Widget`, `StatefulWidget`), text types (`Span`, `Line`, `Text`), layout types
(`Rect`, `Layout`, `Constraint`), `Buffer`, `Style`, and `Color`. Intended for widget library
authors who want minimal dependencies and maximum stability.

**Source:** ratatui-architecture.md; ratatui-website/architecture.md

**Canonical spelling:** `ratatui-core` (with hyphen, as a crate name)

---

## scrollbar

**Definition:** A UI element indicating position within scrollable content. In ratatui,
implemented by the `Scrollbar` widget with `ScrollbarState`. Rendered as a `StatefulWidget`.

**Source:** ratatui-website/api-widgets-overview.md; ratatui-examples/scrollbar.rs

**Canonical spelling:** scrollbar (one word, lowercase in prose); `Scrollbar` (type name)

**Symbols module:** `ratatui::symbols::scrollbar` (moved from `widgets::scrollbar` in v0.23.0)

---

## Span

**Definition:** A piece of text with an optional `Style`. The smallest unit in ratatui's text
model. `Span::raw("text")` for unstyled; `Span::styled("text", style)` for styled.

**Source:** ratatui-website/api-widgets-overview.md

**Canonical spelling:** `Span`

---

## StatefulWidget

**Definition:** A ratatui trait for widgets that require external mutable state across renders.
Signature: `fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)`. The state
is owned by the application. Examples: `List`/`ListState`, `Table`/`TableState`,
`Scrollbar`/`ScrollbarState`.

**Source:** ratatui-website/concepts-widgets.md

**Canonical spelling:** `StatefulWidget`

---

## Style (ratatui)

**Definition:** Represents incremental changes to a terminal cell's appearance. Contains optional
foreground color, background color, underline color, and `Modifier` flags. Styles are merged
additively — applying S1 then S2 produces the union.

**Source:** ratatui-website/api-style.md

**Canonical spelling:** `Style` (type name)

**Shorthand:** `Stylize` trait provides fluent methods (`.red()`, `.bold()`) on widgets and text.

**Breaking change (v0.30.0):** `Style` no longer implements `Styled`. Methods are now directly
on `Style`. `Style::reset()` replaces the trait method.

---

## Stylize

**Definition:** A ratatui trait providing fluent shorthand methods for styling. Implemented on
`Widget`, `Text`, `Line`, `Span`, `&str`, `String`. Methods: `.red()`, `.on_blue()`, `.bold()`,
`.italic()`, `.reversed()`, etc.

**Source:** ratatui-website/api-style.md; crossterm-style.md (crossterm also has a Stylize)

**Canonical spelling:** `Stylize` (trait name)

**Disambiguation:** Both ratatui and crossterm have a `Stylize` trait. Ratatui's `Stylize` is
for TUI widget styling; crossterm's `Stylize` is for ANSI terminal output. In ratatui TUI code,
use `use ratatui::style::Stylize`.

---

## Table

**Definition:** A ratatui widget displaying data in formatted columns. Supports row, column, and
cell selection via `TableState`. Built with `Table::new(rows, widths)`.

**Source:** ratatui-website/api-widgets-overview.md; ratatui-examples/table.rs

**Canonical spelling:** `Table`

**Note:** `highlight_style` was renamed to `row_highlight_style` in v0.29.0. `Table::new` requires
widths since v0.25.0.

---

## Tailwind palette

**Definition:** A set of CSS Tailwind color constants available in ratatui at
`ratatui::style::palette::tailwind`. Each palette (e.g., `tailwind::BLUE`, `tailwind::SLATE`)
provides shades `c50` through `c950`.

**Source:** ratatui-website/api-style.md; ratatui-examples/table.rs

**Canonical spelling:** Tailwind palette (prose); `tailwind::BLUE.c900` (code)

---

## Terminal (ratatui)

**Definition:** The main entry point for ratatui. Generic over a `Backend`. Manages double
buffering, cursor, and viewport. Provides `terminal.draw(|frame| {...})` as the primary rendering
API.

**Source:** ratatui-website/concepts-backends.md

**Canonical spelling:** `Terminal` (type name)

---

## Text

**Definition:** A collection of `Line`s. Top level of ratatui's text model: `Text` → `Line` →
`Span`. Accepts `&str`, `String`, `Vec<Line>`, and styled variants.

**Source:** ratatui-website/api-widgets-overview.md

**Canonical spelling:** `Text`

---

## tick rate

**Definition:** The interval at which the application checks for new events and redraws. Commonly
expressed as a `Duration` (e.g., `Duration::from_millis(50)` for ~20 fps). Used as the timeout
for `event::poll()`.

**Source:** ratatui-website/concepts-application-architecture.md; ratatui-examples/scrollbar.rs

**Canonical spelling:** tick rate (two words, lowercase)

---

## Widget (ratatui)

**Definition:** A ratatui trait for types that can render themselves into a `Buffer`. Consuming:
`fn render(self, area: Rect, buf: &mut Buffer)`. Implement on `&YourType` to avoid consuming the
value on each frame.

**Source:** ratatui-website/concepts-widgets.md; ratatui-website/architecture.md

**Canonical spelling:** `Widget` (trait name)

**Recommended pattern (v0.30.0+):** `impl Widget for &YourType` — not `impl WidgetRef for YourType`.
