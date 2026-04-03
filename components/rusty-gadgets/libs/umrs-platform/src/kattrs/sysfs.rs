// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Sysfs kernel attribute types (`/sys/`).
//!
//! All types in this module read from sysfs and are verified against
//! `SYSFS_MAGIC` before any bytes are parsed.
//!
//! `SysfsText` handles dynamic-path sysfs attributes whose paths are only
//! known at runtime (e.g., `/sys/class/block/{dev}/dm/type`). Path prefix
//! is validated at construction; the sysfs magic check is performed inside
//! `SecureReader::read_generic_text` before any bytes are consumed.
//!
//! NIST SP 800-53 SI-7: Software and Information Integrity — provenance
//! verification before trusting kernel pseudo-filesystem data.
//! NSA RTB RAIN: Non-bypassable — all reads flow through `SecureReader`.

use std::io;
use std::path::{Path, PathBuf};

use nix::sys::statfs::FsType;

use super::traits::SecureReader;

// ===========================================================================
// SYSFS_MAGIC
// ===========================================================================

/// Filesystem magic number for sysfs (`/sys/`).
///
/// Value `0x62656572` from `linux/magic.h`. Not exposed as a named constant
/// in nix 0.27 on this target, so defined locally.
///
/// NIST SP 800-53 SI-7: magic verification prevents bind-mount spoofing of
/// `/sys/` paths — an attacker with a bind-mount could substitute any
/// filesystem; only genuine sysfs carries this magic.
#[expect(clippy::unreadable_literal, reason = "sysfs magic constant — the raw hex value is the canonical form from the kernel source")]
pub const SYSFS_MAGIC: FsType = FsType(0x62656572_i64);

// ===========================================================================
// SysfsText — dynamic-path sysfs text reader
// ===========================================================================

/// Dynamic sysfs text reader for runtime-determined attribute paths.
///
/// Unlike `StaticSource` types (64-byte stack buffer, compile-time path),
/// `SysfsText` accepts a `PathBuf` at runtime and reads unbounded UTF-8
/// content. Intended for sysfs attribute nodes whose paths include
/// device names or other runtime values (e.g.,
/// `/sys/class/block/{dev}/dm/type`).
///
/// Path prefix is validated at construction (`/sys/` required). The
/// `SYSFS_MAGIC` constant is bound at the type level — callers cannot
/// substitute a different magic. All reads flow through
/// `SecureReader::read_generic_text` — the single trusted engine.
///
/// NIST SP 800-53 SI-7: provenance-verified read; magic check before any bytes
/// are consumed. NSA RTB RAIN: Non-Bypassable — callers cannot skip the
/// fstatfs gate.
pub struct SysfsText {
    pub(super) path: PathBuf,
}

impl SysfsText {
    /// Construct a sysfs text reader, validating the path prefix.
    ///
    /// # Errors
    ///
    /// Returns `io::ErrorKind::InvalidInput` if `path` does not start with
    /// `/sys/`.
    ///
    /// NIST SP 800-53 SI-10: Input Validation — rejects non-sysfs paths
    /// before any I/O is attempted.
    pub fn new(path: PathBuf) -> io::Result<Self> {
        if !path.starts_with("/sys/") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "SysfsText path must be under /sys/",
            ));
        }
        Ok(Self {
            path,
        })
    }

    #[must_use = "pure accessor — returns the sysfs attribute path validated at construction"]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl SecureReader<SysfsText> {
    /// Provenance-verified read of a sysfs text attribute.
    ///
    /// Opens the file, verifies `SYSFS_MAGIC` via fd-anchored `fstatfs`,
    /// then reads the full content as a UTF-8 string. Fails closed on any
    /// I/O error or magic mismatch.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the file cannot be opened, if the filesystem
    /// magic does not match `SYSFS_MAGIC` (integrity failure), or if the
    /// file content is not valid UTF-8.
    ///
    /// NIST SP 800-53 SI-7 / NSA RTB RAIN.
    pub fn read_generic_text(&self, node: &SysfsText) -> io::Result<String> {
        Self::execute_read_text(&node.path, SYSFS_MAGIC)
    }
}
