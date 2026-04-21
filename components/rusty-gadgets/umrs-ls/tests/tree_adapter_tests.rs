// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// Integration tests for `umrs_ls::tree_adapter`.
//
// TEST-ID: TREE-ADAPTER-001 through TREE-ADAPTER-007
// REQUIREMENT: TUI Phase 1 — DirListing → TreeModel conversion
// COMPLIANCE: NIST SP 800-53 AU-3 (audit record completeness)
//             NIST SP 800-53 AC-3 / AC-4 (access control / information flow)

use std::path::Path;
use std::time::SystemTime;

use umrs_ls::tree_adapter::{ScanStats, build_tree, compute_stats};
use umrs_selinux::SelinuxCtxState;
use umrs_selinux::fs_encrypt::EncryptionSource;
use umrs_selinux::posix::identity::LinuxOwnership;
use umrs_selinux::posix::primitives::{DevId, FileMode, FileSize, Gid, HardLinkCount, Inode, Uid};
use umrs_selinux::secure_dirent::{
    AbsolutePath, FileType, InodeSecurityFlags, SecureDirent, ValidatedFileName,
};
use umrs_selinux::utils::dirlist::{DirGroup, DirListing, GroupKey, ListEntry};

// ============================================================================
// Test helpers
// ============================================================================

fn make_entry_full(name: &str, size: u64, file_type: FileType) -> ListEntry {
    let path_str = format!("/test/{name}");
    let path = AbsolutePath::new(&path_str).expect("test path valid");
    let file_name = ValidatedFileName::new(name).expect("test name valid");
    let ownership = LinuxOwnership::from_ids(Uid::new(0), Gid::new(0));

    let dirent = SecureDirent {
        path,
        name: file_name,
        file_type,
        inode: Inode::new(1),
        size: FileSize::new(size),
        mode: FileMode::from_mode(0o644),
        nlink: HardLinkCount::new(1),
        dev: DevId::new(0),
        ownership,
        selinux_label: SelinuxCtxState::Unlabeled,
        sec_flags: InodeSecurityFlags::empty(),
        is_mountpoint: false,
        encryption: EncryptionSource::None,
        symlink_target: None,
        access_denied: false,
    };

    ListEntry {
        dirent,
        mtime: Some(SystemTime::UNIX_EPOCH),
    }
}

/// Regular file entry.
fn make_entry(name: &str, size: u64) -> ListEntry {
    make_entry_full(name, size, FileType::RegularFile)
}

/// Directory entry.
fn make_dir_entry(name: &str) -> ListEntry {
    make_entry_full(name, 0, FileType::Directory)
}

/// Construct a DirListing from a list of (selinux_type, marking, entries) tuples.
fn make_listing(groups: Vec<(&str, &str, Vec<ListEntry>)>) -> DirListing {
    let dir_groups = groups
        .into_iter()
        .map(|(selinux_type, marking, entries)| DirGroup {
            key: GroupKey {
                selinux_type: selinux_type.to_owned(),
                marking: marking.to_owned(),
            },
            entries,
        })
        .collect();

    DirListing {
        path: std::path::PathBuf::from("/test"),
        groups: dir_groups,
        access_denied: Vec::new(),
        elapsed_us: 100,
    }
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-001 — empty listing
// ============================================================================

#[test]
fn empty_listing_has_only_parent_nav() {
    let listing = make_listing(vec![]);
    let model = build_tree(&listing, Path::new("/home/user"), false);

    // Only the parent nav entry (no "." entry).
    assert_eq!(model.roots.len(), 1, "expected exactly 1 root (parent nav)");
    assert!(
        model.roots[0].label.contains("parent directory"),
        "roots[0] must be the parent nav entry; got: {}",
        model.roots[0].label
    );
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-002 — single group with files
// ============================================================================

#[test]
fn single_group_with_files_produces_expanded_branch() {
    let entries = vec![
        make_entry("alpha.conf", 100),
        make_entry("beta.conf", 200),
        make_entry("gamma.conf", 300),
    ];
    let listing = make_listing(vec![("httpd_t", "s0", entries)]);
    let model = build_tree(&listing, Path::new("/var/www"), false);

    // 1 parent nav + 1 group = 2 roots.
    assert_eq!(model.roots.len(), 2);

    let group_node = &model.roots[1];
    assert!(
        group_node.label.contains("httpd_t"),
        "group label should contain type: {}",
        group_node.label
    );
    assert!(
        group_node.label.contains("s0"),
        "group label should contain marking: {}",
        group_node.label
    );

    // Group starts expanded.
    assert!(
        group_node.expanded,
        "SELinux group node should start expanded"
    );

    // Three file children (no siblings, so all leaves).
    assert_eq!(group_node.children.len(), 3);
    for child in &group_node.children {
        assert!(child.is_leaf(), "standalone file should be a leaf node");
    }
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-003 — cuddled siblings
// ============================================================================

#[test]
fn cuddled_siblings_produce_expanded_branch_with_leaf_children() {
    // "known_hosts" will cuddle "known_hosts.old" and "known_hosts.bak".
    let entries = vec![
        make_entry("known_hosts", 1024),
        make_entry("known_hosts.bak", 1024),
        make_entry("known_hosts.old", 1024),
    ];
    let listing = make_listing(vec![("admin_home_t", "s0", entries)]);
    let model = build_tree(&listing, Path::new("/root"), false);

    let group_node = &model.roots[1];
    // The grouper should produce one FileGroup with 2 siblings.
    assert_eq!(group_node.children.len(), 1, "one FileGroup expected");

    let base_node = &group_node.children[0];
    // Base with siblings → branch node.
    assert!(!base_node.is_leaf(), "cuddled base should be a branch node");
    assert!(
        !base_node.expanded,
        "cuddled base should start collapsed for less clutter"
    );
    assert_eq!(base_node.children.len(), 2, "two sibling children expected");

    // Sibling children are leaves.
    for sib_child in &base_node.children {
        assert!(sib_child.is_leaf(), "sibling entry should be a leaf");
    }

    // Sibling metadata: sibling_kind should be set.
    for sib_child in &base_node.children {
        assert!(
            sib_child.metadata.contains_key("sibling_kind"),
            "sibling leaf should have sibling_kind metadata"
        );
    }

    // Base metadata: sibling_count and sibling_summary should be set.
    assert!(
        base_node.metadata.contains_key("sibling_count"),
        "cuddled base should have sibling_count metadata"
    );
    assert!(
        base_node.metadata.contains_key("sibling_summary"),
        "cuddled base should have sibling_summary metadata"
    );
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-004 — restricted group starts collapsed
// ============================================================================

#[test]
fn restricted_group_starts_expanded() {
    let entries = vec![make_entry(".gnupg", 4096)];
    let listing = make_listing(vec![
        ("httpd_t", "s0", vec![make_entry("index.html", 100)]),
        ("<restricted>", "<restricted>", entries),
    ]);
    let model = build_tree(&listing, Path::new("/home/user"), false);

    // Find the <restricted> group — it must be last.
    let last = model.roots.last().expect("at least one root beyond nav");
    assert!(
        last.label.contains("<restricted>"),
        "last root should be the restricted group; got: {}",
        last.label
    );
    assert!(
        last.expanded,
        "restricted group must start expanded so operators do not miss it"
    );
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-005 — dot and dotdot are always roots[0] and roots[1]
// ============================================================================

#[test]
fn parent_nav_is_always_first_root() {
    let entries = vec![make_entry("file.txt", 512)];
    let listing = make_listing(vec![("unlabeled_t", "s0", entries)]);
    let model = build_tree(&listing, Path::new("/some/dir"), false);

    assert!(
        model.roots[0].label.contains("parent directory"),
        "roots[0] must be the parent nav entry; got: {}",
        model.roots[0].label
    );
    assert_eq!(
        model.roots[0].metadata.get("name").map(String::as_str),
        Some(".."),
        "parent nav name metadata must be '..' for navigation"
    );
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-006 — directory entries have is_dir = "true"
// ============================================================================

#[test]
fn directory_entries_have_is_dir_true_metadata() {
    let entries = vec![make_dir_entry(".ssh"), make_entry("authorized_keys", 256)];
    let listing = make_listing(vec![("admin_home_t", "s0", entries)]);
    let model = build_tree(&listing, Path::new("/root"), false);

    let group_node = &model.roots[1];
    // 2 children: .ssh/ (dir) and authorized_keys (file).
    // grouper: both are standalone since ".ssh" and "authorized_keys" don't share prefix.
    assert_eq!(group_node.children.len(), 2);

    // Find the directory child (label ends with /).
    let dir_child = group_node
        .children
        .iter()
        .find(|c| c.label.ends_with('/'))
        .expect("should find a directory node");
    assert_eq!(
        dir_child.metadata.get("is_dir").map(String::as_str),
        Some("true"),
        "directory node should have is_dir=true"
    );
    assert_eq!(
        dir_child.metadata.get("file_type").map(String::as_str),
        Some("d"),
        "directory node should have file_type=d"
    );

    // Regular file child.
    let file_child = group_node
        .children
        .iter()
        .find(|c| !c.label.ends_with('/'))
        .expect("should find a regular file node");
    assert_eq!(
        file_child.metadata.get("is_dir").map(String::as_str),
        Some("false"),
        "file node should have is_dir=false"
    );
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-007 — compute_stats counts correctly
// ============================================================================

#[test]
fn compute_stats_counts_files_and_directories() {
    let group1_entries =
        vec![make_dir_entry("subdir"), make_entry("file1.txt", 100), make_entry("file2.txt", 200)];
    let group2_entries = vec![make_entry("other.conf", 50), make_dir_entry("etc")];
    let listing = make_listing(vec![
        ("type_a", "s0", group1_entries),
        ("type_b", "s0", group2_entries),
    ]);

    let stats: ScanStats = compute_stats(&listing);

    // group1: 1 dir + 2 files; group2: 1 file + 1 dir → 2 dirs, 3 files total.
    assert_eq!(stats.dir_count, 2, "expected 2 directories");
    assert_eq!(stats.file_count, 3, "expected 3 regular files");
    assert_eq!(stats.elapsed_us, 100, "elapsed_us should match listing");
}

// ============================================================================
// TEST-ID: TREE-ADAPTER-008 — group header nodes carry kind and marking metadata
// ============================================================================

/// Verify that SELinux group nodes carry the metadata fields required by the
/// label detail popup.
///
/// - `kind = "group_header"` — used by the event loop to identify group
///   header nodes and route Enter to the label detail popup.
/// - `marking` — the MCS marking string used to look up the catalog entry.
///
/// NIST SP 800-53 AC-16 — marking metadata on group nodes makes the full
/// label definition accessible via the detail popup without re-scanning.
#[test]
fn group_header_nodes_carry_kind_and_marking_metadata() {
    let entries = vec![make_entry("secret.conf", 512)];
    let listing = make_listing(vec![("httpd_t", "s0:c1,c5", entries)]);
    let model = build_tree(&listing, Path::new("/etc/httpd"), false);

    // roots[0] = parent nav, roots[1] = SELinux group.
    assert_eq!(model.roots.len(), 2);
    let group_node = &model.roots[1];

    assert_eq!(
        group_node.metadata.get("kind").map(String::as_str),
        Some("group_header"),
        "group node must carry kind=group_header for popup routing"
    );
    assert_eq!(
        group_node.metadata.get("marking").map(String::as_str),
        Some("s0:c1,c5"),
        "group node must carry the exact marking string for catalog lookup"
    );
}
