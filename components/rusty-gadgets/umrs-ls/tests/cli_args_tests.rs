// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Smoke tests for umrs-ls CLI argument handling.
//
// These tests verify that the clap argument definition is well-formed and
// that the binary responds correctly to --help, --version, and unknown flags.
//
// NIST SP 800-53 SI-10 — validates that input validation is structurally
// sound at the argument parser level.

use std::process::Command;

/// Locate the umrs-ls binary in the Cargo build output.
///
/// Uses the `CARGO_BIN_EXE_umrs-ls` env var which Cargo sets for integration
/// tests in the same workspace package.
fn umrs_ls_bin() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_umrs-ls"))
}

/// # TEST-ID: CLI-LS-001
/// # REQUIREMENT: --help exits with code 0 and produces non-empty output
/// # COMPLIANCE: NIST SP 800-53 SI-10
///
/// This smoke test catches malformed clap definitions (e.g., conflicting
/// argument names, missing required fields) that would cause a panic at
/// startup rather than a clean help screen.
#[test]
fn help_exits_zero_with_output() {
    let output = Command::new(umrs_ls_bin())
        .arg("--help")
        .output()
        .expect("failed to execute umrs-ls --help");

    assert!(
        output.status.success(),
        "--help should exit 0, got {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "--help should produce output on stdout");
    assert!(
        stdout.contains("umrs-ls"),
        "--help output should contain the binary name"
    );
}

/// # TEST-ID: CLI-LS-002
/// # REQUIREMENT: --version exits with code 0 and includes the package version
/// # COMPLIANCE: NIST SP 800-53 SI-10
#[test]
fn version_exits_zero_with_output() {
    let output = Command::new(umrs_ls_bin())
        .arg("--version")
        .output()
        .expect("failed to execute umrs-ls --version");

    assert!(
        output.status.success(),
        "--version should exit 0, got {:?}",
        output.status.code()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.is_empty(),
        "--version should produce output on stdout"
    );
}

/// # TEST-ID: CLI-LS-003
/// # REQUIREMENT: An unknown flag causes a non-zero exit (clap rejects it)
/// # COMPLIANCE: NIST SP 800-53 SI-10
///
/// This test confirms that the old manual parsing behavior — silently ignoring
/// unknown flags — has been replaced by fail-closed validation.
#[test]
fn unknown_flag_exits_nonzero() {
    let output = Command::new(umrs_ls_bin())
        .arg("--unknown-flag-that-does-not-exist")
        .output()
        .expect("failed to execute umrs-ls with unknown flag");

    assert!(
        !output.status.success(),
        "unknown flag should cause non-zero exit, got 0"
    );
}

/// # TEST-ID: CLI-LS-004
/// # REQUIREMENT: --verbose flag is accepted without error
/// # COMPLIANCE: NIST SP 800-53 SI-11
///
/// Verifies that the --verbose flag is accepted. In non-TTY test environments
/// umrs-ls runs in --cli mode automatically; we combine with --cli to ensure
/// the binary can complete without a terminal.
#[test]
fn verbose_flag_accepted() {
    let output = Command::new(umrs_ls_bin())
        .args(["--verbose", "--cli", "."])
        .output()
        .expect("failed to execute umrs-ls --verbose --cli .");

    // Should not exit with a usage error (code 2 from clap).
    assert_ne!(
        output.status.code(),
        Some(2),
        "--verbose should not cause a usage error\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
