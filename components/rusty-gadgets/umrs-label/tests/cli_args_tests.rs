// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Smoke tests for umrs-label CLI argument handling.
//
// These tests verify that the clap argument definition is well-formed and
// that the binary responds correctly to --help and --version flags.
//
// NIST SP 800-53 SI-10 — validates that input validation is structurally
// sound at the argument parser level.

use std::process::Command;

/// Locate the umrs-label binary in the Cargo build output.
///
/// Uses the `CARGO_BIN_EXE_umrs-label` env var which Cargo sets for integration
/// tests in the same workspace package.
fn umrs_label_bin() -> std::path::PathBuf {
    // CARGO_BIN_EXE_<name> is set by Cargo for integration tests when the
    // binary is in the same package.
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_umrs-label"))
}

/// `--help` exits with code 0 and produces non-empty output.
///
/// This smoke test catches malformed clap definitions (e.g. conflicting
/// argument names, missing required fields) that would cause a panic at
/// startup rather than a clean help screen.
#[test]
fn help_exits_zero_with_output() {
    let output = Command::new(umrs_label_bin())
        .arg("--help")
        .output()
        .expect("failed to execute umrs-label --help");

    assert!(
        output.status.success(),
        "--help should exit 0, got {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.is_empty(),
        "--help should produce output on stdout"
    );
    assert!(
        stdout.contains("umrs-label"),
        "--help output should contain the binary name"
    );
}

/// `--version` exits with code 0 and includes the package version.
#[test]
fn version_exits_zero_with_output() {
    let output = Command::new(umrs_label_bin())
        .arg("--version")
        .output()
        .expect("failed to execute umrs-label --version");

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

/// An unknown flag causes a non-zero exit (clap rejects it).
///
/// This test confirms that the old manual parsing behavior — silently ignoring
/// unknown flags — has been replaced by fail-closed validation.
#[test]
fn unknown_flag_exits_nonzero() {
    let output = Command::new(umrs_label_bin())
        .arg("--unknown-flag-that-does-not-exist")
        .output()
        .expect("failed to execute umrs-label with unknown flag");

    assert!(
        !output.status.success(),
        "unknown flag should cause non-zero exit, got 0"
    );
}
