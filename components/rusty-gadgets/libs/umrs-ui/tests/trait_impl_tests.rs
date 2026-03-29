// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for the [`AuditCardApp`] trait via a minimal mock impl.
//!
//! Verifies object safety, correct return values for all trait methods, and
//! that an invalid tab index produces a defined fallback from `data_rows`.
//! No terminal backend is required.

use umrs_ui::app::{
    AuditCardApp, DataRow, HeaderField, StatusLevel, StatusMessage, StyleHint,
    TabDef,
};
use umrs_ui::tabs::tabs_from_labels;

// ---------------------------------------------------------------------------
// Mock implementation
// ---------------------------------------------------------------------------

/// Minimal `AuditCardApp` implementation used exclusively for testing.
///
/// Provides two tabs of static data: "Summary" and "Details".
struct MockApp {
    tabs: Vec<TabDef>,
    status: StatusMessage,
}

impl MockApp {
    fn new() -> Self {
        Self {
            tabs: tabs_from_labels(&["Summary", "Details"]),
            status: StatusMessage::new(StatusLevel::Ok, "All checks passed"),
        }
    }
}

impl AuditCardApp for MockApp {
    fn report_name(&self) -> &'static str {
        "Test Report"
    }

    fn report_subject(&self) -> &'static str {
        "mock.host.example"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => vec![
                DataRow::normal("Kernel", "6.12.0"),
                DataRow::separator(),
                DataRow::new("SELinux", "Enforcing", StyleHint::TrustGreen),
            ],
            1 => vec![
                DataRow::normal("Arch", "aarch64"),
                DataRow::normal("FIPS", "active"),
            ],
            // Fail-closed: unknown tab returns empty — no panic, no garbage data
            _ => vec![],
        }
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }
}

// ---------------------------------------------------------------------------
// report_name / report_subject
// ---------------------------------------------------------------------------

#[test]
fn report_name_returns_expected_value() {
    let app = MockApp::new();
    assert_eq!(app.report_name(), "Test Report");
}

#[test]
fn report_subject_returns_expected_value() {
    let app = MockApp::new();
    assert_eq!(app.report_subject(), "mock.host.example");
}

// ---------------------------------------------------------------------------
// tabs
// ---------------------------------------------------------------------------

#[test]
fn tabs_returns_two_entries() {
    let app = MockApp::new();
    assert_eq!(app.tabs().len(), 2);
}

#[test]
fn tabs_first_label_is_summary() {
    let app = MockApp::new();
    assert_eq!(app.tabs()[0].label, "Summary");
}

#[test]
fn tabs_second_label_is_details() {
    let app = MockApp::new();
    assert_eq!(app.tabs()[1].label, "Details");
}

// ---------------------------------------------------------------------------
// active_tab
// ---------------------------------------------------------------------------

#[test]
fn active_tab_returns_zero() {
    let app = MockApp::new();
    assert_eq!(app.active_tab(), 0);
}

// ---------------------------------------------------------------------------
// data_rows — tab 0
// ---------------------------------------------------------------------------

#[test]
fn data_rows_tab_zero_returns_three_rows() {
    let app = MockApp::new();
    let rows = app.data_rows(0);
    assert_eq!(rows.len(), 3, "tab 0 must have 3 rows");
}

#[test]
fn data_rows_tab_zero_first_row_key_is_kernel() {
    let app = MockApp::new();
    let rows = app.data_rows(0);
    match &rows[0] {
        DataRow::KeyValue {
            key,
            ..
        } => assert_eq!(key, "Kernel"),
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

#[test]
fn data_rows_tab_zero_separator_is_separator_variant() {
    let app = MockApp::new();
    let rows = app.data_rows(0);
    assert!(
        matches!(rows[1], DataRow::Separator),
        "row[1] must be DataRow::Separator"
    );
}

#[test]
fn data_rows_tab_zero_selinux_row_has_trust_green_hint() {
    let app = MockApp::new();
    let rows = app.data_rows(0);
    match &rows[2] {
        DataRow::KeyValue {
            style_hint,
            ..
        } => {
            assert_eq!(*style_hint, StyleHint::TrustGreen);
        }
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// data_rows — tab 1
// ---------------------------------------------------------------------------

#[test]
fn data_rows_tab_one_returns_two_rows() {
    let app = MockApp::new();
    let rows = app.data_rows(1);
    assert_eq!(rows.len(), 2, "tab 1 must have 2 rows");
}

#[test]
fn data_rows_tab_one_arch_row_value() {
    let app = MockApp::new();
    let rows = app.data_rows(1);
    match &rows[0] {
        DataRow::KeyValue {
            value,
            ..
        } => assert_eq!(value, "aarch64"),
        other => panic!("expected DataRow::KeyValue, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// data_rows — invalid tab index (fail-closed)
// ---------------------------------------------------------------------------

#[test]
fn data_rows_invalid_tab_index_returns_empty_vec() {
    let app = MockApp::new();
    let rows = app.data_rows(99);
    assert!(
        rows.is_empty(),
        "invalid tab index must return empty Vec — fail closed, no panic"
    );
}

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

#[test]
fn status_returns_ok_level() {
    let app = MockApp::new();
    assert_eq!(app.status().level, StatusLevel::Ok);
}

#[test]
fn status_returns_expected_text() {
    let app = MockApp::new();
    assert_eq!(app.status().text, "All checks passed");
}

// ---------------------------------------------------------------------------
// Object safety — trait usable as &dyn AuditCardApp
// ---------------------------------------------------------------------------

#[test]
fn audit_card_app_is_object_safe() {
    let app = MockApp::new();
    // If AuditCardApp were not object-safe this would fail to compile.
    let dyn_app: &dyn AuditCardApp = &app;
    assert_eq!(dyn_app.report_name(), "Test Report");
    assert_eq!(dyn_app.tabs().len(), 2);
}

#[test]
fn dyn_audit_card_app_data_rows_works() {
    let app = MockApp::new();
    let dyn_app: &dyn AuditCardApp = &app;
    let rows = dyn_app.data_rows(0);
    assert_eq!(rows.len(), 3);
}

#[test]
fn dyn_audit_card_app_status_works() {
    let app = MockApp::new();
    let dyn_app: &dyn AuditCardApp = &app;
    assert_eq!(dyn_app.status().level, StatusLevel::Ok);
}

// ---------------------------------------------------------------------------
// HeaderField — Phase 2
// ---------------------------------------------------------------------------

/// Minimal `AuditCardApp` that overrides `header_fields()` with two fields.
struct MockAppWithFields {
    tabs: Vec<TabDef>,
    status: StatusMessage,
    fields: Vec<HeaderField>,
}

impl MockAppWithFields {
    fn new() -> Self {
        Self {
            tabs: tabs_from_labels(&["Summary"]),
            status: StatusMessage::new(StatusLevel::Info, "Ready"),
            fields: vec![
                HeaderField::normal("Version", "1.0.0"),
                HeaderField::new("Trust", "verified", StyleHint::TrustGreen),
            ],
        }
    }
}

impl AuditCardApp for MockAppWithFields {
    fn report_name(&self) -> &'static str {
        "Fields Test"
    }

    fn report_subject(&self) -> &'static str {
        "test.subject"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, _tab_index: usize) -> Vec<DataRow> {
        vec![]
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }

    fn header_fields(&self) -> &[HeaderField] {
        &self.fields
    }
}

#[test]
fn header_fields_default_returns_empty_slice() {
    // MockApp does not override header_fields; the default impl must return &[].
    let app = MockApp::new();
    assert!(
        app.header_fields().is_empty(),
        "default header_fields() must return an empty slice"
    );
}

#[test]
fn header_fields_override_returns_custom_fields() {
    let app = MockAppWithFields::new();
    assert_eq!(
        app.header_fields().len(),
        2,
        "overridden header_fields() must return the two configured fields"
    );
}

#[test]
fn header_field_new_sets_all_fields() {
    let field = HeaderField::new("Key", "Val", StyleHint::TrustRed);
    assert_eq!(field.label, "Key");
    assert_eq!(field.value, "Val");
    assert_eq!(field.style_hint, StyleHint::TrustRed);
}

#[test]
fn header_field_normal_sets_normal_hint() {
    let field = HeaderField::normal("Label", "value");
    assert_eq!(field.label, "Label");
    assert_eq!(field.value, "value");
    assert_eq!(field.style_hint, StyleHint::Normal);
}

// ---------------------------------------------------------------------------
// Three-tab mock — Phase 7 validation
// ---------------------------------------------------------------------------

/// Minimal `AuditCardApp` with three tabs to verify that a third tab
/// (Kernel Security placeholder) can be added without panics or fallback.
///
/// Mirrors the structure of `OsDetectApp` after Phase 7: tab 0 = OS info,
/// tab 1 = Trust / Evidence, tab 2 = Kernel Security. An invalid index
/// returns empty (fail-closed).
struct MockAppThreeTabs {
    tabs: Vec<TabDef>,
    status: StatusMessage,
}

impl MockAppThreeTabs {
    fn new() -> Self {
        Self {
            tabs: tabs_from_labels(&[
                "OS Information",
                "Trust / Evidence",
                "Kernel Security",
            ]),
            status: StatusMessage::new(StatusLevel::Ok, "Ready"),
        }
    }
}

impl AuditCardApp for MockAppThreeTabs {
    fn report_name(&self) -> &'static str {
        "Three Tab Test"
    }

    fn report_subject(&self) -> &'static str {
        "mock.host.example"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => vec![DataRow::normal("ID", "rhel")],
            1 => vec![DataRow::normal("trust_level", "T3")],
            2 => vec![
                DataRow::group_title("FIPS STATE"),
                DataRow::normal("fips_enabled", "active"),
            ],
            _ => vec![],
        }
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }
}

#[test]
fn three_tab_mock_has_three_tabs() {
    let app = MockAppThreeTabs::new();
    assert_eq!(app.tabs().len(), 3);
}

#[test]
fn three_tab_mock_tab_two_label_is_kernel_security() {
    let app = MockAppThreeTabs::new();
    assert_eq!(app.tabs()[2].label, "Kernel Security");
}

#[test]
fn three_tab_mock_data_rows_tab_two_returns_two_rows() {
    let app = MockAppThreeTabs::new();
    let rows = app.data_rows(2);
    assert_eq!(rows.len(), 2, "kernel security tab must have 2 rows");
}

#[test]
fn three_tab_mock_data_rows_tab_two_first_row_is_group_title() {
    let app = MockAppThreeTabs::new();
    let rows = app.data_rows(2);
    assert!(
        matches!(rows[0], DataRow::GroupTitle { .. }),
        "first row of kernel security tab must be a GroupTitle"
    );
}

#[test]
fn three_tab_mock_data_rows_invalid_index_returns_empty() {
    let app = MockAppThreeTabs::new();
    let rows = app.data_rows(3);
    assert!(
        rows.is_empty(),
        "invalid tab index must return empty Vec — fail closed, no panic"
    );
}

#[test]
fn three_tab_mock_is_object_safe_with_three_tabs() {
    let app = MockAppThreeTabs::new();
    let dyn_app: &dyn AuditCardApp = &app;
    assert_eq!(dyn_app.tabs().len(), 3);
    let rows = dyn_app.data_rows(2);
    assert_eq!(rows.len(), 2);
}
