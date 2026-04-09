// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — this is a
// compile-time proof, not a policy. An auditor can verify it mechanically.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]
#![deny(clippy::unwrap_used)]
//
//! # UMRS Labels Library
//!
//! Universal label abstraction crate for CUI (Controlled Unclassified
//! Information) and Canadian Protected markings.
//!
//! This crate provides:
//! - `cui` — CUI catalog loading, label and marking types, and palette
//! - `validate` — `CuiMarking` pattern validation
//!
//! The CUI module originated in `umrs-core` and has been promoted here as a
//! first-class library crate, framework-agnostic and reusable across all UMRS
//! tools. The binary entry point (`umrs-labels`) demonstrates catalog
//! inspection.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — CUI markings are security
//!   attributes that must be displayed accurately and consistently.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — CUI labels appear in
//!   audit-visible directory listings and security reports.
//! - **CMMC AC.L2-3.1.3**: Control CUI flow in accordance with approved
//!   authorizations.

pub mod cui;
pub mod tui;
pub mod validate;

pub use tui::app::marking_to_detail;
