// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! UMRS Audit Module
//!
//! Authoritative, durable, machine-consumable audit events.
//!
//! This module is intentionally separate from console output.
//! Console events are UX artifacts; audit events are records of truth.

pub mod emit;
pub mod events;
pub mod schema;
