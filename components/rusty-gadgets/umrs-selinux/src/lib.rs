//!
//! # UMRS SELinux Modeling Library
//!
//! - Project: Unclassified MLS Reference System (exploring CUI)
//! - Author: Jamie Adams (a.k.a, Imodium Operator)
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
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]

pub mod status;
pub mod category;
pub mod sensitivity;

pub mod mls;
pub mod mcs;

pub mod context;

pub mod user;
pub mod role;
pub mod type_id;

pub mod xattrs;

//
// Re-export for ergonomic API, so consumers can write:
//     use umrs_selinux::{SecurityContext, SelinuxUser};
//
// Instead of:
//     use umrs_selinux::context::SecurityContext;
//
pub use status::SelinuxStatus;

pub use category::{Category, CategorySet};
pub use sensitivity::SensitivityLevel;
pub use mls::level::MlsLevel;

pub use context::SecurityContext;
pub use user::SelinuxUser;
pub use role::SelinuxRole;
pub use type_id::SelinuxType;
