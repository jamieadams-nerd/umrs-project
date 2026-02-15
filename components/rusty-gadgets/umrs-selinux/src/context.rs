// ============================================================================
// UMRS SELINUX: Security Context (Security Label)
// NIST 800-53 AC-4 / NSA RTB (Strong Data Modeling & Lattice Math)
// ============================================================================
//! Security Context (a.k.a, Security Label or just Label)
//!
//! Author: Jamie Adams (a.k.a, Imodium Operator)
//!
//! This module defines the strongly-typed `SecurityContext` structure used
//! throughout the UMRS SELinux userland modeling layer.
//!
//! A Security Context represents the canonical SELinux label format:
//!     user : role : type [:level]
//!
//! NIST 800-53 AC-4: This module enforces the internal representation of 
//! security attributes used for Information Flow Enforcement.

use std::fmt;
use std::str::FromStr;

use crate::role::SelinuxRole;
use crate::type_id::SelinuxType;
use crate::user::SelinuxUser;
use crate::sensitivity::SensitivityLevel;
use crate::category::CategorySet;

// Note: CategorySet will be integrated here for full MLS/MCS bitmask support.

// ===========================================================================
// MlsLevel structure (Sensitivity + Categories)
// ===========================================================================
/// Represents the hierarchical and non-hierarchical components of an MLS label.
/// NSA RTB Principle: Determinism via strong data modeling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MlsLevel {
    pub sensitivity: SensitivityLevel,
    pub categories: CategorySet, // Integrated in next phase
}

impl fmt::Display for MlsLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sensitivity)
    }
}

// ===========================================================================
// SecurityContext structure
// ===========================================================================
/// NIST 800-53 AC-3: Access Enforcement logic depends on this structure.
/// NSA RTB: Minimized TCB via strictly bounded data structures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct SecurityContext {
    user: SelinuxUser,
    role: SelinuxRole,
    security_type: SelinuxType,
    level: Option<MlsLevel>,
}

impl SecurityContext {
    /// Creates a new SecurityContext with optional MLS level.
    pub const fn new(
        user: SelinuxUser,
        role: SelinuxRole,
        security_type: SelinuxType,
        level: Option<MlsLevel>,
    ) -> Self {
        Self {
            user,
            role,
            security_type,
            level,
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

    /// Returns the optional MLS/MCS level component.
    #[must_use]
    pub fn level(&self) -> Option<&MlsLevel> {
        self.level.as_ref()
    }

    /// NIST 800-53 AC-4: Information Flow Enforcement
    /// TODO: Implement Bell-LaPadula Dominance Check (Lattice Math)
    /// (Subject Sensitivity >= Object Sensitivity) AND (Subject Categories ⊇ Object Categories)
    pub fn dominates(&self, _other: &Self) -> bool {
        todo!("Lattice dominance logic pending CategorySet bitmask integration")
    }

}

/// Provides canonical string serialization in standard SELinux format.
impl fmt::Display for SecurityContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.level {
            Some(lvl) => write!(f, "{}:{}:{}:{}", self.user, self.role, self.security_type, lvl),
            None => write!(f, "{}:{}:{}", self.user, self.role, self.security_type),
        }
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
    InvalidLevel,
}

impl fmt::Display for ContextParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid security context format"),
            Self::InvalidUser => write!(f, "invalid SELinux user field"),
            Self::InvalidRole => write!(f, "invalid SELinux role field"),
            Self::InvalidType => write!(f, "invalid SELinux type field"),
            Self::InvalidLevel => write!(f, "invalid SELinux level/MLS field"),
        }
    }
}

impl std::error::Error for ContextParseError {}

// ===========================================================================
// Traits Implementations
// ===========================================================================
impl FromStr for SecurityContext {
    type Err = ContextParseError;

    /// NIST 800-53 SI-7: Software Integrity 
    /// This parser handles both 3-part (TE only) and 4-part (MLS) labels.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() < 3 || parts.len() > 4 {
            return Err(ContextParseError::InvalidFormat);
        }

        let user = SelinuxUser::from_str(parts[0])
            .map_err(|_| ContextParseError::InvalidUser)?;

        let role = SelinuxRole::from_str(parts[1])
            .map_err(|_| ContextParseError::InvalidRole)?;

        let security_type = SelinuxType::from_str(parts[2])
            .map_err(|_| ContextParseError::InvalidType)?;

        let level = if parts.len() == 4 {
            let level_input = if parts[3] == "SystemLow" { "s0" } else { parts[3] };
            
            let sens = SensitivityLevel::from_str(level_input)
                .map_err(|_| ContextParseError::InvalidLevel)?;
            
            // Reusing the same helper for TPI agreement
            let cats = crate::xattrs::parse_mcs_categories(level_input)
                .unwrap_or_else(|_| CategorySet::new());

            Some(MlsLevel { sensitivity: sens, categories: cats })
        } else {
            None
        };

        Ok(Self::new(user, role, security_type, level))
    }
}

