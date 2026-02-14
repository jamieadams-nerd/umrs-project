//!
//! UMRS SELinux — Security Context Primitive
//!
//! This module defines the strongly-typed `SecurityContext` structure used
//! throughout the UMRS SELinux userland modeling layer.
//!
//! A Security Context represents the canonical SELinux label format:
//!
//     user : role :type [:level]
//!
//! This implementation intentionally models the structural components as
//! discrete typed fields rather than an unstructured string. This supports
//! high-assurance validation, deterministic serialization, and future MLS
//! expansion without redesign.
//!
//! ## NOTE:
//!
//! Level handling is not yet implemented in this initial primitive and will
//! be introduced in a later phase once MLS datatypes are finalized.
//!
//! ## Implementation Lineage & Design Note:
//!
//! This module provides an independent, original implementation functionality conceptually 
//! comparable to traditional userland libraries. Behavioral interfaces and operational 
//! semantics were studied ensure familiarity for long-time SELinux developers.
//! No source code has been copied or translated, and line-by-line reimplementation 
//! was performed. Where appropriate, this implementation takes advantage of RUST language 
//! features such as strong typing, validation at and memory safety guarantees to improve
//! and assurance beyond legacy approaches.

use std::fmt;
use std::str::FromStr;

use crate::user::SelinuxUser;
use crate::role::SelinuxRole;
use crate::type_id::SelinuxType;

///
/// `SecurityContext`
///
/// Strongly-typed representation of an SELinux security context.
///
/// Canonical format:
///
//     `user:role:type`
///
/// Level / MLS components will be integrated once the MLS primitive layer
/// is completed.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct SecurityContext {
             user: SelinuxUser,
             role: SelinuxRole,
    security_type: SelinuxType,
}

impl SecurityContext {
    /// 
    /// new
    /// 
    /// Constructs a new `SecurityContext` from validated primitive components.
    ///
    /// This constructor assumes the supplied primitives have already passed
    /// their respective validation routines.
    ///
    pub const fn new(
        user: SelinuxUser,
        role: SelinuxRole,
        security_type: SelinuxType,
    ) -> Self {
        Self { user, role, security_type }
    }

    /// Returns the SELinux user component.
    #[must_use]
    pub const fn user(&self) -> &SelinuxUser {
        &self.user
    }

    /// Returns the SELinux role component.
    #[must_use]
    pub const fn role(&self) -> &SelinuxRole {
        &self.role
    }

    /// Returns the SELinux type component.
    #[must_use]
    pub const fn security_type(&self) -> &SelinuxType {
        &self.security_type
    }
}

/// 
/// Display Implementation
///
///
/// Provides canonical string serialization in standard SELinux format.
///
impl fmt::Display for SecurityContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.user, self.role, self.security_type)
    }
}

/// 
/// Parse Errors
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextParseError {
    /// Context string did not contain the required 3 fields.
    InvalidFormat,

    /// User field failed validation.
    InvalidUser,

    /// Role field failed validation.
    InvalidRole,

    /// Type field failed validation.
    InvalidType,
}

impl fmt::Display for ContextParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => {
                write!(f, "invalid security context format")
            }
            Self::InvalidUser => {
                write!(f, "invalid SELinux user field")
            }
            Self::InvalidRole => {
                write!(f, "invalid SELinux role field")
            }
            Self::InvalidType => {
                write!(f, "invalid SELinux type field")
            }
        }
    }
}

impl std::error::Error for ContextParseError {}

/// 
/// `FromStr` Implementation
/// 
/// Enables parsing from canonical SELinux context strings.
///
/// Example:
///
//     let ctx: SecurityContext =
//         "system_u:system_r:sshd_t".parse().unwrap();
///
impl FromStr for SecurityContext {
    type Err = ContextParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 3 {
            return Err(ContextParseError::InvalidFormat);
        }

        let user = SelinuxUser::from_str(parts[0])
            .map_err(|_| ContextParseError::InvalidUser)?;

        let role = SelinuxRole::from_str(parts[1])
            .map_err(|_| ContextParseError::InvalidRole)?;

        let security_type = SelinuxType::from_str(parts[2])
            .map_err(|_| ContextParseError::InvalidType)?;

        Ok(Self::new(user, role, security_type))
    }
}
