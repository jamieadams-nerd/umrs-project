// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # Filesystem Utilities
//!
//! Encrypted mount detection and LUKS verification helpers.
//!
//! ## Architectural Note — NOT WIRED IN
//!
//! This module is declared in the `fs/` directory but is NOT currently declared
//! in `lib.rs`. It must NOT be wired in until the following architectural
//! requirement is satisfied:
//!
//! All reads from `/proc/` and `/sys/` in this module use `std::fs::read_to_string`
//! directly. On RHEL 10 targets, these reads MUST be replaced with
//! `ProcfsText`/`SysfsText` + `SecureReader` from `umrs-platform` to satisfy
//! provenance verification. That requires adding `umrs-platform` as a dependency
//! to `umrs-core`. Do not wire this module in until that change is complete.
//!
//! Additionally, `is_luks_encrypted` reads `/proc/self/mounts` and
//! `/sys/class/block/` inline — both are architectural violations on RHEL 10
//! that must be addressed before enabling this code path.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SC-28**: Protection of information at rest — encrypted
//!   mount detection supports operator awareness of filesystem encryption posture.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — provenance
//!   verification of procfs/sysfs reads is required before this code is active.
//! - **NSA RTB RAIN**: Non-bypassability — provenance-checked reads must be used
//!   for all kernel attribute paths.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

// Known encrypted filesystem type identifiers as reported in /proc/mounts.
// Covers common in-kernel and FUSE-based encrypted FS types on RHEL/Linux.
// Add additional types here as deployment coverage expands.
const ENCRYPTED_FS_TYPES: &[&str] = &["ecryptfs", "gocryptfs", "cryfs", "encfs", "fscrypt"];

//
// Encrypted mount detection
//

/// Read `/proc/mounts` and return the set of mount-point paths that use a
/// known encrypted filesystem type (ecryptfs, gocryptfs, etc.).
///
/// # Architectural Warning
///
/// This function reads `/proc/mounts` via `std::fs::read_to_string`. Before
/// enabling this function, replace the read with `ProcfsText` + `SecureReader`
/// from `umrs-platform`. See module-level doc for details.
fn load_encrypted_mounts() -> HashSet<String> {
    let mut set = HashSet::new();
    let Ok(contents) = std::fs::read_to_string("/proc/mounts") else {
        return set;
    };
    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let _device = parts.next();
        let Some(mount_point) = parts.next() else {
            continue;
        };
        let Some(fs_type) = parts.next() else {
            continue;
        };
        if ENCRYPTED_FS_TYPES.contains(&fs_type) {
            set.insert(mount_point.to_owned());
        }
    }

    set
}

/// Check whether the given mount point is backed by LUKS encryption.
///
/// Queries `/proc/self/mounts` to locate the underlying block device, then
/// inspects `/sys/class/block/<dev>/dm/type` and `/sys/class/block/<dev>/dm/uuid`
/// to determine whether device-mapper encryption (CRYPT-LUKS) is active.
///
/// Returns `true` only if the device-mapper type is `"crypt"` or the UUID
/// starts with `"CRYPT-LUKS"`.
///
/// # Architectural Warning
///
/// This function reads `/proc/self/mounts` and `/sys/class/block/` via
/// `std::fs::read_to_string`. Before enabling this function, replace all
/// procfs/sysfs reads with `ProcfsText`/`SysfsText` + `SecureReader` from
/// `umrs-platform`. See module-level doc for details.
///
/// NIST SP 800-53 SC-28 — LUKS check verifies at-rest encryption is active.
pub fn is_luks_encrypted<P: AsRef<Path>>(mount_point: P) -> bool {
    let target = mount_point.as_ref();

    // 1. Find the device in /proc/self/mounts.
    let mounts = match fs::read_to_string("/proc/self/mounts") {
        Ok(s) => s,
        Err(_) => return false,
    };
    let dev_node = mounts.lines().find_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Index 1 is the mount point, Index 0 is the device.
        if parts.len() >= 2 && Path::new(parts[1]) == target {
            Some(parts[0].to_string())
        } else {
            None
        }
    });

    let Some(dev_path) = dev_node else {
        return false;
    };

    // 2. Resolve to the kernel name (e.g., /dev/mapper/data -> /dev/dm-0).
    let Ok(real_path) = fs::canonicalize(&dev_path) else {
        return false;
    };
    let Some(kernel_name) = real_path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };

    // 3. Check the kernel's device-mapper type in sysfs.
    // This is the source of truth for active encryption.
    let type_path = format!("/sys/class/block/{kernel_name}/dm/type");
    if let Ok(dm_type) = fs::read_to_string(type_path) {
        if dm_type.trim() == "crypt" {
            return true;
        }
    }

    // 4. Fallback: check the UUID for the LUKS prefix.
    let uuid_path = format!("/sys/class/block/{kernel_name}/dm/uuid");
    if let Ok(uuid) = fs::read_to_string(uuid_path) {
        return uuid.trim().starts_with("CRYPT-LUKS");
    }

    false
}
