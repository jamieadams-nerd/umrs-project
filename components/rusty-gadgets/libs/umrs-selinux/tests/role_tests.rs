// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// =============================================================================
// role_tests.rs
// =============================================================================
//
// Integration tests for SelinuxRole primitive.
//
// Validates construction, parsing, formatting, and structural
// identifier enforcement.
//
// =============================================================================

use std::str::FromStr;

use umrs_selinux::role::{RoleError, SelinuxRole};

//
// -----------------------------------------------------------------------------
// Construction — Valid Inputs
// -----------------------------------------------------------------------------

#[test]
fn valid_roles_construct() {
    assert!(SelinuxRole::new("system_r").is_ok());
    assert!(SelinuxRole::new("staff_r").is_ok());
    assert!(SelinuxRole::new("object_r").is_ok());
}

//
// -----------------------------------------------------------------------------
// Empty Identifier
// -----------------------------------------------------------------------------

#[test]
fn rejects_empty_string() {
    let role = SelinuxRole::new("");
    assert!(matches!(role, Err(RoleError::Empty)));
}

//
// -----------------------------------------------------------------------------
// Suffix Enforcement
// -----------------------------------------------------------------------------

#[test]
fn rejects_missing_suffix() {
    let role = SelinuxRole::new("system");
    assert!(matches!(role, Err(RoleError::InvalidSuffix)));
}

#[test]
fn rejects_wrong_suffix() {
    let role = SelinuxRole::new("system_u");
    assert!(matches!(role, Err(RoleError::InvalidSuffix)));
}

//
// -----------------------------------------------------------------------------
// Stem Validation
// -----------------------------------------------------------------------------

#[test]
fn rejects_empty_stem() {
    let role = SelinuxRole::new("_r");
    assert!(matches!(role, Err(RoleError::InvalidStem)));
}

//
// -----------------------------------------------------------------------------
// Character Validation
// -----------------------------------------------------------------------------

#[test]
fn rejects_invalid_characters() {
    let role = SelinuxRole::new("system-r");
    assert!(matches!(role, Err(RoleError::InvalidCharacter('-'))));
}

#[test]
fn rejects_whitespace() {
    let role = SelinuxRole::new("system r");
    assert!(role.is_err());
}

#[test]
fn rejects_uppercase() {
    let role = SelinuxRole::new("System_r");
    assert!(role.is_err());
}

//
// -----------------------------------------------------------------------------
// Length Validation
// -----------------------------------------------------------------------------

#[test]
fn rejects_too_short() {
    let role = SelinuxRole::new("r");
    assert!(matches!(role, Err(RoleError::InvalidStem)));
}

#[test]
fn rejects_too_long() {
    let long_name = format!("{}{}", "a".repeat(254), "_r");

    let role = SelinuxRole::new(long_name);

    assert!(matches!(role, Err(RoleError::TooLong(_))));
}

//
// -----------------------------------------------------------------------------
// Display / Parsing Round Trip
// -----------------------------------------------------------------------------

#[test]
fn display_round_trip() {
    let role: SelinuxRole = "staff_r".parse().unwrap();

    assert_eq!(role.to_string(), "staff_r");
}

#[test]
fn from_str_constructor_equivalence() {
    let r1 = SelinuxRole::new("system_r").unwrap();
    let r2 = SelinuxRole::from_str("system_r").unwrap();

    assert_eq!(r1, r2);
}

//
// -----------------------------------------------------------------------------
// AsRef Behavior
// -----------------------------------------------------------------------------

#[test]
fn as_ref_returns_inner_str() {
    let role = SelinuxRole::new("object_r").unwrap();

    assert_eq!(role.as_ref(), "object_r");
}
