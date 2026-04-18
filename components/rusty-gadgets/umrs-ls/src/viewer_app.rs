// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)

//! # viewer_app — ViewerApp Implementation for umrs-ls
//!
//! Provides [`DirViewerApp`], which implements the [`umrs_ui::viewer::ViewerApp`]
//! trait for the `umrs-ls` TUI directory browser.  The struct wraps a
//! [`DirListing`] and cached [`ScanStats`], exposing read-only data to the
//! viewer render path.
//!
//! ## Key Exported Types
//!
//! - [`DirViewerApp`] — implements `ViewerApp`; constructed via [`DirViewerApp::scan`]
//!   or, for tests, [`DirViewerApp::from_listing`].
//!
//! ## Re-scan on Navigation
//!
//! [`DirViewerApp::navigate_to`] accepts a new directory path, performs a fresh
//! [`list_directory`] call, updates the internal listing and stats in place, and
//! returns a new [`TreeModel`].  The event loop (Phase 3) calls
//! `ViewerState::load_tree(new_tree)` to replace the display.
//!
//! ## Performance
//!
//! - `status()` and `viewer_header()` are called every rendered frame.  Both
//!   read from pre-computed cached state — no I/O, no allocation beyond
//!   `format!()` for the status text.
//! - `navigate_to()` is called only on user-initiated directory navigation.
//!   A single `list_directory` + `build_tree` + `compute_stats` pass runs and
//!   the result is stored.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: The `ViewerApp` trait provides no mutation
//!   operations; the directory listing is unconditionally read-only from the
//!   viewer's perspective.
//! - **NIST SP 800-53 AU-3**: `viewer_header()` carries tool identity,
//!   data source (directory path), and record count on every rendered frame.
//! - **NSA RTB RAIN**: Non-bypassable read-only contract — all mutation is
//!   funnelled through `navigate_to()` and `scan()`, never through the trait.

use std::io;
use std::path::{Path, PathBuf};

#[cfg(debug_assertions)]
use std::time::Instant;

use umrs_selinux::fs_encrypt::{EncryptionSource, detect_mount_encryption};
use umrs_selinux::secure_dirent::SecureDirent;
use umrs_selinux::utils::dirlist::{DirListing, list_directory};
use umrs_ui::app::{StatusLevel, StatusMessage, TabDef};
use umrs_ui::viewer::tree::TreeModel;
use umrs_ui::viewer::{ViewerApp, ViewerHeaderContext};

use crate::identity::resolve_owner_display;
use crate::tree_adapter::{ScanStats, build_tree, compute_stats};

// ────────────────────────────────────────────────────────────────────────────
// DirViewerApp
// ────────────────────────────────────────────────────────────────────────────

/// Security metadata for the current directory itself (not its contents).
///
/// Displayed in the header panel beneath the directory path so the operator
/// can see the directory's own security posture at a glance.
///
/// ## Fields:
///
/// - `mode` — mode string (e.g., `"drwxr-xr-x"`).
/// - `owner` — owner username (or numeric uid).
/// - `group` — group name (or numeric gid).
/// - `selinux_type` — SELinux type (e.g., `"var_log_t"`).
/// - `marking` — MCS/MLS marking (e.g., `"s0"`).
/// - `is_mountpoint` — whether this directory is a mount point.
/// - `encryption` — at-rest encryption source of the containing mount point, if any; `None`
///   means the directory's backing mount point is not encrypted; `Some("LUKS")` means
///   LUKS/dm-crypt protected; `Some(fs)` means filesystem-layer encryption (e.g.,
///   `"ecryptfs"`, `"fuse.gocryptfs"`). NIST SP 800-53 SC-28.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Mode, ownership, and MAC label of the listed directory are
///   audit-relevant context.
/// - **NIST SP 800-53 SC-28**: At-rest protection posture is surfaced on every frame.
#[derive(Debug, Clone)]
pub struct DirMeta {
    pub mode: String,
    pub owner: String,
    pub group: String,
    pub selinux_type: String,
    pub marking: String,
    pub is_mountpoint: bool,
    pub encryption: Option<String>,
}

impl DirMeta {
    /// Stat a directory path and extract its security metadata.
    ///
    /// Attempts `SecureDirent::from_path` first.  For paths where
    /// `SecureDirent` cannot construct a `ValidatedFileName` (notably `/`,
    /// which has no filename component), falls back to a manual stat +
    /// SELinux context read so the header still shows real metadata.
    fn from_path(path: &Path) -> Self {
        if let Ok(d) = SecureDirent::from_path(path) {
            let ft = crate::tree_adapter::file_type_char_pub(d.file_type);
            let mode = format!("{ft}{}", d.mode.as_mode_str());
            let uid = d.ownership.user.uid.as_u32();
            let gid = d.ownership.group.gid.as_u32();
            let (owner, group) = resolve_owner_display(uid, gid);
            let (selinux_type, marking) = extract_selinux_short(&d);
            let is_mountpoint = d.is_mountpoint;
            let encryption = detect_enclosing_encryption(path);
            return Self {
                mode,
                owner,
                group,
                selinux_type,
                marking,
                is_mountpoint,
                encryption,
            };
        }

        // Fallback: manual stat for paths rejected by ValidatedFileName
        // (e.g., `/` which has no filename component).
        // DIRECT-IO-EXCEPTION: display-only header metadata for the root
        // directory; no trust decision is made from this data.
        Self::from_metadata_fallback(path)
    }

    /// Stat fallback used when `SecureDirent::from_path` cannot construct a
    /// `ValidatedFileName` for the path (e.g., `/`).
    ///
    /// Uses `std::fs::symlink_metadata` for POSIX attributes and
    /// `umrs_selinux::utils::get_file_context` (fd-anchored, TPI-validated)
    /// for the SELinux context.  The result is display-only; no security
    /// decision is derived from it.
    fn from_metadata_fallback(path: &Path) -> Self {
        use std::os::unix::fs::MetadataExt as _;

        use umrs_selinux::posix::primitives::FileMode;
        use umrs_selinux::secure_dirent::FileType;

        let Ok(meta) = std::fs::symlink_metadata(path) else {
            return Self::placeholder();
        };

        let file_mode = FileMode::from_mode(meta.mode());
        let file_type = FileType::from_mode(meta.mode());
        let ft = crate::tree_adapter::file_type_char_pub(file_type);
        let mode = format!("{ft}{}", file_mode.as_mode_str());

        let (owner, group) = resolve_owner_display(meta.uid(), meta.gid());

        let (selinux_type, marking) = selinux_from_path(path);

        let is_mountpoint = path == Path::new("/")
            || path.parent().is_none_or(|parent| {
                std::fs::symlink_metadata(parent).map_or(true, |pm| pm.dev() != meta.dev())
            });

        let encryption = detect_enclosing_encryption(path);

        Self {
            mode,
            owner,
            group,
            selinux_type,
            marking,
            is_mountpoint,
            encryption,
        }
    }

    /// Return a placeholder `DirMeta` used when stat fails entirely.
    fn placeholder() -> Self {
        Self {
            mode: "?".to_owned(),
            owner: "?".to_owned(),
            group: "?".to_owned(),
            selinux_type: "?".to_owned(),
            marking: "?".to_owned(),
            is_mountpoint: false,
            encryption: None,
        }
    }

    /// True if the containing mount point has any at-rest encryption.
    ///
    /// NIST SP 800-53 SC-28 — used by the listing renderer to inherit
    /// encryption state to every file in the directory.
    #[must_use = "encryption state drives the IOVE column; discarding it hides compliance gaps"]
    pub const fn is_encrypted(&self) -> bool {
        self.encryption.is_some()
    }

    /// Human-readable label for the at-rest encryption state.
    ///
    /// Returns `"LUKS"` for LUKS/dm-crypt, the filesystem type name for
    /// filesystem-layer encryption (e.g., `"ecryptfs"`), or `"unencrypted"`
    /// when no at-rest protection is detected on the containing mount.
    #[must_use = "encryption label is rendered in the directory header; discarding it leaves the display blank"]
    pub fn encryption_label(&self) -> &str {
        self.encryption.as_deref().unwrap_or("unencrypted")
    }
}

/// Walk from `path` upward, calling [`detect_mount_encryption`] at each
/// ancestor until a mount point is hit.  Returns the encryption source of
/// the containing mount point, or `None` if the path is on an unencrypted
/// filesystem.
///
/// `detect_mount_encryption` only matches exact mount point paths in
/// `/proc/mounts`, so non-mount ancestors return [`EncryptionSource::None`]
/// and the walk continues upward.  The walk is bounded by path depth — at
/// most a handful of `/proc/mounts` parses per directory navigation.
///
/// Fail-closed: any unexpected condition returns `None` so the IOVE column
/// does not fabricate encryption state that is not provable.
///
/// NIST SP 800-53 SC-28 — at-rest encryption detection.
/// NIST SP 800-53 SI-7 — each probe routes through the provenance-verified
/// reader inside `detect_mount_encryption`.
fn detect_enclosing_encryption(path: &Path) -> Option<String> {
    let mut current: Option<&Path> = Some(path);
    while let Some(p) = current {
        match detect_mount_encryption(p) {
            EncryptionSource::LuksDevice => return Some("LUKS".to_owned()),
            EncryptionSource::EncryptedFilesystem(fs) => return Some(fs),
            EncryptionSource::None => {
                current = p.parent();
            }
        }
    }
    None
}

/// Directory viewer application state for the `umrs-ls` TUI.
///
/// Wraps a [`DirListing`] and its cached [`ScanStats`].  Implements
/// [`ViewerApp`] so the viewer render path can obtain the header context,
/// status message, and initial tree model without performing any I/O.
///
/// Construct via [`DirViewerApp::scan`] at startup, or
/// [`DirViewerApp::from_listing`] in tests.  Call [`DirViewerApp::navigate_to`]
/// when the user selects a subdirectory.
///
/// ## Fields:
///
/// - `current_path` (private) — current directory path being displayed.
/// - `listing` (private) — the latest directory listing.
/// - `stats` (private) — cached scan stats for the status bar.
/// - `dir_meta` (private) — security metadata for the current directory itself.
/// - `tabs` (private) — tab definitions; a single tab labelled `"Directory"`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3 / AC-4**: Read-only view; the underlying listing is not mutated
///   through the `ViewerApp` interface.
/// - **NIST SP 800-53 AU-3**: Header and status carry identification fields on every rendered
///   frame.
pub struct DirViewerApp {
    current_path: PathBuf,
    listing: DirListing,
    stats: ScanStats,
    dir_meta: DirMeta,
    tabs: Vec<TabDef>,
}

impl DirViewerApp {
    /// Scan a directory and construct the viewer app.
    ///
    /// Calls [`list_directory`], then [`compute_stats`].  Returns `Err` if
    /// the directory cannot be listed (e.g., permission denied, not a directory).
    ///
    /// NIST SP 800-53 AU-3 — the resulting stats carry elapsed time and entry
    /// counts for the status bar.
    #[must_use = "DirViewerApp must be used in the event loop; scan result must not be discarded"]
    pub fn scan(path: &Path) -> io::Result<Self> {
        #[cfg(debug_assertions)]
        let start = Instant::now();

        let listing = list_directory(path)?;
        let stats = compute_stats(&listing);
        let current_path = path.to_path_buf();
        let tabs = vec![TabDef::new("Directory")];

        #[cfg(debug_assertions)]
        {
            let elapsed = start.elapsed();
            log::debug!(
                "viewer_app::DirViewerApp::scan completed in {} µs (path: {}, {} groups)",
                elapsed.as_micros(),
                path.display(),
                listing.groups.len(),
            );
        }

        let dir_meta = DirMeta::from_path(path);

        Ok(Self {
            current_path,
            listing,
            stats,
            dir_meta,
            tabs,
        })
    }

    /// Construct a `DirViewerApp` from an already-collected [`DirListing`].
    ///
    /// Intended for testing: callers can build a synthetic listing without
    /// performing real I/O.  `path` is used as the current directory path
    /// stored in the app; it does not need to exist on the filesystem.
    ///
    /// NIST SP 800-53 AU-3 — `compute_stats` is still applied so the status
    /// bar reflects the provided listing's actual counts.
    #[must_use = "DirViewerApp must be used in the event loop or inspected in tests"]
    pub fn from_listing(path: PathBuf, listing: DirListing) -> Self {
        let stats = compute_stats(&listing);
        let tabs = vec![TabDef::new("Directory")];
        let dir_meta = DirMeta::from_path(&path);
        Self {
            current_path: path,
            listing,
            stats,
            dir_meta,
            tabs,
        }
    }

    /// Re-scan a new directory and update internal state.
    ///
    /// Called when the user selects a subdirectory in the TUI.  Performs a
    /// fresh [`list_directory`] call on `path`, then updates `current_path`,
    /// `listing`, and `stats`.
    ///
    /// Returns the new [`TreeModel`] ready to be passed to
    /// `ViewerState::load_tree(new_tree)`.
    ///
    /// Returns `Err` if the directory cannot be listed.  On error, internal
    /// state is **not** updated — the previous listing remains valid.
    ///
    /// NIST SP 800-53 AU-3 — stats are refreshed after each navigation so
    /// the status bar always reflects the currently displayed directory.
    #[must_use = "the returned TreeModel must be loaded into ViewerState; discarding it leaves the display stale"]
    pub fn navigate_to(&mut self, path: &Path) -> io::Result<TreeModel> {
        #[cfg(debug_assertions)]
        let start = Instant::now();

        let listing = list_directory(path)?;
        let stats = compute_stats(&listing);
        let new_dir_meta = DirMeta::from_path(path);
        let tree = build_tree(&listing, path, new_dir_meta.is_encrypted());

        // Commit only after all fallible operations succeed.
        self.current_path = path.to_path_buf();
        self.listing = listing;
        self.stats = stats;
        self.dir_meta = new_dir_meta;

        #[cfg(debug_assertions)]
        {
            let elapsed = start.elapsed();
            log::debug!(
                "viewer_app::DirViewerApp::navigate_to completed in {} µs (path: {})",
                elapsed.as_micros(),
                path.display(),
            );
        }

        Ok(tree)
    }

    /// Return the current directory path.
    #[must_use = "current_path is required for display and navigation logic"]
    pub fn current_path(&self) -> &Path {
        &self.current_path
    }

    /// Return the cached scan stats.
    ///
    /// Updated after every successful [`DirViewerApp::navigate_to`] call and
    /// after construction.  Contains file count, directory count, and elapsed
    /// microseconds.
    #[must_use = "ScanStats is the only output; discarding it prevents status bar updates"]
    pub const fn stats(&self) -> &ScanStats {
        &self.stats
    }

    /// Return the security metadata for the current directory itself.
    ///
    /// Updated after every successful navigation. Contains mode, ownership,
    /// SELinux type/marking, and mountpoint status.
    #[must_use = "DirMeta is needed by the renderer for the header display"]
    pub const fn dir_meta(&self) -> &DirMeta {
        &self.dir_meta
    }

    /// Build a [`TreeModel`] from the current listing.
    ///
    /// Called by [`ViewerApp::initial_tree`] and available directly to the
    /// event loop for explicit rebuilds.  No I/O is performed.
    ///
    /// NIST SP 800-53 AU-3 / AC-3 / AC-4 — tree preserves all metadata fields
    /// from the listing; see `tree_adapter::build_tree` for details.
    #[must_use = "the returned TreeModel must be loaded into ViewerState; discarding it leaves the display empty"]
    pub fn build_current_tree(&self) -> TreeModel {
        build_tree(
            &self.listing,
            &self.current_path,
            self.dir_meta.is_encrypted(),
        )
    }
}

// ────────────────────────────────────────────────────────────────────────────
// ViewerApp impl
// ────────────────────────────────────────────────────────────────────────────

impl ViewerApp for DirViewerApp {
    fn card_title(&self) -> &'static str {
        "UMRS Directory Security"
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    /// Format: `"{elapsed}ms · {file_count} files · {dir_count} directories"`.
    ///
    /// If the elapsed time is less than 1 ms, renders `"<1ms"`.
    /// Uses [`StatusLevel::Info`] for all normal scans.
    ///
    /// NIST SP 800-53 AU-3 — scan timing and entry counts are always visible.
    fn status(&self) -> StatusMessage {
        let elapsed_str = format_elapsed(self.stats.elapsed_us);
        let text = format!(
            "{elapsed_str} · {} files · {} directories",
            self.stats.file_count, self.stats.dir_count,
        );
        StatusMessage::new(StatusLevel::Info, text)
    }

    /// Returns tool identification, the current directory path, and entry
    /// count summary.
    ///
    /// Summary format: `"{group_count} SELinux groups · {total} entries"`.
    ///
    /// Called every rendered frame — no I/O, no heap allocation beyond the
    /// `String` fields required by [`ViewerHeaderContext`].
    ///
    /// NIST SP 800-53 AU-3 — tool name, data source, and record count are
    /// always present in the header.
    fn viewer_header(&self) -> ViewerHeaderContext {
        let data_source = self.current_path.to_string_lossy().into_owned();
        let total_entries = self.stats.file_count.saturating_add(self.stats.dir_count);
        let group_count = self.listing.groups.len();

        let summary = format!("{group_count} SELinux groups · {total_entries} entries",);

        ViewerHeaderContext::new("umrs-ls", data_source, total_entries).with_summary(summary)
    }

    /// Provide the initial tree model for the viewer.
    ///
    /// Always returns `Some` — the listing is available at construction time.
    ///
    /// NIST SP 800-53 AU-3 — the tree carries complete audit metadata for
    /// every entry in the listing.
    fn initial_tree(&self) -> Option<TreeModel> {
        Some(self.build_current_tree())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Private helpers
// ────────────────────────────────────────────────────────────────────────────

/// Format a microsecond elapsed time as a milliseconds string.
///
/// Returns `"<1ms"` for sub-millisecond values to avoid showing `"0ms"`,
/// which could mislead an operator into thinking no timing was recorded.
fn format_elapsed(elapsed_us: u64) -> String {
    const MICROS_PER_MS: u64 = 1_000;
    if elapsed_us < MICROS_PER_MS {
        "<1ms".to_owned()
    } else {
        format!("{}ms", elapsed_us / MICROS_PER_MS)
    }
}

/// Read SELinux type and translated marking directly from a filesystem path.
///
/// Used as a fallback when `SecureDirent::from_path` cannot construct a
/// `ValidatedFileName` (e.g., for `/`).  Opens the path via
/// `umrs_selinux::utils::get_file_context`, which is fd-anchored and
/// TPI-validated.  Returns `("?", "?")` on any failure.
///
/// NIST SP 800-53 AU-3 — security context of the listed directory is
/// audit-relevant header context.
fn selinux_from_path(path: &Path) -> (String, String) {
    use umrs_selinux::mcs::translator::{GLOBAL_TRANSLATOR, SecurityRange};
    use umrs_selinux::utils::get_file_context;

    match get_file_context(path) {
        Ok(ctx) => {
            let selinux_type = ctx.security_type().to_string();
            let marking = ctx.level().map_or_else(
                || "<no-level>".to_owned(),
                |lvl| {
                    let range = SecurityRange::from_level(lvl);
                    GLOBAL_TRANSLATOR.read().map_or_else(
                        |_| lvl.raw().to_owned(),
                        |g| g.lookup(&range).unwrap_or_else(|| lvl.raw().to_owned()),
                    )
                },
            );
            (selinux_type, marking)
        }
        Err(_) => ("?".to_owned(), "?".to_owned()),
    }
}

/// Extract SELinux type and translated marking from a `SecureDirent`.
///
/// Uses the global setrans translator to convert the raw MLS/MCS level
/// into a human-readable marking (e.g., `"s0"` → `"SystemLow"`, or
/// `"s0:c90"` → `"CUI//LEI"`).  Falls back to the raw level string if
/// the translator is not loaded or has no mapping.
fn extract_selinux_short(dirent: &SecureDirent) -> (String, String) {
    use umrs_selinux::mcs::translator::{GLOBAL_TRANSLATOR, SecurityRange};

    match &dirent.selinux_label {
        umrs_selinux::SelinuxCtxState::Labeled(ctx) => {
            let selinux_type = ctx.security_type().to_string();
            let marking = ctx.level().map_or_else(
                || "<no-level>".to_owned(),
                |lvl| {
                    let range = SecurityRange::from_level(lvl);
                    GLOBAL_TRANSLATOR.read().map_or_else(
                        |_| lvl.raw().to_owned(),
                        |g| g.lookup(&range).unwrap_or_else(|| lvl.raw().to_owned()),
                    )
                },
            );
            (selinux_type, marking)
        }
        _ => ("<unlabeled>".to_owned(), "<no-level>".to_owned()),
    }
}
