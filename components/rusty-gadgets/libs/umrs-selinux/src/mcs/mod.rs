// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//! # SELinux Multi-Category Security (MCS) Namespace
//!
//! MCS (Multi-Category Security) is a SELinux compartmentalization mechanism
//! that assigns non-hierarchical category labels to files and processes. Only
//! subjects whose category set subsumes the object's category set may access it.
//!
//! MCS is a simplified form of MLS: it uses a fixed sensitivity level (`s0`)
//! and relies entirely on category-set containment for access decisions, rather
//! than the full hierarchical Bell-LaPadula model.
//!
//! ## Sub-modules
//!
//! - `translator` — `McsSensitivityTranslator`: parses `setrans.conf` and
//!   converts MCS/MLS kernel ranges to human-readable regulatory markings
//!   (e.g., NARA CUI labels). Uses a deterministic, audit-focused parse engine.
//! - `colors` — `secolor.conf` parser and lazy color cache for rendering
//!   context-sensitive color coding in directory listings and TUI displays.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — category-set
//!   containment implements the non-hierarchical compartment access control check.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — the `translator` module
//!   maps kernel MCS ranges to human-readable markings for audit record labels.
//! - **NSA RTB**: Deterministic Execution — `translator` uses a single
//!   explicit imperative parser with no heuristic fallback paths.
//!
// ===========================================================================
pub mod colors;
pub mod translator;
