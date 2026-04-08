// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::unreadable_literal)]

//! # umrs-stat — File Security Audit Card Library
//!
//! Library crate backing the `umrs-stat` binary and the `umrs-ls` stat popup
//! overlay.  Exposes [`FileStatApp`] and its row-builder helpers so callers
//! can construct a security audit view for any file without running a full TUI.
//!
//! ## Key Exported Types
//!
//! - [`FileStatApp`] — pre-built row data for Identity, Security, Observations tabs
//! - [`FsInfo`] — filesystem metadata extracted from `/proc/mounts`
//! - [`ElfInfo`] — ELF binary class and type from the file header
//! - [`format_size`] — human-readable file size string
//! - [`find_fs_info`] — longest-prefix mount-point lookup via `ProcfsText`
//! - [`read_elf_info`] — ELF header reader (display-only, not trust-relevant)
//! - [`build_identity_rows`], [`build_security_rows`], [`build_observation_rows`] —
//!   row builders for each tab
//! - [`build_status`] — derive a [`StatusMessage`] from the observation list
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access Enforcement — MAC label and mode displayed.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — every card is
//!   self-identifying (report name + subject).
//! - **NIST SP 800-53 CA-7**: Continuous monitoring — observations surfaced as
//!   typed data, not free-form text.
//! - **NIST SP 800-53 SC-28**: Protection at rest — encryption source display.
//! - **NIST SP 800-53 SI-7**: Integrity — SELinux TPI state is displayed;
//!   `/proc/mounts` is read via provenance-verified `ProcfsText`.

use std::path::{Path, PathBuf};

use umrs_core::i18n;
use umrs_platform::kattrs::{ProcfsText, SecureReader};
use umrs_selinux::fs_encrypt::EncryptionSource;
use umrs_selinux::mcs::translator::{GLOBAL_TRANSLATOR, SecurityRange};
use umrs_selinux::posix::primitives::FileSize;
use umrs_selinux::secure_dirent::{InodeSecurityFlags, SecDirError, SecureDirent};
use umrs_selinux::{ObservationKind, SecurityObservation, SelinuxCtxState};
use umrs_ui::app::{DataRow, StatusLevel, StatusMessage, StyleHint, TabDef};

// ---------------------------------------------------------------------------
// Size formatting helper
// ---------------------------------------------------------------------------

/// Format a `FileSize` as "N bytes (X.Y KB/MB/GB)".
///
/// The raw byte count is always shown; the human-readable suffix is appended
/// when size is >= 1024 bytes.
#[must_use = "formatted size string is the only output; discarding it means the size is never shown"]
pub fn format_size(size: FileSize) -> String {
    let bytes = size.as_u64();
    if bytes < 1024 {
        return format!("{bytes} bytes");
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "display hint only; precision loss on large byte counts is acceptable"
    )]
    let kb_frac = bytes as f64 / 1024.0;
    if kb_frac < 1024.0 {
        return format!("{bytes} bytes ({kb_frac:.1} KB)");
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "display hint only; precision loss on large byte counts is acceptable"
    )]
    let mb_frac = bytes as f64 / (1024.0 * 1024.0);
    if mb_frac < 1024.0 {
        return format!("{bytes} bytes ({mb_frac:.1} MB)");
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "display hint only; precision loss on large byte counts is acceptable"
    )]
    let gb_frac = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{bytes} bytes ({gb_frac:.2} GB)")
}

// ---------------------------------------------------------------------------
// Status bar builder
// ---------------------------------------------------------------------------

/// Derive a [`StatusMessage`] from the observation list.
///
/// Priority: any Risk finding overrides warnings; warnings override clean.
///
/// NIST SP 800-53 AU-3 — the status bar is a single-line summary of the
/// finding count, providing rapid-scan context for the operator.
#[must_use = "StatusMessage must be stored or displayed; discarding it hides the security posture"]
pub fn build_status(observations: &[SecurityObservation]) -> StatusMessage {
    let risk_count = observations
        .iter()
        .filter(|o: &&SecurityObservation| o.kind() == ObservationKind::Risk)
        .count();
    let warn_count = observations
        .iter()
        .filter(|o: &&SecurityObservation| o.kind() == ObservationKind::Warning)
        .count();

    if risk_count > 0 {
        StatusMessage::new(
            StatusLevel::Error,
            format!("{risk_count} risk finding(s) detected"),
        )
    } else if warn_count > 0 {
        StatusMessage::new(
            StatusLevel::Warn,
            format!("{warn_count} warning(s), no risk findings"),
        )
    } else {
        StatusMessage::new(StatusLevel::Ok, i18n::tr("No security concerns"))
    }
}

// ---------------------------------------------------------------------------
// Filesystem info helper
// ---------------------------------------------------------------------------

/// Filesystem metadata extracted from `/proc/mounts` for a file's path.
///
/// NIST SP 800-53 SI-7 — filesystem origin is audit-relevant context for
/// understanding access controls and encryption state.
pub struct FsInfo {
    /// Device node path (e.g., `/dev/sda1`).
    pub device: String,
    /// Mount point (e.g., `/home`).
    pub mount_point: String,
    /// Filesystem type (e.g., `ext4`, `xfs`).
    pub fs_type: String,
}

/// Find the filesystem entry for `path` by walking `/proc/mounts` and
/// selecting the longest-prefix mount point match.
///
/// Uses provenance-verified `ProcfsText` + `SecureReader` to satisfy
/// NIST SP 800-53 SI-7 (integrity) — raw `File::open` on `/proc/` is
/// prohibited by project policy.
///
/// Returns `None` when `/proc/mounts` is unreadable or no prefix matches.
#[must_use = "FsInfo is the only output; discarding it means filesystem context is not displayed"]
pub fn find_fs_info(path: &Path) -> Option<FsInfo> {
    let node = ProcfsText::new(PathBuf::from("/proc/mounts")).ok()?;
    let contents = SecureReader::<ProcfsText>::new().read_generic_text(&node).ok()?;

    let path_str = path.to_str()?;
    let mut best: Option<FsInfo> = None;
    let mut best_len: usize = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let Some(device) = parts.next() else {
            continue;
        };
        let Some(mp) = parts.next() else {
            continue;
        };
        let Some(fs_type) = parts.next() else {
            continue;
        };

        if path_str.starts_with(mp) && mp.len() > best_len {
            best_len = mp.len();
            best = Some(FsInfo {
                device: device.to_owned(),
                mount_point: mp.to_owned(),
                fs_type: fs_type.to_owned(),
            });
        }
    }

    best
}

// ---------------------------------------------------------------------------
// ELF info helper
// ---------------------------------------------------------------------------

/// ELF binary metadata extracted from the file header.
///
/// Display-only — not a trust-relevant assertion.  The caller must not use
/// this in any policy decision.
pub struct ElfInfo {
    /// ELF class string: `"ELF64"` or `"ELF32"`.
    pub class: &'static str,
    /// ELF type string: `"Executable"`, `"Shared object (DSO/PIE)"`, etc.
    pub elf_type: &'static str,
}

/// Read the first 20 bytes of `path` and extract ELF class and type.
///
/// Returns `None` if the file is not ELF or cannot be read.
///
/// This opens its own fd for MIME detection (display-only, not a trust-relevant
/// assertion; not part of any policy decision).
///
/// DIRECT-IO-EXCEPTION: ELF magic byte read is display-only (binary type hint),
/// never influences a security decision, and no umrs-platform abstraction exists
/// for this purpose.
#[must_use = "ElfInfo is the only output; discarding it means binary type context is not displayed"]
pub fn read_elf_info(path: &Path) -> Option<ElfInfo> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = [0u8; 20];
    f.read_exact(&mut buf).ok()?;

    if buf[0..4] != [0x7f, b'E', b'L', b'F'] {
        return None;
    }

    let class = match buf[4] {
        1 => "ELF32",
        2 => "ELF64",
        _ => "ELF (unknown class)",
    };

    let e_type = u16::from_le_bytes([buf[16], buf[17]]);
    let elf_type = match e_type {
        1 => "Relocatable",
        2 => "Executable",
        3 => "Shared object (DSO/PIE)",
        4 => "Core dump",
        _ => "Unknown",
    };

    Some(ElfInfo { class, elf_type })
}

// ---------------------------------------------------------------------------
// Row builders
// ---------------------------------------------------------------------------

/// Build the Identity tab rows for a successfully-read `SecureDirent`.
///
/// Row order: path/filename/type, optional symlink target, MIME type,
/// optional ELF info, size/mode/inode/device, hard links, separator,
/// owner/group, separator, mount point, filesystem info.
///
/// NIST SP 800-53 AU-3 — every row carries a labelled field; no anonymous blobs.
/// NIST SP 800-53 SI-7 — filesystem info read via provenance-verified ProcfsText.
#[must_use = "identity rows are the only output; discarding them means the Identity tab is empty"]
pub fn build_identity_rows(dirent: &SecureDirent, mime: &str, path: &Path) -> Vec<DataRow> {
    let mut rows = Vec::new();

    rows.push(DataRow::normal(i18n::tr("Path"), dirent.path.to_string()));
    rows.push(DataRow::normal(
        i18n::tr("Filename"),
        dirent.name.to_string(),
    ));
    rows.push(DataRow::new(
        i18n::tr("File type"),
        dirent.file_type.to_string(),
        StyleHint::Highlight,
    ));

    if dirent.file_type.is_symlink() {
        let target = match std::fs::read_link(path) {
            Ok(p) => p.to_string_lossy().into_owned(),
            Err(_) => "(unreadable)".to_owned(),
        };
        rows.push(DataRow::new(
            "Link target",
            format!("--> {target}"),
            StyleHint::Highlight,
        ));
    }

    rows.push(DataRow::new(
        i18n::tr("MIME type"),
        mime.to_owned(),
        StyleHint::Highlight,
    ));

    if dirent.file_type.is_regular()
        && let Some(elf) = read_elf_info(path)
    {
        rows.push(DataRow::new(
            "Binary format",
            elf.class,
            StyleHint::Highlight,
        ));
        rows.push(DataRow::new(
            "Binary type",
            elf.elf_type,
            StyleHint::Highlight,
        ));
    }

    rows.push(DataRow::normal(i18n::tr("Size"), format_size(dirent.size)));
    rows.push(DataRow::normal(i18n::tr("Mode"), dirent.mode.as_mode_str()));
    rows.push(DataRow::normal("Inode", dirent.inode.to_string()));
    rows.push(DataRow::normal("Device", dirent.dev.to_string()));

    let nlink_val: u32 = dirent.nlink.into();
    let (nlink_str, nlink_hint) = if nlink_val > 1 && !dirent.file_type.is_directory() {
        (format!("{nlink_val} (hard-linked)"), StyleHint::TrustYellow)
    } else {
        (dirent.nlink.to_string(), StyleHint::Normal)
    };
    rows.push(DataRow::new("Hard links", nlink_str, nlink_hint));

    rows.push(DataRow::separator());

    let (owner_str, owner_hint) = match &dirent.ownership.user.name {
        Some(name) => (
            format!("{}:{}", dirent.ownership.user.uid, name.as_str()),
            StyleHint::Normal,
        ),
        None => (
            format!("{}:(unresolved)", dirent.ownership.user.uid),
            StyleHint::TrustYellow,
        ),
    };
    rows.push(DataRow::new("Owner", owner_str, owner_hint));

    let (group_str, group_hint) = match &dirent.ownership.group.name {
        Some(name) => (
            format!("{}:{}", dirent.ownership.group.gid, name.as_str()),
            StyleHint::Normal,
        ),
        None => (
            format!("{}:(unresolved)", dirent.ownership.group.gid),
            StyleHint::TrustYellow,
        ),
    };
    rows.push(DataRow::new("Group", group_str, group_hint));

    rows.push(DataRow::separator());

    let mp_str = if dirent.is_mountpoint { "yes" } else { "no" };
    rows.push(DataRow::normal("Mount point", mp_str));

    if let Some(fs) = find_fs_info(path) {
        rows.push(DataRow::normal("Filesystem", fs.fs_type));
        rows.push(DataRow::normal("Device node", fs.device));
        rows.push(DataRow::normal("Mounted on", fs.mount_point));
    }

    rows
}

/// Build inode security flag rows.
///
/// Each flag is surfaced individually with a human-readable label.
///
/// NIST SP 800-53 SI-7, AU-9, AC-3.
#[must_use = "inode flag rows are the only output; discarding them hides security flag state"]
pub fn build_inode_flag_rows(dirent: &SecureDirent) -> Vec<DataRow> {
    let mut rows = Vec::new();

    let (immut_str, immut_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::IMMUTABLE) {
        ("Yes", StyleHint::TrustGreen)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new(i18n::tr("Immutable"), immut_str, immut_hint));

    let (ima_str, ima_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::IMA_PRESENT) {
        ("Yes — integrity hash present", StyleHint::TrustGreen)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new(i18n::tr("IMA/EVM"), ima_str, ima_hint));

    let (append_str, append_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::APPEND_ONLY) {
        ("Yes", StyleHint::TrustGreen)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new("Append-only", append_str, append_hint));

    let (acl_str, acl_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::ACL_PRESENT) {
        ("Yes — extended DAC in effect", StyleHint::TrustYellow)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new("POSIX ACL", acl_str, acl_hint));

    let (access_str, access_hint) = if dirent.access_denied {
        ("yes", StyleHint::TrustYellow)
    } else {
        ("no", StyleHint::Normal)
    };
    rows.push(DataRow::new("Access denied", access_str, access_hint));

    rows
}

/// Build the Security tab rows for a successfully-read `SecureDirent`.
///
/// Row order: SELinux label summary, per-component fields (when labeled),
/// raw label, label state, separator, security marking (MCS translation),
/// separator, inode flag rows, separator, encryption source.
///
/// NIST SP 800-53 AC-3, AC-4, SI-7, SC-28.
#[must_use = "security rows are the only output; discarding them hides the Security tab content"]
pub fn build_security_rows(dirent: &SecureDirent) -> Vec<DataRow> {
    let mut rows = Vec::new();

    let label_hint = match &dirent.selinux_label {
        SelinuxCtxState::Labeled(_) => StyleHint::TrustGreen,
        SelinuxCtxState::Unlabeled
        | SelinuxCtxState::ParseFailure
        | SelinuxCtxState::TpiDisagreement => StyleHint::TrustRed,
    };
    rows.push(DataRow::new(
        i18n::tr("SELinux Context"),
        " ",
        label_hint,
    ));

    if let SelinuxCtxState::Labeled(ctx) = &dirent.selinux_label {
        rows.push(DataRow::normal("  SELinux user", ctx.user().to_string()));
        rows.push(DataRow::normal("  SELinux role", ctx.role().to_string()));
        rows.push(DataRow::normal(
            "  SELinux type",
            ctx.security_type().to_string(),
        ));
        let level_str = ctx.level().map_or_else(|| "(none)".to_owned(), |l| l.raw().to_owned());
        rows.push(DataRow::normal("  Raw label", level_str));

        let (state_str, state_hint) = match &dirent.selinux_label {
            SelinuxCtxState::Labeled(_) => ("Labeled", StyleHint::TrustGreen),
            SelinuxCtxState::Unlabeled => ("Unlabeled", StyleHint::TrustRed),
            SelinuxCtxState::ParseFailure => ("ParseFailure", StyleHint::TrustRed),
            SelinuxCtxState::TpiDisagreement => ("TpiDisagreement", StyleHint::TrustRed),
        };
        rows.push(DataRow::new(
            i18n::tr("  Label state"),
            state_str,
            state_hint,
        ));

        rows.push(DataRow::separator());

        if let Some(lvl) = ctx.level() {
            let range = SecurityRange::from_level(lvl);
            let marking = GLOBAL_TRANSLATOR.read().map_or_else(
                |_| lvl.raw().to_owned(),
                |g| g.lookup(&range).unwrap_or_else(|| lvl.raw().to_owned()),
            );
            rows.push(DataRow::new(
                i18n::tr("Marking"),
                marking,
                StyleHint::Highlight,
            ));
        }
    }

    rows.push(DataRow::separator());
    rows.extend(build_inode_flag_rows(dirent));
    rows.push(DataRow::separator());

    let (enc_str, enc_hint) = match &dirent.encryption {
        EncryptionSource::None => ("None".to_owned(), StyleHint::Normal),
        EncryptionSource::LuksDevice => ("LUKS (dm-crypt)".to_owned(), StyleHint::TrustGreen),
        EncryptionSource::EncryptedFilesystem(fs) => (
            format!("Encrypted filesystem ({fs})"),
            StyleHint::TrustGreen,
        ),
    };
    rows.push(DataRow::new(i18n::tr("Encryption"), enc_str, enc_hint));

    rows
}

/// Build the Observations tab rows from a slice of `SecurityObservation`s.
///
/// Each observation is rendered with polarity-driven color coding:
/// Risk → red, Warning → yellow, Good → green.
///
/// NIST SP 800-53 CA-7 — observations surface findings as typed data.
/// NIST SP 800-53 AU-3 — indexed findings provide traceable records.
#[must_use = "observation rows are the only output; discarding them hides the Observations tab"]
pub fn build_observation_rows(observations: &[SecurityObservation]) -> Vec<DataRow> {
    let mut rows = Vec::new();

    rows.push(DataRow::normal(
        i18n::tr("Count"),
        observations.len().to_string(),
    ));
    rows.push(DataRow::separator());

    if observations.is_empty() {
        rows.push(DataRow::new(
            i18n::tr("Findings"),
            i18n::tr("No security observations"),
            StyleHint::TrustGreen,
        ));
    } else {
        for (i, obs) in observations.iter().enumerate() {
            let idx = i.saturating_add(1);
            let hint = match obs.kind() {
                ObservationKind::Good => StyleHint::TrustGreen,
                ObservationKind::Warning => StyleHint::TrustYellow,
                ObservationKind::Risk => StyleHint::TrustRed,
            };
            rows.push(DataRow::new(format!("[{idx}]"), obs.to_string(), hint));
        }
    }

    rows
}

// ---------------------------------------------------------------------------
// Error path row builders
// ---------------------------------------------------------------------------

/// Build Identity tab rows for the error path (file could not be read).
#[must_use = "error rows are the only output; discarding them means the Identity tab shows nothing"]
pub fn build_identity_error_rows() -> Vec<DataRow> {
    vec![DataRow::new(
        "Status",
        i18n::tr("Failed to read file"),
        StyleHint::TrustRed,
    )]
}

/// Build Security tab rows for the error path.
#[must_use = "error rows are the only output; discarding them means the Security tab shows nothing"]
pub fn build_security_error_rows(err: &SecDirError) -> Vec<DataRow> {
    vec![
        DataRow::new("Status", i18n::tr("No data available"), StyleHint::TrustRed),
        DataRow::new("Error", err.to_string(), StyleHint::TrustRed),
    ]
}

/// Build Observations tab rows for the error path.
#[must_use = "error rows are the only output; discarding them means the Observations tab is empty"]
pub fn build_observation_error_rows() -> Vec<DataRow> {
    vec![DataRow::new(
        "Status",
        "Cannot analyze — read failed",
        StyleHint::TrustRed,
    )]
}

// ---------------------------------------------------------------------------
// FileStatApp
// ---------------------------------------------------------------------------

/// Audit card data source backed by `SecureDirent` for a single file.
///
/// Constructed once via [`FileStatApp::from_dirent`] or
/// [`FileStatApp::from_error`]; the `SecureDirent` is consumed at build time
/// and its data stored in pre-built row vectors. The render path is pure
/// read — no I/O on any draw call.
///
/// Public row fields allow callers (e.g., the `umrs-ls` stat popup) to render
/// individual tabs without invoking the `AuditCardApp` trait or taking a
/// `&dyn AuditCardApp` reference.
///
/// NIST SP 800-53 AC-3, AU-3, CA-7, SC-28, SI-7.
pub struct FileStatApp {
    /// Tab definitions for the three-tab layout.
    pub tabs: Vec<TabDef>,
    /// Pre-built Identity tab rows.
    pub identity_rows: Vec<DataRow>,
    /// Pre-built Security tab rows.
    pub security_rows: Vec<DataRow>,
    /// Pre-built Observations tab rows.
    pub observation_rows: Vec<DataRow>,
    /// Status bar message derived from the observation list.
    pub status: StatusMessage,
    /// Subject path used as the report subject in the standalone TUI.
    ///
    /// Stored as `String`; the `AuditCardApp` trait impl leaks this to a
    /// `&'static str` only in the standalone binary, where the allocation
    /// lifetime is the process lifetime.
    pub report_subject: String,
}

impl FileStatApp {
    /// Build the app from a successfully-read `SecureDirent`.
    ///
    /// The `mime` string is the MIME type detected by the caller (typically
    /// `tree_magic_mini::from_filepath`).  It is stored verbatim in the
    /// Identity tab rows — display-only, not a trust-relevant assertion.
    ///
    /// NIST SP 800-53 AU-3 — file path and MIME type are stored as labelled
    /// fields so the operator has full context for the file's identity.
    #[must_use = "FileStatApp must be stored and displayed; discarding it wastes the I/O work"]
    pub fn from_dirent(dirent: &SecureDirent, mime: &str) -> Self {
        let observations = dirent.security_observations();
        let status = build_status(&observations);

        let tabs = vec![
            TabDef::new(i18n::tr("Identity")),
            TabDef::new(i18n::tr("Security")),
            TabDef::new(i18n::tr("Observations")),
        ];

        let identity_rows = build_identity_rows(dirent, mime, Path::new(&*dirent.path));
        let security_rows = build_security_rows(dirent);
        let observation_rows = build_observation_rows(&observations);

        Self {
            tabs,
            identity_rows,
            security_rows,
            observation_rows,
            status,
            report_subject: dirent.path.to_string(),
        }
    }

    /// Build the app from a `SecDirError` (file could not be read).
    ///
    /// All three tab row vectors are populated with error-state rows.
    ///
    /// NIST SP 800-53 AU-3 — the path is preserved even on error so the
    /// audit card is always self-identifying.
    #[must_use = "FileStatApp must be stored and displayed; discarding it wastes the I/O work"]
    pub fn from_error(path_str: &str, err: &SecDirError) -> Self {
        let tabs = vec![
            TabDef::new(i18n::tr("Identity")),
            TabDef::new(i18n::tr("Security")),
            TabDef::new(i18n::tr("Observations")),
        ];

        Self {
            tabs,
            identity_rows: build_identity_error_rows(),
            security_rows: build_security_error_rows(err),
            observation_rows: build_observation_error_rows(),
            status: StatusMessage::new(
                StatusLevel::Error,
                i18n::tr("Failed to read file attributes"),
            ),
            report_subject: path_str.to_owned(),
        }
    }

    /// Return the rows for the given tab index.
    ///
    /// Index 0 = Identity, 1 = Security, 2 = Observations.
    /// Out-of-range indices return a single "invalid tab" row rather than
    /// panicking — fail-closed per NSA RTB.
    ///
    /// NIST SP 800-53 SI-10 — input bounds are checked; the caller cannot
    /// trigger a panic via an out-of-range tab index.
    #[must_use = "data rows must be rendered; discarding them means the tab shows nothing"]
    pub fn rows_for_tab(&self, tab_index: usize) -> &[DataRow] {
        match tab_index {
            0 => &self.identity_rows,
            1 => &self.security_rows,
            2 => &self.observation_rows,
            _ => &[],
        }
    }
}
