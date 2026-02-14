// -----------------------------------------------------------------------------
// UMRS SELinux — SecurityContext Integration Tests
// -----------------------------------------------------------------------------

use std::str::FromStr;

use umrs_selinux::context::SecurityContext;
use umrs_selinux::user::SelinuxUser;
use umrs_selinux::role::SelinuxRole;
use umrs_selinux::type_id::SelinuxType;

// -----------------------------------------------------------------------------
// Construction
// -----------------------------------------------------------------------------

#[test]
fn construct_security_context() {
    let user = SelinuxUser::from_str("system_u").unwrap();
    let role = SelinuxRole::from_str("system_r").unwrap();
    let security_type = SelinuxType::from_str("sshd_t").unwrap();

    let ctx = SecurityContext::new(
        user.clone(),
        role.clone(),
        security_type.clone(),
    );

    assert_eq!(ctx.user(), &user);
    assert_eq!(ctx.role(), &role);
    assert_eq!(ctx.security_type(), &security_type);
}

// -----------------------------------------------------------------------------
// Display Serialization
// -----------------------------------------------------------------------------

#[test]
fn display_formats_canonical_context() {
    let ctx: SecurityContext =
        "system_u:system_r:sshd_t".parse().unwrap();

    assert_eq!(
        ctx.to_string(),
        "system_u:system_r:sshd_t"
    );
}

// -----------------------------------------------------------------------------
// Parsing — Valid
// -----------------------------------------------------------------------------

#[test]
fn parse_valid_context() {
    let ctx: SecurityContext =
        "unconfined_u:unconfined_r:unconfined_t"
            .parse()
            .unwrap();

    assert_eq!(ctx.user().to_string(), "unconfined_u");
    assert_eq!(ctx.role().to_string(), "unconfined_r");
    assert_eq!(ctx.security_type().to_string(), "unconfined_t");
}

// -----------------------------------------------------------------------------
// Parsing — Invalid Format
// -----------------------------------------------------------------------------

#[test]
fn parse_invalid_format_missing_fields() {
    let result: Result<SecurityContext, _> =
        "system_u:sshd_t".parse();

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
    let result: Result<SecurityContext, _> =
        "bad user:system_r:sshd_t".parse();

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
    let result: Result<SecurityContext, _> =
        "system_u:bad role:sshd_t".parse();

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
    let result: Result<SecurityContext, _> =
        "system_u:system_r:bad type".parse();

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
    let a: SecurityContext =
        "system_u:system_r:sshd_t".parse().unwrap();

    let b: SecurityContext =
        "system_u:system_r:sshd_t".parse().unwrap();

    assert_eq!(a, b);
}

// -----------------------------------------------------------------------------
// Inequality
// -----------------------------------------------------------------------------

#[test]
fn contexts_compare_not_equal() {
    let a: SecurityContext =
        "system_u:system_r:sshd_t".parse().unwrap();

    let b: SecurityContext =
        "system_u:system_r:cron_t".parse().unwrap();

    assert_ne!(a, b);
}
