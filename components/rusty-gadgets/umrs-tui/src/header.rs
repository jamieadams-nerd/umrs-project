// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Header — Header Panel Widget
//!
//! Renders the left-side header panel: report name, hostname, and report
//! subject. The hostname is obtained via `rustix::system::uname()` for
//! display purposes only.
//!
//! ## Trust Note
//!
//! The hostname shown here is display-only — not a trust-relevant assertion.
//! It comes from `uname(2)` which reflects the running kernel's nodename.
//! Do not use this value for identity or policy decisions.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit record content — the header identifies
//!   the report, subject, and host for every rendered card.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use umrs_core::i18n;

use crate::app::{
    AuditCardApp, HeaderField, IndicatorValue, SecurityIndicators,
};
use crate::theme::{Theme, style_hint_color};

// ---------------------------------------------------------------------------
// Hostname
// ---------------------------------------------------------------------------

/// Read the system hostname via `uname(2)`.
///
/// Returns a display string. Falls back to `"(unknown)"` if the syscall
/// fails or the nodename contains non-UTF-8 bytes.
///
/// # Security Rationale — Display-Only Use of `uname(2)`
///
/// This value is used **exclusively** as a visual label in the TUI header
/// so the operator can identify which host they are viewing. It is:
///
/// - **NOT** part of the evidence chain (`EvidenceBundle`)
/// - **NOT** included in any sealed cache payload (`SealedCache`)
/// - **NOT** used in any policy, identity, or trust decision
/// - **NOT** compared against any expected value
///
/// The hostname is inherently mutable (sethostname(2)) and therefore
/// unsuitable for security assertions. Any auditor reviewing this call
/// can confirm: the return value flows only to a ratatui `Span` for
/// on-screen rendering — no other consumer exists.
fn hostname() -> String {
    // Display-only — not a trust-relevant assertion.
    // See audit-card.md and the doc comment above for full rationale.
    let uname = rustix::system::uname();
    let nodename = uname.nodename();
    nodename.to_str().unwrap_or("(unknown)").to_owned()
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Render the header text panel (left side of the header row).
///
/// Displays report name, hostname, report subject, a compact security indicator
/// row, and any supplemental [`HeaderField`] entries provided by
/// `app.header_fields()`.
///
/// Fixed rows (report, host, subject, indicators) occupy the first 4 interior
/// lines. Any remaining interior height (area height minus 2 border rows minus
/// 4 fixed rows) is allocated to supplemental fields. If `app.header_fields()`
/// returns more entries than fit, the rendered list is truncated and a `"…"`
/// continuation marker is appended in place of the last slot.
///
/// The indicator row shows live kernel security state read via `SecureReader`.
/// Each badge is colored by `theme.indicator_style()`:
/// - Active → green bold
/// - Inactive → dark gray
/// - Unavailable → yellow (attention — read failure)
///
/// NIST SP 800-53 AU-3 — identifies report, subject, host, kernel security
/// posture, and supplemental context on every rendered card.
/// NIST SP 800-53 SI-7 — indicator values are sourced from provenance-verified
/// kernel attribute reads.
pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    indicators: &SecurityIndicators,
    theme: &Theme,
) {
    let host = hostname();

    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                format!("{:<8} : ", i18n::tr("Report")),
                theme.header_field,
            ),
            Span::styled(app.report_name().to_owned(), theme.header_name),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{:<8} : ", i18n::tr("Host")),
                theme.header_field,
            ),
            // Display-only — not a trust-relevant assertion (see audit-card plan)
            Span::styled(host, theme.header_name),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{:<8} : ", i18n::tr("Subject")),
                theme.header_field,
            ),
            Span::styled(app.report_subject().to_owned(), theme.header_name),
        ]),
    ];

    // Indicator row — compact security posture badges.
    lines.push(indicator_row(indicators, theme));

    // Supplemental header fields from the app impl.
    // Fixed rows: 3 (report/host/subject) + 1 (indicators) = 4.
    // Interior rows = area.height - 2 border rows.
    // Available slots for supplemental fields = interior.saturating_sub(4).
    let interior_rows = (area.height as usize).saturating_sub(2);
    let available = interior_rows.saturating_sub(4);

    if available > 0 {
        let fields = app.header_fields();
        append_header_fields(&mut lines, fields, available, theme);
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
// Supplemental header fields helper
// ---------------------------------------------------------------------------

/// Append supplemental [`HeaderField`] entries to the header line list.
///
/// Renders at most `available` lines. If `fields` contains more entries than
/// `available`, the last rendered slot is replaced with a `"…"` continuation
/// marker to signal truncation — the operator knows more fields exist than
/// can be displayed in the current header height.
///
/// Uses the same key/value column layout as the fixed report/host/subject rows
/// for visual consistency.
fn append_header_fields(
    lines: &mut Vec<Line<'_>>,
    fields: &[HeaderField],
    available: usize,
    theme: &Theme,
) {
    if fields.is_empty() || available == 0 {
        return;
    }

    let fits_all = fields.len() <= available;

    if fits_all {
        for field in fields {
            lines.push(header_field_line(field, theme));
        }
    } else {
        // Render the first (available - 1) fields, then append the truncation marker.
        let to_render = available.saturating_sub(1);
        for field in fields.iter().take(to_render) {
            lines.push(header_field_line(field, theme));
        }
        lines
            .push(Line::from(Span::styled("…".to_owned(), theme.header_field)));
    }
}

/// Build a single [`HeaderField`] line using the key/value column layout.
fn header_field_line<'a>(field: &HeaderField, theme: &Theme) -> Line<'a> {
    use ratatui::style::Style;
    let value_style = Style::default().fg(style_hint_color(field.style_hint));
    Line::from(vec![
        Span::styled(format!("{:<8} : ", field.label), theme.header_field),
        Span::styled(field.value.clone(), value_style),
    ])
}

// ---------------------------------------------------------------------------
// Indicator row helper
// ---------------------------------------------------------------------------

/// Build the compact security indicator row line.
///
/// Format: `[SEL:<value>] [FIPS:<value>] [LSM:<value>] [LKD:<value>] [SB:<value>]`
///
/// Each badge is styled per its `IndicatorValue` variant. Unavailable values
/// display as `"?"` to make the degraded state obvious.
fn indicator_row<'a>(
    indicators: &SecurityIndicators,
    theme: &Theme,
) -> Line<'a> {
    let mut spans: Vec<Span<'a>> = Vec::new();

    let badges: &[(&str, &IndicatorValue)] = &[
        ("SEL", &indicators.selinux_status),
        ("FIPS", &indicators.fips_mode),
        ("LSM", &indicators.active_lsm),
        ("LKD", &indicators.lockdown_mode),
        ("SB", &indicators.secure_boot),
    ];

    for (i, (label, value)) in badges.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" "));
        }
        let display = indicator_display_text(value);
        let style = theme.indicator_style(value);
        spans.push(Span::styled(format!("[{label}:{display}]"), style));
    }

    Line::from(spans)
}

/// Extract the short display text for an `IndicatorValue`.
///
/// `Active(s)` and `Inactive(s)` return the inner string; `Unavailable` returns `"?"`.
const fn indicator_display_text(value: &IndicatorValue) -> &str {
    match value {
        IndicatorValue::Active(s) | IndicatorValue::Inactive(s) => s.as_str(),
        IndicatorValue::Unavailable => "?",
    }
}
