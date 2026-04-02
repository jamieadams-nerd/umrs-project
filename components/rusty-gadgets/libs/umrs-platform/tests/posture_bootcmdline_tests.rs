// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for the `posture::bootcmdline` module (Phase 2b).
//!
//! Tests are grouped by subsystem:
//!
//! 1. **BLS content parser** — `parse_bls_content` directly: options line,
//!    multi-field entries, comment lines, blank lines, absent fields, tabs.
//!    These tests run in any environment (no filesystem dependency).
//! 2. **Options extraction** — `read_configured_cmdline` graceful degrade and
//!    environment-conditional assertions.
//! 3. **Snapshot integration** — KernelCmdline indicators have `configured_value`
//!    when BLS entries are present; gracefully return `None` when absent.

use umrs_platform::posture::bootcmdline::{parse_bls_content, read_configured_cmdline};
use umrs_platform::posture::indicator::IndicatorId;
use umrs_platform::posture::snapshot::PostureSnapshot;

// ===========================================================================
// 1. BLS content parser — direct tests (no filesystem dependency)
//
// These tests call `parse_bls_content` directly on in-memory strings,
// so they run correctly in any environment (CI, containers, systems without
// /boot/loader/entries/). This resolves the T-01 coverage gap identified
// in the Phase 2b security review (2026-03-16).
// ===========================================================================

/// Options line is extracted correctly from a typical RHEL BLS entry.
#[test]
fn parse_bls_content_options_typical_rhel_entry() {
    let entry = "title Red Hat Enterprise Linux\n\
                 version 5.14.0-503.el10.aarch64\n\
                 linux /vmlinuz-5.14.0-503.el10.aarch64\n\
                 initrd /initramfs-5.14.0-503.el10.aarch64.img\n\
                 options root=UUID=abc rhgb quiet fips=1 audit=1\n";

    let result = parse_bls_content(entry, "options");
    assert_eq!(
        result,
        Some("root=UUID=abc rhgb quiet fips=1 audit=1"),
        "options field must be extracted from a typical RHEL BLS entry"
    );
}

/// `version` field is extracted correctly.
#[test]
fn parse_bls_content_version_field() {
    let entry = "title My Kernel\n\
                 version 5.14.0-503.el10.aarch64\n\
                 options root=/dev/sda1\n";

    let result = parse_bls_content(entry, "version");
    assert_eq!(
        result,
        Some("5.14.0-503.el10.aarch64"),
        "version field must be extracted correctly"
    );
}

/// Comment lines (starting with `#`) are skipped.
#[test]
fn parse_bls_content_comment_lines_skipped() {
    let entry = "# This is a comment\n\
                 title My Kernel\n\
                 # options should not match this comment\n\
                 options root=/dev/sda1 quiet\n";

    let result = parse_bls_content(entry, "options");
    assert_eq!(
        result,
        Some("root=/dev/sda1 quiet"),
        "comment lines must not be matched as fields"
    );
}

/// Blank lines are skipped without error.
#[test]
fn parse_bls_content_blank_lines_skipped() {
    let entry = "\n\n\
                 title My Kernel\n\
                 \n\
                 options root=/dev/sda1\n\
                 \n";

    let result = parse_bls_content(entry, "options");
    assert_eq!(
        result,
        Some("root=/dev/sda1"),
        "blank lines must not interfere with field extraction"
    );
}

/// Absent field returns `None`.
#[test]
fn parse_bls_content_absent_field_returns_none() {
    let entry = "title My Kernel\n\
                 version 5.14.0\n\
                 linux /vmlinuz\n";

    let result = parse_bls_content(entry, "options");
    assert!(
        result.is_none(),
        "absent field must return None from parse_bls_content"
    );
}

/// Empty content returns `None`.
#[test]
fn parse_bls_content_empty_string_returns_none() {
    let result = parse_bls_content("", "options");
    assert!(
        result.is_none(),
        "empty content must return None from parse_bls_content"
    );
}

/// Tab-separated key/value (BLS allows tabs as well as spaces).
#[test]
fn parse_bls_content_tab_separated_field() {
    let entry = "title\tMy Kernel\noptions\troot=/dev/sda1 fips=1\n";
    let result = parse_bls_content(entry, "options");
    assert_eq!(
        result,
        Some("root=/dev/sda1 fips=1"),
        "tab-separated key/value must be parsed correctly"
    );
}

/// A field with no value (key only) returns an empty string (not None).
/// This preserves the first occurrence — a line `options` with no value
/// returns `Some("")`.
#[test]
fn parse_bls_content_key_only_line_returns_empty_string() {
    let entry = "title My Kernel\noptions\n";
    // `options` line has no value — parser returns the empty trimmed value.
    let result = parse_bls_content(entry, "options");
    assert_eq!(
        result,
        Some(""),
        "a key-only line must return Some(\"\"), not None"
    );
}

/// The field match is exact: `optionsfoo` must not match `options`.
#[test]
fn parse_bls_content_exact_field_match() {
    let entry = "optionsfoo root=/dev/sda1\n\
                 options root=/dev/sda2\n";
    let result = parse_bls_content(entry, "options");
    assert_eq!(
        result,
        Some("root=/dev/sda2"),
        "field match must be exact — prefix matches must not fire"
    );
}

/// First occurrence of a duplicate field is returned.
#[test]
fn parse_bls_content_first_occurrence_wins() {
    let entry = "options root=/dev/sda1\n\
                 options root=/dev/sda2\n";
    let result = parse_bls_content(entry, "options");
    assert_eq!(
        result,
        Some("root=/dev/sda1"),
        "first occurrence of a duplicate field must be returned"
    );
}

// ===========================================================================
// 2. read_configured_cmdline — environment-conditional tests
// ===========================================================================

/// `read_configured_cmdline` must not panic on any system.
#[test]
fn read_configured_cmdline_does_not_panic() {
    // On most test environments (containers, CI) /boot/loader/entries/
    // is absent. The function must return None gracefully.
    // On RHEL 10 with BLS, it may return Some(options).
    // Either result is acceptable — the test verifies no panic.
    let result = read_configured_cmdline();
    let _ = result; // intentional discard — we only test no-panic
}

/// If BLS entries exist on this system, the configured cmdline must be a
/// non-empty string.
#[test]
fn read_configured_cmdline_non_empty_if_some() {
    if let Some(cmdline) = read_configured_cmdline() {
        assert!(
            !cmdline.is_empty(),
            "configured cmdline from BLS entry must not be empty"
        );
    }
    // None is acceptable — test passes silently when BLS is absent.
}

/// If BLS entries exist on this system, the options line parses as
/// whitespace-separated tokens with no empty tokens.
#[test]
fn bls_options_parses_as_whitespace_separated_tokens() {
    if let Some(cmdline) = read_configured_cmdline() {
        for token in cmdline.split_whitespace() {
            assert!(
                !token.is_empty(),
                "BLS options token must not be empty: '{token}'"
            );
        }
    }
}

// ===========================================================================
// 3. Snapshot integration — KernelCmdline indicators
// ===========================================================================

/// KernelCmdline indicators in the snapshot must have `descriptor.class ==
/// IndicatorClass::KernelCmdline`.
#[test]
fn snapshot_cmdline_indicators_have_correct_class() {
    use umrs_platform::posture::indicator::IndicatorClass;

    let snap = PostureSnapshot::collect();
    let cmdline_ids = [
        IndicatorId::ModuleSigEnforce,
        IndicatorId::Mitigations,
        IndicatorId::Pti,
        IndicatorId::RandomTrustCpu,
        IndicatorId::RandomTrustBootloader,
    ];

    for id in cmdline_ids {
        let report = snap.get(id).unwrap_or_else(|| panic!("{id:?} must appear in snapshot"));
        assert_eq!(
            report.descriptor.class,
            IndicatorClass::KernelCmdline,
            "{id:?} must have KernelCmdline class"
        );
        // live_value is Some or None depending on /proc/cmdline availability.
        // Either is acceptable in test environments.
    }
}

/// `PostureSnapshot::collect()` must not panic regardless of whether BLS
/// entries are present. The configured_value for cmdline indicators is either
/// Some (BLS available) or None (BLS absent) — both are valid.
#[test]
fn snapshot_cmdline_configured_value_does_not_panic() {
    let snap = PostureSnapshot::collect();
    let cmdline_ids = [IndicatorId::ModuleSigEnforce, IndicatorId::Mitigations];
    for id in cmdline_ids {
        let report = snap.get(id).unwrap_or_else(|| panic!("{id:?} must appear in snapshot"));
        // configured_value is Some (BLS) or None (no BLS) — either is correct.
        // We just verify the field is accessible without panic.
        let _ = &report.configured_value;
    }
}

/// If BLS entries are available, configured_value for cmdline indicators must
/// be Some with a non-empty raw string. This test is environment-conditional:
/// it only asserts when BLS is available.
#[test]
fn snapshot_cmdline_configured_value_non_empty_when_bls_available() {
    // Check whether BLS is available on this system.
    let bls_available = read_configured_cmdline().is_some();
    if !bls_available {
        // BLS not available — test passes silently (container, minimal env).
        return;
    }

    let snap = PostureSnapshot::collect();
    // At least one cmdline indicator should have a configured_value.
    let has_configured = snap
        .iter()
        .filter(|r| {
            r.descriptor.class == umrs_platform::posture::indicator::IndicatorClass::KernelCmdline
        })
        .any(|r| r.configured_value.is_some());

    assert!(
        has_configured,
        "when BLS entries are available, at least one KernelCmdline indicator \
         must have a configured_value"
    );
}
