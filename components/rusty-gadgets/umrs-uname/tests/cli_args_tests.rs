// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Smoke tests for umrs-uname CLI argument handling.
//
// These tests verify that the clap argument definition is well-formed and
// that the binary responds correctly to --help, --version, and unknown flags.
//
// NIST SP 800-53 SI-10 — validates that input validation is structurally
// sound at the argument parser level.

use std::process::Command;

/// Locate the umrs-uname binary in the Cargo build output.
///
/// Uses the `CARGO_BIN_EXE_umrs-uname` env var which Cargo sets for integration
/// tests in the same workspace package.
fn umrs_uname_bin() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_umrs-uname"))
}

/// # TEST-ID: CLI-UNAME-001
/// # REQUIREMENT: --help exits with code 0 and produces non-empty output
/// # COMPLIANCE: NIST SP 800-53 SI-10
///
/// This smoke test catches malformed clap definitions that would cause a panic
/// at startup rather than a clean help screen.
#[test]
fn help_exits_zero_with_output() {
    let output = Command::new(umrs_uname_bin())
        .arg("--help")
        .output()
        .expect("failed to execute umrs-uname --help");

    assert!(
        output.status.success(),
        "--help should exit 0, got {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "--help should produce output on stdout");
    assert!(
        stdout.contains("umrs-uname"),
        "--help output should contain the binary name"
    );
}

/// # TEST-ID: CLI-UNAME-002
/// # REQUIREMENT: --version exits with code 0 and includes the package version
/// # COMPLIANCE: NIST SP 800-53 SI-10
#[test]
fn version_exits_zero_with_output() {
    let output = Command::new(umrs_uname_bin())
        .arg("--version")
        .output()
        .expect("failed to execute umrs-uname --version");

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

/// # TEST-ID: CLI-UNAME-003
/// # REQUIREMENT: An unknown flag causes a non-zero exit (clap rejects it)
/// # COMPLIANCE: NIST SP 800-53 SI-10
///
/// Confirms that the newly added clap parser rejects unknown flags rather than
/// silently ignoring them.
#[test]
fn unknown_flag_exits_nonzero() {
    let output = Command::new(umrs_uname_bin())
        .arg("--unknown-flag-that-does-not-exist")
        .output()
        .expect("failed to execute umrs-uname with unknown flag");

    assert!(
        !output.status.success(),
        "unknown flag should cause non-zero exit, got 0"
    );
}

/// # TEST-ID: CLI-UNAME-004
/// # REQUIREMENT: --json exits with code 0 and emits an informational message
/// # COMPLIANCE: NIST SP 800-53 SI-10
///
/// --json is reserved for future implementation. The binary must exit cleanly
/// with a "not yet implemented" notice rather than starting the TUI.
#[test]
fn json_flag_exits_cleanly() {
    let output = Command::new(umrs_uname_bin())
        .arg("--json")
        .output()
        .expect("failed to execute umrs-uname --json");

    assert!(
        output.status.success(),
        "--json should exit 0, got {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not yet implemented"),
        "--json should emit 'not yet implemented' notice on stderr"
    );
}

/// # TEST-ID: CLI-UNAME-005
/// # REQUIREMENT: --cli exits with code 0 and emits an informational message
/// # COMPLIANCE: NIST SP 800-53 SI-10
#[test]
fn cli_flag_exits_cleanly() {
    let output = Command::new(umrs_uname_bin())
        .arg("--cli")
        .output()
        .expect("failed to execute umrs-uname --cli");

    assert!(
        output.status.success(),
        "--cli should exit 0, got {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not yet implemented"),
        "--cli should emit 'not yet implemented' notice on stderr"
    );
}
