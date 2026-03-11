// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
//! # Security Observations
//!
//! `SecurityObservation` is the vocabulary of per-object security findings
//! emitted by [`crate::secure_dirent::SecureDirent::security_observations`].
//!
//! Observations are data, not log messages. They are queryable, sortable,
//! and typed — a caller can filter by kind, count by category, or drive
//! automated remediation without parsing strings.
//!
//! Each observation carries an [`ObservationKind`] that indicates whether
//! the finding is a positive security signal, a condition requiring attention,
//! or a clear risk finding.
//!
//! ## Adding new observations
//!
//! 1. Add the variant in the appropriate section (Good / Warning / Risk).
//! 2. Add its `kind()` match arm.
//! 3. Add its `Display` match arm with the descriptive message.
//! 4. Add detection logic in `SecureDirent::security_observations()`.
//!
//! NIST SP 800-53 CA-7: Continuous monitoring.
//! NIST SP 800-53 RA-5: Vulnerability scanning.
//! CMMC Level 2 CA.L2-3.12.1: Periodic security control assessment.

use std::fmt;

use crate::posix::primitives::{Gid, Uid};

// ===========================================================================
// ObservationKind

/// Polarity of a [`SecurityObservation`].
///
/// Use this to filter, score, or display observations without pattern-matching
/// every variant. Callers that check `kind() == ObservationKind::Risk` remain
/// correct as new variants are added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationKind {
    /// Positive security signal — the object is doing something right.
    Good,
    /// Condition warrants attention but may be intentional or acceptable.
    Warning,
    /// Clear negative finding with direct security relevance.
    Risk,
}

// ===========================================================================
// SecurityObservation

/// A security-relevant finding on a single directory entry.
///
/// These are not errors — the file was read successfully. They are
/// findings a security auditor would care about, grouped by polarity.
///
/// Use [`SecurityObservation::kind`] to query polarity without matching
/// every variant — this keeps calling code stable as new observations
/// are added.
///
/// NIST SP 800-53 CA-7: Continuous monitoring.
/// NIST SP 800-53 RA-5: Vulnerability scanning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityObservation {
    // ── Risk ──────────────────────────────────────────────────────────────────

    /// World-writable file or directory — any user can modify.
    /// Does not fire on symbolic links (world-writable mode is expected on symlinks).
    /// NIST SP 800-53 AC-3 / CMMC AC.L1-3.1.1.
    WorldWritable,

    /// No SELinux context available — file is unlabeled.
    /// On an MLS system this is a critical finding: MAC cannot be enforced
    /// on an unlabeled object.
    /// NIST SP 800-53 AC-4 / CMMC AC.L2-3.1.3.
    NoSelinuxContext,

    /// Setuid binary is also group- or world-writable — direct privilege
    /// escalation vector. Owner-write on a setuid file is accepted;
    /// group- or world-write is not.
    /// NIST SP 800-53 CM-6 / CMMC CM.L2-3.4.2.
    SetuidWritable,

    // ── Warning ───────────────────────────────────────────────────────────────

    /// Setuid bit set — privilege escalation risk if exploited.
    /// Common on system binaries (`sudo`, `passwd`); warrants audit coverage.
    /// NIST SP 800-53 CM-6 / CMMC CM.L2-3.4.2.
    SetuidBitSet,

    /// Setgid bit set — group privilege escalation risk.
    /// NIST SP 800-53 CM-6.
    SetgidBitSet,

    /// File has multiple hard links — same inode reachable via multiple paths.
    /// On SELinux systems the MAC label follows the inode, so policy is
    /// enforced on all paths; the concern is audit trail complexity.
    /// Symbolic links are preferred in high-assurance environments because
    /// they carry their own inode and SELinux controls symlink-following
    /// permission separately.
    /// NIST SP 800-53 AC-3.
    HardLinked {
        /// Hard link count from `stat`.
        nlink: u32,
    },

    /// Root-owned regular file that is not write-protected.
    /// A mutable root-owned file is a persistent escalation risk.
    /// NIST SP 800-53 CM-5 / CMMC CM.L2-3.4.5.
    RootOwnedMutable,

    /// Uid has no entry in `/etc/passwd` — orphaned file owner.
    /// Indicates a deleted account that still owns files on disk.
    /// Does not fire for uids that resolve normally (including uid 0).
    /// NIST SP 800-53 AC-2: Account Management.
    UnresolvedOwner {
        /// Numeric uid of the orphaned owner.
        uid: Uid,
    },

    /// Gid has no entry in `/etc/group` — orphaned group ownership.
    /// NIST SP 800-53 AC-2: Account Management.
    UnresolvedGroup {
        /// Numeric gid of the orphaned group.
        gid: Gid,
    },

    /// Access was denied when opening the entry for attribute reads.
    /// Security attributes (SELinux context, IMA hash, inode flags) could
    /// not be fully collected. This may indicate a correctly tightened
    /// directory (e.g., mode 711) that intentionally restricts inspection,
    /// or a MAC policy decision. Treated as Warning because restricted
    /// access is a valid and desirable posture in tightened deployments.
    /// NIST SP 800-53 AU-3: incomplete audit record indicator.
    AccessDenied,

    // ── Good ──────────────────────────────────────────────────────────────────

    /// An IMA integrity hash (`security.ima` xattr) is present on this file.
    /// The inode is under active integrity measurement or appraisal.
    /// NIST SP 800-53 SI-7 / CMMC SI.L2-3.14.1.
    ImaHashPresent,

    /// The immutable inode flag (`FS_IMMUTABLE_FL`) is set.
    /// Content cannot be modified, renamed, or deleted until the flag is
    /// cleared — even by root.
    /// NIST SP 800-53 AU-9, CM-5.
    ImmutableFlagSet,

    // ── Add new observations above this line ──────────────────────────────────
}

impl SecurityObservation {
    /// Returns the polarity of this observation.
    ///
    /// Use this to filter or score without pattern-matching every variant.
    /// New variants added to this enum must also be added here.
    #[must_use]
    pub const fn kind(&self) -> ObservationKind {
        match self {
            Self::WorldWritable | Self::NoSelinuxContext | Self::SetuidWritable => {
                ObservationKind::Risk
            }
            Self::SetuidBitSet
            | Self::SetgidBitSet
            | Self::HardLinked { .. }
            | Self::RootOwnedMutable
            | Self::UnresolvedOwner { .. }
            | Self::UnresolvedGroup { .. }
            | Self::AccessDenied => ObservationKind::Warning,
            Self::ImaHashPresent | Self::ImmutableFlagSet => ObservationKind::Good,
        }
    }
}

impl fmt::Display for SecurityObservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.kind() {
            ObservationKind::Good => "\u{2713}",    // ✓
            ObservationKind::Warning => "\u{26A0}",  // ⚠
            ObservationKind::Risk => "\u{2717}",     // ✗
        };
        let msg = match self {
            Self::WorldWritable => "world-writable".to_owned(),
            Self::NoSelinuxContext => "no SELinux context (unlabeled)".to_owned(),
            Self::SetuidWritable => "setuid with group/world write permissions".to_owned(),
            Self::SetuidBitSet => "SETUID bit set".to_owned(),
            Self::SetgidBitSet => "SETGID bit set".to_owned(),
            Self::HardLinked { nlink } => format!("hard-linked (nlink={nlink})"),
            Self::RootOwnedMutable => "root-owned mutable file".to_owned(),
            Self::UnresolvedOwner { uid } => format!("orphaned owner (uid={uid})"),
            Self::UnresolvedGroup { gid } => format!("orphaned group (gid={gid})"),
            Self::AccessDenied => "access denied during attribute read".to_owned(),
            Self::ImaHashPresent => "IMA integrity hash present".to_owned(),
            Self::ImmutableFlagSet => "immutable flag set".to_owned(),
        };
        write!(f, "{symbol} {msg}")
    }
}
