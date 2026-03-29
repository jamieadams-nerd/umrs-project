// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for umrs-labels::validate.
//
// Exercises CuiPattern::CuiMarking valid/invalid cases.

use umrs_labels::validate::{CuiPattern, is_valid};

// ---------------------------------------------------------------------------
// Valid CuiMarking inputs
// ---------------------------------------------------------------------------

#[test]
fn valid_cui_base() {
    // "CUI" alone is not a valid marking by the pattern (requires at least one
    // //SEGMENT). Test what the pattern actually requires.
    // Pattern: ^CUI(//[A-Z][-A-Z]*)(/[A-Z][-A-Z]*)*$
    // "CUI" alone does not match — it requires at least one //SEGMENT.
    assert!(
        !is_valid(CuiPattern::CuiMarking, "CUI"),
        "bare CUI without category should not match"
    );
}

#[test]
fn valid_cui_single_segment() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//LEI"));
}

#[test]
fn valid_cui_single_segment_with_hyphen() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//SP-CTI"));
}

#[test]
fn valid_cui_two_segment_slash() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//LEI/JUV"));
}

#[test]
fn valid_cui_agr() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//AGR"));
}

#[test]
fn valid_cui_crit_ceii() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//CRIT/CEII"));
}

// ---------------------------------------------------------------------------
// Invalid CuiMarking inputs
// ---------------------------------------------------------------------------

#[test]
fn invalid_empty_string() {
    assert!(!is_valid(CuiPattern::CuiMarking, ""));
}

#[test]
fn invalid_lowercase_marking() {
    assert!(!is_valid(CuiPattern::CuiMarking, "cui//lei"));
}

#[test]
fn invalid_missing_double_slash() {
    // Single slash is not the correct separator for the first segment.
    assert!(!is_valid(CuiPattern::CuiMarking, "CUI/LEI"));
}

#[test]
fn invalid_trailing_slash() {
    assert!(!is_valid(CuiPattern::CuiMarking, "CUI//LEI/"));
}

#[test]
fn invalid_numeric_segment() {
    assert!(!is_valid(CuiPattern::CuiMarking, "CUI//123"));
}

#[test]
fn invalid_spaces_in_marking() {
    assert!(!is_valid(CuiPattern::CuiMarking, "CUI// LEI"));
}

#[test]
fn invalid_completely_random_string() {
    assert!(!is_valid(CuiPattern::CuiMarking, "not-a-marking"));
}

#[test]
fn invalid_double_slash_only() {
    assert!(!is_valid(CuiPattern::CuiMarking, "CUI//"));
}

// ---------------------------------------------------------------------------
// Pattern is callable multiple times (regex cache hit)
// ---------------------------------------------------------------------------

#[test]
fn pattern_is_idempotent_across_calls() {
    let input = "CUI//LEI";
    let first = is_valid(CuiPattern::CuiMarking, input);
    let second = is_valid(CuiPattern::CuiMarking, input);
    assert_eq!(first, second, "validation result must be stable");
    assert!(first);
}
