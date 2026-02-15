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

use crate::category::CategorySet;
use crate::role::SelinuxRole;
use crate::sensitivity::SensitivityLevel;
use crate::type_id::SelinuxType;
use crate::user::SelinuxUser;

// Note: CategorySet will be integrated here for full MLS/MCS bitmask support.

// ===========================================================================
// MlsLevel structure (Sensitivity + Categories)
// ===========================================================================
/// Represents the hierarchical and non-hierarchical components of an MLS label.
/// NSA RTB Principle: Determinism via strong data modeling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MlsLevel {
    pub sensitivity: SensitivityLevel,
    pub categories: CategorySet,

    /// NIST 800-53 AU-3: The exact string found in the xattr (e.g., "SystemLow")
    pub raw_level: String,
}

impl MlsLevel {
    /// Returns the raw, untranslated string (Provenance).
    pub fn raw(&self) -> &str {
        &self.raw_level
    }

    /// Returns the translated, canonical string (Lattice representation).
    /// e.g., s0:c0.c15
    pub fn translated(&self) -> String {
        // This gives you the s0: categories view
        format!("{}:{}", self.sensitivity, self.categories)
    }
}

impl fmt::Display for MlsLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Default display to the raw level as requested
        write!(f, "{}", self.raw_level)
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
            Some(lvl) => write!(
                f,
                "{}:{}:{}:{}",
                self.user, self.role, self.security_type, lvl
            ),
            None => {
                write!(f, "{}:{}:{}", self.user, self.role, self.security_type)
            }
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
    ///
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

        //
        // Senstivity Level and Categories
        //
        let level = if parts.len() == 4 {
            let level_raw = parts[3];
            log::debug!("[PATH B] Raw Level string: '{}'", level_raw);

            // 1. Sensitivity Logic
            let sens = match SensitivityLevel::from_str(level_raw) {
                Ok(s) => s,
                Err(_) => SensitivityLevel::new(0).unwrap(),
            };

            // 2. Category Logic
            let cats = crate::xattrs::parse_mcs_categories(level_raw)
                .unwrap_or_else(|_| CategorySet::new());

            log::debug!("[PATH B] Sens resolved, categories parsed.");

            Some(MlsLevel {
                sensitivity: sens,
                categories: cats,
                raw_level: level_raw.to_string(),
            })
        } else {
            log::debug!("[PATH B] No level found (parts.len != 4)");
            None
        };

        Ok(Self::new(user, role, security_type, level))
    }
}
