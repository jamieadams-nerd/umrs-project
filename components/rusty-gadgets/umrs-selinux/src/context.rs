//!
//! Security Context (a.k.a, Security Label or just Label)
//!
//! Author: Jamie Adams (a.k.a, Imodium Operator)
//!
//! This module defines the strongly-typed `SecurityContext` structure used
//! throughout the UMRS SELinux userland modeling layer.
//!
//! A Security Context represents the canonical SELinux label format:
//!
//     user : role :type [:level]
//!
use std::fmt;
use std::str::FromStr;

use crate::role::SelinuxRole;
use crate::type_id::SelinuxType;
use crate::user::SelinuxUser;

// ===========================================================================
// SecurityContext structure
// ===========================================================================
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct SecurityContext {
    user: SelinuxUser,
    role: SelinuxRole,
    security_type: SelinuxType,
}

impl SecurityContext {
    pub const fn new(
        user: SelinuxUser,
        role: SelinuxRole,
        security_type: SelinuxType,
    ) -> Self {
        Self {
            user,
            role,
            security_type,
        }
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

/// Provides canonical string serialization in standard SELinux format.
impl fmt::Display for SecurityContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.user, self.role, self.security_type)
    }
}

// ===========================================================================
// Error Taxonomy
// ===========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextParseError {
    InvalidFormat,
    InvalidUser,
    InvalidRole,
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

// ===========================================================================
// Traits Implementations
// ===========================================================================
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
