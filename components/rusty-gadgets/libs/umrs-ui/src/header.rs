// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Header — Two-Column Audit Card Header Panel
//!
//! Renders the audit card header panel with a two-column layout presenting
//! system identification and live security posture indicators aligned to
//! OSCAL / SP 800-53A assessment terminology.
//!
//! ## Layout (4 fixed rows + optional supplemental fields)
//!
//! ```text
//!  Assessment : OS Detection / Platform Identity and Integrity
//!  Host       : goldeneye                  Tool       : umrs-ui 0.1.0
//!  OS         : RHEL 10.0 (aarch64)        Assessed   : 2026-03-15 19:31:54 UTC
//!  SELinux    : enforcing                  FIPS       : active
//!  Version    : 0.3.1                                              ← supplemental
//!  …                                                               ← truncation marker
//! ```
//!
//! Row 1 is a full-width single-column line combining the report name and
//! subject as `"<report_name> / <report_subject>"`. Rows 2–3 are two-column
//! identification lines. Row 4 carries security indicator values from
//! `ctx.indicators`, styled by `theme.indicator_style()`.
//!
//! Supplemental fields supplied by [`AuditCardApp::header_fields`] are appended
//! after the four fixed rows. If the available interior height is exhausted before
//! all supplemental fields are rendered, a `…` truncation marker replaces the
//! last visible line so that an operator can see that fields were omitted.
//!
//! ## Width-Aware Fallback
//!
//! When the terminal is narrower than `TWO_COL_MIN_WIDTH` columns, the right
//! column is omitted and the header renders left-column-only. This ensures the
//! minimum viable header is always present, regardless of terminal size.
//!
//! ## Trust Note
//!
//! All values in this header are display-only. Hostname and OS name come from
//! `uname(2)` and the OS detection pipeline respectively; they are not
//! trust-relevant assertions. Security indicator values originate from
//! `SecureReader`-verified kattr reads, but they are rendered here for operator
//! awareness — they do not affect any policy or access control decision made
//! by this library.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit record content — the header identifies the
//!   assessment type, host, OS, tool, and collection time on every frame.
//! - **NIST SP 800-53 CA-7**: Continuous monitoring — the `Assessed` field
//!   timestamps each collection event for monitoring record dating.
//! - **NIST SP 800-53 SA-11**: Developer testing — the `Tool` field provides
//!   traceability to the specific tool version that collected evidence.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — security
//!   indicator values originate from provenance-verified kattr reads.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use umrs_core::i18n;

use crate::app::{AuditCardApp, HeaderContext, HeaderField, IndicatorValue, StyleHint};
use crate::theme::{Theme, style_hint_color};

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Minimum terminal width (in columns) to enable the two-column layout.
///
/// Below this threshold, only the left column is rendered. The value is
/// chosen so that a typical 80-column terminal gets the full layout while
/// very narrow terminals (xterm defaults, embedded consoles) get the
/// minimum-viable left column.
const TWO_COL_MIN_WIDTH: u16 = 90;

/// Width of the label field within each column pair.
///
/// `"Assessment"` is 10 chars — the widest label. Using 10 aligns the ` : `
/// separator consistently across all four rows.
const LABEL_WIDTH: usize = 10;

/// Fixed width for the left-column value field in two-column layout.
///
/// The left value is padded to this width so the right column always starts
/// at the same character position regardless of value length. The total left
/// half occupies 1 (left pad) + `LABEL_WIDTH` (10) + ` : ` (3) +
/// `LEFT_VALUE_WIDTH` (35) = 49 characters. This comfortably fits the
/// longest left value observed in practice: `"RHEL 10.0 (aarch64)"` (20 chars),
/// with room for translated labels that may be slightly longer.
///
/// If a value exceeds this width it overflows rather than being truncated —
/// truncating security data is worse than misalignment.
const LEFT_VALUE_WIDTH: usize = 35;

// ---------------------------------------------------------------------------
// Render entry point
// ---------------------------------------------------------------------------

/// Render the audit card header panel.
///
/// The header presents system identification and live kernel security
/// indicators in a two-column layout (when the terminal is wide enough)
/// or single-column layout (narrow terminals or constrained areas).
///
/// Fixed row order (4 rows):
/// 1. Assessment — full-width single-column: `"<report_name> / <report_subject>"`
/// 2. Host / Tool
/// 3. OS (formatted as `"{os_name} ({architecture})"`) / Assessed
/// 4. SELinux / FIPS (security indicators)
///
/// Row 4 carries the security indicator values from `ctx.indicators`.
/// All indicator values use full words ("enforcing", "active", "none")
/// styled by `theme.indicator_style()`.
///
/// After the 4 fixed rows, supplemental fields from [`AuditCardApp::header_fields`]
/// are appended one per line, up to the available interior height. If more fields
/// are provided than available rows, the last rendered line is replaced by a `…`
/// truncation marker so the operator knows fields were omitted.
///
/// NIST SP 800-53 AU-3 — every rendered frame carries full identification;
/// supplemental fields extend audit card identification for SP 800-53A Examine objects.
pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    ctx: &HeaderContext,
    theme: &Theme,
) {
    let two_col = area.width >= TWO_COL_MIN_WIDTH;

    // Build the 4 fixed header lines.
    let mut lines = if two_col {
        build_two_column_lines(app, ctx, theme)
    } else {
        build_single_column_lines(app, ctx, theme)
    };

    // Append supplemental fields from app.header_fields().
    // Interior height = area height - 2 border rows.
    let interior_height = area.height.saturating_sub(2) as usize;
    let fixed_count = lines.len(); // 4 content + 2 blank = 6

    if interior_height > fixed_count {
        let available = interior_height - fixed_count;
        append_supplemental_fields(&mut lines, app.header_fields(), available, theme);
    }

    let title = format!(" {} ", app.card_title());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(title, theme.header_name));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------------
// Two-column layout builder
// ---------------------------------------------------------------------------

/// Build header lines using the two-column layout.
///
/// Produces exactly 4 lines:
/// - Row 1: full-width Assessment line combining report name and subject
/// - Row 2: Host (left) / Tool (right)
/// - Row 3: OS formatted as `"{os_name} ({architecture})"` (left) / Assessed (right)
/// - Row 4: SELinux (left) / FIPS (right) — both indicator values
///
/// Translated labels are stored as locals so the borrowed `&str` slices
/// remain valid for the duration of this function.
///
/// This function is public so that integration tests can inspect the built
/// `Line` objects without requiring a terminal backend.
#[doc(hidden)]
pub fn build_two_column_lines<'a>(
    app: &dyn AuditCardApp,
    ctx: &HeaderContext,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    // Translated labels — must be stored as locals because i18n::tr() returns
    // String; the helpers borrow &str slices with 'a lifetime.
    let lbl_assessment = i18n::tr("Assessment");
    let lbl_host = i18n::tr("Host");
    let lbl_tool = i18n::tr("Tool");
    let lbl_os = i18n::tr("OS");
    let lbl_assessed = i18n::tr("Assessed");
    let lbl_selinux = i18n::tr("SELinux");
    let lbl_fips = i18n::tr("FIPS");

    let tool_str = format!("{} {}", ctx.tool_name, ctx.tool_version);
    // Row 3: OS formatted as "{os_name} ({architecture})" — display-only, not
    // a trust-relevant assertion (see module doc).
    let os_str = format!("{} ({})", ctx.os_name, ctx.architecture);
    // Row 1: combine report_name and report_subject with " / " separator.
    let assessment_value = format!("{} / {}", app.report_name(), app.report_subject());

    vec![
        // Blank line before the Assessment row for visual breathing room.
        Line::from(""),
        // Row 1: full-width Assessment — single column, styled as header_name.
        single_col_line(&lbl_assessment, &assessment_value, theme.header_name, theme),
        // Blank line after the Assessment row separates it from the detail rows.
        Line::from(""),
        // Row 2: Host | Tool
        two_col_line(
            &lbl_host,
            // Display-only — not a trust-relevant assertion (see module doc).
            &ctx.hostname,
            theme.header_field,
            &lbl_tool,
            &tool_str,
            theme.header_field,
            theme,
        ),
        // Row 3: OS | Assessed
        two_col_line(
            &lbl_os,
            &os_str,
            theme.header_field,
            &lbl_assessed,
            &ctx.assessed_at,
            theme.header_field,
            theme,
        ),
        // Row 4: SELinux | FIPS (security indicators)
        two_col_indicator_line(
            &lbl_selinux,
            &ctx.indicators.selinux_status,
            &lbl_fips,
            &ctx.indicators.fips_mode,
            theme,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Single-column layout builder (narrow terminal fallback)
// ---------------------------------------------------------------------------

/// Build header lines using the single-column (left-only) layout.
///
/// Produces exactly 4 lines: Assessment, Host, OS, SELinux.
/// The right-column fields (Tool, Assessed, FIPS) are omitted when the
/// terminal is too narrow.
///
/// Translated labels are stored as locals (see `build_two_column_lines`
/// for the same rationale).
///
/// This function is public so that integration tests can inspect the built
/// `Line` objects without requiring a terminal backend.
#[doc(hidden)]
pub fn build_single_column_lines<'a>(
    app: &dyn AuditCardApp,
    ctx: &HeaderContext,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    let lbl_assessment = i18n::tr("Assessment");
    let lbl_host = i18n::tr("Host");
    let lbl_os = i18n::tr("OS");
    let lbl_selinux = i18n::tr("SELinux");

    let os_str = format!("{} ({})", ctx.os_name, ctx.architecture);
    let assessment_value = format!("{} / {}", app.report_name(), app.report_subject());

    vec![
        Line::from(""),
        single_col_line(&lbl_assessment, &assessment_value, theme.header_name, theme),
        Line::from(""),
        single_col_line(
            &lbl_host,
            // Display-only — not a trust-relevant assertion (see module doc).
            &ctx.hostname,
            theme.header_field,
            theme,
        ),
        single_col_line(&lbl_os, &os_str, theme.header_field, theme),
        single_col_indicator_line(&lbl_selinux, &ctx.indicators.selinux_status, theme),
    ]
}

// ---------------------------------------------------------------------------
// Line builders (private helpers)
// ---------------------------------------------------------------------------

/// Build a two-column line with independently styled values.
///
/// Left pair: `left_label : left_value` styled with `left_value_style`.
/// Right pair: `right_label : right_value` styled with `right_value_style`.
#[allow(clippy::too_many_arguments)] // lint does not fire at 7 args (threshold is 7); keep allow to document intent
fn two_col_line<'a>(
    left_label: &str,
    left_value: &str,
    left_value_style: ratatui::style::Style,
    right_label: &str,
    right_value: &str,
    right_value_style: ratatui::style::Style,
    theme: &'a Theme,
) -> Line<'a> {
    let left_label_str = format!(" {left_label:<LABEL_WIDTH$} : ");
    // Pad the left value to LEFT_VALUE_WIDTH so the right column always
    // starts at the same character position. Overflow (value longer than
    // LEFT_VALUE_WIDTH) is permitted — misalignment is preferable to
    // truncating security-relevant data.
    let left_value_str = format!("{left_value:<LEFT_VALUE_WIDTH$}");
    let right_label_str = format!("  {right_label:<LABEL_WIDTH$} : ");
    Line::from(vec![
        Span::styled(left_label_str, theme.header_field),
        Span::styled(left_value_str, left_value_style),
        Span::styled(right_label_str, theme.header_field),
        Span::styled(right_value.to_owned(), right_value_style),
    ])
}

/// Build a two-column line where both values are `IndicatorValue` types.
///
/// Each value is styled by `theme.indicator_style()` based on whether it
/// is `Active`, `Inactive`, or `Unavailable`. The display text for
/// `Unavailable` is `"unavailable"` — explicit, never empty.
fn two_col_indicator_line<'a>(
    left_label: &str,
    left_value: &IndicatorValue,
    right_label: &str,
    right_value: &IndicatorValue,
    theme: &'a Theme,
) -> Line<'a> {
    let left_label_str = format!(" {left_label:<LABEL_WIDTH$} : ");
    let right_label_str = format!("  {right_label:<LABEL_WIDTH$} : ");
    let left_text = indicator_text(left_value);
    let right_text = indicator_text(right_value);
    let left_style = theme.indicator_style(left_value);
    let right_style = theme.indicator_style(right_value);
    // Pad left value to LEFT_VALUE_WIDTH so the right column starts at the
    // same position as two_col_line rows. Without this, short indicator
    // values like "enforcing" would misalign the right column.
    let left_text_padded = format!("{left_text:<LEFT_VALUE_WIDTH$}");
    Line::from(vec![
        Span::styled(left_label_str, theme.header_field),
        Span::styled(left_text_padded, left_style),
        Span::styled(right_label_str, theme.header_field),
        Span::styled(right_text, right_style),
    ])
}

/// Build a single-column line with a styled value.
fn single_col_line<'a>(
    label: &str,
    value: &str,
    value_style: ratatui::style::Style,
    theme: &'a Theme,
) -> Line<'a> {
    let label_str = format!(" {label:<LABEL_WIDTH$} : ");
    Line::from(vec![
        Span::styled(label_str, theme.header_field),
        Span::styled(value.to_owned(), value_style),
    ])
}

/// Build a single-column line for an `IndicatorValue`.
fn single_col_indicator_line<'a>(
    label: &str,
    value: &IndicatorValue,
    theme: &'a Theme,
) -> Line<'a> {
    let label_str = format!(" {label:<LABEL_WIDTH$} : ");
    let text = indicator_text(value);
    let style = theme.indicator_style(value);
    Line::from(vec![
        Span::styled(label_str, theme.header_field),
        Span::styled(text, style),
    ])
}

// ---------------------------------------------------------------------------
// Indicator display helpers
// ---------------------------------------------------------------------------

/// Extract the display string for an `IndicatorValue`.
///
/// `Enabled(s)` and `Disabled(s)` return the inner string. `Unavailable`
/// returns `"unavailable"` — explicit, never empty or misleading.
fn indicator_text(value: &IndicatorValue) -> String {
    match value {
        IndicatorValue::Enabled(s) | IndicatorValue::Disabled(s) => s.clone(),
        IndicatorValue::Unavailable => "unavailable".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Supplemental field rendering
// ---------------------------------------------------------------------------

/// Append supplemental [`HeaderField`] lines to the header line list.
///
/// At most `available` lines are appended. If `fields` contains more entries
/// than `available` slots, the last rendered line is replaced by a `…`
/// truncation marker so the operator knows fields were omitted.
///
/// Each supplemental field is rendered using the same label+value layout as
/// the fixed rows: `"Label      : value"`. The value is styled according to
/// the field's `style_hint`.
///
/// Supplemental fields are rendered after the four fixed header rows
/// (Assessment, Host/Tool, OS/Assessed, SELinux/FIPS). They extend audit
/// card identification so each rendered card can serve as a standalone
/// SP 800-53A Examine object.
///
/// NIST SP 800-53 AU-3 — labelled supplemental fields ensure every audit card
/// carries sufficient identification for independent assessment.
fn append_supplemental_fields<'a>(
    lines: &mut Vec<Line<'a>>,
    fields: &[HeaderField],
    available: usize,
    theme: &'a Theme,
) {
    if fields.is_empty() || available == 0 {
        return;
    }

    // Determine how many fields we can render, reserving one slot for the
    // truncation marker if there are more fields than available slots.
    let truncated = fields.len() > available;
    let render_count = if truncated {
        available.saturating_sub(1)
    } else {
        fields.len()
    };

    for field in fields.iter().take(render_count) {
        lines.push(supplemental_field_line(field, theme));
    }

    if truncated {
        // Show a truncation marker so the operator knows fields were omitted.
        lines.push(Line::from(Span::styled(
            " …".to_owned(),
            theme.header_field,
        )));
    }
}

/// Build a single header line for a supplemental [`HeaderField`].
///
/// Uses the same `"Label      : value"` format as the fixed rows. The value
/// column is styled by the field's `style_hint`.
fn supplemental_field_line<'a>(field: &HeaderField, theme: &'a Theme) -> Line<'a> {
    let label_str = format!(" {:<LABEL_WIDTH$} : ", field.label);
    let value_color = style_hint_color(field.style_hint);
    let value_style = if field.style_hint == StyleHint::Normal {
        theme.header_field
    } else {
        ratatui::style::Style::default().fg(value_color)
    };
    Line::from(vec![
        Span::styled(label_str, theme.header_field),
        Span::styled(field.value.clone(), value_style),
    ])
}
