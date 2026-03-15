# tui-cli RAG Collection Index

Collection: `tui-cli`
Purpose: TUI and CLI development reference material for the UMRS project.
Last updated: 2026-03-15
Ratatui version covered: v0.30.0 (December 2024)

See `SOURCE.md` for source URLs and update check guidance.

---

## Directory Layout

```
tui-cli/
  _index.md                         — this file
  SOURCE.md                         — source URLs, provenance, update check URLs

  architecture/
    ratatui-architecture.md         — modular workspace, crate roles, Frame API, layout intro
    ratatui-breaking-changes.md     — breaking change log v0.20.0 through v0.31.0

  backend/
    crossterm-overview.md           — crossterm crate: Command API, execute!/queue! macros
    crossterm-event.md              — event poll/read, Event enum, KeyCode, mouse, Kitty protocol
    crossterm-terminal.md           — raw mode, alternate screen, terminal size, clear
    crossterm-style.md              — Color, Attribute, StyledContent, Stylize trait, NO_COLOR
    color-eyre.md                   — error handling for TUI apps, install(), WrapErr
    clap.md                         — CLI argument parsing: derive API, subcommands, ValueEnum

  cli-ux/
    clig-guidelines.md              — CLI design guidelines (human output, --json, errors)
    no-color.md                     — NO_COLOR environment variable convention

  ratatui-website/                  — (added 2026-03-15)
    architecture.md                 — v0.30.0 workspace structure, rendering model, layout system
    concepts-layout.md              — Constraint types, Flex variants, nested layouts, recipes
    concepts-widgets.md             — Widget traits, built-in widgets, custom widgets, styling
    concepts-backends.md            — CrosstermBackend, Terminal, Frame, raw mode, alt screen
    concepts-application-architecture.md — event loop patterns, App struct, TEA, async
    api-style.md                    — Style, Color, Modifier, Stylize trait, Tailwind palette
    api-widgets-overview.md         — Block, Paragraph, List, Table, Gauge, Scrollbar, Tabs,
                                      Clear, Text/Line/Span

  ratatui-examples/                 — (added 2026-03-15, ratatui v0.30.0)
    demo2_app.rs                    — multi-tab app, App-as-Widget, mode state machine
    popup.rs                        — popup overlay, Clear, Flex::Center centering
    table.rs                        — interactive Table with row/column/cell selection
    scrollbar.rs                    — vertical + horizontal scrollbars, tick-rate loop
    flex_layouts.rs                 — all Flex variants and Constraint types demonstrated

  awesome-ratatui/                  — (added 2026-03-15)
    README.md                       — curated list of ratatui libraries and applications
```

---

## Key Topics Covered

| Topic | Files |
|---|---|
| Application architecture | `ratatui-website/concepts-application-architecture.md`, `architecture/ratatui-architecture.md` |
| Layout and constraints | `ratatui-website/concepts-layout.md`, `ratatui-examples/flex_layouts.rs` |
| Widget API | `ratatui-website/concepts-widgets.md`, `ratatui-website/api-widgets-overview.md` |
| Styling | `ratatui-website/api-style.md`, `backend/crossterm-style.md` |
| Event handling | `backend/crossterm-event.md`, `ratatui-website/concepts-application-architecture.md` |
| Terminal setup | `backend/crossterm-terminal.md`, `ratatui-website/concepts-backends.md` |
| Error handling | `backend/color-eyre.md` |
| CLI argument parsing | `backend/clap.md` |
| CLI UX guidance | `cli-ux/clig-guidelines.md`, `cli-ux/no-color.md` |
| Breaking changes | `architecture/ratatui-breaking-changes.md` |
| Real-world apps | `awesome-ratatui/README.md` |
| Code examples | `ratatui-examples/*.rs` |

---

## Ratatui Version Notes

v0.30.0 introduced:
- Modular workspace: `ratatui-core`, `ratatui-widgets`, backend crates
- `no_std` support
- `ratatui::run()` convenience function (wraps init/draw/restore)
- `Sparkline::absent_value_style` and `absent_value_symbol`
- Layout cache tuning via `Layout::init_cache()`

v0.29.0 introduced:
- `Sparkline` `SparklineBar` type (breaking: `data()` now takes `IntoIterator<Item=SparklineBar>`)
- Negative spacing in layouts (`Layout::spacing(-1)`)

v0.28.1 introduced:
- `ratatui::init()` and `ratatui::restore()` convenience functions
- `DefaultTerminal` type alias

v0.26.0 introduced:
- `Flex` layout system (Start, End, Center, SpaceEvenly, SpaceBetween, SpaceAround, Legacy)
- `Constraint::Fill(u16)`
- `Layout::spacing()`
