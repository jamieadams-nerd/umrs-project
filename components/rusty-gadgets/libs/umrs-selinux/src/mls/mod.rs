// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//! # SELinux Multi-Level Security (MLS) Namespace
//!
//! SELinux MLS is a strict mandatory access control mechanism based on the
//! Bell-LaPadula confidentiality model. It restricts data access by enforcing
//! "no read-up, no write-down" across hierarchical sensitivity levels and
//! non-hierarchical category compartments.
//!
//! An MLS level is composed of:
//!
//! ```text
//! SensitivityLevel : CategorySet
//! ```
//!
//! Example: `s3:c0,c2,c9`
//!
//! Where:
//! - `SensitivityLevel` defines the hierarchical classification tier.
//! - `CategorySet` defines non-hierarchical compartment membership.
//!
//! ## Sub-modules
//!
//! - `level` — `MlsLevel`: a single sensitivity + category-set pair.
//! - `range` — `MlsRange`: a low–high clearance span (e.g., `s0-s3:c0.c1023`).
//!
//! Primitive components (`SensitivityLevel`, `CategorySet`) are defined at the
//! crate root and consumed here.
//!
//! No SELinux kernel or userland source code has been copied or translated.
//! All implementations are original Rust constructions aligned at the semantic
//! level only.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — MLS dominance
//!   math implements the Bell-LaPadula model for information flow control.
//! - **NIST SP 800-53 AC-3**: Access Enforcement — level and range containment
//!   checks drive mandatory access control decisions.
//! - **NSA RTB**: Deterministic Execution — dominance is a pure boolean function
//!   of typed values; no string comparison or heuristic fallback.
// ===========================================================================

pub mod level;
pub mod range;
