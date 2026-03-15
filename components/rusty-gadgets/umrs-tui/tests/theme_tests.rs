// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for color-mapping functions in [`umrs_tui::theme`].
//!
//! Verifies that each [`StatusLevel`] and [`StyleHint`] variant maps to the
//! documented terminal [`Color`]. These are pure const functions — no
//! terminal backend required.

use ratatui::style::{Color, Style};
use umrs_tui::Theme;
use umrs_tui::app::{IndicatorValue, StatusLevel, StyleHint};
use umrs_tui::theme::{status_bg_color, style_hint_color};

// ---------------------------------------------------------------------------
// Theme::default — field coverage
// ---------------------------------------------------------------------------

#[test]
fn theme_default_has_group_title_style() {
    let theme = Theme::default();
    // group_title must not be the zero Style — it should carry at least a
    // foreground color so it is visually distinct from data rows.
    assert_ne!(
        theme.group_title,
        Style::default(),
        "theme.group_title must not be the zero Style"
    );
}

#[test]
fn theme_default_indicator_unavailable_is_yellow() {
    let theme = Theme::default();
    // indicator_unavailable must use Color::Yellow so it is visually
    // distinct from indicator_inactive (DarkGray). A failed kernel probe
    // is operationally different from a known-inactive feature.
    // NIST SP 800-53 CA-7 — continuous monitoring requires this distinction.
    let style = theme.indicator_style(&IndicatorValue::Unavailable);
    assert_eq!(
        style.fg,
        Some(Color::Yellow),
        "indicator_unavailable must use Yellow foreground (CA-7: \
         failed probe is distinct from known-inactive)"
    );
}

#[test]
fn theme_default_indicator_inactive_and_unavailable_differ() {
    let theme = Theme::default();
    let inactive =
        theme.indicator_style(&IndicatorValue::Inactive("off".to_owned()));
    let unavailable = theme.indicator_style(&IndicatorValue::Unavailable);
    assert_ne!(
        inactive, unavailable,
        "inactive and unavailable must have distinct styles"
    );
}

#[test]
fn theme_default_all_indicator_styles_differ() {
    let theme = Theme::default();
    let active =
        theme.indicator_style(&IndicatorValue::Active("on".to_owned()));
    let inactive =
        theme.indicator_style(&IndicatorValue::Inactive("off".to_owned()));
    let unavailable = theme.indicator_style(&IndicatorValue::Unavailable);
    assert_ne!(
        active, inactive,
        "active and inactive must have distinct styles"
    );
    assert_ne!(
        active, unavailable,
        "active and unavailable must have distinct styles"
    );
    assert_ne!(
        inactive, unavailable,
        "inactive and unavailable must have distinct styles"
    );
}

// ---------------------------------------------------------------------------
// status_bg_color
// ---------------------------------------------------------------------------

#[test]
fn status_bg_info_is_blue() {
    assert_eq!(
        status_bg_color(StatusLevel::Info),
        Color::Blue,
        "Info must map to Blue background"
    );
}

#[test]
fn status_bg_ok_is_green() {
    assert_eq!(
        status_bg_color(StatusLevel::Ok),
        Color::Green,
        "Ok must map to Green background"
    );
}

#[test]
fn status_bg_warn_is_yellow() {
    assert_eq!(
        status_bg_color(StatusLevel::Warn),
        Color::Yellow,
        "Warn must map to Yellow background"
    );
}

#[test]
fn status_bg_error_is_red() {
    assert_eq!(
        status_bg_color(StatusLevel::Error),
        Color::Red,
        "Error must map to Red background"
    );
}

#[test]
fn all_status_levels_produce_distinct_colors() {
    let colors = [
        status_bg_color(StatusLevel::Info),
        status_bg_color(StatusLevel::Ok),
        status_bg_color(StatusLevel::Warn),
        status_bg_color(StatusLevel::Error),
    ];
    // Each pair must be distinct — duplicates would make status ambiguous.
    for (i, a) in colors.iter().enumerate() {
        for (j, b) in colors.iter().enumerate() {
            if i != j {
                assert_ne!(
                    a, b,
                    "status_bg_color must return distinct colors for every level"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// style_hint_color
// ---------------------------------------------------------------------------

#[test]
fn style_hint_normal_is_white() {
    assert_eq!(
        style_hint_color(StyleHint::Normal),
        Color::White,
        "Normal must map to White"
    );
}

#[test]
fn style_hint_highlight_is_cyan() {
    assert_eq!(
        style_hint_color(StyleHint::Highlight),
        Color::Cyan,
        "Highlight must map to Cyan"
    );
}

#[test]
fn style_hint_dim_is_dark_gray() {
    assert_eq!(
        style_hint_color(StyleHint::Dim),
        Color::DarkGray,
        "Dim must map to DarkGray"
    );
}

#[test]
fn style_hint_trust_green_is_green() {
    assert_eq!(
        style_hint_color(StyleHint::TrustGreen),
        Color::Green,
        "TrustGreen must map to Green"
    );
}

#[test]
fn style_hint_trust_yellow_is_yellow() {
    assert_eq!(
        style_hint_color(StyleHint::TrustYellow),
        Color::Yellow,
        "TrustYellow must map to Yellow"
    );
}

#[test]
fn style_hint_trust_red_is_red() {
    assert_eq!(
        style_hint_color(StyleHint::TrustRed),
        Color::Red,
        "TrustRed must map to Red"
    );
}
