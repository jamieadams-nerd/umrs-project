// SPDX-License-Identifier: MIT                                                                     
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//! MLS Level primitive for SELinux contexts.
//!
//! This module models a single MLS/MCS level consisting of:
//!
//! - A sensitivity component (e.g., `s0`, `s2`)
//! - An optional category set (e.g., `c0,c3,c7`)
//!
//! Canonical forms supported:
//!
//! ```text
//! s0
//! s0:c0
//! s0:c0,c1,c7
//! ```
//!
//! Range expressions (e.g., `s0-s3:c0.c1023`) are handled by the
//! `range` module and are intentionally out of scope here.
//!
//! This type is strongly typed and does not rely on libselinux.
// ===========================================================================

use std::fmt;
use std::str::FromStr;

use crate::category::{Category, CategorySet};
use crate::sensitivity::SensitivityLevel;

/// Represents a single MLS/MCS level.
///
/// A level combines a sensitivity classification with
/// an optional set of categories.
///
/// Examples:
///
/// ```text
/// s0
/// s2:c0
/// s3:c1,c7,c42
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct MlsLevel {
    sensitivity: SensitivityLevel,
    categories: CategorySet,
}

impl MlsLevel {
    /// Constructs a new MLS level from validated components.
    ///
    /// This constructor assumes the supplied primitives have
    /// already passed their respective validation checks.
    pub const fn new(
        sensitivity: SensitivityLevel,
        categories: CategorySet,
    ) -> Self {
        Self {
            sensitivity,
            categories,
        }
    }

    /// Returns the sensitivity component.
    #[must_use]
    pub const fn sensitivity(&self) -> &SensitivityLevel {
        &self.sensitivity
    }

    /// Returns the category set.
    #[must_use]
    pub const fn categories(&self) -> &CategorySet {
        &self.categories
    }

    /// Returns true if the level contains any categories.
    #[must_use]
    pub fn has_categories(&self) -> bool {
        !self.categories.is_empty()
    }
}

/// Errors that can occur while parsing an MLS level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MlsLevelError {
    /// The input string was empty.
    Empty,

    /// The level string was malformed.
    InvalidFormat,

    /// The sensitivity component failed validation.
    InvalidSensitivity,

    /// One or more categories failed validation.
    InvalidCategory,
}

impl fmt::Display for MlsLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "MLS level cannot be empty")
            }
            Self::InvalidFormat => {
                write!(f, "invalid MLS level format")
            }
            Self::InvalidSensitivity => {
                write!(f, "invalid sensitivity component in MLS level")
            }
            Self::InvalidCategory => {
                write!(f, "invalid category component in MLS level")
            }
        }
    }
}

impl std::error::Error for MlsLevelError {}

impl fmt::Display for MlsLevel {
    /// Serializes the MLS level into canonical SELinux form.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.categories.is_empty() {
            write!(f, "{}", self.sensitivity)
        } else {
            write!(f, "{}:{}", self.sensitivity, self.categories)
        }
    }
}

impl FromStr for MlsLevel {
    type Err = MlsLevelError;

    /// Parses an MLS level from canonical string form.
    ///
    /// Supported inputs:
    ///
    /// ```text
    /// s0
    /// s0:c0
    /// s0:c0,c1
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(MlsLevelError::Empty);
        }

        // Split sensitivity from categories.
        let mut parts = s.splitn(2, ':');

        let sens_str =
            parts.next().ok_or(MlsLevelError::InvalidFormat)?;

        let sensitivity =
            SensitivityLevel::from_str(sens_str)
                .map_err(|_| MlsLevelError::InvalidSensitivity)?;

        let categories = match parts.next() {
            None => CategorySet::default(),
            Some(raw) => parse_categories(raw)?,
        };

        Ok(Self::new(sensitivity, categories))
    }
}

/// Parses comma-separated categories into a CategorySet.
fn parse_categories(raw: &str) -> Result<CategorySet, MlsLevelError> {
    let raw = raw.trim();

    if raw.is_empty() {
        return Err(MlsLevelError::InvalidFormat);
    }

    let mut set = CategorySet::default();

    for token in raw.split(',') {
        let tok = token.trim();

        if tok.is_empty() {
            return Err(MlsLevelError::InvalidFormat);
        }

        let cat = Category::from_str(tok)
            .map_err(|_| MlsLevelError::InvalidCategory)?;

        set.insert(cat);
    }

    Ok(set)
}

