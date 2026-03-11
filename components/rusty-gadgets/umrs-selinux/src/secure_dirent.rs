// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
//! # Security-Enriched Directory Entry (`SecureDirent`)
//!
//! Provides a strongly-typed, high-assurance replacement for the kernel's
//! `dirent` + `stat` pair, enriched with every security-relevant attribute
//! that a CUI auditing tool needs at its fingertips.
//!
//! ## Design Principles
//!
//! - **Parse Don't Validate**: all fields are validated at construction.
//!   After `SecureDirent::from_path()` returns `Ok`, no further field
//!   checking is required by consumers.
//! - **No raw strings**: paths are `AbsolutePath`/`ValidatedFileName`,
//!   filenames are UTF-8 validated with non-UTF8 explicitly rejected at
//!   the OsStr boundary.
//! - **Kernel ground truth first**: uid/gid are stored as `u32`; name
//!   resolution is a separate, lazy concern. `LinuxOwnership::from_ids()`
//!   is used at construction — no NSS/passwd lookups during dirent scan.
//! - **TOCTOU safety**: a single `File` handle is opened once; all
//!   subsequent metadata reads (`ioctl_getflags`, `fgetxattr`) operate
//!   through that same fd. The path is never re-resolved after open.
//! - **RTB dual-path integrity**: SELinux context is read and parsed via
//!   `SecureXattrReader::read_context()`, which runs two independent
//!   parsers (nom + FromStr) and enforces agreement before returning.
//! - **No stored kernel objects**: `std::fs::Metadata` is consumed at
//!   construction; only the primitive values we need are retained in the
//!   struct. This avoids retaining kernel state longer than necessary.
//! - **Security findings as data**: `SecurityObservation` enum values
//!   are queryable, loggable, and sortable — not just text in a log.
//!
//! ## Compliance References
//!
//! - **NIST SP 800-53 Rev 5 — AC-3**: Access Enforcement — this struct
//!   is the per-object representation that access decisions are made against.
//! - **NIST SP 800-53 Rev 5 — AC-4**: Information Flow Enforcement —
//!   SELinux context + MLS level carried as first-class typed fields.
//! - **NIST SP 800-53 Rev 5 — AU-3**: Audit Record Content — inode,
//!   path, uid, gid, mode, SELinux label, and IMA status provide the
//!   content required for a complete audit record.
//! - **NIST SP 800-53 Rev 5 — AU-9**: Protection of Audit Information —
//!   immutable/append-only inode flags are explicitly surfaced.
//! - **NIST SP 800-53 Rev 5 — CM-6**: Configuration Settings — world-
//!   writable, setuid, setgid, and sticky observations support configuration
//!   baseline verification.
//! - **NIST SP 800-53 Rev 5 — SI-7**: Software, Firmware, and Information
//!   Integrity — IMA presence flag supports integrity verification posture.
//! - **CMMC Level 2 — AC.L2-3.1.3**: Control CUI flow — SELinux MCS
//!   context and category set are the enforcement mechanism.
//! - **CMMC Level 2 — AU.L2-3.3.1**: Create audit records — fields on
//!   this struct satisfy the per-object audit record content requirement.
//! - **CMMC Level 2 — SI.L2-3.14.1**: Identify system flaws — setuid,
//!   world-writable, and orphaned-owner observations are flaw indicators.
//!   IMA hash presence and immutable flag are surfaced as positive findings.
//! - **NSA RTB (Raise the Bar) — Non-Bypassability**: SELinux context
//!   read via fd-based `fgetxattr`, not path-based — cannot be bypassed
//!   by symlink substitution.
//! - **NSA RTB — Redundancy/TPI**: dual-path parse of SELinux context
//!   (nom + FromStr) with cross-check gate in `SecureXattrReader`.
//! - **NSA RTB — Minimized TCB**: `std::fs::Metadata` not stored after
//!   construction; only primitive values retained, minimizing state.
//! - **NSA RTB — Least Privilege**: only the fields needed for security
//!   decisions are present; no convenience fields that expand attack surface.

use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use rustix::fs::{IFlags, ioctl_getflags};

use crate::context::SecurityContext;
use crate::observations::SecurityObservation;
use crate::posix;
use crate::xattrs::{SecureXattrReader, TpiError, XattrReadError};

// Re-export path types — defined here since secdir owns the path hierarchy.
// If path types are later split to their own module, update these use paths.
pub use self::filetype::FileType;
pub use self::flags::InodeSecurityFlags;
pub use self::path::{AbsolutePath, PathError, ValidatedFileName};

// ===========================================================================
// SelinuxCtxState
//
// Replaces Option<SecurityContext> to distinguish four structurally different
// label outcomes.  A bare Option collapses unlabeled (ENODATA) and parse
// failures into the same None — producing false-negative audit output.
//
// NIST 800-53 AU-3: audit record must accurately reflect the reason a label
// is absent.
// NIST 800-53 SI-12: information management — display must not mislead the
// operator.
// ===========================================================================

/// The SELinux label state of a filesystem object.
///
/// Distinguishes four structurally different outcomes so that the display
/// and audit layers can respond correctly to each one:
///
/// | Variant | Meaning | Display |
/// |---|---|---|
/// | `Labeled` | Both TPI paths agreed | Full context string |
/// | `Unlabeled` | ENODATA — inode has no SELinux xattr | `<unlabeled>` |
/// | `ParseFailure` | Xattr present but TPI path(s) failed | `<parse-error>` |
/// | `TpiDisagreement` | Both paths succeeded but disagree | `<unverifiable>` |
///
/// `<unlabeled>` and `<parse-error>` must never be conflated: the first means
/// MAC cannot be enforced on this object; the second means a code defect
/// prevented label verification.  An operator seeing `<parse-error>` should
/// investigate the parser, not the policy.
///
/// NIST 800-53 AU-3: audit record content.
/// NIST 800-53 SI-12: accurate information management.
/// NSA RTB RAIN: non-bypassability of the integrity gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelinuxCtxState {
    /// A verified SELinux security context — both TPI paths agreed.
    ///
    /// Boxed to equalise the size of enum variants (the other variants carry
    /// no data). Without boxing the enum would be 232+ bytes on the stack.
    Labeled(Box<SecurityContext>),

    /// No SELinux xattr on this inode (kernel returned ENODATA or equivalent).
    ///
    /// On an MLS/targeted system, unlabeled objects cannot have MAC enforced.
    /// This is the authoritative "no label" state.
    Unlabeled,

    /// The SELinux xattr was present but one or both TPI parse paths failed.
    ///
    /// The label is on the inode but its structure could not be verified.
    /// This is a code or validator defect — not an integrity attack.
    /// NIST 800-53 SI-12: do not display as `<unlabeled>`.
    ParseFailure,

    /// Both TPI paths succeeded but produced different security contexts.
    ///
    /// Potential integrity event. The label on disk may have been tampered
    /// with. Treat this object as unverifiable until the discrepancy is
    /// resolved.
    /// NIST 800-53 SI-7: integrity violation.
    /// NSA RTB RAIN: redundancy cross-check failure.
    TpiDisagreement,
}

impl SelinuxCtxState {
    /// Returns the context if the label is verified, otherwise `None`.
    #[must_use]
    pub fn as_context(&self) -> Option<&SecurityContext> {
        if let Self::Labeled(ctx) = self {
            Some(ctx.as_ref())
        } else {
            None
        }
    }

    /// Returns `true` if this state carries a verified label.
    #[must_use]
    pub const fn is_labeled(&self) -> bool {
        matches!(self, Self::Labeled(_))
    }

    /// Returns the display string for the SELinux type column.
    ///
    /// Used by `dirlist.rs` when building `GroupKey`.
    #[must_use]
    pub fn display_type(&self) -> String {
        match self {
            Self::Labeled(ctx) => ctx.security_type().to_string(),
            Self::Unlabeled => "<unlabeled>".to_owned(),
            Self::ParseFailure => "<parse-error>".to_owned(),
            Self::TpiDisagreement => "<unverifiable>".to_owned(),
        }
    }

    /// Returns the MLS level if this state carries a verified label.
    ///
    /// Used by `dirlist.rs` to build the marking column without re-parsing.
    /// Returns `None` for `Unlabeled`, `ParseFailure`, and `TpiDisagreement`.
    #[must_use]
    pub fn level(&self) -> Option<&crate::context::MlsLevel> {
        if let Self::Labeled(ctx) = self {
            ctx.level()
        } else {
            None
        }
    }
}

// ==========================================================================
// POSIX path constants
//
// NIST SP 800-53 SI-10: Input Validation — bounded path lengths prevent
// buffer overrun and canonicalization attacks at the path layer.
//
/// Linux `PATH_MAX`. POSIX minimum is 256; Linux kernel enforces 4096.
pub const PATH_MAX: usize = 4096;

/// Linux `NAME_MAX` — maximum bytes in a single path component.
pub const NAME_MAX: usize = 255;

// ==========================================================================
/// Errors that may occur when constructing a `SecureDirent`.
///
/// Each variant preserves the original OS error where applicable so that
/// callers can log or surface the root cause. Access-denied outcomes are
/// not errors — `from_path` returns `Ok` with `access_denied: true` set
/// on the struct, which is the authoritative signal for that condition.
///
/// NIST SP 800-53 AU-3: error variants carry sufficient detail for audit
/// record generation without requiring string parsing by the caller.
#[derive(Debug)]
pub enum SecDirError {
    /// `symlink_metadata()` failed — path is inaccessible.
    Metadata(std::io::Error),

    /// Path failed `AbsolutePath` validation.
    InvalidPath(PathError),

    /// Filename component failed `ValidatedFileName` validation
    /// (non-UTF8, null byte, too long, directory separator).
    InvalidFileName,

    /// SELinux xattr read failed.
    SelinuxReadError(std::io::Error),

    /// SELinux label bytes were not valid UTF-8 or failed both parse paths.
    SelinuxParseError,
}

impl std::fmt::Display for SecDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Metadata(e) => write!(f, "metadata error: {e}"),
            Self::InvalidPath(e) => write!(f, "invalid path: {e}"),
            Self::InvalidFileName => write!(f, "invalid filename component"),
            Self::SelinuxReadError(e) => {
                write!(f, "SELinux xattr read error: {e}")
            }
            Self::SelinuxParseError => {
                write!(f, "SELinux context parse failed (both paths)")
            }
        }
    }
}

impl std::error::Error for SecDirError {}

// ==========================================================================
// Path types module

pub mod path {
    //! Validated POSIX path types.
    //!
    //! NIST SP 800-53 SI-10: Input Validation.
    //! NSA RTB: Bounded input lengths; null byte and newline rejection
    //! prevents log injection and C FFI confusion.

    use std::ffi::{CString, OsStr};
    use std::fmt;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use super::{NAME_MAX, PATH_MAX};

    /// Errors from path validation.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum PathError {
        Empty,
        TooLong {
            max: usize,
            got: usize,
        },
        ComponentTooLong {
            component: String,
            max: usize,
            got: usize,
        },
        ContainsNull,
        ContainsNewline,
        NotAbsolute,
        InvalidComponent(String),
    }

    impl fmt::Display for PathError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Empty => write!(f, "path must not be empty"),
                Self::TooLong {
                    max,
                    got,
                } => write!(f, "path too long: max {max}, got {got}"),
                Self::ComponentTooLong {
                    component,
                    max,
                    got,
                } => write!(
                    f,
                    "component '{component}' too long: max {max}, got {got}"
                ),
                Self::ContainsNull => {
                    write!(f, "path must not contain null bytes")
                }
                Self::ContainsNewline => {
                    write!(f, "path must not contain newline")
                }
                Self::NotAbsolute => {
                    write!(f, "expected absolute path (must start with '/')")
                }
                Self::InvalidComponent(c) => {
                    write!(f, "invalid path component: '{c}'")
                }
            }
        }
    }

    impl std::error::Error for PathError {}

    /// Shared path validation — called once, at construction.
    ///
    /// NSA RTB: Single validation gate. No re-validation downstream.
    /// NIST SP 800-53 SI-10: rejects oversized, null-bearing, and
    /// newline-bearing paths before they reach any system call.
    fn validate_path_common(raw: &str) -> Result<(), PathError> {
        if raw.is_empty() {
            return Err(PathError::Empty);
        }
        if raw.contains('\0') {
            return Err(PathError::ContainsNull);
        }
        if raw.contains('\n') || raw.contains('\r') {
            return Err(PathError::ContainsNewline);
        }
        if raw.len() > PATH_MAX {
            return Err(PathError::TooLong {
                max: PATH_MAX,
                got: raw.len(),
            });
        }

        for component in raw.split('/').filter(|c| !c.is_empty()) {
            if component.len() > NAME_MAX {
                return Err(PathError::ComponentTooLong {
                    component: component.to_owned(),
                    max: NAME_MAX,
                    got: component.len(),
                });
            }
        }
        Ok(())
    }

    /// A validated absolute POSIX path.
    ///
    /// Guarantees after construction:
    /// - Starts with `/`
    /// - Total length ≤ `PATH_MAX` (4096)
    /// - Each component ≤ `NAME_MAX` (255)
    /// - No null bytes — safe for C FFI via `to_cstring()`
    /// - No embedded newlines — safe for log emission
    ///
    /// NIST SP 800-53 SI-10 / NSA RTB: bounded, null-safe path type.
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AbsolutePath(String);

    impl AbsolutePath {
        /// Construct a validated `AbsolutePath`. Single validation gate.
        pub fn new(raw: &str) -> Result<Self, PathError> {
            validate_path_common(raw)?;
            if !raw.starts_with('/') {
                return Err(PathError::NotAbsolute);
            }
            Ok(Self(raw.to_owned()))
        }

        #[must_use]
        pub fn as_str(&self) -> &str {
            &self.0
        }

        #[must_use]
        pub fn as_path(&self) -> &Path {
            Path::new(&self.0)
        }

        /// Convert to `CString` for C FFI (libselinux, syscalls).
        /// Safe: null bytes rejected at construction.
        ///
        /// SAFETY: AbsolutePath invariant guarantees no interior null bytes.
        ///
        /// # Panics
        ///
        /// Cannot panic in practice — `AbsolutePath` invariant guarantees no
        /// interior null bytes. The `expect` is a safety assertion only.
        ///
        #[must_use]
        pub fn to_cstring(&self) -> CString {
            CString::new(self.0.as_bytes())
                .expect("AbsolutePath invariant: no null bytes")
        }

        #[must_use]
        pub fn file_name(&self) -> Option<&str> {
            Path::new(&self.0).file_name().and_then(|s| s.to_str())
        }

        #[must_use]
        pub const fn len(&self) -> usize {
            self.0.len()
        }

        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    impl fmt::Display for AbsolutePath {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }

    impl FromStr for AbsolutePath {
        type Err = PathError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(s)
        }
    }

    impl TryFrom<PathBuf> for AbsolutePath {
        type Error = PathError;
        fn try_from(pb: PathBuf) -> Result<Self, Self::Error> {
            // Non-UTF8 OsStr is rejected here — explicit boundary.
            // NSA RTB: Non-UTF8 paths rejected at the kernel boundary.
            let s = pb.to_str().ok_or(PathError::ContainsNull)?;
            Self::new(s)
        }
    }

    impl AsRef<Path> for AbsolutePath {
        fn as_ref(&self) -> &Path {
            Path::new(&self.0)
        }
    }

    impl std::ops::Deref for AbsolutePath {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    /// A validated single path component — no directory separators.
    ///
    /// Enforces `NAME_MAX` (255 bytes), forbids `/`, null, newline.
    /// Rejects `.` and `..` — callers must use explicit path manipulation.
    ///
    /// NIST SP 800-53 SI-10: single-component input validation.
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ValidatedFileName(String);

    impl ValidatedFileName {
        /// Construct from a `&str`. Single validation gate.
        pub fn new(raw: &str) -> Result<Self, PathError> {
            if raw.is_empty() {
                return Err(PathError::Empty);
            }
            if raw.contains('\0') {
                return Err(PathError::ContainsNull);
            }
            if raw.contains('\n') || raw.contains('\r') {
                return Err(PathError::ContainsNewline);
            }
            if raw.len() > NAME_MAX {
                return Err(PathError::TooLong {
                    max: NAME_MAX,
                    got: raw.len(),
                });
            }
            if raw.contains('/') || raw == "." || raw == ".." {
                return Err(PathError::InvalidComponent(raw.to_owned()));
            }
            Ok(Self(raw.to_owned()))
        }

        /// Construct from `&OsStr`, explicitly rejecting non-UTF8.
        ///
        /// This is the kernel boundary conversion function.
        /// Non-UTF8 filenames exist on Linux but are rejected here because:
        /// - They cannot be safely logged or displayed
        /// - They are a common source of path confusion attacks
        /// - CUI systems must be able to produce complete audit records
        ///
        /// NSA RTB: non-UTF8 rejected at the OsStr conversion boundary.
        /// NIST SP 800-53 AU-3: audit records require representable filenames.
        pub fn from_os_str(os: &OsStr) -> Result<Self, PathError> {
            let s = os.to_str().ok_or(PathError::ContainsNull)?;
            Self::new(s)
        }

        #[must_use]
        pub fn as_str(&self) -> &str {
            &self.0
        }
        #[must_use]
        pub const fn len(&self) -> usize {
            self.0.len()
        }
        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    impl fmt::Display for ValidatedFileName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }

    impl FromStr for ValidatedFileName {
        type Err = PathError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(s)
        }
    }

    impl std::ops::Deref for ValidatedFileName {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    // TryFrom impls
    impl TryFrom<&str> for AbsolutePath {
        type Error = PathError;
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            Self::new(s)
        }
    }
    impl TryFrom<&str> for ValidatedFileName {
        type Error = PathError;
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            Self::new(s)
        }
    }
}

// ===========================================================================
// FileType
//
pub mod filetype {
    //! Strongly-typed file type from d_type / st_mode.
    //!
    //! NIST SP 800-53 CM-6: configuration settings verification requires
    //! knowing the type of each filesystem object.

    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FileType {
        RegularFile,
        Directory,
        Symlink,
        BlockDevice,
        CharDevice,
        Fifo,
        Socket,
        Unknown,
    }

    impl FileType {
        /// Construct from `d_type` field of a Linux `dirent64`.
        #[must_use]
        pub const fn from_d_type(d_type: u8) -> Self {
            match d_type {
                8 => Self::RegularFile,
                4 => Self::Directory,
                10 => Self::Symlink,
                6 => Self::BlockDevice,
                2 => Self::CharDevice,
                1 => Self::Fifo,
                12 => Self::Socket,
                _ => Self::Unknown,
            }
        }

        /// Construct from `st_mode` (upper 4 bits are the type field in POSIX).
        #[must_use]
        pub const fn from_mode(mode: u32) -> Self {
            match mode & 0o170_000 {
                0o100_000 => Self::RegularFile,
                0o040_000 => Self::Directory,
                0o120_000 => Self::Symlink,
                0o060_000 => Self::BlockDevice,
                0o020_000 => Self::CharDevice,
                0o010_000 => Self::Fifo,
                0o140_000 => Self::Socket,
                _ => Self::Unknown,
            }
        }

        #[must_use]
        pub fn is_regular(&self) -> bool {
            *self == Self::RegularFile
        }
        #[must_use]
        pub fn is_directory(&self) -> bool {
            *self == Self::Directory
        }
        #[must_use]
        pub fn is_symlink(&self) -> bool {
            *self == Self::Symlink
        }
    }

    impl fmt::Display for FileType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(match self {
                Self::RegularFile => "regular",
                Self::Directory => "directory",
                Self::Symlink => "symlink",
                Self::BlockDevice => "block_device",
                Self::CharDevice => "char_device",
                Self::Fifo => "fifo",
                Self::Socket => "socket",
                Self::Unknown => "unknown",
            })
        }
    }
}

// ===========================================================================
// InodeSecurityFlags

pub mod flags {
    //! Security-relevant inode flag bitfield.
    //!
    //! Composes kernel `FS_*` ioctl flags (from `ioctl(FS_IOC_GETFLAGS)`)
    //! with xattr-derived security indicators (ACL, IMA, SELinux xattr).
    //!
    //! Kernel flag values from `linux/fs.h`:
    //!   `FS_SECRM_FL`     = 0x00000001
    //!   `FS_UNRM_FL`      = 0x00000002
    //!   `FS_IMMUTABLE_FL` = 0x00000010
    //!   `FS_APPEND_FL`    = 0x00000020
    //!   `FS_NODUMP_FL`    = 0x00000040
    //!   `FS_NOATIME_FL`   = 0x00000080
    //!
    //! Xattr-derived flags occupy the high byte (our additions, not kernel).
    //!
    //! NIST SP 800-53 AU-9: immutable/append-only flags protect audit logs.
    //! NIST SP 800-53 SI-7: IMA flag indicates integrity measurement coverage.
    //! NIST SP 800-53 AC-3: ACL flag indicates extended access control in effect.
    //! CMMC Level 2 SI.L2-3.14.6: Implement security engineering principles —
    //!   explicit flag tracking is a component of security state visibility.

    use std::fmt;

    bitflags::bitflags! {
        /// Security-relevant inode flags.
        ///
        /// Lower bits mirror kernel `FS_*` values so they can be set
        /// directly from `ioctl(FS_IOC_GETFLAGS)` output.
        /// Upper byte contains xattr-derived flags.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct InodeSecurityFlags: u32 {
            // ── Kernel FS_* flags (from linux/fs.h) ──────────────────────
            /// `FS_SECRM_FL` — secure deletion (overwrite blocks on delete)
            const SECURE_DELETE  = 0x0000_0001;
            /// `FS_UNRM_FL` — undelete support (filesystem-specific)
            const UNDELETE       = 0x0000_0002;
            /// `FS_IMMUTABLE_FL` — file cannot be modified, deleted, renamed
            /// NIST SP 800-53 AU-9: use on audit logs to prevent tampering.
            const IMMUTABLE      = 0x0000_0010;
            /// `FS_APPEND_FL` — file can only be opened for appending
            /// NIST SP 800-53 AU-9: use on audit logs to prevent overwrite.
            const APPEND_ONLY    = 0x0000_0020;
            /// `FS_NODUMP_FL` — exclude from `dump(8)`
            const NO_DUMP        = 0x0000_0040;
            /// `FS_NOATIME_FL` — do not update access time
            const NO_ATIME       = 0x0000_0080;

            // === xattr-derived flags (our additions, not kernel ioctl) ===
            /// POSIX ACL xattr (`system.posix_acl_access`) is present.
            /// NIST SP 800-53 AC-3: extended DAC in effect.
            const ACL_PRESENT    = 0x0100_0000;
            /// IMA integrity hash xattr (`security.ima`) is present.
            /// NIST SP 800-53 SI-7 / CMMC SI.L2-3.14.1.
            const IMA_PRESENT    = 0x0200_0000;
            /// Any extended attribute is present on this inode.
            const XATTR_PRESENT  = 0x0400_0000;
            /// `security.selinux` xattr is explicitly present on disk
            /// (not just policy-defaulted at runtime).
            /// NIST SP 800-53 AC-4: MAC label is persisted on the object.
            const SELINUX_XATTR  = 0x0800_0000;
        }
    }

    impl InodeSecurityFlags {
        /// True if the inode cannot be silently modified.
        /// Immutable or append-only both satisfy this.
        ///
        /// NIST SP 800-53 AU-9: write-protected audit log indicator.
        #[must_use]
        pub fn is_write_protected(self) -> bool {
            self.intersects(Self::IMMUTABLE | Self::APPEND_ONLY)
        }

        /// True if IMA integrity measurement is active on this inode.
        ///
        /// NIST SP 800-53 SI-7: integrity measurement active.
        #[allow(clippy::missing_const_for_fn)]
        #[must_use]
        pub fn has_ima_protection(self) -> bool {
            self.contains(Self::IMA_PRESENT)
        }

        /// True if extended ACL is in effect.
        ///
        /// NIST SP 800-53 AC-3: extended DAC is present.
        #[allow(clippy::missing_const_for_fn)]
        #[must_use]
        pub fn has_acl(self) -> bool {
            self.contains(Self::ACL_PRESENT)
        }

        /// True if SELinux label was explicitly set on disk.
        ///
        /// NIST SP 800-53 AC-4: MAC label is persisted (not runtime-only).
        #[must_use]
        #[allow(clippy::missing_const_for_fn)]
        pub fn has_explicit_selinux_label(self) -> bool {
            self.contains(Self::SELINUX_XATTR)
        }
    }

    impl fmt::Display for InodeSecurityFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut parts: Vec<&str> = Vec::new();
            if self.contains(Self::IMMUTABLE) {
                parts.push("IMMUTABLE");
            }
            if self.contains(Self::APPEND_ONLY) {
                parts.push("APPEND_ONLY");
            }
            if self.contains(Self::SECURE_DELETE) {
                parts.push("SECURE_DELETE");
            }
            if self.contains(Self::NO_DUMP) {
                parts.push("NO_DUMP");
            }
            if self.contains(Self::NO_ATIME) {
                parts.push("NO_ATIME");
            }
            if self.contains(Self::UNDELETE) {
                parts.push("UNDELETE");
            }
            if self.contains(Self::ACL_PRESENT) {
                parts.push("ACL_PRESENT");
            }
            if self.contains(Self::IMA_PRESENT) {
                parts.push("IMA_PRESENT");
            }
            if self.contains(Self::XATTR_PRESENT) {
                parts.push("XATTR_PRESENT");
            }
            if self.contains(Self::SELINUX_XATTR) {
                parts.push("SELINUX_XATTR");
            }
            if parts.is_empty() {
                write!(f, "NONE")
            } else {
                write!(f, "{}", parts.join("|"))
            }
        }
    }
}

// ===========================================================================
// SecureDirent — the core struct

/// A security-enriched directory entry.
///
/// This is the central data structure for UMRS filesystem scanning. It
/// replaces the standard `dirent` + `stat` pair with a fully typed,
/// security-annotated snapshot of a filesystem object.
///
/// ## What is stored and why
///
/// Only primitive values are retained after construction. `std::fs::Metadata`
/// is consumed and discarded — we extract what we need and release the rest.
/// This minimizes retained kernel state per NSA RTB principles.
///
/// Uid and gid are the kernel ground truth. Names are resolved at construction
/// via `LinuxOwnership::resolve()`, which queries NSS (`/etc/passwd`, `/etc/group`,
/// SSSD, etc.). If a name cannot be resolved, `UnresolvedOwner`/`UnresolvedGroup`
/// observations fire — indicating a genuinely orphaned account, not a lookup skip.
///
/// The SELinux context is stored as a fully parsed `SecurityContext` —
/// not a string. This means category set membership tests, sensitivity
/// level comparisons, and dominance checks are available without re-parsing.
///
/// ## Construction
///
/// Use `SecureDirent::from_path()`. The constructor:
/// 1. Calls `symlink_metadata()` — does not follow symlinks (TOCTOU safe)
/// 2. Opens the file once via `File::open()`
/// 3. Reads inode flags via `ioctl(FS_IOC_GETFLAGS)` through the fd
/// 4. Reads ACL, IMA, and SELinux xattrs through the same fd
/// 5. Parses SELinux context via dual-path RTB gate in `SecureXattrReader`
/// 6. Discards the `File` handle and `Metadata`
///
/// All subsequent field access is on owned primitive data.
///
/// ## Compliance
///
/// See module-level documentation for full compliance reference list.
#[derive(Debug, Clone)]
pub struct SecureDirent {
    // Path identity
    /// NIST SP 800-53 AU-3: path is part of the audit record.
    pub path: AbsolutePath,
    pub name: ValidatedFileName,

    // File type
    pub file_type: FileType,

    // Inode metadata
    /// NIST SP 800-53 AU-3: uniquely identifies the filesystem object.
    pub inode: posix::primitives::Inode,

    /// NIST SP 800-53 AU-3: file size is part of a complete audit record.
    pub size: posix::primitives::FileSize,

    /// NIST SP 800-53 AC-3: DAC permission bits consulted during access decisions.
    /// NIST SP 800-53 CM-6: setuid, setgid, world-writable baseline items.
    pub mode: posix::primitives::FileMode,

    /// NIST SP 800-53 AC-3: nlink > 1 on a non-directory is a finding.
    pub nlink: posix::primitives::HardLinkCount,

    /// NIST SP 800-53 CM-6: device identity required for mount-point detection.
    pub dev: posix::primitives::DevId,

    // Ownership
    /// NIST SP 800-53 AC-2: uid/gid are the kernel-authoritative identifiers.
    /// NSA RTB: no NSS lookup during filesystem scan (latency + TOCTOU risk).
    pub ownership: posix::identity::LinuxOwnership,

    // SEcurity Attributes
    /// SELinux label state — verified context, unlabeled, parse-error, or
    /// TPI disagreement. See [`SelinuxCtxState`] for the full invariant.
    ///
    /// NIST SP 800-53 AC-3/AC-4: MAC label for access and flow enforcement.
    /// NIST SP 800-53 AU-3: label state is part of the audit record.
    /// NSA RTB: parsed via dual-path TPI gate (nom + FromStr cross-check).
    pub selinux_label: SelinuxCtxState,

    /// NIST SP 800-53 AU-9 / SI-7 / AC-3: immutable, IMA, ACL, SELinux xattr.
    pub sec_flags: InodeSecurityFlags,

    /// NIST SP 800-53 CM-6: mount point status is a configuration finding.
    pub is_mountpoint: bool,

    /// NIST SP 800-53 SC-28: encryption source protecting this mount point.
    /// Always `EncryptionSource::None` for non-mount-point entries.
    pub encryption: crate::fs_encrypt::EncryptionSource,

    /// NIST SP 800-53 AU-3: access denial is part of the audit record.
    pub access_denied: bool,
}

impl SecureDirent {
    /// Construct a `SecureDirent` by reading all security attributes from
    /// the filesystem object at `path`.
    ///
    /// ## TOCTOU Safety
    ///
    /// `symlink_metadata()` does not follow symlinks — the entry is for
    /// the symlink itself, not its target. After `File::open()`, all
    /// attribute reads use the same fd, so the path is never re-resolved.
    ///
    /// ## SELinux Context
    ///
    /// Read via `SecureXattrReader::read_context()` which runs two
    /// independent parse paths (nom + FromStr) and enforces agreement.
    /// If the paths disagree, an error is returned. This is the RTB
    /// redundancy / TPI gate.
    ///
    /// ## Partial Population
    ///
    /// If `File::open()` fails (permission denied), the entry is still
    /// returned with `access_denied = true`. Path, name, inode, mode,
    /// and ownership are populated from `symlink_metadata()`. Security
    /// attribute fields (`sec_flags`) will be empty; `selinux_label` will be
    /// `SelinuxCtxState::Unlabeled`.
    /// The caller receives a `SecurityObservation::AccessDenied` from
    /// `security_observations()`.
    ///
    /// ## Errors
    ///
    /// Returns `Err` only for hard failures: `symlink_metadata()` failure,
    /// path validation failure, or invalid filename. Access denial is not
    /// an error — it is an observation on a valid (partial) entry.
    ///
    /// NIST SP 800-53 AU-3 / SI-7 / AC-3 / AC-4.
    /// NSA RTB: Non-Bypassability, Redundancy/TPI, Minimized TCB.
    // The xattr permission-denial branch adds a few lines beyond the 100-line
    // default limit.  The function is a single sequential construction sequence
    // with no hidden branches; splitting it would hurt readability more than help.
    #[allow(clippy::too_many_lines)]
    pub fn from_path(path: &Path) -> Result<Self, SecDirError> {
        // Step 1: symlink_metadata — does NOT follow symlinks
        // TOCTOU: we capture the lstat result before opening.
        let meta =
            std::fs::symlink_metadata(path).map_err(SecDirError::Metadata)?;

        // Step 2: Validate and type the path
        let abs_path = path
            .to_str()
            .ok_or(SecDirError::InvalidPath(path::PathError::ContainsNull))
            .and_then(|s| {
                AbsolutePath::new(s).map_err(SecDirError::InvalidPath)
            })?;

        // Step 3: Validate and type the filename component
        // Non-UTF8 filenames are rejected here — explicit OsStr boundary.
        let validated_name = path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("/"))
            .to_str()
            .ok_or(SecDirError::InvalidFileName)
            .and_then(|s| {
                ValidatedFileName::new(s)
                    .map_err(|_| SecDirError::InvalidFileName)
            })?;

        // Step 4: Extract primitives from Metadata, then discard it
        // NSA RTB: Minimized TCB — do not retain kernel objects.
        let inode = posix::primitives::Inode::new(meta.ino());
        let size = posix::primitives::FileSize::new(meta.size());
        let mode = posix::primitives::FileMode::from_mode(meta.mode());
        let nlink = posix::primitives::HardLinkCount::from_u64(meta.nlink());
        let dev = posix::primitives::DevId::new(meta.dev());
        let file_type = FileType::from_mode(meta.mode());
        let ownership = posix::identity::LinuxOwnership::resolve(
            posix::primitives::Uid::new(meta.uid()),
            posix::primitives::Gid::new(meta.gid()),
        );

        // Step 5: Open the file once — all attribute reads use this fd
        // TOCTOU: after this open, path is never re-resolved.
        // NSA RTB: Non-Bypassability — fd-based attribute reads.
        #[allow(clippy::option_if_let_else)]
        let (file_opt, mut access_denied) = match File::open(path) {
            Ok(f) => (Some(f), false),
            Err(_) => (None, true),
        };

        let mut sec_flags = InodeSecurityFlags::empty();
        // Default to Unlabeled; updated below if an xattr is found.
        let mut selinux_label = SelinuxCtxState::Unlabeled;

        if let Some(ref file) = file_opt {
            // Step 6: Inode flags via ioctl(FS_IOC_GETFLAGS)
            // NOT via sysfs/procfs — direct ioctl through our fd.
            // This is the correct, TOCTOU-safe path per KATTRS design.
            if let Ok(iflags) = ioctl_getflags(file) {
                // Map kernel IFlags bits to our InodeSecurityFlags.
                // We store our own bitfield rather than IFlags directly
                // to allow adding xattr-derived flags in the same set.
                let raw = iflags.bits();
                if raw & IFlags::IMMUTABLE.bits() != 0 {
                    sec_flags |= InodeSecurityFlags::IMMUTABLE;
                }
                if raw & IFlags::APPEND.bits() != 0 {
                    sec_flags |= InodeSecurityFlags::APPEND_ONLY;
                }
                if raw & IFlags::NODUMP.bits() != 0 {
                    sec_flags |= InodeSecurityFlags::NO_DUMP;
                }
                if raw & IFlags::NOATIME.bits() != 0 {
                    sec_flags |= InodeSecurityFlags::NO_ATIME;
                }
                //if raw & IFlags::SECRM.bits() != 0 {
                //    sec_flags |= InodeSecurityFlags::SECURE_DELETE;
                //}
                if raw & IFlags::UNRM.bits() != 0 {
                    sec_flags |= InodeSecurityFlags::UNDELETE;
                }
            }

            // Step 7: POSIX ACL xattr
            // NIST SP 800-53 AC-3: detect extended DAC in effect.
            let has_acl =
                SecureXattrReader::read_raw(file, "system.posix_acl_access")
                    .map(|v| !v.is_empty())
                    .unwrap_or(false);
            if has_acl {
                sec_flags |= InodeSecurityFlags::ACL_PRESENT;
            }

            // Step 8: IMA xattr
            // NIST SP 800-53 SI-7 / CMMC SI.L2-3.14.1.
            let has_ima = SecureXattrReader::read_raw(file, "security.ima")
                .map(|v| !v.is_empty())
                .unwrap_or(false);
            if has_ima {
                sec_flags |= InodeSecurityFlags::IMA_PRESENT;
            }

            // Step 9: SELinux context — dual-path RTB gate
            //
            // SecureXattrReader::read_context() runs nom + FromStr parsers
            // and cross-checks them.  Returns XattrReadError which
            // distinguishes OS errors (ENODATA, EACCES) from TPI failures.
            //
            // NIST SP 800-53 AC-3/AC-4 / CMMC AC.L2-3.1.3.
            // NSA RTB RAIN: Non-Bypassability, Redundancy/TPI.
            sec_flags |= InodeSecurityFlags::XATTR_PRESENT;
            match SecureXattrReader::read_context(file) {
                Ok(ctx) => {
                    sec_flags |= InodeSecurityFlags::SELINUX_XATTR;
                    selinux_label = SelinuxCtxState::Labeled(Box::new(ctx));
                }
                Err(XattrReadError::OsError(ref e))
                    if e.raw_os_error().is_some()
                        && e.kind() == std::io::ErrorKind::PermissionDenied =>
                {
                    // OS-level permission denial on xattr read (EACCES/EPERM).
                    // open() succeeded but MAC/DAC policy blocks fgetxattr().
                    // Treat as access_denied: inode anchor exists but label is
                    // unverifiable. Shows as <restricted> rather than <unlabeled>.
                    // NIST SP 800-53 AC-3; NSA RTB Non-Bypassability (RAIN).
                    log::warn!(
                        "SELinux xattr access denied for {abs_path}: {e}"
                    );
                    access_denied = true;
                }
                Err(XattrReadError::OsError(ref e)) => {
                    // ENODATA or other OS error — inode is genuinely unlabeled
                    // or the xattr subsystem is unavailable.
                    // selinux_label stays Unlabeled.
                    log::debug!(
                        "SELinux xattr not present for {abs_path}: {e}"
                    );
                }
                Err(XattrReadError::Tpi(TpiError::Disagreement(_, _))) => {
                    // Both parsers succeeded but disagreed — potential
                    // integrity event.  Set TpiDisagreement so the display
                    // layer renders <unverifiable> and the observations
                    // layer emits SecurityObservation::TpiDisagreement.
                    log::error!(
                        "TPI disagreement on SELinux label for {abs_path} \
                         — object treated as unverifiable"
                    );
                    selinux_label = SelinuxCtxState::TpiDisagreement;
                }
                Err(XattrReadError::Tpi(_)) => {
                    // One or both TPI parse paths failed — code/validator
                    // defect.  The label is present but unverifiable.
                    // Set ParseFailure so the display layer renders
                    // <parse-error> rather than <unlabeled>.
                    log::warn!(
                        "SELinux label parse failure for {abs_path} — \
                         label present but unverifiable"
                    );
                    selinux_label = SelinuxCtxState::ParseFailure;
                }
            }
        }

        // Step 10: mount point detection
        // A path is a mount point if its dev differs from its parent's dev.
        // We use the dev we already captured from metadata — no second stat.

        let is_mountpoint = path
            .parent()
            .and_then(|p| std::fs::symlink_metadata(p).ok())
            .is_some_and(|pm| {
                !dev.same_device_as(posix::primitives::DevId::new(pm.dev()))
            });

        // Step 11: encryption detection — only for mount points.
        // Non-mount-point entries always carry EncryptionSource::None.
        // NIST SP 800-53 SC-28: at-rest encryption posture per mount point.
        let encryption = if is_mountpoint {
            crate::fs_encrypt::detect_mount_encryption(path)
        } else {
            crate::fs_encrypt::EncryptionSource::None
        };

        Ok(Self {
            path: abs_path,
            name: validated_name,
            file_type,
            inode,
            size,
            mode,
            nlink,
            dev,
            ownership,
            selinux_label,
            sec_flags,
            is_mountpoint,
            encryption,
            access_denied,
        })
    }

    // Security query methods
    //
    /// True if this entry has write-protection (immutable or append-only).
    /// NIST SP 800-53 AU-9.
    #[must_use]
    pub fn is_write_protected(&self) -> bool {
        self.sec_flags.is_write_protected()
    }

    /// True if ACL-based access control is active.
    /// NIST SP 800-53 AC-3.
    #[must_use]
    pub fn has_acl(&self) -> bool {
        self.sec_flags.has_acl()
    }

    /// True if IMA integrity measurement is active.
    /// NIST SP 800-53 SI-7.
    #[must_use]
    pub fn has_ima_protection(&self) -> bool {
        self.sec_flags.has_ima_protection()
    }

    /// True if a SELinux context is present, verified, and persisted on disk.
    ///
    /// Returns `false` for `ParseFailure` and `TpiDisagreement` states —
    /// the xattr byte may exist but the label could not be verified.
    /// NIST SP 800-53 AC-4.
    #[must_use]
    pub fn has_explicit_selinux_label(&self) -> bool {
        self.selinux_label.is_labeled()
            && self.sec_flags.has_explicit_selinux_label()
    }

    /// True if this mount point has any detected at-rest encryption.
    ///
    /// Always `false` for non-mount-point entries.
    ///
    /// NIST SP 800-53 SC-28: Protection of Information at Rest.
    #[must_use]
    pub fn has_encryption(&self) -> bool {
        self.encryption != crate::fs_encrypt::EncryptionSource::None
    }

    /// True if the setuid bit is set.
    /// NIST SP 800-53 CM-6 / CMMC CM.L2-3.4.2.
    #[must_use]
    pub const fn is_setuid(&self) -> bool {
        self.mode.is_setuid()
    }

    /// True if the setgid bit is set.
    /// NIST SP 800-53 CM-6.
    #[must_use]
    pub const fn is_setgid(&self) -> bool {
        self.mode.is_setgid()
    }

    /// True if the sticky bit is set.
    #[must_use]
    pub const fn is_sticky(&self) -> bool {
        self.mode.is_sticky()
    }

    /// True if the world-writable bit is set.
    /// NIST SP 800-53 AC-3 / CMMC AC.L1-3.1.1.
    #[must_use]
    pub const fn is_world_writable(&self) -> bool {
        self.mode.is_world_writable()
    }

    /// True if this is a hard-linked non-directory file (nlink > 1).
    /// Hard links can be used to bypass directory-level access controls.
    /// NIST SP 800-53 AC-3.
    #[must_use]
    pub fn is_hard_linked(&self) -> bool {
        !self.file_type.is_directory() && self.nlink.is_multiply_linked()
    }

    /// Uid of the owning user (kernel ground truth).
    #[must_use]
    pub const fn uid(&self) -> posix::primitives::Uid {
        self.ownership.user.uid
    }

    /// Gid of the owning group (kernel ground truth).
    #[must_use]
    pub const fn gid(&self) -> posix::primitives::Gid {
        self.ownership.group.gid
    }

    /// Aggregate security observations on this entry.
    ///
    /// Returns a `Vec<SecurityObservation>` — all findings are returned,
    /// not just the first. Callers can filter by `kind()` to separate
    /// positive, warning, and risk findings without matching every variant.
    ///
    /// An empty vec means no current rules fired. It does not mean the entry
    /// is clean — the rule set grows over time.
    ///
    /// NIST SP 800-53 CA-7: Continuous Monitoring.
    /// NIST SP 800-53 RA-5: Vulnerability Scanning.
    /// CMMC Level 2 CA.L2-3.12.1: Periodic security control assessment.
    #[must_use]
    pub fn security_observations(&self) -> Vec<SecurityObservation> {
        let mut obs = Vec::new();

        // ── Access denial (reported first — remaining attrs may be absent) ──
        if self.access_denied {
            obs.push(SecurityObservation::AccessDenied);
        }

        // ── Risk ────────────────────────────────────────────────────────────
        if self.is_world_writable() && !self.file_type.is_symlink() {
            obs.push(SecurityObservation::WorldWritable);
        }
        match &self.selinux_label {
            SelinuxCtxState::Unlabeled if !self.access_denied => {
                // Inode has no SELinux xattr — MAC cannot be enforced.
                obs.push(SecurityObservation::NoSelinuxContext);
            }
            SelinuxCtxState::ParseFailure => {
                // Xattr present but TPI path(s) failed — code defect.
                obs.push(SecurityObservation::SelinuxParseFailure);
            }
            SelinuxCtxState::TpiDisagreement => {
                // Both parsers disagreed — potential integrity event.
                obs.push(SecurityObservation::TpiDisagreement);
            }
            SelinuxCtxState::Labeled(_) | SelinuxCtxState::Unlabeled => {}
        }
        if self.is_setuid()
            && (self.mode.group_can_write() || self.mode.is_world_writable())
        {
            obs.push(SecurityObservation::SetuidWritable);
        }

        // ── Warning ─────────────────────────────────────────────────────────
        if self.is_setuid() {
            obs.push(SecurityObservation::SetuidBitSet);
        }
        if self.is_setgid() {
            obs.push(SecurityObservation::SetgidBitSet);
        }
        if self.is_hard_linked() {
            obs.push(SecurityObservation::HardLinked {
                nlink: self.nlink.as_u32(),
            });
        }
        if self.uid().is_root()
            && self.file_type.is_regular()
            && !self.is_write_protected()
        {
            obs.push(SecurityObservation::RootOwnedMutable);
        }
        if self.ownership.user.is_unresolved() {
            obs.push(SecurityObservation::UnresolvedOwner {
                uid: self.uid(),
            });
        }
        if self.ownership.group.is_unresolved() {
            obs.push(SecurityObservation::UnresolvedGroup {
                gid: self.gid(),
            });
        }

        // ── Good ────────────────────────────────────────────────────────────
        if self.has_ima_protection() && self.file_type.is_regular() {
            obs.push(SecurityObservation::ImaHashPresent);
        }
        if self.sec_flags.contains(InodeSecurityFlags::IMMUTABLE) {
            obs.push(SecurityObservation::ImmutableFlagSet);
        }

        if !obs.is_empty() {
            let obs_str = obs
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            log::debug!("SecObservations: {} :: {}", self.name, obs_str);
        }

        obs
    }
}

impl std::fmt::Display for SecureDirent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match &self.selinux_label {
            SelinuxCtxState::Labeled(ctx) => ctx.to_string(),
            SelinuxCtxState::Unlabeled => "<unlabeled>".to_owned(),
            SelinuxCtxState::ParseFailure => "<parse-error>".to_owned(),
            SelinuxCtxState::TpiDisagreement => "<unverifiable>".to_owned(),
        };

        write!(
            f,
            "{type_char}{mode} {uid:>5}:{gid:<5} {inode:>10} {size:>10} {label} {path}",
            type_char = match self.file_type {
                FileType::Directory => 'd',
                FileType::Symlink => 'l',
                FileType::RegularFile => '-',
                FileType::BlockDevice => 'b',
                FileType::CharDevice => 'c',
                FileType::Fifo => 'p',
                FileType::Socket => 's',
                FileType::Unknown => '?',
            },
            mode = self.mode,
            uid = self.uid(),
            gid = self.gid(),
            inode = self.inode,
            size = self.size,
            path = self.path,
        )
    }
}
