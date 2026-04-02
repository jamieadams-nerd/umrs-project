// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for [`ColumnLayout`] and the two-column trait methods on
//! [`AuditCardApp`].
//!
//! Tests cover:
//! - `ColumnLayout` variant equality and the `Default` impl.
//! - Default trait method behaviour: `column_layout()` returns `Full`,
//!   `data_rows_left()` / `data_rows_right()` return empty `Vec`.
//! - Override behaviour: a custom impl returning `TwoColumn` propagates
//!   correctly and the per-column row lists are independent.
//!
//! All tests are pure logic — no terminal backend required.

use umrs_ui::app::{
    AuditCardApp, AuditCardState, ColumnLayout, DataRow, HeaderField, StatusMessage, StyleHint,
    TabDef,
};
use umrs_ui::keymap::Action;

// ---------------------------------------------------------------------------
// ColumnLayout — basic invariants
// ---------------------------------------------------------------------------

#[test]
fn column_layout_default_is_full() {
    let layout = ColumnLayout::default();
    assert_eq!(
        layout,
        ColumnLayout::Full,
        "ColumnLayout::default() must be Full (backward-compatible baseline)"
    );
}

#[test]
fn column_layout_variants_are_distinct() {
    assert_ne!(
        ColumnLayout::Full,
        ColumnLayout::TwoColumn,
        "Full and TwoColumn must be distinct variants"
    );
}

#[test]
fn column_layout_full_matches_full() {
    let layout = ColumnLayout::Full;
    assert!(
        matches!(layout, ColumnLayout::Full),
        "ColumnLayout::Full must match the Full pattern"
    );
}

#[test]
fn column_layout_two_column_matches_two_column() {
    let layout = ColumnLayout::TwoColumn;
    assert!(
        matches!(layout, ColumnLayout::TwoColumn),
        "ColumnLayout::TwoColumn must match the TwoColumn pattern"
    );
}

// ---------------------------------------------------------------------------
// Minimal AuditCardApp impl for default-trait-method testing
// ---------------------------------------------------------------------------

/// Minimal single-tab app that uses all default trait method implementations.
///
/// Returned by `MinimalApp::new()`. Allows testing default behaviour
/// without wiring up platform crates.
struct MinimalApp {
    tabs: [TabDef; 1],
    rows: Vec<DataRow>,
    status: StatusMessage,
}

impl MinimalApp {
    fn new() -> Self {
        Self {
            tabs: [TabDef::new("Tab 0")],
            rows: vec![DataRow::normal("key", "value")],
            status: StatusMessage::default(),
        }
    }
}

impl AuditCardApp for MinimalApp {
    fn report_name(&self) -> &'static str {
        "Test Report"
    }

    fn report_subject(&self) -> &'static str {
        "test subject"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, _tab_index: usize) -> Vec<DataRow> {
        self.rows.clone()
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }
}

#[test]
fn default_column_layout_returns_full() {
    let app = MinimalApp::new();
    assert_eq!(
        app.column_layout(0),
        ColumnLayout::Full,
        "default column_layout() must return Full"
    );
}

#[test]
fn default_data_rows_left_returns_empty() {
    let app = MinimalApp::new();
    let left = app.data_rows_left(0);
    assert!(
        left.is_empty(),
        "default data_rows_left() must return an empty Vec"
    );
}

#[test]
fn default_data_rows_right_returns_empty() {
    let app = MinimalApp::new();
    let right = app.data_rows_right(0);
    assert!(
        right.is_empty(),
        "default data_rows_right() must return an empty Vec"
    );
}

#[test]
fn default_pinned_rows_returns_empty() {
    let app = MinimalApp::new();
    assert!(
        app.pinned_rows(0).is_empty(),
        "default pinned_rows() must return an empty Vec"
    );
}

#[test]
fn default_header_fields_returns_empty() {
    let app = MinimalApp::new();
    let fields: &[HeaderField] = app.header_fields();
    assert!(
        fields.is_empty(),
        "default header_fields() must return an empty slice"
    );
}

// ---------------------------------------------------------------------------
// Two-column override — verifies propagation of left/right rows
// ---------------------------------------------------------------------------

/// App that overrides column_layout() to return TwoColumn for tab 0 and Full
/// for tab 1. Returns distinct row lists for left and right columns.
struct TwoColumnApp {
    tabs: [TabDef; 2],
    left: Vec<DataRow>,
    right: Vec<DataRow>,
    single: Vec<DataRow>,
    status: StatusMessage,
}

impl TwoColumnApp {
    fn new() -> Self {
        Self {
            tabs: [TabDef::new("Two-Col"), TabDef::new("Single-Col")],
            left: vec![
                DataRow::normal("left-key-1", "left-val-1"),
                DataRow::normal("left-key-2", "left-val-2"),
            ],
            right: vec![DataRow::new("right-key-1", "right-val-1", StyleHint::Highlight)],
            single: vec![DataRow::normal("single-key", "single-val")],
            status: StatusMessage::default(),
        }
    }
}

impl AuditCardApp for TwoColumnApp {
    fn report_name(&self) -> &'static str {
        "TwoColumn Test"
    }

    fn report_subject(&self) -> &'static str {
        "two-column layout test"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            1 => self.single.clone(),
            _ => Vec::new(), // two-column path, not used
        }
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }

    fn column_layout(&self, tab_index: usize) -> ColumnLayout {
        match tab_index {
            0 => ColumnLayout::TwoColumn,
            _ => ColumnLayout::Full,
        }
    }

    fn data_rows_left(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => self.left.clone(),
            _ => Vec::new(),
        }
    }

    fn data_rows_right(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => self.right.clone(),
            _ => Vec::new(),
        }
    }
}

#[test]
fn two_column_app_tab0_layout_is_two_column() {
    let app = TwoColumnApp::new();
    assert_eq!(
        app.column_layout(0),
        ColumnLayout::TwoColumn,
        "tab 0 must return TwoColumn layout"
    );
}

#[test]
fn two_column_app_tab1_layout_is_full() {
    let app = TwoColumnApp::new();
    assert_eq!(
        app.column_layout(1),
        ColumnLayout::Full,
        "tab 1 must return Full layout"
    );
}

#[test]
fn two_column_app_left_rows_returned_for_tab0() {
    let app = TwoColumnApp::new();
    let left = app.data_rows_left(0);
    assert_eq!(
        left.len(),
        2,
        "tab 0 left column must return the two configured rows"
    );
    match &left[0] {
        DataRow::KeyValue {
            key,
            value,
            ..
        } => {
            assert_eq!(key, "left-key-1");
            assert_eq!(value, "left-val-1");
        }
        other => panic!("expected KeyValue, got {other:?}"),
    }
}

#[test]
fn two_column_app_right_rows_returned_for_tab0() {
    let app = TwoColumnApp::new();
    let right = app.data_rows_right(0);
    assert_eq!(
        right.len(),
        1,
        "tab 0 right column must return the one configured row"
    );
    match &right[0] {
        DataRow::KeyValue {
            key,
            value,
            style_hint,
            ..
        } => {
            assert_eq!(key, "right-key-1");
            assert_eq!(value, "right-val-1");
            assert_eq!(*style_hint, StyleHint::Highlight);
        }
        other => panic!("expected KeyValue, got {other:?}"),
    }
}

#[test]
fn two_column_app_left_rows_empty_for_tab1() {
    let app = TwoColumnApp::new();
    let left = app.data_rows_left(1);
    assert!(
        left.is_empty(),
        "tab 1 left column must return empty Vec (single-column tab)"
    );
}

#[test]
fn two_column_app_right_rows_empty_for_tab1() {
    let app = TwoColumnApp::new();
    let right = app.data_rows_right(1);
    assert!(
        right.is_empty(),
        "tab 1 right column must return empty Vec (single-column tab)"
    );
}

#[test]
fn two_column_app_left_right_are_independent() {
    let app = TwoColumnApp::new();
    let left = app.data_rows_left(0);
    let right = app.data_rows_right(0);

    // They must have different lengths — 2 vs 1 in this test fixture.
    assert_ne!(
        left.len(),
        right.len(),
        "left and right column row lists are independent"
    );
}

// ---------------------------------------------------------------------------
// AuditCardState — column layout does not affect state machine
// ---------------------------------------------------------------------------

#[test]
fn audit_card_state_scroll_works_with_two_column_app() {
    // AuditCardState is layout-agnostic — scroll offset applies to both columns.
    let mut state = AuditCardState::new(2);
    assert_eq!(state.scroll_offset, 0, "initial scroll offset must be 0");

    state.handle_action(&Action::ScrollDown);
    assert_eq!(
        state.scroll_offset, 1,
        "scroll down must increment offset by 1"
    );

    // Tab switch resets scroll offset regardless of layout mode.
    state.handle_action(&Action::NextTab);
    assert_eq!(
        state.scroll_offset, 0,
        "tab switch must reset scroll offset to 0"
    );
}
