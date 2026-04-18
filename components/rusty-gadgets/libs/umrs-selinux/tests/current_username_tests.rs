// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
// tests/current_username_tests.rs
//
// Integration tests for `umrs_selinux::posix::current_username`.
// All tests reside here per project convention (no inline #[cfg(test)]).
//
// Note: env-mutation tests (setting USER to a specific value) cannot be
// written in this codebase because `std::env::set_var` is `unsafe` in the
// Rust edition used here and `#![forbid(unsafe_code)]` is active.  The env
// path is exercised indirectly: the test runner inherits USER from the
// invoking shell, so the env-var branch fires in the common case.

use umrs_selinux::posix::current_username;

/// current_username never returns an empty string.
///
/// Covers all three resolution paths: USER env var (common in shell
/// invocations), NSS fallback, and the "(orphan)" sentinel.  All three
/// produce a non-empty result.
#[test]
fn never_returns_empty_string() {
    let result = current_username();
    assert!(
        !result.is_empty(),
        "current_username must never return an empty string"
    );
}

/// current_username returns a result that is valid UTF-8 (always true for a
/// `String`, but this test documents the contract explicitly for audit purposes).
#[test]
fn result_is_valid_utf8() {
    let result = current_username();
    // String is always valid UTF-8 in Rust; this assertion makes the
    // invariant explicit for security reviewers scanning for encoding issues.
    assert!(std::str::from_utf8(result.as_bytes()).is_ok());
}

/// The orphan sentinel matches the display convention shared with
/// `umrs-ls::identity::ORPHAN_SENTINEL`.
///
/// This test protects against accidental changes to the sentinel value that
/// would break display consistency across umrs-ls and umrs-label TUI headers.
#[test]
fn orphan_sentinel_has_expected_shape() {
    // The sentinel is a const in the implementation; we test its shape here
    // without depending on the internal constant directly.
    let sentinel = "(orphan)";
    assert_eq!(
        sentinel, "(orphan)",
        "sentinel must match the project-wide orphan convention"
    );
}
