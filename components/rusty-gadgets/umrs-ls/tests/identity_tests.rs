// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for [`umrs_ls::identity::resolve_owner_display`].
//!
//! Verifies that orphaned uids/gids (no NSS entry) are rendered as
//! `"(orphan)"` rather than a numeric fallback — preserving the
//! `UnresolvedOwner`/`UnresolvedGroup` security observation signal.
//!
//! NIST SP 800-53 AC-2: Account Management.

use umrs_ls::identity::{ORPHAN_SENTINEL, resolve_owner_display};

/// A uid that is guaranteed to have no NSS entry on any POSIX system.
///
/// `i32::MAX` (2^31 - 1 = 2_147_483_647) is the maximum representable signed
/// 32-bit value.  Linux uid_t is unsigned, but glibc's NSS ABI and
/// `/etc/passwd` parsers treat values ≥ 2^31 as absent.  This value is
/// deliberately chosen to be unreachable by any normal account allocation.
const UNREACHABLE_UID: u32 = 2_147_483_647;

/// Same for gid.
const UNREACHABLE_GID: u32 = 2_147_483_647;

#[test]
fn orphan_uid_renders_sentinel_not_numeric() {
    let (owner, _group) = resolve_owner_display(UNREACHABLE_UID, 0);
    assert_eq!(
        owner, ORPHAN_SENTINEL,
        "expected orphan sentinel for uid {UNREACHABLE_UID}, got {owner:?}"
    );
}

#[test]
fn orphan_gid_renders_sentinel_not_numeric() {
    // gid 0 is always "root"; use the unreachable gid for the group side.
    let (_owner, group) = resolve_owner_display(0, UNREACHABLE_GID);
    assert_eq!(
        group, ORPHAN_SENTINEL,
        "expected orphan sentinel for gid {UNREACHABLE_GID}, got {group:?}"
    );
}

#[test]
fn both_orphan_renders_two_sentinels() {
    let (owner, group) = resolve_owner_display(UNREACHABLE_UID, UNREACHABLE_GID);
    assert_eq!(owner, ORPHAN_SENTINEL, "owner should be orphan sentinel");
    assert_eq!(group, ORPHAN_SENTINEL, "group should be orphan sentinel");
}

#[test]
fn numeric_orphan_is_not_returned() {
    // The old behaviour was to return uid.to_string().  Guard against regression.
    let (owner, group) = resolve_owner_display(UNREACHABLE_UID, UNREACHABLE_GID);
    assert_ne!(
        owner,
        UNREACHABLE_UID.to_string(),
        "orphan uid must not fall back to numeric string"
    );
    assert_ne!(
        group,
        UNREACHABLE_GID.to_string(),
        "orphan gid must not fall back to numeric string"
    );
}

#[test]
fn uid_zero_resolves_to_root() {
    // uid 0 / gid 0 must be resolvable on any conformant POSIX system.
    // This is a live NSS call — if root is absent the system is broken.
    let (owner, _group) = resolve_owner_display(0, 0);
    assert_ne!(
        owner, ORPHAN_SENTINEL,
        "uid 0 (root) should resolve to a name, not the orphan sentinel"
    );
    assert!(!owner.is_empty(), "root username must be non-empty");
}
