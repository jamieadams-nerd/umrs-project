---
name: tui-cli corpus knowledge
description: Key standards, conventions, and UMRS-specific audit checkpoints drawn from the tui-cli RAG collection (ratatui, crossterm, clap, CLIG, NO_COLOR)
type: project
---

# TUI/CLI Corpus — Audit Knowledge

Corpus ingested 2026-03-15. Location: `.claude/references/tui-cli/`.
Index: `.claude/references/tui-cli/_index.md`.
Ratatui version covered: v0.30.0 (December 2024).

## NO_COLOR Standard (mandatory UMRS requirement)

- When `NO_COLOR` is set to any non-empty value, ALL ANSI color output must be suppressed.
- Crossterm respects NO_COLOR internally — but only if the UMRS code does not bypass crossterm
  by writing raw ANSI sequences.
- UMRS TUI code must not hard-code `\x1b[...m` escape sequences anywhere.
- Current umrs-tui codebase: **no NO_COLOR check found in any source file** (confirmed by grep).
  Crossterm's built-in suppression is relied upon implicitly — this is acceptable IF
  the code never bypasses crossterm's style API. Must audit all `write!(stdout, ...)` calls.
- Companion: `FORCE_COLOR` forces color in CI; UMRS does not need to implement this.

## CLIG Key Rules for UMRS Audit Work

- **`--json` flag required** on all commands that return structured data (UMRS CLAUDE.md rule).
  Current status: `umrs-tui` main binary and `umrs-file-stat` have NO `--json` flag.
  This is a known gap; the TUI is the primary interface; `--json` belongs on the underlying
  platform crates' query interface, not necessarily the TUI binary itself.
- **Error messages to stderr**: TUI binaries should never write to stderr after `ratatui::init()`
  (would corrupt terminal state). Errors before init go to stderr; after init, log to journald.
  Current code correctly uses `eprintln!` only before terminal init (argument parsing).
- **`--help` / `-h`**: `umrs-file-stat` uses hand-rolled arg parsing with `--help` support.
  No clap used. CLIG recommends help on `-h`/`--help`; current implementation satisfies this.
- **Exit codes**: Binaries exit 1 on argument error (correct). No documented exit codes for
  detection failure vs. success — this is a gap for scripting use.
- **Verbose mode**: Neither binary implements `--verbose` / `-v`. Journald captures debug logs;
  for operator use `journalctl -f` is the intended mechanism.

## Clap Patterns to Enforce in Future Code Reviews

- All UMRS binaries must use clap derive API (not hand-rolled arg parsing).
  Exception: `umrs-file-stat` currently hand-rolls; this is acceptable for single-arg tools
  but should be migrated to clap for consistency when extended.
- `--json` flag: use `#[arg(long)]` with `bool` type; place in an `Output Options` help group.
- `ValueEnum` for format selection: `Text` / `Json` variants; default `Text`.
- `arg_required_else_help = true` on all multi-subcommand CLIs.
- Doc comments on Parser struct and fields become `--help` text — always fill them in.
- `#[command(version)]` reads from Cargo.toml — use it; do not hard-code version strings.

## Ratatui Architecture — Key Facts for Code Review

- Primary entry: `ratatui::init()` → enables raw mode + alternate screen + panic hook.
  Paired cleanup: `ratatui::restore()`. Both must be called; panic hook guarantees restore on panic.
- `DefaultTerminal` type alias = `Terminal<CrosstermBackend<Stdout>>`. Use the alias.
- `terminal.draw(|frame| { ... })` is the render entry point. Frame provides `area()` and
  `render_widget()` / `render_stateful_widget()`.
- Layout system uses Cassowary constraint solver. `Constraint::Fill(u16)` absorbs excess space.
  `Flex::Center` for centering popups. `Layout::spacing(-1)` for overlapping (use carefully).
- Widget traits: `Widget` (consuming), `StatefulWidget` (with external state), implement on `&T`
  (WidgetRef pattern) to avoid consuming the app struct.
- Implement `Widget for &App` so the app renders itself without being consumed.

## Current umrs-tui Compliance Status

- `ratatui::init()` / `ratatui::restore()` — correctly used in both binaries.
- Event loop pattern — correctly uses `event::poll()` with 250ms timeout.
- `color_eyre` — NOT used; error handling is `log::error!` + break. This is acceptable for
  a TUI binary (color_eyre's colored output would go to the wrong fd after init).
- Theme system — `Theme::default()` centralizes all color definitions; no inline color literals
  in render functions. Correct pattern.
- NO_COLOR — implicitly handled by crossterm; no explicit env check. Acceptable but note-worthy.
- `--json` flag — absent from both binaries (TUI-mode only). Not a compliance violation for
  the TUI binary itself; the underlying platform APIs (umrs-platform) should expose JSON output.
- Arg parsing — `umrs-file-stat` is hand-rolled. Functional but should migrate to clap.

## Ratatui v0.30.0 Breaking Changes (relevant for audit)

- `block::Title` removed — use `Line::from()` directly.
- `Flex::SpaceAround` semantics changed — outer gap is now half of inner gap.
- `Alignment` renamed to `HorizontalAlignment` (alias still present).
- `Frame::size()` renamed to `frame.area()` — current code uses `.area()` (correct).
- `Backend` now has an associated Error type.
- `Marker` is `#[non_exhaustive]` — add wildcard arm to exhaustive matches.

## Audit Checklist — TUI/CLI Code Review

When reviewing new TUI/CLI code:
1. Verify `ratatui::init()` + `ratatui::restore()` present and paired.
2. No `eprintln!` / `println!` after `ratatui::init()`.
3. No raw ANSI escape sequences (`\x1b`). All styling via ratatui `Style` / crossterm `Stylize`.
4. `NO_COLOR` — verify crossterm is the only color emission path; no bypasses.
5. `--json` flag present on commands returning structured data (CLAUDE.md requirement).
6. Error messages: pre-init to stderr with actionable description; post-init to journald.
7. Help text: `-h` / `--help` always available; doc comments fill `--help` text via clap.
8. Exit codes: 0 = success, nonzero = failure; document what each code means.
9. `clap` derive API used (not hand-rolled) for all but the simplest single-arg tools.
10. Version string: `#[command(version)]` reads from Cargo.toml; no hard-coded strings.
