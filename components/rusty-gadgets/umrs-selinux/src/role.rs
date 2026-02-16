// SPDX-License-Identifier: MIT                                                                     
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//! # SELinux Security Role Identifier
//!
//! Strongly-typed Rust primitive modeling SELinux security roles.
//!
//! This module provides an independent Rust implementation of the
//! SELinux security role construct. It models only the identifier 
//! primitive — not policy bindings, transitions, or authorization rules.
//!
//! Kernel / Policy Sources Consulted:
//! - security/selinux/ss/policydb.c
//! - security/selinux/include/security.h
//! - libselinux RBAC interfaces
//!
//! In SELinux policy, roles are symbol table entries associated with:
//! - Authorized domain (type) sets
//! - Role transition rules
//! - User-role authorization mappings
//!
//! ## Implementation Lineage & Design Note
//!
//! Behavioral semantics were studied from SELinux userland libraries
//! and policydb structures to preserve familiarity for experienced
//! SELinux practitioners. However:
//!
//! - No source code has been copied or translated.
//! - No line-by-line derivation has occurred.
//!
//! This implementation introduces strong typing and construction-time
//! validation to prevent malformed security contexts and improve
//! assurance in higher-level labeling systems.
//!
// =============================================================================

use std::fmt;
use std::str::FromStr;

//
// =============================================================================
// SelinuxRole Primitive
// =============================================================================
//
// Represents a validated `SELinux` security role identifier.
//
// Example values:
//
//   system_r
//   staff_r
//   object_r
//
// Validation rules enforced:
//
// • ASCII only
// • No whitespace
// • Character set: [a-z0-9_]
// • Must end in "_r"
// • Non-empty identifier stem
// • Length 3–255 bytes
//

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SelinuxRole(String);

pub const MAX_ROLE_LEN: usize = 255;
pub const MIN_ROLE_LEN: usize = 3;

//
// =============================================================================
// Error Taxonomy
// =============================================================================
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleError {
    Empty,
    TooLong(usize),
    InvalidCharacter(char),
    InvalidSuffix,
    InvalidStem,
}

impl fmt::Display for RoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "SELinux role cannot be empty")
            }

            Self::TooLong(len) => {
                write!(f, "SELinux role exceeds maximum length ({len})")
            }

            Self::InvalidCharacter(ch) => {
                write!(f, "invalid character '{ch}' in SELinux role")
            }

            Self::InvalidSuffix => {
                write!(f, "SELinux role has an invalid suffix")
            }

            Self::InvalidStem => {
                write!(f, "SELinux role has an invalid stem")
            }
        }
    }
}

impl std::error::Error for RoleError {}


//
// =============================================================================
// Constructors
// =============================================================================
//

impl SelinuxRole {

    /// Creates a new validated `SELinux` role identifier.
    ///
    /// Validation rules:
    /// • ASCII only
    /// • No whitespace
    /// • Must end with `_r`
    /// • Length within policy bounds
    ///
    /// # Errors
    ///
    /// Returns `RoleError` if:
    /// • The identifier contains non-ASCII characters.
    /// • The identifier contains whitespace.
    /// • The identifier does not follow `SELinux` naming conventions.
    /// • The identifier exceeds length constraints.
    ///
    pub fn new<S: Into<String>>(input: S) -> Result<Self, RoleError> {
        let value = input.into();

        validate_role(&value)?;

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

fn validate_role(value: &str) -> Result<(), RoleError> {
    if value.is_empty() {
        return Err(RoleError::Empty);
    }

    if value.len() > MAX_ROLE_LEN {
        return Err(RoleError::TooLong(value.len()));
    }

    if value.len() < MIN_ROLE_LEN {
        return Err(RoleError::InvalidStem);
    }

    for ch in value.chars() {
        if !ch.is_ascii_lowercase()
            && !ch.is_ascii_digit()
            && ch != '_'
        {
            return Err(RoleError::InvalidCharacter(ch));
        }
    }

    if !value.ends_with("_r") {
        return Err(RoleError::InvalidSuffix);
    }

    // Ensure identifier stem is non-empty
    let stem = &value[..value.len() - 2];

    if stem.is_empty() {
        return Err(RoleError::InvalidStem);
    }

    Ok(())
}

//
// =============================================================================
// Trait Implementations
// =============================================================================
//

impl fmt::Display for SelinuxRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SelinuxRole {
    type Err = RoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for SelinuxRole {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
