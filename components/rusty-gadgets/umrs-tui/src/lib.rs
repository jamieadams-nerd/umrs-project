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

//! # umrs-tui — Audit Card Template Library
//!
//! Reusable ratatui-based audit card template for UMRS tool binaries.
//! Provides a structured, high-tech-looking TUI layout with:
//!
//! - Header panel (report name, hostname, subject, wizard logo)
//! - Tab bar for multiple views
//! - Scrollable key-value data area
//! - Single-line status bar with level-based color coding
//!
//! ## Usage Pattern
//!
//! 1. Implement [`app::AuditCardApp`] on your data struct.
//! 2. Create an [`app::AuditCardState`] with `AuditCardState::new(tab_count)`.
//! 3. Call [`layout::render_audit_card`] inside `terminal.draw(...)`.
//! 4. Feed [`keymap::KeyMap`] events into `state.handle_action(...)`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit record content — the card structure ensures
//!   that every report surface includes identification, subject, and status.
//! - **NSA RTB**: Presentation of security state must be unambiguous. Status
//!   levels (`Ok`, `Warn`, `Error`) map directly to actionable security postures.

pub mod app;
pub mod data_panel;
pub mod header;
pub mod keymap;
pub mod layout;
pub mod status_bar;
pub mod tabs;
pub mod theme;

// Convenience re-exports for callers
pub use app::{
    AuditCardApp, AuditCardState, DataRow, StatusLevel, StatusMessage,
    StyleHint, TabDef,
};
pub use keymap::{Action, KeyMap};
pub use layout::render_audit_card;
pub use theme::Theme;
