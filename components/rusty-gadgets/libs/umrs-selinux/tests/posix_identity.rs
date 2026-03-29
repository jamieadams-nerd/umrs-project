// tests/posix_identity.rs
//
// Integration-style tests for posix_identity module types.
// Kept separate from source per project convention.

use umrs_selinux::posix::{
    Gid, LinuxGroup, LinuxGroupName, LinuxOwnership, LinuxUser, LinuxUsername,
    PosixNameError, Uid, UserIdentity,
};

// ── LinuxUsername: valid cases ────────────────────────────────────────────────

#[test]
fn valid_simple_username() {
    assert!(LinuxUsername::new("alice").is_ok());
}

#[test]
fn valid_username_with_hyphen_and_digits() {
    assert!(LinuxUsername::new("svc-account1").is_ok());
}

#[test]
fn valid_username_underscore_start() {
    assert!(LinuxUsername::new("_daemon").is_ok());
}

#[test]
fn valid_samba_machine_account() {
    // Trailing '$' is valid for usernames (Samba convention)
    assert!(LinuxUsername::new("MYHOST$").is_ok());
}

#[test]
fn valid_username_mixed_case() {
    assert!(LinuxUsername::new("SvcNginx").is_ok());
}

#[test]
fn valid_exactly_at_max_len() {
    let exactly_max = "a".repeat(32);
    assert!(LinuxUsername::new(&exactly_max).is_ok());
}

// ── LinuxUsername: invalid cases ──────────────────────────────────────────────

#[test]
fn invalid_empty_username() {
    assert_eq!(LinuxUsername::new("").unwrap_err(), PosixNameError::Empty);
}

#[test]
fn invalid_starts_with_digit() {
    assert!(matches!(
        LinuxUsername::new("1root"),
        Err(PosixNameError::InvalidFirstChar('1'))
    ));
}

#[test]
fn invalid_starts_with_hyphen() {
    assert!(matches!(
        LinuxUsername::new("-service"),
        Err(PosixNameError::InvalidFirstChar('-'))
    ));
}

#[test]
fn invalid_contains_space() {
    assert!(matches!(
        LinuxUsername::new("alice bob"),
        Err(PosixNameError::InvalidChar {
            ch: ' ',
            position: 5
        })
    ));
}

#[test]
fn invalid_too_long() {
    let long = "a".repeat(33);
    assert!(matches!(
        LinuxUsername::new(&long),
        Err(PosixNameError::TooLong {
            max: 32,
            got: 33
        })
    ));
}

#[test]
fn invalid_contains_null() {
    assert_eq!(
        LinuxUsername::new("alice\0bob").unwrap_err(),
        PosixNameError::ContainsNull
    );
}

#[test]
fn invalid_contains_at_sign() {
    assert!(matches!(
        LinuxUsername::new("alice@example"),
        Err(PosixNameError::InvalidChar {
            ch: '@',
            ..
        })
    ));
}

#[test]
fn invalid_contains_dot() {
    // '.' is NOT valid in Linux usernames
    assert!(matches!(
        LinuxUsername::new("first.last"),
        Err(PosixNameError::InvalidChar {
            ch: '.',
            ..
        })
    ));
}

#[test]
fn invalid_dollar_not_at_end() {
    // '$' is only valid as the last character
    assert!(matches!(
        LinuxUsername::new("MY$HOST"),
        Err(PosixNameError::InvalidChar {
            ch: '$',
            ..
        })
    ));
}

// ── LinuxGroupName ────────────────────────────────────────────────────────────

#[test]
fn valid_groupnames() {
    assert!(LinuxGroupName::new("wheel").is_ok());
    assert!(LinuxGroupName::new("docker").is_ok());
    assert!(LinuxGroupName::new("ssl-cert").is_ok());
    assert!(LinuxGroupName::new("_system").is_ok());
}

#[test]
fn groupname_rejects_trailing_dollar() {
    // Groups never allow trailing '$'
    assert!(matches!(
        LinuxGroupName::new("MYHOST$"),
        Err(PosixNameError::InvalidChar {
            ch: '$',
            ..
        })
    ));
}

#[test]
fn groupname_rejects_dot() {
    assert!(matches!(
        LinuxGroupName::new("group.name"),
        Err(PosixNameError::InvalidChar {
            ch: '.',
            ..
        })
    ));
}

// ── TryFrom impls ─────────────────────────────────────────────────────────────

#[test]
fn try_from_str_username() {
    use std::convert::TryFrom;
    let u = LinuxUsername::try_from("svc-nginx").unwrap();
    assert_eq!(u.as_str(), "svc-nginx");
}

#[test]
fn try_from_string_username() {
    use std::convert::TryFrom;
    let u = LinuxUsername::try_from(String::from("alice")).unwrap();
    assert_eq!(u.as_str(), "alice");
}

#[test]
fn try_from_str_groupname() {
    use std::convert::TryFrom;
    let g = LinuxGroupName::try_from("wheel").unwrap();
    assert_eq!(g.as_str(), "wheel");
}

// ── into_inner ────────────────────────────────────────────────────────────────

#[test]
fn into_inner_recovers_string() {
    let u = LinuxUsername::new("alice").unwrap();
    assert_eq!(u.into_inner(), String::from("alice"));
}

// ── Type safety ───────────────────────────────────────────────────────────────

#[test]
fn username_and_groupname_are_distinct_types() {
    // Same string, different types — cannot be compared or substituted
    let u = LinuxUsername::new("alice").unwrap();
    let g = LinuxGroupName::new("alice").unwrap();
    // assert_eq!(u, g); // ← compile error: type mismatch — correct behavior
    assert_eq!(u.as_str(), g.as_str()); // only equal via explicit &str extraction
}

// ── LinuxUser ─────────────────────────────────────────────────────────────────

#[test]
fn linux_user_with_name() {
    let u = LinuxUser::from_raw(Uid::new(1000), "alice").unwrap();
    assert_eq!(u.uid, Uid::new(1000));
    assert!(!u.is_unresolved());
    assert_eq!(u.to_string(), "alice(1000)");
}

#[test]
fn linux_user_uid_only() {
    let u = LinuxUser::from_uid(Uid::new(1337));
    assert!(u.is_unresolved());
    assert_eq!(u.to_string(), "<uid:1337>");
}

#[test]
fn linux_user_from_raw_rejects_bad_name() {
    assert!(LinuxUser::from_raw(Uid::new(0), "1bad").is_err());
}

// ── LinuxGroup ────────────────────────────────────────────────────────────────

#[test]
fn linux_group_with_name() {
    let g = LinuxGroup::from_raw(Gid::new(1000), "staff").unwrap();
    assert_eq!(g.gid, Gid::new(1000));
    assert!(!g.is_unresolved());
    assert_eq!(g.to_string(), "staff(1000)");
}

#[test]
fn linux_group_gid_only() {
    let g = LinuxGroup::from_gid(Gid::new(999));
    assert!(g.is_unresolved());
    assert_eq!(g.to_string(), "<gid:999>");
}

// ── LinuxOwnership ────────────────────────────────────────────────────────────

#[test]
fn linux_ownership_from_raw() {
    let o = LinuxOwnership::from_raw(
        Uid::new(1000),
        "alice",
        Gid::new(1000),
        "staff",
    )
    .unwrap();
    assert_eq!(o.to_string(), "alice(1000):staff(1000)");
    assert!(!o.has_unresolved());
}

#[test]
fn linux_ownership_from_ids() {
    let o = LinuxOwnership::from_ids(Uid::new(0), Gid::new(0));
    assert!(o.has_unresolved());
    assert_eq!(o.to_string(), "<uid:0>:<gid:0>");
}

#[test]
fn linux_ownership_rejects_bad_group() {
    assert!(
        LinuxOwnership::from_raw(Uid::new(0), "root", Gid::new(0), "bad.group")
            .is_err()
    );
}

// ── UserIdentity ──────────────────────────────────────────────────────────────

#[test]
fn user_identity_from_raw() {
    let id = UserIdentity::from_raw("alice", "staff").unwrap();
    assert_eq!(id.to_string(), "alice:staff");
}

#[test]
fn user_identity_rejects_bad_username() {
    assert!(UserIdentity::from_raw("1bad", "staff").is_err());
}

// ── FromStr (parse()) ─────────────────────────────────────────────────────────

#[test]
fn parse_username_from_str() {
    let u: LinuxUsername = "svc-nginx".parse().unwrap();
    assert_eq!(u.as_str(), "svc-nginx");
}

#[test]
fn parse_groupname_from_str() {
    let g: LinuxGroupName = "docker".parse().unwrap();
    assert_eq!(g.as_str(), "docker");
}
