// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! Audit event emission.
//!
//! Responsible for writing audit events to their authoritative sinks
//! (e.g., journald, files, sockets).
//!
//! No formatting or console output belongs here.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-9**: Protection of Audit Information — emission is
//!   isolated from console and presentation paths; audit sinks are the sole
//!   destination for authoritative event records.
//! - **NIST SP 800-53 AU-12**: Audit Record Generation — this module is the
//!   single authoritative write path for audit events; all callers route through
//!   here rather than writing directly to sinks.

// Placeholder.
