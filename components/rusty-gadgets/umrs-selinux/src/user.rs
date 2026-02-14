//!
//! # `SELinux` Security User Identifier
//!
//! Author: Jamie Adams
//!
//! Strongly-typed Rust primitive modeling `SELinux` security users.
//! This module models only the identifier primitive — not policy
//! bindings or clearance mappings.
//!
//! Kernel / Policy Sources Consulted:
//! - security/selinux/include/security.h
//! - security/selinux/ss/policydb.c
//! - libselinux user mapping interfaces
//!
//! In `SELinux` policy, users are symbol table entries associated with:
//! - Role authorization sets
//! - MLS clearance ranges
//! - Login mapping records
//!
//! ## Implementation Lineage & Design Note
//!
//! This module provides an independent Rust implementation of the
//! `SELinux` security user construct.
//!
//! `SELinux` users are policy-defined identity symbols that participate
//! in clearance mapping, role association, and login translation
//! (e.g., via seusers and login mapping databases).
//!
//! Behavioral semantics were studied from `SELinux` userland libraries
//! and policydb structures to preserve familiarity for experienced
//! `SELinux` practitioners. However:
//!
//! - No source code has been copied or translated.
//! - No line-by-line derivation has occurred.
//!
//! This implementation introduces strong typing and construction-time
//! validation to prevent malformed security contexts and improve
//! assurance in higher-level labeling systems.

use std::fmt;
use std::str::FromStr;

//
// =============================================================================
// SelinuxUser Primitive
// =============================================================================
//
// Represents a validated `SELinux` security user identifier.
//
// Example values:
//
//   system_u
//   staff_u
//   user_u
//
// Validation rules enforced:
//
// • ASCII only
// • No whitespace
// • Character set: [a-z0-9_]
// • Must end in "_u"
// • Non-empty identifier stem
// • Length 3–255 bytes
//

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SelinuxUser(String);

pub const MAX_USER_LEN: usize = 255;
pub const MIN_USER_LEN: usize = 3;

//
// =============================================================================
// Error Taxonomy
// =============================================================================
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserError {
    Empty,
    TooLong(usize),
    InvalidCharacter(char),
    InvalidSuffix,
    InvalidStem,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "SELinux user cannot be empty")
            }

            Self::TooLong(len) => {
                write!(f, "SELinux user exceeds maximum length ({len})")
            }

            Self::InvalidCharacter(ch) => {
                write!(f, "invalid character '{ch}' in SELinux user")
            }

            Self::InvalidSuffix => {
                write!(f, "SELinux user has an invalid suffix")
            }

            Self::InvalidStem => {
                write!(f, "SELinux user has an invalid stem")
            }
        }
    }
}

impl std::error::Error for UserError {}


//
// =============================================================================
// Constructors
// =============================================================================
//

impl SelinuxUser {
    ///
    /// Creates a new validated `SELinux` user identifier.
    ///
    /// Validation rules:
    /// • ASCII only
    /// • No whitespace
    /// • Must end with `_u`
    /// • Length within policy bounds
    ///
    /// # Errors
    ///
    /// Returns `UserError` if:
    /// • The identifier contains non-ASCII characters.
    /// • The identifier contains whitespace.
    /// • The identifier does not follow `SELinux` naming conventions.
    /// • The identifier exceeds length constraints.
    ///
    pub fn new<S: Into<String>>(input: S) -> Result<Self, UserError> {
        let value = input.into();

        validate_user(&value)?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

//
// =============================================================================
// Validation Logic
// =============================================================================
//

fn validate_user(value: &str) -> Result<(), UserError> {
    if value.is_empty() {
        return Err(UserError::Empty);
    }

    if value.len() > MAX_USER_LEN {
        return Err(UserError::TooLong(value.len()));
    }

    if value.len() < MIN_USER_LEN {
        return Err(UserError::InvalidStem);
    }

    for ch in value.chars() {
        if !ch.is_ascii_lowercase()
            && !ch.is_ascii_digit()
            && ch != '_'
        {
            return Err(UserError::InvalidCharacter(ch));
        }
    }

    if !value.ends_with("_u") {
        return Err(UserError::InvalidSuffix);
    }

    // Ensure identifier stem is non-empty
    let stem = &value[..value.len() - 2];

    if stem.is_empty() {
        return Err(UserError::InvalidStem);
    }

    Ok(())
}

//
// =============================================================================
// Trait Implementations
// =============================================================================
//

impl fmt::Display for SelinuxUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SelinuxUser {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for SelinuxUser {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
