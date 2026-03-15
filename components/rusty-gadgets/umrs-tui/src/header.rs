// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Header — Two-Column Audit Card Header Panel
//!
//! Renders the audit card header panel with a two-column layout presenting
//! system identification and live security posture indicators aligned to
//! OSCAL / SP 800-53A assessment terminology.
//!
//! ## Layout (6 fixed rows + optional supplemental fields)
//!
//! ```text
//! Assessment : OS Detection               Boot ID   : a3f7c2d1-...
//! Scope      : Platform Identity          Assessed  : 2026-03-15 14:32 UTC
//! Host       : goldeneye                  Kernel    : 6.12.0-211.el10
//! Tool       : umrs-os-detect 0.3.1       System ID : 550e8400-...
//! SELinux    : enforcing                  LSM       : unavailable
//! FIPS       : active                     Lockdown  : none
//! Version    : 0.3.1                                              ← supplemental
//! …                                                               ← truncation marker
//! ```
//!
//! Supplemental fields supplied by [`AuditCardApp::header_fields`] are appended
//! after the six fixed rows. If the available interior height is exhausted before
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
//! All values in this header are display-only. Hostname and kernel version
//! come from `uname(2)` and are not trust-relevant assertions. Security
//! indicator values originate from `SecureReader`-verified kattr reads, but
//! they are rendered here for operator awareness — they do not affect any
//! policy or access control decision made by this library.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit record content — the header identifies the
//!   assessment type, scope, host, tool, and collection time on every frame.
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
/// separator consistently across all six rows.
const LABEL_WIDTH: usize = 10;

// ---------------------------------------------------------------------------
// Render entry point
// ---------------------------------------------------------------------------

/// Render the audit card header panel.
///
/// The header presents system identification and live kernel security
/// indicators in a two-column layout (when the terminal is wide enough)
/// or single-column layout (narrow terminals or constrained areas).
///
/// Fixed row order (6 rows):
/// 1. Assessment / Boot ID
/// 2. Scope / Assessed
/// 3. Host / Kernel
/// 4. Tool / System ID
/// 5. SELinux / LSM
/// 6. FIPS / Lockdown
///
/// Rows 5–6 carry the security indicator values from `ctx.indicators`.
/// All indicator values use full words ("enforcing", "active", "none")
/// styled by `theme.indicator_style()`.
///
/// After the 6 fixed rows, supplemental fields from [`AuditCardApp::header_fields`]
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

    // Build the 6 fixed header lines.
    let mut lines = if two_col {
        build_two_column_lines(app, ctx, theme)
    } else {
        build_single_column_lines(app, ctx, theme)
    };

    // Append supplemental fields from app.header_fields().
    // Interior height = area height - 2 border rows.
    let interior_height = area.height.saturating_sub(2) as usize;
    let fixed_count = lines.len(); // always 6

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
/// Each line contains a left pair (label + value) and a right pair
/// (label + value). Values are styled by their semantic meaning
/// (header field vs. indicator).
///
/// Translated labels are stored as locals so the borrowed `&str` slices
/// remain valid for the duration of this function.
fn build_two_column_lines<'a>(
    app: &dyn AuditCardApp,
    ctx: &HeaderContext,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    // Translated labels — must be stored as locals because i18n::tr() returns
    // String; the helpers borrow &str slices with 'a lifetime.
    let lbl_assessment = i18n::tr("Assessment");
    let lbl_boot_id = i18n::tr("Boot ID");
    let lbl_scope = i18n::tr("Scope");
    let lbl_assessed = i18n::tr("Assessed");
    let lbl_host = i18n::tr("Host");
    let lbl_kernel = i18n::tr("Kernel");
    let lbl_tool = i18n::tr("Tool");
    let lbl_system_id = i18n::tr("System ID");
    let lbl_selinux = i18n::tr("SELinux");
    let lbl_lsm = i18n::tr("LSM");
    let lbl_fips = i18n::tr("FIPS");
    let lbl_lockdown = i18n::tr("Lockdown");
    let tool_str = format!("{} {}", ctx.tool_name, ctx.tool_version);

    vec![
        // Row 1: Assessment | Boot ID
        two_col_line(
            &lbl_assessment,
            app.report_name(),
            theme.header_name,
            &lbl_boot_id,
            &ctx.boot_id,
            theme.header_field,
            theme,
        ),
        // Row 2: Scope | Assessed
        two_col_line(
            &lbl_scope,
            app.report_subject(),
            theme.header_name,
            &lbl_assessed,
            &ctx.assessed_at,
            theme.header_field,
            theme,
        ),
        // Row 3: Host | Kernel
        two_col_line(
            &lbl_host,
            // Display-only — not a trust-relevant assertion (see module doc).
            &ctx.hostname,
            theme.header_name,
            &lbl_kernel,
            &ctx.kernel_version,
            theme.header_field,
            theme,
        ),
        // Row 4: Tool | System ID
        two_col_line(
            &lbl_tool,
            &tool_str,
            theme.header_name,
            &lbl_system_id,
            &ctx.system_uuid,
            theme.header_field,
            theme,
        ),
        // Row 5: SELinux | LSM (security indicators)
        two_col_indicator_line(
            &lbl_selinux,
            &ctx.indicators.selinux_status,
            &lbl_lsm,
            &ctx.indicators.active_lsm,
            theme,
        ),
        // Row 6: FIPS | Lockdown (security indicators)
        two_col_indicator_line(
            &lbl_fips,
            &ctx.indicators.fips_mode,
            &lbl_lockdown,
            &ctx.indicators.lockdown_mode,
            theme,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Single-column layout builder (narrow terminal fallback)
// ---------------------------------------------------------------------------

/// Build header lines using the single-column (left-only) layout.
///
/// Renders only the left column: Assessment, Scope, Host, Tool, SELinux,
/// FIPS. The right-column fields (Boot ID, Assessed, Kernel, System ID,
/// LSM, Lockdown) are omitted when the terminal is too narrow.
///
/// Translated labels are stored as locals (see `build_two_column_lines`
/// for the same rationale).
fn build_single_column_lines<'a>(
    app: &dyn AuditCardApp,
    ctx: &HeaderContext,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    let lbl_assessment = i18n::tr("Assessment");
    let lbl_scope = i18n::tr("Scope");
    let lbl_host = i18n::tr("Host");
    let lbl_tool = i18n::tr("Tool");
    let lbl_selinux = i18n::tr("SELinux");
    let lbl_fips = i18n::tr("FIPS");
    let tool_str = format!("{} {}", ctx.tool_name, ctx.tool_version);

    vec![
        single_col_line(
            &lbl_assessment,
            app.report_name(),
            theme.header_name,
            theme,
        ),
        single_col_line(
            &lbl_scope,
            app.report_subject(),
            theme.header_name,
            theme,
        ),
        single_col_line(&lbl_host, &ctx.hostname, theme.header_name, theme),
        single_col_line(&lbl_tool, &tool_str, theme.header_name, theme),
        single_col_indicator_line(
            &lbl_selinux,
            &ctx.indicators.selinux_status,
            theme,
        ),
        single_col_indicator_line(&lbl_fips, &ctx.indicators.fips_mode, theme),
    ]
}

// ---------------------------------------------------------------------------
// Line builders (private helpers)
// ---------------------------------------------------------------------------

/// Build a two-column line with independently styled values.
///
/// Left pair: `left_label : left_value` styled with `left_value_style`.
/// Right pair: `right_label : right_value` styled with `right_value_style`.
#[allow(clippy::too_many_arguments)]
fn two_col_line<'a>(
    left_label: &str,
    left_value: &str,
    left_value_style: ratatui::style::Style,
    right_label: &str,
    right_value: &str,
    right_value_style: ratatui::style::Style,
    theme: &'a Theme,
) -> Line<'a> {
    let left_label_str = format!("{left_label:<LABEL_WIDTH$} : ");
    let right_label_str = format!("  {right_label:<LABEL_WIDTH$} : ");
    Line::from(vec![
        Span::styled(left_label_str, theme.header_field),
        Span::styled(left_value.to_owned(), left_value_style),
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
    let left_label_str = format!("{left_label:<LABEL_WIDTH$} : ");
    let right_label_str = format!("  {right_label:<LABEL_WIDTH$} : ");
    let left_text = indicator_text(left_value);
    let right_text = indicator_text(right_value);
    let left_style = theme.indicator_style(left_value);
    let right_style = theme.indicator_style(right_value);
    Line::from(vec![
        Span::styled(left_label_str, theme.header_field),
        Span::styled(left_text, left_style),
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
    let label_str = format!("{label:<LABEL_WIDTH$} : ");
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
    let label_str = format!("{label:<LABEL_WIDTH$} : ");
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
/// `Active(s)` and `Inactive(s)` return the inner string. `Unavailable`
/// returns `"unavailable"` — explicit, never empty or misleading.
fn indicator_text(value: &IndicatorValue) -> String {
    match value {
        IndicatorValue::Active(s) | IndicatorValue::Inactive(s) => s.clone(),
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
/// Supplemental fields are rendered after the six fixed header rows (Assessment,
/// Scope, Host, Tool, indicator pairs). They extend audit card identification
/// so each rendered card can serve as a standalone SP 800-53A Examine object.
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
            "…".to_owned(),
            theme.header_field,
        )));
    }
}

/// Build a single header line for a supplemental [`HeaderField`].
///
/// Uses the same `"Label      : value"` format as the fixed rows. The value
/// column is styled by the field's `style_hint`.
fn supplemental_field_line<'a>(field: &HeaderField, theme: &'a Theme) -> Line<'a> {
    let label_str = format!("{:<LABEL_WIDTH$} : ", field.label);
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
