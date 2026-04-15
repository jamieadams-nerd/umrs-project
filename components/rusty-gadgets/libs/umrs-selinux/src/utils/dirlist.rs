// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
//! # High-Assurance Directory Listing
//!
//! Produces a [`DirListing`] — entries grouped by `(SELinux type, security
//! marking)`, each group pre-sorted directories-first then alphabetically.
//! No display logic lives here; all rendering is left to the caller.
//!
//! ## Architectural invariants
//!
//! - **TOCTOU safety**: all per-file reads delegate to
//!   [`SecureDirent::from_path`], which anchors every syscall to a single fd.
//! - **Single lock acquisition**: [`GLOBAL_TRANSLATOR`] is read-locked once
//!   per entry (not per field), minimising contention.
//! - **Fail-open on access errors**: entries where `open()` was denied return
//!   `Ok` from `SecureDirent::from_path` with `access_denied: true`. These are
//!   grouped under `<restricted>` — distinct from genuinely unlabeled objects.
//!   All other errors are logged and skipped.
//! - **No display output**: no `println!`, no ANSI codes, no formatted
//!   strings — only structured, owned data.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: access decisions recorded per entry.
//! - **NIST SP 800-53 AC-4**: information-flow labels drive grouping.
//! - **NIST SP 800-53 AU-3**: complete audit record per directory entry.
//! - **NSA RTB Non-Bypassability**: TOCTOU safety via `SecureDirent`.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};

use crate::mcs::translator::{GLOBAL_TRANSLATOR, SecurityRange};
use crate::secure_dirent::SecureDirent;

// GroupKey
//
/// Compound group key: SELinux type paired with its security marking.
///
/// `Ord` is derived so [`BTreeMap`] provides automatic lexicographic ordering:
/// type-alpha first, then marking-alpha — no explicit sort step required.
///
/// NIST SP 800-53 AC-4: information-flow labelling.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupKey {
    /// SELinux type component — e.g., `bin_t`, `httpd_t`, `<unlabeled>`.
    pub selinux_type: String,
    /// setrans translation when available; raw level string otherwise.
    /// E.g., `CUI//INV/LEI` (alphabetized combined marking), `s0`, `<no-level>`.
    pub marking: String,
}

// ListEntry
//
/// A [`SecureDirent`] paired with its modification time.
///
/// [`SecureDirent`] intentionally omits `mtime` (display-layer concern).
/// [`ListEntry`] carries it here so the caller has everything needed to
/// render a complete `ls`-style row.
///
/// NIST SP 800-53 AU-3: `mtime` is a required audit record field.
#[derive(Debug, Clone)]
pub struct ListEntry {
    /// Fully validated, security-enriched directory entry.
    pub dirent: SecureDirent,
    /// Modification time captured from `DirEntry::metadata()`.
    /// `None` if the OS did not return a valid mtime.
    pub mtime: Option<SystemTime>,
}

/// All entries sharing the same [`GroupKey`], pre-sorted.
///
/// Sort order within a group: directories first (alphabetical by name),
/// then all other entries (alphabetical by name). This ordering is stable
/// and can be consumed directly by a ratatui list widget.
///
/// GroupKey - is the compound key that defines this group.
/// entries  - pres-roted entries: directories first (alpha), then files (alpha)
///
#[derive(Debug, Clone)]
pub struct DirGroup {
    pub key: GroupKey,
    pub entries: Vec<ListEntry>,
}

/// The complete result of listing a directory.
///
/// Owned, `Clone`-able, and entirely free of display logic. A ratatui
/// `Widget` or a slint data model can consume this struct directly in a
/// future interactive phase.
///
/// NIST SP 800-53 AU-3: complete directory audit record.
#[derive(Debug, Clone)]
pub struct DirListing {
    /// The directory that was listed.
    pub path: PathBuf,

    /// Groups sorted by [`GroupKey`] (type-alpha, then marking-alpha).
    pub groups: Vec<DirGroup>,

    /// Filenames that could not be listed at all (e.g., `readdir` error on
    /// the entry itself). Access-denied entries are now grouped under
    /// `<restricted>` in [`DirListing::groups`] rather than collected here.
    /// Field retained for API stability; currently always empty.
    ///
    /// NIST SP 800-53 AU-3: an incomplete listing is itself an audit observation.
    pub access_denied: Vec<String>,

    /// Wall-clock time of the listing operation in microseconds.
    pub elapsed_us: u64,
}

/// A single display column.
///
/// `Column` is `Copy` so that [`ColumnSet`] operations are allocation-free.
/// Adding a new column requires: a new variant here, a rendering arm in the
/// caller's cell renderer, and a label in `col_header()`.
///
/// CMMC Level 2 — CM.L2-3.4.2: column vocabulary maps to security baseline
/// items (mode bits, ownership, SELinux label, integrity flags).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Column {
    /// `drwxr-xr-x` — file-type char + 9 permission bits (10 chars total).
    Mode,

    /// `I`=immutable · `O`=security observations present · `V`=IMA — 3-char
    /// security-posture marker.  ACL presence is now indicated by `+` in the
    /// [`Column::Mode`] string.
    ///
    /// NIST SP 800-53 AU-9 (immutable), RA-5 (observations), SI-7 (IMA).
    Iov,

    /// SELinux type component extracted from the security context.
    ///
    /// NIST SP 800-53 AC-4.
    SelinuxType,

    /// setrans-translated marking, or raw level string when untranslated.
    ///
    /// NIST SP 800-53 AC-4 / CMMC AC.L2-3.1.3.
    Marking,

    /// `owner:group` — resolved names; numeric uid/gid used as fallback.
    ///
    /// NIST SP 800-53 AC-2, AC-3.
    UidGid,

    /// File size in bytes. Off by default.
    ///
    /// NIST SP 800-53 AU-3.
    Size,

    /// Modification time in `YYYY-MM-DD HH:MM` format.
    ///
    /// NIST SP 800-53 AU-3.
    Mtime,

    /// Inode number. Off by default.
    ///
    /// NIST SP 800-53 AU-3.
    Inode,

    /// Filename. Directories get a trailing `/`; executables get `*`.
    ///
    /// Always present as the final column. [`ColumnSet::without`] is a
    /// no-op for this variant — removing the filename would produce
    /// unusable output.
    ///
    Name,
}

// COLUMN SET - THIS IS WHERE WE CUSTOMIZE THE OUTPUT

/// An ordered, deduplicated selection of display columns.
///
/// Builder-pattern API:
/// ```ignore
/// let cols = ColumnSet::default()
///     .without(Column::Iov)
///     .with(Column::Size);
/// ```
///
/// [`Column::Name`] is always the final column and cannot be removed.
/// The internal [`Vec`] preserves explicit display order, ready to be bound
/// to a ratatui column-picker or a slint list model.
///
#[derive(Debug, Clone)]
pub struct ColumnSet(Vec<Column>);

impl Default for ColumnSet {
    /// Default column order:
    /// `Mode | Iov | SelinuxType | Marking | UidGid | Mtime | Name`.
    fn default() -> Self {
        Self(vec![
            Column::Mode,
            Column::Iov,
            Column::SelinuxType,
            Column::Marking,
            Column::UidGid,
            Column::Mtime,
            Column::Name,
        ])
    }
}

impl ColumnSet {
    /// Remove a column from the active set.
    ///
    /// [`Column::Name`] is silently preserved — removing the filename
    /// column would produce unusable output.
    #[must_use = "builder method returns the modified ColumnSet; the original is consumed"]
    pub fn without(mut self, col: Column) -> Self {
        if col != Column::Name {
            self.0.retain(|c| *c != col);
        }
        self
    }

    /// Add a column, inserting it immediately before [`Column::Name`].
    ///
    /// No-op if the column is already present.
    #[must_use = "builder method returns the modified ColumnSet; the original is consumed"]
    pub fn with(mut self, col: Column) -> Self {
        if !self.0.contains(&col) {
            let pos = self.0.iter().position(|c| *c == Column::Name).unwrap_or(self.0.len());
            self.0.insert(pos, col);
        }
        self
    }

    /// Returns `true` if the column is currently active.
    #[must_use = "pure accessor; callers that discard this cannot determine whether a column will appear in output"]
    pub fn contains(&self, col: Column) -> bool {
        self.0.contains(&col)
    }

    /// The active columns in display order.
    #[must_use = "pure accessor returning the ordered column slice used to drive display rendering"]
    pub fn columns(&self) -> &[Column] {
        &self.0
    }
}

/// List a directory, grouping entries by `(SELinux type, security marking)`.
///
/// Each group's entries are pre-sorted: directories first (alphabetical),
/// then all other entries (alphabetical). Groups are ordered lexicographically
/// by their [`GroupKey`] — type-alpha first, then marking-alpha.
///
/// Entries where `open()` was denied are returned by `SecureDirent::from_path`
/// as `Ok` with `access_denied: true`. `extract_group_key` maps these to the
/// `<restricted>` group, keeping them visible without aborting the listing.
///
/// ## Errors
///
/// Returns `Err` only if the directory stream itself cannot be opened
/// (`fs::read_dir` fails). Individual entry errors are logged and skipped.
///
/// NIST SP 800-53 AC-3, AC-4, AU-3 / NSA RTB Non-Bypassability & Provenance.
pub fn list_directory(dir_path: &Path) -> io::Result<DirListing> {
    let start = Instant::now();

    let mut map: BTreeMap<GroupKey, Vec<ListEntry>> = BTreeMap::new();
    let access_denied: Vec<String> = Vec::new();

    let read_dir = fs::read_dir(dir_path)?;

    for entry_result in read_dir {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                log::warn!("readdir error in {}: {e}", dir_path.display());
                continue;
            }
        };

        // Capture mtime cheaply from the DirEntry metadata (OS-cached in
        // most kernel versions; avoids an extra stat(2) call).
        let mtime = entry.metadata().ok().and_then(|m| m.modified().ok());

        let full_path = dir_path.join(entry.file_name());

        match SecureDirent::from_path(&full_path) {
            Ok(sd) => {
                let key = extract_group_key(&sd);
                map.entry(key).or_default().push(ListEntry {
                    dirent: sd,
                    mtime,
                });
            }
            Err(e) => {
                log::warn!("skipping {}: {e}", full_path.display());
            }
        }
    }

    // BTreeMap iteration is already lexicographically ordered by GroupKey.
    // Sort within each group: dirs first (alpha), then files (alpha).
    let groups = map
        .into_iter()
        .map(|(key, mut entries)| {
            entries.sort_by(|a, b| {
                let a_dir = a.dirent.file_type.is_directory();
                let b_dir = b.dirent.file_type.is_directory();
                b_dir.cmp(&a_dir).then_with(|| a.dirent.name.as_str().cmp(b.dirent.name.as_str()))
            });
            DirGroup {
                key,
                entries,
            }
        })
        .collect();

    let elapsed_us = u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX);

    Ok(DirListing {
        path: dir_path.to_path_buf(),
        groups,
        access_denied,
        elapsed_us,
    })
}

/// Derive the compound group key from a [`SecureDirent`].
///
/// If `entry.access_denied` is true, `open()` was refused and the inode-level
/// label cannot be verified. The entry is placed in the `<restricted>` group,
/// distinct from genuinely unlabeled objects where open succeeded but no
/// SELinux xattr was present.
///
/// For all other entries, acquires a single read-lock on [`GLOBAL_TRANSLATOR`]
/// per call, covering both the setrans lookup and the raw-level fallback.
///
/// NSA RTB Non-Bypassability; NIST SP 800-53 AC-4.
fn extract_group_key(entry: &SecureDirent) -> GroupKey {
    // If open() was denied we cannot verify the label at the inode level.
    // Mark as <restricted> — distinct from genuinely unlabeled objects.
    if entry.access_denied {
        return GroupKey {
            selinux_type: "<restricted>".to_string(),
            marking: "<restricted>".to_string(),
        };
    }

    // Derive the type column from SelinuxCtxState.
    // ParseFailure and TpiDisagreement use their own sentinel values so they
    // group separately from genuinely unlabeled objects.
    let selinux_type = entry.selinux_label.display_type();

    // Derive the marking/level column.
    // Only Labeled entries have a level; all other states use <no-level>.
    let marking = entry.selinux_label.level().map_or_else(
        || "<no-level>".to_owned(),
        |lvl| {
            let range = SecurityRange::from_level(lvl);
            GLOBAL_TRANSLATOR.read().map_or_else(
                |_| lvl.raw().to_owned(),
                |g| g.lookup(&range).unwrap_or_else(|| lvl.raw().to_owned()),
            )
        },
    );

    GroupKey {
        selinux_type,
        marking,
    }
}
