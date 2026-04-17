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
//! - [`compute_file_digests`] — SHA-256 and SHA-384 digest display helper
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
//!   `/proc/mounts` is read via provenance-verified `ProcfsText`;
//!   SHA-256 and SHA-384 digest rows (FIPS 180-4) provide tamper-evidence display.

use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256, Sha384};
use umrs_core::i18n;
use umrs_platform::kattrs::{ProcfsText, SecureReader};
use umrs_selinux::fs_encrypt::EncryptionSource;
use umrs_selinux::mcs::translator::{GLOBAL_TRANSLATOR, SecurityRange};
use umrs_selinux::posix::primitives::FileSize;
use umrs_selinux::secure_dirent::{InodeSecurityFlags, SecDirError, SecureDirent};
use umrs_selinux::{ObservationKind, SecurityObservation, SelinuxCtxState};
use umrs_ui::app::{DataRow, StatusLevel, StatusMessage, StyleHint, TabDef};
use umrs_ui::icons::{EM_DASH, ICON_MOUNT, ICON_WARNING};
use umrs_ui::popup::PopupCardData;

// ---------------------------------------------------------------------------
// Size formatting helper
// ---------------------------------------------------------------------------

/// Format a `FileSize` as a human-readable string.
///
/// - Below 1024 bytes: `"N bytes"`
/// - 1 KB – 999 KB: `"X.Y KB"`
/// - 1 MB – 999 MB: `"X.Y MB"`
/// - 1 GB and above: `"X.YZ GB"`
///
/// Raw byte counts are omitted above 1 KB — they add noise at larger scales
/// without aiding operator comprehension.
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
        return format!("{kb_frac:.1} KB");
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "display hint only; precision loss on large byte counts is acceptable"
    )]
    let mb_frac = bytes as f64 / (1024.0 * 1024.0);
    if mb_frac < 1024.0 {
        return format!("{mb_frac:.1} MB");
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "display hint only; precision loss on large byte counts is acceptable"
    )]
    let gb_frac = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{gb_frac:.2} GB")
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
/// TOCTOU: the file may have been replaced since SecureDirent construction;
/// the ELF bytes are display hints, not assertions.
#[must_use = "ElfInfo is the only output; discarding it means binary type context is not displayed"]
pub fn read_elf_info(path: &Path) -> Option<ElfInfo> {
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

    Some(ElfInfo {
        class,
        elf_type,
    })
}

// ---------------------------------------------------------------------------
// File digest helper
// ---------------------------------------------------------------------------

/// Compute SHA-256 and SHA-384 digests of a regular file in a single pass.
///
/// Returns `None` if the file cannot be read (access denied, disappeared, etc.).
/// Reads the file in 8 KiB chunks to bound memory usage on arbitrarily large files.
/// Both hashers are updated on each chunk so the file is read exactly once.
///
/// The returned strings are lowercase hex-encoded digests.
///
/// ## DIRECT-IO-EXCEPTION
///
/// Opens the file with `std::fs::File::open` rather than routing through
/// a provenance-verified abstraction.  This is permissible because:
/// 1. No `umrs-platform` or `umrs-selinux` abstraction exists for raw file
///    content hashing.
/// 2. The digest is display-only — it is never compared against a stored
///    reference value, never used in a trust decision, and never influences
///    any policy gate.
/// 3. TOCTOU: the file may have changed since `SecureDirent` construction;
///    the displayed digest reflects the file at the moment of the read, which
///    is the correct semantics for a live "what is this file right now" view.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: Software and Information Integrity — displayed
///   digests give operators tamper-evidence for the file's content.
/// - **FIPS 180-4**: SHA-256 and SHA-384 are both FIPS-approved hash algorithms.
#[must_use = "digest strings are the only output; discarding them means the hash rows are never shown"]
pub fn compute_file_digests(path: &Path) -> Option<(String, String)> {
    // DIRECT-IO-EXCEPTION: display-only digest, no trust decision.
    // See doc comment above for full rationale.
    let file = std::fs::File::open(path).ok()?;
    let mut reader = BufReader::new(file);

    let mut sha256 = Sha256::new();
    let mut sha384 = Sha384::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = reader.read(&mut buf).ok()?;
        if n == 0 {
            break;
        }
        sha256.update(&buf[..n]);
        sha384.update(&buf[..n]);
    }

    let digest256 = sha256.finalize();
    let digest384 = sha384.finalize();

    Some((format!("{digest256:x}"), format!("{digest384:x}")))
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

    let dir_path = Path::new(&*dirent.path).parent().map_or_else(
        || "/".to_owned(),
        |p| {
            let s = p.to_string_lossy().into_owned();
            if s.ends_with('/') {
                s
            } else {
                format!("{s}/")
            }
        },
    );
    rows.push(DataRow::normal(i18n::tr("Path"), dir_path));
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
        // DIRECT-IO-EXCEPTION: display-only symlink target resolution.
        // No trust decision is derived from this value. TOCTOU: the link
        // target may have changed since SecureDirent construction.
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

    if dirent.file_type.is_regular()
        && let Some((sha256, sha384)) = compute_file_digests(path)
    {
        rows.push(DataRow::separator());
        rows.push(DataRow::separator());
        rows.push(DataRow::group_title("Computed Hashes"));
        rows.push(DataRow::separator());
        rows.push(DataRow::normal(" SHA-256:", ""));
        rows.push(DataRow::normal("", format!("  {sha256}")));
        rows.push(DataRow::separator());
        rows.push(DataRow::normal(" SHA-384:", ""));
        rows.push(DataRow::normal("", format!("  {sha384}")));
    }

    rows.push(DataRow::separator());

    // ── Storage Location ─────────────────────────────────────────────────────
    rows.push(DataRow::group_title("Storage Location"));

    let fs_info = find_fs_info(path);

    if let Some(ref fs) = fs_info {
        let mounted_on_str = if dirent.is_mountpoint {
            format!("{ICON_MOUNT} {} (mount point)", fs.mount_point)
        } else {
            fs.mount_point.clone()
        };
        rows.push(DataRow::normal(" Mounted on", mounted_on_str));
        rows.push(DataRow::normal(" Filesystem type", fs.fs_type.clone()));
    }

    rows.push(DataRow::normal(" Inode", dirent.inode.to_string()));

    let nlink_val: u32 = dirent.nlink.into();
    let (nlink_str, nlink_hint) = if nlink_val > 1 && !dirent.file_type.is_directory() {
        (format!("{nlink_val} (hard-linked)"), StyleHint::TrustYellow)
    } else {
        (dirent.nlink.to_string(), StyleHint::Normal)
    };
    rows.push(DataRow::new(" Hard links", nlink_str, nlink_hint));

    if let Some(fs) = fs_info {
        rows.push(DataRow::normal(" Device node", fs.device));
        rows.push(DataRow::normal(" Device ID", dirent.dev.to_string()));
    }

    rows
}

/// Build the Security tab rows for a successfully-read `SecureDirent`.
///
/// Layout:
/// 1. Marking (top-level, always visible)
/// 2. Encryption + Access denied
/// 3. Mandatory Access Controls (SELinux) — user/role/type/label
/// 4. Integrity Controls — IMA/EVM
/// 5. Discretionary Access Controls — Owner/Group/Mode/Immutable/Append-only/ACL
///
/// NIST SP 800-53 AC-3, AC-4, SI-7, SC-28.
#[must_use = "security rows are the only output; discarding them hides the Security tab content"]
#[expect(
    clippy::too_many_lines,
    reason = "five security sections form a single logical unit; splitting would scatter the tab layout across multiple functions"
)]
pub fn build_security_rows(dirent: &SecureDirent) -> Vec<DataRow> {
    let mut rows = Vec::new();

    // ── General Security ──────────────────────────────────────────────────────
    rows.push(DataRow::GroupTitle("General Security".to_owned()));

    if let SelinuxCtxState::Labeled(ctx) = &dirent.selinux_label
        && let Some(lvl) = ctx.level()
    {
        let range = SecurityRange::from_level(lvl);
        let marking = GLOBAL_TRANSLATOR.read().map_or_else(
            |_| lvl.raw().to_owned(),
            |g| g.lookup(&range).unwrap_or_else(|| lvl.raw().to_owned()),
        );
        rows.push(DataRow::new(
            format!(" {}", i18n::tr("Marking")),
            marking,
            StyleHint::Highlight,
        ));
    }
    rows.push(DataRow::separator());

    // ── Encryption & access ──────────────────────────────────────────────────
    // Determine whether this file carries MCS categories (CUI or Protected
    // marking). Files with categories are expected to be encrypted at rest
    // per NIST SP 800-53 SC-28 and CMMC SC.L2-3.13.16.
    let has_marking = matches!(
        &dirent.selinux_label,
        SelinuxCtxState::Labeled(ctx) if ctx.level().is_some_and(|lvl| !lvl.categories.is_empty())
    );
    let (enc_str, enc_hint) = match &dirent.encryption {
        EncryptionSource::None if has_marking => (
            format!("{ICON_WARNING} None {EM_DASH} CUI/protected asset on unencrypted storage"),
            StyleHint::TrustYellow,
        ),
        EncryptionSource::None => ("None".to_owned(), StyleHint::Normal),
        EncryptionSource::LuksDevice => ("LUKS (dm-crypt)".to_owned(), StyleHint::TrustGreen),
        EncryptionSource::EncryptedFilesystem(fs) => (
            format!("Encrypted filesystem ({fs})"),
            StyleHint::TrustGreen,
        ),
    };
    rows.push(DataRow::new(
        format!(" {}", i18n::tr("Encryption")),
        enc_str,
        enc_hint,
    ));

    let (access_str, access_hint) = if dirent.access_denied {
        ("yes", StyleHint::TrustYellow)
    } else {
        ("no", StyleHint::Normal)
    };
    rows.push(DataRow::new(" Access denied", access_str, access_hint));
    rows.push(DataRow::separator());

    // ── Mandatory Access Controls (SELinux) ──────────────────────────────────
    rows.push(DataRow::GroupTitle(
        "Mandatory Access Controls (SELinux)".to_owned(),
    ));

    if let SelinuxCtxState::Labeled(ctx) = &dirent.selinux_label {
        let (user_str, user_hint) = {
            let u = ctx.user().to_string();
            if u == "unconfined_u" {
                (format!("{ICON_WARNING} {u}"), StyleHint::TrustYellow)
            } else {
                (u, StyleHint::Normal)
            }
        };
        rows.push(DataRow::new(" SELinux user", user_str, user_hint));
        rows.push(DataRow::normal(" SELinux role", ctx.role().to_string()));
        rows.push(DataRow::normal(
            " SELinux type",
            ctx.security_type().to_string(),
        ));
        let level_str = ctx.level().map_or_else(|| "(none)".to_owned(), |l| l.raw().to_owned());
        rows.push(DataRow::normal(" Raw label", level_str));

        let (state_str, state_hint) = match &dirent.selinux_label {
            SelinuxCtxState::Labeled(_) => ("Labeled", StyleHint::TrustGreen),
            SelinuxCtxState::Unlabeled => ("Unlabeled", StyleHint::TrustRed),
            SelinuxCtxState::ParseFailure => ("ParseFailure", StyleHint::TrustRed),
            SelinuxCtxState::TpiDisagreement => ("TpiDisagreement", StyleHint::TrustRed),
        };
        rows.push(DataRow::new(" Label state", state_str, state_hint));
    }
    rows.push(DataRow::separator());

    // ── Integrity Controls ───────────────────────────────────────────────────
    rows.push(DataRow::GroupTitle("Integrity Controls".to_owned()));

    let (ima_str, ima_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::IMA_PRESENT) {
        ("Yes — integrity hash present", StyleHint::TrustGreen)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new(
        format!(" {}", i18n::tr("IMA/EVM")),
        ima_str,
        ima_hint,
    ));
    rows.push(DataRow::separator());

    // ── Discretionary Access Controls ────────────────────────────────────────
    rows.push(DataRow::GroupTitle(
        "Discretionary Access Controls".to_owned(),
    ));

    let (owner_str, owner_hint) = match &dirent.ownership.user.name {
        Some(name) => (
            format!("{} ({})", name.as_str(), dirent.ownership.user.uid),
            StyleHint::Normal,
        ),
        None => (
            format!("{} (unresolved)", dirent.ownership.user.uid),
            StyleHint::TrustYellow,
        ),
    };
    rows.push(DataRow::new(" Owner", owner_str, owner_hint));

    let (group_str, group_hint) = match &dirent.ownership.group.name {
        Some(name) => (
            format!("{} ({})", name.as_str(), dirent.ownership.group.gid),
            StyleHint::Normal,
        ),
        None => (
            format!("{} (unresolved)", dirent.ownership.group.gid),
            StyleHint::TrustYellow,
        ),
    };
    rows.push(DataRow::new(" Group", group_str, group_hint));

    rows.push(DataRow::normal(" Mode", dirent.mode.as_mode_str()));

    let (immut_str, immut_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::IMMUTABLE) {
        ("Yes", StyleHint::TrustGreen)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new(" Immutable", immut_str, immut_hint));

    let (append_str, append_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::APPEND_ONLY) {
        ("Yes", StyleHint::TrustGreen)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new(" Append-only", append_str, append_hint));

    let (acl_str, acl_hint) = if dirent.sec_flags.contains(InodeSecurityFlags::ACL_PRESENT) {
        ("Yes — extended DAC in effect", StyleHint::TrustYellow)
    } else {
        ("No", StyleHint::Normal)
    };
    rows.push(DataRow::new(" POSIX ACL", acl_str, acl_hint));

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
            format!("\u{2705} {}", i18n::tr("No security observations")),
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
    /// Translated security marking for the file (e.g., `"SystemLow"`,
    /// `"CUI//LEI"`).  `None` when the file has no MCS/MLS level.
    ///
    /// Displayed in the upper-right corner of the popup tab bar so the
    /// operator always sees the marking context.
    ///
    /// NIST SP 800-53 AC-16 — security attribute visibility.
    pub marking: Option<String>,
    /// Index group for the marking (e.g., `"Critical Infrastructure"`).
    /// Used to select the palette color in the popup tab bar.
    /// Set by callers with catalog access (umrs-ls); `None` in standalone mode.
    ///
    /// NIST SP 800-53 AC-16 — visual consistency across tools.
    pub marking_index_group: Option<String>,
    /// Number of security observations (Risk + Warning + Good).
    /// Used to show a flag icon on the Observations tab when > 0.
    ///
    /// NIST SP 800-53 CA-7 — finding count drives visual indicator.
    pub observation_count: usize,
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

        // Extract the translated marking for the popup header display.
        let marking = if let SelinuxCtxState::Labeled(ctx) = &dirent.selinux_label
            && let Some(lvl) = ctx.level()
        {
            let range = SecurityRange::from_level(lvl);
            Some(GLOBAL_TRANSLATOR.read().map_or_else(
                |_| lvl.raw().to_owned(),
                |g| g.lookup(&range).unwrap_or_else(|| lvl.raw().to_owned()),
            ))
        } else {
            None
        };

        Self {
            tabs,
            identity_rows,
            security_rows,
            observation_rows,
            status,
            report_subject: dirent.path.to_string(),
            marking,
            marking_index_group: None,
            observation_count: observations.len(),
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
            marking: None,
            marking_index_group: None,
            observation_count: 0,
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

// ---------------------------------------------------------------------------
// PopupCardData implementation
// ---------------------------------------------------------------------------

/// Static tab name slice for the three-tab layout.
///
/// These are the English-language display names used by the popup tab bar.
/// They match the tab order in [`FileStatApp::rows_for_tab`]: Identity (0),
/// Security (1), Observations (2).
static FILE_STAT_TAB_NAMES: &[&str] = &["Identity", "Security", "Observations"];

/// Implement [`PopupCardData`] for [`FileStatApp`] so that
/// [`umrs_ui::popup::render_audit_card_popup`] can render a file audit card
/// popup without a direct dependency on this crate's concrete type.
///
/// NIST SP 800-53 AU-3 — tabbed data layout ensures complete file identity
/// and security metadata is always accessible without information truncation.
/// NIST SP 800-53 SI-10 — `rows_for_tab` delegates to the existing
/// bounds-checked implementation; out-of-range indices return an empty slice.
impl PopupCardData for FileStatApp {
    fn tab_names(&self) -> &[&'static str] {
        FILE_STAT_TAB_NAMES
    }

    fn rows_for_tab(&self, tab: usize) -> &[DataRow] {
        // Delegates to the existing bounds-checked implementation so the
        // bounds logic is never duplicated.
        Self::rows_for_tab(self, tab)
    }

    fn marking(&self) -> Option<&str> {
        self.marking.as_deref()
    }

    fn marking_index_group(&self) -> Option<&str> {
        self.marking_index_group.as_deref()
    }

    fn observation_count(&self) -> usize {
        self.observation_count
    }
}
