// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// Integration tests for `umrs_ls::grouping`.
//
// Tests are split into three layers:
//   1. `is_sibling()` — separator check invariant (pure string logic)
//   2. `classify_suffix()` — kind classification (pure string logic)
//   3. `group_entries()` — full grouping pipeline (requires ListEntry values)
//
// TEST-ID: GROUPING-001 through GROUPING-030
// REQUIREMENT: File cuddling — compact view for umrs-ls
// COMPLIANCE: NIST SP 800-53 AU-3 (audit record completeness preserved)

use umrs_ls::grouping::{
    SiblingKind, aggregate_size, classify_suffix, group_entries, is_sibling, sibling_summary,
};

// ============================================================================
// Helpers
// ============================================================================

use std::time::SystemTime;

use umrs_selinux::SelinuxCtxState;
use umrs_selinux::fs_encrypt::EncryptionSource;
use umrs_selinux::posix::identity::LinuxOwnership;
use umrs_selinux::posix::primitives::{DevId, FileMode, FileSize, Gid, HardLinkCount, Inode, Uid};
use umrs_selinux::secure_dirent::{
    AbsolutePath, FileType, InodeSecurityFlags, SecureDirent, ValidatedFileName,
};
use umrs_selinux::utils::dirlist::ListEntry;

/// Construct a minimal `ListEntry` with the given filename and byte size.
///
/// All security-irrelevant fields are set to safe defaults.  This helper
/// exists solely to drive grouping logic tests — do not use the resulting
/// entry for any display or security-decision path.
fn make_entry(name: &str, size: u64) -> ListEntry {
    let path_str = format!("/test/{name}");
    let path = AbsolutePath::new(&path_str).expect("test path valid");
    let file_name = ValidatedFileName::new(name).expect("test name valid");
    let ownership = LinuxOwnership::from_ids(Uid::new(0), Gid::new(0));

    let dirent = SecureDirent {
        path,
        name: file_name,
        file_type: FileType::RegularFile,
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

// ============================================================================
// Layer 1: is_sibling() — separator check
// ============================================================================

// TEST-ID: GROUPING-001
#[test]
fn is_sibling_dot_separator() {
    assert!(is_sibling("boot.log", "boot.log.1"));
}

// TEST-ID: GROUPING-002
#[test]
fn is_sibling_dash_separator() {
    assert!(is_sibling("boot.log", "boot.log-20260301"));
}

// TEST-ID: GROUPING-003
#[test]
fn is_sibling_underscore_separator() {
    assert!(is_sibling("access", "access_log"));
}

// TEST-ID: GROUPING-004 — separator check prevents false positives
#[test]
fn is_sibling_no_false_positive_same_prefix_no_separator() {
    // "file.log" must NOT absorb "file.logging"
    assert!(!is_sibling("file.log", "file.logging"));
}

// TEST-ID: GROUPING-005
#[test]
fn is_sibling_candidate_shorter_than_base() {
    assert!(!is_sibling("boot.log.1", "boot.log"));
}

// TEST-ID: GROUPING-006
#[test]
fn is_sibling_equal_names_not_sibling() {
    assert!(!is_sibling("boot.log", "boot.log"));
}

// TEST-ID: GROUPING-007
#[test]
fn is_sibling_unrelated_names() {
    assert!(!is_sibling("syslog", "messages"));
}

// TEST-ID: GROUPING-008
#[test]
fn is_sibling_compressed_rotation_is_sibling() {
    assert!(is_sibling("syslog", "syslog.1.gz"));
}

// TEST-ID: GROUPING-008a
// Regression: `jvm-common` must NOT cuddle under `jvm`. A `-` separator
// is only valid when followed by an ASCII digit (rotation / date suffix).
#[test]
fn is_sibling_dash_requires_digit_suffix() {
    assert!(!is_sibling("jvm", "jvm-common"));
    assert!(!is_sibling("foo", "foo-bar"));
    // Dash-digit rotations still work.
    assert!(is_sibling("boot.log", "boot.log-1"));
    assert!(is_sibling("boot.log", "boot.log-20260301"));
}

// ============================================================================
// Layer 2: classify_suffix() — kind classification
// ============================================================================

// TEST-ID: GROUPING-009
#[test]
fn classify_dot_number_is_rotation() {
    assert_eq!(classify_suffix(".1"), SiblingKind::Rotation);
    assert_eq!(classify_suffix(".2"), SiblingKind::Rotation);
    assert_eq!(classify_suffix(".10"), SiblingKind::Rotation);
}

// TEST-ID: GROUPING-010
#[test]
fn classify_dash_date_is_rotation() {
    assert_eq!(classify_suffix("-20260301"), SiblingKind::Rotation);
    assert_eq!(classify_suffix("-20251210"), SiblingKind::Rotation);
}

// TEST-ID: GROUPING-011
#[test]
fn classify_gz_extension_is_compressed() {
    assert_eq!(classify_suffix(".gz"), SiblingKind::CompressedRotation);
    assert_eq!(classify_suffix(".1.gz"), SiblingKind::CompressedRotation);
    assert_eq!(
        classify_suffix("-20260301.gz"),
        SiblingKind::CompressedRotation
    );
}

// TEST-ID: GROUPING-012
#[test]
fn classify_other_compression_extensions() {
    assert_eq!(classify_suffix(".bz2"), SiblingKind::CompressedRotation);
    assert_eq!(classify_suffix(".xz"), SiblingKind::CompressedRotation);
    assert_eq!(classify_suffix(".zst"), SiblingKind::CompressedRotation);
    assert_eq!(classify_suffix(".2.xz"), SiblingKind::CompressedRotation);
}

// TEST-ID: GROUPING-013
#[test]
fn classify_signature_extensions() {
    assert_eq!(classify_suffix(".sig"), SiblingKind::Signature);
    assert_eq!(classify_suffix(".asc"), SiblingKind::Signature);
    assert_eq!(classify_suffix(".p7s"), SiblingKind::Signature);
}

// TEST-ID: GROUPING-014
#[test]
fn classify_checksum_extensions() {
    assert_eq!(classify_suffix(".sha256"), SiblingKind::Checksum);
    assert_eq!(classify_suffix(".sha512"), SiblingKind::Checksum);
    assert_eq!(classify_suffix(".md5"), SiblingKind::Checksum);
}

// TEST-ID: GROUPING-015
#[test]
fn classify_backup_extensions() {
    assert_eq!(classify_suffix(".bak"), SiblingKind::Backup);
    assert_eq!(classify_suffix(".orig"), SiblingKind::Backup);
    assert_eq!(classify_suffix(".old"), SiblingKind::Backup);
}

// TEST-ID: GROUPING-016
#[test]
fn classify_unrecognised_suffix_is_related() {
    assert_eq!(classify_suffix(".tmp"), SiblingKind::Related);
    assert_eq!(classify_suffix(".disabled"), SiblingKind::Related);
    assert_eq!(classify_suffix("-backup-copy"), SiblingKind::Related);
}

// ============================================================================
// Layer 3: group_entries() — full grouping pipeline
// ============================================================================

// TEST-ID: GROUPING-017 — empty input
#[test]
fn group_entries_empty_input_returns_empty() {
    let result = group_entries(&[]);
    assert!(result.is_empty());
}

// TEST-ID: GROUPING-018 — standalone file (no siblings)
#[test]
fn group_entries_standalone_file() {
    let entries = vec![make_entry("syslog", 1000)];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].base.dirent.name.as_str(), "syslog");
    assert!(groups[0].siblings.is_empty());
}

// TEST-ID: GROUPING-019 — rotation pattern
#[test]
fn group_entries_rotation_pattern() {
    let entries = vec![
        make_entry("boot.log", 4200),
        make_entry("boot.log-20260301", 3100),
        make_entry("boot.log-20260302", 3200),
        make_entry("boot.log-20260303", 3300),
    ];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].base.dirent.name.as_str(), "boot.log");
    assert_eq!(groups[0].siblings.len(), 3);
    for sib in &groups[0].siblings {
        assert_eq!(sib.kind, SiblingKind::Rotation);
    }
}

// TEST-ID: GROUPING-020 — compressed rotation
#[test]
fn group_entries_compressed_rotation() {
    let entries = vec![
        make_entry("syslog", 10_000),
        make_entry("syslog.1", 9_500),
        make_entry("syslog.2.gz", 800),
        make_entry("syslog.3.gz", 750),
    ];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].siblings.len(), 3);
    assert_eq!(groups[0].siblings[0].kind, SiblingKind::Rotation);
    assert_eq!(groups[0].siblings[1].kind, SiblingKind::CompressedRotation);
    assert_eq!(groups[0].siblings[2].kind, SiblingKind::CompressedRotation);
}

// TEST-ID: GROUPING-021 — signature detection
#[test]
fn group_entries_signature_sibling() {
    let entries = vec![make_entry("report.pdf", 1_200_000), make_entry("report.pdf.sig", 512)];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].siblings.len(), 1);
    assert_eq!(groups[0].siblings[0].kind, SiblingKind::Signature);
}

// TEST-ID: GROUPING-022 — checksum detection
#[test]
fn group_entries_checksum_sibling() {
    let entries = vec![make_entry("package.tar", 50_000), make_entry("package.tar.sha256", 64)];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].siblings.len(), 1);
    assert_eq!(groups[0].siblings[0].kind, SiblingKind::Checksum);
}

// TEST-ID: GROUPING-023 — backup detection
#[test]
fn group_entries_backup_sibling() {
    let entries = vec![
        make_entry("config", 1024),
        make_entry("config.bak", 1024),
        make_entry("config.orig", 900),
    ];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].siblings.len(), 2);
    assert_eq!(groups[0].siblings[0].kind, SiblingKind::Backup);
    assert_eq!(groups[0].siblings[1].kind, SiblingKind::Backup);
}

// TEST-ID: GROUPING-024 — mixed group (rotations + signature)
#[test]
fn group_entries_mixed_rotation_and_signature() {
    let entries = vec![
        make_entry("messages", 12_000),
        make_entry("messages-20260301", 9_000),
        make_entry("messages-20260302", 8_500),
        make_entry("messages.sig", 256),
    ];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].siblings.len(), 3);

    let rotation_count =
        groups[0].siblings.iter().filter(|s| s.kind == SiblingKind::Rotation).count();
    let sig_count = groups[0].siblings.iter().filter(|s| s.kind == SiblingKind::Signature).count();
    assert_eq!(rotation_count, 2);
    assert_eq!(sig_count, 1);
}

// TEST-ID: GROUPING-025 — no false positives: file.log does not absorb file.logging
#[test]
fn group_entries_no_false_positive_shared_prefix() {
    let entries = vec![make_entry("file.log", 100), make_entry("file.logging", 200)];
    let groups = group_entries(&entries);
    // "file.logging" lacks a separator at position 8, so it starts its own group.
    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].base.dirent.name.as_str(), "file.log");
    assert!(groups[0].siblings.is_empty());
    assert_eq!(groups[1].base.dirent.name.as_str(), "file.logging");
    assert!(groups[1].siblings.is_empty());
}

// TEST-ID: GROUPING-026 — multiple independent groups
#[test]
fn group_entries_multiple_independent_groups() {
    let entries = vec![
        make_entry("auth.log", 5_000),
        make_entry("auth.log.1", 4_800),
        make_entry("auth.log.2.gz", 400),
        make_entry("kern.log", 8_000),
        make_entry("kern.log.1", 7_500),
        make_entry("syslog", 15_000),
    ];
    let groups = group_entries(&entries);
    assert_eq!(groups.len(), 3);

    assert_eq!(groups[0].base.dirent.name.as_str(), "auth.log");
    assert_eq!(groups[0].siblings.len(), 2);

    assert_eq!(groups[1].base.dirent.name.as_str(), "kern.log");
    assert_eq!(groups[1].siblings.len(), 1);

    assert_eq!(groups[2].base.dirent.name.as_str(), "syslog");
    assert!(groups[2].siblings.is_empty());
}

// ============================================================================
// Layer 4: aggregate_size() and sibling_summary()
// ============================================================================

// TEST-ID: GROUPING-027 — aggregate_size standalone file
#[test]
fn aggregate_size_standalone_is_zero() {
    let entries = vec![make_entry("solo", 1234)];
    let groups = group_entries(&entries);
    assert_eq!(aggregate_size(&groups[0]), 0);
}

// TEST-ID: GROUPING-028 — aggregate_size sums sibling sizes
#[test]
fn aggregate_size_sums_siblings() {
    let entries = vec![
        make_entry("kern.log", 1000),
        make_entry("kern.log.1", 900),
        make_entry("kern.log.2.gz", 100),
    ];
    let groups = group_entries(&entries);
    // Base (kern.log) is excluded — only siblings summed.
    assert_eq!(aggregate_size(&groups[0]), 1000);
}

// TEST-ID: GROUPING-029 — sibling_summary empty for standalone
#[test]
fn sibling_summary_empty_for_standalone() {
    let entries = vec![make_entry("solo", 500)];
    let groups = group_entries(&entries);
    assert_eq!(sibling_summary(&groups[0]), "");
}

// TEST-ID: GROUPING-030 — sibling_summary mixed types
#[test]
fn sibling_summary_mixed_types() {
    let entries = vec![
        make_entry("messages", 10_000),
        make_entry("messages-20260301", 9_000),
        make_entry("messages-20260302", 8_500),
        make_entry("messages.sig", 256),
    ];
    let groups = group_entries(&entries);
    let summary = sibling_summary(&groups[0]);
    // Must mention 2 rotations and 1 signature.
    assert!(
        summary.contains("rotation"),
        "expected 'rotation' in: {summary}"
    );
    assert!(
        summary.contains("signature"),
        "expected 'signature' in: {summary}"
    );
}
