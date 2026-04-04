// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// Integration tests for `umrs_ls::viewer_app`.
//
// TEST-ID: VIEWER-APP-001 through VIEWER-APP-007
// REQUIREMENT: TUI Phase 2 — DirViewerApp implements ViewerApp for umrs-ls
// COMPLIANCE: NIST SP 800-53 AU-3 (header carries tool identity and counts)
//             NIST SP 800-53 AC-3 / AC-4 (read-only viewer interface)

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use umrs_ls::viewer_app::DirViewerApp;
use umrs_selinux::SelinuxCtxState;
use umrs_selinux::fs_encrypt::EncryptionSource;
use umrs_selinux::posix::identity::LinuxOwnership;
use umrs_selinux::posix::primitives::{DevId, FileMode, FileSize, Gid, HardLinkCount, Inode, Uid};
use umrs_selinux::secure_dirent::{
    AbsolutePath, FileType, InodeSecurityFlags, SecureDirent, ValidatedFileName,
};
use umrs_selinux::utils::dirlist::{DirGroup, DirListing, GroupKey, ListEntry};
use umrs_ui::viewer::ViewerApp;

// ============================================================================
// Test helpers
// ============================================================================

fn make_entry(name: &str, size: u64, file_type: FileType) -> ListEntry {
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
        access_denied: false,
    };

    ListEntry {
        dirent,
        mtime: Some(SystemTime::UNIX_EPOCH),
    }
}

fn make_file(name: &str) -> ListEntry {
    make_entry(name, 512, FileType::RegularFile)
}

fn make_dir(name: &str) -> ListEntry {
    make_entry(name, 0, FileType::Directory)
}

/// Build a minimal synthetic `DirListing` for testing.
///
/// Contains two groups:
/// - `"bin_t :: s0"` — 3 regular files
/// - `"usr_t :: s0"` — 1 directory + 1 file
fn make_listing(path: &Path) -> DirListing {
    let group_a = DirGroup {
        key: GroupKey {
            selinux_type: "bin_t".to_owned(),
            marking: "s0".to_owned(),
        },
        entries: vec![
            make_file("cat"),
            make_file("ls"),
            make_file("sh"),
        ],
    };

    let group_b = DirGroup {
        key: GroupKey {
            selinux_type: "usr_t".to_owned(),
            marking: "s0".to_owned(),
        },
        entries: vec![
            make_dir("lib"),
            make_file("hello.so"),
        ],
    };

    DirListing {
        path: path.to_path_buf(),
        groups: vec![group_a, group_b],
        access_denied: vec![],
        elapsed_us: 1_500,
    }
}

// ============================================================================
// TEST-ID: VIEWER-APP-001
// card_title returns the expected constant.
// ============================================================================

#[test]
fn viewer_app_card_title_is_umrs_directory_security() {
    let path = PathBuf::from("/test");
    let listing = make_listing(&path);
    let app = DirViewerApp::from_listing(path, listing);

    assert_eq!(app.card_title(), "UMRS Directory Security");
}

// ============================================================================
// TEST-ID: VIEWER-APP-002
// tabs() returns exactly one tab labelled "Directory".
// ============================================================================

#[test]
fn viewer_app_tabs_contains_exactly_one_directory_tab() {
    let path = PathBuf::from("/test");
    let listing = make_listing(&path);
    let app = DirViewerApp::from_listing(path, listing);

    let tabs = app.tabs();
    assert_eq!(tabs.len(), 1, "expected exactly one tab");
    assert_eq!(tabs[0].label, "Directory");
}

// ============================================================================
// TEST-ID: VIEWER-APP-003
// status() text contains "files", "directories", and "ms".
// ============================================================================

#[test]
fn viewer_app_status_text_contains_files_directories_and_ms() {
    let path = PathBuf::from("/test");
    let listing = make_listing(&path);
    let app = DirViewerApp::from_listing(path, listing);

    let status = app.status();
    assert!(
        status.text.contains("files"),
        "status text should contain 'files', got: {:?}",
        status.text
    );
    assert!(
        status.text.contains("directories"),
        "status text should contain 'directories', got: {:?}",
        status.text
    );
    assert!(
        status.text.contains("ms"),
        "status text should contain 'ms', got: {:?}",
        status.text
    );
}

// ============================================================================
// TEST-ID: VIEWER-APP-004
// status() text shows correct file and directory counts.
// ============================================================================

#[test]
fn viewer_app_status_text_shows_correct_counts() {
    let path = PathBuf::from("/test");
    let listing = make_listing(&path);
    // Listing has: 3 + 1 = 4 files, 1 directory.
    let app = DirViewerApp::from_listing(path, listing);

    let status = app.status();
    // 4 regular files (3 in bin_t, 1 in usr_t) + 1 directory (lib in usr_t)
    assert!(
        status.text.contains("4 files"),
        "expected '4 files' in status text, got: {:?}",
        status.text
    );
    assert!(
        status.text.contains("1 director"),
        "expected '1 director' in status text, got: {:?}",
        status.text
    );
}

// ============================================================================
// TEST-ID: VIEWER-APP-005
// viewer_header() tool_name is "umrs-ls", data_source contains the path.
// ============================================================================

#[test]
fn viewer_app_header_tool_name_and_data_source() {
    let path = PathBuf::from("/srv/data");
    let listing = make_listing(&path);
    let app = DirViewerApp::from_listing(path, listing);

    let header = app.viewer_header();
    assert_eq!(header.tool_name, "umrs-ls");
    assert!(
        header.data_source.contains("/srv/data"),
        "data_source should contain the path, got: {:?}",
        header.data_source
    );
}

// ============================================================================
// TEST-ID: VIEWER-APP-006
// initial_tree() always returns Some.
// ============================================================================

#[test]
fn viewer_app_initial_tree_is_some() {
    let path = PathBuf::from("/test");
    let listing = make_listing(&path);
    let app = DirViewerApp::from_listing(path, listing);

    assert!(
        app.initial_tree().is_some(),
        "initial_tree() must always return Some"
    );
}

// ============================================================================
// TEST-ID: VIEWER-APP-007
// Sub-millisecond elapsed_us renders as "<1ms" in the status bar.
// ============================================================================

#[test]
fn viewer_app_status_sub_millisecond_shows_less_than_1ms() {
    let path = PathBuf::from("/test");
    // Build a listing with elapsed_us < 1000 to trigger the "<1ms" branch.
    let mut listing = make_listing(&path);
    listing.elapsed_us = 500;
    let app = DirViewerApp::from_listing(path, listing);

    let status = app.status();
    assert!(
        status.text.starts_with("<1ms"),
        "expected status to start with '<1ms', got: {:?}",
        status.text
    );
}

// ============================================================================
// TEST-ID: VIEWER-APP-008
// navigate_to() updates stats and returns a tree model (requires I/O).
// ============================================================================

#[test]
#[ignore = "requires a real filesystem directory; run manually or in integration env"]
fn viewer_app_navigate_to_updates_stats() {
    let mut app = DirViewerApp::scan(std::path::Path::new("/tmp"))
        .expect("scan /tmp failed");

    let original_path = app.current_path().to_path_buf();

    // Navigate to /tmp itself (same directory — guaranteed to work in test env).
    let tree = app
        .navigate_to(std::path::Path::new("/tmp"))
        .expect("navigate_to /tmp failed");

    // After navigation the path is updated.
    assert_eq!(app.current_path(), std::path::Path::new("/tmp"));
    // The original was also /tmp — this is a self-navigation; path should be the same.
    assert_eq!(original_path, app.current_path());
    // Tree must be non-empty (root nodes: . and ..).
    assert!(
        tree.roots.len() >= 2,
        "expected at least 2 root nodes (. and ..)"
    );
}

// ============================================================================
// TEST-ID: VIEWER-APP-009
// scan() succeeds on /tmp and returns sensible stats (requires I/O).
// ============================================================================

#[test]
#[ignore = "requires a real filesystem directory; run manually or in integration env"]
fn viewer_app_scan_tmp_returns_ok() {
    let app = DirViewerApp::scan(std::path::Path::new("/tmp"))
        .expect("DirViewerApp::scan(/tmp) failed");

    assert_eq!(app.current_path(), std::path::Path::new("/tmp"));
    // Status bar text should be well-formed.
    let status = app.status();
    assert!(status.text.contains("ms"), "status should contain 'ms'");
    assert!(
        status.text.contains("files"),
        "status should contain 'files'"
    );
}
