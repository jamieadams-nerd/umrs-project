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

use crate::app::{IndicatorValue, StatusLevel, StyleHint};

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

    /// Value column in data rows (white, no bold).
    pub data_value: Style,

    /// Header report name (bold bright white).
    pub header_name: Style,

    /// Header sub-fields (cyan).
    pub header_field: Style,

    /// Wizard logo lines (green).
    pub wizard: Style,

    /// Status bar text (bold white on colored background).
    pub status_text: Style,

    /// Indicator badge style for `IndicatorValue::Enabled` (green, bold).
    pub indicator_active: Style,

    /// Indicator badge style for `IndicatorValue::Disabled` (dark gray).
    pub indicator_inactive: Style,

    /// Indicator badge style for `IndicatorValue::Unavailable` (yellow).
    ///
    /// Yellow signals that the kernel source could not be read — the probe
    /// failed rather than returning a known-disabled state. This is visually
    /// distinct from `indicator_inactive` (dark gray) so that operators can
    /// immediately distinguish "explicitly disabled" from "could not determine".
    ///
    /// NIST SP 800-53 CA-7 — a failed probe must be distinguishable from a
    /// known-disabled feature during continuous monitoring.
    pub indicator_unavailable: Style,

    /// Group title style in the data panel (bold white).
    ///
    /// Group titles are visual organizers that mark the start of a named
    /// section in the data panel. Bold white makes them stand out from
    /// dim-cyan key labels while remaining unobtrusive.
    ///
    /// NIST SP 800-53 AU-3 — labelled sections improve audit record
    /// readability; an assessor can locate assessment objects by group.
    pub group_title: Style,

    // -----------------------------------------------------------------------
    // Dialog styles
    // -----------------------------------------------------------------------
    /// Border style for `Info` and `Confirm` dialogs (cyan).
    ///
    /// NIST SP 800-53 AU-3 — visually distinct dialog modes reduce operator
    /// error when interpreting dialog severity.
    pub dialog_info_border: Style,

    /// Border style for `Error` dialogs (red).
    ///
    /// NIST SP 800-53 AU-3 — error dialogs must be visually distinguishable
    /// from informational dialogs without relying solely on text.
    pub dialog_error_border: Style,

    /// Border style for `SecurityWarning` dialogs (yellow).
    ///
    /// Yellow signals a security-relevant warning — distinct from error (red)
    /// and informational (cyan). Operators must make a deliberate choice before
    /// confirming; the yellow border reinforces heightened attention.
    ///
    /// NIST SP 800-53 SC-5 — visual distinction reinforces the fail-safe
    /// default (Cancel) by signaling that this dialog requires care.
    pub dialog_security_border: Style,

    /// Style for the currently focused dialog button (bold cyan on black).
    ///
    /// NIST SP 800-53 SC-5, SI-10 — focused button must be unambiguously
    /// distinguishable from the unfocused button.
    pub dialog_button_focused: Style,

    /// Style for the unfocused dialog button (dim gray).
    pub dialog_button_unfocused: Style,

    /// Style for dialog title text (bold white).
    pub dialog_title: Style,

    /// Style for dialog message body text (white).
    pub dialog_message: Style,
}

impl Theme {
    /// Return the appropriate indicator badge style for the given `IndicatorValue`.
    ///
    /// Maps `Enabled` → `indicator_active`, `Disabled` → `indicator_inactive`,
    /// `Unavailable` → `indicator_unavailable`.
    ///
    /// NIST SP 800-53 AU-3 — security state must be visually unambiguous;
    /// enabled, disabled, and unavailable are rendered with distinct styles.
    #[must_use = "indicator style is used for rendering; discarding it has no effect"]
    pub const fn indicator_style(&self, value: &IndicatorValue) -> Style {
        match value {
            IndicatorValue::Enabled(_) => self.indicator_active,
            IndicatorValue::Disabled(_) => self.indicator_inactive,
            IndicatorValue::Unavailable => self.indicator_unavailable,
        }
    }
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
            data_value: Style::default().fg(Color::White),
            header_name: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            header_field: Style::default().fg(Color::Cyan),
            wizard: Style::default().fg(Color::Green),
            status_text: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            indicator_active: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            indicator_inactive: Style::default().fg(Color::DarkGray),
            indicator_unavailable: Style::default().fg(Color::Yellow),
            group_title: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            // Dialog styles
            dialog_info_border: Style::default().fg(Color::Cyan),
            dialog_error_border: Style::default().fg(Color::Red),
            dialog_security_border: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            dialog_button_focused: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            dialog_button_unfocused: Style::default().fg(Color::DarkGray),
            dialog_title: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            dialog_message: Style::default().fg(Color::White),
        }
    }
}
