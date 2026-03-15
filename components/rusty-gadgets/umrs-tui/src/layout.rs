// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Layout — Master Audit Card Render Function
//!
//! Composes all panel widgets into the full audit card layout:
//!
//! ```text
//! ┌─────────────────────────────────┬──────────────┐
//! │ Header (report, host, subject)  │  WIZARD_SMALL │
//! ├─────────────────────────────────┴──────────────┤
//! │ Tab bar                                         │
//! ├─────────────────────────────────────────────────┤
//! │                                                 │
//! │  Data panel (scrollable key-value rows)         │
//! │                                                 │
//! ├─────────────────────────────────────────────────┤
//! │ Status bar                                      │
//! └─────────────────────────────────────────────────┘
//! ```
//!
//! The header height is fixed at `WIZARD_SMALL.height + 2 + 1` (borders + indicator row).
//! The logo panel width is fixed at `WIZARD_SMALL.width + 2` (borders).
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Every frame contains identification (header),
//!   context (tabs), data, and status — no field is ever silently omitted.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use umrs_core::robots::WIZARD_SMALL;

use crate::app::{AuditCardApp, AuditCardState, SecurityIndicators};
use crate::data_panel::render_data_panel;
use crate::header::render_header;
use crate::status_bar::render_status_bar;
use crate::tabs::render_tabs;
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Logo panel width = wizard width + 2 border columns.
///
/// `WIZARD_SMALL.width` is 15 — well within u16 range. The cast is safe.
#[allow(clippy::cast_possible_truncation)]
const LOGO_PANEL_WIDTH: u16 = WIZARD_SMALL.width as u16 + 2;

/// Header height = wizard height + 2 border rows + 1 indicator row.
///
/// `WIZARD_SMALL.height` is 7 — well within u16 range. The cast is safe.
/// The +1 accounts for the security indicator badge row added in Phase 1.
#[allow(clippy::cast_possible_truncation)]
const HEADER_HEIGHT: u16 = WIZARD_SMALL.height as u16 + 2 + 1;

// ---------------------------------------------------------------------------
// Master render entry point
// ---------------------------------------------------------------------------

/// Render the complete audit card into `area`.
///
/// Call this inside `terminal.draw(|f| render_audit_card(f, f.area(), ...))`.
///
/// `state` carries mutable UI state (active tab, scroll offset). The data
/// itself is read from `app` on every frame — keep `data_rows()` cheap.
/// `indicators` is a `SecurityIndicators` snapshot populated once per session
/// via [`crate::indicators::read_security_indicators`].
///
/// NIST SP 800-53 AU-3 — all fields (report, host, subject, indicators, data,
/// status) are always present in every rendered frame.
/// NIST SP 800-53 SI-7 — indicator values originate from provenance-verified
/// kernel attribute reads; the layout renders them unmodified.
pub fn render_audit_card(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    state: &AuditCardState,
    indicators: &SecurityIndicators,
    theme: &Theme,
) {
    // ── Outer vertical split ─────────────────────────────────────────────
    // [header_row, tab_bar, data_panel, status_bar]
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEADER_HEIGHT),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let header_area = outer[0];
    let tab_area = outer[1];
    let data_area = outer[2];
    let status_area = outer[3];

    // ── Header row: text info + logo ─────────────────────────────────────
    let header_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(LOGO_PANEL_WIDTH)])
        .split(header_area);

    render_header(frame, header_cols[0], app, indicators, theme);
    render_logo(frame, header_cols[1], theme);

    // ── Tab bar ──────────────────────────────────────────────────────────
    render_tabs(frame, tab_area, app, state.active_tab, theme);

    // ── Data panel ───────────────────────────────────────────────────────
    render_data_panel(
        frame,
        data_area,
        app,
        state.active_tab,
        state.scroll_offset,
        theme,
    );

    // ── Status bar ───────────────────────────────────────────────────────
    render_status_bar(frame, status_area, app, theme);
}

// ---------------------------------------------------------------------------
// Logo panel
// ---------------------------------------------------------------------------

/// Render the WIZARD_SMALL logo inside a bordered panel.
///
/// Each line is rendered green (wizard color from theme). The border uses
/// the standard theme border style.
fn render_logo(frame: &mut Frame, area: Rect, theme: &Theme) {
    let lines: Vec<Line<'_>> = WIZARD_SMALL
        .lines
        .iter()
        .map(|l| Line::from(Span::styled(*l, theme.wizard)))
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .style(Style::default());

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
