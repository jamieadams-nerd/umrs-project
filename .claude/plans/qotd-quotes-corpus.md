# Plan: Quote of the Day (QOTD) — Engineering Culture Delivery

**Status:** Approved — fun future work

**ROADMAP alignment:** G8 (Human-Centered Design), G10 (AI Transparency — personality/culture)

**Source:** `.claude/jamies_brain/fun/idea.txt`, `.claude/jamies_brain/fun/jamie.md`

---

## Concept

Inspired by the classic Unix QOTD/`fortune` tradition: ambient, contextual engineering wisdom embedded into UMRS tools. Not policy enforcement — culture delivery. The kind of thing that makes a system feel alive, opinionated, and principled.

A curated corpus of engineering quotes — security, design, assurance, humor — displayed in a visually polished popup box. Available in both terminal (raw ANSI) and ratatui (TUI widget) modes.

---

## Corpus Design

### Structure

Two JSON source files merged into a single corpus, compiled into Rust constants (no runtime file I/O needed).

```rust
pub struct Quote {
    pub id: &'static str,           // e.g., "quote.thompson.trust"
    pub author: &'static str,       // attribution
    pub quote: &'static str,        // the text
    pub domains: &'static [&'static str],  // security, assurance, design, humor, etc.
    pub tags: &'static [&'static str],     // fine-grained topic tags
    pub tone: Tone,                 // Advisory, Critical, Humorous, Reflective, etc.
}

pub enum Tone {
    Advisory,
    Critical,
    Humorous,
    Reflective,
    Direct,
    Warning,
    Serious,
    Light,
    Neutral,
    Clarifying,
}
```

### Sources (to merge)

1. **Core corpus** (`idea.txt`) — ~30 entries including:
   - Classics: Knuth, Dijkstra, Thompson, Schneier, Hoare, Brooks, Kernighan, Pike, Wirth, Lampson
   - UMRS originals: "An event not recorded is an event that never happened", "One source is data. Two sources are evidence.", etc.
   - Unix philosophy: McIlroy trilogy
   - Tanenbaum standards quote (gap-filled per Jamie's catch)

2. **Gap-fill corpus** (`idea.txt` second section) — ~25 canonical additions:
   - Torvalds, Carmack, Feynman, Turing, Postel, Ritchie, Lamport, Dijkstra (simplicity), Stroustrup, Spolsky, Parnas, Hamming

3. **Future expansion** — corpus is designed for growth:
   - Stable IDs enable referencing in logs/UI/analytics
   - Domain/tag system supports contextual selection
   - Tone enables filtering by display context

### Corpus location

Compiled into `umrs-core` as a `const` array. Source JSON maintained alongside for editing convenience, with a build step or `include!()` macro to embed.

**Chosen approach:** Source of truth is a JSON file (`umrs-core/data/quotes.json`) that Jamie edits directly. A build script (`build.rs`) or `include_str!()` + `serde_json` at startup loads it. This keeps adding quotes trivial — edit JSON, rebuild, done. No Rust syntax to worry about.

Adding a quote is just appending to the JSON array:
```json
{
  "id": "quote.new.whatever",
  "author": "Someone",
  "quote": "The quote text.",
  "domains": ["security"],
  "tags": ["trust"],
  "tone": "advisory"
}
```

---

## Display: The QOTD Popup

### Visual Design

```
┌─ Quote of the Day ──────────────────────────────────┐
│                                                      │
│   "Simplicity is prerequisite for reliability."      │
│                                                      │
│                        — Edsger W. Dijkstra          │
│                                                      │
│                                  [ESC to close]      │
└──────────────────────────────────────────────────────┘
```

**Layout rules:**
- Always centered vertically and horizontally on screen
- Always wider than tall (landscape aspect ratio — looks better)
- Min width: 50 chars. Max width: 80% of terminal width
- Padding: 2 chars horizontal, 1 line vertical
- Quote text word-wrapped within box
- Author right-aligned, preceded by em-dash
- Title "Quote of the Day" inset into top-left border
- `[ESC to close]` hint in bottom-right corner
- Unicode box-drawing characters (falls back to ASCII if `TERM=dumb`)

### Text Effects (the pizzazz)

Leverage existing `umrs-core::console` infrastructure:

| Effect | Implementation | When to use |
|---|---|---|
| **Typography mapping** | `console::typography::stylize()` — DoubleStruck, Script, Bold, Gothic | Title text, author name |
| **Color gradients** | `console::ansi::fg_rgb()` — horizontal gradient across quote text | Default display mode |
| **Shimmer animation** | Cycle gradient offset on a timer (reuse spinner threading pattern) | Optional, `--animate` or config |
| **Typewriter effect** | Character-by-character reveal with slight delay | Optional, fun mode |
| **Random style rotation** | Pick a random `TypographyStyle` for the title each time | Adds variety |

### Two Rendering Backends

1. **Terminal mode** (`umrs-core::console`)
   - Uses `boxmsg::box_lines()` for the frame
   - ANSI escape sequences for color/effects
   - Listens for ESC keypress to dismiss
   - Works in any terminal, including over SSH

2. **Ratatui mode** (`umrs-tui`)
   - `ratatui::widgets::Block` + `Paragraph` centered popup
   - Same visual spec, native TUI rendering
   - Dismisses on ESC within the ratatui event loop
   - Overlay on top of existing TUI content (popup layer)

### API

```rust
/// Display a random Quote of the Day in a centered popup box.
///
/// Blocks until the user presses ESC.
///
/// # Parameters
/// - `style`: Optional text effect (gradient, typewriter, shimmer, etc.)
///
/// # Controls
/// - Presentation Tone Rule (approachable without reducing correctness)
pub fn show_qotd(style: Option<QotdStyle>) -> io::Result<()>

/// Get a random quote from the corpus.
pub fn random_quote() -> &'static Quote

/// Get a quote by domain (e.g., "security", "assurance").
/// Returns a random quote from matching domain entries.
pub fn quote_by_domain(domain: &str) -> Option<&'static Quote>

/// Get a quote by deterministic hash (stable selection from event ID, etc.)
pub fn quote_by_hash(seed: u64) -> &'static Quote

/// Display style for QOTD popup
pub enum QotdStyle {
    Plain,              // No effects, just the box
    Gradient,           // Horizontal color gradient on quote text
    Shimmer,            // Animated gradient cycling
    Typewriter,         // Character-by-character reveal
    Typography(TypographyStyle),  // Unicode alphabet mapping on title
}
```

### Contextual Selection (future)

The domain/tag system enables smart quote selection:

| Context | Domain filter | Example |
|---|---|---|
| Integrity check failure | `security`, `assurance` | Thompson trust quote |
| Logging anomaly | `audit`, `forensics` | "An event not recorded..." |
| Tool startup (idle) | Random | Any quote |
| Performance benchmark | `performance` | Knuth optimization quote |
| Post-lint clean run | `humor`, `light` | Kernighan debugging quote |

Deterministic mapping via `quote_by_hash()`:
```rust
let idx = hash(event_id) % quotes.len();
```

---

## Module Layout

```
umrs-core/src/quotes/
├── mod.rs          ← public API: random_quote(), quote_by_domain(), show_qotd()
├── corpus.rs       ← const QUOTES: [Quote; N] — the full compiled corpus
├── display.rs      ← terminal popup rendering (box + effects)
└── style.rs        ← QotdStyle, gradient/shimmer/typewriter implementations
```

For the ratatui backend (TUI popup):
```
umrs-tui/src/qotd.rs   ← ratatui widget: centered popup overlay
```

---

## File Changes

| File | Change |
|---|---|
| `umrs-core/src/quotes/mod.rs` | New module — `Quote`, `Tone`, selection functions |
| `umrs-core/src/quotes/corpus.rs` | Compiled corpus (~55+ quotes, const array) |
| `umrs-core/src/quotes/display.rs` | Terminal popup rendering with effects |
| `umrs-core/src/quotes/style.rs` | `QotdStyle`, gradient/shimmer/typewriter |
| `umrs-core/src/lib.rs` | Add `pub mod quotes;` |
| `umrs-tui/src/qotd.rs` | Ratatui popup overlay widget |
| `umrs-core/tests/quotes_tests.rs` | Corpus integrity, selection, display |

---

## Tests

1. Corpus has no duplicate IDs
2. All quotes have non-empty author and text
3. All domains are from a known set
4. `random_quote()` returns valid entry
5. `quote_by_domain("security")` returns only security-domain quotes
6. `quote_by_hash()` is deterministic (same seed → same quote)
7. `box_lines` output fits within specified max width
8. Typography styling doesn't corrupt non-ASCII characters in quotes

---

## Corpus Maintenance

- **Adding quotes:** Edit `umrs-core/data/quotes.json` — append to the array, rebuild. No Rust changes needed.
- Attribution must be verified — no internet misquotes
- UMRS-original quotes (tagged `"UMRS Principle"`) are project IP
- Corpus is append-only in normal operation (IDs are stable)
- Gap analysis pass recommended before v1: ensure the ~25 canonical engineering quotes are all present
- Startup validates corpus integrity (no duplicate IDs, required fields present) and logs warnings for any issues — never crashes on a bad quote entry

---

## Integration Points

1. **CLI startup banner** — random quote after tool version line
2. **TUI idle state** — popup when system is idle / no alerts
3. **Post-command footer** — optional quote after long-running operations
4. **`umrs-qotd` standalone command** — just shows a quote (like `fortune`)
5. **Wizard mascot** — the wizard could "say" the quote in a speech bubble (future)

---

## Fun Factor

This is exactly the kind of feature that makes UMRS feel alive — not just correct, but opinionated and principled. The Unix tradition of `fortune`, `/etc/motd`, and login banners built engineering culture. UMRS carries that forward with curated, domain-aware, beautifully rendered engineering wisdom.
