// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! UMRS Audit Module
//!
//! Authoritative, durable, machine-consumable audit events.
//!
//! This module is intentionally separate from console output.
//! Console events are UX artifacts; audit events are records of truth.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — this module defines the
//!   canonical event types; each event must carry sufficient context for an
//!   auditor to determine what occurred, by whom, on what resource, and with
//!   what outcome.
//! - **NIST SP 800-53 AU-9**: Protection of Audit Information — audit output is
//!   written only through the `emit` sub-module to authoritative sinks; console
//!   output is intentionally excluded from this path.
//! - **NIST SP 800-53 AU-12**: Audit Record Generation — structured event
//!   definitions in `events` and schema constraints in `schema` ensure every
//!   generated record conforms to the required format before emission.

pub mod emit;
pub mod events;
pub mod schema;
