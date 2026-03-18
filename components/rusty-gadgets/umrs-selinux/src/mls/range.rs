// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ===========================================================================
//! # MLS Clearance Range
//!
//! An MLS clearance range is a pair of `MlsLevel` values — low and high — that
//! define the security span a subject is permitted to operate within.
//!
//! In SELinux MLS policy, a range is written as `low-high` (e.g.,
//! `s0-s3:c0.c1023`). A single-level context (e.g., `s0`) is represented as a
//! degenerate range where low == high.
//!
//! Range containment and dominance semantics are defined by the lattice
//! dominance relation: a range A contains range B iff A.low dominates B.low
//! and B.high dominates A.high.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access Enforcement — range containment checks
//!   are the mechanism by which clearance-based read access is granted or denied.
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — the range bounds
//!   the set of levels from which a subject may receive information flows.
//! - **NSA RTB**: Deterministic Execution — range evaluation produces a binary
//!   decision; no ambiguous or probabilistic outcomes.
