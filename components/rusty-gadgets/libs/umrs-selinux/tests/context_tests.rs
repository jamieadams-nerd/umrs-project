// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
// -----------------------------------------------------------------------------
// UMRS SELinux — SecurityContext Integration Tests
// -----------------------------------------------------------------------------

use std::str::FromStr;

use umrs_selinux::context::SecurityContext;
use umrs_selinux::role::SelinuxRole;
use umrs_selinux::type_id::SelinuxType;
use umrs_selinux::user::SelinuxUser;

// -----------------------------------------------------------------------------
// Construction
// -----------------------------------------------------------------------------

#[test]
fn construct_security_context() {
    let user = SelinuxUser::from_str("system_u").unwrap();
    let role = SelinuxRole::from_str("system_r").unwrap();
    let security_type = SelinuxType::from_str("sshd_t").unwrap();

    let ctx = SecurityContext::new(user.clone(), role.clone(), security_type.clone(), None);

    assert_eq!(ctx.user(), &user);
    assert_eq!(ctx.role(), &role);
    assert_eq!(ctx.security_type(), &security_type);
}

// -----------------------------------------------------------------------------
// Display Serialization
// -----------------------------------------------------------------------------

#[test]
fn display_formats_canonical_context() {
    let ctx: SecurityContext = "system_u:system_r:sshd_t".parse().unwrap();

    assert_eq!(ctx.to_string(), "system_u:system_r:sshd_t");
}

// -----------------------------------------------------------------------------
// Parsing — Valid
// -----------------------------------------------------------------------------

#[test]
fn parse_valid_context() {
    let ctx: SecurityContext = "unconfined_u:unconfined_r:unconfined_t".parse().unwrap();

    assert_eq!(ctx.user().to_string(), "unconfined_u");
    assert_eq!(ctx.role().to_string(), "unconfined_r");
    assert_eq!(ctx.security_type().to_string(), "unconfined_t");
}

// -----------------------------------------------------------------------------
// Parsing — Invalid Format
// -----------------------------------------------------------------------------

#[test]
fn parse_invalid_format_missing_fields() {
    let result: Result<SecurityContext, _> = "system_u:sshd_t".parse();

    assert!(matches!(
        result,
        Err(umrs_selinux::context::ContextParseError::InvalidFormat)
    ));
}

// -----------------------------------------------------------------------------
// Parsing — Invalid User
// -----------------------------------------------------------------------------

#[test]
fn parse_invalid_user() {
    let result: Result<SecurityContext, _> = "bad user:system_r:sshd_t".parse();

    assert!(matches!(
        result,
        Err(umrs_selinux::context::ContextParseError::InvalidUser)
    ));
}

// -----------------------------------------------------------------------------
// Parsing — Invalid Role
// -----------------------------------------------------------------------------

#[test]
fn parse_invalid_role() {
    let result: Result<SecurityContext, _> = "system_u:bad role:sshd_t".parse();

    assert!(matches!(
        result,
        Err(umrs_selinux::context::ContextParseError::InvalidRole)
    ));
}

// -----------------------------------------------------------------------------
// Parsing — Invalid Type
// -----------------------------------------------------------------------------

#[test]
fn parse_invalid_type() {
    let result: Result<SecurityContext, _> = "system_u:system_r:bad type".parse();

    assert!(matches!(
        result,
        Err(umrs_selinux::context::ContextParseError::InvalidType)
    ));
}

// -----------------------------------------------------------------------------
// Equality
// -----------------------------------------------------------------------------

#[test]
fn contexts_compare_equal() {
    let a: SecurityContext = "system_u:system_r:sshd_t".parse().unwrap();

    let b: SecurityContext = "system_u:system_r:sshd_t".parse().unwrap();

    assert_eq!(a, b);
}

// -----------------------------------------------------------------------------
// Inequality
// -----------------------------------------------------------------------------

#[test]
fn contexts_compare_not_equal() {
    let a: SecurityContext = "system_u:system_r:sshd_t".parse().unwrap();

    let b: SecurityContext = "system_u:system_r:cron_t".parse().unwrap();

    assert_ne!(a, b);
}

// -----------------------------------------------------------------------------
// Mixed-case SELinux type identifiers (regression test)
//
// SELinux policy modules such as NetworkManager use types with uppercase
// initial characters and internal uppercase letters. The parser must accept
// these identifiers — they are valid per the SELinux kernel policy parser
// character set [a-zA-Z0-9_].
//
// This test reproduces the TPI Path A + Path B failure on
// `NetworkManager_etc_t` that was diagnosed in task #2. Both paths
// (nom + FromStr) delegate validation to `SelinuxType::new`, which
// previously rejected uppercase characters.
// -----------------------------------------------------------------------------

#[test]
fn parse_context_with_mixed_case_type() {
    let input = "system_u:object_r:NetworkManager_etc_t";
    let ctx: SecurityContext = input.parse().unwrap();

    assert_eq!(ctx.user().to_string(), "system_u");
    assert_eq!(ctx.role().to_string(), "object_r");
    assert_eq!(ctx.security_type().to_string(), "NetworkManager_etc_t");
    assert!(ctx.level().is_none());
}

#[test]
fn parse_context_with_mixed_case_type_and_mls_level() {
    let input = "system_u:object_r:NetworkManager_etc_t:s0";
    let ctx: SecurityContext = input.parse().unwrap();

    assert_eq!(ctx.security_type().to_string(), "NetworkManager_etc_t");
    assert!(ctx.level().is_some());
}

#[test]
fn display_round_trip_mixed_case_type() {
    let input = "system_u:object_r:NetworkManager_etc_t";
    let ctx: SecurityContext = input.parse().unwrap();

    assert_eq!(ctx.to_string(), input);
}
