// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for the supporting data types in [`umrs_tui::app`]
//! and [`umrs_tui::tabs`].
//!
//! Covers `DataRow`, `TabDef`, `StatusMessage`, `StyleHint`, `StatusLevel`,
//! and the `tabs_from_labels` convenience helper. All tests are pure logic —
//! no terminal backend required.

use umrs_tui::app::{DataRow, StatusLevel, StatusMessage, StyleHint, TabDef};
use umrs_tui::tabs::tabs_from_labels;

// ---------------------------------------------------------------------------
// DataRow
// ---------------------------------------------------------------------------

#[test]
fn data_row_new_sets_key_value_and_hint() {
    let row = DataRow::new("Hostname", "rhel10.example", StyleHint::Highlight);
    assert_eq!(row.key, "Hostname");
    assert_eq!(row.value, "rhel10.example");
    assert_eq!(row.style_hint, StyleHint::Highlight);
}

#[test]
fn data_row_normal_sets_normal_hint() {
    let row = DataRow::normal("OS", "RHEL 10");
    assert_eq!(row.key, "OS");
    assert_eq!(row.value, "RHEL 10");
    assert_eq!(
        row.style_hint,
        StyleHint::Normal,
        "DataRow::normal must set StyleHint::Normal"
    );
}

#[test]
fn data_row_separator_has_empty_key_and_value() {
    let row = DataRow::separator();
    assert!(row.key.is_empty(), "separator key must be empty");
    assert!(row.value.is_empty(), "separator value must be empty");
}

#[test]
fn data_row_separator_has_dim_style() {
    let row = DataRow::separator();
    assert_eq!(
        row.style_hint,
        StyleHint::Dim,
        "separator must use StyleHint::Dim"
    );
}

#[test]
fn data_row_accepts_string_owned_values() {
    let key = String::from("Dynamic Key");
    let val = String::from("Dynamic Value");
    let row = DataRow::new(key.clone(), val.clone(), StyleHint::TrustGreen);
    assert_eq!(row.key, key);
    assert_eq!(row.value, val);
}

// ---------------------------------------------------------------------------
// TabDef
// ---------------------------------------------------------------------------

#[test]
fn tab_def_new_sets_label() {
    let tab = TabDef::new("Overview");
    assert_eq!(tab.label, "Overview");
}

#[test]
fn tab_def_accepts_owned_string() {
    let label = String::from("Dynamic Tab");
    let tab = TabDef::new(label.clone());
    assert_eq!(tab.label, label);
}

// ---------------------------------------------------------------------------
// StatusMessage
// ---------------------------------------------------------------------------

#[test]
fn status_message_new_sets_level_and_text() {
    let msg = StatusMessage::new(StatusLevel::Warn, "Degraded state detected");
    assert_eq!(msg.level, StatusLevel::Warn);
    assert_eq!(msg.text, "Degraded state detected");
}

#[test]
fn status_message_default_level_is_info() {
    let msg = StatusMessage::default();
    assert_eq!(
        msg.level,
        StatusLevel::Info,
        "default StatusMessage must have Info level"
    );
}

#[test]
fn status_message_default_text_is_ready() {
    let msg = StatusMessage::default();
    assert_eq!(
        msg.text, "Ready",
        "default StatusMessage must have text 'Ready'"
    );
}

#[test]
fn status_message_error_level() {
    let msg = StatusMessage::new(StatusLevel::Error, "Pipeline failure");
    assert_eq!(msg.level, StatusLevel::Error);
}

#[test]
fn status_message_ok_level() {
    let msg = StatusMessage::new(StatusLevel::Ok, "All checks passed");
    assert_eq!(msg.level, StatusLevel::Ok);
}

// ---------------------------------------------------------------------------
// StyleHint — equality and coverage
// ---------------------------------------------------------------------------

#[test]
fn style_hint_variants_are_distinct() {
    assert_ne!(StyleHint::Normal, StyleHint::Highlight);
    assert_ne!(StyleHint::Dim, StyleHint::TrustGreen);
    assert_ne!(StyleHint::TrustYellow, StyleHint::TrustRed);
}

// ---------------------------------------------------------------------------
// StatusLevel — equality and coverage
// ---------------------------------------------------------------------------

#[test]
fn status_level_variants_are_distinct() {
    assert_ne!(StatusLevel::Info, StatusLevel::Ok);
    assert_ne!(StatusLevel::Warn, StatusLevel::Error);
    assert_ne!(StatusLevel::Ok, StatusLevel::Error);
}

// ---------------------------------------------------------------------------
// tabs_from_labels
// ---------------------------------------------------------------------------

#[test]
fn tabs_from_labels_produces_correct_count() {
    let tabs = tabs_from_labels(&["A", "B", "C"]);
    assert_eq!(tabs.len(), 3, "three labels must produce three TabDefs");
}

#[test]
fn tabs_from_labels_preserves_label_order() {
    let tabs = tabs_from_labels(&["Overview", "Details", "Raw"]);
    assert_eq!(tabs[0].label, "Overview");
    assert_eq!(tabs[1].label, "Details");
    assert_eq!(tabs[2].label, "Raw");
}

#[test]
fn tabs_from_labels_single_entry() {
    let tabs = tabs_from_labels(&["Only"]);
    assert_eq!(tabs.len(), 1);
    assert_eq!(tabs[0].label, "Only");
}

#[test]
fn tabs_from_labels_empty_slice_produces_empty_vec() {
    let tabs = tabs_from_labels(&[]);
    assert!(tabs.is_empty(), "empty input must produce empty Vec");
}
