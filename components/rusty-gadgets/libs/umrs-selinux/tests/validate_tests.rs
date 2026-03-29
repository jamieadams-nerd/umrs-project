// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for umrs-selinux::validate.
//
// Exercises SelinuxPattern::SelinuxContext and SelinuxPattern::MlsRange
// valid/invalid cases.

use umrs_selinux::validate::{SelinuxPattern, is_valid};

// ---------------------------------------------------------------------------
// SelinuxContext — valid inputs
// ---------------------------------------------------------------------------

#[test]
fn valid_context_standard_form() {
    assert!(is_valid(
        SelinuxPattern::SelinuxContext,
        "system_u:system_r:httpd_t:s0"
    ));
}

#[test]
fn valid_context_unconfined() {
    assert!(is_valid(
        SelinuxPattern::SelinuxContext,
        "unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023"
    ));
}

#[test]
fn valid_context_with_mls_range() {
    assert!(is_valid(
        SelinuxPattern::SelinuxContext,
        "user_u:user_r:user_t:s0:c0,c5"
    ));
}

#[test]
fn valid_context_minimal_nonempty_fields() {
    // All four fields non-empty, minimal content.
    assert!(is_valid(SelinuxPattern::SelinuxContext, "u:r:t:s0"));
}

// ---------------------------------------------------------------------------
// SelinuxContext — invalid inputs
// ---------------------------------------------------------------------------

#[test]
fn invalid_context_empty_string() {
    assert!(!is_valid(SelinuxPattern::SelinuxContext, ""));
}

#[test]
fn invalid_context_only_three_fields() {
    // Exactly three colons required (four fields). Three fields = two colons.
    assert!(!is_valid(SelinuxPattern::SelinuxContext, "u:r:t"));
}

#[test]
fn invalid_context_empty_field() {
    // An empty field between colons should not match.
    assert!(!is_valid(SelinuxPattern::SelinuxContext, "u::t:s0"));
}

#[test]
fn invalid_context_no_colons() {
    assert!(!is_valid(SelinuxPattern::SelinuxContext, "httpd_t"));
}

// ---------------------------------------------------------------------------
// MlsRange — valid inputs
// ---------------------------------------------------------------------------

#[test]
fn valid_mls_sensitivity_only() {
    assert!(is_valid(SelinuxPattern::MlsRange, "s0"));
}

#[test]
fn valid_mls_with_single_category() {
    assert!(is_valid(SelinuxPattern::MlsRange, "s0:c0"));
}

#[test]
fn valid_mls_with_multiple_categories() {
    assert!(is_valid(SelinuxPattern::MlsRange, "s0:c0,c5"));
}

#[test]
fn valid_mls_higher_sensitivity() {
    assert!(is_valid(SelinuxPattern::MlsRange, "s3:c10,c20,c100"));
}

#[test]
fn valid_mls_range_low_high() {
    // MLS range: low-high form (s0-s3:c0).
    assert!(is_valid(SelinuxPattern::MlsRange, "s0-s3:c0"));
}

#[test]
fn valid_mls_range_both_with_categories() {
    assert!(is_valid(SelinuxPattern::MlsRange, "s0:c0-s3:c0,c5"));
}

// ---------------------------------------------------------------------------
// MlsRange — invalid inputs
// ---------------------------------------------------------------------------

#[test]
fn invalid_mls_empty_string() {
    assert!(!is_valid(SelinuxPattern::MlsRange, ""));
}

#[test]
fn invalid_mls_no_sensitivity_prefix() {
    // Must start with 's' followed by digits.
    assert!(!is_valid(SelinuxPattern::MlsRange, "0:c0"));
}

#[test]
fn invalid_mls_c_without_number() {
    assert!(!is_valid(SelinuxPattern::MlsRange, "s0:c"));
}

#[test]
fn invalid_mls_alphabetic_level() {
    assert!(!is_valid(SelinuxPattern::MlsRange, "secret"));
}

#[test]
fn invalid_mls_spaces() {
    assert!(!is_valid(SelinuxPattern::MlsRange, "s0 :c0"));
}

// ---------------------------------------------------------------------------
// Regex cache stability
// ---------------------------------------------------------------------------

#[test]
fn selinux_context_validation_is_idempotent() {
    let input = "system_u:system_r:httpd_t:s0";
    let first = is_valid(SelinuxPattern::SelinuxContext, input);
    let second = is_valid(SelinuxPattern::SelinuxContext, input);
    assert_eq!(first, second);
    assert!(first);
}

#[test]
fn mls_range_validation_is_idempotent() {
    let input = "s0:c0,c5";
    let first = is_valid(SelinuxPattern::MlsRange, input);
    let second = is_valid(SelinuxPattern::MlsRange, input);
    assert_eq!(first, second);
    assert!(first);
}
