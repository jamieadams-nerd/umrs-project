// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — this is a
// compile-time proof, not a policy. An auditor can verify it mechanically.
#![forbid(unsafe_code)]
//
//! UMRS Core Library
//!
//! Foundational primitives and shared infrastructure for all UMRS tools.
//!
//! Guarantees:
//! - Stable, versioned APIs for cross-tool reuse
//! - Deterministic behavior across supported platforms
//! - Security-first design aligned with MLS and FIPS constraints
//!
//! Non-goals:
//! - End-user CLI interfaces
//! - Tool-specific business logic
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — the `audit` sub-module
//!   provides structured, machine-consumable audit event types for cross-tool
//!   use.
//! - **NIST SP 800-53 SI-11**: Error Handling — structured error returns
//!   throughout; no sensitive data surfaces in user-visible error strings.
//! - **NIST SP 800-218 SSDF PW.4**: Secure Coding — `#![forbid(unsafe_code)]`
//!   provides a compile-time proof of memory-safe execution across all crate
//!   consumers.
//! - **NSA RTB RAIN**: Non-Bypassable — shared validation and audit primitives
//!   are centralized here; each consuming crate cannot skip them by reimplementing
//!   local alternatives.
//

// Local modules
pub mod audit;
pub mod console;
pub mod human;
pub mod i18n;
pub mod prelude;
pub mod robots;
pub mod timed_result;
pub mod validate;

