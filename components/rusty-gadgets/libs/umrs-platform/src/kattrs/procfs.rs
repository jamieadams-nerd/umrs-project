// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! # Procfs Kernel Attribute Types — `/proc/` Nodes
//!
//! Provides `ProcFips`, `ModuleLoadLatch`, and `ProcfsText` — the three procfs
//! attribute types used throughout this crate.
//!
//! - `ProcFips` — FIPS mode status at `/proc/sys/crypto/fips_enabled`
//! - `ModuleLoadLatch` — module loading lock at `/proc/sys/kernel/modules_disabled`
//! - `ProcfsText` — generic single-line text reader for any procfs node; the most
//!   broadly used type in this module (relied on by `SealedCache`, `PostureSnapshot`,
//!   and `SecureReader`-based callers throughout the platform)
//!
//! All types read from procfs and are verified against `PROC_SUPER_MAGIC` before
//! any bytes are parsed.
//!
//! NIST SP 800-53 CM-7, SC-28, SI-7: Least functionality, protection of
//! information at rest, and software integrity.
//! NSA RTB: Non-bypassable reads via `SecureReader`.

use nix::sys::statfs::{FsType, PROC_SUPER_MAGIC};
use std::io;
use std::path::{Path, PathBuf};

use super::traits::{KernelFileSource, SecureReader, StaticSource};

// ===========================================================================
// ProcFips
// ===========================================================================

/// FIPS mode attribute node (`/proc/sys/crypto/fips_enabled`).
///
/// NIST SP 800-53 SC-28 / SI-7: Protection of Information at Rest / Software
/// Integrity — confirms that the kernel is operating with FIPS 140-2/140-3
/// validated cryptographic primitives. Verified via `PROC_SUPER_MAGIC`.
pub struct ProcFips;
impl KernelFileSource for ProcFips {
    type Output = bool;
    const KOBJECT: &'static str = "crypto";
    const ATTRIBUTE_NAME: &'static str = "fips_enabled";
    const DESCRIPTION: &'static str = "0 -- FIPS disabled\n1 -- FIPS enabled";
    const KERNEL_NOTE: &'static str =
        "Verified via /proc/sys/crypto/fips_enabled (PROC_SUPER_MAGIC).";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid FIPS bit",
            )),
        }
    }
}
impl StaticSource for ProcFips {
    const PATH: &'static str = "/proc/sys/crypto/fips_enabled";
    const EXPECTED_MAGIC: FsType = PROC_SUPER_MAGIC;
}

// ===========================================================================
// ModuleLoadLatch
// ===========================================================================

/// Kernel module loading disabled latch (`/proc/sys/kernel/modules_disabled`).
///
/// Reads the one-way latch that controls whether kernel module loading is
/// permitted. Once set to `1` (disabled), the latch cannot be cleared without
/// a reboot — making `true` a permanent enforcement state for the lifetime of
/// the running kernel.
///
/// This attribute requires no TPI parsing: a single byte (`0` or `1`) has no
/// structural ambiguity. Provenance verification via `PROC_SUPER_MAGIC`
/// (fd-anchored `fstatfs` before any read) is the security layer that ensures
/// the value originates from the kernel and not a bind-mounted substitute.
///
/// NIST SP 800-53 CM-7: Least Functionality — a locked latch enforces that no
/// new modules can extend the kernel attack surface at runtime.
/// NIST SP 800-53 SI-7: Software and Information Integrity — the latch value is
/// read from a provenance-verified kernel pseudo-filesystem.
/// NSA RTB: minimized attack surface; no write path is provided here.
pub struct ModuleLoadLatch;
impl KernelFileSource for ModuleLoadLatch {
    type Output = bool;
    const KOBJECT: &'static str = "proc/sys/kernel";
    const ATTRIBUTE_NAME: &'static str = "modules_disabled";
    const DESCRIPTION: &'static str = "0 -- module loading enabled\n1 -- module loading permanently disabled (latch)";
    const KERNEL_NOTE: &'static str = "One-way latch: once set to 1, cannot be cleared without a reboot. \
         Verified via PROC_SUPER_MAGIC.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid modules_disabled value",
            )),
        }
    }
}
impl StaticSource for ModuleLoadLatch {
    const PATH: &'static str = "/proc/sys/kernel/modules_disabled";
    const EXPECTED_MAGIC: FsType = PROC_SUPER_MAGIC;
}

// ===========================================================================
// ProcfsText — dynamic-path procfs text reader
// ===========================================================================

/// Dynamic procfs text reader for large-content files (e.g., `/proc/mounts`).
///
/// Unlike `StaticSource` types (which use a 64-byte stack buffer sufficient
/// for single-value kernel attributes), `ProcfsText` uses `read_to_string`
/// for files whose content length is unbounded at compile time.
///
/// Path prefix is validated at construction (`/proc/` required). The
/// expected filesystem magic (`PROC_SUPER_MAGIC`) is bound at construction
/// and cannot be overridden by the caller. All reads flow through
/// `SecureReader::read_generic_text` — the single trusted engine.
///
/// NIST SP 800-53 SI-7: provenance-verified read; magic check before any bytes
/// are consumed. NSA RTB RAIN: Non-Bypassable — callers cannot skip the
/// fstatfs gate.
pub struct ProcfsText {
    pub(super) path: PathBuf,
}

impl ProcfsText {
    /// Construct a procfs text reader, validating the path prefix.
    ///
    /// # Errors
    ///
    /// Returns `io::ErrorKind::InvalidInput` if `path` does not start with
    /// `/proc/`.
    ///
    /// NIST SP 800-53 SI-10: Input Validation — rejects non-procfs paths
    /// before any I/O is attempted.
    pub fn new(path: PathBuf) -> io::Result<Self> {
        if !path.starts_with("/proc/") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "ProcfsText path must be under /proc/",
            ));
        }
        Ok(Self {
            path,
        })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl SecureReader<ProcfsText> {
    /// Provenance-verified read of a procfs text file.
    ///
    /// Opens the file, verifies `PROC_SUPER_MAGIC` via fd-anchored `fstatfs`,
    /// then reads the full content as a UTF-8 string. Fails closed on any
    /// I/O error or magic mismatch.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the file cannot be opened, if the filesystem
    /// magic does not match `PROC_SUPER_MAGIC` (integrity failure), or if the
    /// file content is not valid UTF-8.
    ///
    /// NIST SP 800-53 SI-7 / NSA RTB RAIN.
    pub fn read_generic_text(&self, node: &ProcfsText) -> io::Result<String> {
        Self::execute_read_text(&node.path, PROC_SUPER_MAGIC)
    }
}
