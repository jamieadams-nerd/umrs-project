// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//!
//! Userland utility helpers for umrs-selinux.
//!
//! Provides directory listing (`dirlist`) and file context helpers
//! (`get_file_context`, `lget_file_context`, `fget_file_context`,
//! `get_pid_context`, `get_self_context`).
//!
//! Kernel attribute access (formerly `kattrs`) has been promoted to the
//! `umrs-platform` crate and is re-exported as `umrs_platform::kattrs`.
//!

pub mod dirlist;

use std::fs::File;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::str::FromStr;

use crate::context::SecurityContext;
use crate::xattrs::{SecureXattrReader, XattrReadError};

use nix::libc;

///
/// High-Assurance file security context retrieval (libselinux-style)
///
/// Opens the file, anchors to inode, and retrieves the verified
/// SELinux security context.
///
pub fn get_file_context(path: &Path) -> io::Result<SecurityContext> {
    let file = File::open(path)?;
    SecureXattrReader::read_context(&file).map_err(xattr_err_to_io)
}

///
/// Symbolic link security context retrieval
///
pub fn lget_file_context(path: &Path) -> io::Result<SecurityContext> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)?;

    SecureXattrReader::read_context(&file).map_err(xattr_err_to_io)
}

///
/// Retrieve security context from file descriptor
///
pub fn fget_file_context(file: &File) -> io::Result<SecurityContext> {
    SecureXattrReader::read_context(file).map_err(xattr_err_to_io)
}

/// Map an `XattrReadError` to an `io::Error` for callers that use the
/// `io::Result<SecurityContext>` compatibility API.
///
/// OS errors are forwarded as-is.  TPI errors are mapped to
/// `InvalidData` so callers that cannot inspect `XattrReadError` directly
/// still receive a meaningful error kind.
fn xattr_err_to_io(e: XattrReadError) -> io::Error {
    match e {
        XattrReadError::OsError(io_err) => io_err,
        XattrReadError::Tpi(tpi_err) => {
            io::Error::new(io::ErrorKind::InvalidData, tpi_err.to_string())
        }
    }
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
