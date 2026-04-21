// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// =============================================================================
//! # SELinux Security Type Identifier
//!
//! Strongly-typed Rust primitive modeling `SELinux` security types.
//! This module models only the identifier primitive — not policy rule
//! bindings, transitions, or attribute associations.
//!
//! Kernel / Policy Sources Consulted:
//!
//!   security/selinux/ss/policydb.c
//!   security/selinux/include/security.h
//!   libselinux type and context interfaces
//!
//! In SELinux policy, types are symbol table entries associated with:
//!
//! - Domain execution contexts
//! - Object labeling rules
//! - Type transitions
//! - Allow/deny TE rules
//!
//! This module models only the identifier primitive — not policy rule
//! bindings, transitions, or attribute associations.
//!
//! ## Implementation Lineage & Design Note
//! This module provides an independent Rust implementation of the
//! SELinux security type construct.
//!
//! SELinux types (domains) are the primary enforcement anchors within
//! Type Enforcement (TE). They define process domains, object classes,
//! transition boundaries, and allow/deny rule applicability.
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
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access Enforcement — `SelinuxType` is the domain
//!   (type) component of the SELinux security context; correct type identity is
//!   the primary enforcement anchor for Type Enforcement (TE) policy decisions.
//! - **NIST SP 800-53 AC-16**: Security and Privacy Attributes — type identifiers
//!   are validated at construction time to ensure only well-formed, policy-legal
//!   values are admitted into security context structures.
//! - **NSA RTB RAIN**: Non-Bypassable — the private constructor and newtype
//!   wrapper ensure callers cannot instantiate an unvalidated `SelinuxType`;
//!   every value of this type has passed the format invariant.
//!
// =============================================================================
use std::fmt;
use std::str::FromStr;

// =============================================================================
// SelinuxType Primitive
//
// Represents a validated `SELinux` security type identifier.
//
// Example values:
//
//   sshd_t
//   var_log_t
//   httpd_t
//   NetworkManager_etc_t
//
// Validation rules enforced:
//
// • ASCII only
// • No whitespace
// • Character set: [a-zA-Z0-9_]
// • Must end in "_t"
// • Non-empty identifier stem
// • Length 3–255 bytes
//
// Note: SELinux type identifiers may begin with or contain uppercase letters.
// Third-party and distribution-supplied policy modules (e.g., NetworkManager)
// use mixed-case names. The character set is aligned with the SELinux kernel
// policy source parser, which accepts [a-zA-Z0-9_] for type identifiers.
//

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SelinuxType(String);

pub const MAX_TYPE_LEN: usize = 255;
pub const MIN_TYPE_LEN: usize = 3;

// =============================================================================
// Error Taxonomy
// =============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    Empty,
    TooLong(usize),
    InvalidCharacter(char),
    InvalidSuffix,
    InvalidStem,
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "SELinux type cannot be empty")
            }

            Self::TooLong(len) => {
                write!(f, "SELinux type exceeds maximum length ({len})")
            }

            Self::InvalidCharacter(ch) => {
                write!(f, "invalid character '{ch}' in SELinux type")
            }

            Self::InvalidSuffix => {
                write!(f, "SELinux type has an invalid suffix")
            }

            Self::InvalidStem => {
                write!(f, "SELinux type has an invalid stem")
            }
        }
    }
}

impl std::error::Error for TypeError {}

// =============================================================================
// Constructors
// =============================================================================
impl SelinuxType {
    ///
    /// Creates a new validated `SELinux` type identifier.
    ///
    /// Validation rules:
    /// • ASCII only
    /// • No whitespace
    /// • Character set: `[a-zA-Z0-9_]` — uppercase letters are permitted
    /// • Must end with `_t`
    /// • Length within policy bounds (3–255 bytes)
    ///
    /// Real-world policy types such as `NetworkManager_etc_t` begin with or
    /// contain uppercase letters. The character set matches the SELinux kernel
    /// policy parser acceptance rules.
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if:
    /// • The identifier contains non-ASCII characters.
    /// • The identifier contains whitespace or non-alphanumeric/underscore bytes.
    /// • The identifier does not follow `SELinux` naming conventions.
    /// • The identifier exceeds length constraints.
    ///
    pub fn new<S: Into<String>>(input: S) -> Result<Self, TypeError> {
        let value = input.into();

        validate_type(&value)?;

        Ok(Self(value))
    }

    #[must_use = "pure accessor returning the validated SELinux type identifier string"]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Validation Logic
// =============================================================================
fn validate_type(value: &str) -> Result<(), TypeError> {
    if value.is_empty() {
        return Err(TypeError::Empty);
    }

    if value.len() > MAX_TYPE_LEN {
        return Err(TypeError::TooLong(value.len()));
    }

    if value.len() < MIN_TYPE_LEN {
        return Err(TypeError::InvalidStem);
    }

    for ch in value.chars() {
        if !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() && ch != '_' {
            return Err(TypeError::InvalidCharacter(ch));
        }
    }

    if !value.ends_with("_t") {
        return Err(TypeError::InvalidSuffix);
    }

    // Ensure identifier stem is non-empty
    let stem = &value[..value.len() - 2];

    if stem.is_empty() {
        return Err(TypeError::InvalidStem);
    }

    Ok(())
}

// =============================================================================
// Trait Implementations
// =============================================================================
impl fmt::Display for SelinuxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SelinuxType {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for SelinuxType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
