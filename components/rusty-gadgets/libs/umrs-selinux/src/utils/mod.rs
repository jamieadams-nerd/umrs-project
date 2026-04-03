// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! # Userland Utility Helpers
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
//! ## TPI Routing
//!
//! All file context helpers that return a `SecurityContext` route through
//! `SecureXattrReader::read_context()`, which enforces TPI. Callers do not
//! invoke the `nom` or `FromStr` parsers directly.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access Enforcement — file context helpers are
//!   the primary mechanism for retrieving the security label used in access
//!   control decisions.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — PID and self-context
//!   helpers provide the subject label required in audit records.
//! - **NSA RTB RAIN**: Non-Bypassability — all label reads route through the
//!   TPI gate in `xattrs.rs`; no direct parser invocation is permitted here.
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
///
/// # Errors
///
/// Returns `io::Error` if the file's security context xattr cannot be read.
pub fn get_file_context(path: &Path) -> io::Result<SecurityContext> {
    let file = File::open(path)?;
    SecureXattrReader::read_context(&file).map_err(xattr_err_to_io)
}

///
/// Symbolic link security context retrieval
///
///
/// # Errors
///
/// Returns `io::Error` if the symlink's security context xattr cannot be read.
pub fn lget_file_context(path: &Path) -> io::Result<SecurityContext> {
    let file = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW).open(path)?;

    SecureXattrReader::read_context(&file).map_err(xattr_err_to_io)
}

///
/// Retrieve security context from file descriptor
///
///
/// # Errors
///
/// Returns `io::Error` if the file descriptor's security context xattr cannot be read.
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
/// Reads `/proc/<pid>/attr/current` to retrieve the SELinux security context
/// of the process with the given PID.
///
/// ## Design Note — SecureReader not applicable for arbitrary PID paths
///
/// The UMRS project rules require all `/proc/` reads to route through
/// `SecureReader::read_generic_text` (via `ProcfsText`).  That pattern uses
/// compile-time path binding via the `StaticSource` trait — the path is an
/// associated constant, verified at compile time, and the filesystem magic is
/// confirmed before the read.
///
/// `ProcfsText` / `StaticSource` cannot express paths with a runtime component
/// (`/proc/<pid>/...`) because the path is not known at compile time.  Each
/// unique PID produces a distinct path; there is no way to bind an arbitrary
/// PID path as a `StaticSource`.
///
/// The risk is bounded by the following mitigations already in place:
///
/// - The path is constructed entirely from a validated `u32` PID.  No
///   user-supplied string is interpolated; no path traversal is possible.
/// - `/proc/<pid>/attr/current` is a kernel-generated virtual file.  Only the
///   kernel writes it; its content is the SELinux context of the target process
///   as assigned by the kernel at process creation.
/// - The `SecurityContext::from_str` parser is TPI-validated (two independent
///   parse paths via `nom` and `FromStr`), so any malformed content is rejected.
/// - SELinux enforcing mode prevents unauthorized reads of other processes'
///   `/proc/attr` entries via type enforcement policy (AC-3 primary protection).
///
/// A future enhancement could introduce a `DynamicProcReader` abstraction that
/// performs statfs-based provenance verification for runtime-constructed procfs
/// paths, but this requires architectural work beyond the current scope.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Access Enforcement — SELinux type enforcement
///   is the primary access control gate; this read retrieves subject labels for
///   audit records.
/// - **NIST SP 800-53 AU-3**: Audit Record Content — PID context is used in
///   audit events to identify the subject of an access decision.
/// - **NSA RTB RAIN**: Non-Bypassability — the parsed `SecurityContext` is
///   constructed via TPI, preventing a malformed procfs entry from producing
///   an accepted but incorrect label.
///
/// # Errors
///
/// Returns `io::Error` if `/proc/<pid>/attr/current` cannot be read.
pub fn get_pid_context(pid: u32) -> io::Result<SecurityContext> {
    let path = format!("/proc/{pid}/attr/current");

    let raw = std::fs::read_to_string(&path)
        .map_err(|e| io::Error::new(e.kind(), format!("ACCESS DENIED: Cannot read {path}: {e}")))?;

    let s = raw.trim();

    SecurityContext::from_str(s).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid security context in {path}: {e}"),
        )
    })
}

/// Convenience helper for current process
///
/// # Errors
///
/// Returns `io::Error` if `/proc/self/attr/current` cannot be read.
pub fn get_self_context() -> io::Result<SecurityContext> {
    get_pid_context(std::process::id())
}
