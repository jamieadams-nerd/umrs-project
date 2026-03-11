use umrs_selinux::user::{SelinuxUser, UserError};

#[test]
fn valid_users_construct() {
    assert!(SelinuxUser::new("system_u").is_ok());
    assert!(SelinuxUser::new("staff_u").is_ok());
}

#[test]
fn rejects_empty() {
    let u = SelinuxUser::new("");
    assert!(matches!(u, Err(UserError::Empty)));
}

#[test]
fn rejects_bad_suffix() {
    let u = SelinuxUser::new("system");
    assert!(matches!(u, Err(UserError::InvalidSuffix)));
}

#[test]
fn rejects_invalid_characters() {
    let u = SelinuxUser::new("system-u");
    assert!(matches!(u, Err(UserError::InvalidCharacter('-'))));
}

#[test]
fn rejects_empty_stem() {
    let u = SelinuxUser::new("_u");
    assert!(matches!(u, Err(UserError::InvalidStem)));
}

#[test]
fn display_round_trip() {
    let u: SelinuxUser = "staff_u".parse().unwrap();
    assert_eq!(u.to_string(), "staff_u");
}
