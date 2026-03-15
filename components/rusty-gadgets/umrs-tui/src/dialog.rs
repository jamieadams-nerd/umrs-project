// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Dialog — Modal Overlay API
//!
//! Provides a centered modal dialog overlay for audit card binaries.
//! Supports four modes: informational, error, security warning, and
//! confirmation. Two-button dialogs include focus management so the
//! caller can move focus with Tab/arrow keys before confirming.
//!
//! ## Visibility Model
//!
//! The dialog is visible when the caller holds a `Some(DialogState)`.
//! The caller passes `Option<&DialogState>` to [`render_dialog`]; when
//! `None`, the function is a no-op and nothing is rendered. There is no
//! separate `visible` flag — presence in the `Option` is the authoritative
//! visibility signal. The type system enforces this contract.
//!
//! ## Usage Pattern
//!
//! ```text
//! // Show a security warning:
//! let mut dialog: Option<DialogState> = Some(DialogState::security_warning("..."));
//!
//! // In the render closure:
//! render_dialog(frame, frame.area(), dialog.as_ref(), &theme);
//!
//! // After user acts (dialog.as_ref().unwrap().response is Some(...)):
//! // Log the interaction to journald, then clear:
//! dialog = None;
//! ```
//!
//! ## Audit Obligation
//!
//! The library has no logging dependency and cannot emit audit records.
//! Callers MUST emit a structured journald record when a `SecurityWarning`
//! or `Confirm` dialog produces a `Some(...)` response. See [`render_dialog`]
//! for the required record fields.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-10**: Input validation — dialog state validates
//!   operator intent before any action is taken.
//! - **NIST SP 800-53 AC-3**: Access enforcement — security-affecting actions
//!   require a distinct confirmation path (`SecurityWarning` mode).
//! - **NIST SP 800-53 SC-5**: Denial of service protection — fail-safe default
//!   focus (Cancel) prevents accidental confirmation under time pressure.
//! - **NIST SP 800-53 AU-2, AU-3**: Event logging — operator acknowledgement
//!   of a security warning is an auditable event; callers must log it.
//! - **NIST SP 800-53 AC-2**: Explicit session-state lifecycle; no implicit
//!   global modal state.

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::theme::Theme;

// ---------------------------------------------------------------------------
// DialogMode
// ---------------------------------------------------------------------------

/// Visual and interaction mode for a modal dialog.
///
/// Determines button labels, border styling, and initial focus behavior.
///
/// ## Mode characteristics
///
/// | Mode | Buttons | Default focus | Border color |
/// |---|---|---|---|
/// | Info | \[OK\] | Primary | info (blue) |
/// | Error | \[OK\] | Primary | error (red) |
/// | SecurityWarning | \[Cancel\] \[OK\] | **Secondary** (Cancel) | security (yellow) |
/// | Confirm | \[No\] \[Yes\] | **Secondary** (No) | info (blue) |
///
/// ## Fail-safe default for two-button modes
///
/// `SecurityWarning` and `Confirm` default to `DialogFocus::Secondary`
/// (Cancel / No). A reflexive Enter keypress on a DoD terminal will never
/// confirm a security-affecting action. The operator must make an affirmative
/// hand movement (Tab or arrow key) before pressing Enter to confirm.
///
/// ## Irreversible or destructive actions
///
/// For actions that are irreversible or affect classified data, consider
/// whether a text-entry confirmation ("type CONFIRM to proceed") is more
/// appropriate than this two-button dialog. The two-button design is
/// sufficient for reversible operations and advisory warnings. Phase 10
/// (Control Text Pop-Up) may require the text-entry pattern.
///
/// NIST SP 800-53 AC-3 — security-affecting actions require a distinct
/// confirmation path.
/// NIST SP 800-53 SC-5 — fail-safe default (Cancel) protects against
/// accidental confirmation under time pressure.
/// NIST SP 800-53 SI-10 — dialog state validates operator intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogMode {
    /// Informational message — single \[OK\] button to dismiss.
    ///
    /// Response is always `Some(true)` when dismissed.
    Info,

    /// Error condition — single \[OK\] button, error border styling.
    ///
    /// Response is always `Some(true)` when dismissed.
    Error,

    /// Security-serious warning — two-button (\[Cancel\] / \[OK\]) dialog.
    ///
    /// Default focus is `Secondary` (Cancel). Operator must actively move
    /// focus to \[OK\] before confirming. Use for any action that modifies
    /// security policy, changes enforcement mode, or is difficult to undo.
    ///
    /// NIST SP 800-53 SC-5 — fail-safe default prevents reflexive confirmation.
    SecurityWarning,

    /// Confirmation prompt — two-button (\[No\] / \[Yes\]) dialog.
    ///
    /// Default focus is `Secondary` (No). Use for operations that require
    /// explicit operator acknowledgement. For destructive or irreversible
    /// operations, consider `SecurityWarning` mode instead.
    Confirm,
}

// ---------------------------------------------------------------------------
// DialogFocus
// ---------------------------------------------------------------------------

/// Which button currently has focus in a two-button dialog.
///
/// `Primary` is the affirmative button (OK / Yes). `Secondary` is the
/// dismissal button (Cancel / No). Two-button dialogs (`SecurityWarning`,
/// `Confirm`) start with `Secondary` focus so a reflexive Enter keypress
/// never confirms a security-affecting action.
///
/// NIST SP 800-53 SC-5, SI-10 — conservative default prevents accidental
/// confirmation of security-relevant operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogFocus {
    /// The affirmative button (OK / Yes) has focus.
    Primary,
    /// The dismissal button (Cancel / No) has focus.
    Secondary,
}

impl DialogFocus {
    /// Toggle focus between `Primary` and `Secondary`.
    ///
    /// Used to implement Tab / Left / Right navigation within a two-button
    /// dialog. Calling `toggle()` on `Primary` returns `Secondary` and
    /// vice versa.
    #[must_use = "toggle() returns a new DialogFocus; the caller must apply it to dialog state"]
    pub const fn toggle(self) -> Self {
        match self {
            Self::Primary => Self::Secondary,
            Self::Secondary => Self::Primary,
        }
    }
}

// ---------------------------------------------------------------------------
// DialogState
// ---------------------------------------------------------------------------

/// Mutable state for a single active modal dialog.
///
/// Created by the caller when a dialog should appear. Destroyed (dropped)
/// by the caller after reading `response`. The library has zero authority
/// over dialog lifetime — there is no implicit global modal state.
///
/// Pass `Some(&state)` to [`render_dialog`] to render the dialog. Pass
/// `None` to suppress it. The `Option` wrapper is the authoritative
/// visibility signal; there is no separate `visible` field.
///
/// ## Response lifecycle
///
/// - `response == None` — dialog is pending; user has not acted yet.
/// - `response == Some(true)` — user confirmed (OK / Yes).
/// - `response == Some(false)` — user cancelled (Cancel / No / dismiss).
///
/// The calling binary sets `response` in its event loop when `DialogConfirm`
/// or `DialogCancel` actions are received from the keymap. The library does
/// not mutate `response` — only the render path reads it.
///
/// ## Audit obligation (SecurityWarning and Confirm modes)
///
/// When `response` transitions to `Some(...)`, the calling binary MUST emit
/// a structured journald record before clearing the dialog. Required fields:
/// - Dialog message or a stable identifier for it
/// - Outcome (`true` = confirmed, `false` = cancelled)
/// - Tool name (`env!("CARGO_PKG_NAME")`)
/// - Boot ID (from [`crate::app::HeaderContext::boot_id`]) for journald correlation
///
/// NIST SP 800-53 AU-2 — operator acknowledgement of a security warning
/// is an auditable event that must appear in the audit trail.
/// NIST SP 800-53 AU-3 — audit records must include who, what, and when.
///
/// NIST SP 800-53 SI-10 — dialog state is the validated gateway for
/// operator intent; callers must not bypass it.
/// NIST SP 800-53 AC-2 — explicit lifecycle; no hidden modal state.
/// NIST SP 800-53 AC-3 — security-affecting operations require a distinct
/// confirmation path (see `DialogMode::SecurityWarning`).
#[derive(Debug, Clone)]
pub struct DialogState {
    /// User response: `None` = pending, `Some(true)` = confirmed, `Some(false)` = cancelled.
    ///
    /// Set by the calling binary's event loop. Never set by the render path.
    pub response: Option<bool>,

    /// The message displayed to the operator inside the dialog box.
    ///
    /// Must not contain security labels, credentials, or classified data
    /// (NIST SP 800-53 SI-12). Keep concise — the dialog minimum width
    /// is 40 characters; messages longer than `area.width - 8` are clipped.
    pub message: String,

    /// The mode controlling button labels and visual styling.
    pub mode: DialogMode,

    /// Currently focused button (for two-button dialog modes).
    ///
    /// Irrelevant for single-button modes (`Info`, `Error`) — the sole
    /// button always has implicit focus. For two-button modes, the caller
    /// toggles this field in response to `DialogToggleFocus` actions.
    pub focused: DialogFocus,
}

impl DialogState {
    /// Construct an informational dialog with a single \[OK\] button.
    ///
    /// Default focus is `Primary` (the sole button). Response is `None` until
    /// the user dismisses with `DialogConfirm` or `DialogCancel`.
    ///
    /// NIST SP 800-53 AU-2 — informational dialogs do not require audit logging;
    /// the obligation applies only to `SecurityWarning` and `Confirm` modes.
    #[must_use = "DialogState::info() must be stored and passed to render_dialog(); \
                  discarding it silently suppresses the dialog"]
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            response: None,
            message: message.into(),
            mode: DialogMode::Info,
            focused: DialogFocus::Primary,
        }
    }

    /// Construct an error dialog with a single \[OK\] button and error styling.
    ///
    /// Default focus is `Primary` (the sole button). Response is `None` until
    /// the user dismisses.
    ///
    /// NIST SP 800-53 AU-2 — error dialogs do not require audit logging unless
    /// the error itself is a security-relevant event (log at the detection site).
    #[must_use = "DialogState::error() must be stored and passed to render_dialog(); \
                  discarding it silently suppresses the dialog"]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            response: None,
            message: message.into(),
            mode: DialogMode::Error,
            focused: DialogFocus::Primary,
        }
    }

    /// Construct a security warning dialog with \[Cancel\] / \[OK\] buttons.
    ///
    /// **Default focus is `Secondary` (Cancel).** A reflexive Enter keypress
    /// on a DoD terminal will never confirm a security-affecting action. The
    /// operator must Tab or arrow to \[OK\] before pressing Enter to confirm.
    ///
    /// When the response transitions to `Some(...)`, the calling binary MUST
    /// emit a structured journald record (see [`DialogState`] audit obligation).
    ///
    /// NIST SP 800-53 SC-5 — fail-safe default (Cancel) protects against
    /// accidental confirmation under time pressure.
    /// NIST SP 800-53 SI-10 — validates operator intent for security actions.
    /// NIST SP 800-53 AU-2, AU-3 — caller must log the acknowledgement.
    #[must_use = "DialogState::security_warning() must be stored and passed to render_dialog(); \
                  discarding it silently suppresses the security warning dialog"]
    pub fn security_warning(message: impl Into<String>) -> Self {
        Self {
            response: None,
            message: message.into(),
            mode: DialogMode::SecurityWarning,
            focused: DialogFocus::Secondary, // Cancel is the safe default
        }
    }

    /// Construct a confirmation dialog with \[No\] / \[Yes\] buttons.
    ///
    /// **Default focus is `Secondary` (No).** The operator must actively move
    /// focus to \[Yes\] before pressing Enter to confirm. Use for operations
    /// that require explicit acknowledgement but are not security-policy-affecting.
    /// For security-policy-affecting operations, prefer `security_warning()`.
    ///
    /// When the response transitions to `Some(...)`, the calling binary MUST
    /// emit a structured journald record (see [`DialogState`] audit obligation).
    ///
    /// NIST SP 800-53 SC-5, SI-10 — conservative default prevents accidental
    /// confirmation.
    /// NIST SP 800-53 AU-2, AU-3 — caller must log the acknowledgement.
    #[must_use = "DialogState::confirm() must be stored and passed to render_dialog(); \
                  discarding it silently suppresses the confirmation dialog"]
    pub fn confirm(message: impl Into<String>) -> Self {
        Self {
            response: None,
            message: message.into(),
            mode: DialogMode::Confirm,
            focused: DialogFocus::Secondary, // No is the safe default
        }
    }
}

// ---------------------------------------------------------------------------
// render_dialog
// ---------------------------------------------------------------------------

/// Render a modal dialog box centered in `area`.
///
/// When `state` is `None`, this function is a no-op — nothing is rendered.
/// Presence of `Some(&state)` is the sole visibility signal; there is no
/// separate `visible` field on [`DialogState`].
///
/// The dialog is rendered on top of existing content. Call this **after**
/// `render_audit_card()` so it overlays the card. Uses ratatui [`Clear`]
/// to blank the region before drawing — content behind the dialog is not
/// visible through the border.
///
/// ## Width calculation
///
/// ```text
/// dialog_width = message.len().max(40).min(area.width as usize - 8)
/// ```
///
/// The dialog is never narrower than 40 characters and never wider than
/// the available area minus 8 characters (4-character margin on each side).
///
/// ## Button layout
///
/// - `Info` / `Error`: single \[OK\] button centered below the message.
/// - `SecurityWarning`: \[Cancel\] on the left, \[OK\] on the right.
/// - `Confirm`: \[No\] on the left, \[Yes\] on the right.
///
/// The focused button is styled with `theme.dialog_button_focused`;
/// the unfocused button uses `theme.dialog_button_unfocused`.
///
/// ## No auto-dismiss
///
/// This function does not set any timer or auto-dismiss the dialog. The
/// operator must press a key that maps to `DialogConfirm` or `DialogCancel`.
/// This is intentional — NIST SP 800-53 AC-2 requires that session-state
/// changes be explicitly initiated by the operator, not triggered by timeouts.
///
/// ## Audit obligation (IMPORTANT — callers MUST read this)
///
/// This function cannot emit audit records (the library has no logging
/// dependency). Callers are responsible for emitting a structured journald
/// record when a `SecurityWarning` or `Confirm` dialog produces a
/// `Some(...)` response. The record MUST include:
///
/// - Dialog message or a stable identifier for it
/// - Outcome (`true` = confirmed, `false` = cancelled)
/// - Tool name (`env!("CARGO_PKG_NAME")`)
/// - Boot ID (from `HeaderContext::boot_id`) for journald session correlation
///
/// NIST SP 800-53 AU-2 — operator acknowledgement of a security warning
/// is an auditable event.
/// NIST SP 800-53 AU-3 — audit records must include event, outcome,
/// principal (tool), and session identifier (boot ID).
/// NIST SP 800-53 AC-2 — no auto-dismiss; explicit operator action required.
/// NIST SP 800-53 SI-10 — dialog validates operator intent before action.
/// NIST SP 800-53 SC-5 — fail-safe defaults for two-button modes.
/// NIST SP 800-53 AC-3 — security-affecting operations use `SecurityWarning`.
pub fn render_dialog(
    frame: &mut Frame,
    area: Rect,
    state: Option<&DialogState>,
    theme: &Theme,
) {
    let Some(state) = state else {
        return;
    };

    // --- Compute dialog dimensions ---
    // Maximum width is terminal width minus 4-char margin on each side (8 total).
    // `dialog_width` is bounded by `area.width - 8` which is at most u16::MAX,
    // so the conversion to u16 via `try_from` will always succeed in practice.
    // The `.unwrap_or` fallback clamps to the safe maximum if somehow exceeded.
    let max_width_u16 = area.width.saturating_sub(8);
    let max_width = max_width_u16 as usize;
    let dialog_width_usize = state.message.len().max(40).min(max_width.max(40));
    // The value is bounded by area.width (a u16), so the conversion is safe.
    let dialog_width =
        u16::try_from(dialog_width_usize).unwrap_or(max_width_u16);

    // Height: title (1) + top border (1) + message (1) + blank (1) + buttons
    // (1) + bottom border (1) = 6 lines total.
    let dialog_height: u16 = 6;

    // --- Center the dialog in `area` ---
    let dialog_rect = centered_rect(dialog_width, dialog_height, area);

    // --- Select border style by mode ---
    let border_style = match state.mode {
        DialogMode::Info | DialogMode::Confirm => theme.dialog_info_border,
        DialogMode::Error => theme.dialog_error_border,
        DialogMode::SecurityWarning => theme.dialog_security_border,
    };

    // --- Title by mode ---
    let title = match state.mode {
        DialogMode::Info => " Information ",
        DialogMode::Error => " Error ",
        DialogMode::SecurityWarning => " Security Warning ",
        DialogMode::Confirm => " Confirm ",
    };

    // --- Clear the region, then draw the border block ---
    frame.render_widget(Clear, dialog_rect);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Span::styled(title, theme.dialog_title))
        .title_alignment(Alignment::Center);

    // Inner area inside the block borders.
    let inner = block.inner(dialog_rect);
    frame.render_widget(block, dialog_rect);

    // --- Split inner area: message row + spacer + button row ---
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // message
            Constraint::Length(1), // spacer
            Constraint::Length(1), // buttons
        ])
        .split(inner);

    // --- Message line ---
    let msg_para = Paragraph::new(state.message.as_str())
        .style(theme.dialog_message)
        .alignment(Alignment::Center);
    frame.render_widget(msg_para, chunks[0]);

    // --- Button row ---
    render_buttons(frame, chunks[2], state, theme);
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Render the button row for the given dialog state.
fn render_buttons(
    frame: &mut Frame,
    area: Rect,
    state: &DialogState,
    theme: &Theme,
) {
    match state.mode {
        DialogMode::Info | DialogMode::Error => {
            // Single [OK] button, centered.
            let btn = Span::styled(" [OK] ", theme.dialog_button_focused);
            let para = Paragraph::new(Line::from(vec![btn]))
                .alignment(Alignment::Center);
            frame.render_widget(para, area);
        }
        DialogMode::SecurityWarning => {
            // [Cancel]   [OK] — left/right split.
            let (cancel_style, ok_style) = match state.focused {
                DialogFocus::Secondary => {
                    (theme.dialog_button_focused, theme.dialog_button_unfocused)
                }
                DialogFocus::Primary => {
                    (theme.dialog_button_unfocused, theme.dialog_button_focused)
                }
            };
            let line = Line::from(vec![
                Span::styled(" [Cancel] ", cancel_style),
                Span::raw("  "),
                Span::styled(" [OK] ", ok_style),
            ]);
            let para = Paragraph::new(line).alignment(Alignment::Center);
            frame.render_widget(para, area);
        }
        DialogMode::Confirm => {
            // [No]   [Yes] — left/right split.
            let (no_style, yes_style) = match state.focused {
                DialogFocus::Secondary => {
                    (theme.dialog_button_focused, theme.dialog_button_unfocused)
                }
                DialogFocus::Primary => {
                    (theme.dialog_button_unfocused, theme.dialog_button_focused)
                }
            };
            let line = Line::from(vec![
                Span::styled(" [No] ", no_style),
                Span::raw("  "),
                Span::styled(" [Yes] ", yes_style),
            ]);
            let para = Paragraph::new(line).alignment(Alignment::Center);
            frame.render_widget(para, area);
        }
    }
}

/// Compute a centered [`Rect`] of the given width and height inside `area`.
///
/// If the requested dimensions exceed the available space, the rect is
/// clamped to `area`. This is a pure geometry function — no rendering.
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let clamped_width = width.min(area.width);
    let clamped_height = height.min(area.height);

    let x = area.x.saturating_add(area.width.saturating_sub(clamped_width) / 2);
    let y =
        area.y.saturating_add(area.height.saturating_sub(clamped_height) / 2);

    Rect::new(x, y, clamped_width, clamped_height)
}
