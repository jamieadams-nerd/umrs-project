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
//! ## Compliance
//!
//! - **NIST SP 800-53 CA-7**: Continuous monitoring.
//! - **NIST SP 800-53 RA-5**: Vulnerability scanning.
//! - **CMMC Level 2 CA.L2-3.12.1**: Periodic security control assessment.

use std::fmt;

use crate::posix::primitives::{Gid, Uid};

// ===========================================================================
// ObservationKind

/// Polarity of a [`SecurityObservation`].
///
/// Use this to filter, score, or display observations without pattern-matching
/// every variant. Callers that check `kind() == ObservationKind::Risk` remain
/// correct as new variants are added.
///
/// ## Variants:
///
/// - `Good` — positive security signal; the object is doing something right.
/// - `Warning` — condition warrants attention but may be intentional or acceptable.
/// - `Risk` — clear negative finding with direct security relevance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationKind {
    Good,
    Warning,
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
/// ## Variants:
///
/// *Risk*
///
/// - `WorldWritable` — world-writable file or directory; any user can modify. Does not fire on
///   symbolic links (world-writable mode is expected on symlinks). NIST SP 800-53 AC-3 /
///   CMMC AC.L1-3.1.1.
/// - `NoSelinuxContext` — no SELinux context available; file is unlabeled. On an MLS system this
///   is a critical finding: MAC cannot be enforced on an unlabeled object. NIST SP 800-53 AC-4 /
///   CMMC AC.L2-3.1.3.
/// - `SetuidWritable` — setuid binary is also group- or world-writable; direct privilege
///   escalation vector. Owner-write on a setuid file is accepted; group- or world-write is not.
///   NIST SP 800-53 CM-6 / CMMC CM.L2-3.4.2.
/// - `TpiDisagreement` — both TPI parse paths succeeded but produced structurally different
///   security contexts. This is a potential integrity event: an adversary manipulating the xattr
///   byte stream at the kernel interface could produce this outcome. Reserve for the TPI
///   Disagreement case only; do not emit for single-path failures (those produce
///   `SelinuxParseFailure`). NIST SP 800-53 SI-7 / NSA RTB RAIN.
///
/// *Warning*
///
/// - `SetuidBitSet` — setuid bit set; privilege escalation risk if exploited. Common on system
///   binaries (`sudo`, `passwd`); warrants audit coverage. NIST SP 800-53 CM-6 /
///   CMMC CM.L2-3.4.2.
/// - `SetgidBitSet` — setgid bit set; group privilege escalation risk. NIST SP 800-53 CM-6.
/// - `HardLinked { nlink: u32 }` — file has multiple hard links; same inode reachable via
///   multiple paths. On SELinux systems the MAC label follows the inode, so policy is enforced on
///   all paths; the concern is audit trail complexity. Symbolic links are preferred in
///   high-assurance environments because they carry their own inode and SELinux controls
///   symlink-following permission separately. `nlink` is the hard link count from `stat`.
///   NIST SP 800-53 AC-3.
/// - `RootOwnedExcessiveWrite` — root-owned regular file with group-writable or world-writable
///   permissions, or with extended POSIX ACLs that may grant non-owner write access. Standard
///   `0755 root:root` posture does NOT fire this — only permission sets that allow non-root
///   modification of a root-owned file. NIST SP 800-53 CM-5 / CMMC CM.L2-3.4.5.
/// - `UnresolvedOwner { uid: Uid }` — uid has no entry in `/etc/passwd`; orphaned file owner.
///   Indicates a deleted account that still owns files on disk. Does not fire for uids that
///   resolve normally (including uid 0). `uid` is the numeric uid of the orphaned owner.
///   NIST SP 800-53 AC-2.
/// - `UnresolvedGroup { gid: Gid }` — gid has no entry in `/etc/group`; orphaned group
///   ownership. `gid` is the numeric gid of the orphaned group. NIST SP 800-53 AC-2.
/// - `AccessDenied` — access was denied when opening the entry for attribute reads. Security
///   attributes (SELinux context, IMA hash, inode flags) could not be fully collected. May
///   indicate a correctly tightened directory (e.g., mode 711) that intentionally restricts
///   inspection, or a MAC policy decision. Treated as Warning because restricted access is a
///   valid and desirable posture in tightened deployments. NIST SP 800-53 AU-3.
/// - `SelinuxParseFailure` — a SELinux xattr was present but one or both TPI parse paths
///   failed. The label is on the inode but could not be verified. This is a code/validator
///   defect — distinct from a genuinely unlabeled inode (`NoSelinuxContext`) and from a TPI
///   integrity disagreement (`TpiDisagreement`). Displayed as `<parse-error>` rather than
///   `<unlabeled>` to prevent false negatives in audit output. NIST SP 800-53 AU-3, SI-12.
///
/// *Good*
///
/// - `ImaHashPresent` — an IMA integrity hash (`security.ima` xattr) is present on this file;
///   the inode is under active integrity measurement or appraisal. NIST SP 800-53 SI-7 /
///   CMMC SI.L2-3.14.1.
/// - `ImmutableFlagSet` — the immutable inode flag (`FS_IMMUTABLE_FL`) is set; content cannot
///   be modified, renamed, or deleted until the flag is cleared, even by root.
///   NIST SP 800-53 AU-9, CM-5.
///
/// ## Compliance
///
/// - **NIST SP 800-53 CA-7**: Continuous monitoring.
/// - **NIST SP 800-53 RA-5**: Vulnerability scanning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityObservation {
    // ── Risk ──────────────────────────────────────────────────────────────────
    WorldWritable,
    NoSelinuxContext,
    SetuidWritable,

    // ── Warning ───────────────────────────────────────────────────────────────
    SetuidBitSet,
    SetgidBitSet,
    HardLinked {
        nlink: u32,
    },
    RootOwnedExcessiveWrite,
    UnresolvedOwner {
        uid: Uid,
    },
    UnresolvedGroup {
        gid: Gid,
    },
    AccessDenied,
    SelinuxParseFailure,

    // ── Good ──────────────────────────────────────────────────────────────────
    ImaHashPresent,
    ImmutableFlagSet,

    // ── Risk (integrity events) ────────────────────────────────────────────────
    TpiDisagreement,
    // ── Add new observations above this line ──────────────────────────────────
}

impl SecurityObservation {
    /// Returns the polarity of this observation.
    ///
    /// Use this to filter or score without pattern-matching every variant.
    /// New variants added to this enum must also be added here.
    #[must_use = "pure accessor returning the polarity of this finding; callers that discard it cannot filter or score observations"]
    pub const fn kind(&self) -> ObservationKind {
        match self {
            Self::WorldWritable
            | Self::NoSelinuxContext
            | Self::SetuidWritable
            | Self::TpiDisagreement => ObservationKind::Risk,
            Self::SetuidBitSet
            | Self::SetgidBitSet
            | Self::HardLinked {
                ..
            }
            | Self::RootOwnedExcessiveWrite
            | Self::UnresolvedOwner {
                ..
            }
            | Self::UnresolvedGroup {
                ..
            }
            | Self::AccessDenied
            | Self::SelinuxParseFailure => ObservationKind::Warning,
            Self::ImaHashPresent | Self::ImmutableFlagSet => ObservationKind::Good,
        }
    }
}

impl fmt::Display for SecurityObservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.kind() {
            ObservationKind::Good => "\u{2713}",    // ✓
            ObservationKind::Warning => "\u{26A0}", // ⚠
            ObservationKind::Risk => "\u{2717}",    // ✗
        };
        let msg = match self {
            Self::WorldWritable => "world-writable".to_owned(),
            Self::NoSelinuxContext => "no SELinux context (unlabeled)".to_owned(),
            Self::SetuidWritable => "setuid with group/world write permissions".to_owned(),
            Self::SetuidBitSet => "SETUID bit set".to_owned(),
            Self::SetgidBitSet => "SETGID bit set".to_owned(),
            Self::HardLinked {
                nlink,
            } => format!("hard-linked (nlink={nlink})"),
            Self::RootOwnedExcessiveWrite => {
                "root-owned file with non-owner write access".to_owned()
            }
            Self::UnresolvedOwner {
                uid,
            } => format!("orphaned owner (uid={uid})"),
            Self::UnresolvedGroup {
                gid,
            } => format!("orphaned group (gid={gid})"),
            Self::AccessDenied => "access denied during attribute read".to_owned(),
            Self::SelinuxParseFailure => {
                "SELinux label present but unverifiable (parse failure)".to_owned()
            }
            Self::TpiDisagreement => "SELinux TPI integrity violation: parsers disagree".to_owned(),
            Self::ImaHashPresent => "IMA integrity hash present".to_owned(),
            Self::ImmutableFlagSet => "immutable flag set".to_owned(),
        };
        write!(f, "{symbol} {msg}")
    }
}
