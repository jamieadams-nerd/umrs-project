// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//! # CUI Label Support
//!
//! Provides Controlled Unclassified Information (CUI) label definitions,
//! catalog loading, and color palette support for UMRS tool displays.
//!
//! ## Sub-modules
//!
//! - `catalog` — `Catalog`, `Marking`: deserializes the CUI label
//!   catalog from a JSON file; maps MCS security ranges to regulatory markings.
//! - `palette` — color palette definitions for CUI marking display.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — CUI markings are security
//!   attributes that must be displayed accurately and consistently.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — CUI labels appear in
//!   audit-visible directory listings and security reports.

pub mod catalog;
pub mod palette;
