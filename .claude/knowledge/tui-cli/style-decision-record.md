# tui-cli Collection — Style Decision Record

Generated: 2026-03-15
Refreshed: 2026-03-28 (all existing decisions verified; one new placeholder added)
Ratatui version covered: v0.30.0

This file records project-specific resolutions to cross-reference tensions identified in
`cross-reference-map.md`. Each entry states the ruling and the conditions under which it applies.

---

## SDR-1: Terminal Initialization Pattern

**Tension:** `ratatui::run()` (v0.30+) vs. manual `init()`/`restore()` loop.

**Ruling:** Use the manual pattern for all UMRS tool binaries:
```rust
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}
```

**Why:** UMRS tools have structured `App` types with state machines and need full control over
the event loop. `ratatui::run()` works for simple demo-style loops but constrains the structure.
The manual pattern also makes `restore()` placement explicit and auditable.

**Context:** `ratatui::run()` is acceptable in `examples/` only where brevity is the goal.

---

## SDR-2: Widget Trait Implementation

**Tension:** `Widget for &Foo` (current v0.30+) vs. `WidgetRef for Foo` (removed in v0.30.0).

**Ruling:** Always implement `Widget for &YourType` for application and component types.
Never use `WidgetRef` for new code.

```rust
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) { ... }
}
```

**Why:** `WidgetRef` blanket impl was reversed in v0.30.0 (see ARCH-2). Implementing `Widget for &T`
is both the current idiomatic form and compatible with future ratatui releases.

---

## SDR-3: NO_COLOR Check Placement

**Tension:** crossterm auto-suppresses NO_COLOR vs. manual check needed for non-TUI output.

**Ruling:**
- **TUI rendering paths** (ratatui widgets, styles): rely on crossterm's built-in NO_COLOR
  suppression — no explicit check needed in application code.
- **CLI output paths** (direct `println!`/`eprintln!` with ANSI): add an explicit check at
  startup and store the result:
  ```rust
  let color_enabled = std::env::var("NO_COLOR").map_or(true, |v| v.is_empty());
  ```
  Pass `color_enabled` to any function that writes styled text outside ratatui.

**Why:** The CLAUDE.md rules require honoring NO_COLOR unconditionally. For TUI paths, crossterm
handles it. For CLI paths, the burden is on the application.

---

## SDR-4: Event Loop Strategy

**Tension:** Blocking `event::read()` vs. `poll(timeout)?` + conditional `read()`.

**Ruling:** Always use `poll(timeout)?` + `read()?` pattern in UMRS tools:

```rust
fn handle_events(&mut self) -> Result<()> {
    let timeout = Duration::from_millis(50);
    if event::poll(timeout)? {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code { ... }
        }
    }
    Ok(())
}
```

**Why:** UMRS tools will display live security status indicators that must refresh periodically.
Blocking on `event::read()` would freeze the display. The poll timeout controls frame rate.
50ms (~20 fps) is appropriate for security posture dashboards.

---

## SDR-5: Scroll Arithmetic

**Tension:** Plain `+`/`-` vs. `saturating_add`/`saturating_sub` for scroll offsets.

**Ruling:** Always use `saturating_add(1)` / `saturating_sub(1)` for scroll offsets and counters.
Never use plain arithmetic on `usize` or `u16` scroll values.

**Why:** UMRS has a project rule on secure arithmetic for bounded values (NIST SP 800-53 SI-10).
Scroll values are bounded by content length; overflow or underflow silently wrapping would be a
logic error. `saturating_*` is the correct safe choice.

---

## SDR-6: Color Palette Selection

**Tension:** Named colors (e.g., `Color::Red`) vs. Tailwind palette vs. Rgb.

**Ruling:**
- **Default UX**: Use Tailwind palette constants (`tailwind::BLUE.c900`, `tailwind::SLATE.c200`)
  for themed UI elements (headers, selected rows, borders). This produces professional-looking,
  consistent results.
- **Security status indicators**: Use explicit named colors or Rgb for unambiguous status signals
  (red for critical, yellow for warning, green for healthy). These must be distinguishable even
  on terminals with limited palettes.
- **Dark backgrounds**: Prefer `tailwind::SLATE.c950` / `c900` for backgrounds. UMRS tools target
  operator terminals which are typically dark-themed.

**Why:** Consistent theming improves readability and professionalism in a security tool. The wizard
mascot/posture-reflective theming goal (from project memory) benefits from a coherent palette.

---

## SDR-7: Clap `--json` Flag

**Tension:** `--json` as a `bool` flag vs. `ValueEnum` format selection.

**Ruling:** For commands that have exactly two output modes (human + JSON), use a plain `bool`
flag:
```rust
#[arg(long)]
json: bool,
```
For commands with more than two modes (e.g., text, json, csv), use `ValueEnum`:
```rust
#[derive(ValueEnum)]
enum OutputFormat { Text, Json }
```

**Why:** `bool` is simpler and matches CLIG guidance that `--json` is the conventional flag name.
`ValueEnum` is more extensible but adds complexity when only two modes exist.

---

## SDR-8: Layout Destructuring Style

**Tension:** Array destructuring `let [a, b, c] = area.layout(&layout)` vs. index access
`chunks[0]`, `chunks[1]`.

**Ruling:** Always use array destructuring with meaningful names. Never use index access.

```rust
// Preferred:
let [header, content, footer] = frame.area().layout(&layout);

// Rejected:
let chunks = Layout::vertical([...]).split(frame.area());
render(chunks[0]); // unclear what chunks[0] represents
```

**Why:** Named destructuring makes the relationship between layout regions and their contents
immediately clear to reviewers. It also eliminates off-by-one index errors.

---

## SDR-9: `block::Title` Usage (v0.30.0 migration)

**Tension:** Old `block::Title::from("text").position(...)` API was removed in v0.30.0.

**Ruling:** Always use the new title API:
```rust
// Title at top:
Block::bordered().title(Line::from("My Title"))

// Title at bottom:
Block::bordered().title_bottom(Line::from("My Title").centered())

// Title at top right:
Block::bordered().title(Line::from("Right").right_aligned())
```

**Why:** `block::Title` and `block::Position` were removed in v0.30.0 (ARCH-2). New code must use
the fluent `Line`-based API. This is a hard requirement to avoid compile errors.

---

## SDR-10: `Frame::size` vs. `Frame::area`

**Tension:** `frame.size()` (deprecated v0.28.0) vs. `frame.area()` (current).

**Ruling:** Always use `frame.area()`. Never use `frame.size()`.

**Why:** `frame.size()` was deprecated in v0.28.0 and renamed to `frame.area()` (ARCH-2).
Using the deprecated name will produce warnings and eventually fail.

---

## Placeholders — Decisions Requiring Project Owner Input

### PH-1: Async event loop adoption — DECIDED (2026-03-15)
**Decision:** Yes — adopt async tokio for TUI event loops.
**Rationale:** Enables concurrent I/O for live security status feeds without blocking the render loop.
SDR-4's synchronous `poll(timeout)` pattern remains acceptable for simple tools; tokio is the path
forward for tools with background data sources.

### PH-2: Mouse support policy — DECIDED (2026-03-15)
**Decision:** Keyboard-only for now.
**Rationale:** UMRS targets operator terminals, often over SSH. Keyboard-only ensures accessibility
and compatibility with remote sessions. Mouse support may be revisited for specific tools later.

### PH-3: Shell completion generation — DECIDED (2026-03-15)
**Decision:** Yes — ship shell completions via `clap_complete`.
**Rationale:** Low effort with clap, high ergonomic value for operators. Generate for bash, zsh,
and fish.

---

### PH-4: `#[expect]` attribute for lint suppression — OPEN (2026-03-28)
**Context:** The ratatui example code (`table.rs`, `flex_layouts.rs`) uses `#[expect(clippy::...)]`
rather than `#[allow(clippy::...)]`. `#[expect]` (stabilized Rust 1.81+) causes a compiler error
if the named lint no longer fires, making suppressions self-auditing and safer than `#[allow]`.
CLAUDE.md requires Jamie's approval before adding any `#[allow]` attribute. It is not clear whether
the same approval requirement applies to `#[expect]`.

**Question for Jamie:** Is `#[expect(clippy::cast_possible_truncation)]` pre-approved for
known-bounded `usize`→`u16` casts (e.g., terminal width values), or does it require the same
review process as `#[allow]`?

**Interim ruling:** Treat `#[expect]` as requiring the same approval process as `#[allow]` until
Jamie decides. Do not add either unilaterally.
