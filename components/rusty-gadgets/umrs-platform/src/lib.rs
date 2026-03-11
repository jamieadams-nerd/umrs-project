// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//!
//! UMRS Platform — High-Assurance Kernel Attribute Access
//!
//! This crate provides platform-level infrastructure for interacting with
//! Linux kernel pseudo-filesystems (/sys/fs/selinux, /proc) in a
//! provenance-verified, fail-closed manner.
//!
//! NIST 800-53 SI-7: Software and Information Integrity.
//! NSA RTB RAIN: Non-Bypassable security checks.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]

pub mod kattrs;
