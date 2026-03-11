// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//
// ============================================================================
//!
//! UMRS PLATFORM: High-Assurance Kernel Attribute Modeling
//!
//! This module provides a High-Assurance (HA) substrate for interacting with
//! kernel-resident security attributes on Red Hat Enterprise Linux 10.
//!
//! RATIONALE FOR SPECIALIZED READERS:
//! Standard userspace I/O primitives are insufficient for systems operating
//! under NSA "Raise the Bar" (RTB) mandates or NIST 800-53 High-Impact controls.
//! Traditional file access ignores the "provenance" of the data source and
//! relies on weak, string-based typing that is prone to logic flaws.
//!
//! DESIGN PRINCIPLES:
//!
//! 1. PROVENANCE VERIFICATION (NIST 800-53 SI-7):
//!    Before any byte is read, the SecureReader verifies the backing filesystem
//!    integrity using fd-anchored fstatfs. The file is opened first to anchor
//!    to the inode, then the filesystem magic is checked on the open fd —
//!    eliminating the TOCTOU race present in path-based statfs checks.
//!
//! 2. STRONG DATA MODELING (VERNACULAR FIDELITY):
//!    This library models kernel 'attributes' (kobjects) as unique Rust types.
//!    By encoding kernel-doc metadata (DESCRIPTION, ATTRIBUTE_NAME) directly
//!    into the type, we eliminate "Type Confusion" and ensure that the software
//!    vernacular is 1:1 with the Linux Kernel Source (selinuxfs.rst).
//!
//! 3. DETERMINISTIC PARSING (NSA RTB - MINIMIZED TCB):
//!    Parsing logic is strictly bounded and heap-free where possible. The use
//!    of "Redundant (TPI) Parsing" ensures that security decisions are not
//!    dependent on a single logic path. If two independent parsing strategies
//!    (Declarative vs. Imperative) disagree, the system Fails-Closed.
//!
//! 4. NON-BYPASSABILITY (NSA RTB - RAIN):
//!    By leveraging Rust's type system and private execution engines, we ensure
//!    that security checks are Always Invoked. The use of PhantomData and
//!    StaticSource traits routes every read through SecureReader::execute_read,
//!    preventing callers from skipping the filesystem magic check.
//!
//! ## Submodule Layout
//!
//! | Module | Contents |
//! |---|---|
//! | `traits` | `KernelFileSource`, `StaticSource`, `SecureReader`, `AttributeCard` |
//! | `types` | `EnforceState`, `DualBool` |
//! | `selinux` | selinuxfs attributes: `SelinuxEnforce`, `SelinuxMls`, `SelinuxPolicyVers`, generics |
//! | `procfs` | procfs attributes: `ProcFips`, `ModuleLoadLatch` |
//! | `security` | securityfs attributes: `KernelLockdown`, `LockdownMode`, `SECURITYFS_MAGIC` |
//! | `tpi` | `validate_type_redundant` — TPI context type extraction |
//!
// ============================================================================

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
    GenericDualBool, GenericKernelBool, SelinuxEnforce, SelinuxMls,
    SelinuxPolicyVers,
};

// procfs attributes
pub use procfs::{ModuleLoadLatch, ProcFips, ProcfsText};

// sysfs attributes
pub use sysfs::{SYSFS_MAGIC, SysfsText};

// securityfs attributes
pub use security::{KernelLockdown, LockdownMode, SECURITYFS_MAGIC};

// TPI validation
pub use tpi::validate_type_redundant;
