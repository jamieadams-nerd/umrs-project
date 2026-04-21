// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
//! # Secure File-Level Operations
//!
//! Secure file-level operations built on top of `SecureDirent`. Each function
//! opens the file once, builds a `SecureDirent` metadata snapshot, performs the
//! requested operation using the same fd, and drops the fd before returning.
//! Callers receive the `SecureDirent` alongside the operation result so they can
//! inspect security posture (SELinux label, mode, ownership) before trusting the
//! output. No kernel objects are retained past return.
//!
//! ## Design Pattern
//!
//! All functions delegate to the crate-private `SecureDirent::open_and_observe`
//! helper, which provides:
//! - A single `open(2)` covering both metadata collection and content read
//! - TOCTOU-safe fd-anchored attribute reads (inode flags, xattrs, SELinux TPI)
//! - Automatic fd drop before return (NSA RTB: Minimized TCB)
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-10**: `MAX_DIGEST_BYTES` and per-call `max_bytes`
//!   enforce explicit input-size bounds, preventing resource exhaustion from
//!   adversarial inputs. See `devel:bounded-reads.adoc` for the project-wide
//!   bounded-read catalog.
//! - **NIST SP 800-53 SI-7**: `compute_digests` uses FIPS-approved SHA-256 and
//!   SHA-384 (FIPS 180-4) to give operators tamper-evidence for file content.
//! - **NIST SP 800-53 AC-3 / AC-4**: SELinux label and mode are captured from
//!   the same fd as the content — no TOCTOU window between metadata and bytes.
//! - **NIST SP 800-53 AU-3**: Callers receive the full `SecureDirent` alongside
//!   the operation result, providing a complete audit record at the call site.
//! - **NSA RTB — Non-Bypassability**: all reads route through `SecureDirent`,
//!   which mandates the TPI SELinux gate and fd-anchored xattr reads.
//! - **FIPS 180-4**: SHA-256 (32-byte) and SHA-384 (48-byte) are FIPS-approved
//!   hash algorithms; used exclusively in `compute_digests`.

use std::io::Read as _;
use std::path::Path;

#[cfg(debug_assertions)]
use std::time::Instant;

use sha2::{Digest, Sha256, Sha384};

use crate::secure_dirent::{SecDirError, SecureDirent};

/// Maximum bytes for a digest computation. 16 GiB — files larger than this
/// fail with [`SecDirError::ContentTooLarge`]. This is a sanity ceiling,
/// not a buffer size; digests are streamed in fixed-size chunks regardless of
/// file length.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-5 / SI-10 / CWE-400**: bounded resource consumption.
///   See `devel:bounded-reads.adoc` for the project-wide bounded-read catalog.
pub const MAX_DIGEST_BYTES: u64 = 16 * 1024 * 1024 * 1024;

/// Open `path`, read up to `max_bytes` of content, return the metadata
/// snapshot alongside the bytes. Single `open(2)`.
///
/// Non-regular-file and over-limit cases fail without reading any content.
///
/// The caller can inspect `dirent.selinux_label`, `dirent.mode`, and
/// `dirent.ownership` before trusting the returned bytes.
///
/// # Errors
///
/// - [`SecDirError::NotRegularFile`] — path is not a regular file.
/// - [`SecDirError::ContentTooLarge`] — file size exceeds `max_bytes`.
/// - [`SecDirError::Metadata`] — `symlink_metadata()` failed.
/// - [`SecDirError::InvalidPath`] / [`SecDirError::InvalidFileName`] — path validation failed.
/// - [`SecDirError::Io`] — I/O error reading content.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: `max_bytes` cap enforces input-size bound.
/// - **NIST SP 800-53 AC-3 / AC-4**: SELinux label and content share one fd.
/// - **NSA RTB — Minimized TCB**: fd is dropped before return.
#[must_use = "content bytes and metadata snapshot are the only output; discarding them loses both"]
pub fn read_bytes(
    path: &Path,
    max_bytes: usize,
) -> Result<(SecureDirent, Vec<u8>), SecDirError> {
    #[cfg(debug_assertions)]
    let start = Instant::now();

    let result = SecureDirent::from_path_with_content(path, max_bytes)?;

    #[cfg(debug_assertions)]
    log::debug!(
        "secure_file::read_bytes completed in {} µs for {}",
        start.elapsed().as_micros(),
        path.display(),
    );

    Ok(result)
}

/// Open `path`, read up to `max_bytes` of content, decode as UTF-8, and return
/// the metadata snapshot alongside the decoded string. Single `open(2)`.
///
/// Non-regular-file, over-limit, and non-UTF-8 cases all fail cleanly. The
/// caller can inspect `dirent.selinux_label`, `dirent.mode`, and
/// `dirent.ownership` before trusting the returned string.
///
/// # Errors
///
/// - [`SecDirError::NotRegularFile`] — path is not a regular file.
/// - [`SecDirError::ContentTooLarge`] — file size exceeds `max_bytes`.
/// - [`SecDirError::InvalidUtf8`] — file content is not valid UTF-8.
/// - [`SecDirError::Metadata`] — `symlink_metadata()` failed.
/// - [`SecDirError::InvalidPath`] / [`SecDirError::InvalidFileName`] — path validation failed.
/// - [`SecDirError::Io`] — I/O error reading content.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: `max_bytes` cap enforces input-size bound;
///   UTF-8 validation prevents log injection from non-UTF-8 security-relevant
///   files.
/// - **NIST SP 800-53 AC-3 / AC-4**: SELinux label and content share one fd.
/// - **NSA RTB — Minimized TCB**: fd is dropped before return.
#[must_use = "decoded content and metadata snapshot are the only output; discarding them loses both"]
pub fn read_to_string(
    path: &Path,
    max_bytes: usize,
) -> Result<(SecureDirent, String), SecDirError> {
    #[cfg(debug_assertions)]
    let start = Instant::now();

    let (dirent, bytes) = SecureDirent::from_path_with_content(path, max_bytes)?;
    let s = std::str::from_utf8(&bytes).map_err(SecDirError::InvalidUtf8)?;

    #[cfg(debug_assertions)]
    log::debug!(
        "secure_file::read_to_string completed in {} µs for {}",
        start.elapsed().as_micros(),
        path.display(),
    );

    Ok((dirent, s.to_owned()))
}

/// Stream the file at `path` through SHA-256 and SHA-384 hashers simultaneously.
///
/// Returns the `SecureDirent` metadata snapshot alongside both digests as fixed-size
/// byte arrays (32 bytes for SHA-256, 48 bytes for SHA-384). The file is read in
/// 8 KiB chunks; memory usage is bounded to the chunk size regardless of file length.
///
/// Files larger than `max_bytes` fail before any I/O is performed.
///
/// This is a caller-facing helper for tools that show file identity to an operator.
///
/// # Errors
///
/// - [`SecDirError::NotRegularFile`] — path is not a regular file.
/// - [`SecDirError::ContentTooLarge`] — file size exceeds `max_bytes`.
/// - [`SecDirError::Metadata`] — `symlink_metadata()` failed.
/// - [`SecDirError::InvalidPath`] / [`SecDirError::InvalidFileName`] — path validation failed.
/// - [`SecDirError::Io`] — I/O error during streaming.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: SHA-256 and SHA-384 provide tamper-evidence display.
/// - **FIPS 180-4**: both algorithms are FIPS-approved.
/// - **NIST SP 800-53 SC-5 / SI-10 / CWE-400**: `max_bytes` cap and streaming
///   prevent resource exhaustion. See `devel:bounded-reads.adoc`.
/// - **NIST SP 800-53 AC-3 / AC-4**: metadata and bytes share one fd.
/// - **NSA RTB — Minimized TCB**: fd is dropped before return.
#[must_use = "digest arrays and metadata snapshot are the only output; discarding them loses tamper-evidence data"]
pub fn compute_digests(
    path: &Path,
    max_bytes: u64,
) -> Result<(SecureDirent, [u8; 32], [u8; 48]), SecDirError> {
    #[cfg(debug_assertions)]
    let start = Instant::now();

    // Pre-check size via lstat before opening, consistent with from_path_with_content.
    // open_and_observe will re-stat, but we need an early rejection here so that the
    // streaming closure never runs on an oversized file.
    //
    // NIST SP 800-53 SI-10: reject before opening.
    use std::os::unix::fs::MetadataExt as _;
    let pre_meta = std::fs::symlink_metadata(path).map_err(SecDirError::Metadata)?;
    let file_type = crate::secure_dirent::FileType::from_mode(pre_meta.mode());
    if !file_type.is_regular() {
        return Err(SecDirError::NotRegularFile);
    }
    let raw_size = pre_meta.size();
    if raw_size > max_bytes {
        return Err(SecDirError::ContentTooLarge {
            size: raw_size,
            limit: max_bytes as usize,
        });
    }

    let (dirent, (sha256_arr, sha384_arr)) =
        SecureDirent::open_and_observe(path, |file, _dirent| {
            let mut sha256 = Sha256::new();
            let mut sha384 = Sha384::new();
            let mut chunk = [0u8; 8192];
            let mut total: u64 = 0;

            loop {
                let n = file.read(&mut chunk).map_err(SecDirError::Io)?;
                if n == 0 {
                    break;
                }
                // Defense in depth: abort if more bytes arrive than lstat reported.
                // This guards against a growing file between lstat and read.
                // NIST SP 800-53 SI-10.
                total = total.saturating_add(n as u64);
                if total > max_bytes {
                    return Err(SecDirError::ContentTooLarge {
                        size: total,
                        limit: max_bytes as usize,
                    });
                }
                sha256.update(&chunk[..n]);
                sha384.update(&chunk[..n]);
            }

            let digest256: [u8; 32] = sha256
                .finalize()
                .into();
            let digest384: [u8; 48] = sha384
                .finalize()
                .into();

            Ok((digest256, digest384))
        })?;

    #[cfg(debug_assertions)]
    log::debug!(
        "secure_file::compute_digests completed in {} µs for {}",
        start.elapsed().as_micros(),
        path.display(),
    );

    Ok((dirent, sha256_arr, sha384_arr))
}

/// Open `path` and read exactly `N` bytes from the start of the file.
///
/// Returns `(SecureDirent, [u8; N])`. If the file is shorter than `N` bytes,
/// fails with [`SecDirError::FileTooSmall`] rather than a generic I/O error,
/// so callers can distinguish "file is too short to hold a magic header" from
/// genuine I/O failure.
///
/// Used for magic-byte identification (ELF, PE, ZIP, etc.). The const-generic
/// buffer avoids heap allocation for small magic reads.
///
/// # Errors
///
/// - [`SecDirError::NotRegularFile`] — path is not a regular file.
/// - [`SecDirError::FileTooSmall`] — file is shorter than `N` bytes.
/// - [`SecDirError::Metadata`] — `symlink_metadata()` failed.
/// - [`SecDirError::InvalidPath`] / [`SecDirError::InvalidFileName`] — path validation failed.
/// - [`SecDirError::Io`] — I/O error reading the magic bytes.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: fixed-size read bounds resource consumption.
/// - **NIST SP 800-53 AC-3 / AC-4**: metadata and bytes share one fd.
/// - **NSA RTB — Minimized TCB**: fd is dropped before return.
#[must_use = "magic bytes and metadata snapshot are the only output; discarding them loses file-type identification"]
pub fn read_magic<const N: usize>(
    path: &Path,
) -> Result<(SecureDirent, [u8; N]), SecDirError> {
    #[cfg(debug_assertions)]
    let start = Instant::now();

    // Pre-check: file must be regular and at least N bytes long.
    use std::os::unix::fs::MetadataExt as _;
    let pre_meta = std::fs::symlink_metadata(path).map_err(SecDirError::Metadata)?;
    let file_type = crate::secure_dirent::FileType::from_mode(pre_meta.mode());
    if !file_type.is_regular() {
        return Err(SecDirError::NotRegularFile);
    }
    let raw_size = pre_meta.size();
    if (raw_size as usize) < N {
        return Err(SecDirError::FileTooSmall {
            size: raw_size,
            wanted: N,
        });
    }

    let (dirent, buf) = SecureDirent::open_and_observe(path, |file, _dirent| {
        let mut buf = [0u8; N];
        file.read_exact(&mut buf).map_err(|e| {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                SecDirError::FileTooSmall {
                    size: raw_size,
                    wanted: N,
                }
            } else {
                SecDirError::Io(e)
            }
        })?;
        Ok(buf)
    })?;

    #[cfg(debug_assertions)]
    log::debug!(
        "secure_file::read_magic::<{}> completed in {} µs for {}",
        N,
        start.elapsed().as_micros(),
        path.display(),
    );

    Ok((dirent, buf))
}
