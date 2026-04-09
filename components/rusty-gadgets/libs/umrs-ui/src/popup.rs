// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Popup — Reusable Overlay Frame and Content Renderers
//!
//! Provides a shared popup infrastructure for UMRS TUI tools.  Two concrete
//! popup renderers eliminate duplicate rendering code across `umrs-ls` and
//! any future tool that needs the same overlay patterns.
//!
//! ## Key Exported Types
//!
//! - [`PopupConfig`] — geometry specification for a centered popup overlay
//! - [`PopupCardData`] — trait for tabbed data sources (implemented by `FileStatApp`)
//! - [`render_popup_frame`] — compute popup rect, clear background, draw border
//! - [`render_marking_detail_popup`] — label detail overlay (replaces `render_label_popup`)
//! - [`render_audit_card_popup`] — file audit card with tab bar (replaces `render_stat_popup`)
//! - [`data_row_to_line`] — single [`DataRow`] → [`ratatui::text::Line`] conversion
//!
//! ## Architecture
//!
//! All rendering is decomposed into three layers:
//!
//! 1. **Frame** ([`render_popup_frame`]) — geometry, `Clear`, border, hint line.
//! 2. **Content** — content-type-specific: label lines or tabbed DataRow lists.
//! 3. **Data** — callers or trait impls supply pre-built data; no I/O here.
//!
//! This matches the umrs-ui layered separation rule and keeps the render path
//! free of I/O, validation, or allocation beyond ratatui `Line` construction.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Security attribute visibility — the marking detail
//!   popup renders the full label definition so operators can always inspect
//!   classification context without leaving the current view.
//! - **NIST SP 800-53 AC-16**: Security attribute display — label metadata is
//!   rendered with palette-coded styling that reinforces index-group identity.
//! - **NIST SP 800-53 AU-3**: Audit record content — the audit card popup surfaces
//!   complete identity and security metadata for every selected file node.
//! - **NIST SP 800-53 CA-7**: Continuous monitoring — security observations are
//!   accessible on demand without leaving the directory view.
//! - **NSA RTB**: Render path performs no I/O; all data is pre-built by callers.

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

use crate::app::DataRow;
use crate::marking_detail::{MarkingDetailData, build_detail_lines};
use crate::theme::{Theme, style_hint_color};

// ---------------------------------------------------------------------------
// PopupConfig
// ---------------------------------------------------------------------------

/// Geometry specification for a centered popup overlay.
///
/// Controls the popup's proportional size and clamped bounds.  Pass a value
/// of this type to [`render_popup_frame`] to compute and draw the frame.
///
/// ## Compliance
///
/// - **NSA RTB**: Bounds are expressed as clamped percentages; there is no
///   unchecked path that can produce a zero-size or out-of-bounds popup rect.
#[derive(Debug, Clone)]
#[must_use = "PopupConfig must be passed to render_popup_frame; discarding it skips all popup rendering"]
pub struct PopupConfig {
    /// Title text displayed centered in the top border.
    pub title: &'static str,
    /// Hint line text shown dim at the bottom of the popup content area.
    ///
    /// Typically a key legend such as `"[ESC] close"`.
    pub hint: &'static str,
    /// Target popup width as a fraction of the terminal width (0.0–1.0).
    pub width_pct: f32,
    /// Target popup height as a fraction of the terminal height (0.0–1.0).
    pub height_pct: f32,
    /// Minimum popup width in columns.
    pub min_width: u16,
    /// Maximum popup width in columns.
    pub max_width: u16,
    /// Minimum popup height in rows.
    pub min_height: u16,
    /// Maximum popup height in rows.
    pub max_height: u16,
}

// ---------------------------------------------------------------------------
// PopupCardData trait
// ---------------------------------------------------------------------------

/// Trait for types that can provide tabbed [`DataRow`] content for popup display.
///
/// Implement this trait on data structs (e.g., `FileStatApp`) to make them
/// renderable inside [`render_audit_card_popup`] without a hard dependency on
/// the concrete type in this crate.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Tabbed data display ensures every file identity
///   and security attribute is accessible without information truncation.
/// - **NIST SP 800-53 SI-10**: Bounds are enforced — `rows_for_tab` must
///   return an empty slice for out-of-range tab indices, never panic.
pub trait PopupCardData {
    /// Return the display name for each tab, in order.
    ///
    /// The returned slice length defines the tab count.
    fn tab_names(&self) -> &[&'static str];

    /// Return the rows for the given tab index.
    ///
    /// Out-of-range indices MUST return an empty slice — never panic.
    /// This is the fail-closed contract enforced by [`render_audit_card_popup`].
    ///
    /// NIST SP 800-53 SI-10 — bounds checking; no panic path.
    fn rows_for_tab(&self, tab: usize) -> &[DataRow];

    /// Return the security marking for the file, if any.
    ///
    /// Rendered in the upper-right corner of the tab bar line so the
    /// operator always sees the marking without switching to the Security tab.
    /// Return `None` when no marking is available.
    ///
    /// NIST SP 800-53 AC-16 — security attribute is always visible.
    fn marking(&self) -> Option<&str> {
        None
    }

    /// Return the index group for the file's marking, if any.
    ///
    /// Used to select the palette color for the marking pill in the popup
    /// tab bar. Must match the index group used in the directory listing
    /// so colors are consistent across tools.
    ///
    /// NIST SP 800-53 AC-16 — visual consistency reinforces marking identity.
    fn marking_index_group(&self) -> Option<&str> {
        None
    }

    /// Return the observation count for the file.
    ///
    /// When > 0, the Observations tab label is prefixed with a flag icon
    /// so the operator knows findings exist without switching tabs.
    ///
    /// NIST SP 800-53 CA-7 — visual indicator ensures findings are not overlooked.
    fn observation_count(&self) -> usize {
        0
    }
}

// ---------------------------------------------------------------------------
// render_popup_frame
// ---------------------------------------------------------------------------

/// Compute a centered popup rect, clear the background, draw the border, and
/// reserve the bottom hint row.
///
/// Returns the usable inner content area: the block's inner rect minus one row
/// at the bottom for the hint line.  Pass the returned rect to the content
/// renderer of your choice.
///
/// ## Centering math
///
/// ```text
/// raw_w = area.width  × width_pct  (truncated toward zero)
/// raw_h = area.height × height_pct (truncated toward zero)
/// popup_w = raw_w.clamp(min_width,  max_width)
/// popup_h = raw_h.clamp(min_height, max_height)
/// x = area.x + (area.width  − popup_w) / 2
/// y = area.y + (area.height − popup_h) / 2
/// ```
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Title text identifies the popup's purpose so
///   operators always know what they are looking at.
/// - **NSA RTB**: All dimension arithmetic uses saturating operations; no
///   overflow or zero-size rect is possible.
#[must_use = "render_popup_frame returns the inner content Rect; pass it to a content renderer"]
pub fn render_popup_frame(
    frame: &mut Frame,
    area: Rect,
    config: &PopupConfig,
    theme: &Theme,
) -> Rect {
    // Proportional sizing with clamp.
    // f32::from(u16) is lossless; the f32 → u16 cast truncates toward zero.
    // Both values are bounded by area dimensions (u16), so truncation is safe.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "popup dimension: f32 of (pct * u16) is bounded; truncation toward zero is intentional"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "pct is positive and u16 is non-negative; product is always non-negative"
    )]
    let raw_w = (f32::from(area.width) * config.width_pct) as u16;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "popup dimension: f32 of (pct * u16) is bounded; truncation toward zero is intentional"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "pct is positive and u16 is non-negative; product is always non-negative"
    )]
    let raw_h = (f32::from(area.height) * config.height_pct) as u16;

    let popup_width = raw_w.clamp(config.min_width, config.max_width);
    let popup_height = raw_h.clamp(config.min_height, config.max_height);

    let popup_rect = centered_rect(popup_width, popup_height, area);

    // Clear the screen area behind the popup.
    frame.render_widget(Clear, popup_rect);

    // Double border with cyan bold — visually distinct from the rounded panels
    // used in the main view, making the overlay immediately identifiable.
    let title_line = Line::from(Span::styled(
        format!(" {} ", config.title),
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
    ));
    let block = Block::default()
        .title(title_line)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let inner = block.inner(popup_rect);
    frame.render_widget(block, popup_rect);

    // Split inner area: content (fills) + hint line (1 row at bottom).
    let content_height = inner.height.saturating_sub(1);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(content_height), Constraint::Length(1)])
        .split(inner);

    let [content_area, hint_area] = *chunks else {
        // Defensive: if layout fails, return inner as-is.
        return inner;
    };

    // Render the hint text dim at the bottom of the popup.
    let hint_line = Line::from(Span::styled(
        config.hint,
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(Paragraph::new(hint_line).style(theme.data_value), hint_area);

    content_area
}

// ---------------------------------------------------------------------------
// render_marking_detail_popup
// ---------------------------------------------------------------------------

/// Render a security label detail popup centered in `area`.
///
/// This is the shared replacement for the `render_label_popup` function that
/// was previously duplicated in `umrs-ls`.  The popup is sized to 70% of the
/// terminal, clamped to 40–80 columns × 12–30 rows.
///
/// Content is built via [`build_detail_lines`] and rendered directly inside
/// the popup's Double border — no nested block.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — the full regulatory label
///   definition is accessible without leaving the directory view.
/// - **NIST SP 800-53 AU-3**: Every rendered panel is self-identifying via the
///   marking key header; no field is rendered without a label.
pub fn render_marking_detail_popup(
    frame: &mut Frame,
    area: Rect,
    data: &MarkingDetailData,
    scroll_offset: u16,
    theme: &Theme,
) {
    let config = PopupConfig {
        title: "SECURITY LABEL DETAILS",
        hint: "  [ESC] close",
        width_pct: 0.70,
        height_pct: 0.70,
        min_width: 40,
        max_width: 80,
        min_height: 12,
        max_height: 30,
    };

    let content_area = render_popup_frame(frame, area, &config, theme);

    let lines = build_detail_lines(data, theme, content_area.width as usize);
    let paragraph = Paragraph::new(lines).scroll((scroll_offset, 0));
    frame.render_widget(paragraph, content_area);
}

// ---------------------------------------------------------------------------
// render_audit_card_popup
// ---------------------------------------------------------------------------

/// Render a tabbed file security audit popup centered in `area`.
///
/// Displays a tab bar at the top followed by scrollable [`DataRow`] content
/// for the currently active tab.  The caller controls the active tab index
/// and per-tab scroll offsets via [`StatPopupState`]-equivalent state stored
/// in the calling binary.
///
/// The popup is sized to 70% of the terminal, clamped to 60–90 columns ×
/// 18–35 rows.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Tabbed layout ensures complete file identity and
///   security metadata is always accessible; no field is hidden.
/// - **NIST SP 800-53 CA-7**: Security observations are surfaced on demand for
///   every file node without leaving the directory view.
pub fn render_audit_card_popup(
    frame: &mut Frame,
    area: Rect,
    app: &dyn PopupCardData,
    active_tab: usize,
    scroll_offset: u16,
    theme: &Theme,
) {
    let config = PopupConfig {
        title: "FILE SECURITY AUDIT",
        hint: "  [ESC/q] close  [TAB/\u{2192}] next tab  [\u{2191}\u{2193}/j/k] scroll",
        width_pct: 0.70,
        height_pct: 0.70,
        min_width: 60,
        max_width: 90,
        min_height: 18,
        max_height: 35,
    };

    let content_area = render_popup_frame(frame, area, &config, theme);

    // Split content area: blank (1) + tab bar (1) + blank separator (1) + scrollable (rest).
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // blank line before tabs
            Constraint::Length(1), // tab bar
            Constraint::Length(1), // blank separator after tabs
            Constraint::Min(1),    // scrollable content
        ])
        .split(content_area);

    let [_, tab_bar_area, _, scroll_area] = *chunks else {
        return;
    };

    // ── Tab bar with optional marking in upper-right ────────────────────────
    let tab_names = app.tab_names();
    let tab_count = tab_names.len();
    let obs_count = app.observation_count();
    let mut tab_spans: Vec<Span<'_>> = tab_names
        .iter()
        .enumerate()
        .flat_map(|(i, name)| {
            // Prefix the Observations tab with a flag icon when findings exist.
            // NIST SP 800-53 CA-7 — visual cue ensures findings are not overlooked.
            let display_name = if *name == "Observations" && obs_count > 0 {
                format!(" [\u{2691} {name}] ")  // ⚑ flag
            } else {
                format!(" [{name}] ")
            };
            let style = if i == active_tab {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            if i < tab_count.saturating_sub(1) {
                vec![Span::styled(display_name, style), Span::raw(" ")]
            } else {
                vec![Span::styled(display_name, style)]
            }
        })
        .collect();

    // Right-align the security marking on the same line as the tabs.
    // Uses the same palette as the directory listing group headers so the
    // color is consistent across tools.
    // NIST SP 800-53 AC-16 — marking is always visible without switching tabs.
    if let Some(marking) = app.marking() {
        let tabs_width: usize = tab_spans.iter().map(|s| s.content.len()).sum();
        let marking_display = format!(" {marking} ");
        let marking_width = marking_display.len();
        let available = usize::from(tab_bar_area.width);
        let gap = available.saturating_sub(tabs_width).saturating_sub(marking_width);
        if gap > 0 {
            tab_spans.push(Span::raw(" ".repeat(gap)));
        }
        let group = app.marking_index_group().unwrap_or("");
        let marking_style = Style::default()
            .fg(crate::palette::palette_fg(group))
            .bg(crate::palette::palette_bg(group))
            .add_modifier(Modifier::BOLD);
        tab_spans.push(Span::styled(marking_display, marking_style));
    }

    frame.render_widget(Paragraph::new(Line::from(tab_spans)), tab_bar_area);

    // ── Scrollable content ─────────────────────────────────────────────────
    let rows = app.rows_for_tab(active_tab);
    // Compute key column width dynamically from the actual rows so that
    // padding aligns consistently regardless of key string lengths.
    let col_width: usize = rows
        .iter()
        .filter_map(|r| match r {
            DataRow::KeyValue {
                key,
                ..
            }
            | DataRow::IndicatorRow {
                key,
                ..
            } => Some(key.len()),
            _ => None,
        })
        .max()
        .unwrap_or(16);
    let val_width = usize::from(scroll_area.width).saturating_sub(col_width).saturating_sub(3);
    let lines: Vec<Line<'_>> =
        rows.iter().map(|row| data_row_to_line(row, col_width, val_width, theme)).collect();
    let content = Paragraph::new(lines).scroll((scroll_offset, 0));
    frame.render_widget(content, scroll_area);
}

// ---------------------------------------------------------------------------
// data_row_to_line
// ---------------------------------------------------------------------------

/// Convert a single [`DataRow`] into a ratatui [`Line`] for popup display.
///
/// This is the authoritative implementation of the DataRow → Line conversion
/// for popup contexts, using styling consistent with `data_panel.rs`.  It
/// eliminates the duplicate `stat_row_to_line` that previously existed in
/// `umrs-ls`.
///
/// ## Parameters
///
/// - `col_width` — key column padding in characters
/// - `val_width` — available value column width for truncation (0 = no truncation)
/// - `theme` — style source for key/value/group colors
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Every data row is rendered with a labelled key
///   column; no field is displayed without an identifier.
/// - **NIST SP 800-53 SI-10**: `val_width` truncation uses checked saturation;
///   zero val_width safely skips truncation without panic.
#[must_use = "data_row_to_line returns a styled Line; pass it to a Paragraph widget or collect it"]
pub fn data_row_to_line<'a>(
    row: &'a DataRow,
    col_width: usize,
    val_width: usize,
    theme: &Theme,
) -> Line<'a> {
    match row {
        DataRow::Separator => Line::from(""),

        DataRow::KeyValue {
            key,
            value,
            style_hint,
            ..
        } => {
            let key_style = theme.data_key;
            let val_style = theme.data_value.fg(style_hint_color(*style_hint));
            let key_padded = format!(" {key:<col_width$}");
            // Truncate value with ellipsis when it exceeds the available column.
            let value_display: String = if val_width > 0 && value.len() > val_width {
                format!("{}…", &value[..val_width.saturating_sub(1)])
            } else {
                value.clone()
            };
            Line::from(vec![
                Span::styled(key_padded, key_style),
                Span::raw(" : "),
                Span::styled(value_display, val_style),
            ])
        }

        DataRow::GroupTitle(title) => Line::from(Span::styled(
            format!(" {title}"),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),

        DataRow::TwoColumn {
            left_key,
            left_value,
            right_key,
            right_value,
            ..
        } => Line::from(vec![
            Span::styled(
                format!(" {left_key:<col_width$} : {left_value}"),
                theme.data_value,
            ),
            Span::raw("  "),
            Span::styled(format!("{right_key} : {right_value}"), theme.data_value),
        ]),

        DataRow::IndicatorRow {
            key,
            value,
            style_hint,
            ..
        } => Line::from(vec![
            Span::styled(format!(" {key:<col_width$}"), theme.data_key),
            Span::raw(" : "),
            Span::styled(
                value.as_str(),
                theme.data_value.fg(style_hint_color(*style_hint)),
            ),
        ]),

        DataRow::TableRow {
            col1,
            col2,
            col3,
            style_hint,
        } => {
            let col3_style = theme.data_value.fg(style_hint_color(*style_hint));
            Line::from(vec![
                Span::styled(format!(" {col1:<col_width$}  {col2}"), theme.data_value),
                Span::raw("  "),
                Span::styled(col3.clone(), col3_style),
            ])
        }

        DataRow::TableHeader {
            col1,
            col2,
            col3,
        } => Line::from(Span::styled(
            format!(" {col1:<col_width$}  {col2}  {col3}"),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
    }
}

// ---------------------------------------------------------------------------
// Geometry helper
// ---------------------------------------------------------------------------

/// Compute a centered [`Rect`] of the given width and height inside `area`.
///
/// Clamped to `area` dimensions — the rect never exceeds available space.
pub(crate) fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let clamped_width = width.min(area.width);
    let clamped_height = height.min(area.height);
    let x = area.x.saturating_add(area.width.saturating_sub(clamped_width) / 2);
    let y = area.y.saturating_add(area.height.saturating_sub(clamped_height) / 2);
    Rect::new(x, y, clamped_width, clamped_height)
}
