// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA — Media Provenance and Chain-of-Custody Library
//!
//! Provides C2PA manifest inspection, signing, and chain-of-custody reporting
//! for UMRS-ingested media assets. Supports FIPS-safe signing algorithms and
//! embeds UMRS security markings as tamper-evident manifest assertions.
//!
//! ## Key Exported Types
//!
//! - [`c2pa::UmrsConfig`] — top-level configuration loaded from `umrs-c2pa.toml`
//! - [`c2pa::InspectError`] — all errors produced by this library
//! - [`c2pa::manifest::ChainEntry`] — a single entry in the chain of custody
//! - [`c2pa::manifest::TrustStatus`] — trust evaluation for a chain entry
//! - [`c2pa::ingest::IngestResult`] — result of ingesting a file into UMRS
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-10**: Non-repudiation — C2PA manifests provide
//!   cryptographically signed, tamper-evident provenance records.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — chain entries carry
//!   signer identity, timestamp, algorithm, and security marking.
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — only FIPS-safe
//!   signing algorithms (ES256/384/512, PS256/384/512) are permitted.
//! - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
//!   C2PA asset hashing and manifest signing detect tampering.
//! - **CMMC SC.L2-3.13.10**: Employ FIPS-validated cryptography — algorithm
//!   allow-list enforced at configuration parse and signer construction time.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
// Approved project suppressions (see rust_design_rules.md):
#![allow(clippy::option_if_let_else)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::doc_markdown)]

/// UMRS C2PA library — re-exported for integration tests and downstream crates.
pub mod c2pa;
