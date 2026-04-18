// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # Identity Resolution — Display Layer
//!
//! Thin façade over [`umrs_selinux::posix::LinuxOwnership::resolve`] for
//! translating numeric POSIX ownership values into operator-visible display
//! strings.
//!
//! ## Why a façade?
//!
//! `LinuxOwnership::resolve` is the canonical NSS resolution path in
//! `umrs-selinux`.  Calling it preserves the `None`-means-orphan signal:
//! when a uid or gid has no `/etc/passwd` / `/etc/group` / NSS entry,
//! the resolved `name` field is `None`.  Callers that collapse `None` into
//! `uid.to_string()` silently destroy that signal, suppressing the
//! `UnresolvedOwner` / `UnresolvedGroup` security observations that
//! `SecureDirent::security_observations()` is designed to emit.
//!
//! This module surfaces the orphan case as `"(orphan)"` so the condition is
//! visible in all umrs-ls output modes (CLI, TUI, and JSON).
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-2**: Account Management — orphaned accounts whose
//!   files remain on disk after deletion are surfaced as a visible anomaly
//!   rather than silently folded into a numeric fallback.

use umrs_selinux::posix::{Gid, LinuxOwnership, Uid};

/// Sentinel rendered when a uid or gid has no NSS entry (orphaned account).
///
/// Chosen to be unambiguous in both human-readable and JSON output: a
/// parenthesised label that cannot be a valid POSIX name.
pub const ORPHAN_SENTINEL: &str = "(orphan)";

/// Resolve a (uid, gid) pair to operator-visible display strings.
///
/// Delegates to [`LinuxOwnership::resolve`], which performs at most two NSS
/// lookups (one for uid, one for gid).  When a uid or gid has no entry in
/// `/etc/passwd`, `/etc/group`, or any configured name service (LDAP, SSSD,
/// etc.), the corresponding display string is [`ORPHAN_SENTINEL`] rather than
/// the numeric id — making orphaned-account ownership visible in all output
/// modes.
///
/// This is the **only** UID/GID resolution path in umrs-ls.  All three
/// previous per-module duplicates (`main.rs`, `tree_adapter.rs`,
/// `viewer_app.rs`) have been replaced with this call.
///
/// # Returns
///
/// `(owner_display, group_display)` — both are non-empty strings safe for
/// direct inclusion in CLI rows, TUI cells, and JSON field values.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-2**: Account Management — orphaned UID/GID values
///   (no corresponding passwd/group entry) are surfaced as the sentinel
///   string rather than silently omitted from audit output.
#[must_use = "discarding resolved owner strings loses the orphan visibility signal"]
pub fn resolve_owner_display(uid: u32, gid: u32) -> (String, String) {
    let ownership = LinuxOwnership::resolve(Uid::from(uid), Gid::from(gid));

    let owner = ownership
        .user
        .name
        .as_ref()
        .map_or_else(|| ORPHAN_SENTINEL.to_owned(), ToString::to_string);

    let group = ownership
        .group
        .name
        .as_ref()
        .map_or_else(|| ORPHAN_SENTINEL.to_owned(), ToString::to_string);

    (owner, group)
}
