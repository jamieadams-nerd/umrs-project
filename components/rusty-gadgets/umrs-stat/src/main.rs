// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
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

//! # umrs-stat — File Security Audit Card Binary
//!
//! Entry point for the `umrs-stat` command-line tool.  All reusable logic
//! lives in the `umrs_stat` library crate; this file contains only the
//! argument parsing, terminal lifecycle, and event loop.
//!
//! ## Usage
//!
//! ```text
//! umrs-stat <PATH>
//! umrs-stat <PATH> --json
//! umrs-stat --help
//! umrs-stat --version
//! ```
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Operator supplies the target path explicitly;
//!   no implicit subject selection occurs.
//! - **NIST SP 800-53 CM-6**: Output mode is operator-controlled via `--json`.

use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::Parser;
use crossterm::event::{self, Event};
use umrs_core::i18n;
use umrs_selinux::secure_dirent::SecureDirent;
use umrs_stat::FileStatApp;
use umrs_ui::app::{AuditCardApp, AuditCardState, DataRow, StatusMessage, TabDef};
use umrs_ui::indicators::{build_header_context, detect_os_name};
use umrs_ui::keymap::KeyMap;
use umrs_ui::layout::render_audit_card;
use umrs_ui::theme::Theme;

// ---------------------------------------------------------------------------
// AuditCardApp impl for FileStatApp in standalone TUI mode
// ---------------------------------------------------------------------------

/// Wrapper that implements `AuditCardApp` for the standalone TUI binary.
///
/// The `report_subject()` method leaks the path string to `&'static str` for
/// the binary's process lifetime.  This is intentional and acceptable for a
/// single-shot CLI tool; the `FileStatApp` library type uses `String` instead
/// so it can be embedded in longer-running contexts like `umrs-ls`.
///
/// NIST SP 800-53 AU-3 — every audit card in standalone mode is
/// self-identifying via report name and subject.
struct StandaloneAuditCard(FileStatApp);

impl AuditCardApp for StandaloneAuditCard {
    fn report_name(&self) -> &'static str {
        "File Security Audit"
    }

    fn report_subject(&self) -> &'static str {
        // Leak once per process lifetime — acceptable for a CLI binary.
        Box::leak(self.0.report_subject.clone().into_boxed_str())
    }

    fn card_title(&self) -> String {
        i18n::tr("File Security Audit")
    }

    fn tabs(&self) -> &[TabDef] {
        &self.0.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        self.0.rows_for_tab(tab_index).to_vec()
    }

    fn status(&self) -> &StatusMessage {
        &self.0.status
    }
}

// ---------------------------------------------------------------------------
// CLI argument definition
// ---------------------------------------------------------------------------

/// Command-line arguments for `umrs-stat`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Operator supplies the target path explicitly.
/// - **NIST SP 800-53 CM-6**: Output mode is operator-controlled via `--json`.
#[derive(Parser)]
#[command(
    name = "umrs-stat",
    version,
    about = "Display security attributes of a file as an interactive audit card"
)]
struct Args {
    /// Path to the file whose security attributes will be inspected.
    path: PathBuf,

    /// Emit machine-readable JSON output instead of the interactive TUI.
    ///
    /// JSON output support is reserved for a future implementation phase.
    #[arg(long)]
    json: bool,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    i18n::init("umrs-stat");

    if let Ok(logger) = systemd_journal_logger::JournalLog::new() {
        let _ = logger.install();
        log::set_max_level(log::LevelFilter::Info);
    }

    let args = Args::parse();

    let path_str = if let Some(s) = args.path.to_str() {
        s.to_owned()
    } else {
        eprintln!("error: path contains non-UTF-8 characters and cannot be displayed");
        std::process::exit(1);
    };

    log::debug!(
        "umrs-stat: json={} (JSON output not yet implemented)",
        args.json
    );

    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let dirent_result = SecureDirent::from_path(Path::new(&path_str));

    #[cfg(debug_assertions)]
    log::debug!(
        "TOCTOU-safe SecureDirent construction completed in {} µs",
        t0.elapsed().as_micros()
    );

    let mime: &str = match &dirent_result {
        Ok(_) => tree_magic_mini::from_filepath(Path::new(&path_str)).unwrap_or("unknown"),
        Err(_) => "unknown",
    };

    let stat_app: FileStatApp = match &dirent_result {
        Ok(dirent) => {
            log::info!("umrs-stat: loaded {path_str}");
            FileStatApp::from_dirent(dirent, mime)
        }
        Err(e) => {
            log::warn!("umrs-stat: failed to read {path_str}: {e}");
            FileStatApp::from_error(&path_str, e)
        }
    };

    let app = StandaloneAuditCard(stat_app);

    let mut state = AuditCardState::new(app.tabs().len());
    let keymap = KeyMap::default();
    // NO_COLOR environment variable compliance (https://no-color.org/).
    // Presence alone is the signal; the value is not decoded (`var_os`).
    // NIST SP 800-53 SI-11 / WCAG 1.4.1 — meaningful output without color.
    let theme = if std::env::var_os("NO_COLOR").is_some() {
        Theme::no_color()
    } else {
        Theme::dark()
    };
    let ctx = build_header_context(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        detect_os_name(),
    );

    let mut terminal = ratatui::init();

    loop {
        if let Err(e) = terminal.draw(|f| {
            render_audit_card(f, f.area(), &app, &state, &ctx, &theme);
        }) {
            log::error!("terminal draw error: {e}");
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
                    log::warn!("event read error: {e}");
                }
            },
            Ok(false) => {}
            Err(e) => {
                log::warn!("event poll error: {e}");
            }
        }

        if state.should_quit {
            break;
        }
    }

    ratatui::restore();
}
