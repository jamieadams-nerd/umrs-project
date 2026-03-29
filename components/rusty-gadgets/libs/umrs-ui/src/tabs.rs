// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Tabs — Tab Bar Widget
//!
//! Renders the horizontal tab bar below the header. The active tab is
//! highlighted using the theme's `tab_active` style; inactive tabs use
//! `tab_inactive`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: The active tab label communicates the current
//!   view context in every rendered frame.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Tabs;

use crate::app::{AuditCardApp, TabDef};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Render the tab bar for the given app state and active tab index.
///
/// `active_tab` comes from [`AuditCardState::active_tab`], which is the
/// authoritative source for the currently displayed tab.
///
/// NIST SP 800-53 AU-3 — current view context is always visible.
pub fn render_tabs(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    active_tab: usize,
    theme: &Theme,
) {
    let tab_titles: Vec<Line<'_>> = app
        .tabs()
        .iter()
        .map(|t| Line::from(format!(" {} ", t.label)))
        .collect();

    let tabs = Tabs::new(tab_titles)
        .select(active_tab)
        .style(Style::default())
        .highlight_style(theme.tab_active)
        .divider("|");

    frame.render_widget(tabs, area);
}

/// Build a tab definition list from string labels (convenience helper).
///
/// Useful when constructing `AuditCardApp` impls with a fixed tab set.
#[must_use = "tab definitions must be stored; discarding them leaves the card with no tabs"]
pub fn tabs_from_labels(labels: &[&str]) -> Vec<TabDef> {
    labels.iter().map(|l| TabDef::new(*l)).collect()
}
