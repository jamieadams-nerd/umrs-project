// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! # Kernel Attribute Read Engine — Traits, `SecureReader`, and `AttributeCard`
//!
//! Defines the two foundational traits (`KernelFileSource`, `StaticSource`) and
//! the mandatory read path (`SecureReader`) for all kernel pseudo-filesystem
//! attributes.  Every attribute type in this crate is required to route reads
//! through `SecureReader::execute_read`, which performs fd-anchored `fstatfs`
//! provenance verification before any bytes are parsed.
//!
//! NIST SP 800-53 SI-7: Software and Information Integrity.
//! NSA RTB RAIN: Non-bypassable — all reads must flow through this module.

use nix::sys::statfs::{FsType, SELINUX_MAGIC, fstatfs};
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::marker::PhantomData;
use std::path::Path;
use std::time::SystemTime;

/// Maximum byte buffer for a single kernel attribute read.
///
/// Sized to accommodate the longest attribute value in this codebase:
/// `/sys/kernel/security/lockdown` emits up to 33 bytes
/// (`none integrity [confidentiality]\n`).  A 64-byte buffer provides
/// comfortable headroom while remaining stack-allocated and heap-free.
pub(crate) const MAX_KATTR_READ: usize = 64;

// ===========================================================================
// Trait definitions
// ===========================================================================

/// Core contract for any file originating from a trusted Kernel Pseudo-FS.
///
/// NIST SP 800-53 SI-7: Software and Information Integrity — every implementor
/// must provide a parse function and the required kobject metadata constants.
pub trait KernelFileSource {
    type Output;

    /// The formal attribute name as defined in kernel kobject/sysfs vernacular.
    const ATTRIBUTE_NAME: &'static str;

    /// Documentation or format string derived from kernel-parameters.txt or rst docs.
    const DESCRIPTION: &'static str;

    /// Additional context regarding deprecation, defaults, or kernel version specifics.
    const KERNEL_NOTE: &'static str = "";

    /// The parent kobject in the kernel hierarchy (e.g., `"selinuxfs"` or `"crypto"`).
    const KOBJECT: &'static str;

    /// Low-level byte-slice parser for this attribute type.
    ///
    /// # Errors
    ///
    /// Returns `io::ErrorKind::InvalidData` if the byte slice does not
    /// represent a valid value for this attribute type. The exact conditions
    /// are implementation-defined by each `KernelFileSource` implementor.
    ///
    /// # Warning
    ///
    /// This is a low-level primitive. It operates on arbitrary byte slices
    /// with no provenance verification. For production use, always read
    /// kernel attributes through `SecureReader` or `StaticSource::read()`,
    /// which open the file and verify filesystem magic before any bytes are
    /// parsed. Calling `parse()` directly on bytes not obtained from a
    /// verified kernel source bypasses all NIST SP 800-53 SI-7 / NSA RTB RAIN
    /// guarantees.
    fn parse(data: &[u8]) -> io::Result<Self::Output>;
}

/// Contract for kernel attribute nodes with fixed, immutable paths.
///
/// The default `read()` method delegates to `SecureReader::execute_read`,
/// which opens the file first (anchoring to the inode) then verifies the
/// backing filesystem magic via fd-anchored `fstatfs` before parsing any
/// bytes. The magic check cannot be bypassed via this path.
///
/// NIST SP 800-53 SI-7: provenance-verified read path for all static nodes.
pub trait StaticSource: KernelFileSource + Sized {
    const PATH: &'static str;
    const EXPECTED_MAGIC: FsType = SELINUX_MAGIC;

    /// Provenance-verified read. Routes through `SecureReader::execute_read`.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the kernel attribute file cannot be opened, if
    /// the backing filesystem magic does not match `EXPECTED_MAGIC` (integrity
    /// failure), or if the byte content fails to parse.
    ///
    /// NIST SP 800-53 SI-10, SA-11: the result carries the security-relevant
    /// kernel attribute value and must not be silently discarded.
    #[must_use = "kernel attribute read result carries the provenance-verified value — \
                  discarding it silently loses the security-relevant kernel state"]
    fn read() -> io::Result<Self::Output> {
        SecureReader::<Self>::new().read()
    }
}

// ===========================================================================
// Audit Card
// ===========================================================================

/// A High-Assurance Audit Card capturing a kernel attribute read event.
///
/// Records the parsed value, path, and wall-clock read timestamp. For the
/// fully provenance-verified path, construct via `SecureReader::read_with_card()`
/// — only cards produced by that method carry the implicit proof that the value
/// was obtained through the magic-verified read path.
///
/// Direct field construction is permitted but does not carry provenance proof;
/// it is intended for display-format testing only.
///
/// NIST SP 800-53 AU-3: Audit record completeness (what, when, where, outcome).
pub struct AttributeCard<T: KernelFileSource> {
    pub value: T::Output,
    pub path: &'static str,
    pub read_at: SystemTime,
}

impl<T: KernelFileSource + StaticSource> fmt::Display for AttributeCard<T>
where
    T::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let read_at_secs = if let Ok(dur) = self.read_at.duration_since(SystemTime::UNIX_EPOCH) {
            dur.as_secs()
        } else {
            0u64
        };
        write!(
            f,
            r"--- [ UMRS KERNEL ATTRIBUTE CARD ] ---
KernelObj : {}
Attribute : {}
Path      : {}
Value     : {:?}
ReadAt    : {} (unix seconds)

Description:
{}

Note:
{}
---------------------------------------",
            T::KOBJECT,
            T::ATTRIBUTE_NAME,
            T::PATH,
            self.value,
            read_at_secs,
            T::DESCRIPTION,
            T::KERNEL_NOTE
        )
    }
}

// ===========================================================================
// SecureReader — the mandatory read engine
// ===========================================================================

/// Provenance-verified kernel attribute reader.
///
/// This is the mandatory read path for all kernel pseudo-filesystem attributes.
/// `execute_read` opens the file first (anchoring to the inode), then calls
/// fd-anchored `fstatfs` to verify the filesystem magic — eliminating the
/// TOCTOU window present in path-based magic checks — before parsing any bytes.
///
/// NIST SP 800-53 SI-7: Software and Information Integrity.
/// NSA RTB RAIN: Non-bypassable — all reads must flow through this type.
pub struct SecureReader<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for SecureReader<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SecureReader<T> {
    #[must_use = "SecureReader must be retained to call read() or read_with_card()"]
    #[allow(clippy::new_without_default)] // lint does not fire; const fn suppresses it — keep allow to document intent
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Core text read engine: open → fstatfs → read\_to\_string.
    ///
    /// Parallel to `execute_read` but for large-content pseudo-filesystem
    /// files (e.g., `/proc/mounts`) where a fixed-size stack buffer is
    /// inadequate. Uses `read_to_string` and allocates on the heap.
    ///
    /// Callers **must** be `ProcfsText` or `SysfsText` wrapper types that
    /// validate the path prefix at construction — this function is the
    /// engine; the caller is the gate.
    ///
    /// `pub(crate)` — all external callers route through `read_generic_text`
    /// on a typed wrapper (NIST SP 800-53 SI-7, NSA RTB RAIN).
    pub(crate) fn execute_read_text(path: &Path, expected_magic: FsType) -> io::Result<String> {
        // TOCTOU safety: open first to anchor the inode, then verify magic
        // on the open fd. Path is never re-resolved after open.
        let mut file = File::open(path)?;
        let stats = fstatfs(&file).map_err(io::Error::other)?;

        if stats.filesystem_type() != expected_magic {
            log::error!(
                "INTEGRITY FAILURE: filesystem magic mismatch on '{}' \
                 (expected {:?}, got {:?})",
                path.display(),
                expected_magic,
                stats.filesystem_type()
            );
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Integrity failure: unauthorized filesystem",
            ));
        }

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

impl<T: StaticSource> SecureReader<T> {
    /// Provenance-verified read of a static kernel attribute node.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the kernel attribute file cannot be opened, if
    /// the backing filesystem magic does not match `T::EXPECTED_MAGIC`
    /// (integrity failure), or if the byte content fails to parse.
    ///
    /// NIST SP 800-53 SI-10, SA-11 / NSA RTB Fail Secure: the result carries
    /// the security-relevant kernel attribute value and must be examined.
    #[must_use = "kernel attribute read result carries the provenance-verified value — \
                  discarding it silently loses the security-relevant kernel state"]
    pub fn read(&self) -> io::Result<T::Output> {
        Self::execute_read(Path::new(T::PATH), T::EXPECTED_MAGIC)
    }
}

impl<T: StaticSource> SecureReader<T>
where
    T::Output: fmt::Debug,
{
    /// Provenance-verified read that also returns an `AttributeCard` audit record.
    ///
    /// The card captures the parsed value, path, and wall-clock read time.
    /// Cards constructed via this method are proof that the value was obtained
    /// through the full provenance-verified path (open → fstatfs → parse).
    /// See [`AttributeCard`] — only cards produced via this method carry the
    /// provenance guarantee.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the kernel attribute file cannot be opened, if
    /// the backing filesystem magic does not match `T::EXPECTED_MAGIC`
    /// (integrity failure), or if the byte content fails to parse.
    ///
    /// NIST SP 800-53 AU-3: Audit record completeness.
    #[must_use = "AttributeCard is the audit record for this kernel attribute read — \
                  discarding it loses the provenance-verified audit trail"]
    pub fn read_with_card(&self) -> io::Result<AttributeCard<T>> {
        let value = Self::execute_read(Path::new(T::PATH), T::EXPECTED_MAGIC)?;
        Ok(AttributeCard {
            value,
            path: T::PATH,
            read_at: SystemTime::now(),
        })
    }
}

impl<T: KernelFileSource> SecureReader<T> {
    /// Core read engine: open → fstatfs → parse.
    ///
    /// Opens the file first to anchor to the inode, then verifies the
    /// filesystem magic on the open fd before reading any bytes.
    /// This ordering eliminates the TOCTOU race that exists in path-based
    /// statfs checks (NIST SP 800-53 SI-7, NSA RTB RAIN).
    ///
    /// `pub(crate)` so that sibling modules (e.g., `selinux`) can implement
    /// specialised `read_generic` wrappers for dynamic-path nodes.
    pub(crate) fn execute_read(path: &Path, expected_magic: FsType) -> io::Result<T::Output> {
        // TOCTOU safety (NIST SP 800-53 SI-7): open the file FIRST to anchor to the
        // inode, then verify filesystem magic on the open fd via fstatfs. Using
        // fd-anchored fstatfs eliminates the race window between a path-based statfs
        // and a subsequent open that could be exploited via bind-mount substitution.
        let mut file = File::open(path)?;
        let stats = fstatfs(&file).map_err(io::Error::other)?;

        if stats.filesystem_type() != expected_magic {
            // NIST SP 800-53 AU-2 / Loud Failure: log path to the audit stream
            // (access-controlled); return a generic error to the caller (SI-12).
            log::error!(
                "INTEGRITY FAILURE: filesystem magic mismatch on '{}' \
                 (expected {:?}, got {:?})",
                path.display(),
                expected_magic,
                stats.filesystem_type()
            );
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Integrity failure: unauthorized filesystem",
            ));
        }

        let mut buffer = [0u8; MAX_KATTR_READ];
        let bytes_read = file.read(&mut buffer)?;

        T::parse(&buffer[..bytes_read])
    }
}
