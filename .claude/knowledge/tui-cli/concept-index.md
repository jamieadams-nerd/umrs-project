# tui-cli Collection — Concept Index

Generated: 2026-03-15
Ratatui version covered: v0.30.0

---

## ratatui-architecture.md (ARCH-1)

**What it covers:** Describes ratatui's modular Cargo workspace introduced in v0.30.0. Explains
the role of each crate (`ratatui-core`, `ratatui-widgets`, `ratatui-crossterm`, etc.) and who
should depend on what. Covers the `DefaultTerminal` type alias, `init()`/`restore()` helpers,
the `Frame`/`draw()` API, and `Layout` basics with cache behavior.

**Key terms and concepts:**
- `ratatui-core` — foundational types and traits; for widget library authors
- `ratatui` — umbrella crate; for application authors; re-exports everything
- `DefaultTerminal` — type alias for `Terminal<CrosstermBackend<Stdout>>`
- `ratatui::init()` — enables raw mode, alternate screen, installs panic hook
- `ratatui::restore()` — paired cleanup; leaves alternate screen, disables raw mode
- `Frame` — rendering context passed to `terminal.draw(|frame| {...})` closure
- `Layout::init_cache(NonZeroUsize)` — tunes LRU layout cache size
- MSRV 1.86.0 for v0.30.0; 1.88.0 for v0.31.0

**Governs:** Entry-point boilerplate, crate dependency selection, terminal setup/teardown.

**Related:** ratatui-website/architecture.md (overlapping content, more detail on rendering model),
ratatui-website/concepts-backends.md (backend setup detail).

---

## ratatui-breaking-changes.md (ARCH-2)

**What it covers:** Full breaking change log from v0.20.0 (first Ratatui release) through v0.31.0.
Provides diff examples for each migration. Critical for understanding API evolution and avoiding
use of removed or renamed items.

**Key terms and concepts:**
- `block::Title` removed in v0.30.0 — use `Line::from(...)` and `.title_bottom()` directly
- `Alignment` → `HorizontalAlignment` (alias available, v0.30.0)
- `Frame` no longer generic over Backend (v0.24.0) — `fn ui(frame: &mut Frame)` is correct
- `Flex::SpaceAround` semantics changed in v0.30.0 — old behavior is now `Flex::SpaceEvenly`
- `Flex::Start` became default in v0.26.0 — old stretch behavior via `Flex::Legacy`
- `Table::new(rows, widths)` — widths required since v0.25.0
- `Table::highlight_style` → `Table::row_highlight_style` (v0.29.0)
- `Frame::size` → `Frame::area` (v0.28.0)
- `WidgetRef` blanket impl reversed in v0.30.0 — implement `Widget for &Foo`, not `WidgetRef for Foo`
- `Scrollbar` symbols moved to `symbols::scrollbar` (v0.23.0)
- `ratatui::terminal` module now private — import `Frame`, `Terminal` from root (v0.28.0)
- `Rect::area()` returns `u32` not `u16` (v0.29.0)
- `AsRef<Self>` added to widgets in v0.30.1 — may affect type inference

**Governs:** Migration decisions, avoiding deprecated/removed APIs in new code.

**Related:** ARCH-1 (architecture overview), all widget docs.

---

## crossterm-overview.md (CT-1)

**What it covers:** Overview of the `crossterm` crate — a pure-Rust cross-platform terminal
manipulation library. Covers the Command API pattern, `execute!` and `queue!` macros, feature
flags, and module summary.

**Key terms and concepts:**
- `Command` trait — abstraction over platform-specific terminal operations
- `execute!(writer, cmd1, cmd2)` — immediate execution with flush
- `queue!(writer, cmd1, cmd2)` + `writer.flush()` — deferred execution for batching
- Feature flags: `events` (default), `event-stream` (async), `bracketed-paste`, `serde`
- Modules: `event`, `terminal`, `style`, `cursor`, `clipboard`
- `derive_more` feature — adds `is_*` helper functions on event types

**Governs:** Terminal I/O primitives, command dispatch pattern.

**Related:** CT-2 (events), CT-3 (terminal control), CT-4 (styling).

---

## crossterm-event.md (CT-2)

**What it covers:** The `crossterm::event` module for reading keyboard, mouse, focus, and resize
events. Covers `poll()`/`read()` functions, the `Event` enum, `KeyEvent`, `KeyCode`,
`MouseEvent`, and the Kitty keyboard enhancement protocol.

**Key terms and concepts:**
- `event::poll(Duration)` — non-blocking check; returns `true` if event ready
- `event::read()` — blocking read; guaranteed non-blocking after `poll()` returned `true`
- `Event::Key(KeyEvent)`, `Event::Mouse(MouseEvent)`, `Event::Resize(u16, u16)`, `Event::Paste`, `Event::FocusGained/Lost`
- `as_key_press_event()` — convenience; returns `Option<KeyEvent>` for Press events only
- `KeyEvent { code, modifiers, kind, state }` — full key event structure
- `KeyCode::Char(char)`, `KeyCode::F(u8)`, arrow/function/control variants
- `KeyModifiers` — `NONE`, `SHIFT`, `CONTROL`, `ALT`, `SUPER`
- `KeyEventKind` — `Press`, `Release`, `Repeat` (last two require Kitty enhancement)
- Kitty enhancement: `PushKeyboardEnhancementFlags` / `PopKeyboardEnhancementFlags`
- Mouse events require `EnableMouseCapture` command; not enabled by default
- Ctrl+C (0x03) → `KeyCode::Char('c')` + `KeyModifiers::CONTROL`

**Governs:** All event handling code, key binding implementations.

**Related:** CT-1 (overview), ratatui-website/concepts-application-architecture.md (event loop patterns).

---

## crossterm-terminal.md (CT-3)

**What it covers:** The `crossterm::terminal` module. Covers raw mode enable/disable, terminal
size queries, the `Clear` command with `ClearType` variants, and the alternate screen
(`EnterAlternateScreen` / `LeaveAlternateScreen`). Also covers `BeginSynchronizedUpdate`.

**Key terms and concepts:**
- Raw mode — character-by-character input, no driver processing of special keys
- `enable_raw_mode()` / `disable_raw_mode()` — must be paired; use Drop guard for safety
- `size()` — returns `(columns, rows)` as `(u16, u16)`
- `Clear(ClearType)` — `All`, `Purge`, `CurrentLine`, `UntilNewLine`, etc.
- Alternate screen — separate buffer, no scrollback; shell content preserved on exit
- `EnterAlternateScreen` / `LeaveAlternateScreen`
- `BeginSynchronizedUpdate` / `EndSynchronizedUpdate` — suppresses intermediate redraws
- `ratatui::init()` handles raw mode + alternate screen + panic hook automatically

**Governs:** Terminal lifecycle management, manual setup patterns.

**Related:** CT-1 (overview), CT-2 (events), ratatui-website/concepts-backends.md.

---

## crossterm-style.md (CT-4)

**What it covers:** The `crossterm::style` module for terminal text appearance. Covers the `Color`
enum, `Attribute` enum, style commands (`SetForegroundColor`, etc.), `ContentStyle`,
`StyledContent`, and the `Stylize` fluent trait. Documents NO_COLOR compliance.

**Key terms and concepts:**
- `Color` — `Reset`, named (Black/Red/etc.), `Rgb { r, g, b }`, `AnsiValue(u8)`
- `Attribute` — Bold, Italic, Underlined, Reverse, Hidden, CrossedOut, and more
- `SetForegroundColor(Color)`, `SetBackgroundColor(Color)`, `ResetColor`, `Print(value)`
- `ContentStyle` — groups fg, bg, underline, attributes
- `StyledContent<D>` — pairs style with displayable content
- `Stylize` trait — fluent methods on `&str`/`String` → `StyledContent`; `.red().bold()` etc.
- NO_COLOR: crossterm suppresses color when `NO_COLOR` env var is non-empty

**Governs:** Low-level terminal styling; use ratatui's `Style`/`Stylize` for TUI widgets instead.

**Related:** ratatui-website/api-style.md (ratatui's higher-level style API), cli-ux/no-color.md.

---

## color-eyre.md (CE-1)

**What it covers:** The `color-eyre` crate for colorized, human-readable error reports in TUI
binaries. Covers `install()` placement, `eyre::Result<T>` alias, `WrapErr` for context, section
helpers (`.with_note()`, `.with_suggestion()`), `bail!`, `ensure!`, and the canonical TUI error
handling pattern.

**Key terms and concepts:**
- `color_eyre::install()` — call once at top of `main()`, before any `?` usage
- `eyre::Result<T>` — alias for `Result<T, color_eyre::Report>`
- `WrapErr` trait — `.wrap_err("msg")` / `.wrap_err_with(|| ...)` for lazy context
- `color_eyre::Section` — `.with_note()`, `.with_warning()`, `.with_suggestion()`, `.with_error()`
- `eyre::bail!(msg)` — early return with error
- `eyre::ensure!(condition, msg)` — assertion that returns error on failure
- `RUST_BACKTRACE=1` — enables backtraces in error reports
- Canonical TUI pattern: `color_eyre::install()` → `ratatui::init()` → `run()` → `ratatui::restore()`

**Governs:** Error handling in all UMRS TUI binaries.

**Related:** ratatui-website/concepts-application-architecture.md (canonical main loop pattern).

---

## clap.md (CL-1)

**What it covers:** The `clap` crate for CLI argument parsing via the Derive API. Covers
`Parser`/`Args`/`Subcommand`/`ValueEnum` macros, `#[command(flatten)]`, nested subcommands,
range and custom validation, environment variable fallback, help text formatting, `--json` pattern,
and `arg_required_else_help`.

**Key terms and concepts:**
- `#[derive(Parser)]` on top-level struct — exposes `Cli::parse()`
- `#[derive(Args)]` — reusable argument group; inlined with `#[command(flatten)]`
- `#[derive(Subcommand)]` — enum where each variant is a subcommand
- `#[derive(ValueEnum)]` — enum restricting argument to a finite set of values
- `#[arg(short, long)]` — auto short/long flag from field name
- `#[arg(env = "VAR")]` — environment variable fallback (requires `env` feature)
- `value_parser = clap::value_parser!(u16).range(1..=65535)` — range validation
- `#[command(arg_required_else_help = true)]` — show help on missing args
- `--json` flag — UMRS convention for all structured-output commands
- Doc comments (`///`) become `--help` text

**Governs:** All UMRS CLI binary argument definitions.

**Related:** clig-guidelines.md (UX requirements that clap implements), CE-1 (error handling).

---

## clig-guidelines.md (CLIG-1)

**What it covers:** The Command Line Interface Guidelines — an open-source guide for writing
better CLI programs. Very large document covering: human output design, error messages, `--help`
standards, environment variables, `--json` for machine output, interactivity, signals, pipes,
subcommands, UX best practices, and philosophical foundations.

**Key terms and concepts:**
- Human output: designed for humans by default; use `--json` for machines
- Errors: go to stderr; actionable; say what went wrong and what to do next
- `--help` / `-h`: always works; includes examples; shows defaults
- `--json`: machine-readable output on stdout
- `--no-color` / `NO_COLOR`: honor unconditionally
- `--verbose` / `-v`: for operator-facing detail; not the default
- Exit codes: 0 = success; 1 = general error; 2 = misuse of shell command
- Never output ANSI colors when stdout is not a TTY
- `--version` / `-V`: always supported
- Subcommands: use when tool has multiple clearly distinct operations
- Stdin/stdout/stderr distinction: output on stdout, errors on stderr
- Config files: XDG convention (`~/.config/app/config`)
- Signals: handle SIGINT gracefully; clean up resources

**Governs:** UX design for all UMRS CLI output, error messages, flag naming, and behavior.

**Related:** no-color.md (NO_COLOR standard), clap.md (implementation), CT-4 (crossterm NO_COLOR).

---

## no-color.md (NC-1)

**What it covers:** The NO_COLOR informal standard. When the `NO_COLOR` environment variable is
set to any non-empty value, ANSI color output must be suppressed. Covers the detection pattern,
rationale, and companion `FORCE_COLOR` standard (for CI environments).

**Key terms and concepts:**
- `NO_COLOR` — env var; non-empty value means suppress all ANSI color
- `FORCE_COLOR` — companion standard; forces color even when stdout is not a TTY
- Detection: `if (getenv("NO_COLOR") != NULL && getenv("NO_COLOR")[0] != '\0')`
- Rust: `std::env::var("NO_COLOR").is_ok()` — but also check non-empty value
- crossterm: built-in suppression when `NO_COLOR` is set
- Ratatui inherits crossterm's suppression
- Does NOT affect cursor positioning or other terminal features — color only

**Governs:** Color suppression logic in all UMRS TUI and CLI output.

**Related:** CLIG-1 (UX guidelines), CT-4 (crossterm implementation).

---

## ratatui-website/architecture.md (WEB-ARCH)

**What it covers:** Ratatui v0.30.0 architecture from the official documentation. Covers the
modular workspace, rendering model (immediate mode with double buffering), layout system
(Cassowary solver), widget traits, and `no_std` support added in v0.30.0.

**Key terms and concepts:**
- Immediate rendering with intermediate buffers — redraw everything each frame
- Double buffering — two buffers compared; only diffs written to terminal
- Cassowary constraint solver — determines rectangle sizes from constraints
- `Widget` trait — consuming render; `StatefulWidget` — with external state
- `no_std` — supported in `ratatui-core` and `ratatui-widgets` since v0.30.0

**Governs:** Architectural decisions, understanding rendering model.

**Related:** ARCH-1 (overlapping), WEB-BACK (backend details).

---

## ratatui-website/concepts-layout.md (WEB-LAYOUT)

**What it covers:** Ratatui layout system — constraint types, Flex variants, negative spacing,
nested layouts, and the `centered_area` recipe. Covers the Cassowary solver behavior and how
constraints interact.

**Key terms and concepts:**
- `Constraint::Length(u16)` — absolute; `Percentage(u16)` — relative; `Ratio(u16, u16)` — fractional
- `Constraint::Min(u16)` — grows to absorb excess; `Constraint::Max(u16)` — ceiling
- `Constraint::Fill(u16)` — proportional excess distribution; lowest priority
- `Flex::Legacy` — excess at end (pre-v0.26 default); `Flex::Start` — current default
- `Flex::Center` — centered; `Flex::SpaceBetween` — between only; `Flex::SpaceEvenly` / `SpaceAround`
- `Layout::spacing(-1)` — negative spacing for overlapping segments
- `area.layout(&layout)` — array destructuring syntax
- `centered_area(area, percent_x, percent_y)` — canonical popup centering recipe

**Governs:** All UI layout decisions in UMRS TUI.

**Related:** WEB-ARCH (layout system overview), ratatui-examples/flex_layouts.rs (live demo),
ratatui-examples/popup.rs (centering in practice).

---

## ratatui-website/concepts-widgets.md (WEB-WIDGET)

**What it covers:** Widget system fundamentals — built-in widget catalog, `Widget`/`StatefulWidget`/
`WidgetRef` trait differences, rendering methods, external state management, the `Block` container,
custom widget implementation, and styling.

**Key terms and concepts:**
- `Widget` — consuming; `StatefulWidget` — with `&mut State`; implement `Widget for &T` (recommended)
- `frame.render_widget(w, area)` — for `Widget`; `frame.render_stateful_widget(w, area, state)` — for `StatefulWidget`
- Built-in: `Block`, `Paragraph`, `List`, `Table`, `Gauge`, `Sparkline`, `BarChart`, `Chart`, `Tabs`, `Canvas`, `Scrollbar`, `Calendar`
- `ListState`, `TableState`, `ScrollbarState` — external state types
- `Block` — container; use `.block(block)` method on most widgets; `block.inner(area)` for inner rect
- Custom widget: implement `Widget for &YourWidget`; use `Block::bordered().style()` + nested layout
- Styles are incremental — applying S1 then S2 merges both

**Governs:** Widget selection, custom widget design, state management patterns.

**Related:** WEB-LAYOUT (layout for positioning), ratatui-website/api-widgets-overview.md (widget API detail).

---

## ratatui-website/concepts-backends.md (WEB-BACK)

**What it covers:** Backend abstraction layer. Covers the available backends (Crossterm, Termion,
Termwiz), `CrosstermBackend`, the `Terminal` struct and double-buffering model, `DefaultTerminal`,
and the `Frame` API.

**Key terms and concepts:**
- `CrosstermBackend` — default; Linux/macOS/Windows; wraps a `Write` implementor
- `Terminal` — manages two buffers, cursor, viewport; generic over `Backend`
- `DefaultTerminal` = `Terminal<CrosstermBackend<Stdout>>` — use `ratatui::init()`
- `frame.area()` — full terminal area; `frame.set_cursor_position(pos)` — cursor control
- Modular backends in v0.30.0 — `ratatui-crossterm` etc. as separate crates
- Widget library authors: depend on `ratatui-core` only, not backend crates

**Governs:** Backend selection, terminal initialization, understanding `Frame`.

**Related:** CT-3 (crossterm terminal commands), WEB-ARCH (rendering model).

---

## ratatui-website/concepts-application-architecture.md (WEB-APP)

**What it covers:** Application architecture patterns — the basic event loop, `App` struct pattern,
three event handling strategies (blocking poll, channel-based, async/tokio), the Elm Architecture
(TEA), component decomposition, and tick-based timing.

**Key terms and concepts:**
- Immediate-mode: redraw everything every frame; no retained state in widgets
- Separate `run()` function from `main()` — ensures `restore()` called on early `?` return
- `App` struct — holds all business logic and state; methods for updates
- `impl Widget for &App` — renders without consuming; recommended pattern
- Blocking poll: `event::poll(timeout)?` + `event::read()?.as_key_press_event()`
- Channel-based: `std::sync::mpsc` thread + `rx.try_recv()` in main loop
- Async: `crossterm::event::EventStream` + `tokio::select!`
- TEA: Model + Message enum + `update(model, msg) -> Model` + view function
- Tick-based: `tick_rate.saturating_sub(last_tick.elapsed())` as poll timeout

**Governs:** App structure design, event loop implementation, architecture decisions.

**Related:** WEB-BACK (terminal setup), CT-2 (event API), CE-1 (error handling).

---

## ratatui-website/api-style.md (WEB-STYLE)

**What it covers:** Ratatui's `Style`, `Color`, `Modifier`, and `Stylize` API. Covers explicit
builder construction, Stylize shorthand, the Tailwind palette, underline color feature flag,
and `Into<Style>` conversions.

**Key terms and concepts:**
- `Style::default().fg(Color::Red).bg(Color::Blue).add_modifier(Modifier::BOLD)`
- `Style::reset()` — clears all styling back to terminal default
- `Color` — `Black`/`Red`/.../`Gray`/`DarkGray`, `Rgb(u8, u8, u8)`, `Indexed(u8)`
- Tailwind palette: `ratatui::style::palette::tailwind` — `tailwind::BLUE.c900`, `tailwind::SLATE.c200`
- `Modifier` — bitflags: `BOLD`, `DIM`, `ITALIC`, `UNDERLINED`, `REVERSED`, `HIDDEN`, `CROSSED_OUT`
- `Stylize` — fluent shorthand: `.red()`, `.on_blue()`, `.bold()`, `.italic()` on widgets and text
- `underline-color` feature flag — non-standard ANSI; crossterm backend only
- `Color` and `Modifier` implement `Into<Style>` — usable where `Into<Style>` is accepted

**Governs:** All color and styling decisions in UMRS TUI.

**Related:** CT-4 (crossterm raw styling), WEB-WIDGET (style application).

---

## ratatui-website/api-widgets-overview.md (WEB-WOV)

**What it covers:** Detailed API reference for the key built-in widgets: `Block`, `Paragraph`,
`List`, `Table`, `Gauge`, `Sparkline`, `Scrollbar`, `Tabs`, `Clear`, and the `Text`/`Line`/`Span`
text model.

**Key terms and concepts:**
- `Block::bordered()` / `Block::new().borders(Borders::TOP | ...)` / border types (`Rounded`, `Double`, `Thick`)
- `Block::inner(area)` — inner rect after borders/padding
- `Paragraph::new(text).wrap(Wrap { trim }).scroll((y, x)).alignment(Alignment::Center)`
- `List::new(items).highlight_style(...).highlight_symbol(">> ")` + `ListState`
- `Table::new(rows, widths).header(header).row_highlight_style(...).column_highlight_style(...)` + `TableState`
- `TableState::select_next_column()` / `select_previous_column()` — column selection
- `Gauge::default().percent(42).label("42%").gauge_style(...)`
- `Sparkline::default().data(&[...]).absent_value_style(...)` — v0.29+ absent value styling
- `Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(Some("↑")).end_symbol(Some("↓"))`
- `ScrollbarState::new(content_length).position(current_position)`
- `Scrollbar` rendered with `area.inner(Margin { vertical: 1, horizontal: 0 })`
- `Tabs::new(titles).select(idx).highlight_style(...).divider("|").padding(" ", " ")`
- `Clear` — clears area before rendering popup overlay on top
- `Text` → `Line` → `Span` hierarchy; `Line::centered()`, `Line::left_aligned()`

**Governs:** Widget API usage, interactive widget state management.

**Related:** WEB-WIDGET (concepts), ratatui-examples (live code).

---

## ratatui-examples/demo2_app.rs (EX-DEMO2)

**What it covers:** Multi-tab application with `App`-as-Widget pattern. Demonstrates: `Mode`
enum state machine (Running/Destroy/Quit), `Tab` enum with `strum` iteration, per-tab
navigation dispatch, nested layout (title bar / content / bottom bar), `Tabs` widget with
custom styling, key binding display in footer via `Line::from(spans).centered()`.

**Key terms and concepts:**
- `strum` crate — `EnumIter`, `FromRepr`, `Display` for tab enum
- `as_key_press_event()` — only fires on `KeyEventKind::Press`
- `area.layout(&layout)` destructuring into named `[title_bar, tab, bottom_bar]`
- `Tab::from_repr(index)` — safe index-to-variant conversion
- Separate render methods per region (render_title_bar, render_selected_tab, render_bottom_bar)

**Governs:** Multi-tab application structure reference.

**Related:** WEB-APP (architecture patterns), WEB-WOV (Tabs widget).

---

## ratatui-examples/popup.rs (EX-POPUP)

**What it covers:** Popup overlay pattern using `Clear` + `Block`, centered with `Flex::Center`.
Demonstrates `ratatui::run()` convenience function (v0.30+), toggle state with `bool`, and the
`centered_area()` helper function.

**Key terms and concepts:**
- `ratatui::run(|terminal| { ... })` — v0.30+ convenience wrapping init/restore
- `centered_area(area, percent_x, percent_y)` — `Flex::Center` on both axes
- `frame.render_widget(Clear, popup_area)` before popup content — clears background
- `Constraint::Fill(1)` for flexible content area

**Governs:** Popup and overlay implementation pattern.

**Related:** WEB-LAYOUT (Flex::Center), WEB-WOV (Clear widget).

---

## ratatui-examples/table.rs (EX-TABLE)

**What it covers:** Interactive table with row/column/cell selection, scrollbar integration,
Tailwind palette theming, and dynamic column width calculation. Shows `TableState`,
`ScrollbarState`, keyboard navigation with Shift modifier, and multi-cell multi-height rows.

**Key terms and concepts:**
- `TableState::default().with_selected(0)` — initial selection
- `state.select_next_column()` / `state.select_previous_column()`
- `row_highlight_style`, `column_highlight_style`, `cell_highlight_style` — three selection levels
- `HighlightSpacing::Always` — always reserve highlight column space
- Tailwind palette: `const PALETTES: [tailwind::Palette; 4]` for runtime color switching
- `unicode_width::UnicodeWidthStr::width` — correct column width calculation
- `ScrollbarState::new(n).position(i)` — keeps scrollbar synchronized with table row

**Governs:** Table widget usage, dynamic column sizing, combined scrollbar.

**Related:** WEB-WOV (Table/Scrollbar APIs), WEB-STYLE (Tailwind palette).

---

## ratatui-examples/scrollbar.rs (EX-SCROLL)

**What it covers:** Vertical and horizontal scrollbars with different symbol styles (arrows,
thumb-only, mirrored). Demonstrates tick-based event loop with `Duration` + `Instant`,
`Masked` widget for censored text display, and `ScrollbarOrientation` variants.

**Key terms and concepts:**
- `ScrollbarOrientation::VerticalRight`, `VerticalLeft`, `HorizontalBottom`
- `.begin_symbol(Some("↑")).end_symbol(Some("↓"))` / `None` for no arrows
- `.track_symbol(None)` — hide track
- `.thumb_symbol("🬋")` — custom thumb character
- `symbols::scrollbar::VERTICAL` — preset symbol set
- `Masked::new("password", '*')` — masks string for display
- `saturating_sub` / `saturating_add` for scroll offset arithmetic (no overflow)
- `tick_rate.saturating_sub(last_tick.elapsed())` — canonical tick timeout calculation

**Governs:** Scrollbar implementation patterns.

**Related:** WEB-WOV (Scrollbar API), EX-TABLE (scrollbar + table).

---

## ratatui-examples/flex_layouts.rs (EX-FLEX)

**What it covers:** Interactive explorer for all Flex variants and Constraint types. Demonstrates
`Buffer::empty` for off-screen rendering + scrolling, `split_with_spacers` for inspecting gaps,
`Layout::init_cache(NonZeroUsize)` tuning, `StatefulWidget` with custom state (`u16` spacing),
and `strum` for tab enum iteration.

**Key terms and concepts:**
- `split_with_spacers(area)` — returns `(blocks, spacers)` for layout visualization
- `Buffer::empty(area)` — create an isolated buffer for scrollable sub-rendering
- Copy cells from sub-buffer into main buffer with `skip(offset).take(visible)` — manual scroll
- `Layout::init_cache(NonZeroUsize::new(n).unwrap())` — pre-allocate cache for known layout count
- Constraint priority ordering: `Min/Max` > `Length` > `Percentage` > `Ratio` > `Fill`
- `Fill(0)` collapses when non-zero `Fill(_)` constraints are present

**Governs:** Complex layout debugging, scrollable buffer technique, layout cache tuning.

**Related:** WEB-LAYOUT (constraint semantics), ratatui-examples/scrollbar.rs.

---

## awesome-ratatui/README.md (AWE-1)

**What it covers:** Curated list of ratatui-based libraries (frameworks, widgets, utilities) and
applications (dev tools, sysadmin, networking, productivity). Useful for discovering third-party
widget crates and real-world usage patterns.

**Key terms and concepts (notable for UMRS):**
- `tui-logger` — logger widget with smart TUI integration
- `tui-input` — headless input library
- `ratatui-textarea` — multi-line text editor widget
- `tui-tree-widget` — tree view
- `tui-popup` — popup widget abstraction
- `systemctl-tui`, `systeroid`, `kmon`, `bottom`, `zenith` — system admin reference apps
- `gpg-tui`, `flawz` (CVE browser) — security-relevant tool examples

**Governs:** Third-party crate discovery; supply chain hygiene review required before adopting.

**Related:** cli-ux/awesome-tuis.md (broader TUI ecosystem).

---

## cli-ux/awesome-tuis.md (AWE-2)

**What it covers:** Broader catalog of TUI applications across all languages (Python, Go, C, C++,
Rust, .NET, Node). Useful for design inspiration and identifying established UX patterns in
security/sysadmin tools. Notable entries: `k9s`, `bottom`, `bpftop`, `kmon`, `journalview`,
`systeroid`, `termshark`.

**Key terms and concepts:**
- Security-adjacent TUI apps: `termshark` (Wireshark TUI), `flawz` (CVE browser), `gpg-tui`, `neoss`
- Sysadmin reference: `k9s`, `systemctl-tui`, `journalview`, `lazyjournal`
- Established Rust TUI libraries: `ratatui`, `tui-rs` (unmaintained), `iocraft`
- Go alternatives: `bubbletea` (Elm-based), `tview`, `tcell`

**Governs:** Design inspiration; not a dependency source.

**Related:** AWE-1 (ratatui-specific catalog).

---
