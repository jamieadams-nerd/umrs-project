# How to Add a New TUI Binary

This guide explains how to create a new audit card binary inside the `umrs-tui`
crate. No new crate is needed -- every binary shares the same audit card template
library.

---

## Architecture

The `umrs-tui` crate has two halves:

```
umrs-tui/
  src/
    lib.rs          <-- shared library: AuditCardApp trait, Theme, KeyMap,
    app.rs              render_audit_card(), header, tabs, data panel,
    theme.rs            status bar, layout composition
    keymap.rs
    layout.rs
    header.rs
    tabs.rs
    data_panel.rs
    status_bar.rs

    main.rs         <-- binary #1: umrs-os-detect-tui (OS detection card)

    bin/            <-- future binaries go here
      file_stat.rs  <-- binary #2: umrs-file-stat (planned)
```

**Library** (`src/lib.rs` + modules) -- reusable card template. All binaries
import from `umrs_tui::*`.

**Binaries** (`src/main.rs`, `src/bin/*.rs`) -- each implements `AuditCardApp`
and provides its own data. Gets the header, tabs, scrolling, keyboard handling,
theme, and status bar for free.

---

## Step-by-Step: Add a New Binary

### 1. Create the source file

Create `src/bin/<your_binary>.rs`. For example, `src/bin/file_stat.rs`.

### 2. Register it in Cargo.toml

Add a `[[bin]]` section. The existing sections stay untouched:

```toml
# --- Existing (do not touch) ---

[lib]
name = "umrs_tui"
path = "src/lib.rs"

[[bin]]
name = "umrs-os-detect-tui"
path = "src/main.rs"

# --- Add this ---

[[bin]]
name = "umrs-file-stat"
path = "src/bin/file_stat.rs"
```

You can add as many `[[bin]]` sections as you need. Each gets its own binary
name and source file.

### 3. Add any new dependencies (if needed)

If your binary needs crates that the library doesn't already depend on, add them
to `[dependencies]` in `Cargo.toml`. The library deps (`ratatui`, `crossterm`,
`umrs-core`, `umrs-platform`, etc.) are already available to all binaries.

### 4. Write the binary

Every binary follows the same pattern. Here is a minimal template:

```rust
// src/bin/file_stat.rs

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::unreadable_literal)]

//! # umrs-file-stat — File Security Audit Card
//!
//! Displays a security-enriched audit card for a single file path,
//! including SecureDirent metadata, MIME type, and SELinux context.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p umrs-tui --bin umrs-file-stat -- /etc/os-release
//! ```

use std::time::Duration;

use crossterm::event::{self, Event};
use umrs_tui::{
    AuditCardApp, AuditCardState, DataRow, KeyMap, StatusLevel,
    StatusMessage, StyleHint, TabDef, Theme, render_audit_card,
};

// ---------------------------------------------------------------------------
// Your data struct
// ---------------------------------------------------------------------------

struct FileStatApp {
    tabs: Vec<TabDef>,
    // your data fields here
    status: StatusMessage,
}

impl FileStatApp {
    fn new(/* your arguments */) -> Self {
        let tabs = vec![
            TabDef::new("File Info"),
            // add more tabs as needed
        ];

        Self {
            tabs,
            status: StatusMessage::new(StatusLevel::Info, "Ready"),
        }
    }
}

// ---------------------------------------------------------------------------
// Implement the trait -- this is the "plug" in plug-and-play
// ---------------------------------------------------------------------------

impl AuditCardApp for FileStatApp {
    fn report_name(&self) -> &'static str {
        "File Security Audit"
    }

    fn report_subject(&self) -> &'static str {
        // In practice, return the file path being inspected.
        "/path/to/file"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => {
                // Build your key-value rows here.
                vec![
                    DataRow::new("field", "value", StyleHint::Normal),
                    DataRow::new("SELinux context",
                                 "system_u:object_r:etc_t:s0",
                                 StyleHint::Highlight),
                    DataRow::new("MIME type",
                                 "text/plain",
                                 StyleHint::Dim),
                ]
            }
            _ => vec![],
        }
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }
}

// ---------------------------------------------------------------------------
// Entry point -- same pattern as every binary
// ---------------------------------------------------------------------------

fn main() {
    // Best-effort journald logging
    if let Ok(logger) = systemd_journal_logger::JournalLog::new() {
        let _ = logger.install();
        log::set_max_level(log::LevelFilter::Info);
    }

    // Build your data (parse args, run detection, read file, etc.)
    let app = FileStatApp::new();

    // UI state, keymap, theme -- same for every binary
    let mut state = AuditCardState::new(app.tabs().len());
    let keymap = KeyMap::default();
    let theme = Theme::default();

    // Terminal setup
    let mut terminal = ratatui::init();

    // Event loop -- identical in every binary
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_audit_card(f, f.area(), &app, &state, &theme);
        }) {
            log::error!("terminal draw error: {e}");
            break;
        }

        match event::poll(Duration::from_millis(250)) {
            Ok(true) => {
                if let Ok(Event::Key(key)) = event::read() {
                    if let Some(action) = keymap.lookup(&key) {
                        state.handle_action(&action);
                    }
                }
            }
            Ok(false) => {}
            Err(e) => {
                log::warn!("event poll error: {e}");
            }
        }

        if state.should_quit {
            break;
        }
    }

    // Terminal teardown
    ratatui::restore();
}
```

### 5. Build and run

```bash
# Build everything in the crate (library + all binaries)
cargo build -p umrs-tui

# Run a specific binary
cargo run -p umrs-tui --bin umrs-os-detect-tui
cargo run -p umrs-tui --bin umrs-file-stat -- /etc/os-release

# Run the example
cargo run -p umrs-tui --example show_logo
```

### 6. Verify

```bash
cd components/rusty-gadgets
cargo xtask fmt
cargo xtask clippy    # must be zero warnings
cargo xtask test      # must pass
```

---

## What You Write vs. What You Get for Free

| You write (per binary)                     | The library gives you (shared)         |
|--------------------------------------------|----------------------------------------|
| `struct YourApp` with your data fields     | Header panel with hostname and logo    |
| `impl AuditCardApp` -- 6 methods          | Tab bar with keyboard navigation       |
| `data_rows()` -- your key-value pairs      | Scrollable data area with scrollbar    |
| `status()` -- your status message          | Color-coded status bar                 |
| Build your data in `main()`                | Theme with trust-level color mapping   |
| (event loop is copy-paste boilerplate)     | KeyMap with standard TUI keybindings   |

---

## Custom Keybindings

The default `KeyMap` provides standard TUI keybindings. If your binary needs
extra keys, you can extend it:

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use umrs_tui::keymap::{Action, KeyMap};

let mut keymap = KeyMap::default();

// Add r for refresh
keymap.bind(
    KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE),
    Action::Refresh,
);
```

Then handle `Action::Refresh` after `state.handle_action()` in your event loop
to re-run your data gathering logic.

---

## Custom Theme

If a binary needs different colors:

```rust
use ratatui::style::{Color, Style};
use umrs_tui::Theme;

let mut theme = Theme::default();
theme.border = Style::default().fg(Color::Magenta);
```

All style fields on `Theme` are public, so you can override any of them.

---

## Planned Binaries

| Binary name         | Purpose                                               |
|---------------------|-------------------------------------------------------|
| `umrs-os-detect-tui`| OS detection pipeline results (implemented)           |
| `umrs-file-stat`   | SecureDirent audit of a file: metadata, MIME, SELinux |
| *(future)*          | SELinux status card, CUI label viewer, etc.           |

---

## Quick Reference

| Want to...                           | Do this                                        |
|--------------------------------------|------------------------------------------------|
| Change the card layout/theme/widgets | Edit `src/lib.rs` modules -- all binaries benefit |
| Add a new TUI tool                   | Add `src/bin/<name>.rs` + `[[bin]]` in Cargo.toml |
| Add shared helpers                   | Add to the library side (`src/*.rs` modules)   |
| Create a completely different UI     | Consider a new crate                           |
