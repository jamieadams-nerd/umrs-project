// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Tests for `umrs_ui::text_fit` — width-aware left truncation helper.
// Covers the width-boundary edges, multi-byte UTF-8 safety, wide-glyph
// counting, and the zero-width degenerate cases.

use umrs_ui::text_fit::{display_width, truncate_left, truncate_right};

const ELLIPSIS: &str = "\u{2026}";

// ----------------------------------------------------------------------------
// No-op cases: input already fits
// ----------------------------------------------------------------------------

#[test]
fn short_input_returns_unchanged_when_fits() {
    assert_eq!(truncate_left("hello", 10), "hello");
}

#[test]
fn exact_fit_returns_unchanged() {
    assert_eq!(truncate_left("hello", 5), "hello");
}

#[test]
fn empty_input_returns_empty() {
    assert_eq!(truncate_left("", 10), "");
}

// ----------------------------------------------------------------------------
// Truncation cases
// ----------------------------------------------------------------------------

#[test]
fn truncates_from_left_with_ellipsis_prefix() {
    // "hello world" is 11 cells, budget 6 → ellipsis + 5 tail chars.
    let result = truncate_left("hello world", 6);
    assert_eq!(result, format!("{ELLIPSIS}world"));
    assert_eq!(display_width(&result), 6);
}

#[test]
fn path_truncation_preserves_leaf() {
    // Operator-relevant case: a long path keeps the filename.
    let path = "/DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-ls/src/main.rs";
    let result = truncate_left(path, 30);
    assert!(result.starts_with(ELLIPSIS));
    assert!(result.ends_with("main.rs"));
    assert_eq!(display_width(&result), 30);
}

#[test]
fn truncation_result_never_exceeds_budget() {
    for budget in 2..50 {
        let result = truncate_left("abcdefghijklmnopqrstuvwxyz", budget);
        assert!(
            display_width(&result) <= budget,
            "budget {budget}: result {result:?} is {} cells",
            display_width(&result)
        );
    }
}

// ----------------------------------------------------------------------------
// Degenerate budgets
// ----------------------------------------------------------------------------

#[test]
fn zero_budget_returns_empty() {
    assert_eq!(truncate_left("hello", 0), "");
}

#[test]
fn budget_of_one_returns_just_ellipsis() {
    assert_eq!(truncate_left("hello", 1), ELLIPSIS);
}

#[test]
fn budget_equal_to_ellipsis_width_returns_ellipsis() {
    assert_eq!(truncate_left("long string here", 1), ELLIPSIS);
}

// ----------------------------------------------------------------------------
// UTF-8 safety — must never split a multi-byte sequence
// ----------------------------------------------------------------------------

#[test]
fn multibyte_utf8_never_splits_at_boundary() {
    // "café résumé" — the `é` characters are 2 bytes each but 1 cell wide.
    let input = "café résumé";
    for budget in 1..=display_width(input) {
        let result = truncate_left(input, budget);
        assert!(
            result.is_char_boundary(result.len()),
            "budget {budget}: {result:?} ended mid-codepoint"
        );
        assert!(
            display_width(&result) <= budget,
            "budget {budget}: {result:?} overflows"
        );
    }
}

// ----------------------------------------------------------------------------
// Wide-glyph counting (CJK / emoji occupying 2 cells)
// ----------------------------------------------------------------------------

#[test]
fn wide_glyphs_counted_as_two_cells() {
    // `漢` is a single codepoint that occupies 2 terminal cells.
    let input = "abc漢def";
    // 3 + 2 + 3 = 8 cells.
    assert_eq!(display_width(input), 8);
    // With budget 5, we drop from the left; must not split the wide glyph.
    let result = truncate_left(input, 5);
    assert!(display_width(&result) <= 5);
    assert!(result.starts_with(ELLIPSIS));
}

// ============================================================================
// truncate_right — preserves the start of the input, drops the tail
// ============================================================================

#[test]
fn right_short_input_returns_unchanged_when_fits() {
    assert_eq!(truncate_right("hello", 10), "hello");
}

#[test]
fn right_exact_fit_returns_unchanged() {
    assert_eq!(truncate_right("hello", 5), "hello");
}

#[test]
fn right_empty_input_returns_empty() {
    assert_eq!(truncate_right("", 10), "");
}

#[test]
fn right_truncates_with_ellipsis_suffix() {
    let result = truncate_right("hello world", 6);
    assert_eq!(result, format!("hello{ELLIPSIS}"));
    assert_eq!(display_width(&result), 6);
}

#[test]
fn right_preserves_start_of_hostname() {
    let host = "host123.datacenter.corp.example.com";
    let result = truncate_right(host, 14);
    assert!(result.starts_with("host123"));
    assert!(result.ends_with(ELLIPSIS));
    assert_eq!(display_width(&result), 14);
}

#[test]
fn right_zero_budget_returns_empty() {
    assert_eq!(truncate_right("hello", 0), "");
}

#[test]
fn right_budget_of_one_returns_just_ellipsis() {
    assert_eq!(truncate_right("hello", 1), ELLIPSIS);
}

#[test]
fn right_result_never_exceeds_budget() {
    for budget in 2..50 {
        let result = truncate_right("abcdefghijklmnopqrstuvwxyz", budget);
        assert!(
            display_width(&result) <= budget,
            "budget {budget}: result {result:?} overflows"
        );
    }
}

#[test]
fn right_multibyte_utf8_never_splits_at_boundary() {
    let input = "café résumé";
    for budget in 1..=display_width(input) {
        let result = truncate_right(input, budget);
        assert!(
            result.is_char_boundary(result.len()),
            "budget {budget}: {result:?} ended mid-codepoint"
        );
        assert!(
            display_width(&result) <= budget,
            "budget {budget}: {result:?} overflows"
        );
    }
}

#[test]
fn right_wide_glyphs_counted_as_two_cells() {
    let input = "abc漢def";
    let result = truncate_right(input, 5);
    assert!(display_width(&result) <= 5);
    assert!(result.ends_with(ELLIPSIS));
}
