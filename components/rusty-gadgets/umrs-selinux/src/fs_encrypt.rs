// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
//! # Filesystem Encryption Detection (`fs_encrypt`)
//!
//! Detects whether a mounted filesystem is protected by encryption, either
//! at the block-device layer (LUKS/dm-crypt) or at the filesystem layer
//! (ecryptfs, gocryptfs, etc.).
//!
//! All reads from `/proc` use `ProcfsText` + `SecureReader`, which performs
//! fd-anchored `fstatfs` provenance verification against `PROC_SUPER_MAGIC`
//! before any bytes are consumed. All reads from `/sys` use `SysfsText` +
//! `SecureReader`, verified against `SYSFS_MAGIC`. Both types validate their
//! path prefix at construction — non-procfs or non-sysfs paths are rejected
//! before any I/O is attempted.
//!
//! This ensures all reads flow through the single trusted engine established
//! in `umrs-platform::kattrs` (NSA RTB RAIN — Non-Bypassable).
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SC-28**: Protection of Information at Rest — this
//!   module is the detection layer for at-rest encryption posture.
//! - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
//!   provenance verification before trusting kernel pseudo-filesystem data.
//! - **NIST SP 800-53 SI-10**: Input Validation — path prefix validated at
//!   construction; non-procfs/non-sysfs paths rejected before I/O.
//! - **NSA RTB RAIN**: All reads route through `SecureReader` — no bypass
//!   path exists.

use std::path::{Path, PathBuf};

use umrs_platform::kattrs::{ProcfsText, SecureReader, SysfsText};

/// Encrypted filesystem types recognized at the VFS mount layer.
///
/// These are the `fstype` strings as they appear in `/proc/mounts`.
///
/// NIST SP 800-53 SC-28: filesystem-layer encryption detection.
const ENCRYPTED_FS_TYPES: &[&str] = &[
    "ecryptfs",
    "fuse.encfs",
    "fuse.gocryptfs",
    "fuse.securefs",
    "fuse.cryfs",
];

/// The source of encryption protecting a mounted filesystem.
///
/// NIST SP 800-53 SC-28: Protection of Information at Rest.
/// NSA RTB: typed result prevents silent conflation of encryption layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptionSource {
    /// No encryption detected at any layer.
    None,

    /// Block device is LUKS-encrypted (kernel device-mapper "crypt" type).
    ///
    /// Detected via `/sys/class/block/{dev}/dm/type` == `"crypt"` or
    /// `/sys/class/block/{dev}/dm/uuid` prefix `"CRYPT-LUKS"`.
    LuksDevice,

    /// Filesystem-level encryption. The inner string is the `fstype`
    /// from `/proc/mounts` (e.g., `"ecryptfs"`, `"fuse.gocryptfs"`).
    EncryptedFilesystem(String),
}

/// Detect the encryption source protecting `mount_point`.
///
/// Parses `/proc/mounts` once (provenance-verified) to extract both the
/// filesystem type and the underlying device in a single pass. Checks
/// filesystem-layer encryption first; falls back to block-layer LUKS
/// detection. Returns [`EncryptionSource::None`] on any read error —
/// fail-closed.
///
/// NIST SP 800-53 SC-28 / SI-7.
/// NSA RTB: Provenance Verification, Fail-Closed, Non-Bypassable.
#[must_use]
pub fn detect_mount_encryption(mount_point: &Path) -> EncryptionSource {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = detect_inner(mount_point);

    #[cfg(debug_assertions)]
    log::debug!(
        "Provenance-verified encryption detection completed in {} µs",
        t0.elapsed().as_micros()
    );

    result
}

fn detect_inner(mount_point: &Path) -> EncryptionSource {
    // Single-pass parse of /proc/mounts — extracts fstype and device together.
    // This eliminates the double-read inconsistency window that would exist if
    // fstype and device were fetched in separate calls.
    let Some(entry) = find_mount_entry(mount_point) else {
        return EncryptionSource::None;
    };

    if ENCRYPTED_FS_TYPES.contains(&entry.fs_type.as_str()) {
        return EncryptionSource::EncryptedFilesystem(entry.fs_type);
    }

    if check_luks_encrypted(&entry.device) {
        return EncryptionSource::LuksDevice;
    }

    EncryptionSource::None
}

// ===========================================================================
// /proc/mounts — single-pass extraction
// ===========================================================================

/// Fields extracted from a single `/proc/mounts` line.
struct MountEntry {
    device: String,
    fs_type: String,
}

/// Parse `/proc/mounts` (provenance-verified via `ProcfsText` + `SecureReader`)
/// and return the device and fstype for `mount_point` in a single pass.
///
/// Both fields are extracted from the same line in one read of the file,
/// eliminating the consistency gap that two separate reads would introduce.
///
/// Returns `None` if the mount point is not found or the read fails.
fn find_mount_entry(mount_point: &Path) -> Option<MountEntry> {
    let node = ProcfsText::new(PathBuf::from("/proc/mounts")).ok()?;
    let contents = SecureReader::<ProcfsText>::new()
        .read_generic_text(&node)
        .ok()?;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let Some(device) = parts.next() else { continue; };
        let Some(mp) = parts.next() else { continue; };
        let Some(fs_type) = parts.next() else { continue; };
        if Path::new(mp) == mount_point {
            return Some(MountEntry {
                device: device.to_owned(),
                fs_type: fs_type.to_owned(),
            });
        }
    }
    None
}

// ===========================================================================
// LUKS detection
// ===========================================================================

/// Determine whether `dev_path` is a LUKS-encrypted block device.
///
/// Steps:
/// 1. Canonicalize `dev_path` to resolve `/dev/mapper/X` → `/dev/dm-N`.
///    Accepted residual risk: `canonicalize` is path-based (no fd variant
///    exists in the Linux API); the device string was obtained from a
///    provenance-verified `/proc/mounts` read.
/// 2. Extract the kernel device name (final path component; cannot contain
///    `/` by `file_name()` contract — path traversal is structurally impossible).
/// 3. Construct `/sys/class/block/{dev}/dm/type` and read via `SysfsText`
///    (provenance-verified against `SYSFS_MAGIC`); check for `"crypt"`.
/// 4. Fallback: read `/sys/class/block/{dev}/dm/uuid` and check for the
///    `"CRYPT-LUKS"` prefix.
///
/// Returns `false` on any read or resolve error — fail-closed.
///
/// NIST SP 800-53 SC-28 / SI-7.
#[allow(clippy::collapsible_if)]
fn check_luks_encrypted(dev_path: &str) -> bool {
    // Canonicalize /dev/mapper/X -> /dev/dm-N (or similar).
    let Ok(real_path) = std::fs::canonicalize(dev_path) else {
        return false;
    };

    // file_name() returns only the terminal component — no `/` can appear,
    // so the kernel_name cannot escape /sys/class/block/ via traversal.
    let Some(kernel_name) = real_path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };

    // Primary check: dm/type == "crypt"
    // SysfsText::new validates the path starts with /sys/ before any I/O.
    let type_path = format!("/sys/class/block/{kernel_name}/dm/type");
    if let Ok(node) = SysfsText::new(PathBuf::from(&type_path)) {
        if let Ok(dm_type) =
            SecureReader::<SysfsText>::new().read_generic_text(&node)
        {
            if dm_type.trim() == "crypt" {
                return true;
            }
        }
    }

    // Fallback: dm/uuid prefix "CRYPT-LUKS"
    let uuid_path = format!("/sys/class/block/{kernel_name}/dm/uuid");
    if let Ok(node) = SysfsText::new(PathBuf::from(&uuid_path)) {
        if let Ok(uuid) =
            SecureReader::<SysfsText>::new().read_generic_text(&node)
        {
            return uuid.trim().starts_with("CRYPT-LUKS");
        }
    }

    false
}
