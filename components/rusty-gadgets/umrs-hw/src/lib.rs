// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # umrs-hw — Hardware Timestamp Isolation Crate
//!
//! Provides a safe public API for reading high-resolution hardware timestamps
//! used to time each phase of the OS detection pipeline.
//!
//! ## Unsafe Isolation Boundary
//!
//! This is the ONLY crate in the `rusty-gadgets` workspace that does NOT carry
//! `#![forbid(unsafe_code)]`. The single `unsafe` block in this crate wraps
//! the RDTSCP inline assembly instruction on x86_64. RDTSCP has no equivalent
//! in `core::arch` (confirmed: `intrinsics-map.md`, Timing section) — raw
//! `asm!` is the only available path for a serializing cycle counter read.
//!
//! All callers of this crate receive a fully safe API. The unsafe boundary is
//! confined to [`hw_timestamp`] and is greppable with `rg 'unsafe' umrs-hw/`.
//!
//! ## Architecture
//!
//! - **x86_64**: RDTSCP via `asm!` — serializing, cycle-accurate, AU-8.
//! - **other architectures (aarch64, etc.)**: `CLOCK_MONOTONIC_RAW` via
//!   `rustix::time::clock_gettime` — nanosecond-precision monotonic fallback.
//!
//! Both paths return a `u64` nanosecond-scale value suitable for computing
//! phase duration spans via `end.saturating_sub(start)`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-8**: Time Stamps — high-resolution timestamps ensure
//!   audit record temporal ordering is preserved with sub-microsecond precision.
//! - **NIST SP 800-218 SSDF PW.4**: Unsafe isolation — the only unsafe block in
//!   the workspace is mechanically bounded to this crate.

// ─────────────────────────────────────────────────────────────────────────────
// NOTE: #![forbid(unsafe_code)] is intentionally ABSENT from this crate root.
// This is the workspace's designated unsafe isolation boundary. See Cargo.toml
// for the architectural decision record. All other workspace crates retain
// #![forbid(unsafe_code)].
// ─────────────────────────────────────────────────────────────────────────────

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]

pub mod hw_timestamp;

// Re-export the two public functions at the crate root for ergonomic use.
pub use hw_timestamp::{read_hw_timestamp, tsc_is_invariant};
