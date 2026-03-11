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
// NIST 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — this is a
// compile-time proof, not a policy. An auditor can verify it mechanically.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
//
#![deny(clippy::unwrap_used)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]
// ===========================================================================
pub mod category;
pub mod sensitivity;
pub mod status;

pub mod mcs;
pub mod mls;

pub mod context;

pub mod role;
pub mod type_id;
pub mod user;

pub mod fs_encrypt;
pub mod posix;
pub mod secure_dirent;
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
    SelinuxPolicy, SelinuxStatus, is_selinux_enabled, is_selinux_mls_enabled,
    selinux_policy,
};
