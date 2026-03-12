// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Theme — Color Scheme and Style Constants
//!
//! Centralizes all visual style definitions for the audit card layout.
//! A single `Theme` instance is constructed once and passed into all
//! rendering functions — callers never hard-code colors inline.
//!
//! The default theme uses a "high-tech" dark palette: cyan borders,
//! green wizard logo, bright key labels, and level-keyed status colors.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Security state (trust level, status) must be
//!   visually unambiguous. Color choices map directly to severity tiers.

use ratatui::style::{Color, Modifier, Style};

use crate::app::{StatusLevel, StyleHint};

// ---------------------------------------------------------------------------
// Trust level color helpers — imported from app to avoid circular deps
// ---------------------------------------------------------------------------

/// Map a [`StatusLevel`] to a terminal background color.
///
/// Colors are chosen to be unambiguous even on 256-color terminals:
/// - Info → dark blue
/// - Ok → dark green
/// - Warn → dark yellow/amber
/// - Error → dark red
///
/// NIST SP 800-53 AU-3 — status display must be unambiguous.
#[must_use = "color is used for rendering; discarding it has no effect"]
pub const fn status_bg_color(level: StatusLevel) -> Color {
    match level {
        StatusLevel::Info => Color::Blue,
        StatusLevel::Ok => Color::Green,
        StatusLevel::Warn => Color::Yellow,
        StatusLevel::Error => Color::Red,
    }
}

/// Map a [`StyleHint`] to a foreground [`Color`].
#[must_use = "color is used for rendering; discarding it has no effect"]
pub const fn style_hint_color(hint: StyleHint) -> Color {
    match hint {
        StyleHint::Normal => Color::White,
        StyleHint::Highlight => Color::Cyan,
        StyleHint::Dim => Color::DarkGray,
        StyleHint::TrustGreen => Color::Green,
        StyleHint::TrustYellow => Color::Yellow,
        StyleHint::TrustRed => Color::Red,
    }
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

/// Visual style definitions for every audit card element.
///
/// Construct once via `Theme::default()` and pass to all render functions.
/// Override individual fields to customise for a specific binary.
///
/// NIST SP 800-53 AU-3 — consistent visual language for security state.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Outer border style (cyan, dim).
    pub border: Style,

    /// Active tab highlight style (cyan bold).
    pub tab_active: Style,

    /// Inactive tab style (dim).
    pub tab_inactive: Style,

    /// Key column in data rows (dim cyan).
    pub data_key: Style,

    /// Value column in data rows (bright white).
    pub data_value: Style,

    /// Header report name (bold bright white).
    pub header_name: Style,

    /// Header sub-fields (cyan).
    pub header_field: Style,

    /// Wizard logo lines (green).
    pub wizard: Style,

    /// Status bar text (bold black on colored background).
    pub status_text: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            border: Style::default().fg(Color::Cyan),
            tab_active: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            tab_inactive: Style::default().fg(Color::DarkGray),
            data_key: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::DIM),
            data_value: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            header_name: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            header_field: Style::default().fg(Color::Cyan),
            wizard: Style::default().fg(Color::Green),
            status_text: Style::default()
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        }
    }
}
