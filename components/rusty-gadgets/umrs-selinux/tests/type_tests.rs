//
// =============================================================================
// type_tests.rs
// =============================================================================
//
// Integration tests for SelinuxType primitive.
//
// Validates construction, parsing, formatting, and structural
// identifier enforcement.
//
// =============================================================================

use std::str::FromStr;

use umrs_selinux::type_id::{SelinuxType, TypeError};

//
// -----------------------------------------------------------------------------
// Construction — Valid Inputs
// -----------------------------------------------------------------------------

#[test]
fn valid_types_construct() {
    assert!(SelinuxType::new("sshd_t").is_ok());
    assert!(SelinuxType::new("var_log_t").is_ok());
    assert!(SelinuxType::new("httpd_t").is_ok());
}

//
// -----------------------------------------------------------------------------
// Empty Identifier
// -----------------------------------------------------------------------------

#[test]
fn rejects_empty_string() {
    let ty = SelinuxType::new("");
    assert!(matches!(ty, Err(TypeError::Empty)));
}

//
// -----------------------------------------------------------------------------
// Suffix Enforcement
// -----------------------------------------------------------------------------

#[test]
fn rejects_missing_suffix() {
    let ty = SelinuxType::new("sshd");
    assert!(matches!(ty, Err(TypeError::InvalidSuffix)));
}

#[test]
fn rejects_wrong_suffix() {
    let ty = SelinuxType::new("sshd_r");
    assert!(matches!(ty, Err(TypeError::InvalidSuffix)));
}

//
// -----------------------------------------------------------------------------
// Stem Validation
// -----------------------------------------------------------------------------

#[test]
fn rejects_empty_stem() {
    let ty = SelinuxType::new("_t");
    assert!(matches!(ty, Err(TypeError::InvalidStem)));
}

//
// -----------------------------------------------------------------------------
// Character Validation
// -----------------------------------------------------------------------------

#[test]
fn rejects_invalid_characters() {
    let ty = SelinuxType::new("sshd-t");
    assert!(matches!(ty, Err(TypeError::InvalidCharacter('-'))));
}

#[test]
fn rejects_whitespace() {
    let ty = SelinuxType::new("sshd t");
    assert!(ty.is_err());
}

#[test]
fn rejects_uppercase() {
    let ty = SelinuxType::new("Sshd_t");
    assert!(ty.is_err());
}

//
// -----------------------------------------------------------------------------
// Length Validation
// -----------------------------------------------------------------------------

#[test]
fn rejects_too_short() {
    let ty = SelinuxType::new("t");
    assert!(matches!(ty, Err(TypeError::InvalidStem)));
}

#[test]
fn rejects_too_long() {
    let long_name = format!("{}{}", "a".repeat(254), "_t");

    let ty = SelinuxType::new(long_name);

    assert!(matches!(ty, Err(TypeError::TooLong(_))));
}

//
// -----------------------------------------------------------------------------
// Display / Parsing Round Trip
// -----------------------------------------------------------------------------

#[test]
fn display_round_trip() {
    let ty: SelinuxType = "httpd_t".parse().unwrap();

    assert_eq!(ty.to_string(), "httpd_t");
}

#[test]
fn from_str_constructor_equivalence() {
    let t1 = SelinuxType::new("sshd_t").unwrap();
    let t2 = SelinuxType::from_str("sshd_t").unwrap();

    assert_eq!(t1, t2);
}

//
// -----------------------------------------------------------------------------
// AsRef Behavior
// -----------------------------------------------------------------------------

#[test]
fn as_ref_returns_inner_str() {
    let ty = SelinuxType::new("var_log_t").unwrap();

    assert_eq!(ty.as_ref(), "var_log_t");
}
