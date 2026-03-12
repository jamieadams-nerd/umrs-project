// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # App — Audit Card Trait, State, and Supporting Types
//!
//! Defines the [`AuditCardApp`] trait that callers implement to feed data
//! into the audit card layout, plus [`AuditCardState`] which tracks mutable
//! UI state (active tab, scroll position, quit flag).
//!
//! ## Design
//!
//! The trait is object-safe: `render_audit_card` accepts `&dyn AuditCardApp`.
//! State is separate from data — the calling binary owns `AuditCardState`
//! and updates it in the event loop. The trait impl provides read-only data.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit record content — the trait requires a
//!   report name and subject, ensuring every card is self-identifying.
//! - **NSA RTB**: Security state is represented as typed enum variants
//!   (`StatusLevel`, `StyleHint`), never as raw strings.

use crate::keymap::Action;

// ---------------------------------------------------------------------------
// StyleHint
// ---------------------------------------------------------------------------

/// Visual emphasis hint for a data row value.
///
/// Maps to a foreground color in the theme. Callers use this to convey
/// semantic meaning (e.g., trust tier) without hard-coding color values.
///
/// NIST SP 800-53 AU-3 — security state is typed, not free-form strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleHint {
    /// Default foreground (white).
    Normal,

    /// Cyan highlight for attention.
    Highlight,

    /// Dimmed for secondary information.
    Dim,

    /// Green — trust verified / positive security outcome.
    TrustGreen,

    /// Yellow — trust degraded / advisory condition.
    TrustYellow,

    /// Red — trust failed / security concern.
    TrustRed,
}

// ---------------------------------------------------------------------------
// StatusLevel
// ---------------------------------------------------------------------------

/// Severity tier for the status bar.
///
/// Maps to a background color in the theme. Callers set the level to
/// communicate the current security posture at a glance.
///
/// NIST SP 800-53 AU-3, SI-5 — status must be visually unambiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    /// Informational — no action required.
    Info,

    /// Positive outcome — security posture is good.
    Ok,

    /// Advisory — degraded or uncertain state; review recommended.
    Warn,

    /// Error — security concern or pipeline failure; action required.
    Error,
}

// ---------------------------------------------------------------------------
// StatusMessage
// ---------------------------------------------------------------------------

/// A status bar message with an associated severity level.
///
/// The text is a short (≤80 char) summary suitable for single-line display.
/// Must not contain security labels, credentials, or classified data
/// (NIST SP 800-53 SI-12).
#[derive(Debug, Clone)]
pub struct StatusMessage {
    /// Severity tier controlling the background color.
    pub level: StatusLevel,

    /// Short display text. Must not contain sensitive data.
    pub text: String,
}

impl StatusMessage {
    /// Construct a new `StatusMessage`.
    #[must_use]
    pub fn new(level: StatusLevel, text: impl Into<String>) -> Self {
        Self {
            level,
            text: text.into(),
        }
    }
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self::new(StatusLevel::Info, "Ready")
    }
}

// ---------------------------------------------------------------------------
// TabDef
// ---------------------------------------------------------------------------

/// Definition of a single tab in the tab bar.
#[derive(Debug, Clone)]
pub struct TabDef {
    /// Display label shown in the tab bar.
    pub label: String,
}

impl TabDef {
    /// Construct a tab definition with the given label.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// DataRow
// ---------------------------------------------------------------------------

/// A single key-value row in the data panel.
///
/// The `style_hint` controls value color. Keys are always rendered in the
/// dim-cyan key style from the theme.
#[derive(Debug, Clone)]
pub struct DataRow {
    /// Left column: field name or label.
    pub key: String,

    /// Right column: field value or description.
    pub value: String,

    /// Visual hint applied to the value column.
    pub style_hint: StyleHint,
}

impl DataRow {
    /// Construct a data row with the given key, value, and style hint.
    #[must_use]
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        hint: StyleHint,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            style_hint: hint,
        }
    }

    /// Construct a data row with `Normal` style.
    #[must_use]
    pub fn normal(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(key, value, StyleHint::Normal)
    }

    /// Construct a blank separator row (empty key and value, Dim style).
    #[must_use]
    pub fn separator() -> Self {
        Self::new("", "", StyleHint::Dim)
    }
}

// ---------------------------------------------------------------------------
// AuditCardApp trait
// ---------------------------------------------------------------------------

/// Data provider trait for the audit card layout.
///
/// Implement this on your application data struct. The layout engine calls
/// these methods on every frame draw — implementations must be cheap (no I/O,
/// no locking, no expensive computation).
///
/// NIST SP 800-53 AU-3 — every audit card surface is self-identifying
/// via `report_name` and `report_subject`.
pub trait AuditCardApp {
    /// Short name of the report displayed in the header (e.g., "OS Detection").
    fn report_name(&self) -> &'static str;

    /// Card title displayed in the header border.
    ///
    /// Override to customize; the default returns `"UMRS Audit Card"`.
    /// Returns `String` (not `&'static str`) so that binaries can supply
    /// a translated string via `i18n::tr()`.
    fn card_title(&self) -> String {
        "UMRS Audit Card".to_owned()
    }

    /// Subject of the report (e.g., a hostname, file path, or component name).
    fn report_subject(&self) -> &'static str;

    /// Ordered list of tab definitions. Must not be empty.
    fn tabs(&self) -> &[TabDef];

    /// Index of the currently active tab (0-based).
    ///
    /// The value returned here is informational for the header; the authoritative
    /// active tab index is held in [`AuditCardState::active_tab`].
    fn active_tab(&self) -> usize;

    /// Data rows to display for the given tab index.
    ///
    /// Called on every draw. Must be cheap — do not perform I/O here.
    fn data_rows(&self, tab_index: usize) -> Vec<DataRow>;

    /// Current status message to display in the status bar.
    fn status(&self) -> &StatusMessage;
}

// ---------------------------------------------------------------------------
// AuditCardState
// ---------------------------------------------------------------------------

/// Mutable UI state for the audit card event loop.
///
/// Owned by the calling binary. Updated via [`handle_action`] in the event loop.
/// Passed alongside the immutable `AuditCardApp` impl to the render function.
///
/// NIST SP 800-53 AC-2 — `should_quit` drives clean session termination.
pub struct AuditCardState {
    /// Index of the currently displayed tab (0-based).
    pub active_tab: usize,

    /// Total number of tabs (set at construction; does not change).
    tab_count: usize,

    /// Current vertical scroll offset into the data panel.
    pub scroll_offset: usize,

    /// Set to `true` by [`Action::Quit`]; the event loop exits when this is `true`.
    pub should_quit: bool,
}

impl AuditCardState {
    /// Construct a new state for a card with `tab_count` tabs.
    ///
    /// `tab_count` must be at least 1. If 0 is passed it is clamped to 1.
    #[must_use]
    pub fn new(tab_count: usize) -> Self {
        Self {
            active_tab: 0,
            tab_count: tab_count.max(1),
            scroll_offset: 0,
            should_quit: false,
        }
    }

    /// Update state in response to an [`Action`].
    ///
    /// This is the single entry point for all keyboard-driven state changes.
    /// Call once per key event in the event loop.
    pub const fn handle_action(&mut self, action: &Action) {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::NextTab => {
                // Saturating modulo — wraps to 0 after the last tab.
                self.active_tab =
                    self.active_tab.saturating_add(1) % self.tab_count;
                self.scroll_offset = 0;
            }
            Action::PrevTab => {
                if self.active_tab == 0 {
                    self.active_tab = self.tab_count.saturating_sub(1);
                } else {
                    self.active_tab = self.active_tab.saturating_sub(1);
                }
                self.scroll_offset = 0;
            }
            Action::ScrollDown => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
            }
            Action::ScrollUp => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
            Action::PageDown => {
                self.scroll_offset = self.scroll_offset.saturating_add(10);
            }
            Action::PageUp => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
            }
            Action::Refresh => {
                // Refresh is application-defined; state itself has no refresh
                // behavior. Callers may use this signal to re-run detection.
            }
        }
    }

    /// Reset scroll offset to zero (e.g., after a data refresh).
    pub const fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
    }
}
