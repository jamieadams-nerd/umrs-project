// SPDX-License-Identifier: MIT                                                                     
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//!
//! # SELinux Multi-Level Security (MLS) Namespace 
//!
//! SELinux MLS is a strict mandatory access control mechanism that restricts 
//! data access based on confidentiality levels (e.g., Confidential, Secret) 
//! and categories, enforcing a "no read-up, no write-down" policy. It 
//! enhances data protection by applying hierarchical security levels to both 
//! subjects (processes/users) and objects (files/devices).
//!
//! This namespace defines:
//! - MLS Level structures
//! - Clearance ranges
//! - Dominance mathematics
//! - Range containment logic
//! 
//! Primitive label components such as `SensitivityLevel` and `CategorySet` are
//!  defined at the crate root and consumed by this module.
//!
//! ## Module Architecture 
//! This module contains composite structures modeling `SELinux`  Multi-Level 
//! Security (MLS) labels and clearance ranges.
//!
//! MLS extends the SELinux security context by introducing hierarchical classification and
//! compartmentalization semantics.
//!
//! ## Security Label Composition
//! An MLS level is composed of:
//!
//!  `SensitivityLevel` : `CategorySet`
//!
//! Example: s3:c0,c2,c9
//!
//! Where:
//! - `SensitivityLevel` defines hierarchical classification.
//! - `CategorySet` defines compartment membership.
//!
//! ## Conceptual lineage was studied from:
//!
//! - SELinux  kernel MLS subsystem
//! - policydb sensitivity handling
//! - mls.c dominance logic
//!
//! No kernel or `SELinux`  userland source code has been copied or translated.
//!
//! All implementations are original Rust constructions aligned at the
//! semantic level only.
// ===========================================================================

pub mod level;
pub mod range;

