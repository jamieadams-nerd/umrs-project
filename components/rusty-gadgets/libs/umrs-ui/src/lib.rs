// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
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

//! # umrs-ui — TUI Template Library
//!
//! Reusable ratatui-based TUI templates for UMRS tool binaries.
//! Provides three distinct layout patterns:
//!
//! ## Patterns
//!
//! ### AuditCardApp
//!
//! Security assessment tools (umrs-stat, umrs-uname). Header shows live
//! kernel security posture (SELinux, FIPS, lockdown). Scrollable key-value
//! data, tabs, status bar with the wizard logo.
//!
//! 1. Implement [`app::AuditCardApp`] on your data struct.
//! 2. Create an [`app::AuditCardState`] with `AuditCardState::new(tab_count)`.
//! 3. Call [`indicators::read_security_indicators`] once to populate a
//!    [`app::SecurityIndicators`] snapshot; then call
//!    [`indicators::build_header_context`] to build the [`app::HeaderContext`]
//!    from the snapshot and your app data.
//! 4. Call [`layout::render_audit_card`] inside `terminal.draw(...)`.
//! 5. Feed [`keymap::KeyMap`] events into `state.handle_action(...)`.
//!
//! ### ViewerApp
//!
//! Read-only hierarchical data browsers (umrs-labels catalog viewer, umrs-ls
//! TUI mode). Tool-contextual header (tool name, data source, record count,
//! breadcrumb). Tree navigation with expand/collapse, search/filter, detail panel.
//!
//! 1. Implement [`viewer::ViewerApp`] on your data struct.
//! 2. Create a [`viewer::ViewerState`] with `ViewerState::new(tab_count)`.
//! 3. Populate `state.tree` with root nodes; call `state.tree.rebuild_display()`.
//! 4. Call [`viewer::render_viewer`] inside `terminal.draw(...)`.
//! 5. Feed [`keymap::KeyMap`] events into `state.handle_action(...)`.
//!
//! ### ConfigApp
//!
//! Interactive configuration editors (label assignment, setrans.conf editing).
//! Config-contextual header (tool name, target, dirty/clean, validation state).
//! Editable fields with per-field validation, diff view, save/discard.
//!
//! 1. Implement [`config::ConfigApp`] on your data struct.
//! 2. Create a [`config::ConfigState`] with `ConfigState::new(tab_count)`.
//! 3. Populate `state.fields` with [`config::fields::FieldDef`] instances.
//! 4. Call [`config::render_config`] inside `terminal.draw(...)`.
//! 5. Feed [`keymap::KeyMap`] events into `state.handle_action(...)`.
//! 6. On `ConfigStateEvent::Save`: call `state.validate_all()`, check
//!    `state.can_save()`, perform I/O, call `state.mark_saved()`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Every pattern ensures identification, subject,
//!   status, and context are present in every rendered frame.
//! - **NIST SP 800-53 SI-7**: Security indicators are sourced from
//!   provenance-verified kernel attribute reads (AuditCardApp only).
//! - **NIST SP 800-53 CM-3**: ConfigApp gates saves on validation; diff view
//!   exposes all pending changes before commit.
//! - **NIST SP 800-53 SI-10**: Every ConfigApp field carries a validator;
//!   values are never committed without passing.
//! - **NIST SP 800-53 AC-3**: ViewerApp is unconditionally read-only.
//! - **NSA RTB**: Security state is always typed, never free-form strings.

pub mod app;
pub mod config;
pub mod data_panel;
pub mod dialog;
pub mod header;
pub mod indicators;
pub mod keymap;
pub mod layout;
pub mod status_bar;
pub mod tabs;
pub mod theme;
pub mod viewer;

// Convenience re-exports for callers
pub use app::{
    AuditCardApp, AuditCardState, ColumnLayout, DataRow, HeaderContext,
    HeaderField, IndicatorValue, SecurityIndicators, StatusLevel,
    StatusMessage, StyleHint, TabDef,
};
pub use config::{
    ConfigApp, ConfigHeaderContext, ConfigState, ConfigStateEvent, render_config,
};
pub use config::fields::{FieldDef, FieldValue, ValidationResult};
pub use dialog::{DialogFocus, DialogMode, DialogState, render_dialog};
pub use indicators::{
    build_header_context, read_security_indicators, read_system_uuid,
};
pub use keymap::{Action, KeyMap};
pub use layout::render_audit_card;
pub use theme::Theme;
pub use viewer::{ViewerApp, ViewerHeaderContext, ViewerState, render_viewer};
pub use viewer::tree::{NodeId, TreeModel, TreeNode};
