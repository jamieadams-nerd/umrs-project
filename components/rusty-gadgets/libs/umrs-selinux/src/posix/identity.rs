// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # POSIX Linux Identity Types
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
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-2**: Account Management — typed username and group
//!   name newtypes prevent identity confusion in access control paths.
//! - **NIST SP 800-53 IA-5**: Authenticator Management — construct-time
//!   validation ensures only well-formed identity values are representable.
//! - **NSA RTB RAIN**: Validate at construction; invalid identity values
//!   cannot be constructed and therefore cannot propagate downstream.

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
///
/// ## Variants:
///
/// - `Empty` — identifier was empty.
/// - `TooLong { max: usize, got: usize }` — identifier exceeded maximum allowed length; `max` is
///   the limit, `got` is the actual length supplied.
/// - `InvalidFirstChar(char)` — first character violated the `[a-zA-Z_]` rule.
/// - `InvalidChar { ch: char, position: usize }` — a character at a specific position violated
///   the allowed set; `ch` is the offending character, `position` is its byte offset.
/// - `ContainsNull` — identifier contained a null byte; always rejected for C FFI safety.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PosixNameError {
    Empty,
    TooLong {
        max: usize,
        got: usize,
    },
    InvalidFirstChar(char),
    InvalidChar {
        ch: char,
        position: usize,
    },
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
fn validate_posix_name(raw: &str, allow_trailing_dollar: bool) -> Result<(), PosixNameError> {
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
    ///
    /// # Errors
    ///
    /// Returns [`PosixNameError`] if the name is empty, exceeds the length limit, or contains invalid characters.
    pub fn new(raw: &str) -> Result<Self, PosixNameError> {
        validate_posix_name(raw, /* allow_trailing_dollar = */ true)?;
        Ok(Self(raw.to_owned()))
    }

    /// Borrow the inner string as `&str`.
    #[must_use = "pure accessor returning the validated username string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the inner `String`.
    ///
    /// Use sparingly — prefer borrowing via `as_str()` or `Deref`.
    /// This exists for interop with APIs that require owned `String`.
    #[must_use = "consumes the wrapper and returns the inner String; the value is lost if the result is discarded"]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Length in bytes. POSIX names are ASCII, so bytes == chars.
    #[must_use = "pure accessor returning the byte length of the username"]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Always false for a valid `LinuxUsername` — construction rejects empty.
    /// Provided for API completeness and to satisfy clippy.
    #[must_use = "pure accessor; always false for a valid LinuxUsername — construction rejects empty names"]
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
    ///
    /// # Errors
    ///
    /// Returns [`PosixNameError`] if the name is empty, exceeds the length limit, or contains invalid characters.
    pub fn new(raw: &str) -> Result<Self, PosixNameError> {
        validate_posix_name(raw, /* allow_trailing_dollar = */ false)?;
        Ok(Self(raw.to_owned()))
    }

    /// Borrow the inner string as `&str`.
    #[must_use = "pure accessor returning the validated group name string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the inner `String`.
    #[must_use = "consumes the wrapper and returns the inner String; the value is lost if the result is discarded"]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Length in bytes.
    #[must_use = "pure accessor returning the byte length of the group name"]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Always false for a valid `LinuxGroupName`.
    #[must_use = "pure accessor; always false for a valid LinuxGroupName — construction rejects empty names"]
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
///
/// ## Fields:
///
/// - `uid` — numeric user ID; authoritative kernel identity.
/// - `name` — resolved username, if available; `None` if resolution was not performed or the uid
///   has no name database entry (orphaned file owner).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxUser {
    pub uid: crate::posix::primitives::Uid,
    pub name: Option<LinuxUsername>,
}

impl LinuxUser {
    #[must_use = "returns a LinuxUser with both uid and resolved name; discarding it loses the identity record"]
    pub const fn new(uid: Uid, name: LinuxUsername) -> Self {
        Self {
            uid,
            name: Some(name),
        }
    }

    #[must_use = "returns a LinuxUser with uid only (name unresolved); discarding it loses the numeric identity"]
    pub const fn from_uid(uid: Uid) -> Self {
        Self {
            uid,
            name: None,
        }
    }

    /// Construct a `LinuxUser` from a raw UID and username string, validating
    /// the username at construction time.
    ///
    /// # Errors
    ///
    /// Returns [`PosixNameError`] if the username fails validation.
    pub fn from_raw(uid: Uid, name: &str) -> Result<Self, PosixNameError> {
        Ok(Self {
            uid,
            name: Some(LinuxUsername::new(name)?),
        })
    }

    #[must_use = "pure accessor; an unresolved uid on a CUI file is a security finding that must not be silently dropped"]
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
///
/// ## Fields:
///
/// - `gid` — numeric group ID; authoritative kernel identity.
/// - `name` — resolved group name, if available; `None` if resolution was not performed or the
///   gid has no name database entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxGroup {
    pub gid: crate::posix::primitives::Gid,
    pub name: Option<LinuxGroupName>,
}

impl LinuxGroup {
    /// Construct with both gid and resolved name.
    #[must_use = "returns a LinuxGroup with both gid and resolved name; discarding it loses the group identity record"]
    pub const fn new(gid: Gid, name: LinuxGroupName) -> Self {
        Self {
            gid,
            name: Some(name),
        }
    }

    /// Construct with gid only.
    #[must_use = "returns a LinuxGroup with gid only (name unresolved); discarding it loses the numeric group identity"]
    pub const fn from_gid(gid: Gid) -> Self {
        Self {
            gid,
            name: None,
        }
    }

    /// Convenience: construct with gid and raw name string.
    ///
    /// # Errors
    ///
    /// Returns [`PosixNameError`] if the group name fails validation.
    pub fn from_raw(gid: Gid, name: &str) -> Result<Self, PosixNameError> {
        Ok(Self {
            gid,
            name: Some(LinuxGroupName::new(name)?),
        })
    }

    /// Returns true if this gid has no resolvable name.
    #[must_use = "pure accessor; an unresolved gid on a CUI file is a security finding that must not be silently dropped"]
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
    #[must_use = "returns a LinuxOwnership pairing user and group; discarding it loses the complete ownership record"]
    pub const fn new(user: LinuxUser, group: LinuxGroup) -> Self {
        Self {
            user,
            group,
        }
    }

    /// Convenience: construct from raw uid/gid/name strings.
    /// Validates both names. Returns the first error encountered.
    ///
    /// # Errors
    ///
    /// Returns [`PosixNameError`] if the group name fails validation.
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
    #[must_use = "returns a LinuxOwnership from raw ids without name resolution; discarding it loses the ownership record"]
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
    #[must_use = "returns resolved ownership via NSS lookup; discarding it wastes the name-resolution calls"]
    pub fn resolve(uid: Uid, gid: Gid) -> Self {
        use nix::unistd::{Gid as NixGid, Group as NixGroup, Uid as NixUid, User as NixUser};

        let user = if let Ok(Some(entry)) = NixUser::from_uid(NixUid::from_raw(uid.as_u32())) {
            if let Ok(resolved) = LinuxUser::from_raw(uid, &entry.name) {
                resolved
            } else {
                LinuxUser::from_uid(uid)
            }
        } else {
            LinuxUser::from_uid(uid)
        };

        let group = if let Ok(Some(entry)) = NixGroup::from_gid(NixGid::from_raw(gid.as_u32())) {
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
    #[must_use = "security finding indicator; discarding this silently misses orphaned account ownership on CUI files"]
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
    #[must_use = "returns a validated username + group pair for configuration-level identity; discarding it loses the identity record"]
    pub const fn new(username: LinuxUsername, primary_group: LinuxGroupName) -> Self {
        Self {
            username,
            primary_group,
        }
    }

    /// Convenience constructor from raw strings — validates both at once.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the user or group cannot be resolved from the system database.
    pub fn from_raw(username: &str, group: &str) -> Result<Self, PosixNameError> {
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
// current_username — process-level session identity
//
// Separate from LinuxOwnership::resolve (which is for filesystem objects).
// This resolves the *running process owner* for TUI header / audit display.
// ────────────────────────────────────────────────────────────────────────────

/// Resolve the current process's username for display in TUI headers and
/// operator-facing session lines.
///
/// Resolution order:
///
/// 1. `USER` environment variable — preferred for interactive sessions.
///    If set to a non-empty value, returned as-is without NSS lookup.
///    (`DIRECT-IO-EXCEPTION`: `USER` is read for UX-only session display.
///    No security decision is derived from this value. It is user-controlled
///    and not validated beyond being non-empty.)
/// 2. NSS lookup via `getuid()` → `/etc/passwd` (plus any configured name
///    service: LDAP, SSSD, etc.).  Used when `USER` is absent or empty —
///    common in non-interactive sessions, daemons, and containers.
/// 3. `"(orphan)"` sentinel — emitted when both sources fail.  Matches the
///    file-owner orphan convention used by `umrs_ls::identity::ORPHAN_SENTINEL`
///    and makes the degraded environment visible rather than silently folding
///    it into a numeric uid string.
///
/// The `None`-means-orphan signal from the NSS lookup is preserved at this
/// layer; callers that need the typed `LinuxUser` should call
/// [`LinuxOwnership::resolve`] directly.
///
/// # Compliance
///
/// - **NIST SP 800-53 AU-3**: Audit Record Content — subject identity is a
///   required field in session audit records; this function provides the
///   operator-visible subject for every TUI session header.
/// - **NIST SP 800-53 IA-2**: Identification and Authentication — visible
///   identification of the running user supports operator accountability.
#[must_use = "returns the current session username; discarding it wastes the NSS lookup"]
pub fn current_username() -> String {
    use nix::unistd::{Uid as NixUid, User as NixUser};

    // Step 1: USER env var (UX-only, DIRECT-IO-EXCEPTION — see doc comment)
    if let Ok(val) = std::env::var("USER")
        && !val.is_empty()
    {
        return val;
    }

    // Step 2: NSS resolution via getuid()
    let uid = NixUid::current();
    if let Ok(Some(entry)) = NixUser::from_uid(uid)
        && !entry.name.is_empty()
    {
        return entry.name;
    }

    // Step 3: orphan sentinel — uid has no NSS entry; surface as visible anomaly
    log::warn!(
        "current_username: uid {} has no NSS entry; session identity unresolvable",
        uid.as_raw()
    );
    "(orphan)".to_owned()
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
