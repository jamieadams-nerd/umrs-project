// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//
//!
//! # UMRS SELinux Modeling Library
//!
//! - Project: Unclassified MLS Reference System (exploring CUI)
//!
//! This work represents a clean-room, strongly-typed modeling layer designed
//! to provide ergonomic and high-assurance abstractions over SELinux security
//! constructs.
//!
//! Where appropriate, this implementation leverages Rust language features
//! such as:
//!
//! - Strong typing
//! - Validation at construction
//! - Deterministic memory layout
//! - Ownership and borrowing guarantees
//!
//! These design choices aim to improve correctness, safety, and auditability
//! beyond legacy approaches while preserving semantic compatibility.
//!
//!
//! ## Module Map
//!
//! | Module | Contents |
//! |---|---|
//! | `context` | `SecurityContext` — full SELinux label with TPI parse architecture |
//! | `category` | `Category`, `CategorySet` — 1024-bit MLS category bitmask |
//! | `sensitivity` | `SensitivityLevel` — s0–s15 hierarchical levels |
//! | `mls` | `MlsLevel`, `MlsRange` — composite MLS label and clearance range |
//! | `mcs` | MCS translation engine, color coding |
//! | `status` | `SelinuxStatus`, `SelinuxPolicy` — live kernel state queries |
//! | `secure_dirent` | `SecureDirent` — TOCTOU-safe, security-enriched directory entry |
//! | `xattrs` | `SecureXattrReader` — fd-anchored xattr access with TPI gate |
//! | `posix` | `Uid`, `Gid`, `Inode`, `FileMode`, `LinuxUser`, `LinuxGroup` |
//! | `observations` | `SecurityObservation`, `ObservationKind` — typed security findings |
//! | `utils` | Directory listing helpers and file context accessors |
//! | `validate` | `SelinuxPattern`, `is_valid` — regex-cached syntax validation for SELinux contexts and MLS ranges |
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access Enforcement — `SecurityContext` and
//!   `CategorySet` carry the label data driving access decisions.
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — MLS dominance
//!   math and category-set subsumption checks are implemented in `mls/` and
//!   `category/`.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — `SecureDirent` and
//!   `SecurityObservation` ensure security-relevant fields are present in
//!   every directory entry audit record.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — TPI parse
//!   architecture in `xattrs.rs` ensures label parse integrity.
//! - **NIST SP 800-218 SSDF PW.4**: Secure Coding — `#![forbid(unsafe_code)]`
//!   is a compile-time proof; no unsafe block can exist in this crate.
//! - **NSA RTB RAIN**: Non-Bypassability — all xattr reads route through
//!   `SecureXattrReader`; the TPI gate cannot be skipped.
//!
//! ## Implementation Lineage & Design Note
//! This crate provides an independent, original implementation of primitives
//! conceptually comparable to traditional SELinux userland libraries and MLS
//! policy constructs.
//!
//! Behavioral interfaces and operational semantics were studied to ensure
//! familiarity for long-time SELinux developers and administrators.
//! However:
//!
//! - No SELinux source code has been copied.
//! - No code has been translated from C to Rust.
//! - No line-by-line reimplementation has been performed.
//!
// ===========================================================================
//  Don't mess with the order unless YOU know what you're doing!
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — this is a
// compile-time proof, not a policy. An auditor can verify it mechanically.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
//
#![deny(clippy::unwrap_used)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::doc_markdown)]
// ===========================================================================
pub mod category;
pub mod sensitivity;
pub mod status;
pub mod validate;

pub mod mcs;
pub mod mls;

pub mod context;

pub mod role;
pub mod type_id;
pub mod user;

pub mod fs_encrypt;
pub mod posix;
pub mod secure_dirent;
pub mod secure_file;
pub mod utils;
pub mod xattrs;

// Private implementation module — types are re-exported at crate root below.
mod observations;

//
// Re-export for ergonomic API, so consumers can write:
//     use umrs_selinux::{SecurityContext, SelinuxUser};
//
// Instead of:
//     use umrs_selinux::context::SecurityContext;
//
pub use observations::{ObservationKind, SecurityObservation};

pub use secure_dirent::SelinuxCtxState;
pub use xattrs::{TpiError, XattrReadError};

pub use category::{Category, CategorySet};
pub use mls::level::MlsLevel;
pub use sensitivity::SensitivityLevel;

pub use context::SecurityContext;
pub use role::SelinuxRole;
pub use type_id::SelinuxType;
pub use user::SelinuxUser;

// ===========================================================================
// Expose some "legacy-looking" functions to appease the old-gurd (even me).
// ===========================================================================

pub use crate::status::{
    SelinuxPolicy, SelinuxStatus, is_selinux_enabled, is_selinux_mls_enabled, selinux_policy,
};
