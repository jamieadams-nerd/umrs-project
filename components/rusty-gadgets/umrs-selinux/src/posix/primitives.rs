// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
//! # POSIX / Linux Filesystem Primitives
//!
//! Strongly-typed newtypes for Linux kernel filesystem identifiers and
//! metadata values. These types replace bare `u32`/`u64` integers
//! wherever kernel-derived filesystem data is handled.
//!
//! ## Why typed primitives?
//!
//! `inode`, `dev`, `mode`, and `nlink` are all integers at the C layer.
//! They are semantically completely distinct — passing a `DevId` where
//! an `Inode` is expected is a logic error the compiler cannot catch if
//! both are `u64`. These newtypes make such errors compile-time failures.
//!
//! Additionally, security-relevant query methods belong on the types
//! themselves (`FileMode::is_setuid()`) rather than scattered as bare
//! integer comparisons (`mode & 0o4000 != 0`) throughout calling code.
//! This consolidates security logic, makes it testable in isolation, and
//! gives auditors a single place to verify correctness.
//!
//! ## What is validated?
//!
//! These values arrive from the kernel via `MetadataExt` — the kernel
//! guarantees their numeric validity. What we validate is:
//!
//! - **Semantic constraints**: `FileMode` masks to the lower 16 bits only
//!   (the kernel's `st_mode` field); upper bits are not mode bits and
//!   must not be stored or compared as if they are.
//! - **Type identity**: once wrapped, `Inode(42)` cannot be compared to
//!   `DevId(42)` — different types, different semantics.
//! - **Security query correctness**: `is_setuid()`, `is_world_writable()`
//!   etc. live here, tested once, correct everywhere.
//!
//! ## Compliance References
//!
//! - **NIST SP 800-53 Rev 5 — SA-15**: Development Process, Standards,
//!   and Tools — strong typing enforces correctness at the language level,
//!   reducing the class of defects reachable at runtime.
//! - **NIST SP 800-53 Rev 5 — CM-6**: Configuration Settings — mode bit
//!   queries (`is_setuid`, `is_world_writable`) support baseline checking.
//! - **NIST SP 800-53 Rev 5 — AC-3**: Access Enforcement — `FileMode`
//!   encodes the DAC permission bits consulted during access decisions.
//! - **NIST SP 800-53 Rev 5 — AU-3**: Audit Record Content — inode, dev,
//!   mode, nlink are all required fields in a complete audit record.
//! - **CMMC Level 2 — CM.L2-3.4.2**: Establish and enforce security
//!   configuration settings — setuid/setgid/world-writable detection.
//! - **NSA RTB — Minimized TCB**: security-relevant logic is concentrated
//!   in typed methods, not distributed as magic constants.
//! - **NSA RTB — Determinism**: fixed-width integer types with explicit
//!   masking give deterministic behaviour across all kernel versions.

use std::fmt;

// ────────────────────────────────────────────────────────────────────────────
// Inode
// ────────────────────────────────────────────────────────────────────────────

/// A Linux inode number (`st_ino` from `stat(2)`).
///
/// Inode numbers uniquely identify a filesystem object within a single
/// device. They are **not** globally unique — the same number can appear
/// on different devices. Always pair with `DevId` for global uniqueness.
///
/// Inode 0 is reserved by POSIX and should not appear on a mounted
/// filesystem; we accept it here because the kernel occasionally returns
/// it for pseudo-filesystems and synthetic entries.
///
/// NIST SP 800-53 AU-3: inode number is a required audit record field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inode(u64);

impl Inode {
    /// Wrap a raw kernel inode number.
    /// No validation beyond type wrapping — kernel-supplied values are trusted.
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Return the raw numeric value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Inode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Inode {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<Inode> for u64 {
    fn from(i: Inode) -> Self {
        i.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// DevId
// ────────────────────────────────────────────────────────────────────────────

/// A Linux device ID (`st_dev` from `stat(2)`).
///
/// Encodes the device (block device or pseudo-filesystem) on which a
/// filesystem object resides. Used for mount-point detection: if a
/// directory entry's `dev` differs from its parent's `dev`, the entry
/// is a mount point.
///
/// The value is opaque — its internal encoding (`major:minor`) is
/// kernel-version and architecture-dependent. Do not decompose it;
/// use it only for equality comparison.
///
/// NIST SP 800-53 AU-3 / CM-6: device identity is part of a complete
/// filesystem audit record and is required for mount-point detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DevId(u64);

impl DevId {
    /// Wrap a raw kernel device ID.
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Return the raw numeric value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// True if this device ID equals another — i.e., both objects are
    /// on the same device. Used for mount-point detection.
    ///
    /// NIST SP 800-53 CM-6: mount point detection is a configuration finding.
    #[must_use]
    pub const fn same_device_as(self, other: Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for DevId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dev:{:#x}", self.0)
    }
}

impl From<u64> for DevId {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<DevId> for u64 {
    fn from(d: DevId) -> Self {
        d.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// HardLinkCount
// ────────────────────────────────────────────────────────────────────────────

/// The hard link count for a filesystem object (`st_nlink` from `stat(2)`).
///
/// For regular files: `nlink == 1` means exactly one directory entry
/// points to this inode. `nlink > 1` means the inode is shared — there
/// are multiple directory entries pointing to the same data. This is a
/// security-relevant finding on CUI systems: hard links can be used to
/// access files outside their expected directory tree, bypassing
/// directory-level access controls and quota enforcement.
///
/// For directories: `nlink` is the number of hard links including `.`
/// and `..` entries. Values > 2 are normal and indicate subdirectories.
/// Directory hard-link findings are suppressed in `SecureDirent`.
///
/// NIST SP 800-53 AC-3: hard links may bypass directory-level controls.
/// NIST SP 800-53 AU-3: nlink is part of a complete audit record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardLinkCount(u32);

impl HardLinkCount {
    /// Wrap a raw kernel nlink value.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Convenience: construct from a `u64` kernel value (nlink is u64
    /// in `MetadataExt` on Linux despite fitting in u32 in practice).
    #[must_use]
    pub fn from_u64(raw: u64) -> Self {
        // Saturating cast: .min() guarantees the value fits in u32; the cast
        // is therefore infallible but clippy can't see through the clamp.
        #[allow(clippy::cast_possible_truncation)]
        Self(raw.min(u64::from(u32::MAX)) as u32)
    }

    /// Return the raw count.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// True if this is a hard-linked non-directory inode.
    ///
    /// Callers are responsible for checking `FileType` — this method
    /// does not know whether the inode is a directory. For directories,
    /// `nlink > 1` is normal and should not be flagged.
    ///
    /// NIST SP 800-53 AC-3: hard-linked files may bypass path-based controls.
    #[must_use]
    pub const fn is_multiply_linked(self) -> bool {
        self.0 > 1
    }
}

impl fmt::Display for HardLinkCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for HardLinkCount {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<u64> for HardLinkCount {
    fn from(v: u64) -> Self {
        Self::from_u64(v)
    }
}

impl From<HardLinkCount> for u32 {
    fn from(h: HardLinkCount) -> Self {
        h.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// FileMode
// ────────────────────────────────────────────────────────────────────────────

/// POSIX file mode bits (`st_mode` lower 16 bits from `stat(2)`).
///
/// Encodes three categories of information in the lower 16 bits of `st_mode`:
///
/// ```text
/// Bits 15-12: file type  (not stored here — use FileType for that)
/// Bits 11-9:  special    (setuid=11, setgid=10, sticky=9)
/// Bits 8-6:   owner rwx
/// Bits 5-3:   group rwx
/// Bits 2-0:   other rwx
/// ```
///
/// Construction masks to the lower 12 bits (permissions + special bits).
/// The upper 4 bits (file type) are intentionally excluded — `FileType`
/// is the correct type for that information.
///
/// ## Security-relevant bit queries
///
/// All mode bit checks are methods on this type. No bare `& 0o4000`
/// comparisons should appear elsewhere in the codebase.
///
/// ## Display
///
/// `fmt::Display` produces the 9-character `ls`-style string with correct
/// setuid (`s`/`S`), setgid (`s`/`S`), and sticky (`t`/`T`) encoding.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: DAC permission bits — owner/group/other rwx.
/// - **NIST SP 800-53 CM-6**: setuid, setgid, sticky, world-writable are
///   configuration baseline items.
/// - **CMMC Level 2 — CM.L2-3.4.2**: security configuration enforcement.
/// - **NSA RTB**: concentrated, tested mode-bit logic; no magic constants
///   scattered through calling code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileMode(u32);

impl FileMode {
    // ── Special bits ─────────────────────────────────────────────────────
    /// Setuid bit — process runs with file owner's uid.
    /// NIST SP 800-53 CM-6 / CMMC CM.L2-3.4.2.
    pub const SETUID: u32 = 0o4000;
    /// Setgid bit — process runs with file group's gid,
    /// or on directories: new files inherit the directory's group.
    /// NIST SP 800-53 CM-6.
    pub const SETGID: u32 = 0o2000;
    /// Sticky bit — on directories: only owner/root can delete/rename files.
    pub const STICKY: u32 = 0o1000;

    // ── Owner bits ────────────────────────────────────────────────────────
    pub const OWNER_READ: u32 = 0o400;
    pub const OWNER_WRITE: u32 = 0o200;
    pub const OWNER_EXEC: u32 = 0o100;

    // ── Group bits ────────────────────────────────────────────────────────
    pub const GROUP_READ: u32 = 0o040;
    pub const GROUP_WRITE: u32 = 0o020;
    pub const GROUP_EXEC: u32 = 0o010;

    // ── Other bits ────────────────────────────────────────────────────────
    /// World-readable.
    pub const OTHER_READ: u32 = 0o004;
    /// World-writable — always a security finding on CUI systems.
    /// NIST SP 800-53 AC-3 / CMMC AC.L1-3.1.1.
    pub const OTHER_WRITE: u32 = 0o002;
    /// World-executable.
    pub const OTHER_EXEC: u32 = 0o001;

    /// Mask covering all 12 permission + special bits.
    pub const PERM_MASK: u32 = 0o7777;

    // ── Constructor ───────────────────────────────────────────────────────

    /// Construct from a raw `st_mode` value.
    ///
    /// Masks to the lower 12 bits — file type bits (upper 4) are
    /// intentionally discarded. Use `FileType::from_mode()` for those.
    ///
    /// This is the single construction gate. After this, the stored value
    /// is guaranteed to contain only permission and special bits.
    #[must_use]
    pub const fn from_mode(raw: u32) -> Self {
        Self(raw & Self::PERM_MASK)
    }

    /// Return the raw 12-bit permission value.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    // ── Special bit queries ───────────────────────────────────────────────

    /// True if the setuid bit is set.
    ///
    /// On an executable: the process runs with the file owner's effective uid.
    /// This is a privilege escalation risk and a mandatory audit finding.
    ///
    /// NIST SP 800-53 CM-6 / CMMC CM.L2-3.4.2.
    #[must_use]
    pub const fn is_setuid(self) -> bool {
        self.0 & Self::SETUID != 0
    }

    /// True if the setgid bit is set.
    ///
    /// NIST SP 800-53 CM-6.
    #[must_use]
    pub const fn is_setgid(self) -> bool {
        self.0 & Self::SETGID != 0
    }

    /// True if the sticky bit is set.
    #[must_use]
    pub const fn is_sticky(self) -> bool {
        self.0 & Self::STICKY != 0
    }

    // ── Other-permission queries ──────────────────────────────────────────

    /// True if the world-writable bit is set.
    ///
    /// Any process on the system can modify this file. This is always a
    /// security finding on a CUI system and should be reported unconditionally.
    ///
    /// NIST SP 800-53 AC-3 / CMMC AC.L1-3.1.1.
    #[must_use]
    pub const fn is_world_writable(self) -> bool {
        self.0 & Self::OTHER_WRITE != 0
    }

    /// True if the world-readable bit is set.
    #[must_use]
    pub const fn is_world_readable(self) -> bool {
        self.0 & Self::OTHER_READ != 0
    }

    /// True if the world-executable bit is set.
    #[must_use]
    pub const fn is_world_executable(self) -> bool {
        self.0 & Self::OTHER_EXEC != 0
    }

    // ── Owner queries ─────────────────────────────────────────────────────

    #[must_use]
    pub const fn owner_can_read(self) -> bool {
        self.0 & Self::OWNER_READ != 0
    }
    #[must_use]
    pub const fn owner_can_write(self) -> bool {
        self.0 & Self::OWNER_WRITE != 0
    }
    #[must_use]
    pub const fn owner_can_execute(self) -> bool {
        self.0 & Self::OWNER_EXEC != 0
    }

    // ── Group queries ─────────────────────────────────────────────────────

    #[must_use]
    pub const fn group_can_read(self) -> bool {
        self.0 & Self::GROUP_READ != 0
    }
    #[must_use]
    pub const fn group_can_write(self) -> bool {
        self.0 & Self::GROUP_WRITE != 0
    }
    #[must_use]
    pub const fn group_can_execute(self) -> bool {
        self.0 & Self::GROUP_EXEC != 0
    }

    // ── Composite security queries ────────────────────────────────────────

    /// True if any execute bit (owner, group, or other) is set.
    /// Used to determine whether setuid/setgid is active or dormant (`S`).
    #[must_use]
    pub const fn is_executable(self) -> bool {
        self.0 & (Self::OWNER_EXEC | Self::GROUP_EXEC | Self::OTHER_EXEC) != 0
    }

    /// True if this mode represents a file that should be audited for
    /// privilege risk: setuid or setgid on an executable.
    ///
    /// NIST SP 800-53 CM-6 / CMMC CM.L2-3.4.2.
    #[must_use]
    pub const fn has_privilege_bits(self) -> bool {
        self.is_setuid() || self.is_setgid()
    }

    // ── ls-style formatted string ─────────────────────────────────────────

    /// Format as 9-character `ls`-style permission string.
    ///
    /// Correctly encodes:
    /// - Setuid: owner execute position → `s` (execute set) or `S` (no execute)
    /// - Setgid: group execute position → `s` (execute set) or `S` (no execute)
    /// - Sticky: other execute position → `t` (execute set) or `T` (no execute)
    ///
    /// NIST SP 800-53 AU-3: mode string is a required audit record field.
    #[must_use]
    pub fn as_mode_str(self) -> String {
        let mut s = String::with_capacity(9);

        let ur = self.owner_can_read();
        let uw = self.owner_can_write();
        let ux = self.owner_can_execute();
        let su = self.is_setuid();

        let gr = self.group_can_read();
        let gw = self.group_can_write();
        let gx = self.group_can_execute();
        let sg = self.is_setgid();

        let or_ = self.is_world_readable();
        let ow = self.is_world_writable();
        let ox = self.is_world_executable();
        let st = self.is_sticky();

        s.push(if ur {
            'r'
        } else {
            '-'
        });
        s.push(if uw {
            'w'
        } else {
            '-'
        });
        s.push(match (ux, su) {
            (true, true) => 's',
            (false, true) => 'S',
            (true, false) => 'x',
            (false, false) => '-',
        });

        s.push(if gr {
            'r'
        } else {
            '-'
        });
        s.push(if gw {
            'w'
        } else {
            '-'
        });
        s.push(match (gx, sg) {
            (true, true) => 's',
            (false, true) => 'S',
            (true, false) => 'x',
            (false, false) => '-',
        });

        s.push(if or_ {
            'r'
        } else {
            '-'
        });
        s.push(if ow {
            'w'
        } else {
            '-'
        });
        s.push(match (ox, st) {
            (true, true) => 't',
            (false, true) => 'T',
            (true, false) => 'x',
            (false, false) => '-',
        });

        s
    }
}

/// Display produces the 9-character mode string — same as `as_mode_str()`.
impl fmt::Display for FileMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_mode_str())
    }
}

impl From<u32> for FileMode {
    /// Construct from a raw `st_mode` value, masking to lower 12 bits.
    fn from(v: u32) -> Self {
        Self::from_mode(v)
    }
}

impl From<FileMode> for u32 {
    fn from(m: FileMode) -> Self {
        m.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// FileSize
// ────────────────────────────────────────────────────────────────────────────

/// File size in bytes (`st_size` from `stat(2)`).
///
/// Typed separately from `Inode` and `DevId` to prevent accidental
/// interchange. No validation beyond wrapping — kernel-supplied values
/// are trusted. Size is zero for directories, devices, and symlinks by
/// convention.
///
/// NIST SP 800-53 AU-3: file size is part of a complete audit record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileSize(u64);

impl FileSize {
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// True if the file has no content. Meaningful only for regular files.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for FileSize {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<FileSize> for u64 {
    fn from(s: FileSize) -> Self {
        s.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Uid
// ────────────────────────────────────────────────────────────────────────────

/// A Linux numeric user ID (`st_uid` from `stat(2)`).
///
/// The uid is the kernel-authoritative identity for a file owner or
/// process. Name resolution via `/etc/passwd` or NSS is a separate,
/// fallible operation — the uid is ground truth.
///
/// Uid(0) is root. There is no invalid uid value at the kernel level;
/// all `u32` values are accepted.
///
/// Typed separately from `Gid` so that passing a gid where a uid is
/// expected is a compile-time error.
///
/// NIST SP 800-53 AC-2: uid is the authoritative account identifier.
/// NIST SP 800-53 AC-3: access decisions are made against uid/gid.
/// CMMC Level 2 — AC.L2-3.1.1: uid governs DAC enforcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uid(u32);

impl Uid {
    /// Wrap a raw kernel uid value.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Return the raw numeric value.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// True if this is the root uid (0).
    ///
    /// Root-owned files that are mutable are a persistent escalation risk.
    /// NIST SP 800-53 CM-5 / CMMC CM.L2-3.4.5.
    #[must_use]
    pub const fn is_root(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Uid {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<Uid> for u32 {
    fn from(u: Uid) -> Self {
        u.0
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Gid
// ────────────────────────────────────────────────────────────────────────────

/// A Linux numeric group ID (`st_gid` from `stat(2)`).
///
/// Same rationale as `Uid` — gid is the kernel-authoritative group
/// identity. Typed separately from `Uid` so that the two cannot be
/// accidentally interchanged.
///
/// NIST SP 800-53 AC-2 / AC-3 / CMMC AC.L2-3.1.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gid(u32);

impl Gid {
    /// Wrap a raw kernel gid value.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Return the raw numeric value.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// True if this is the root group (gid 0).
    #[must_use]
    pub const fn is_root(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Gid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Gid {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<Gid> for u32 {
    fn from(g: Gid) -> Self {
        g.0
    }
}
