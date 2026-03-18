// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//! # SELinux Bridge — Core Utilities
//!
//! Provides bridge utilities between the MCS Translation service and the CUI
//! catalog in `umrs_core::cui`.
//!
//! ## Sub-modules
//!
//! - `mcs` — `McsBridge`: maps MCS security ranges from `setrans.conf` to
//!   CUI catalog entries, enabling human-readable marking lookups.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — the MCS-to-CUI
//!   bridge is used to display the regulatory marking for a file's security
//!   label in audit-visible tool output.
//! - **NIST SP 800-53 AC-16**: Security Attributes — this module exposes the
//!   canonical mapping between kernel MCS labels and CUI markings.

pub mod mcs;
