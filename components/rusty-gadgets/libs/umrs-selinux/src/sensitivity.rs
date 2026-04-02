// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//! # SELinux MLS Sensitivity Level
//!
//! Strongly-typed Rust primitive modeling SELinux MLS sensitivity levels
//! (e.g., `s0`–`s15`). Sensitivity levels form the hierarchical portion of an
//! MLS security label and are combined with `CategorySet` bitmaps to produce
//! full `MlsLevel` values.
//!
//! In SELinux MLS policy, sensitivity levels are ordinal — `s3` dominates `s1`.
//! They participate in dominance comparisons, clearance evaluation, and access
//! control decisions.
//!
//! Kernel / Policy Sources Consulted:
//!
//!   security/selinux/ss/mls.c
//!   security/selinux/ss/mls.h
//!   security/selinux/ss/policydb.c
//!
//! No source code has been copied or translated; no line-by-line derivation
//! has occurred.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — sensitivity level
//!   ordering implements the hierarchical component of the Bell-LaPadula
//!   dominance relation.
//! - **NIST SP 800-53 AC-3**: Access Enforcement — sensitivity is required for
//!   every MLS access control decision.
//! - **NSA RTB**: Deterministic Execution — ordinal `u8` representation with
//!   construct-time range validation; no runtime parsing ambiguity.
//!
// =============================================================================

use std::fmt;
use std::str::FromStr;

//
// =============================================================================
// SensitivityLevel Primitive
// =============================================================================
//
// Represents a validated `SELinux` MLS sensitivity level.
//
// Example values:
//
//   s0
//   s1
//   s15
//
// Validation rules enforced:
//
// • ASCII only
// • Must begin with 's'
// • Numeric suffix required
// • Range-bounded (0–65535 default userland bound)
// • Ordered comparison semantics
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SensitivityLevel(u16);

//
// Default policy upper bound.
//
// `SELinux` itself defines sensitivities via policy, but userland
// modeling typically enforces a bounded numeric domain.
//
pub const MAX_SENSITIVITY: u16 = 15;

//
// =============================================================================
// Error Taxonomy
// =============================================================================
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SensitivityError {
    Empty,
    InvalidPrefix,
    InvalidFormat(String),
    OutOfRange(u16),
}

impl fmt::Display for SensitivityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "sensitivity level cannot be empty")
            }

            Self::InvalidPrefix => {
                write!(f, "invalid sensitivity prefix (expected 's')")
            }

            Self::InvalidFormat(raw) => {
                write!(f, "invalid sensitivity format: '{raw}'")
            }

            Self::OutOfRange(val) => {
                write!(f, "sensitivity value out of range ({val})")
            }
        }
    }
}

impl std::error::Error for SensitivityError {}

//
// =============================================================================
// Constructors
// =============================================================================
//

impl SensitivityLevel {
    ///
    /// Creates a new validated MLS sensitivity level.
    ///
    /// Sensitivity levels represent hierarchical classification tiers
    /// within the `SELinux` MLS model (e.g., `s0`, `s1`, `s15`).
    ///
    /// ihis constructor validates that the provided numeric level falls
    /// within the supported sensitivity domain.
    ///
    /// # Errors
    ///
    /// Returns `SensitivityError::OutOfRange` if the provided sensitivity
    /// value exceeds the maximum supported level (`MAX_SENSITIVITY`).
    ///
    pub const fn new(level: u16) -> Result<Self, SensitivityError> {
        if level > MAX_SENSITIVITY {
            return Err(SensitivityError::OutOfRange(level));
        }

        Ok(Self(level))
    }

    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }
}

//
// =============================================================================
// Display Formatting
// =============================================================================
//

impl fmt::Display for SensitivityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s{}", self.0)
    }
}

//
// =============================================================================
// Parsing
// =============================================================================
//

impl FromStr for SensitivityLevel {
    type Err = SensitivityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SensitivityError::Empty);
        }

        if !s.starts_with('s') {
            return Err(SensitivityError::InvalidPrefix);
        }

        let numeric = &s[1..];

        let value: u16 = numeric.parse().map_err(|_| SensitivityError::InvalidFormat(s.into()))?;

        Self::new(value)
    }
}
