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

use crate::app::AuditCardApp;
use crate::theme::Theme;

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
/// Displays report name, hostname, and report subject inside a bordered block.
///
/// NIST SP 800-53 AU-3 — identifies report, subject, and host on every card.
pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    theme: &Theme,
) {
    let host = hostname();

    let lines = vec![
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

    let title = format!(" {} ", app.card_title());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(title, theme.header_name));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
