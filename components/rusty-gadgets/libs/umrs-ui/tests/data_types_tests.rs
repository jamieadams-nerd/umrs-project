// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for the supporting data types in [`umrs_ui::app`]
//! and [`umrs_ui::tabs`].
//!
//! Covers `DataRow`, `TabDef`, `StatusMessage`, `StyleHint`, `StatusLevel`,
//! and the `tabs_from_labels` convenience helper. All tests are pure logic —
//! no terminal backend required.

use umrs_ui::app::{DataRow, StatusLevel, StatusMessage, StyleHint, TabDef};
use umrs_ui::tabs::tabs_from_labels;

// ---------------------------------------------------------------------------
// DataRow
// ---------------------------------------------------------------------------

#[test]
fn data_row_key_value_sets_key_value_and_hint() {
    let row =
        DataRow::key_value("Hostname", "rhel10.example", StyleHint::Highlight);
    match row {
        DataRow::KeyValue {
            key,
            value,
            style_hint,
            ..
        } => {
            assert_eq!(key, "Hostname");
            assert_eq!(value, "rhel10.example");
            assert_eq!(style_hint, StyleHint::Highlight);
        }
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

#[test]
fn data_row_new_is_alias_for_key_value() {
    let row = DataRow::new("Hostname", "rhel10.example", StyleHint::Highlight);
    assert!(
        matches!(row, DataRow::KeyValue { .. }),
        "DataRow::new must produce the KeyValue variant"
    );
}

#[test]
fn data_row_key_value_sets_highlight_key_false_by_default() {
    let row = DataRow::key_value("Version", "6.12.0", StyleHint::Normal);
    match row {
        DataRow::KeyValue {
            highlight_key,
            ..
        } => {
            assert!(
                !highlight_key,
                "key_value must set highlight_key=false by default"
            );
        }
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

#[test]
fn data_row_key_value_highlighted_sets_highlight_key_true() {
    let row = DataRow::key_value_highlighted(
        "Kernel Version",
        "6.12.0",
        StyleHint::Highlight,
    );
    match row {
        DataRow::KeyValue {
            key,
            highlight_key,
            ..
        } => {
            assert_eq!(key, "Kernel Version");
            assert!(
                highlight_key,
                "key_value_highlighted must set highlight_key=true"
            );
        }
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

#[test]
fn data_row_normal_sets_normal_hint() {
    let row = DataRow::normal("OS", "RHEL 10");
    match row {
        DataRow::KeyValue {
            key,
            value,
            style_hint,
            ..
        } => {
            assert_eq!(key, "OS");
            assert_eq!(value, "RHEL 10");
            assert_eq!(
                style_hint,
                StyleHint::Normal,
                "DataRow::normal must set StyleHint::Normal"
            );
        }
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

#[test]
fn data_row_separator_variant_is_separator() {
    let row = DataRow::separator();
    assert!(
        matches!(row, DataRow::Separator),
        "DataRow::separator() must produce the Separator variant"
    );
}

#[test]
fn data_row_accepts_string_owned_values() {
    let key = String::from("Dynamic Key");
    let val = String::from("Dynamic Value");
    let row = DataRow::new(key.clone(), val.clone(), StyleHint::TrustGreen);
    match row {
        DataRow::KeyValue {
            key: k,
            value: v,
            ..
        } => {
            assert_eq!(k, key);
            assert_eq!(v, val);
        }
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

#[test]
fn data_row_two_column_sets_all_fields() {
    let row = DataRow::two_column(
        "left key",
        "left value",
        StyleHint::TrustGreen,
        "right key",
        "right value",
        StyleHint::TrustYellow,
    );
    match row {
        DataRow::TwoColumn {
            left_key,
            left_value,
            left_hint,
            right_key,
            right_value,
            right_hint,
        } => {
            assert_eq!(left_key, "left key");
            assert_eq!(left_value, "left value");
            assert_eq!(left_hint, StyleHint::TrustGreen);
            assert_eq!(right_key, "right key");
            assert_eq!(right_value, "right value");
            assert_eq!(right_hint, StyleHint::TrustYellow);
        }
        other => panic!("expected DataRow::TwoColumn, got {other:?}"),
    }
}

#[test]
fn data_row_group_title_stores_string() {
    let row = DataRow::group_title("SELinux");
    match row {
        DataRow::GroupTitle(title) => {
            assert_eq!(
                title, "SELinux",
                "group_title must store the provided string verbatim"
            );
        }
        other => panic!("expected DataRow::GroupTitle, got {other:?}"),
    }
}

#[test]
fn data_row_group_title_accepts_owned_string() {
    let label = String::from("FIPS Status");
    let row = DataRow::group_title(label.clone());
    match row {
        DataRow::GroupTitle(title) => assert_eq!(title, label),
        other => panic!("expected DataRow::GroupTitle, got {other:?}"),
    }
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
// DataRow::TableRow and DataRow::TableHeader (Phase 6)
// ---------------------------------------------------------------------------

#[test]
fn data_row_table_row_stores_three_columns() {
    let row = DataRow::table_row(
        "package-db",
        "/var/lib/rpm/rpmdb.sqlite",
        "\u{2713} ok (fd)",
        StyleHint::TrustGreen,
    );
    match row {
        DataRow::TableRow {
            col1,
            col2,
            col3,
            style_hint,
        } => {
            assert_eq!(col1, "package-db");
            assert_eq!(col2, "/var/lib/rpm/rpmdb.sqlite");
            assert_eq!(col3, "\u{2713} ok (fd)");
            assert_eq!(style_hint, StyleHint::TrustGreen);
        }
        other => panic!("expected DataRow::TableRow, got {other:?}"),
    }
}

#[test]
fn data_row_table_header_stores_three_columns() {
    let row = DataRow::table_header("Evidence Type", "Source", "Verification");
    match row {
        DataRow::TableHeader {
            col1,
            col2,
            col3,
        } => {
            assert_eq!(col1, "Evidence Type");
            assert_eq!(col2, "Source");
            assert_eq!(col3, "Verification");
        }
        other => panic!("expected DataRow::TableHeader, got {other:?}"),
    }
}

#[test]
fn data_row_table_row_accepts_owned_strings() {
    let c1 = String::from("procfs");
    let c2 = String::from("/proc/sys/kernel/ostype");
    let c3 = String::from("\u{2717} FAIL (path)");
    let row = DataRow::table_row(
        c1.clone(),
        c2.clone(),
        c3.clone(),
        StyleHint::TrustRed,
    );
    match row {
        DataRow::TableRow {
            col1,
            col2,
            col3,
            style_hint,
        } => {
            assert_eq!(col1, c1);
            assert_eq!(col2, c2);
            assert_eq!(col3, c3);
            assert_eq!(style_hint, StyleHint::TrustRed);
        }
        other => panic!("expected DataRow::TableRow, got {other:?}"),
    }
}

#[test]
fn data_row_table_header_accepts_owned_strings() {
    let c1 = String::from("Type");
    let c2 = String::from("Path");
    let c3 = String::from("Result");
    let row = DataRow::table_header(c1.clone(), c2.clone(), c3.clone());
    match row {
        DataRow::TableHeader {
            col1,
            col2,
            col3,
        } => {
            assert_eq!(col1, c1);
            assert_eq!(col2, c2);
            assert_eq!(col3, c3);
        }
        other => panic!("expected DataRow::TableHeader, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// DataRow::IndicatorRow (Round 5 layout)
// ---------------------------------------------------------------------------

#[test]
fn data_row_indicator_row_stores_all_fields() {
    let row = DataRow::indicator_row(
        "  kexec_load_disabled",
        "0 (disabled)",
        "Prevents loading a new kernel image at runtime.",
        StyleHint::TrustRed,
    );
    match row {
        DataRow::IndicatorRow {
            key,
            value,
            description,
            recommendation,
            contradiction,
            configured_line,
            style_hint,
        } => {
            assert_eq!(key, "  kexec_load_disabled");
            assert_eq!(value, "0 (disabled)");
            assert_eq!(
                description,
                "Prevents loading a new kernel image at runtime."
            );
            assert_eq!(recommendation, None);
            // New fields default to None when using the basic constructor.
            assert_eq!(contradiction, None);
            assert_eq!(configured_line, None);
            assert_eq!(style_hint, StyleHint::TrustRed);
        }
        other => panic!("expected DataRow::IndicatorRow, got {other:?}"),
    }
}

#[test]
fn data_row_indicator_row_empty_description_is_valid() {
    // Phase 2b CPU indicators supply an empty description — must not panic.
    let row = DataRow::indicator_row(
        "  some_cpu_indicator",
        "enabled",
        "",
        StyleHint::TrustGreen,
    );
    assert!(
        matches!(row, DataRow::IndicatorRow { .. }),
        "indicator_row with empty description must produce IndicatorRow variant"
    );
}

#[test]
fn data_row_indicator_row_accepts_owned_string_key() {
    let key = String::from("  dynamic_key");
    let row = DataRow::indicator_row(
        key.clone(),
        "1",
        "Some description.",
        StyleHint::Normal,
    );
    match row {
        DataRow::IndicatorRow {
            key: k,
            ..
        } => assert_eq!(k, key),
        other => panic!("expected DataRow::IndicatorRow, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// DataRow::IndicatorRow — recommendation field (Round 6)
// ---------------------------------------------------------------------------

#[test]
fn data_row_indicator_row_recommended_stores_recommendation() {
    // When `recommendation` is `Some`, the field is stored and retrievable.
    let row = DataRow::indicator_row_recommended(
        "  kptr_restrict",
        "0 (pointers visible)",
        "Hides kernel pointer addresses.",
        Some("2 (hidden from all users)"),
        StyleHint::TrustRed,
    );
    match row {
        DataRow::IndicatorRow {
            recommendation,
            style_hint,
            ..
        } => {
            assert_eq!(recommendation, Some("2 (hidden from all users)"));
            assert_eq!(style_hint, StyleHint::TrustRed);
        }
        other => panic!("expected DataRow::IndicatorRow, got {other:?}"),
    }
}

#[test]
fn data_row_indicator_row_recommended_none_matches_indicator_row() {
    // Passing `None` for recommendation is equivalent to `indicator_row`.
    let row_plain = DataRow::indicator_row(
        "  randomize_va_space",
        "2 (full ASLR)",
        "ASLR description.",
        StyleHint::TrustGreen,
    );
    let row_recommended = DataRow::indicator_row_recommended(
        "  randomize_va_space",
        "2 (full ASLR)",
        "ASLR description.",
        None,
        StyleHint::TrustGreen,
    );
    // Both must produce IndicatorRow with recommendation = None.
    match (row_plain, row_recommended) {
        (
            DataRow::IndicatorRow {
                recommendation: r1,
                ..
            },
            DataRow::IndicatorRow {
                recommendation: r2,
                ..
            },
        ) => {
            assert_eq!(r1, None);
            assert_eq!(r2, None);
        }
        _ => panic!("both rows must be DataRow::IndicatorRow"),
    }
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
