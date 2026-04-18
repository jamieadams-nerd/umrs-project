// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
// -----------------------------------------------------------------------------
// UMRS SELinux — MLS Level Integration Tests
// -----------------------------------------------------------------------------

use umrs_selinux::mls::level::MlsLevel;

// -----------------------------------------------------------------------------
// Parse — Sensitivity Only
// -----------------------------------------------------------------------------

#[test]
fn parse_sensitivity_only() {
    let level: MlsLevel = "s0".parse().unwrap();

    assert_eq!(level.sensitivity().to_string(), "s0");
    assert!(!level.has_categories());
}

// -----------------------------------------------------------------------------
// Parse — Single Category
// -----------------------------------------------------------------------------

#[test]
fn parse_single_category() {
    let level: MlsLevel = "s0:c3".parse().unwrap();

    assert_eq!(level.sensitivity().to_string(), "s0");
    assert!(level.has_categories());
    assert_eq!(level.categories().to_string(), "c3");
}

// -----------------------------------------------------------------------------
// Parse — Multiple Categories
// -----------------------------------------------------------------------------

#[test]
fn parse_multiple_categories() {
    let level: MlsLevel = "s2:c1,c7,c42".parse().unwrap();

    assert_eq!(level.sensitivity().to_string(), "s2");
    assert!(level.has_categories());
    assert_eq!(level.categories().to_string(), "c1,c7,c42");
}

// -----------------------------------------------------------------------------
// Display Round-Trip
// -----------------------------------------------------------------------------

#[test]
fn display_round_trip() {
    let input = "s3:c0,c5,c9";

    let level: MlsLevel = input.parse().unwrap();

    assert_eq!(level.to_string(), input);
}

// -----------------------------------------------------------------------------
// Equality
// -----------------------------------------------------------------------------

#[test]
fn levels_compare_equal() {
    let a: MlsLevel = "s1:c0,c1".parse().unwrap();
    let b: MlsLevel = "s1:c0,c1".parse().unwrap();

    assert_eq!(a, b);
}

// -----------------------------------------------------------------------------
// Inequality — Sensitivity
// -----------------------------------------------------------------------------

#[test]
fn levels_not_equal_sensitivity() {
    let a: MlsLevel = "s1:c0".parse().unwrap();
    let b: MlsLevel = "s2:c0".parse().unwrap();

    assert_ne!(a, b);
}

// -----------------------------------------------------------------------------
// Inequality — Categories
// -----------------------------------------------------------------------------

#[test]
fn levels_not_equal_categories() {
    let a: MlsLevel = "s1:c0,c1".parse().unwrap();
    let b: MlsLevel = "s1:c0,c2".parse().unwrap();

    assert_ne!(a, b);
}

// -----------------------------------------------------------------------------
// Invalid — Empty String
// -----------------------------------------------------------------------------

#[test]
fn reject_empty_string() {
    let result: Result<MlsLevel, _> = "".parse();

    assert!(result.is_err());
}

// -----------------------------------------------------------------------------
// Invalid — Missing Sensitivity
// -----------------------------------------------------------------------------

#[test]
fn reject_missing_sensitivity() {
    let result: Result<MlsLevel, _> = ":c0".parse();

    assert!(result.is_err());
}

// -----------------------------------------------------------------------------
// Invalid — Empty Category Section
// -----------------------------------------------------------------------------

#[test]
fn reject_empty_category_section() {
    let result: Result<MlsLevel, _> = "s0:".parse();

    assert!(result.is_err());
}

// -----------------------------------------------------------------------------
// Invalid — Malformed Category Token
// -----------------------------------------------------------------------------

#[test]
fn reject_invalid_category_token() {
    let result: Result<MlsLevel, _> = "s0:c0,INVALID".parse();

    assert!(result.is_err());
}

// -----------------------------------------------------------------------------
// Whitespace Handling
// -----------------------------------------------------------------------------

#[test]
fn parse_with_whitespace() {
    let level: MlsLevel = "  s0:c1,c2  ".parse().unwrap();

    assert_eq!(level.to_string(), "s0:c1,c2");
}
