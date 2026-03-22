// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Kernel Security Posture Probe — Phase 1 + Phase 2a + Phase 2b.
//!
//! Reads, categorises, and reports on Linux kernel security hardening indicators.
//! The probe gives callers a typed, iterable view of the system's runtime
//! security posture — answering questions like "is ASLR fully enabled?" or
//! "are unprivileged user namespaces blocked?" — with the same provenance
//! guarantees established in the `kattrs` engine.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use umrs_platform::posture::{PostureSnapshot, AssuranceImpact, IndicatorId};
//!
//! // Collect all indicators
//! let snap = PostureSnapshot::collect();
//!
//! // Summary
//! println!("{}/{} indicators meet hardened baseline",
//!     snap.hardened_count(), snap.readable_count());
//!
//! // Findings — indicators whose live value does not meet the desired
//! // (hardened) value defined in the catalog. Each report carries the
//! // indicator's descriptor, its live kernel value, and whether it meets the
//! // desired baseline.
//! for report in snap.findings() {
//!     println!("{}: live={:?}", report.descriptor.id, report.live_value);
//! }
//!
//! // Critical-impact indicators only
//! for report in snap.by_impact(AssuranceImpact::Critical) {
//!     println!("{:?}: meets={:?}", report.descriptor.id, report.meets_desired);
//! }
//!
//! // Specific indicator lookup
//! if let Some(r) = snap.get(IndicatorId::KptrRestrict) {
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
//! | `indicator` | Core types: `IndicatorId`, `IndicatorClass`, `AssuranceImpact`, `DesiredValue` |
//! | `catalog` | Static `INDICATORS` array — compile-time catalog with descriptions, recommendations |
//! | `display` | Pure formatting functions — translate raw `LiveValue` into operator-readable strings |
//! | `reader` | Live-value readers routing through `SecureReader` |
//! | `configured` | sysctl.d merge-tree configured-value reading |
//! | `bootcmdline` | BLS entry reader for configured kernel cmdline (Phase 2b) |
//! | `contradiction` | `ContradictionKind` and classification logic |
//! | `snapshot` | `PostureSnapshot` and `IndicatorReport` — the public API |
//!
//! ## Compliance
//!
//! NIST SP 800-53 CA-7: Continuous Monitoring — the posture probe is the
//! mechanism for continuous kernel security baseline assessment.
//! NIST SP 800-53 CM-6: Configuration Settings — live vs. configured
//! contradiction detection identifies configuration management gaps.
//! NIST SP 800-53 AU-3: Audit Record Content — typed indicator reports enable
//! machine-readable audit trail generation.
//! NSA RTB RAIN: Non-Bypassable — all kernel reads route through the
//! provenance-verified `SecureReader` engine.
//! NSA RTB: Compile-Time Path Binding — indicator paths and desired values
//! are `const`, compiler-verified, and catalog-bound.

pub mod bootcmdline;
pub mod catalog;
pub mod configured;
pub mod contradiction;
pub mod display;
pub mod fips_cross;
pub mod indicator;
pub mod modprobe;
pub mod reader;
pub mod snapshot;

// ---------------------------------------------------------------------------
// Public API re-exports
// ---------------------------------------------------------------------------

pub use catalog::{
    CATALOG_KERNEL_BASELINE, INDICATORS, IndicatorDescriptor, lookup,
};
pub use contradiction::ContradictionKind;
pub use display::{
    annotate_integer, annotate_live_value, annotate_signed_integer,
};
pub use fips_cross::FipsCrossCheck;
pub use indicator::{
    AssuranceImpact, ConfiguredValue, DesiredValue, IndicatorClass,
    IndicatorId, LiveValue,
};
pub use modprobe::{ModprobeConfig, ParsedDirective, parse_modprobe_line};
pub use snapshot::{IndicatorReport, PostureSnapshot};
