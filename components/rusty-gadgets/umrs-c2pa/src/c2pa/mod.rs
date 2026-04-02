// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA — Public Module Entry Point
//!
//! Re-exports the primary API surface of the UMRS C2PA library.
//! All public types, functions, and constants are re-exported here so
//! integration tests and binary callers import from a single, stable path.
//!
//! ## Key Re-exports
//!
//! - [`UmrsConfig`] — top-level configuration
//! - [`InspectError`] — unified error type
//! - [`ingest_file`] / [`sha256_hex`] — file ingestion
//! - [`read_chain`] / [`manifest_json`] / [`chain_json`] — manifest reading
//!   (all accept `&UmrsConfig` and perform trust validation from configured anchors)
//! - [`build_c2pa_settings`] — assemble c2pa SDK `Settings` from trust config
//! - [`ALLOWED_ALGORITHMS`] — FIPS-safe algorithm allow-list
//! - [`validate_config`] — configuration preflight checks
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-10**: Non-repudiation — C2PA manifests are
//!   cryptographically signed provenance records.
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — algorithm selection
//!   is gated behind the FIPS-safe allow-list in [`signer::ALLOWED_ALGORITHMS`];
//!   trust validation against configured CA anchors is performed on every read.
//! - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
//!   trust list validation detects manifests signed by unknown or revoked CAs.
//! - **NSA RTB RAIN**: Non-bypassability — all signing paths flow through
//!   `signer::parse_algorithm`, and all reading paths flow through
//!   `trust::build_c2pa_settings`, which cannot be bypassed.

// verbose must be declared first so the `verbose!` macro is available
// to all sibling modules via `#[macro_export]`.
pub mod verbose;

pub mod config;
pub mod creds;
pub mod error;
pub mod ingest;
pub mod manifest;
pub mod report;
pub mod signer;
pub mod trust;
pub mod validate;

pub use config::UmrsConfig;
pub use error::InspectError;
pub use ingest::{ingest_file, sha256_hex};
pub use manifest::{chain_json, has_manifest, manifest_json, read_chain};
pub use report::{print_chain, print_chain_readonly, print_validation_report};
pub use signer::ALLOWED_ALGORITHMS;
pub use trust::build_c2pa_settings;
pub use validate::validate_config;
pub use verbose::enable as enable_verbose;
