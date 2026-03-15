# tui-cli Collection — Cross-Reference Map

Generated: 2026-03-15
Ratatui version covered: v0.30.0

---

## Agreements

Where two or more documents reinforce the same guidance:

### Terminal Initialization Pattern
**Sources:** ARCH-1, WEB-BACK, CE-1, CT-3, WEB-APP, EX-POPUP

All documents agree on the canonical initialization sequence:
```
color_eyre::install()?;
let terminal = ratatui::init();
let result = run(terminal);
ratatui::restore();
result
```
`ratatui::init()` wraps raw mode + alternate screen + panic hook. Manual setup (pre-v0.28 style)
remains valid but is secondary. `ratatui::restore()` must be called even on error — achieved by
separating `run()` from `main()` and using `?` propagation.

### App-as-Widget Pattern
**Sources:** WEB-APP, WEB-WIDGET, EX-DEMO2

All agree: implement `Widget for &App` (shared reference, not consuming). This avoids cloning
state per frame and is the recommended pattern for the root application type.

### NO_COLOR Compliance
**Sources:** NC-1, CT-4, CLIG-1

All three agree: `NO_COLOR` must be honored unconditionally when set to any non-empty value.
crossterm has built-in suppression. UMRS code must not bypass this. CLIG also requires suppressing
color when stdout is not a TTY (even without `NO_COLOR`).

### `--json` for Machine Output
**Sources:** CLIG-1, CL-1

Both agree: all structured-output commands must provide `--json`. `clap`'s `ValueEnum` or a
plain `bool` flag named `json` is the implementation vehicle. CLIG specifies `--json` goes to
stdout while errors still go to stderr.

### `saturating_*` for Scroll Arithmetic
**Sources:** WEB-APP, EX-SCROLL, EX-FLEX

All examples consistently use `saturating_add(1)` and `saturating_sub(1)` for scroll offsets,
never plain `+` or `-`. This matches the project rule on secure arithmetic for bounded values.

### Flex::Start is the Current Default
**Sources:** ARCH-2, WEB-LAYOUT, EX-FLEX

Breaking changes log and concept docs agree: `Flex::Start` became default in v0.26.0.
Pre-v0.26 stretch behavior is now `Flex::Legacy`. This is the migration path for any code
ported from earlier ratatui versions.

### Widget Trait Implementation Style
**Sources:** ARCH-2, WEB-WIDGET, EX-DEMO2, EX-POPUP

v0.30.0 reversed the `WidgetRef` blanket impl. Current recommendation is:
```rust
impl Widget for &Foo {
    fn render(self, area: Rect, buf: &mut Buffer) { ... }
}
```
Not `impl WidgetRef for Foo`. All example code uses this form.

---

## Tensions

Where documents conflict or require judgment:

### Stylize trait import — v0.30.0 change
**Sources:** ARCH-2, WEB-STYLE, CT-4

In v0.30.0, `Style` no longer implements `Styled`. The `Stylize` trait import
(`use ratatui::style::Stylize`) may no longer be needed for `Style::new().red()` — those methods
are now directly on `Style`. However, `Stylize` is still needed for text types (`Line`, `Span`,
`&str`). **Judgment:** Always check whether `Stylize` is being used on a widget/text type (import
needed) vs. on `Style` itself (import may be dropped). Compiler will guide.

### `ratatui::run()` vs. manual loop
**Sources:** EX-POPUP, EX-SCROLL, EX-FLEX (use `ratatui::run()`); ARCH-1, WEB-APP (show manual loop)

`ratatui::run()` is a v0.30+ convenience that wraps init/restore/draw but constrains the loop
structure (it takes a closure). Manual loops are more flexible for complex App structures.
**Judgment:** Use `ratatui::run()` for simple examples; use the manual `init()`/`restore()` + `run()`
pattern for UMRS tools with structured App types.

### Color suppression: crossterm auto vs. manual check
**Sources:** CT-4 (crossterm auto-suppresses), NC-1 (describes env check logic)

crossterm suppresses color automatically via `NO_COLOR`. For ratatui TUI apps using crossterm,
no explicit `NO_COLOR` check should be needed in application code. However, for CLI output paths
that bypass ratatui (e.g., direct `println!` with ANSI codes), manual checking
(`std::env::var("NO_COLOR").map_or(false, |v| !v.is_empty())`) is required.
**Judgment:** Trust crossterm for TUI paths; add explicit check for CLI-only output paths.

### `event::read()` blocking vs. `poll()` + `read()`
**Sources:** WEB-APP (shows both), CT-2 (covers both), EX-DEMO2 (uses `poll()`)

Simple apps may call `event::read()` blocking. Tick-based apps need `poll(timeout)` to allow
regular frame redraws without waiting for input. **Judgment:** Always use `poll(timeout)?` for
UMRS tools — they need periodic updates for security status indicators.

---

## Chains

Where one document defers to another:

- **ARCH-1** defers detail on the rendering model to **WEB-ARCH** and **WEB-BACK**
- **ARCH-1** defers widget details to **WEB-WOV** and **WEB-WIDGET**
- **CT-1** (overview) defers event detail to **CT-2**, terminal detail to **CT-3**, styling to **CT-4**
- **WEB-WIDGET** defers complete widget API detail to **WEB-WOV**
- **WEB-APP** defers terminal setup to **WEB-BACK** and **CT-3**
- **CLIG-1** is the authoritative UX guide; **CL-1** (clap) is its implementation vehicle
- **NC-1** is the authoritative NO_COLOR standard; **CT-4** and **CLIG-1** reference it

---

## Gaps

Topics not covered in this corpus that may arise in UMRS work:

### Testing TUI applications
The corpus contains no guidance on `TestBackend` usage or snapshot-based TUI testing (e.g.,
`insta` crate). Any testing of TUI rendering will need external research. Note: `TestBackend`
was updated in v0.30.0 (uses `Infallible` for errors — see ARCH-2).

### Focus management
No coverage of focus traversal between interactive widgets (Tab key cycling, mouse click focus).
Third-party crates handle this: `ratatui-interact`, `rat-widget`. UMRS TUI currently uses
simple single-focus designs.

### Mouse interaction in UMRS context
`EnableMouseCapture` is documented (CT-2) but no UMRS-specific guidance on whether mouse input
is desired. CLIG-1 does not strongly mandate mouse support for security operator tools.

### Async tokio integration
`EventStream` is mentioned (WEB-APP, CT-2) but not covered in depth. UMRS tools currently use
synchronous event loops. If async is needed, refer to external ratatui async templates.

### Shell completion generation
clap supports `clap_complete` for shell completion generation. Not covered in the corpus.
May be relevant for operator ergonomics.

### Unicode and CJK column width
The `table.rs` example uses `unicode_width::UnicodeWidthStr` for column sizing. The corpus
does not explain the full Unicode column width model. Important for any multi-language content.

### Custom backend implementation
ARCH-2 covers the `Backend` associated Error type change in v0.30.0 but the corpus has no
content on implementing a custom backend. UMRS currently uses `CrosstermBackend`.

### Performance profiling for TUI rendering
No coverage of profiling ratatui render performance. The layout cache (ARCH-1, EX-FLEX) is
the primary tuning lever; its use is documented but profiling guidance is absent.
