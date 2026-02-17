// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//!
//! Simplify some of themore complex tasks
//!
//! All of the actions in this module are read-only and operate in the userland.
//!

pub mod dirlist;
pub mod kattrs;

use std::fs::File;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::str::FromStr;

use crate::context::SecurityContext;
use crate::xattrs::SecureXattrReader;

use nix::libc;

///
/// High-Assurance file security context retrieval (libselinux-style)
///
/// Opens the file, anchors to inode, and retrieves the verified
/// SELinux security context.
///
pub fn get_file_context(path: &Path) -> io::Result<SecurityContext> {
    let file = File::open(path)?;
    SecureXattrReader::read_context(&file)
}

///
/// Symbolic link security context retrieval
///
pub fn lget_file_context(path: &Path) -> io::Result<SecurityContext> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)?;

    SecureXattrReader::read_context(&file)
}

///
/// Retrieve securit context from file descriptor
///
pub fn fget_file_context(file: &File) -> io::Result<SecurityContext> {
    SecureXattrReader::read_context(file)
}

// ===========================================================================

/// libselinux-style: `getpidcon()`
///
/// Reads `/proc/<pid>/attr/current` (procfs attribute contents) and parses it.
pub fn get_pid_context(pid: u32) -> io::Result<SecurityContext> {
    let path = format!("/proc/{pid}/attr/current");

    let raw = std::fs::read_to_string(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("ACCESS DENIED: Cannot read {path}: {e}"),
        )
    })?;

    let s = raw.trim();

    SecurityContext::from_str(s).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid security context in {path}: {e}"),
        )
    })
}

/// Convenience helper for current process
pub fn get_self_context() -> io::Result<SecurityContext> {
    get_pid_context(std::process::id())
}
