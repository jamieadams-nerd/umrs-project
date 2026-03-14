// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Kernel Security Posture Probe — Phase 1 + Phase 2a.
//!
//! Reads, categorises, and reports on Linux kernel security hardening signals.
//! The probe gives callers a typed, iterable view of the system's runtime
//! security posture — answering questions like "is ASLR fully enabled?" or
//! "are unprivileged user namespaces blocked?" — with the same provenance
//! guarantees established in the `kattrs` engine.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use umrs_platform::posture::{PostureSnapshot, AssuranceImpact, SignalId};
//!
//! // Collect all signals
//! let snap = PostureSnapshot::collect();
//!
//! // Summary
//! println!("{}/{} signals meet hardened baseline",
//!     snap.hardened_count(), snap.readable_count());
//!
//! // Findings (signals not meeting desired value)
//! for report in snap.findings() {
//!     println!("{}: live={:?}", report.descriptor.id, report.live_value);
//! }
//!
//! // Critical-impact signals only
//! for report in snap.by_impact(AssuranceImpact::Critical) {
//!     println!("{:?}: meets={:?}", report.descriptor.id, report.meets_desired);
//! }
//!
//! // Specific signal lookup
//! if let Some(r) = snap.get(SignalId::KptrRestrict) {
//!     println!("kptr_restrict hardened: {:?}", r.meets_desired);
//! }
//! ```
//!
//! ## Architecture
//!
//! The posture module is self-contained and depends only on `kattrs` for the
//! `SecureReader` engine. It does not depend on the `detect` module.
//!
//! | Module | Role |
//! |---|---|
//! | `signal` | Core types: `SignalId`, `SignalClass`, `AssuranceImpact`, `DesiredValue` |
//! | `catalog` | Static `SIGNALS` array — compile-time catalog of all signals |
//! | `reader` | Live-value readers routing through `SecureReader` |
//! | `configured` | sysctl.d merge-tree configured-value reading |
//! | `contradiction` | `ContradictionKind` and classification logic |
//! | `snapshot` | `PostureSnapshot` and `SignalReport` — the public API |
//!
//! ## Compliance
//!
//! NIST SP 800-53 CA-7: Continuous Monitoring — the posture probe is the
//! mechanism for continuous kernel security baseline assessment.
//! NIST SP 800-53 CM-6: Configuration Settings — live vs. configured
//! contradiction detection identifies configuration management gaps.
//! NIST SP 800-53 AU-3: Audit Record Content — typed signal reports enable
//! machine-readable audit trail generation.
//! NSA RTB RAIN: Non-Bypassable — all kernel reads route through the
//! provenance-verified `SecureReader` engine.
//! NSA RTB: Compile-Time Path Binding — signal paths and desired values
//! are `const`, compiler-verified, and catalog-bound.

pub mod catalog;
pub mod configured;
pub mod contradiction;
pub mod fips_cross;
pub mod modprobe;
pub mod reader;
pub mod signal;
pub mod snapshot;

// ---------------------------------------------------------------------------
// Public API re-exports
// ---------------------------------------------------------------------------

pub use catalog::{SIGNALS, SignalDescriptor};
pub use contradiction::ContradictionKind;
pub use fips_cross::FipsCrossCheck;
pub use modprobe::{ModprobeConfig, ParsedDirective, parse_modprobe_line};
pub use signal::{
    AssuranceImpact, ConfiguredValue, DesiredValue, LiveValue, SignalClass,
    SignalId,
};
pub use snapshot::{PostureSnapshot, SignalReport};
