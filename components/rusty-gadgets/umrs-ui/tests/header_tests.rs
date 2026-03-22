// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for the header layout builders.
//!
//! Verifies the 6-row fixed layout produced by `build_two_column_lines` and
//! `build_single_column_lines` without requiring a terminal backend. Tests
//! call the builder functions directly and inspect the resulting `Line`
//! objects by concatenating span text.
//!
//! ## Layout (indices)
//!
//! ```text
//! lines[0]  blank
//! lines[1]  Assessment (full-width single column)
//! lines[2]  blank
//! lines[3]  Host | Tool         (two-col) / Host  (single-col)
//! lines[4]  OS   | Assessed
//! lines[5]  SELinux | FIPS
//! ```
//!
//! ## Test Coverage
//!
//! - Row count: both builders produce exactly 6 lines
//! - Field presence: each content row contains the expected labels and values
//! - Field ABSENCE: removed fields (Boot ID, System ID, LSM, Lockdown) must
//!   not appear anywhere in the header output
//! - Left padding: every content line begins with a space character
//!   (blank lines at indices 0 and 2 are skipped for this check)
//! - Column alignment: the ` : ` separator for left-column labels appears at
//!   the same character position on every content row (rows 1, 3, 4, 5 in
//!   two-col; rows 1, 3, 4, 5 in single-col)
//! - Combined value format: Row 1 value is exactly `"{report_name} / {report_subject}"`
//!   and Row 4 OS value is exactly `"{os_name} ({architecture})"`

use umrs_ui::Theme;
use umrs_ui::app::{
    AuditCardApp, AuditCardState, DataRow, HeaderContext, IndicatorValue,
    SecurityIndicators, StatusLevel, StatusMessage, TabDef,
};
use umrs_ui::header::{build_single_column_lines, build_two_column_lines};
use umrs_ui::tabs::tabs_from_labels;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Concatenate all span text in a `Line` into a single owned `String`.
///
/// This is the primary inspection tool for header line tests: it strips styling
/// information and produces the raw character content that would be rendered
/// on screen.
fn line_text(line: &ratatui::text::Line<'_>) -> String {
    line.spans.iter().map(|s| s.content.as_ref()).collect()
}

/// Build a `HeaderContext` with deterministic test values.
///
/// All fields have short, predictable values so tests can assert exact strings
/// without being sensitive to live system state.
fn test_header_context() -> HeaderContext {
    HeaderContext {
        indicators: SecurityIndicators {
            selinux_status: IndicatorValue::Enabled(
                "Enforcing (Targeted)".to_owned(),
            ),
            fips_mode: IndicatorValue::Enabled("Enabled".to_owned()),
            active_lsm: IndicatorValue::Unavailable,
            lockdown_mode: IndicatorValue::Disabled("none".to_owned()),
            secure_boot: IndicatorValue::Unavailable,
        },
        tool_name: "umrs-ui".to_owned(),
        tool_version: "0.1.0".to_owned(),
        assessed_at: "2026-03-15 19:31:54 UTC".to_owned(),
        hostname: "goldeneye".to_owned(),
        kernel_version: "6.12.0-211.el10.aarch64".to_owned(),
        architecture: "aarch64".to_owned(),
        boot_id: "a3f7c2d1-abcd-1234-ef56-000000000000".to_owned(),
        system_uuid: "550e8400-e29b-41d4-a716-446655440000".to_owned(),
        os_name: "RHEL 10.0".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Minimal AuditCardApp mock
// ---------------------------------------------------------------------------

/// Minimal `AuditCardApp` implementation used exclusively for header tests.
struct MockHeaderApp {
    tabs: Vec<TabDef>,
    status: StatusMessage,
}

impl MockHeaderApp {
    fn new() -> Self {
        Self {
            tabs: tabs_from_labels(&["Summary"]),
            status: StatusMessage::new(StatusLevel::Ok, "Ready"),
        }
    }
}

impl AuditCardApp for MockHeaderApp {
    fn report_name(&self) -> &'static str {
        "OS Detection"
    }

    fn report_subject(&self) -> &'static str {
        "Platform Identity and Integrity"
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
}

// Keep AuditCardState in scope to confirm it compiles — satisfies the
// object-safety proof that tests can use both types together.
#[allow(dead_code)]
fn _assert_state_compiles() {
    let _ = AuditCardState::new(1);
}

// ---------------------------------------------------------------------------
// Row count — two-column mode
// ---------------------------------------------------------------------------

#[test]
fn two_col_produces_exactly_six_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    assert_eq!(
        lines.len(),
        6,
        "two-column builder must produce exactly 6 fixed header lines"
    );
}

// ---------------------------------------------------------------------------
// Row count — single-column mode
// ---------------------------------------------------------------------------

#[test]
fn single_col_produces_exactly_six_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    assert_eq!(
        lines.len(),
        6,
        "single-column builder must produce exactly 6 fixed header lines"
    );
}

// ---------------------------------------------------------------------------
// Blank rows — indices 0 and 2 must be empty
// ---------------------------------------------------------------------------

#[test]
fn two_col_row0_is_blank() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[0]);
    assert!(
        text.is_empty(),
        "row 0 must be a blank spacer line; got: {text:?}"
    );
}

#[test]
fn two_col_row2_is_blank() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[2]);
    assert!(
        text.is_empty(),
        "row 2 must be a blank spacer line; got: {text:?}"
    );
}

#[test]
fn single_col_row0_is_blank() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[0]);
    assert!(
        text.is_empty(),
        "single-col row 0 must be a blank spacer line; got: {text:?}"
    );
}

#[test]
fn single_col_row2_is_blank() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[2]);
    assert!(
        text.is_empty(),
        "single-col row 2 must be a blank spacer line; got: {text:?}"
    );
}

// ---------------------------------------------------------------------------
// Row 1 — Assessment label and combined value
// ---------------------------------------------------------------------------

#[test]
fn two_col_row1_contains_assessment_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[1]);
    assert!(
        text.contains("Assessment"),
        "row 1 must contain the 'Assessment' label; got: {text:?}"
    );
}

#[test]
fn two_col_row1_contains_combined_assessment_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[1]);
    // Combined value must be exactly "{report_name} / {report_subject}"
    let expected = "OS Detection / Platform Identity and Integrity";
    assert!(
        text.contains(expected),
        "row 1 value must be '{expected}'; got: {text:?}"
    );
}

#[test]
fn single_col_row1_contains_assessment_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[1]);
    assert!(
        text.contains("Assessment"),
        "row 1 must contain 'Assessment' label; got: {text:?}"
    );
}

#[test]
fn single_col_row1_contains_combined_assessment_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[1]);
    let expected = "OS Detection / Platform Identity and Integrity";
    assert!(
        text.contains(expected),
        "row 1 value must be '{expected}'; got: {text:?}"
    );
}

// ---------------------------------------------------------------------------
// Row 3 — Host and Tool
// ---------------------------------------------------------------------------

#[test]
fn two_col_row3_contains_host_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[3]);
    assert!(
        text.contains("Host"),
        "row 3 must contain 'Host' label; got: {text:?}"
    );
}

#[test]
fn two_col_row3_contains_tool_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[3]);
    assert!(
        text.contains("Tool"),
        "row 3 must contain 'Tool' label; got: {text:?}"
    );
}

#[test]
fn two_col_row3_contains_hostname_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[3]);
    assert!(
        text.contains("goldeneye"),
        "row 3 must contain the hostname value; got: {text:?}"
    );
}

#[test]
fn two_col_row3_contains_tool_version_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[3]);
    // Tool value is formatted as "{tool_name} {tool_version}"
    assert!(
        text.contains("umrs-ui 0.1.0"),
        "row 3 must contain 'umrs-ui 0.1.0'; got: {text:?}"
    );
}

// ---------------------------------------------------------------------------
// Row 4 — OS and Assessed
// ---------------------------------------------------------------------------

#[test]
fn two_col_row4_contains_os_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[4]);
    assert!(
        text.contains("OS"),
        "row 4 must contain 'OS' label; got: {text:?}"
    );
}

#[test]
fn two_col_row4_contains_assessed_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[4]);
    assert!(
        text.contains("Assessed"),
        "row 4 must contain 'Assessed' label; got: {text:?}"
    );
}

#[test]
fn two_col_row4_os_value_is_name_and_arch() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[4]);
    // OS value must be formatted as "{os_name} ({architecture})"
    let expected = "RHEL 10.0 (aarch64)";
    assert!(
        text.contains(expected),
        "row 4 OS value must be '{expected}'; got: {text:?}"
    );
}

#[test]
fn two_col_row4_contains_assessed_at_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[4]);
    assert!(
        text.contains("2026-03-15 19:31:54 UTC"),
        "row 4 must contain the assessed_at timestamp; got: {text:?}"
    );
}

#[test]
fn single_col_row4_contains_os_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[4]);
    assert!(
        text.contains("OS"),
        "single-col row 4 must contain 'OS' label; got: {text:?}"
    );
}

#[test]
fn single_col_row4_os_value_is_name_and_arch() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[4]);
    let expected = "RHEL 10.0 (aarch64)";
    assert!(
        text.contains(expected),
        "single-col row 4 OS value must be '{expected}'; got: {text:?}"
    );
}

// ---------------------------------------------------------------------------
// Row 5 — SELinux and FIPS
// ---------------------------------------------------------------------------

#[test]
fn two_col_row5_contains_selinux_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[5]);
    assert!(
        text.contains("SELinux"),
        "row 5 must contain 'SELinux' label; got: {text:?}"
    );
}

#[test]
fn two_col_row5_contains_fips_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[5]);
    assert!(
        text.contains("FIPS"),
        "row 5 must contain 'FIPS' label; got: {text:?}"
    );
}

#[test]
fn two_col_row5_contains_selinux_indicator_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[5]);
    assert!(
        text.contains("Enforcing"),
        "row 5 must contain selinux indicator value 'Enforcing'; got: {text:?}"
    );
}

#[test]
fn two_col_row5_contains_fips_indicator_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[5]);
    assert!(
        text.contains("Enabled"),
        "row 5 must contain FIPS indicator value 'Enabled'; got: {text:?}"
    );
}

#[test]
fn single_col_row5_contains_selinux_label() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[5]);
    assert!(
        text.contains("SELinux"),
        "single-col row 5 must contain 'SELinux' label; got: {text:?}"
    );
}

#[test]
fn single_col_row5_contains_selinux_indicator_value() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[5]);
    assert!(
        text.contains("Enforcing"),
        "single-col row 5 must contain selinux indicator 'Enforcing'; got: {text:?}"
    );
}

// ---------------------------------------------------------------------------
// Field ABSENCE — removed fields must not appear anywhere
//
// These tests would have caught the regression if the old layout had been
// left in place. Boot ID, System ID, LSM, and Lockdown labels must be absent
// from all header lines.
// ---------------------------------------------------------------------------

#[test]
fn two_col_boot_id_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        assert!(
            !text.contains("Boot ID"),
            "row {i} must NOT contain 'Boot ID' — this field was removed from the header; \
             got: {text:?}"
        );
    }
}

#[test]
fn two_col_system_id_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        // "System ID" is the exact removed label.
        assert!(
            !text.contains("System ID"),
            "row {i} must NOT contain 'System ID' — this field was removed from the header; \
             got: {text:?}"
        );
    }
}

#[test]
fn two_col_lsm_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        // Check for " LSM " (padded) to avoid matching "SELinux" substring
        // or OS names that might incidentally contain "lsm".
        assert!(
            !text.contains(" LSM "),
            "row {i} must NOT contain the 'LSM' label — this field was removed; \
             got: {text:?}"
        );
    }
}

#[test]
fn two_col_lockdown_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        assert!(
            !text.contains("Lockdown"),
            "row {i} must NOT contain 'Lockdown' label — this field was removed; \
             got: {text:?}"
        );
    }
}

#[test]
fn single_col_boot_id_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        assert!(
            !text.contains("Boot ID"),
            "single-col row {i} must NOT contain 'Boot ID'; got: {text:?}"
        );
    }
}

#[test]
fn single_col_system_id_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        assert!(
            !text.contains("System ID"),
            "single-col row {i} must NOT contain 'System ID'; got: {text:?}"
        );
    }
}

#[test]
fn single_col_lsm_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        assert!(
            !text.contains(" LSM "),
            "single-col row {i} must NOT contain 'LSM' label; got: {text:?}"
        );
    }
}

#[test]
fn single_col_lockdown_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        assert!(
            !text.contains("Lockdown"),
            "single-col row {i} must NOT contain 'Lockdown' label; got: {text:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Left padding — every content line starts with a space
//
// Blank rows (indices 0 and 2) are spacers and contain no text; they are
// skipped. Content rows 1, 3, 4, 5 must each begin with a space.
// ---------------------------------------------------------------------------

/// Indices of content rows (non-blank) in the six-row header layout.
const CONTENT_ROWS: [usize; 4] = [1, 3, 4, 5];

#[test]
fn two_col_content_lines_start_with_space() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    for i in CONTENT_ROWS {
        let text = line_text(&lines[i]);
        assert!(
            text.starts_with(' '),
            "content row {i} must start with a space for left padding; got: {text:?}"
        );
    }
}

#[test]
fn single_col_content_lines_start_with_space() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    for i in CONTENT_ROWS {
        let text = line_text(&lines[i]);
        assert!(
            text.starts_with(' '),
            "single-col content row {i} must start with a space for left padding; got: {text:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Column alignment — left-column ` : ` separator at consistent position
//
// For content rows that have a left-column label (rows 1, 3, 4, 5 in both
// modes), the label prefix " {label:<10} : " is exactly 14 characters
// (1 + 10 + 3). Verify that the first occurrence of " : " in each labelled
// line appears at position 11 (0-indexed), consistent with a 10-char label
// preceded by 1 space: " Assessment : " → space at [0], label at [1..=10],
// space at [11], colon at [12], space at [13].
//
// Blank rows (indices 0 and 2) are excluded from this check.
// ---------------------------------------------------------------------------

/// Helper: find the byte position of the first " : " sequence in a string.
fn first_separator_pos(text: &str) -> Option<usize> {
    text.find(" : ")
}

#[test]
fn two_col_label_separator_at_consistent_position_content_rows() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);

    // Content rows 1, 3, 4, 5 all have a left-column label; the separator
    // position must be identical across all of them (determined by LABEL_WIDTH = 10).
    let positions: Vec<(usize, Option<usize>)> = CONTENT_ROWS
        .iter()
        .map(|&i| (i, first_separator_pos(&line_text(&lines[i]))))
        .collect();

    // Every content row must have a separator.
    for &(i, pos) in &positions {
        assert!(
            pos.is_some(),
            "content row {i} must contain a ' : ' separator; text: {:?}",
            line_text(&lines[i])
        );
    }

    // All separator positions must be equal — column alignment.
    let (first_row, first_pos) = positions[0];
    for &(i, pos) in positions.iter().skip(1) {
        assert_eq!(
            pos, first_pos,
            "content row {i} separator position {pos:?} must match row {first_row} \
             position {first_pos:?} — all left-column labels must align"
        );
    }
}

#[test]
fn single_col_label_separator_at_consistent_position_content_rows() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);

    let positions: Vec<(usize, Option<usize>)> = CONTENT_ROWS
        .iter()
        .map(|&i| (i, first_separator_pos(&line_text(&lines[i]))))
        .collect();

    for &(i, pos) in &positions {
        assert!(
            pos.is_some(),
            "single-col content row {i} must contain a ' : ' separator"
        );
    }

    let (first_row, first_pos) = positions[0];
    for &(i, pos) in positions.iter().skip(1) {
        assert_eq!(
            pos, first_pos,
            "single-col content row {i} separator at {pos:?} must match row {first_row} \
             at {first_pos:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Separator position value — sanity-check the expected column
//
// " Assessment : " → 1 space + 10 chars label + space = separator starts at
// character index 11 (the space before the colon in " : ").
// Row 1 (Assessment) carries the widest label and is the reference point.
// ---------------------------------------------------------------------------

#[test]
fn two_col_separator_position_is_eleven() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    // Row 1 uses the Assessment label (10 chars, the widest label).
    let text = line_text(&lines[1]);
    let pos = first_separator_pos(&text);
    assert_eq!(
        pos,
        Some(11),
        "separator must appear at position 11 (1 left pad + 10 label chars); \
         got {pos:?} in: {text:?}"
    );
}

#[test]
fn single_col_separator_position_is_eleven() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_single_column_lines(&app, &ctx, &theme);
    // Row 1 (Assessment) is the reference.
    let text = line_text(&lines[1]);
    let pos = first_separator_pos(&text);
    assert_eq!(
        pos,
        Some(11),
        "single-col separator must appear at position 11; got {pos:?} in: {text:?}"
    );
}

// ---------------------------------------------------------------------------
// Scope label absence (old field, now removed from the header)
// ---------------------------------------------------------------------------

#[test]
fn two_col_scope_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        // "Scope" was the old row-2 label — must not appear in the new layout.
        assert!(
            !text.contains("Scope"),
            "row {i} must NOT contain 'Scope' label — this field was removed; got: {text:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Kernel label absence (old field, now removed from the header)
// ---------------------------------------------------------------------------

#[test]
fn two_col_kernel_label_absent_from_all_lines() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    for (i, line) in lines.iter().enumerate() {
        let text = line_text(line);
        // "Kernel" was the old row-3 right-column label.
        assert!(
            !text.contains("Kernel"),
            "row {i} must NOT contain 'Kernel' label — this field was removed; got: {text:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Right-column alignment — the right label must start at the same position
// across all two-column rows (rows 3–5).
//
// Row 1 is single-column (Assessment) so it has no right column.
// Rows 3–5 each have a right column label introduced by "  {label:<10} : ".
// The right column start position is determined by the left half width:
// 1 (pad) + LABEL_WIDTH (10) + " : " (3) + LEFT_VALUE_WIDTH (35) = 49.
// The right label prefix "  " starts at character 49, so the right label's
// " : " separator appears at 49 + 2 + 10 = 61.
//
// This test catches the exact bug where `two_col_indicator_line` did not
// pad the left indicator value to LEFT_VALUE_WIDTH, causing the right
// column to start at a different position than rows built by `two_col_line`.
// ---------------------------------------------------------------------------

/// Helper: find the position of the SECOND " : " in a line (the right-column separator).
fn second_separator_pos(text: &str) -> Option<usize> {
    let first = text.find(" : ")?;
    text[first + 3..].find(" : ").map(|p| p + first + 3)
}

#[test]
fn two_col_right_column_separator_at_consistent_position() {
    let app = MockHeaderApp::new();
    let ctx = test_header_context();
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);

    // Rows 3–5 are two-column lines; collect their right-column separator positions.
    let positions: Vec<Option<usize>> = lines[3..=5]
        .iter()
        .map(|l| second_separator_pos(&line_text(l)))
        .collect();

    // Every two-column row must have a second separator.
    for (i, pos) in positions.iter().enumerate() {
        let row_idx = i + 3;
        assert!(
            pos.is_some(),
            "row {row_idx} must contain a right-column ' : ' separator; text: {:?}",
            line_text(&lines[row_idx])
        );
    }

    // All right-column separator positions must be equal.
    let first = positions[0];
    for (i, pos) in positions.iter().enumerate().skip(1) {
        let row_idx = i + 3;
        assert_eq!(
            *pos, first,
            "row {row_idx} right-column separator at {pos:?} must match row 3 at {first:?} — \
             right column must be aligned across all two-column rows"
        );
    }
}

// ---------------------------------------------------------------------------
// Unavailable indicator renders as "unavailable" string
// ---------------------------------------------------------------------------

#[test]
fn two_col_unavailable_indicator_renders_as_unavailable_text() {
    let app = MockHeaderApp::new();
    let mut ctx = test_header_context();
    // Replace selinux_status with Unavailable to exercise that branch.
    ctx.indicators.selinux_status = IndicatorValue::Unavailable;
    let theme = Theme::default();
    let lines = build_two_column_lines(&app, &ctx, &theme);
    let text = line_text(&lines[5]);
    assert!(
        text.contains("unavailable"),
        "row 5 with Unavailable selinux_status must show 'unavailable'; got: {text:?}"
    );
}
