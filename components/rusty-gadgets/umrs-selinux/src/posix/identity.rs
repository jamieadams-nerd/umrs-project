//! posix.rs
//!
//! Strong types for POSIX Linux usernames, group names, and numeric identity.
//!
//! Design principles:
//!   - Validation at construction only (Parse, Don't Validate)
//!   - Invalid states are unrepresentable after the boundary
//!   - Zero-cost abstractions — no heap overhead beyond the String itself
//!   - Immutable after construction (no setters, no DerefMut)
//!   - Implements common traits so the types compose naturally
//!   - Numeric uid/gid modeled alongside optional resolved names —
//!     reflecting the kernel's actual representation (names are lookups,
//!     uids/gids are ground truth)
//!
//! POSIX constraints enforced (per IEEE Std 1003.1 + Linux shadow-utils /
//! login.defs):
//!   - Maximum length: 32 characters (Linux login.defs LOGIN_NAME_MAX)
//!     Note: POSIX allows LOGIN_NAME_MAX up to 255, but Linux shadow-utils
//!     enforces 32 in practice. We follow the Linux implementation limit
//!     because we are a Linux-specific security tool. 255 is the filesystem
//!     NAME_MAX — a different constant, and wrong for login names.
//!   - Valid characters: [a-zA-Z0-9_-] with first-char rule
//!   - Must start with a letter or underscore [a-zA-Z_]
//!   - May not be empty
//!   - Trailing '$' permitted for usernames only (Samba machine accounts)
//!   - Null bytes never permitted (explicit FFI safety guard)
//!   - '.' is NOT permitted: not valid in Linux usernames or group names
//!     despite some secondary sources suggesting otherwise

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::posix::primitives::{Gid, Uid};

// ===========================================================================
// Constants
//
/// Linux login.defs LOGIN_NAME_MAX — the operative limit on Linux systems.
/// Do NOT change this to 255 (that is NAME_MAX, a filesystem concept,
/// and incorrect for POSIX login identifiers on Linux).
const POSIX_NAME_MAX_LEN: usize = 32;

// ===========================================================================
// Error type
//
/// Errors that may occur when constructing POSIX identity identifiers.
///
/// The error type carries position and character information where possible
/// so that callers can produce precise diagnostics. This matters for audit
/// logging — "invalid character" is not a useful audit event; the exact
/// character and position is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PosixNameError {
    /// Identifier was empty.
    Empty,

    /// Identifier exceeded maximum allowed length.
    TooLong {
        max: usize,
        got: usize,
    },

    /// First character violated the [a-zA-Z_] rule.
    InvalidFirstChar(char),

    /// A character at a specific position violated the allowed set.
    InvalidChar {
        ch: char,
        position: usize,
    },

    /// Identifier contained a null byte — always rejected for C FFI safety.
    ContainsNull,
}

impl fmt::Display for PosixNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "POSIX name must not be empty"),
            Self::TooLong {
                max,
                got,
            } => {
                write!(f, "POSIX name too long: max {max} chars, got {got}")
            }
            Self::InvalidFirstChar(c) => {
                write!(f, "POSIX name must start with [a-zA-Z_], got '{c}'")
            }
            Self::InvalidChar {
                ch,
                position,
            } => {
                write!(
                    f,
                    "Invalid character '{ch}' at position {position} in POSIX name"
                )
            }
            Self::ContainsNull => {
                write!(f, "POSIX name must not contain null bytes")
            }
        }
    }
}

impl std::error::Error for PosixNameError {}

// ────────────────────────────────────────────────────────────────────────────
// Core validation — the single gate
// ────────────────────────────────────────────────────────────────────────────

/// Validates a raw string slice against POSIX/Linux naming rules.
/// Returns `Ok(())` or the first violation found.
///
/// This is the **single validation gate**. It is called exactly once per
/// type — at construction time. After construction the type carries the
/// proof of validity. Callers must never re-validate a type that has
/// already been successfully constructed.
///
/// `allow_trailing_dollar`: Samba machine accounts use a trailing `$`
/// (e.g., `MYHOST$`). This is a username-only convention; group names
/// must pass `false`.
fn validate_posix_name(
    raw: &str,
    allow_trailing_dollar: bool,
) -> Result<(), PosixNameError> {
    // Null byte check — explicit defense against C interop confusion.
    // Rust strings cannot contain interior nulls, but the explicit check
    // documents intent for security reviewers and guards against future
    // unsafe code paths.
    if raw.contains('\0') {
        return Err(PosixNameError::ContainsNull);
    }

    if raw.is_empty() {
        return Err(PosixNameError::Empty);
    }

    if raw.len() > POSIX_NAME_MAX_LEN {
        return Err(PosixNameError::TooLong {
            max: POSIX_NAME_MAX_LEN,
            got: raw.len(),
        });
    }

    let mut chars = raw.chars().enumerate();

    // First character rule: must be [a-zA-Z_]
    // Digits, hyphens, dots, '$' — all invalid as first char.
    if let Some((_, first)) = chars.next()
        && !matches!(first, 'a'..='z' | 'A'..='Z' | '_')
    {
        return Err(PosixNameError::InvalidFirstChar(first));
    }

    // Remaining characters: [a-zA-Z0-9_-]
    // '.' is intentionally excluded: it is not valid in Linux usernames
    // or group names per shadow-utils, despite appearances in some docs.
    // If allow_trailing_dollar, a single '$' is permitted only at the
    // last byte position.
    let last_idx = raw.len().saturating_sub(1);

    for (i, ch) in chars {
        let valid = match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => true,
            '$' if allow_trailing_dollar && i == last_idx => true,
            _ => false,
        };
        if !valid {
            return Err(PosixNameError::InvalidChar {
                ch,
                position: i,
            });
        }
    }

    Ok(())
}

// ────────────────────────────────────────────────────────────────────────────
// LinuxUsername
// ────────────────────────────────────────────────────────────────────────────

/// A validated POSIX Linux username.
///
/// Construction is the only validation point. Once you hold a `LinuxUsername`,
/// it is guaranteed to satisfy all POSIX/Linux naming constraints. Use it
/// anywhere you would have previously used a raw `String` for a username.
///
/// # Examples
/// ```
/// # use umrs_selinux::posix::LinuxUsername;
/// let u = LinuxUsername::new("alice").unwrap();
/// let u2: LinuxUsername = "svc-nginx".parse().unwrap();
/// let u3 = LinuxUsername::try_from("_daemon").unwrap();
/// let u4 = LinuxUsername::new("MYHOST$").unwrap(); // Samba machine account
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxUsername(String);

impl LinuxUsername {
    /// Construct a validated `LinuxUsername`.
    ///
    /// This is the only public constructor — validation happens here and
    /// nowhere else. Usernames permit a trailing `$` for Samba machine
    /// accounts (e.g., `MYHOST$`).
    pub fn new(raw: &str) -> Result<Self, PosixNameError> {
        validate_posix_name(raw, /* allow_trailing_dollar = */ true)?;
        Ok(Self(raw.to_owned()))
    }

    /// Borrow the inner string as `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the inner `String`.
    ///
    /// Use sparingly — prefer borrowing via `as_str()` or `Deref`.
    /// This exists for interop with APIs that require owned `String`.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Length in bytes. POSIX names are ASCII, so bytes == chars.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Always false for a valid `LinuxUsername` — construction rejects empty.
    /// Provided for API completeness and to satisfy clippy.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for LinuxUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for LinuxUsername {
    type Err = PosixNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for LinuxUsername {
    type Error = PosixNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for LinuxUsername {
    type Error = PosixNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

/// `Deref` to `&str` for ergonomic read-only access.
///
/// Intentionally NOT implementing `DerefMut` — the invariant is sealed
/// at construction and must not be bypassed by mutation.
impl std::ops::Deref for LinuxUsername {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LinuxGroupName
// ────────────────────────────────────────────────────────────────────────────

/// A validated POSIX Linux group name.
///
/// Identical validation rules to `LinuxUsername` with one exception:
/// trailing `$` is **not** permitted (that convention is username-only).
///
/// `LinuxGroupName` and `LinuxUsername` are distinct types even though
/// their underlying validation is nearly identical. This is intentional:
/// a group name must never be silently accepted where a username is
/// expected. The type system enforces this at compile time.
///
/// # Examples
/// ```
/// # use umrs_selinux::posix::LinuxGroupName;
/// let g = LinuxGroupName::new("wheel").unwrap();
/// let g2: LinuxGroupName = "ssl-cert".parse().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxGroupName(String);

impl LinuxGroupName {
    /// Construct a validated `LinuxGroupName`.
    ///
    /// Trailing `$` is rejected — it is a username-only convention.
    pub fn new(raw: &str) -> Result<Self, PosixNameError> {
        validate_posix_name(raw, /* allow_trailing_dollar = */ false)?;
        Ok(Self(raw.to_owned()))
    }

    /// Borrow the inner string as `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the inner `String`.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Length in bytes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Always false for a valid `LinuxGroupName`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for LinuxGroupName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for LinuxGroupName {
    type Err = PosixNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for LinuxGroupName {
    type Error = PosixNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for LinuxGroupName {
    type Error = PosixNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl std::ops::Deref for LinuxGroupName {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LinuxUser — numeric uid paired with optional resolved name
//
// Models the kernel's actual representation: a file's owner IS a uid.
// The username is a lookup result from /etc/passwd or NSS — it may fail,
// and it is not authoritative. Callers that only have a uid should
// construct with name: None. Name resolution is a separate concern.
// ────────────────────────────────────────────────────────────────────────────

/// A Linux user identity: authoritative numeric uid plus optional resolved name.
///
/// The `uid` field is the ground truth — it is what the kernel stores and
/// enforces. The `name` field is a convenience: it may be absent if name
/// resolution was not performed or if the uid has no entry in the name
/// database (orphaned file owner).
///
/// This distinction matters for security tooling: a file owned by uid 1337
/// with no resolvable name is itself a finding — it should be surfaced as
/// such, not silently omitted or substituted with a placeholder string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxUser {
    /// Numeric user ID — authoritative kernel identity.
    pub uid: crate::posix::primitives::Uid,
    /// Resolved username, if available.
    pub name: Option<LinuxUsername>,
}

impl LinuxUser {
    #[must_use]
    pub const fn new(uid: Uid, name: LinuxUsername) -> Self {
        Self {
            uid,
            name: Some(name),
        }
    }

    #[must_use]
    pub const fn from_uid(uid: Uid) -> Self {
        Self {
            uid,
            name: None,
        }
    }

    pub fn from_raw(uid: Uid, name: &str) -> Result<Self, PosixNameError> {
        Ok(Self {
            uid,
            name: Some(LinuxUsername::new(name)?),
        })
    }

    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        self.name.is_none()
    }
}

impl fmt::Display for LinuxUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{name}({uid})", uid = self.uid),
            None => write!(f, "<uid:{uid}>", uid = self.uid),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LinuxGroup — numeric gid paired with optional resolved name
// ────────────────────────────────────────────────────────────────────────────

/// A Linux group identity: authoritative numeric gid plus optional resolved name.
///
/// Same design rationale as `LinuxUser` — gid is ground truth, name is a
/// lookup result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxGroup {
    /// Numeric group ID — authoritative kernel identity.
    pub gid: crate::posix::primitives::Gid,
    /// Resolved group name, if available.
    pub name: Option<LinuxGroupName>,
}

impl LinuxGroup {
    /// Construct with both gid and resolved name.
    #[must_use]
    pub const fn new(gid: Gid, name: LinuxGroupName) -> Self {
        Self {
            gid,
            name: Some(name),
        }
    }

    /// Construct with gid only.
    #[must_use]
    pub const fn from_gid(gid: Gid) -> Self {
        Self {
            gid,
            name: None,
        }
    }

    /// Convenience: construct with gid and raw name string.
    pub fn from_raw(gid: Gid, name: &str) -> Result<Self, PosixNameError> {
        Ok(Self {
            gid,
            name: Some(LinuxGroupName::new(name)?),
        })
    }

    /// Returns true if this gid has no resolvable name.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        self.name.is_none()
    }
}

impl fmt::Display for LinuxGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{name}({gid})", gid = self.gid),
            None => write!(f, "<gid:{gid}>", gid = self.gid),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LinuxOwnership — paired user and group ownership of a filesystem object
//
// This is the natural composition for use in SecureDirent and other
// filesystem-facing types. Both fields carry numeric + optional name.
// ────────────────────────────────────────────────────────────────────────────

/// Ownership of a filesystem object: user and group.
///
/// Used by `SecureDirent` and any other type that needs to represent
/// who owns a file. The name is "Linux" rather than "Posix" because this
/// type is tuned to Linux's actual behavior, not abstract POSIX portability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxOwnership {
    pub user: LinuxUser,
    pub group: LinuxGroup,
}

impl LinuxOwnership {
    #[must_use]
    pub const fn new(user: LinuxUser, group: LinuxGroup) -> Self {
        Self {
            user,
            group,
        }
    }

    /// Convenience: construct from raw uid/gid/name strings.
    /// Validates both names. Returns the first error encountered.
    pub fn from_raw(
        uid: Uid,
        username: &str,
        gid: Gid,
        groupname: &str,
    ) -> Result<Self, PosixNameError> {
        Ok(Self {
            user: LinuxUser::from_raw(uid, username)?,
            group: LinuxGroup::from_raw(gid, groupname)?,
        })
    }

    /// Construct from numeric ids only — names not resolved.
    #[must_use]
    pub const fn from_ids(uid: Uid, gid: Gid) -> Self {
        Self {
            user: LinuxUser::from_uid(uid),
            group: LinuxGroup::from_gid(gid),
        }
    }

    /// Construct from numeric ids, resolving names via NSS.
    ///
    /// Queries `/etc/passwd`, `/etc/group`, and any configured name service
    /// (LDAP, SSSD, etc.) to resolve the display names. If a uid or gid has
    /// no entry, `name` is `None` — indicating an orphaned account whose
    /// files remain on disk after deletion.
    ///
    /// This makes at most two NSS calls (one for uid, one for gid).
    /// `UnresolvedOwner` / `UnresolvedGroup` observations from
    /// `SecureDirent::security_observations()` only fire when this returns
    /// `name: None` — i.e., a genuinely missing account, not a lookup skip.
    ///
    /// NIST SP 800-53 AC-2: Account Management.
    #[must_use]
    pub fn resolve(uid: Uid, gid: Gid) -> Self {
        use nix::unistd::{
            Gid as NixGid, Group as NixGroup, Uid as NixUid, User as NixUser,
        };

        let user = if let Ok(Some(entry)) =
            NixUser::from_uid(NixUid::from_raw(uid.as_u32()))
        {
            if let Ok(resolved) = LinuxUser::from_raw(uid, &entry.name) {
                resolved
            } else {
                LinuxUser::from_uid(uid)
            }
        } else {
            LinuxUser::from_uid(uid)
        };

        let group = if let Ok(Some(entry)) =
            NixGroup::from_gid(NixGid::from_raw(gid.as_u32()))
        {
            if let Ok(resolved) = LinuxGroup::from_raw(gid, &entry.name) {
                resolved
            } else {
                LinuxGroup::from_gid(gid)
            }
        } else {
            LinuxGroup::from_gid(gid)
        };

        Self {
            user,
            group,
        }
    }

    /// Returns true if either owner uid or group gid has no resolved name.
    /// An unresolved owner on a CUI file is a security finding.
    #[must_use]
    pub const fn has_unresolved(&self) -> bool {
        self.user.is_unresolved() || self.group.is_unresolved()
    }
}

impl fmt::Display for LinuxOwnership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.user, self.group)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// UserIdentity — validated name pair (no numeric ids)
//
// Kept for cases where only string-level identity is needed — e.g., policy
// configuration, SELinux user mapping. Not for filesystem objects: use
// LinuxOwnership there.
// ────────────────────────────────────────────────────────────────────────────

/// A validated username + primary group name pair, without numeric ids.
///
/// Use this for configuration-level identity (e.g., SELinux user mapping,
/// Samba configuration). For filesystem object ownership, prefer
/// `LinuxOwnership` which carries the authoritative numeric ids.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentity {
    pub username: LinuxUsername,
    pub primary_group: LinuxGroupName,
}

impl UserIdentity {
    #[must_use]
    pub const fn new(
        username: LinuxUsername,
        primary_group: LinuxGroupName,
    ) -> Self {
        Self {
            username,
            primary_group,
        }
    }

    /// Convenience constructor from raw strings — validates both at once.
    pub fn from_raw(
        username: &str,
        group: &str,
    ) -> Result<Self, PosixNameError> {
        Ok(Self {
            username: LinuxUsername::new(username)?,
            primary_group: LinuxGroupName::new(group)?,
        })
    }
}

impl fmt::Display for UserIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.username, self.primary_group)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Serde support (feature-gated)
// ────────────────────────────────────────────────────────────────────────────
//
// #[cfg(feature = "serde")]
// mod serde_impl {
//     use super::*;
//     use serde::{Deserialize, Deserializer, Serialize, Serializer};
//
//     impl Serialize for LinuxUsername {
//         fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
//             s.serialize_str(self.as_str())
//         }
//     }
//
//     impl<'de> Deserialize<'de> for LinuxUsername {
//         fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//             let s = String::deserialize(d)?;
//             Self::new(&s).map_err(serde::de::Error::custom)
//         }
//     }
//     // Mirror pattern for LinuxGroupName
// }
//
