// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
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

//! # umrs-file-stat — File Security Audit Card
//!
//! Reads a file's security attributes via [`SecureDirent::from_path`] and
//! displays them as an interactive ratatui audit card with three tabs:
//!
//! - **Tab 0 — Identity**: path, type, MIME, size, mode, inode, ownership,
//!   filesystem info, ELF binary metadata, and symlink targets.
//! - **Tab 1 — Security**: SELinux label, inode flags, access state,
//!   encryption source.
//! - **Tab 2 — Observations**: typed [`SecurityObservation`] findings with
//!   polarity-driven color coding.
//!
//! Key bindings: `Tab`/`Right` = next tab, `Shift-Tab`/`Left` = prev tab,
//! `j`/`k` = scroll, `q`/`Esc` = quit.
//!
//! ## Usage
//!
//! ```text
//! umrs-file-stat <PATH>
//! umrs-file-stat <PATH> --json
//! umrs-file-stat --help
//! umrs-file-stat --version
//! ```
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access enforcement — displays MAC label and mode.
//! - **NIST SP 800-53 AU-3**: Audit record content — every card is
//!   self-identifying (report name + subject).
//! - **NIST SP 800-53 CA-7**: Continuous monitoring — observations surface
//!   findings as typed data, not free-form text.
//! - **NIST SP 800-53 SC-28**: Protection at rest — encryption source display.
//! - **NIST SP 800-53 SI-7**: Integrity — SELinux TPI state is displayed;
//!   `/proc/mounts` is read via provenance-verified `ProcfsText`.

use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::Parser;
use crossterm::event::{self, Event};
use umrs_core::i18n;
use umrs_platform::kattrs::{ProcfsText, SecureReader};
use umrs_selinux::fs_encrypt::EncryptionSource;
use umrs_selinux::mcs::translator::{GLOBAL_TRANSLATOR, SecurityRange};
use umrs_selinux::posix::primitives::FileSize;
use umrs_selinux::secure_dirent::{
    InodeSecurityFlags, SecDirError, SecureDirent,
};
use umrs_selinux::{ObservationKind, SecurityObservation, SelinuxCtxState};
use umrs_tui::app::{
    AuditCardApp, AuditCardState, DataRow, StatusLevel, StatusMessage,
    StyleHint, TabDef,
};
use umrs_tui::indicators::build_header_context;
use umrs_tui::keymap::KeyMap;
use umrs_tui::layout::render_audit_card;
use umrs_tui::theme::Theme;

// ---------------------------------------------------------------------------
// Size formatting helper
// ---------------------------------------------------------------------------

/// Format a `FileSize` as "N bytes (X.Y KB/MB/GB)".
///
/// The raw byte count is always shown; the human-readable suffix is appended
/// when size is >= 1024 bytes. No rounding errors affect the raw byte display.
fn format_size(size: FileSize) -> String {
    let bytes = size.as_u64();
    if bytes < 1024 {
        return format!("{bytes} bytes");
    }
    // KB — precision loss from u64→f64 is acceptable: this is a display hint,
    // not a security-relevant value. Files > 2^53 bytes are not a realistic concern.
    #[allow(clippy::cast_precision_loss)]
    let kb_frac = bytes as f64 / 1024.0;
    if kb_frac < 1024.0 {
        return format!("{bytes} bytes ({kb_frac:.1} KB)");
    }
    #[allow(clippy::cast_precision_loss)]
    let mb_frac = bytes as f64 / (1024.0 * 1024.0);
    if mb_frac < 1024.0 {
        return format!("{bytes} bytes ({mb_frac:.1} MB)");
    }
    #[allow(clippy::cast_precision_loss)]
    let gb_frac = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{bytes} bytes ({gb_frac:.2} GB)")
}

// ---------------------------------------------------------------------------
// Status bar builder
// ---------------------------------------------------------------------------

fn build_status(observations: &[SecurityObservation]) -> StatusMessage {
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
struct FsInfo {
    device: String,
    mount_point: String,
    fs_type: String,
}

/// Find the filesystem entry for `path` by walking `/proc/mounts` and
/// selecting the longest-prefix mount point match.
///
/// Uses provenance-verified `ProcfsText` + `SecureReader` to satisfy
/// NIST SP 800-53 SI-7 (integrity) — raw `File::open` on `/proc/` is
/// prohibited by project policy.
fn find_fs_info(path: &Path) -> Option<FsInfo> {
    let node = ProcfsText::new(PathBuf::from("/proc/mounts")).ok()?;
    let contents =
        SecureReader::<ProcfsText>::new().read_generic_text(&node).ok()?;

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

        // Longest-prefix match: if the file path starts with this mount point,
        // and this mount point is longer than any previous match, it wins.
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
struct ElfInfo {
    class: &'static str,    // "ELF64" or "ELF32"
    elf_type: &'static str, // "Executable", "Shared object", "Relocatable", etc.
}

/// Read the first 20 bytes of `path` and extract ELF class and type.
///
/// Returns `None` if the file is not ELF or cannot be read.
///
/// This opens its own fd (like `tree_magic_mini`) — it is display-only and
/// not a trust-relevant assertion. The caller must not use the result in any
/// policy decision.
fn read_elf_info(path: &Path) -> Option<ElfInfo> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = [0u8; 20];
    f.read_exact(&mut buf).ok()?;

    // ELF magic: \x7fELF
    if buf[0..4] != [0x7f, b'E', b'L', b'F'] {
        return None;
    }

    // EI_CLASS (byte 4): 1 = 32-bit, 2 = 64-bit
    let class = match buf[4] {
        1 => "ELF32",
        2 => "ELF64",
        _ => "ELF (unknown class)",
    };

    // e_type (bytes 16-17, little-endian on Linux/x86 and Linux/aarch64)
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
// Row builders
// ---------------------------------------------------------------------------

/// Build the Identity tab rows for a successfully-read `SecureDirent`.
///
/// Row order:
/// 1. Path, Filename, File type
/// 2. Link target (symlink only)
/// 3. MIME type
/// 4. Binary format / Binary type (ELF regular files only)
/// 5. Size, Mode, Inode, Device
/// 6. Hard links (yellow if > 1 on a non-directory)
/// 7. Separator
/// 8. Owner, Group
/// 9. Separator
/// 10. Mount point
/// 11. Filesystem, Device node, Mounted on (if /proc/mounts readable)
fn build_identity_rows(
    dirent: &SecureDirent,
    mime: &str,
    path: &Path,
) -> Vec<DataRow> {
    let mut rows = Vec::new();

    // ── Static identity fields ──────────────────────────────────────────
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

    // ── Symlink target — display-only, we do not examine the target's attributes
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

    // ── MIME type ───────────────────────────────────────────────────────
    rows.push(DataRow::new(
        i18n::tr("MIME type"),
        mime.to_owned(),
        StyleHint::Highlight,
    ));

    // ── ELF binary info — display-only, not a trust-relevant assertion ──
    if dirent.file_type.is_regular() {
        if let Some(elf) = read_elf_info(path) {
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
    }

    // ── Size, mode, inode, device ───────────────────────────────────────
    rows.push(DataRow::normal(i18n::tr("Size"), format_size(dirent.size)));
    rows.push(DataRow::normal(i18n::tr("Mode"), dirent.mode.as_mode_str()));
    rows.push(DataRow::normal("Inode", dirent.inode.to_string()));
    rows.push(DataRow::normal("Device", dirent.dev.to_string()));

    // ── Hard links — yellow when > 1 on a non-directory (security finding) ──
    let nlink_val: u32 = dirent.nlink.into();
    let (nlink_str, nlink_hint) =
        if nlink_val > 1 && !dirent.file_type.is_directory() {
            (format!("{nlink_val} (hard-linked)"), StyleHint::TrustYellow)
        } else {
            (dirent.nlink.to_string(), StyleHint::Normal)
        };
    rows.push(DataRow::new("Hard links", nlink_str, nlink_hint));

    rows.push(DataRow::separator());

    // ── Owner — TrustYellow if name could not be resolved ───────────────
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

    // ── Group — TrustYellow if name could not be resolved ───────────────
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

    // ── Mount point ──────────────────────────────────────────────────────
    let mp_str = if dirent.is_mountpoint {
        "yes"
    } else {
        "no"
    };
    rows.push(DataRow::normal("Mount point", mp_str));

    // ── Filesystem information from /proc/mounts ─────────────────────────
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
/// Raw pipe-delimited Display would be confusing to non-expert readers.
///
/// NIST SP 800-53 SI-7, AU-9, AC-3.
fn build_inode_flag_rows(dirent: &SecureDirent) -> Vec<DataRow> {
    let mut rows = Vec::new();

    // Immutable — NIST SP 800-53 SI-7: Software and Information Integrity.
    let (immut_str, immut_hint) =
        if dirent.sec_flags.contains(InodeSecurityFlags::IMMUTABLE) {
            ("Yes", StyleHint::TrustGreen)
        } else {
            ("No", StyleHint::Normal)
        };
    rows.push(DataRow::new(i18n::tr("Immutable"), immut_str, immut_hint));

    // IMA/EVM — NIST SP 800-53 SI-7: integrity measurement hash present.
    let (ima_str, ima_hint) =
        if dirent.sec_flags.contains(InodeSecurityFlags::IMA_PRESENT) {
            ("Yes — integrity hash present", StyleHint::TrustGreen)
        } else {
            ("No", StyleHint::Normal)
        };
    rows.push(DataRow::new(i18n::tr("IMA/EVM"), ima_str, ima_hint));

    // Append-only — NIST SP 800-53 AU-9: Protection of Audit Information.
    let (append_str, append_hint) =
        if dirent.sec_flags.contains(InodeSecurityFlags::APPEND_ONLY) {
            ("Yes", StyleHint::TrustGreen)
        } else {
            ("No", StyleHint::Normal)
        };
    rows.push(DataRow::new("Append-only", append_str, append_hint));

    // POSIX ACL — NIST SP 800-53 AC-3: extended DAC in effect when present.
    let (acl_str, acl_hint) =
        if dirent.sec_flags.contains(InodeSecurityFlags::ACL_PRESENT) {
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

fn build_security_rows(dirent: &SecureDirent) -> Vec<DataRow> {
    let mut rows = Vec::new();

    // SELinux label — color hint derived from label state; full context is shown
    // per-component below, so the summary row displays a placeholder.
    let label_hint = match &dirent.selinux_label {
        SelinuxCtxState::Labeled(_) => StyleHint::TrustGreen,
        SelinuxCtxState::Unlabeled
        | SelinuxCtxState::ParseFailure
        | SelinuxCtxState::TpiDisagreement => StyleHint::TrustRed,
    };
    let label_str = " ";
    rows.push(DataRow::new(
        i18n::tr("SELinux Context"),
        label_str,
        label_hint,
    ));

    // Per-component fields — only populated when Labeled
    if let SelinuxCtxState::Labeled(ctx) = &dirent.selinux_label {
        rows.push(DataRow::normal("  SELinux user", ctx.user().to_string()));
        rows.push(DataRow::normal("  SELinux role", ctx.role().to_string()));
        rows.push(DataRow::normal(
            "  SELinux type",
            ctx.security_type().to_string(),
        ));
        let level_str = ctx
            .level()
            .map_or_else(|| "(none)".to_owned(), |l| l.raw().to_owned());
        rows.push(DataRow::normal("  Raw label", level_str));

        // Label state variant name
        let (state_str, state_hint) = match &dirent.selinux_label {
            SelinuxCtxState::Labeled(_) => ("Labeled", StyleHint::TrustGreen),
            SelinuxCtxState::Unlabeled => ("Unlabeled", StyleHint::TrustRed),
            SelinuxCtxState::ParseFailure => {
                ("ParseFailure", StyleHint::TrustRed)
            }
            SelinuxCtxState::TpiDisagreement => {
                ("TpiDisagreement", StyleHint::TrustRed)
            }
        };
        rows.push(DataRow::new(
            i18n::tr("  Label state"),
            state_str,
            state_hint,
        ));

        rows.push(DataRow::separator());

        // Security Marking — MCS translation of the MLS level.
        // Provides the human-readable classification marking (e.g. "SECRET//HCS")
        // that corresponds to the raw kernel level string.
        // NIST SP 800-53 AC-4: Information Flow Enforcement.
        //
        //  TODO: Add security handling and ohter information from
        //  our custom json file
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

    // Inode security flags (Immutable, IMA/EVM, Append-only, POSIX ACL,
    // Access denied) — extracted for line-count clarity.
    rows.extend(build_inode_flag_rows(dirent));

    rows.push(DataRow::separator());

    // Encryption source
    let (enc_str, enc_hint) = match &dirent.encryption {
        EncryptionSource::None => ("None".to_owned(), StyleHint::Normal),
        EncryptionSource::LuksDevice => {
            ("LUKS (dm-crypt)".to_owned(), StyleHint::TrustGreen)
        }
        EncryptionSource::EncryptedFilesystem(fs) => (
            format!("Encrypted filesystem ({fs})"),
            StyleHint::TrustGreen,
        ),
    };
    rows.push(DataRow::new(i18n::tr("Encryption"), enc_str, enc_hint));

    rows
}

fn build_observation_rows(
    observations: &[SecurityObservation],
) -> Vec<DataRow> {
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

fn build_identity_error_rows() -> Vec<DataRow> {
    vec![DataRow::new(
        "Status",
        i18n::tr("Failed to read file"),
        StyleHint::TrustRed,
    )]
}

fn build_security_error_rows(err: &SecDirError) -> Vec<DataRow> {
    vec![
        DataRow::new(
            "Status",
            i18n::tr("No data available"),
            StyleHint::TrustRed,
        ),
        DataRow::new("Error", err.to_string(), StyleHint::TrustRed),
    ]
}

fn build_observation_error_rows() -> Vec<DataRow> {
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
/// Constructed once; the `SecureDirent` is consumed at build time and its
/// data stored in pre-built row vectors. The render path is pure read — no
/// I/O on any draw call.
///
/// NIST SP 800-53 AC-3, AU-3, CA-7, SC-28, SI-7.
struct FileStatApp {
    tabs: Vec<TabDef>,
    identity_rows: Vec<DataRow>,
    security_rows: Vec<DataRow>,
    observation_rows: Vec<DataRow>,
    status: StatusMessage,
    /// `'static` lifetime obtained via `Box::leak` — the string is the file
    /// path provided on the command line.
    report_subject: &'static str,
}

impl FileStatApp {
    /// Build the app from a successfully-read `SecureDirent`.
    fn from_dirent(dirent: &SecureDirent, mime: &str) -> Self {
        let observations = dirent.security_observations();
        let status = build_status(&observations);

        let tabs = vec![
            TabDef::new(i18n::tr("Identity")),
            TabDef::new(i18n::tr("Security")),
            TabDef::new(i18n::tr("Observations")),
        ];

        // dirent.path is AbsolutePath which implements Deref<Target = str>,
        // so &*dirent.path gives &str and Path::new converts it to &Path.
        let identity_rows =
            build_identity_rows(dirent, mime, Path::new(&*dirent.path));
        let security_rows = build_security_rows(dirent);
        let observation_rows = build_observation_rows(&observations);

        // Leak the path string to satisfy &'static str in the trait.
        // This is a one-time allocation per binary run — acceptable cost.
        let subject: &'static str =
            Box::leak(dirent.path.to_string().into_boxed_str());

        Self {
            tabs,
            identity_rows,
            security_rows,
            observation_rows,
            status,
            report_subject: subject,
        }
    }

    /// Build the app from a `SecDirError` (file could not be read).
    fn from_error(path_str: &str, err: &SecDirError) -> Self {
        let tabs = vec![
            TabDef::new(i18n::tr("Identity")),
            TabDef::new(i18n::tr("Security")),
            TabDef::new(i18n::tr("Observations")),
        ];

        let subject: &'static str =
            Box::leak(path_str.to_owned().into_boxed_str());

        Self {
            tabs,
            identity_rows: build_identity_error_rows(),
            security_rows: build_security_error_rows(err),
            observation_rows: build_observation_error_rows(),
            status: StatusMessage::new(
                StatusLevel::Error,
                i18n::tr("Failed to read file attributes"),
            ),
            report_subject: subject,
        }
    }
}

impl AuditCardApp for FileStatApp {
    fn report_name(&self) -> &'static str {
        "File Security Audit"
    }

    fn report_subject(&self) -> &'static str {
        self.report_subject
    }

    fn card_title(&self) -> String {
        i18n::tr("File Security Audit")
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => self.identity_rows.clone(),
            1 => self.security_rows.clone(),
            2 => self.observation_rows.clone(),
            _ => vec![DataRow::normal("(no data)", "(invalid tab index)")],
        }
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }
}

// ---------------------------------------------------------------------------
// CLI argument definition
// ---------------------------------------------------------------------------

/// Command-line arguments for `umrs-file-stat`.
///
/// Uses the clap derive API per project style decision SDR-7.
/// The `--json` flag is reserved for future structured output; the flag is
/// accepted today so operator scripts can be written in advance of the
/// implementation.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Operator supplies the target path explicitly;
///   no implicit subject selection occurs.
/// - **NIST SP 800-53 CM-6**: Output mode is operator-controlled via `--json`;
///   the default mode is human-readable with no configuration assumption.
#[derive(Parser)]
#[command(
    name = "umrs-file-stat",
    version,
    about = "Display security attributes of a file as an interactive audit card"
)]
struct Args {
    /// Path to the file whose security attributes will be inspected.
    path: PathBuf,

    /// Emit machine-readable JSON output instead of the interactive TUI.
    ///
    /// JSON output support is reserved for a future implementation phase.
    /// Supplying this flag today is accepted and noted in the debug log.
    #[arg(long)]
    json: bool,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    // ── i18n ─────────────────────────────────────────────────────────────
    // Initialize gettext catalog for the "umrs-tui" domain. Must be called
    // before any i18n::tr() calls. Falls back to the msgid if no catalog
    // is found — no error surfaced to the user.
    i18n::init("umrs-tui");

    // ── Logging ──────────────────────────────────────────────────────────
    // Best-effort journald logger. Failures are silently ignored — a TUI
    // should not write to stderr (would corrupt the terminal state).
    if let Ok(logger) = systemd_journal_logger::JournalLog::new() {
        let _ = logger.install();
        log::set_max_level(log::LevelFilter::Info);
    }

    // ── Argument parsing ─────────────────────────────────────────────────
    // Clap derive API — exits with usage text on missing args or --help.
    let args = Args::parse();

    // Convert PathBuf to &str for downstream callers that require it.
    // Non-UTF-8 paths are rejected with a clear operator message — this tool
    // operates on labelled paths that must be displayable in the audit card.
    let path_str = if let Some(s) = args.path.to_str() {
        s.to_owned()
    } else {
        eprintln!(
            "error: path contains non-UTF-8 characters and cannot be displayed"
        );
        std::process::exit(1);
    };

    // --json flag: accepted today; JSON output is a future implementation phase.
    // The flag is noted in the debug log so operators can verify it was received.
    // Using the value here also prevents a dead-code lint when JSON is not yet wired.
    log::debug!(
        "umrs-file-stat: json={} (JSON output not yet implemented)",
        args.json
    );

    // ── SecureDirent ──────────────────────────────────────────────────────
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let dirent_result = SecureDirent::from_path(Path::new(&path_str));

    #[cfg(debug_assertions)]
    log::debug!(
        "TOCTOU-safe SecureDirent construction completed in {} µs",
        t0.elapsed().as_micros()
    );

    // ── MIME detection ────────────────────────────────────────────────────
    // tree_magic_mini opens its own fd for MIME detection (path-based, not fd-based).
    // This is acceptable: MIME type is display-only, not a trust-relevant assertion
    // and not part of any policy decision.
    let mime: &str = match &dirent_result {
        Ok(_) => tree_magic_mini::from_filepath(Path::new(&path_str))
            .unwrap_or("unknown"),
        Err(_) => "unknown",
    };

    // ── Build app ─────────────────────────────────────────────────────────
    let app: FileStatApp = match &dirent_result {
        Ok(dirent) => {
            log::info!("umrs-file-stat: loaded {path_str}");
            FileStatApp::from_dirent(dirent, mime)
        }
        Err(ref e) => {
            log::warn!("umrs-file-stat: failed to read {path_str}: {e}");
            FileStatApp::from_error(&path_str, e)
        }
    };

    // ── UI state ──────────────────────────────────────────────────────────
    let mut state = AuditCardState::new(app.tabs().len());
    let keymap = KeyMap::default();
    let theme = Theme::default();
    // os_name is "unavailable" in file_stat — this binary does not run
    // the OS detection pipeline. The OS is visible from the RHEL host context;
    // the header row displays the best available value.
    let ctx = build_header_context(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        "unavailable",
    );

    // ── Terminal setup ────────────────────────────────────────────────────
    let mut terminal = ratatui::init();

    // ── Event loop ────────────────────────────────────────────────────────
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_audit_card(f, f.area(), &app, &state, &ctx, &theme);
        }) {
            log::error!("terminal draw error: {e}");
            break;
        }

        match event::poll(Duration::from_millis(250)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => {
                    if let Some(action) = keymap.lookup(&key) {
                        state.handle_action(&action);
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    log::warn!("event read error: {e}");
                }
            },
            Ok(false) => {}
            Err(e) => {
                log::warn!("event poll error: {e}");
            }
        }

        if state.should_quit {
            break;
        }
    }

    // ── Terminal teardown ─────────────────────────────────────────────────
    ratatui::restore();
}
