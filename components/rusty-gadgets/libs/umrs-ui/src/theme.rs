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
//! Three constructors are provided:
//!
//! - [`Theme::dark()`] — high-contrast palette for dark terminal backgrounds.
//! - [`Theme::light()`] — placeholder; currently returns the dark palette.
//! - [`Theme::no_color()`] — no ANSI color codes; honors the `NO_COLOR`
//!   environment variable ([`https://no-color.org/`]). Text modifiers
//!   (bold, dim, reversed) are retained for structural readability.
//!
//! Call sites MUST select the theme based on the `NO_COLOR` environment
//! variable before constructing the terminal:
//!
//! ```rust,ignore
//! let theme = if std::env::var_os("NO_COLOR").is_some() {
//!     Theme::no_color()
//! } else {
//!     Theme::dark()
//! };
//! ```
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Security state (trust level, status) must be
//!   visually unambiguous. Color choices map directly to severity tiers.
//! - **NIST SP 800-53 SI-11**: Error and status information must remain
//!   meaningful when color is unavailable; structural modifiers carry the
//!   semantic hierarchy in `no_color` mode.
//! - **WCAG 1.4.1**: Information must not be conveyed by color alone.
//!   Text-symbol prefixes (✓ / ✗) provide color-independent semantics
//!   for indicator values; `no_color` mode is the proof that the design
//!   satisfies this criterion.

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
/// ## Fields:
///
/// *Core layout*
///
/// - `border` — outer border style (cyan, dim).
/// - `tab_active` — active tab highlight style (cyan bold).
/// - `tab_inactive` — inactive tab style (dim).
/// - `data_key` — key column in data rows (dim cyan).
/// - `data_value` — value column in data rows (white, no bold).
/// - `header_name` — header report name (bold bright white).
/// - `header_field` — header sub-fields (cyan).
/// - `wizard` — wizard logo lines (green).
/// - `status_text` — status bar text (bold white on colored background).
///
/// *Indicator badges*
///
/// - `indicator_active` — badge style for `IndicatorValue::Enabled` (green, bold).
/// - `indicator_inactive` — badge style for `IndicatorValue::Disabled` (dark gray).
/// - `indicator_unavailable` — badge style for `IndicatorValue::Unavailable` (yellow). Yellow
///   signals a failed probe rather than a known-disabled state; visually distinct from
///   `indicator_inactive` (dark gray) so operators can immediately distinguish "explicitly
///   disabled" from "could not determine". NIST SP 800-53 CA-7.
///
/// *List and panel*
///
/// - `list_selection` — selected-row highlight in list widgets; a subtle warm highlight
///   (black-on-light-yellow by default), distinct from the cyan of active tabs so the cursor
///   position never gets confused with a selected tab. Every UMRS TUI tool reads from this
///   single field, so a palette change propagates everywhere at once.
///   NIST SP 800-53 AU-3.
/// - `group_title` — group title style in the data panel (bold white). Group titles mark the
///   start of a named section; bold white makes them stand out from dim-cyan key labels while
///   remaining unobtrusive. NIST SP 800-53 AU-3.
///
/// *Dialog styles*
///
/// - `dialog_info_border` — border style for `Info` and `Confirm` dialogs (cyan).
///   NIST SP 800-53 AU-3.
/// - `dialog_error_border` — border style for `Error` dialogs (red).
///   NIST SP 800-53 AU-3.
/// - `dialog_security_border` — border style for `SecurityWarning` dialogs (yellow). Yellow
///   signals a security-relevant warning; operators must make a deliberate choice before
///   confirming; the yellow border reinforces heightened attention. NIST SP 800-53 SC-5.
/// - `dialog_button_focused` — style for the currently focused dialog button (bold cyan on
///   black). NIST SP 800-53 SC-5, SI-10.
/// - `dialog_button_unfocused` — style for the unfocused dialog button (dim gray).
/// - `dialog_title` — style for dialog title text (bold white).
/// - `dialog_message` — style for dialog message body text (white).
/// - `dialog_detail` — style for detail text inside dialogs (e.g., permission-denied path);
///   lighter gray than `data_value` so secondary dialog text recedes without disappearing.
///   NIST SP 800-53 SI-11.
///
/// *SELinux group-header palette*
///
/// - `selinux_type_bg` — background color for the SELinux type block in group-header rows.
///   Dark terminals: dark navy (`#2D3A4A`); light terminals: pale slate (`#C8D0D8`); no-color:
///   `Color::Reset`. NIST SP 800-53 AC-4.
/// - `selinux_marking_bg` — background color for the SELinux marking block in group-header rows;
///   one shade lighter than `selinux_type_bg` so the two blocks read as related but distinct.
///   Dark terminals: `#3D4A5A`; light terminals: `#D8E0E8`; no-color: `Color::Reset`.
///   NIST SP 800-53 AC-4.
///
/// *Miscellaneous render styles*
///
/// - `restricted_hint` — style for `<restricted>` group-header rows and cuddled-base summaries;
///   dark gray + italic communicates "present but hidden". NIST SP 800-53 AC-3.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Consistent visual language for security state.
#[derive(Debug, Clone)]
pub struct Theme {
    pub border: Style,
    pub tab_active: Style,
    pub tab_inactive: Style,
    pub data_key: Style,
    pub data_value: Style,
    pub header_name: Style,
    pub header_field: Style,
    pub wizard: Style,
    pub status_text: Style,
    pub indicator_active: Style,
    pub indicator_inactive: Style,
    pub indicator_unavailable: Style,
    pub list_selection: Style,
    pub group_title: Style,

    // -----------------------------------------------------------------------
    // Dialog styles
    // -----------------------------------------------------------------------
    pub dialog_info_border: Style,
    pub dialog_error_border: Style,
    pub dialog_security_border: Style,
    pub dialog_button_focused: Style,
    pub dialog_button_unfocused: Style,
    pub dialog_title: Style,
    pub dialog_message: Style,

    // -----------------------------------------------------------------------
    // SELinux group-header palette
    // -----------------------------------------------------------------------
    pub selinux_type_bg: Color,
    pub selinux_marking_bg: Color,

    // -----------------------------------------------------------------------
    // Miscellaneous render styles
    // -----------------------------------------------------------------------
    pub restricted_hint: Style,
    pub dialog_detail: Style,
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

    /// Dark theme — the default UMRS palette.
    ///
    /// High-contrast on a dark terminal background: cyan borders, white
    /// value text, green wizard, warm-yellow list selection.
    ///
    /// Callers should prefer `Theme::dark()` / `Theme::light()` over
    /// `Theme::default()` when they have a preference — `Default` remains
    /// backwards compatible and currently aliases to `dark()`.
    #[must_use = "theme is consumed by render functions"]
    pub fn dark() -> Self {
        Self::default_dark()
    }

    /// Light theme — stub for future light-mode support.
    ///
    /// Currently returns the dark palette unchanged as a placeholder.
    /// When light mode is implemented, every style in this method must be
    /// re-tuned for a light terminal background (dark text on light bg,
    /// muted cyan → navy, light-yellow → amber, etc.).
    ///
    /// The stub exists so that call sites can already be written as
    /// `Theme::light()` and will inherit the new palette automatically.
    #[must_use = "theme is consumed by render functions"]
    pub fn light() -> Self {
        // TODO(theming): implement a real light palette.  Placeholder
        // returns the dark theme so the API is stable today.
        Self::default_dark()
    }

    /// No-color theme — honors the `NO_COLOR` environment variable.
    ///
    /// Every style field uses only text modifiers (bold, dim, reversed) and
    /// carries **no foreground or background color**. The terminal produces
    /// no ANSI color-select sequences when ratatui renders any widget styled
    /// with this theme.
    ///
    /// Text modifiers are retained deliberately:
    /// - `BOLD` preserves heading hierarchy.
    /// - `DIM` preserves the visual distinction between key and value columns.
    /// - `REVERSED` on `list_selection` inverts the terminal's own text and
    ///   background colors, which are host-controlled and contain no ANSI
    ///   color code from this crate.
    ///
    /// Dialog border styles that previously distinguished severity by color
    /// (info = cyan, error = red, warning = yellow) all collapse to the same
    /// neutral style.  Callers that need severity discrimination in no-color
    /// mode must use textual labels in dialog titles.
    ///
    /// ## Compliance
    ///
    /// - **NIST SP 800-53 SI-11**: Meaningful output in environments where
    ///   ANSI color is disabled (e.g., audit log pipelines, screen readers,
    ///   legacy terminals). Structural modifiers carry semantic hierarchy.
    /// - **WCAG 1.4.1**: Information must not be conveyed by color alone.
    ///   This theme enforces that requirement at the palette layer.
    #[must_use = "theme is consumed by render functions"]
    pub fn no_color() -> Self {
        let plain = Style::default();
        let bold = Style::default().add_modifier(Modifier::BOLD);
        let dim = Style::default().add_modifier(Modifier::DIM);
        let bold_dim = Style::default().add_modifier(Modifier::BOLD | Modifier::DIM);
        let reversed = Style::default().add_modifier(Modifier::REVERSED);

        Self {
            border: plain,
            tab_active: bold,
            tab_inactive: dim,
            data_key: dim,
            data_value: plain,
            header_name: bold,
            header_field: plain,
            wizard: plain,
            status_text: bold,
            indicator_active: bold,
            indicator_inactive: dim,
            indicator_unavailable: bold_dim,
            list_selection: reversed,
            group_title: bold,
            // All dialog border severities collapse to the same neutral style
            // in no-color mode.  Dialog titles must carry textual severity labels.
            dialog_info_border: plain,
            dialog_error_border: bold,
            dialog_security_border: bold,
            dialog_button_focused: reversed,
            dialog_button_unfocused: dim,
            dialog_title: bold,
            dialog_message: plain,
            // No-color: strip all background tints from group headers.
            selinux_type_bg: Color::Reset,
            selinux_marking_bg: Color::Reset,
            // Restricted rows: italic only, no color.
            restricted_hint: Style::default().add_modifier(Modifier::ITALIC),
            // Dialog detail: dim, no color.
            dialog_detail: dim,
        }
    }

    /// Internal constructor for the dark palette — shared by `Default`,
    /// `dark()`, and the current `light()` stub.
    fn default_dark() -> Self {
        Self {
            border: Style::default().fg(Color::Cyan),
            tab_active: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            tab_inactive: Style::default().fg(Color::DarkGray),
            data_key: Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
            data_value: Style::default().fg(Color::White),
            header_name: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            header_field: Style::default().fg(Color::Cyan),
            wizard: Style::default().fg(Color::Green),
            status_text: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            indicator_active: Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            indicator_inactive: Style::default().fg(Color::DarkGray),
            indicator_unavailable: Style::default().fg(Color::Yellow),
            // Dim aged-parchment background (RGB 160/145/95) — a
            // recessed warm tint on a dark terminal, distinctly "this
            // row is selected" without any of the glow of the ANSI
            // bright-yellow slot.  Black fg, no bold.
            list_selection: Style::default().fg(Color::Black).bg(Color::Rgb(160, 145, 95)),
            group_title: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            // Dialog styles
            dialog_info_border: Style::default().fg(Color::Cyan),
            dialog_error_border: Style::default().fg(Color::Red),
            dialog_security_border: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            dialog_button_focused: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            dialog_button_unfocused: Style::default().fg(Color::DarkGray),
            dialog_title: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            dialog_message: Style::default().fg(Color::White),
            // Dark navy / slate for the two-block group header banner.
            selinux_type_bg: Color::Rgb(0x2D, 0x3A, 0x4A),
            selinux_marking_bg: Color::Rgb(0x3D, 0x4A, 0x5A),
            restricted_hint: Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            // RGB(180,180,180) — lighter than DarkGray, darker than White.
            dialog_detail: Style::default().fg(Color::Rgb(180, 180, 180)),
        }
    }
}

impl Default for Theme {
    /// Default theme — aliases to [`Theme::dark`].
    ///
    /// Provided for backwards compatibility with call sites that use
    /// `Theme::default()`.  New code should call `Theme::dark()` or
    /// `Theme::light()` explicitly so the theming intent is visible.
    fn default() -> Self {
        Self::default_dark()
    }
}
