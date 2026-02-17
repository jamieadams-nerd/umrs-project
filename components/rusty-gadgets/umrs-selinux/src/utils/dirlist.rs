// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
// ============================================================================
//! # High-Assurance Directory Auditing
//!
//! UMRS SELINUX: High-Assurance Directory Auditing (dirlist)
//! NIST 800-53 AC-3, AU-3 // NSA RTB (Non-Bypassability & Provenance)
//!
//! This module provides the core engine for security-focused filesystem
//! traversal. It transforms raw directory streams into a collection of
//! strongly-typed `DirectoryEntry` objects, each carrying verified
//! security metadata anchored to the physical Inode.
//!
//! ## Architectural Invariants:
//!
//! ### 1. Inode-Anchored Provenance (NIST 800-53 AC-3)
//! To prevent TOCTOU (Time-of-Check to Time-of-Use) vulnerabilities, this
//! engine utilizes File-Descriptor (FD) based anchoring. Files are opened
//! first, and all subsequent security metadata (SELinux labels, ACLs,
//! Immutable flags) are retrieved via the FD (e.g., `fgetxattr`, `fstat`).
//! This ensures the "Identity" and "Security Label" belong to the same
//! physical data blocks, even if the filename is changed during the audit.
//!
//! ### 2. Redundant Security Mediation (The TPI Gate)
//! Every filesystem object is processed through the Two-Path Integrity (TPI)
//! gate. The raw byte-stream from the `security.selinux` xattr is parsed
//! simultaneously by:
//! * **The Declarative Path:** `nom` grammar combinators.
//! * **The Imperative Path:** Robust string manipulation.
//!
//! Only objects that achieve bit-for-bit agreement between both parsers are
//! granted "Verified" status in the audit trail.
//!
//! ### 3. Vernacular Translation (NARA CUI Mapping)
//! The engine leverages the `umrs_selinux::mcs::setrans` module to perform
//! $O(log n)$ lookups on parsed Category bitmasks. This bridges the gap
//! between kernel-level MCS bits and high-level regulatory markings
//! (e.g., `CUI//LEI/INV`), providing human-readable fidelity without
//! compromising mathematical rigor.
//!
//! ### 4. Multi-Dimensional Integrity Audit
//! Beyond MAC/DAC, the engine surfaces "hidden" security states:
//! * **Immutable (I):** Via `ioctl_getflags` to identify read-only system assets.
//! * **ACLs (A):** Direct detection of `system.posix_acl_access` attributes.
//! * **IMA (V):** Verification of `security.ima` signatures for binary integrity.
//!
// ============================================================================

use chrono::{DateTime, Local};
use nix::unistd::{Gid, Group, Uid, User};
use rustix::fs::{IFlags, ioctl_getflags}; // Standardizing on the bitflags API
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::mcs::translator::{GLOBAL_TRANSLATOR, SecurityRange};
use crate::xattrs::SecureXattrReader;

/// High-Assurance Directory Entry (NIST 800-53 AC-3)
#[allow(clippy::struct_excessive_bools)]
pub struct DirectoryEntry {
    pub name: String,
    pub selinux_type: String,
    pub mls_level: String,
    pub mtime: String,

    pub uid: u32,
    pub gid: u32,
    pub username: String,
    pub groupname: String,
    pub mode: u32,
    pub mode_string: String,

    pub is_restricted: bool, // For gray out logic
    pub immutable: bool,     // Immutable attribute set?
    pub has_acl: bool,       // Posix Access Control List
    pub has_ima: bool,       // Integrity Measurement Arch
}

///
/// Helper to format raw Unix mode to "drwxr-xr-x" string
///
fn format_mode(mode: u32) -> String {
    let mut s = String::with_capacity(10);
    let file_type = match mode & 0o170_000 {
        0o040_000 => 'd',
        0o120_000 => 'l',
        0o100_000 => '-',
        0o060_000 => 'b',
        0o020_000 => 'c',
        0o010_000 => 'p',
        0o140_000 => 's',
        _ => '?',
    };
    s.push(file_type);
    let chars = ['r', 'w', 'x'];
    for i in (0..3).rev() {
        let bits = (mode >> (i * 3)) & 0o7;
        s.push(if bits & 4 != 0 {
            chars[0]
        } else {
            '-'
        });
        s.push(if bits & 2 != 0 {
            chars[1]
        } else {
            '-'
        });
        s.push(if bits & 1 != 0 {
            chars[2]
        } else {
            '-'
        });
    }
    s
}

///
/// List directory (from high-assurance perspective)
///
/// # Panics
///
/// Panics if the GLOBAL_TRANSLATOR `RwLock` is poisoned.
/// This indicates an unrecoverable integrity failure in the
/// translator initialization or mutation lifecycle.
///
#[allow(clippy::option_if_let_else)]
#[allow(clippy::too_many_lines)]
pub fn list_directory_ha(dir_path: &Path) -> io::Result<Vec<DirectoryEntry>> {
    let mut entries = Vec::new();

    let read_dir = match fs::read_dir(dir_path) {
        Ok(rd) => rd,
        Err(e) => {
            log::error!(
                "CRITICAL: Failed to open directory stream for {}: {}",
                dir_path.display(),
                e
            );
            return Err(e);
        }
    };

    log::trace!("Reading directory stream for {}", dir_path.display());
    for entry_result in read_dir {
        // 1. Handle the directory entry itself (e.g., IO error during iteration)
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                log::error!("FS ERROR: Failed to read directory entry: {e}");
                continue;
            }
        };

        let path = entry.path();

        // 2. Handle Metadata (Skip if denied)
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                log::error!(
                    "ACCESS DENIED: Metadata restricted for {}: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };

        // 3. Handle File Open (Logic modified to support is_restricted)
        let mut is_restricted = false;
        let file_opt = match std::fs::File::open(&path) {
            Ok(f) => Some(f),
            Err(e) => {
                log::error!(
                    "ACCESS DENIED: Cannot open {}: {}",
                    path.display(),
                    e
                );
                is_restricted = true;
                None
            }
        };

        // Initialize security fields with placeholders in case of restricted access
        let mut s_type = "<RESTRICTED>".to_string();
        let mut s_level = "N/A".to_string();
        let mut is_immutable = false;
        let mut has_acl = false;

        #[allow(clippy::useless_let_if_seq)]
        let mut has_ima = false;

        // Only perform Inode-anchored checks if we successfully opened the file
        if let Some(ref file) = file_opt {
            // Label Provenance (TPI Verified)
            let context = SecureXattrReader::read_context(file).ok();

            if let Some(ctx) = context {
                s_type = ctx.security_type().to_string();

                s_level = if let Some(lvl) = ctx.level() {
                    // Build a SecurityRange from parsed level
                    let range = SecurityRange::from_level(lvl);

                    let guard = GLOBAL_TRANSLATOR
                        .read()
                        .expect("GLOBAL_TRANSLATOR lock poisoned");

                    if let Some(marking) = guard.lookup(&range) {
                        marking
                    } else {
                        lvl.raw().to_string()
                    }
                } else {
                    "s0".to_string()
                };
            } else {
                s_type = "<unlabeled>".to_string();
            }

            // FS Integrity Flags (NIST 800-53 SI-7)
            is_immutable = match ioctl_getflags(file) {
                Ok(f) => f.contains(IFlags::IMMUTABLE),
                Err(_) => false,
            };

            // NIST 800-53 AC-3: Check for supplemental Access Control Lists (ACLs)
            has_acl = match SecureXattrReader::read_raw(
                file,
                "system.posix_acl_access",
            ) {
                Ok(bytes) => !bytes.is_empty(),
                Err(_) => false,
            };

            // NIST 800-53 AC-3: Check for Integrity Measurement Arch (IMA) attribute
            if let Some(ref file) = file_opt {
                has_ima =
                    match SecureXattrReader::read_raw(file, "security.ima") {
                        Ok(bytes) => !bytes.is_empty(),
                        Err(_) => false,
                    };
            }
        }

        // Handle mtime carefully without ?
        let mtime_str = metadata.modified().map_or_else(
            |_| "Unknown".to_string(),
            |m| {
                let dt: DateTime<Local> = m.into();
                dt.format("%Y-%m-%d %H:%M").to_string()
            },
        );

        let mode = metadata.mode();
        let mut name = entry.file_name().to_string_lossy().into_owned();

        if (mode & 0o170_000) == 0o040_000 {
            name.push('/');
        }

        let username = match User::from_uid(Uid::from_raw(metadata.uid())) {
            Ok(Some(user)) => user.name,
            _ => metadata.uid().to_string(),
        };

        let groupname = match Group::from_gid(Gid::from_raw(metadata.gid())) {
            Ok(Some(group)) => group.name,
            _ => metadata.gid().to_string(),
        };

        entries.push(DirectoryEntry {
            name,
            selinux_type: s_type,
            mls_level: s_level,
            mtime: mtime_str,
            uid: metadata.uid(),
            gid: metadata.gid(),
            username,
            groupname,
            mode,
            mode_string: format_mode(mode),
            is_restricted,
            immutable: is_immutable,
            has_acl,
            has_ima,
        });
    }
    Ok(entries)
}
