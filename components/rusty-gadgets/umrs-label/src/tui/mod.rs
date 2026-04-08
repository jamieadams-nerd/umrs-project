// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # TUI — Interactive Security Label Registry Browser
//!
//! Provides the interactive terminal UI for the `umrs-label` binary.
//! When stdout is a TTY and `--cli` is not specified, the binary launches
//! the label registry browser instead of the text listing mode.
//!
//! ## Sub-modules
//!
//! - [`app`] — [`LabelRegistryApp`] struct, tree builder, and
//!   [`MarkingDetailData`] construction from catalog [`Marking`] entries.
//! - [`render`] — [`render_label_registry`] custom renderer that composes
//!   the security posture header, wizard logo, tree/detail split, search
//!   bar, and status bar into a full-screen TUI frame.
//!
//! ## Layout
//!
//! ```text
//! ┌─ Security posture header (host, OS, SELinux, FIPS) ──────┬─ wizard ─┐
//! │                                                          │  logo    │
//! ├── Catalog info row (placeholder) ───────────────────────┴──────────┤
//! ├──────────────────────────────────┬────────────────────────────────  ┤
//! │ Tree (≈40%)                      │ Details (≈60%)                  │
//! │                                  │                                  │
//! │ ▶ United States CUI              │ CUI//ADJ                         │
//! │   ● CUI - Controlled Unclass... │ Name : Status Adjustment         │
//! │ ▼ Group: Immigration             │ Nom  : Rajustement de statut     │
//! │   ● CUI//ADJ - Status Adj...    │ ...                              │
//! ├──────────────────────────────────┴──────────────────────────────────┤
//! │ [search bar — when active]                                          │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │ Security Label Registry | 153 markings | ↑↓:nav Enter:show q:quit  │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — the registry browser
//!   renders all CUI marking fields accurately and completely.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — the security posture
//!   header carries hostname, OS, SELinux mode, and FIPS state on every frame.
//! - **NIST SP 800-53 AC-3**: The registry is unconditionally read-only;
//!   no catalog mutation is possible through the browser interface.
//! - **NSA RTB RAIN**: Non-bypassable read-only contract — no write paths
//!   exist in the viewer or render modules.

pub mod app;
pub mod render;
