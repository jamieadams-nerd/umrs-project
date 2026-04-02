// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # UMRS Platform kattrs — High-Assurance Kernel Attribute Access
//!
//! Provides a high-assurance substrate for reading kernel-resident security
//! attributes on Red Hat Enterprise Linux 10. Every read is provenance-verified
//! against the expected filesystem magic before any bytes are parsed.
//!
//! ## Rationale for Specialized Readers
//!
//! Standard userspace I/O primitives are insufficient for systems operating
//! under NSA "Raise the Bar" (RTB) mandates or NIST SP 800-53 High-Impact
//! controls. Traditional file access ignores the provenance of the data source
//! and relies on weak, string-based typing that is prone to logic flaws.
//!
//! ## Design Principles
//!
//! ### 1. Provenance Verification (NIST SP 800-53 SI-7)
//!
//! Before any byte is read, `SecureReader` verifies the backing filesystem
//! integrity using fd-anchored `fstatfs`. The file is opened first to anchor
//! to the inode, then the filesystem magic is checked on the open fd —
//! eliminating the TOCTOU race present in path-based `statfs` checks.
//!
//! ### 2. Strong Data Modeling (Vernacular Fidelity)
//!
//! This module models kernel attributes (kobjects) as unique Rust types.
//! By encoding kernel-doc metadata (`DESCRIPTION`, `ATTRIBUTE_NAME`) directly
//! into the type, type confusion is eliminated and the software vernacular is
//! 1:1 with the Linux Kernel Source (`selinuxfs.rst`).
//!
//! ### 3. Deterministic Parsing (NSA RTB — Minimized TCB)
//!
//! Parsing logic is strictly bounded and heap-free where possible. Two-Path
//! Independent (TPI) parsing ensures that security decisions are not dependent
//! on a single logic path. If two independent parsing strategies disagree,
//! the system fails closed.
//!
//! ### 4. Non-Bypassability (NSA RTB — RAIN)
//!
//! Rust's type system and private execution engines ensure that security checks
//! are always invoked. `PhantomData` and the `StaticSource` trait route every
//! read through `SecureReader::execute_read`, preventing callers from skipping
//! the filesystem magic check.
//!
//! ## Submodule Layout
//!
//! | Module | Contents |
//! |---|---|
//! | `traits` | `KernelFileSource`, `StaticSource`, `SecureReader`, `AttributeCard` |
//! | `types` | `EnforceState`, `DualBool` |
//! | `selinux` | selinuxfs attributes: `SelinuxEnforce`, `SelinuxMls`, `SelinuxPolicyVers`, generics |
//! | `procfs` | procfs attributes: `ProcFips`, `ModuleLoadLatch`, `ProcfsText` |
//! | `sysfs` | sysfs attributes: `SysfsText`, `SYSFS_MAGIC` |
//! | `security` | securityfs attributes: `KernelLockdown`, `LockdownMode`, `SECURITYFS_MAGIC` |
//! | `tpi` | `validate_type_redundant` — TPI context type extraction |
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — provenance
//!   verification via fd-anchored `fstatfs` before every read.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — kobject metadata is
//!   encoded in each type for complete audit records.
//! - **NSA RTB RAIN**: Non-Bypassability — all reads route through
//!   `SecureReader`; the magic check cannot be skipped.
//! - **NSA RTB VNSSA**: Vernacular fidelity — type names match kernel
//!   kobject names exactly.
//! - **NIST SP 800-218 SSDF PW.4**: Compile-time path and magic binding
//!   prevents runtime parameterization of security-critical constants.

pub mod procfs;
pub mod security;
pub mod selinux;
pub mod sysfs;
pub mod tpi;
pub mod traits;
pub mod types;

// ---------------------------------------------------------------------------
// Re-exports — public API surface (identical to the former kattrs.rs surface)
// ---------------------------------------------------------------------------

// Core engine
pub use traits::{AttributeCard, KernelFileSource, SecureReader, StaticSource};

// Domain value types
pub use types::{DualBool, EnforceState};

// selinuxfs attributes
pub use selinux::{
    GenericDualBool, GenericKernelBool, SelinuxEnforce, SelinuxMls, SelinuxPolicyVers,
};

// procfs attributes
pub use procfs::{ModuleLoadLatch, ProcFips, ProcfsText};

// sysfs attributes
pub use sysfs::{SYSFS_MAGIC, SysfsText};

// securityfs attributes
pub use security::{KernelLockdown, LockdownMode, SECURITYFS_MAGIC};

// TPI validation
pub use tpi::validate_type_redundant;
