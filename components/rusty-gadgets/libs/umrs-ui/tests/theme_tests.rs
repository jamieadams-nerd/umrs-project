// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for color-mapping functions in [`umrs_ui::theme`].
//!
//! Verifies that each [`StatusLevel`] and [`StyleHint`] variant maps to the
//! documented terminal [`Color`]. These are pure const functions — no
//! terminal backend required.
//!
//! ## NO_COLOR coverage
//!
//! The `no_color_*` tests below verify that `Theme::no_color()` produces styles
//! with no foreground and no background color on every field.  This is the
//! compile-time proof that the `NO_COLOR` specification is honored without
//! relying on terminal inspection at runtime.
//!
//! NIST SP 800-53 SI-11 / WCAG 1.4.1.

use ratatui::style::{Color, Style};
use umrs_ui::Theme;
use umrs_ui::app::{IndicatorValue, StatusLevel, StyleHint};
use umrs_ui::theme::{status_bg_color, style_hint_color};

// ---------------------------------------------------------------------------
// Helper: assert a Style carries no fg and no bg color.
// ---------------------------------------------------------------------------

/// Returns `true` when the style has no foreground and no background color.
///
/// Ratatui only emits ANSI color-select sequences when `fg` or `bg` is `Some`.
/// This predicate is the lightweight check that a field is color-free.
const fn has_no_color(style: &Style) -> bool {
    style.fg.is_none() && style.bg.is_none()
}

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
fn theme_default_indicator_disabled_and_unavailable_differ() {
    let theme = Theme::default();
    let disabled = theme.indicator_style(&IndicatorValue::Disabled("off".to_owned()));
    let unavailable = theme.indicator_style(&IndicatorValue::Unavailable);
    assert_ne!(
        disabled, unavailable,
        "disabled and unavailable must have distinct styles"
    );
}

#[test]
fn theme_default_all_indicator_styles_differ() {
    let theme = Theme::default();
    let enabled = theme.indicator_style(&IndicatorValue::Enabled("on".to_owned()));
    let disabled = theme.indicator_style(&IndicatorValue::Disabled("off".to_owned()));
    let unavailable = theme.indicator_style(&IndicatorValue::Unavailable);
    assert_ne!(
        enabled, disabled,
        "enabled and disabled must have distinct styles"
    );
    assert_ne!(
        enabled, unavailable,
        "enabled and unavailable must have distinct styles"
    );
    assert_ne!(
        disabled, unavailable,
        "disabled and unavailable must have distinct styles"
    );
}

// ---------------------------------------------------------------------------
// indicator_style — named coverage (Phase 5)
// ---------------------------------------------------------------------------

#[test]
fn indicator_style_enabled_returns_active_style() {
    let theme = Theme::default();
    let value = IndicatorValue::Enabled("Enforcing".to_owned());
    let style = theme.indicator_style(&value);
    assert_eq!(
        style, theme.indicator_active,
        "Enabled variant must return the indicator_active theme style"
    );
}

#[test]
fn indicator_style_disabled_returns_inactive_style() {
    let theme = Theme::default();
    let value = IndicatorValue::Disabled("Permissive".to_owned());
    let style = theme.indicator_style(&value);
    assert_eq!(
        style, theme.indicator_inactive,
        "Disabled variant must return the indicator_inactive theme style"
    );
}

#[test]
fn indicator_style_unavailable_returns_unavailable_style() {
    let theme = Theme::default();
    let style = theme.indicator_style(&IndicatorValue::Unavailable);
    assert_eq!(
        style, theme.indicator_unavailable,
        "Unavailable variant must return the indicator_unavailable theme style"
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

// ---------------------------------------------------------------------------
// Theme::no_color — field coverage
//
// Each test verifies that one or more closely related fields carry no fg/bg
// color so that ratatui emits no ANSI color-select sequences when rendering.
//
// NIST SP 800-53 SI-11 — output must remain meaningful without color.
// WCAG 1.4.1 — information must not be conveyed by color alone.
// ---------------------------------------------------------------------------

#[test]
fn no_color_border_has_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.border),
        "no_color theme: border must carry no fg/bg color"
    );
}

#[test]
fn no_color_tabs_have_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.tab_active),
        "no_color theme: tab_active must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.tab_inactive),
        "no_color theme: tab_inactive must carry no fg/bg color"
    );
}

#[test]
fn no_color_data_rows_have_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.data_key),
        "no_color theme: data_key must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.data_value),
        "no_color theme: data_value must carry no fg/bg color"
    );
}

#[test]
fn no_color_header_styles_have_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.header_name),
        "no_color theme: header_name must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.header_field),
        "no_color theme: header_field must carry no fg/bg color"
    );
}

#[test]
fn no_color_wizard_has_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.wizard),
        "no_color theme: wizard must carry no fg/bg color"
    );
}

#[test]
fn no_color_status_text_has_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.status_text),
        "no_color theme: status_text must carry no fg/bg color"
    );
}

#[test]
fn no_color_indicators_have_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.indicator_active),
        "no_color theme: indicator_active must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.indicator_inactive),
        "no_color theme: indicator_inactive must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.indicator_unavailable),
        "no_color theme: indicator_unavailable must carry no fg/bg color"
    );
}

#[test]
fn no_color_list_selection_has_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.list_selection),
        "no_color theme: list_selection must carry no fg/bg color (uses REVERSED modifier only)"
    );
}

#[test]
fn no_color_group_title_has_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.group_title),
        "no_color theme: group_title must carry no fg/bg color"
    );
}

#[test]
fn no_color_dialog_styles_have_no_color() {
    let t = Theme::no_color();
    assert!(
        has_no_color(&t.dialog_info_border),
        "no_color theme: dialog_info_border must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.dialog_error_border),
        "no_color theme: dialog_error_border must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.dialog_security_border),
        "no_color theme: dialog_security_border must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.dialog_button_focused),
        "no_color theme: dialog_button_focused must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.dialog_button_unfocused),
        "no_color theme: dialog_button_unfocused must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.dialog_title),
        "no_color theme: dialog_title must carry no fg/bg color"
    );
    assert!(
        has_no_color(&t.dialog_message),
        "no_color theme: dialog_message must carry no fg/bg color"
    );
}

#[test]
fn no_color_indicator_style_dispatch_has_no_color() {
    // Verify the dispatch method itself produces color-free styles.
    let t = Theme::no_color();
    let enabled = t.indicator_style(&IndicatorValue::Enabled("on".to_owned()));
    let disabled = t.indicator_style(&IndicatorValue::Disabled("off".to_owned()));
    let unavailable = t.indicator_style(&IndicatorValue::Unavailable);
    assert!(
        has_no_color(&enabled),
        "no_color theme: indicator_style(Enabled) must carry no fg/bg color"
    );
    assert!(
        has_no_color(&disabled),
        "no_color theme: indicator_style(Disabled) must carry no fg/bg color"
    );
    assert!(
        has_no_color(&unavailable),
        "no_color theme: indicator_style(Unavailable) must carry no fg/bg color"
    );
}

#[test]
fn no_color_dark_themes_differ() {
    // no_color() and dark() must produce different styles — the no_color theme
    // is not silently falling through to the colored palette.
    let nc = Theme::no_color();
    let dk = Theme::dark();
    // The border field is the most direct differentiator: dark uses cyan fg,
    // no_color uses plain Style::default().
    assert_ne!(
        nc.border, dk.border,
        "Theme::no_color().border must differ from Theme::dark().border"
    );
}

#[test]
fn no_color_all_fields_color_free() {
    // Comprehensive sweep: every public field must have no fg and no bg.
    // This is the single authoritative proof that no ANSI color code will be
    // emitted for any rendered element when NO_COLOR is set.
    //
    // NIST SP 800-53 SI-11 / WCAG 1.4.1.
    let t = Theme::no_color();
    let fields: &[(&str, Style)] = &[
        ("border", t.border),
        ("tab_active", t.tab_active),
        ("tab_inactive", t.tab_inactive),
        ("data_key", t.data_key),
        ("data_value", t.data_value),
        ("header_name", t.header_name),
        ("header_field", t.header_field),
        ("wizard", t.wizard),
        ("status_text", t.status_text),
        ("indicator_active", t.indicator_active),
        ("indicator_inactive", t.indicator_inactive),
        ("indicator_unavailable", t.indicator_unavailable),
        ("list_selection", t.list_selection),
        ("group_title", t.group_title),
        ("dialog_info_border", t.dialog_info_border),
        ("dialog_error_border", t.dialog_error_border),
        ("dialog_security_border", t.dialog_security_border),
        ("dialog_button_focused", t.dialog_button_focused),
        ("dialog_button_unfocused", t.dialog_button_unfocused),
        ("dialog_title", t.dialog_title),
        ("dialog_message", t.dialog_message),
    ];
    for (name, style) in fields {
        assert!(
            has_no_color(style),
            "no_color theme: field '{name}' must carry no fg/bg color"
        );
    }
}
