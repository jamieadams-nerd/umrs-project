// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — compile-time proof.
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

//! # show_logo — Robot Gallery Audit Card Demo
//!
//! Minimal demonstration of the `umrs-tui` audit card template. Displays
//! all built-in robot ASCII art entries from `umrs-core` as a scrollable
//! audit card.
//!
//! This example is the guest-coder entry point for evaluating the audit
//! card API. It shows:
//! - Implementing [`AuditCardApp`] on a simple data struct
//! - Constructing [`AuditCardState`] and the event loop
//! - Using `render_audit_card` for all rendering
//!
//! Usage:
//! ```sh
//! cargo run -p umrs-tui --example show_logo
//! ```

use std::time::Duration;

use crossterm::event::{self, Event};
use umrs_core::robots::ALL_ROBOTS;
use umrs_tui::app::{
    AuditCardApp, AuditCardState, DataRow, StatusLevel, StatusMessage,
    StyleHint, TabDef,
};
use umrs_tui::keymap::KeyMap;
use umrs_tui::layout::render_audit_card;
use umrs_tui::theme::Theme;

// ---------------------------------------------------------------------------
// LogoDemoApp
// ---------------------------------------------------------------------------

/// Audit card data source for the robot gallery demo.
///
/// Single tab listing all `ALL_ROBOTS` entries with name, width, and height.
struct LogoDemoApp {
    tabs: Vec<TabDef>,
    rows: Vec<DataRow>,
    status: StatusMessage,
}

impl LogoDemoApp {
    fn new() -> Self {
        let tabs = vec![TabDef::new("Robots")];

        let mut rows = Vec::new();
        for robot in ALL_ROBOTS {
            rows.push(DataRow::new(
                "name",
                robot.name.to_owned(),
                StyleHint::Highlight,
            ));
            rows.push(DataRow::new(
                "  width",
                robot.width.to_string(),
                StyleHint::Normal,
            ));
            rows.push(DataRow::new(
                "  height",
                robot.height.to_string(),
                StyleHint::Normal,
            ));
            rows.push(DataRow::separator());
        }

        let status = StatusMessage::new(
            StatusLevel::Ok,
            format!("{} robots in gallery", ALL_ROBOTS.len()),
        );

        Self {
            tabs,
            rows,
            status,
        }
    }
}

impl AuditCardApp for LogoDemoApp {
    fn report_name(&self) -> &'static str {
        "Robot Gallery"
    }

    fn report_subject(&self) -> &'static str {
        "umrs-core built-in ASCII art"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => self.rows.clone(),
            _ => vec![DataRow::normal("(no data)", "(invalid tab index)")],
        }
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let app = LogoDemoApp::new();
    let mut state = AuditCardState::new(app.tabs().len());
    let keymap = KeyMap::default();
    let theme = Theme::default();

    let mut terminal = ratatui::init();

    loop {
        if let Err(e) = terminal.draw(|f| {
            render_audit_card(f, f.area(), &app, &state, &theme);
        }) {
            // Draw error — restore terminal and exit cleanly.
            eprintln!("draw error: {e}");
            break;
        }

        match event::poll(Duration::from_millis(250)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => {
                    if let Some(action) = keymap.lookup(&key) {
                        state.handle_action(&action);
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("event read error: {e}");
                }
            },
            Ok(false) => {}
            Err(e) => {
                eprintln!("event poll error: {e}");
            }
        }

        if state.should_quit {
            break;
        }
    }

    ratatui::restore();
}
