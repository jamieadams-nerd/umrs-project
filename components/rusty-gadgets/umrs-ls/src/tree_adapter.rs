// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)

//! # tree_adapter — DirListing → TreeModel Conversion
//!
//! Converts a [`DirListing`] (from `umrs_selinux::utils::dirlist`) into a
//! [`TreeModel`] (from `umrs_ui::viewer::tree`) for display in the interactive
//! TUI viewer.
//!
//! ## Tree Structure
//!
//! The produced tree has the following hierarchy:
//!
//! ```text
//! . (current dir)                      ← leaf (directory, triggers navigation)
//! .. (parent dir)                      ← leaf (directory, triggers navigation)
//! ▼ admin_home_t :: s0                 ← branch (SELinux group), expanded
//!   · .bashrc                          ← leaf (file)
//!   ▼ known_hosts                      ← branch (cuddled base), expanded
//!     · known_hosts.old [backup]       ← leaf (sibling)
//! ▶ <restricted> :: <restricted>       ← branch (denied entries), collapsed
//! ```
//!
//! ## Key Exported Types
//!
//! - [`build_tree`] — primary entry point; converts `DirListing` to `TreeModel`
//! - [`compute_stats`] — counts files and directories for the status bar
//! - [`ScanStats`] — the output of `compute_stats`
//!
//! ## Performance
//!
//! - Single pass over entries where possible.
//! - `Vec::with_capacity` pre-allocation when sizes are known.
//! - No I/O — no NSS lookups, no file reads. All data is already in `DirListing`.
//! - Timing logged at debug level under `#[cfg(debug_assertions)]`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Node metadata preserves mode bits and ownership
//!   for access enforcement context in the detail panel.
//! - **NIST SP 800-53 AC-4**: SELinux type and marking are carried as
//!   first-class metadata fields on every node.
//! - **NIST SP 800-53 AU-3**: Complete audit record fields — name, mode, owner,
//!   group, size, inode, SELinux context — are stored in node metadata.
//! - **NSA RTB**: Deterministic Execution — O(n) construction with no hidden
//!   allocations in the conversion path.

use std::collections::BTreeMap;
use std::path::Path;

#[cfg(debug_assertions)]
use std::time::Instant;

use umrs_selinux::ObservationKind;
use umrs_selinux::secure_dirent::{FileType, InodeSecurityFlags};
use umrs_selinux::utils::dirlist::{DirListing, ListEntry};
use umrs_ui::viewer::tree::{TreeModel, TreeNode};

use crate::grouping::{FileGroup, SiblingKind, group_entries, sibling_summary};

// ────────────────────────────────────────────────────────────────────────────
// ScanStats
// ────────────────────────────────────────────────────────────────────────────

/// Aggregate counts derived from a [`DirListing`], used by the status bar.
///
/// Computed by [`compute_stats`] in O(n) over all groups.
///
/// NIST SP 800-53 AU-3 — counts are audit-relevant: a directory with an
/// unexpectedly large number of restricted entries warrants investigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanStats {
    /// Number of regular (non-directory) entries across all groups.
    pub file_count: usize,
    /// Number of directory entries across all groups.
    pub dir_count: usize,
    /// Original listing elapsed time from [`DirListing::elapsed_us`].
    pub elapsed_us: u64,
}

// ────────────────────────────────────────────────────────────────────────────
// Restricted group sentinel
// ────────────────────────────────────────────────────────────────────────────

/// The SELinux type sentinel used by `dirlist.rs` for access-denied entries.
const RESTRICTED_TYPE: &str = "<restricted>";

// ────────────────────────────────────────────────────────────────────────────
// Public API
// ────────────────────────────────────────────────────────────────────────────

/// Convert a [`DirListing`] into a [`TreeModel`] ready for the TUI viewer.
///
/// `current_path` is used only to construct the `.` and `..` leaf entries.
/// No I/O is performed — all data is sourced from the already-collected
/// listing.
///
/// The `.` and `..` navigation entries are always the first two root nodes.
/// SELinux groups follow in the order they appear in `listing.groups`
/// (lexicographic by `GroupKey`). The `<restricted>` group, if present,
/// is placed at the end and starts collapsed.
///
/// After construction, [`TreeModel::rebuild_display`] is called automatically
/// before returning.
///
/// ## Performance
///
/// - `Vec::with_capacity` is used wherever the element count is predictable.
/// - `group_entries()` is O(n) single-pass per group.
/// - No NSS lookups, no file I/O.
///
/// NIST SP 800-53 AC-3 / AC-4 / AU-3 / NSA RTB Deterministic Execution.
#[must_use = "the returned TreeModel must be displayed; constructing and discarding it wastes work"]
pub fn build_tree(listing: &DirListing, current_path: &Path) -> TreeModel {
    #[cfg(debug_assertions)]
    let start = Instant::now();

    let mut model = TreeModel::new();

    // Pre-allocate: 1 nav entry (..) + number of groups.
    model.roots.reserve(listing.groups.len().saturating_add(1));

    // ── Navigation entry: parent directory ────────────────────────────────
    // "." is not shown as a tree entry — its metadata is displayed in the
    // header panel by the renderer.  ".." gets a special styled entry.
    let parent = current_path.parent().unwrap_or(current_path);
    model.roots.push(make_parent_nav_entry(parent));

    // ── Collect the <restricted> group separately so it goes last ───────────
    let mut restricted_group_node: Option<TreeNode> = None;

    for group in &listing.groups {
        let is_restricted = group.key.selinux_type == RESTRICTED_TYPE;
        let label = format!("{} :: {}", group.key.selinux_type, group.key.marking);
        let entry_count = group.entries.len();

        let mut group_node = TreeNode::branch(label, format!("{entry_count} entries"));
        group_node.expanded = true;

        // Run the cuddling grouper on this SELinux group's entries.
        let file_groups = group_entries(&group.entries);
        group_node.children.reserve(file_groups.len());

        for fg in &file_groups {
            let child = build_file_group_node(fg);
            group_node.children.push(child);
        }

        if is_restricted {
            restricted_group_node = Some(group_node);
        } else {
            model.roots.push(group_node);
        }
    }

    // Restricted group goes last and starts collapsed.
    if let Some(restricted) = restricted_group_node {
        model.roots.push(restricted);
    }

    model.rebuild_display();

    #[cfg(debug_assertions)]
    {
        let elapsed = start.elapsed();
        log::debug!(
            "tree_adapter::build_tree completed in {} µs ({} root nodes)",
            elapsed.as_micros(),
            model.roots.len(),
        );
    }

    model
}

/// Compute aggregate file and directory counts from a [`DirListing`].
///
/// Iterates all entries across all groups in a single pass. The
/// `elapsed_us` field is copied directly from the listing's own timing.
///
/// NIST SP 800-53 AU-3 — counts contribute to the status bar audit summary.
#[must_use = "ScanStats is the only output; discarding it means the status bar cannot display counts"]
pub fn compute_stats(listing: &DirListing) -> ScanStats {
    let mut file_count: usize = 0;
    let mut dir_count: usize = 0;

    for group in &listing.groups {
        for entry in &group.entries {
            if entry.dirent.file_type.is_directory() {
                dir_count = dir_count.saturating_add(1);
            } else {
                file_count = file_count.saturating_add(1);
            }
        }
    }

    ScanStats {
        file_count,
        dir_count,
        elapsed_us: listing.elapsed_us,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Private helpers
// ────────────────────────────────────────────────────────────────────────────

/// Build the `..` (parent directory) navigation leaf node.
///
/// Uses `\u{21EA}` (upwards arrow) as the display icon
/// and "parent directory" as the detail text for clarity.
///
/// The node carries `is_dir = "true"` and `name = ".."` so the event loop
/// triggers directory navigation when it is activated.
fn make_parent_nav_entry(parent_path: &Path) -> TreeNode {
    let path_str = parent_path.to_string_lossy().into_owned();
    let mut node = TreeNode::leaf(
        "\u{21EA} parent directory", // ICON_PARENT defined in tui_render.rs
        path_str.clone(),
    );
    node.metadata.insert("name".to_owned(), "..".to_owned());
    node.metadata.insert("is_dir".to_owned(), "true".to_owned());
    node.metadata.insert("file_type".to_owned(), "d".to_owned());
    node.metadata.insert("path".to_owned(), path_str);
    node.metadata.insert("is_parent_nav".to_owned(), "true".to_owned());
    node
}

/// Convert a [`FileGroup`] into a [`TreeNode`].
///
/// - If the group has no siblings, returns a leaf node.
/// - If the group has siblings, returns a branch node (expanded by default)
///   with the siblings as leaf children.
fn build_file_group_node(fg: &FileGroup) -> TreeNode {
    if fg.siblings.is_empty() {
        // Standalone file — leaf node.
        entry_leaf_node(&fg.base)
    } else {
        // Cuddled base — branch node, expanded by default.
        let summary = sibling_summary(fg);
        let sibling_count = fg.siblings.len();
        let label = entry_label(&fg.base);
        let detail = entry_detail(&fg.base);

        let mut node = TreeNode::branch(label, detail);
        // Cuddled siblings start collapsed — less clutter by default.
        // The operator can expand with ← → to see individual siblings.

        // Base-level metadata.
        populate_entry_metadata(&mut node.metadata, &fg.base);
        node.metadata.insert(
            "sibling_count".to_owned(),
            sibling_count.to_string(),
        );
        node.metadata.insert("sibling_summary".to_owned(), summary);

        // Sibling leaf children.
        node.children.reserve(fg.siblings.len());
        for sibling in &fg.siblings {
            let mut child = entry_leaf_node(&sibling.entry);
            child.metadata.insert(
                "sibling_kind".to_owned(),
                sibling_kind_str(&sibling.kind).to_owned(),
            );
            node.children.push(child);
        }

        node
    }
}

/// Construct a leaf node for a single [`ListEntry`].
fn entry_leaf_node(entry: &ListEntry) -> TreeNode {
    let label = entry_label(entry);
    let detail = entry_detail(entry);
    let mut node = TreeNode::leaf(label, detail);
    populate_entry_metadata(&mut node.metadata, entry);
    node
}

/// Display label for an entry — filename with trailing `/` for directories.
fn entry_label(entry: &ListEntry) -> String {
    let name = entry.dirent.name.as_str();
    if entry.dirent.file_type.is_directory() {
        format!("{name}/")
    } else {
        name.to_owned()
    }
}

/// Short detail string — size in bytes for files, empty for directories.
fn entry_detail(entry: &ListEntry) -> String {
    if entry.dirent.file_type.is_directory() {
        String::new()
    } else {
        entry.dirent.size.as_u64().to_string()
    }
}

/// Populate the standard metadata fields for a file or directory node.
///
/// Fields populated:
/// - `name`, `mode`, `owner`, `group`, `size`, `is_dir`, `file_type`
/// - `selinux_context` (full context string, if labeled)
/// - `selinux_type` (type component)
/// - `marking` (MCS marking or sentinel)
///
/// NIST SP 800-53 AU-3 — every node carries complete audit record fields.
/// NIST SP 800-53 AC-3 / AC-4 — SELinux context and mode bits are preserved.
fn populate_entry_metadata(meta: &mut BTreeMap<String, String>, entry: &ListEntry) {
    let dirent = &entry.dirent;

    meta.insert("name".to_owned(), dirent.name.as_str().to_owned());

    // Full mode string includes the file-type character prefix.
    let file_type_char = file_type_char(dirent.file_type);
    let mode_bits = dirent.mode.as_mode_str();
    meta.insert("mode".to_owned(), format!("{file_type_char}{mode_bits}"));

    // Resolve uid/gid to username/groupname via NSS.  Falls back to the
    // numeric id string if NSS resolution fails (e.g., unmapped UIDs).
    let uid = dirent.ownership.user.uid.as_u32();
    let gid = dirent.ownership.group.gid.as_u32();
    meta.insert("owner".to_owned(), resolve_username(uid));
    meta.insert("group".to_owned(), resolve_groupname(gid));

    meta.insert("size".to_owned(), dirent.size.as_u64().to_string());

    // SELinux label fields.
    let (ctx_string, type_string, marking_string) = extract_selinux_strings(dirent);
    meta.insert("selinux_context".to_owned(), ctx_string);
    meta.insert("selinux_type".to_owned(), type_string);
    meta.insert("marking".to_owned(), marking_string);

    // Directory / navigation flag.
    let is_dir = dirent.file_type.is_directory();
    meta.insert("is_dir".to_owned(), if is_dir { "true" } else { "false" }.to_owned());
    meta.insert("file_type".to_owned(), file_type_char.to_owned());

    // Mountpoint and encryption flags — used by the TUI renderer for icon selection.
    if dirent.is_mountpoint {
        meta.insert("is_mountpoint".to_owned(), "true".to_owned());
    }
    if dirent.has_encryption() {
        meta.insert("has_encryption".to_owned(), "true".to_owned());
    }

    // IOV security-posture flags — serialised for the TUI renderer so the
    // I/O/V column can match the CLI output.  `iov_i` = immutable bit set,
    // `iov_v` = IMA signature present, `iov_o` = at least one Risk-kind
    // security observation.  Keys are only inserted when true so absence
    // is equivalent to "unset/clear" in the renderer's metadata lookup.
    // NIST SP 800-53 AU-3, SI-7 — posture markers are audit-relevant.
    let flags = &entry.dirent.sec_flags;
    if flags.contains(InodeSecurityFlags::IMMUTABLE) {
        meta.insert("iov_i".to_owned(), "true".to_owned());
    }
    if flags.contains(InodeSecurityFlags::IMA_PRESENT) {
        meta.insert("iov_v".to_owned(), "true".to_owned());
    }
    // Highest-severity observation drives the O column:
    //   Risk    → red bold flag
    //   Warning → yellow flag
    //   Good    → green dim (not yet rendered — omitted)
    //   none    → dim dash
    // We store the tier as a string so the renderer can pick the glyph/color
    // without re-deriving it from `SecurityObservation`.
    let mut iov_o_tier: Option<&str> = None;
    for o in entry.dirent.security_observations() {
        match o.kind() {
            ObservationKind::Risk => {
                iov_o_tier = Some("risk");
                break;
            }
            ObservationKind::Warning => iov_o_tier = Some("warning"),
            ObservationKind::Good => {}
        }
    }
    if let Some(tier) = iov_o_tier {
        meta.insert("iov_o".to_owned(), tier.to_owned());
    }

    // Mtime — stored as seconds since epoch for display in the TUI listing.
    // Stored on the node so the renderer can format it without I/O.
    // NIST SP 800-53 AU-3 — modification time is a required audit record field.
    if let Some(mtime) = entry.mtime
        && let Ok(dur) = mtime.duration_since(std::time::UNIX_EPOCH)
    {
        meta.insert("mtime_secs".to_owned(), dur.as_secs().to_string());
    }
}

/// Extract display strings from a `SecureDirent`'s SELinux label state.
///
/// Returns `(full_context, selinux_type, marking)`.  All three fields have
/// well-defined sentinel values for non-labeled states so callers need not
/// handle `Option`.
fn extract_selinux_strings(
    dirent: &umrs_selinux::secure_dirent::SecureDirent,
) -> (String, String, String) {
    match &dirent.selinux_label {
        umrs_selinux::SelinuxCtxState::Labeled(ctx) => {
            let full = ctx.to_string();
            let selinux_type = ctx.security_type().to_string();
            let marking = ctx
                .level()
                .map_or_else(|| "<no-level>".to_owned(), |l| l.raw().to_owned());
            (full, selinux_type, marking)
        }
        umrs_selinux::SelinuxCtxState::Unlabeled => (
            "<unlabeled>".to_owned(),
            "<unlabeled>".to_owned(),
            "<no-level>".to_owned(),
        ),
        umrs_selinux::SelinuxCtxState::ParseFailure => (
            "<parse-error>".to_owned(),
            "<parse-error>".to_owned(),
            "<no-level>".to_owned(),
        ),
        umrs_selinux::SelinuxCtxState::TpiDisagreement => (
            "<unverifiable>".to_owned(),
            "<unverifiable>".to_owned(),
            "<no-level>".to_owned(),
        ),
    }
}

/// Single-character file type code used in the mode column (`ls -l` style).
///
/// Public so `viewer_app::DirMeta` can format the directory's own mode string.
#[must_use = "file type character is needed for mode string formatting"]
pub const fn file_type_char_pub(ft: FileType) -> &'static str {
    file_type_char(ft)
}

/// Single-character file type code used in the mode column (`ls -l` style).
const fn file_type_char(ft: FileType) -> &'static str {
    match ft {
        FileType::Directory => "d",
        FileType::Symlink => "l",
        FileType::BlockDevice => "b",
        FileType::CharDevice => "c",
        FileType::Fifo => "p",
        FileType::Socket => "s",
        FileType::RegularFile | FileType::Unknown => "-",
    }
}

/// Short lowercase string for a `SiblingKind`, stored in node metadata.
const fn sibling_kind_str(kind: &SiblingKind) -> &'static str {
    match kind {
        SiblingKind::Rotation => "rotation",
        SiblingKind::CompressedRotation => "compressed_rotation",
        SiblingKind::Signature => "signature",
        SiblingKind::Checksum => "checksum",
        SiblingKind::Backup => "backup",
        SiblingKind::Related => "related",
    }
}

/// Resolve a numeric UID to a username string.
///
/// Falls back to the numeric UID as a string if NSS resolution fails
/// (e.g., unmapped UIDs from network filesystems).
fn resolve_username(uid: u32) -> String {
    match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(uid)) {
        Ok(Some(u)) => u.name,
        _ => uid.to_string(),
    }
}

/// Resolve a numeric GID to a groupname string.
///
/// Falls back to the numeric GID as a string if NSS resolution fails.
fn resolve_groupname(gid: u32) -> String {
    match nix::unistd::Group::from_gid(nix::unistd::Gid::from_raw(gid)) {
        Ok(Some(g)) => g.name,
        _ => gid.to_string(),
    }
}
