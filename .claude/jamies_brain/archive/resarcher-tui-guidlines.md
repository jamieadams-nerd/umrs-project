# TUI/CLI Agent RAG Corpus Acquisition Procedures

**Document type:** Agent operating procedures  
**Applies to:** Researcher agent  
**Collection name:** `tui-cli`  
**Purpose:** Acquire ratatui and CLI/TUI UX reference material for the
developer agent RAG corpus  
**Prerequisite:** Researcher agent has ingestion pipeline operational and
knows what to do with downloaded content.

---

## Section 1 — Ratatui Core Documentation

### 1.1 Ratatui Website — Full Site Crawl

**Source:** `https://ratatui.rs/`  
**Format:** Web (crawlable HTML)  
**Priority:** Critical

1. Crawl `https://ratatui.rs/` recursively, following all in-domain links.
2. Capture all sections including:
   - Tutorials (beginner through advanced)
   - How-to guides and recipes
   - Concepts (immediate rendering model, layout system, widget system)
   - Application architecture patterns
   - Showcase
3. Preserve code blocks verbatim — they are the primary value.
4. Store as structured markdown per page, preserving heading hierarchy.

---

### 1.2 Ratatui API Documentation

**Source:** `https://docs.rs/ratatui/latest/ratatui/`  
**Format:** Web (crawlable HTML)  
**Priority:** Critical

1. Crawl `https://docs.rs/ratatui/latest/ratatui/` recursively.
2. Capture all module documentation, struct/trait/enum documentation,
   and all inline code examples.
3. Pay particular attention to:
   - The `layout` module (constraint-based layout system)
   - The `widgets` module (all built-in widgets)
   - The `style` module (styling, color, modifiers)
   - The `Frame` and `Terminal` types (rendering lifecycle)
   - The `backend` module (crossterm, termion, termwiz backends)
4. Also capture the `ratatui-core`, `ratatui-widgets`, and
   `ratatui-crossterm` sub-crate docs — as of 0.30.0 the library is a
   modular workspace and these are distinct crates.

---

### 1.3 Ratatui Examples — Source Code

**Source:** `https://github.com/ratatui/ratatui/tree/main/examples`  
**Format:** Rust source files (GitHub)  
**Priority:** Critical — agents learn from working code, not descriptions

1. Clone or download the full `examples/` directory from the ratatui
   GitHub repository.
2. Capture all `.rs` files verbatim, preserving comments.
3. Pin to the latest stable release tag (not `main`) to ensure examples
   match released API. Check `https://crates.io/crates/ratatui` for the
   current stable version and use the corresponding tag.
   Example: `https://github.com/ratatui/ratatui/tree/v0.30.0/examples`
4. Also capture the `ratatui-widgets` examples if present in the
   workspace.

---

### 1.4 Awesome Ratatui

**Source:** `https://github.com/ratatui/awesome-ratatui`  
**Format:** Markdown (single file)  
**Priority:** High

1. Fetch the `README.md` from the awesome-ratatui repository.
2. This is a curated index of real-world applications and libraries built
   with ratatui. It is a pattern-mining resource — the developer agent
   uses it to understand what mature ratatui applications look like.
3. For any linked applications that are open source and appear
   architecturally significant (dashboard tools, monitoring tools,
   security tools), optionally fetch their `README.md` and any
   documented architecture notes. Do not attempt to ingest full source
   repositories.

---

## Section 2 — CLI/TUI UX Design

### 2.1 Command Line Interface Guidelines (CLIG)

**Source:** `https://clig.dev/`  
**Format:** Web (single page) and Markdown source  
**Priority:** Critical — primary UX design reference for all CLI/TUI work

1. Fetch the full page at `https://clig.dev/`.
2. Also fetch the raw source directly from GitHub for a clean markdown
   version:
   `https://raw.githubusercontent.com/cli-guidelines/cli-guidelines/main/content/_index.md`
3. The entire guide is a single markdown file — capture it completely.
4. This document governs how the UMRS TUI/CLI communicates with
   operators: output verbosity, error message design, flag naming
   conventions, state change feedback, subcommand structure, and
   composability. It is a design constraint document, not optional
   reading.

---

### 2.2 NO_COLOR Standard

**Source:** `https://no-color.org/`  
**Format:** Web (single page)  
**Priority:** High — mandatory convention for operator-facing tools

1. Fetch the full page at `https://no-color.org/`.
2. This is a short document. Capture it completely including the list of
   supporting applications.
3. The `NO_COLOR` environment variable must be honored unconditionally
   in all UMRS TUI output. This document is the authoritative reference
   for that requirement.

---

### 2.3 Awesome TUIs Reference List

**Source:** `https://github.com/rothgar/awesome-tuis`  
**Format:** Markdown (single file)  
**Priority:** Medium

1. Fetch the `README.md` from the awesome-tuis repository.
2. This is a broad catalog of TUI applications across languages. Use it
   as a pattern reference for keybinding conventions, navigation models,
   and layout approaches seen in mature TUI applications.
3. Focus on Rust entries and any security/monitoring/ops-focused tools
   regardless of language.

---

## Section 3 — Backend and Supporting Crates

### 3.1 crossterm API Documentation

**Source:** `https://docs.rs/crossterm/latest/crossterm/`  
**Format:** Web (crawlable HTML)  
**Priority:** High — ratatui's default backend; agents must understand
the event and terminal model beneath the widget layer

1. Crawl `https://docs.rs/crossterm/latest/crossterm/` recursively.
2. Prioritize:
   - The `event` module (key events, mouse events, event polling)
   - The `terminal` module (raw mode, alternate screen, size)
   - The `style` module (ANSI color and attribute handling)
3. Preserve all code examples verbatim.

---

### 3.2 color-eyre API Documentation

**Source:** `https://docs.rs/color-eyre/latest/color_eyre/`  
**Format:** Web (crawlable HTML)  
**Priority:** Medium — standard error and panic handler used throughout
ratatui examples; ensures consistent error handling

1. Fetch the top-level documentation page and the `section` module docs.
2. This is a relatively small crate — capture fully.
3. Focus on: installation pattern, `install()` call placement, `Report`
   type usage, and the `eyre::Result` alias. These appear in every
   ratatui example and the developer agent must recognize and replicate
   the pattern.

---

### 3.3 clap API Documentation (CLI argument parsing)

**Source:** `https://docs.rs/clap/latest/clap/`  
**Format:** Web (crawlable HTML)  
**Priority:** Medium — standard Rust CLI argument parser; governs how
UMRS exposes its command-line interface

1. Crawl `https://docs.rs/clap/latest/clap/` for the top-level docs
   and the derive macro documentation.
2. Also fetch the clap cookbook / examples:
   `https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html`
3. Focus on: the derive API (not the builder API), subcommand patterns,
   argument validation, and help text formatting conventions.

---

## Section 4 — Architecture Reference

### 4.1 Ratatui ARCHITECTURE.md

**Source:** `https://raw.githubusercontent.com/ratatui/ratatui/main/ARCHITECTURE.md`  
**Format:** Markdown (single file)  
**Priority:** High

1. Fetch the `ARCHITECTURE.md` from the ratatui repository directly.
2. This documents the 0.30.0 modular workspace reorganization — crate
   boundaries, design decisions, and the rationale for the split into
   `ratatui-core`, `ratatui-widgets`, etc.
3. The developer agent must understand this to make correct dependency
   decisions (e.g., whether a widget library should depend on
   `ratatui-core` vs. `ratatui`).

---

### 4.2 Ratatui BREAKING-CHANGES.md

**Source:** `https://raw.githubusercontent.com/ratatui/ratatui/main/BREAKING-CHANGES.md`  
**Format:** Markdown (single file)  
**Priority:** Medium

1. Fetch the `BREAKING-CHANGES.md` from the ratatui repository.
2. This documents API changes across versions. The developer agent uses
   it to avoid generating code against deprecated or removed APIs.

---

## Acquisition Checklist

| Item | Section | Priority | Format | Status |
|------|---------|----------|--------|--------|
| Ratatui website full crawl | 1.1 | Critical | Web crawl | — |
| Ratatui API docs (docs.rs) | 1.2 | Critical | Web crawl | — |
| Ratatui examples source code | 1.3 | Critical | Rust files | — |
| Awesome Ratatui | 1.4 | High | Markdown | — |
| CLIG — clig.dev | 2.1 | Critical | Web + Markdown | — |
| NO_COLOR standard | 2.2 | High | Web | — |
| Awesome TUIs | 2.3 | Medium | Markdown | — |
| crossterm API docs | 3.1 | High | Web crawl | — |
| color-eyre API docs | 3.2 | Medium | Web crawl | — |
| clap API docs + cookbook | 3.3 | Medium | Web crawl | — |
| ratatui ARCHITECTURE.md | 4.1 | High | Markdown | — |
| ratatui BREAKING-CHANGES.md | 4.2 | Medium | Markdown | — |

---

## Processing Note for Researcher

When ingesting the ratatui examples (Section 1.3), tag each file with
the widget or pattern it demonstrates. The developer agent retrieves
examples by pattern (e.g., "table with scrolling", "popup overlay",
"multi-pane layout") not by filename. Metadata tagging at ingest time
dramatically improves retrieval precision.

Suggested tags per file: widget names used, layout pattern, interaction
pattern (stateful/stateless), notable techniques demonstrated.
