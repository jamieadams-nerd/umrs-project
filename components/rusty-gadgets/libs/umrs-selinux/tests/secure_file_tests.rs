// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// Integration tests for `umrs_selinux::secure_file`.
//
// Tests operate on real temporary files created by `tempfile` so that
// SecureDirent construction (ioctl, xattr paths) exercises real kernel paths.
//
// ## Coverage
//
// - `read_bytes`: success, ContentTooLarge, NotRegularFile
// - `read_to_string`: success, InvalidUtf8
// - `compute_digests`: known-input SHA-256 and SHA-384 verification
// - `read_magic::<N>`: success, FileTooSmall
// - `SecureDirent::symlink_target`: populated for symlinks, None for regular files

use std::io::Write as _;
use std::os::unix::fs::symlink;

use tempfile::{NamedTempFile, TempDir};
use umrs_selinux::secure_dirent::SecDirError;
use umrs_selinux::secure_file;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a named temp file with the given content.
fn temp_file_with_content(content: &[u8]) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("tempfile");
    f.write_all(content).expect("write temp content");
    f
}

// ---------------------------------------------------------------------------
// read_bytes
// ---------------------------------------------------------------------------

#[test]
fn read_bytes_returns_content_and_dirent() {
    let f = temp_file_with_content(b"hello world");
    let (dirent, bytes) = secure_file::read_bytes(f.path(), 1024)
        .expect("read_bytes should succeed");
    assert_eq!(bytes, b"hello world");
    assert!(!dirent.access_denied, "should not be access denied");
}

#[test]
fn read_bytes_empty_file_succeeds() {
    let f = temp_file_with_content(b"");
    let (_dirent, bytes) = secure_file::read_bytes(f.path(), 1024)
        .expect("read_bytes on empty file should succeed");
    assert!(bytes.is_empty());
}

#[test]
fn read_bytes_content_too_large() {
    let f = temp_file_with_content(b"more than one byte");
    let result = secure_file::read_bytes(f.path(), 1);
    match result {
        Err(SecDirError::ContentTooLarge { .. }) => {}
        other => panic!("expected ContentTooLarge, got: {other:?}"),
    }
}

#[test]
fn read_bytes_not_regular_file_on_directory() {
    let dir = TempDir::new().expect("tempdir");
    let result = secure_file::read_bytes(dir.path(), 1024);
    match result {
        Err(SecDirError::NotRegularFile) => {}
        other => panic!("expected NotRegularFile, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// read_to_string
// ---------------------------------------------------------------------------

#[test]
fn read_to_string_returns_utf8_content() {
    let f = temp_file_with_content(b"hello UTF-8 world");
    let (_dirent, s) = secure_file::read_to_string(f.path(), 1024)
        .expect("read_to_string should succeed");
    assert_eq!(s, "hello UTF-8 world");
}

#[test]
fn read_to_string_invalid_utf8_returns_error() {
    // 0xFF is not valid UTF-8.
    let f = temp_file_with_content(&[0xFF, 0xFE]);
    let result = secure_file::read_to_string(f.path(), 1024);
    match result {
        Err(SecDirError::InvalidUtf8(_)) => {}
        other => panic!("expected InvalidUtf8, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// compute_digests — known-input verification
//
// SHA-256("a") = ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
// SHA-384("a") = 54a59b9f22b0b80880d8427e548b7c23abd873486e1f035dce9cd697e85175033caa88e6d57bc35efae0b5afd3145f31
//                (first 15 chars sufficient to distinguish from collision)
//
// References: FIPS 180-4 test vectors.
// ---------------------------------------------------------------------------

#[test]
fn compute_digests_known_input_single_byte() {
    let f = temp_file_with_content(b"a");
    let (_dirent, sha256, sha384) =
        secure_file::compute_digests(f.path(), secure_file::MAX_DIGEST_BYTES)
            .expect("compute_digests should succeed");

    // Verify SHA-256 array matches known vector.
    let expected256 = hex_decode("ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb");
    assert_eq!(sha256, expected256.as_slice(), "SHA-256 mismatch for input 'a'");

    // Verify SHA-384 array matches known vector.
    let expected384 = hex_decode(
        "54a59b9f22b0b80880d8427e548b7c23abd873486e1f035dce9cd697e85175033caa88e6d57bc35efae0b5afd3145f31",
    );
    assert_eq!(sha384, expected384.as_slice(), "SHA-384 mismatch for input 'a'");
}

#[test]
fn compute_digests_returns_correct_array_sizes() {
    let f = temp_file_with_content(b"test");
    let (_dirent, sha256, sha384) =
        secure_file::compute_digests(f.path(), secure_file::MAX_DIGEST_BYTES)
            .expect("compute_digests should succeed");
    assert_eq!(sha256.len(), 32, "SHA-256 must be 32 bytes");
    assert_eq!(sha384.len(), 48, "SHA-384 must be 48 bytes");
}

#[test]
fn compute_digests_content_too_large() {
    let f = temp_file_with_content(b"more than zero bytes");
    let result = secure_file::compute_digests(f.path(), 1);
    match result {
        Err(SecDirError::ContentTooLarge { .. }) => {}
        other => panic!("expected ContentTooLarge, got: {other:?}"),
    }
}

/// Decode a lowercase hex string into a `Vec<u8>`.
fn hex_decode(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("valid hex digit"))
        .collect()
}

// ---------------------------------------------------------------------------
// read_magic
// ---------------------------------------------------------------------------

#[test]
fn read_magic_returns_first_n_bytes() {
    let f = temp_file_with_content(b"\x7fELF extra data here");
    let (_dirent, magic) = secure_file::read_magic::<4>(f.path())
        .expect("read_magic should succeed");
    assert_eq!(&magic, b"\x7fELF");
}

#[test]
fn read_magic_file_too_small_returns_error() {
    let f = temp_file_with_content(b"hi");
    let result = secure_file::read_magic::<20>(f.path());
    match result {
        Err(SecDirError::FileTooSmall { size, wanted }) => {
            assert_eq!(size, 2, "size should be 2 bytes");
            assert_eq!(wanted, 20, "wanted should be 20 bytes");
        }
        other => panic!("expected FileTooSmall, got: {other:?}"),
    }
}

#[test]
fn read_magic_exactly_n_bytes_succeeds() {
    let f = temp_file_with_content(b"ABCD");
    let (_dirent, magic) = secure_file::read_magic::<4>(f.path())
        .expect("file with exactly 4 bytes should succeed");
    assert_eq!(&magic, b"ABCD");
}

// ---------------------------------------------------------------------------
// SecureDirent::symlink_target
// ---------------------------------------------------------------------------

#[test]
fn symlink_target_populated_for_symlink() {
    let dir = TempDir::new().expect("tempdir");
    let target_path = dir.path().join("real_file.txt");
    std::fs::write(&target_path, b"content").expect("write real file");

    let link_path = dir.path().join("the_link");
    symlink(&target_path, &link_path).expect("create symlink");

    let dirent = umrs_selinux::secure_dirent::SecureDirent::from_path(&link_path)
        .expect("from_path should succeed on symlink");

    assert!(
        dirent.symlink_target.is_some(),
        "symlink_target must be Some for a symlink"
    );
    let resolved = dirent.symlink_target.expect("just checked is_some");
    assert_eq!(resolved, target_path, "symlink_target must equal the target path");
}

#[test]
fn symlink_target_none_for_regular_file() {
    let f = temp_file_with_content(b"regular file");
    let dirent = umrs_selinux::secure_dirent::SecureDirent::from_path(f.path())
        .expect("from_path should succeed on regular file");
    assert!(
        dirent.symlink_target.is_none(),
        "symlink_target must be None for a regular file"
    );
}
