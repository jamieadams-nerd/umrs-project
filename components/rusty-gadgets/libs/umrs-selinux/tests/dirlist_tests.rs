// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// Pure-logic tests for `umrs_selinux::utils::dirlist`.
//
// These tests exercise the data types and builder APIs in `dirlist.rs`
// without requiring a live SELinux kernel or mounted filesystems.
// Tests that call `list_directory()` directly are marked `#[ignore]`
// and only run in an SELinux-enabled environment.
//
// NIST SP 800-53 AC-4 / NSA RTB Non-Bypassability.

use std::collections::BTreeMap;
use umrs_selinux::utils::dirlist::{Column, ColumnSet, DirListing, GroupKey};

// ── GroupKey ordering ─────────────────────────────────────────────────────────

/// BTreeMap orders GroupKey lexicographically: type-alpha first, then marking.
/// Verifies that the derived Ord implementation provides the expected grouping
/// behaviour without explicit sort steps.
#[test]
fn group_key_btreemap_order_by_type_then_marking() {
    let mut map: BTreeMap<GroupKey, u32> = BTreeMap::new();
    map.insert(
        GroupKey {
            selinux_type: "httpd_t".to_string(),
            marking: "s0".to_string(),
        },
        1,
    );
    map.insert(
        GroupKey {
            selinux_type: "bin_t".to_string(),
            marking: "CUI//LEI".to_string(),
        },
        2,
    );
    map.insert(
        GroupKey {
            selinux_type: "bin_t".to_string(),
            marking: "s0".to_string(),
        },
        3,
    );

    let keys: Vec<&GroupKey> = map.keys().collect();

    // bin_t (marking CUI//LEI) < bin_t (marking s0) < httpd_t (any marking)
    assert_eq!(keys[0].selinux_type, "bin_t");
    assert_eq!(keys[0].marking, "CUI//LEI");
    assert_eq!(keys[1].selinux_type, "bin_t");
    assert_eq!(keys[1].marking, "s0");
    assert_eq!(keys[2].selinux_type, "httpd_t");
}

#[test]
fn group_key_restricted_sorts_before_printable_type() {
    let mut map: BTreeMap<GroupKey, u32> = BTreeMap::new();
    map.insert(
        GroupKey {
            selinux_type: "var_log_t".to_string(),
            marking: "s0".to_string(),
        },
        1,
    );
    map.insert(
        GroupKey {
            selinux_type: "<restricted>".to_string(),
            marking: "<restricted>".to_string(),
        },
        2,
    );

    let keys: Vec<&GroupKey> = map.keys().collect();
    // '<' (0x3C) sorts before lowercase letters
    assert_eq!(keys[0].selinux_type, "<restricted>");
    assert_eq!(keys[1].selinux_type, "var_log_t");
}

// ── ColumnSet builder ─────────────────────────────────────────────────────────

/// Default set includes the expected columns.
#[test]
fn column_set_default_contains_expected_columns() {
    let cols = ColumnSet::default();
    assert!(cols.contains(Column::Mode));
    assert!(cols.contains(Column::Iov));
    assert!(cols.contains(Column::SelinuxType));
    assert!(cols.contains(Column::Marking));
    assert!(cols.contains(Column::UidGid));
    assert!(cols.contains(Column::Mtime));
    assert!(cols.contains(Column::Name));
}

/// Default set does NOT include optional columns.
#[test]
fn column_set_default_excludes_optional_columns() {
    let cols = ColumnSet::default();
    assert!(!cols.contains(Column::Size));
    assert!(!cols.contains(Column::Inode));
}

/// `.without()` removes the column and the column no longer appears.
#[test]
fn column_set_without_removes_column() {
    let cols = ColumnSet::default().without(Column::Iov);
    assert!(!cols.contains(Column::Iov));
}

/// `.without()` on Name is a no-op — Name cannot be removed.
#[test]
fn column_set_without_name_is_noop() {
    let cols = ColumnSet::default().without(Column::Name);
    assert!(cols.contains(Column::Name));
}

/// `.with()` adds a column not previously present.
#[test]
fn column_set_with_adds_column() {
    let cols = ColumnSet::default().with(Column::Size);
    assert!(cols.contains(Column::Size));
}

/// `.with()` on an already-present column is a no-op (no duplicate).
#[test]
fn column_set_with_is_idempotent() {
    let cols = ColumnSet::default().with(Column::Mode);
    let count = cols.columns().iter().filter(|&&c| c == Column::Mode).count();
    assert_eq!(count, 1);
}

/// Name is always the last column after builder operations.
#[test]
fn column_set_name_is_always_last() {
    let cols = ColumnSet::default()
        .without(Column::Iov)
        .with(Column::Size)
        .with(Column::Inode);

    let last = cols.columns().last().copied();
    assert_eq!(last, Some(Column::Name));
}

/// `.with()` inserts the new column immediately before Name.
#[test]
fn column_set_with_inserts_before_name() {
    let cols = ColumnSet::default().with(Column::Size);
    let pos_size = cols.columns().iter().position(|&c| c == Column::Size);
    let pos_name = cols.columns().iter().position(|&c| c == Column::Name);
    assert!(pos_size.is_some());
    assert!(pos_name.is_some());
    assert!(pos_size.unwrap() < pos_name.unwrap());
}

// ── DirListing construction ───────────────────────────────────────────────────

/// A freshly constructed DirListing has the correct field types.
#[test]
fn dir_listing_fields_have_expected_types() {
    let listing = DirListing {
        path: std::path::PathBuf::from("/tmp"),
        groups: Vec::new(),
        access_denied: Vec::new(),
        elapsed_us: 42u64,
    };

    assert_eq!(listing.elapsed_us, 42u64);
    assert!(listing.groups.is_empty());
    assert!(listing.access_denied.is_empty());
}

// ── live-environment tests (ignored by default) ───────────────────────────────

/// Smoke test: list_directory on /tmp returns without error.
/// Requires a running Linux kernel with a writable /tmp.
#[test]
#[ignore = "requires live Linux environment"]
fn list_directory_tmp_returns_ok() {
    use umrs_selinux::utils::dirlist::list_directory;
    let result = list_directory(std::path::Path::new("/tmp"));
    assert!(
        result.is_ok(),
        "list_directory(/tmp) failed: {:?}",
        result.err()
    );
}
