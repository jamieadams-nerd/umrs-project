// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// -----------------------------------------------------------------------------
// TPI Error Enum — Integration Tests
//
// Covers the three TpiError variants and the XattrReadError type.
// Verifies that:
//   - Variant construction and Display work correctly
//   - PathAFailed and PathBFailed never produce "critical"-class messaging
//   - Disagreement carries both contexts
//   - XattrReadError From<io::Error> and From<TpiError> conversions work
//   - nom_error_kind never includes raw input bytes (SI-12)
//
// NIST SP 800-53 SI-7 / SI-12 / NSA RTB RAIN.
// -----------------------------------------------------------------------------

use std::fmt::Write as _;

use umrs_selinux::xattrs::{TpiError, XattrReadError, nom_error_kind};
use umrs_selinux::{SecurityContext, SelinuxRole, SelinuxType, SelinuxUser};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_context(user: &str, role: &str, ty: &str) -> SecurityContext {
    let u: SelinuxUser = user.parse().expect("valid user");
    let r: SelinuxRole = role.parse().expect("valid role");
    let t: SelinuxType = ty.parse().expect("valid type");
    SecurityContext::new(u, r, t, None)
}

// ---------------------------------------------------------------------------
// TpiError::PathAFailed
// ---------------------------------------------------------------------------

#[test]
fn tpi_path_a_failed_display_contains_reason() {
    let err = TpiError::PathAFailed("nom::Tag".to_owned());
    let s = err.to_string();
    assert!(s.contains("Path A"), "Display should mention Path A: {s}");
    assert!(
        s.contains("nom::Tag"),
        "Display should include the reason: {s}"
    );
}

#[test]
fn tpi_path_a_failed_display_does_not_say_critical() {
    // Single-path failure must never look like an integrity event in its
    // Display output — it is a code defect.
    let err = TpiError::PathAFailed("nom::Tag".to_owned());
    let s = err.to_string().to_lowercase();
    assert!(
        !s.contains("critical"),
        "PathAFailed must not contain 'critical' in display: {s}"
    );
    assert!(
        !s.contains("integrity"),
        "PathAFailed must not contain 'integrity' in display: {s}"
    );
}

// ---------------------------------------------------------------------------
// TpiError::PathBFailed
// ---------------------------------------------------------------------------

#[test]
fn tpi_path_b_failed_display_contains_reason() {
    let err = TpiError::PathBFailed("invalid-input".to_owned());
    let s = err.to_string();
    assert!(s.contains("Path B"), "Display should mention Path B: {s}");
    assert!(
        s.contains("invalid-input"),
        "Display should include the reason: {s}"
    );
}

#[test]
fn tpi_path_b_failed_does_not_say_critical() {
    let err = TpiError::PathBFailed("parse-error".to_owned());
    let s = err.to_string().to_lowercase();
    assert!(
        !s.contains("critical"),
        "PathBFailed must not contain 'critical' in display: {s}"
    );
}

// ---------------------------------------------------------------------------
// TpiError::Disagreement
// ---------------------------------------------------------------------------

#[test]
fn tpi_disagreement_carries_both_contexts() {
    let ctx_a = make_context("system_u", "system_r", "sshd_t");
    let ctx_b = make_context("unconfined_u", "unconfined_r", "unconfined_t");

    let err = TpiError::Disagreement(
        Box::new(ctx_a.clone()),
        Box::new(ctx_b.clone()),
    );

    if let TpiError::Disagreement(a, b) = &err {
        assert_eq!(a.as_ref(), &ctx_a, "Path A context should be preserved");
        assert_eq!(b.as_ref(), &ctx_b, "Path B context should be preserved");
    } else {
        panic!("Expected Disagreement variant");
    }
}

#[test]
fn tpi_disagreement_display_does_not_leak_sensitivity_data() {
    // The Display impl for Disagreement must not include the full security
    // context representations (which may contain MLS levels / categories).
    // It should only mention the disagreement, not the context values.
    let ctx_a = make_context("system_u", "system_r", "sshd_t");
    let ctx_b = make_context("unconfined_u", "unconfined_r", "unconfined_t");

    let err = TpiError::Disagreement(Box::new(ctx_a), Box::new(ctx_b));

    let s = err.to_string();
    // The Display should mention disagreement but not dump full context fields.
    assert!(
        s.contains("disagreement") || s.contains("different"),
        "Disagreement display should describe the condition: {s}"
    );
    // Should not include raw user/role/type values from the contexts.
    assert!(
        !s.contains("sshd_t"),
        "Disagreement display must not include raw type values: {s}"
    );
    assert!(
        !s.contains("unconfined_t"),
        "Disagreement display must not include raw type values: {s}"
    );
}

// ---------------------------------------------------------------------------
// TpiError equality and clone
// ---------------------------------------------------------------------------

#[test]
fn tpi_error_path_a_equality() {
    let a = TpiError::PathAFailed("nom::Tag".to_owned());
    let b = TpiError::PathAFailed("nom::Tag".to_owned());
    let c = TpiError::PathAFailed("nom::Eof".to_owned());
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn tpi_error_path_b_equality() {
    let a = TpiError::PathBFailed("invalid-input".to_owned());
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn tpi_error_disagreement_equality() {
    let ctx_a = make_context("system_u", "system_r", "sshd_t");
    let ctx_b = make_context("unconfined_u", "unconfined_r", "unconfined_t");

    let d1 = TpiError::Disagreement(
        Box::new(ctx_a.clone()),
        Box::new(ctx_b.clone()),
    );
    let d2 = TpiError::Disagreement(
        Box::new(ctx_a.clone()),
        Box::new(ctx_b.clone()),
    );
    assert_eq!(d1, d2);
}

// ---------------------------------------------------------------------------
// XattrReadError
// ---------------------------------------------------------------------------

#[test]
fn xattr_read_error_from_io_error() {
    use std::io;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "ENODATA");
    let xattr_err = XattrReadError::from(io_err);
    assert!(
        matches!(xattr_err, XattrReadError::OsError(_)),
        "io::Error should convert to OsError variant"
    );
}

#[test]
fn xattr_read_error_from_tpi_error() {
    let tpi = TpiError::PathAFailed("nom::Tag".to_owned());
    let xattr_err = XattrReadError::from(tpi);
    assert!(
        matches!(xattr_err, XattrReadError::Tpi(_)),
        "TpiError should convert to Tpi variant"
    );
}

#[test]
fn xattr_read_error_os_display() {
    use std::io;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "ENODATA");
    let xattr_err = XattrReadError::OsError(io_err);
    let s = xattr_err.to_string();
    assert!(
        s.contains("OS error") || s.contains("xattr"),
        "OsError display should be descriptive: {s}"
    );
}

#[test]
fn xattr_read_error_tpi_display() {
    let tpi = TpiError::PathBFailed("parse-error".to_owned());
    let xattr_err = XattrReadError::Tpi(tpi);
    let s = xattr_err.to_string();
    assert!(
        s.contains("Path B"),
        "Tpi display should delegate to TpiError display: {s}"
    );
}

// ---------------------------------------------------------------------------
// nom_error_kind — SI-12 compliance
//
// Verify the function never includes raw input bytes in its output.
// nom::Err::Display does include the verbatim input; our helper must strip it.
// ---------------------------------------------------------------------------

#[test]
fn nom_error_kind_tag_does_not_contain_input() {
    // Construct a nom error that would normally include input bytes in its
    // Display output.
    let sensitive_input = "s3:c100,c200.classified";
    let nom_err: nom::Err<nom::error::Error<&str>> = nom::Err::Error(
        nom::error::Error::new(sensitive_input, nom::error::ErrorKind::Tag),
    );

    let kind = nom_error_kind(&nom_err);

    // The helper must return only the ErrorKind name, never the input slice.
    assert!(
        !kind.contains(sensitive_input),
        "nom_error_kind must not include the input slice: {kind}"
    );
    assert_eq!(kind, "Tag", "Should return the ErrorKind variant name");
}

#[test]
fn nom_error_kind_failure_returns_kind_name() {
    let nom_err: nom::Err<nom::error::Error<&str>> = nom::Err::Failure(
        nom::error::Error::new("user:role:type", nom::error::ErrorKind::Tag),
    );
    let kind = nom_error_kind(&nom_err);
    assert_eq!(kind, "Tag");
}

#[test]
fn nom_error_kind_incomplete_returns_label() {
    let nom_err: nom::Err<nom::error::Error<&str>> =
        nom::Err::Incomplete(nom::Needed::Unknown);
    let kind = nom_error_kind(&nom_err);
    assert_eq!(kind, "Incomplete");
}

#[test]
fn nom_error_kind_never_empty() {
    // For any ErrorKind variant, the returned string must be non-empty.
    for ek in [
        nom::error::ErrorKind::Tag,
        nom::error::ErrorKind::TakeUntil,
        nom::error::ErrorKind::Eof,
        nom::error::ErrorKind::Alpha,
    ] {
        let nom_err: nom::Err<nom::error::Error<&str>> =
            nom::Err::Error(nom::error::Error::new("", ek));
        let kind = nom_error_kind(&nom_err);
        assert!(!kind.is_empty(), "kind should never be empty for {ek:?}");
    }
}

// ---------------------------------------------------------------------------
// TpiError Debug output does not contain raw sensitive input
// ---------------------------------------------------------------------------

#[test]
fn tpi_path_a_failed_debug_only_contains_reason_field() {
    // The reason stored in PathAFailed is already sanitized (error kind only).
    // The Debug output of the enum itself must not exceed that.
    let err = TpiError::PathAFailed("nom::Tag".to_owned());
    let mut s = String::new();
    write!(s, "{err:?}").expect("write");
    // Must contain variant name and reason.
    assert!(
        s.contains("PathAFailed"),
        "Debug should name the variant: {s}"
    );
    assert!(s.contains("nom::Tag"), "Debug should include reason: {s}");
    // Must not contain any text suggesting raw MLS input.
    assert!(
        !s.contains("s0:c"),
        "Debug must not contain MLS level data: {s}"
    );
}
