// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//
//! # Security Context (Security Label)
//!
//! Defines the strongly-typed `SecurityContext` structure used throughout
//! the UMRS SELinux userland modeling layer.
//!
//! A security context represents the canonical SELinux label format:
//!
//! ```text
//! user : role : type [:level]
//! ```
//!
//! ## TPI Parse Architecture
//!
//! `SecurityContext` is never constructed from a raw string in this module.
//! The authoritative construction path is `SecureXattrReader::read_context()`
//! in `xattrs.rs`, which enforces Two-Path Independence (TPI):
//!
//! - **Path A**: `nom` combinator parser
//! - **Path B**: `FromStr` (implemented below)
//!
//! Both paths are always run. If they disagree, the read fails closed with
//! `TpiError::Disagreement`. This module provides `FromStr` as Path B only;
//! it must not be called as the sole parse path in security-relevant contexts.
//!
//! ## Note on `MlsLevel` Duplication
//!
//! `MlsLevel` is defined both here (as the type used by `SecurityContext`) and
//! in `mls/level.rs`. The crate root re-exports `mls::level::MlsLevel` as the
//! canonical type. The two definitions are structurally equivalent; the
//! duplication is a known technical debt item. Consumers should import
//! `umrs_selinux::MlsLevel` from the crate root.
//!
//! ## Note on `dominates()`
//!
//! `SecurityContext::dominates()` is currently a `todo!()` stub. The lattice
//! dominance logic is pending integration of the `CategorySet` bitmask
//! comparison with `SensitivityLevel` ordering. Callers must not rely on this
//! method returning a meaningful result until the stub is resolved.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — `SecurityContext`
//!   is the primary carrier of the MLS label used in flow decisions.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — TPI parse
//!   architecture ensures label parse integrity; disagreements are integrity events.
//! - **NSA RTB**: Strong data modeling — all label fields are typed newtypes;
//!   no raw strings escape the parse boundary.

use std::fmt;
use std::str::FromStr;

#[cfg(debug_assertions)]
use std::time::Instant;

use crate::category::CategorySet;
use crate::role::SelinuxRole;
use crate::sensitivity::SensitivityLevel;
use crate::type_id::SelinuxType;
use crate::user::SelinuxUser;

// ===========================================================================
// MlsLevel structure (Sensitivity + Categories)
// ===========================================================================
/// Represents the hierarchical and non-hierarchical components of an MLS label.
/// NSA RTB Principle: Determinism via strong data modeling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MlsLevel {
    pub sensitivity: SensitivityLevel,
    pub categories: CategorySet,

    /// NIST SP 800-53 AU-3: The exact string found in the xattr (e.g., "SystemLow")
    pub raw_level: String,
}

impl MlsLevel {
    /// Returns the raw, untranslated string (Provenance).
    #[must_use = "pure accessor returning the provenance-preserving raw level string"]
    pub fn raw(&self) -> &str {
        &self.raw_level
    }

    /// Returns the translated, canonical string (Lattice representation).
    /// e.g., s0:c0.c15
    #[must_use = "returns owned canonical MLS level string; discarding it wastes the allocation"]
    pub fn translated(&self) -> String {
        format!("{}:{}", self.sensitivity, self.categories)
    }
}

impl fmt::Display for MlsLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw_level)
    }
}

// ===========================================================================
// SecurityContext structure
// ===========================================================================
/// NIST SP 800-53 AC-3: Access Enforcement logic depends on this structure.
/// NSA RTB: Minimized TCB via strictly bounded data structures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use = "security context carries the MAC label used in access control decisions; discarding it bypasses label enforcement"]
pub struct SecurityContext {
    user: SelinuxUser,
    role: SelinuxRole,
    security_type: SelinuxType,
    level: Option<MlsLevel>,
}

impl SecurityContext {
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

    #[must_use = "pure accessor; SELinux user component is required for context rendering and policy lookup"]
    pub const fn user(&self) -> &SelinuxUser {
        &self.user
    }

    #[must_use = "pure accessor; SELinux role component is required for context rendering and RBAC decisions"]
    pub const fn role(&self) -> &SelinuxRole {
        &self.role
    }

    #[must_use = "pure accessor; SELinux type is the primary enforcement anchor in type enforcement policy"]
    pub const fn security_type(&self) -> &SelinuxType {
        &self.security_type
    }

    #[must_use = "pure accessor; MLS level is required for dominance checks and CUI marking resolution"]
    pub const fn level(&self) -> Option<&MlsLevel> {
        self.level.as_ref()
    }

    /// NIST SP 800-53 AC-4: Information Flow Enforcement
    #[must_use = "dominance result determines whether information flow is permitted; discarding it bypasses the flow enforcement decision"]
    pub fn dominates(&self, _other: &Self) -> bool {
        todo!("Lattice dominance logic pending CategorySet bitmask integration")
    }
}

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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(debug_assertions)]
        let start = Instant::now();

        // Path B: allocation-free field extraction.
        //
        // `splitn(5, ':')` splits into at most 5 tokens. With the security
        // context format `user:role:type[:sensitivity[:categories]]`, the 5th
        // token captures everything after the 4th colon — i.e. the category
        // remainder that may itself contain commas but never colons.
        //
        // AXIOM (selinux.md): Targeted policy has exactly one sensitivity level,
        // `s0`. MCS categories at `s0` are comma-separated and contain no colons.
        // Phase 1 is targeted policy only, so the 5th token is always a flat
        // category list with no embedded colon separators.
        //
        // Using splitn avoids collecting a Vec<&str> entirely; each token is
        // consumed in a single forward pass.
        //
        // NIST SP 800-53 SI-7: Path B is one of two independent parse paths in
        // the TPI gate. It must not be called as the sole parse path.
        let mut iter = s.splitn(5, ':');

        let user_str = iter.next().ok_or(ContextParseError::InvalidFormat)?;
        let role_str = iter.next().ok_or(ContextParseError::InvalidFormat)?;
        let type_str = iter.next().ok_or(ContextParseError::InvalidFormat)?;

        let user = SelinuxUser::from_str(user_str).map_err(|_| ContextParseError::InvalidUser)?;
        let role = SelinuxRole::from_str(role_str).map_err(|_| ContextParseError::InvalidRole)?;
        let security_type =
            SelinuxType::from_str(type_str).map_err(|_| ContextParseError::InvalidType)?;

        // Path B: Greedy Level Capture (NIST SP 800-53 SI-7)
        // SI-11: do not log the raw level string — it may contain MLS sensitivity
        // and category values that are security-relevant.
        let level = match iter.next() {
            None => {
                #[cfg(debug_assertions)]
                log::debug!("[PATH B] No level field.");
                None
            }
            Some(sens_str) => {
                // The 5th splitn token (if any) is the category remainder; this
                // is everything after the 4th colon. For `s0:c0,c100` the
                // sensitivity token is `s0` and the remainder token is `c0,c100`.
                // We reconstruct `raw_level` from these two parts — avoiding the
                // `parts[3..].join(":")` allocation used in the Vec path.
                let cats_remainder = iter.next().unwrap_or("");

                #[cfg(debug_assertions)]
                log::debug!("[PATH B] Level field present.");

                // 1. Sensitivity parse.
                let sens = SensitivityLevel::from_str(sens_str).unwrap_or_else(|_| {
                    SensitivityLevel::new(0)
                        .expect("Invariant failure: SensitivityLevel::new(0) must succeed")
                });

                // 2. Category parse — cats_remainder is already the post-sensitivity
                //    portion, so no second split is needed.
                let cats = crate::xattrs::parse_mcs_categories(cats_remainder)
                    .unwrap_or_else(|_| CategorySet::new());

                // 3. Reconstruct raw_level for audit provenance (NIST SP 800-53 AU-3).
                //    One allocation is unavoidable here since MlsLevel stores String.
                let raw_level = if cats_remainder.is_empty() {
                    sens_str.to_owned()
                } else {
                    format!("{sens_str}:{cats_remainder}")
                };

                #[cfg(debug_assertions)]
                log::debug!("[PATH B] Sensitivity resolved, categories parsed.");

                Some(MlsLevel {
                    sensitivity: sens,
                    categories: cats,
                    raw_level,
                })
            }
        };

        let result = Self::new(user, role, security_type, level);

        #[cfg(debug_assertions)]
        log::debug!(
            "SecurityContext::from_str (Path B) completed in {} µs",
            start.elapsed().as_micros(),
        );

        Ok(result)
    }
}
