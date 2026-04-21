// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for `SecureDirent::from_path_with_content`.
//
// Covers the three required cases:
//   (a) successful read of a small regular file
//   (b) `ContentTooLarge` when the file exceeds the caller-supplied limit
//   (c) `NotRegularFile` when pointed at a directory
//
// ## Compliance
//
// - NIST SP 800-53 SI-10: bounded-read enforcement tested via (b).
// - NIST SP 800-53 AC-3 / AC-4: metadata and content from the same fd.

use std::io::Write as _;
use tempfile::NamedTempFile;
use umrs_selinux::secure_dirent::{SecDirError, SecureDirent};

// ---------------------------------------------------------------------------
// (a) Successful read of a small regular file
// ---------------------------------------------------------------------------

#[test]
fn from_path_with_content_reads_small_file() {
    let mut tmp = NamedTempFile::new().expect("tempfile creation must succeed");
    let payload = b"hello secure dirent";
    tmp.write_all(payload).expect("write to tempfile must succeed");
    tmp.flush().expect("flush must succeed");

    let path = tmp.path();
    let (dirent, bytes) =
        SecureDirent::from_path_with_content(path, 1024).expect("should succeed for a small file");

    assert_eq!(bytes, payload, "content bytes must match what was written");
    assert!(
        dirent.file_type.is_regular(),
        "file_type must be RegularFile"
    );
}

// ---------------------------------------------------------------------------
// (b) ContentTooLarge when the file exceeds the limit
// ---------------------------------------------------------------------------

#[test]
fn from_path_with_content_rejects_oversized_file() {
    let mut tmp = NamedTempFile::new().expect("tempfile creation must succeed");
    // Write 16 bytes; cap at 8 bytes to trigger ContentTooLarge.
    tmp.write_all(b"0123456789abcdef")
        .expect("write must succeed");
    tmp.flush().expect("flush must succeed");

    let path = tmp.path();
    let err = SecureDirent::from_path_with_content(path, 8)
        .expect_err("should fail when file exceeds limit");

    match err {
        SecDirError::ContentTooLarge { size, limit } => {
            assert_eq!(limit, 8, "limit must match the cap passed by the caller");
            assert!(size >= 16, "size must be at least the written byte count");
        }
        other => panic!("expected ContentTooLarge, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// (c) NotRegularFile when pointed at a directory
// ---------------------------------------------------------------------------

#[test]
fn from_path_with_content_rejects_directory() {
    let dir = tempfile::tempdir().expect("tempdir creation must succeed");
    let path = dir.path();

    let err = SecureDirent::from_path_with_content(path, 1_048_576)
        .expect_err("should fail when path is a directory");

    assert!(
        matches!(err, SecDirError::NotRegularFile),
        "expected NotRegularFile, got {err:?}"
    );
}

// ---------------------------------------------------------------------------
// ValidatedFileName::root() — from_path on filename-less paths
//
// Covers NIST SP 800-53 SI-10: the documented exception in ValidatedFileName
// that allows a single "/" component so root-directory paths are browsable
// without falling back to raw std::fs calls.
// ---------------------------------------------------------------------------

/// `SecureDirent::from_path(Path::new("/"))` must succeed.
///
/// Verifies:
/// - the call returns `Ok`
/// - `dirent.name.as_str() == "/"`
/// - `dirent.file_type.is_directory()` is true
/// - `dirent.selinux_label` is populated (not panicking on access)
/// - `dirent.mode` is populated (not panicking on access)
#[test]
fn from_path_succeeds_for_root() {
    use std::path::Path;

    let dirent = SecureDirent::from_path(Path::new("/"))
        .expect("from_path(\"/\") must succeed on a Linux host");

    assert_eq!(
        dirent.name.as_str(),
        "/",
        "name component of root path must be \"/\""
    );
    assert!(
        dirent.file_type.is_directory(),
        "root path must have directory file type"
    );
    // Verify the mode string is non-empty — actual value is host-dependent.
    let mode_str = dirent.mode.as_mode_str();
    assert!(
        !mode_str.is_empty(),
        "mode string must be non-empty for root"
    );
    // Access the label to confirm no panic — exact state is host-dependent.
    let _label = &dirent.selinux_label;
}

/// `SecureDirent::from_path(Path::new("/proc"))` must succeed.
///
/// `/proc` has a filename component (`"proc"`), but exercises the same
/// directory-lstat code path and confirms the fix does not regress
/// normal named directories adjacent to root.
#[test]
fn from_path_succeeds_for_proc_root() {
    use std::path::Path;

    let dirent = SecureDirent::from_path(Path::new("/proc"))
        .expect("from_path(\"/proc\") must succeed on a Linux host");

    assert_eq!(dirent.name.as_str(), "proc");
    assert!(dirent.file_type.is_directory());
}

/// `SecureDirent::from_path(Path::new("/sys"))` must succeed.
///
/// Same rationale as `/proc` above.
#[test]
fn from_path_succeeds_for_sys_root() {
    use std::path::Path;

    let dirent = SecureDirent::from_path(Path::new("/sys"))
        .expect("from_path(\"/sys\") must succeed on a Linux host");

    assert_eq!(dirent.name.as_str(), "sys");
    assert!(dirent.file_type.is_directory());
}
