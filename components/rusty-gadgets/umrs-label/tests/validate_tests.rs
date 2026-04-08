// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for umrs-labels::validate.
//
// Exercises CuiPattern::CuiMarking valid/invalid cases.
// Covers the full NARA banner syntax: plain CUI, categories with SP- prefix,
// multi-category banners, and LDC suffixes (NOFORN, FED ONLY, REL TO lists).

use umrs_labels::validate::{CuiPattern, is_valid};

// ---------------------------------------------------------------------------
// Valid CuiMarking inputs
// ---------------------------------------------------------------------------

#[test]
fn valid_cui_base() {
    // Plain "CUI" with no category is a valid marking under NARA rules.
    assert!(is_valid(CuiPattern::CuiMarking, "CUI"), "bare CUI should be valid");
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
// LDC banner inputs (second // block)
// ---------------------------------------------------------------------------

#[test]
fn valid_cui_with_ldc_noforn() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//SP-CTI/EXPT//NOFORN"));
}

/// CUI can stand alone as a generic designation. When combined directly
/// with an LDC (no category), the marking is `CUI//LDC` — e.g.,
/// `CUI//NOFORN`. This is valid per 32 CFR Part 2002.
#[test]
fn valid_cui_bare_with_ldc() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//NOFORN"));
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//FED ONLY"));
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//FEDCON"));
}

#[test]
fn valid_cui_with_ldc_fed_only() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//LEI//FED ONLY"));
}

#[test]
fn valid_cui_with_ldc_rel_to_list() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//SP-CTI//REL TO USA, CAN, GBR"));
}

#[test]
fn valid_cui_single_cat_with_ldc() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//CTI//NOFORN"));
}

#[test]
fn valid_cui_sp_prefix_with_ldc() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//SP-CTI//NOFORN"));
}

#[test]
fn valid_cui_mixed_sp_and_basic_with_ldc() {
    assert!(is_valid(CuiPattern::CuiMarking, "CUI//SP-CTI/EXPT//NOFORN"));
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
