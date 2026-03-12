// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for color-mapping functions in [`umrs_tui::theme`].
//!
//! Verifies that each [`StatusLevel`] and [`StyleHint`] variant maps to the
//! documented terminal [`Color`]. These are pure const functions — no
//! terminal backend required.

use ratatui::style::Color;
use umrs_tui::app::{StatusLevel, StyleHint};
use umrs_tui::theme::{status_bg_color, style_hint_color};

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
