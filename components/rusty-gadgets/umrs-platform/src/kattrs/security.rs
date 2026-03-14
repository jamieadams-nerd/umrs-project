// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! Securityfs kernel attribute types (`/sys/kernel/security/`).
//!
//! All types in this module read from securityfs and are verified against
//! `SECURITYFS_MAGIC` before any bytes are parsed.
//!
//! `KernelLockdown` uses Two-Path Independent (TPI) parsing because the
//! bracketed format (`none [integrity] confidentiality`) carries a security
//! classification decision: any ambiguity must fail closed.
//!
//! NIST SP 800-53 CM-7, SC-39, SI-7: Least functionality, process isolation,
//! and software integrity.
//! NSA RTB RAIN: Non-bypassable enforcement; fail-closed on parse disagreement.

use nix::sys::statfs::FsType;
use nom::{
    IResult,
    bytes::complete::{tag, take_until},
};
use std::fmt;
use std::io;

use super::traits::{KernelFileSource, StaticSource};

// ===========================================================================
// SECURITYFS_MAGIC
// ===========================================================================

/// Filesystem magic number for securityfs (`/sys/kernel/security`).
///
/// Value `0x73636673` is defined in the Linux kernel `include/linux/magic.h`.
/// Used to verify that a file opened under `/sys/kernel/security/` was indeed
/// served by securityfs and not a bind-mounted substitute (NIST SP 800-53 SI-7).
pub const SECURITYFS_MAGIC: FsType = FsType(0x7363_6673);

// ===========================================================================
// LockdownMode
// ===========================================================================

/// Kernel lockdown level — the degree to which the kernel is locked down
/// against privileged userspace access.
///
/// Discriminants ascend with restrictiveness, enabling `Ord`-based comparisons
/// such as "is the current lockdown level at least `Integrity`?"
///
/// NIST SP 800-53 CM-7: Least Functionality — lockdown restricts kernel
/// functionality to the minimum required.
/// NIST SP 800-53 SC-39: Process Isolation — confidentiality lockdown prevents
/// even privileged processes from extracting kernel secrets.
/// NIST SP 800-53 SI-7: Software and Information Integrity — integrity lockdown
/// prevents userspace from modifying kernel code or data.
/// NSA RTB RAIN: once set, lockdown is non-bypassable at the kernel level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LockdownMode {
    /// No lockdown active — privileged userspace retains full kernel access.
    #[default]
    None = 0,
    /// Integrity lockdown — prevents modifications to kernel code and data.
    Integrity = 1,
    /// Confidentiality lockdown — additionally prevents extraction of kernel secrets.
    Confidentiality = 2,
}

impl fmt::Display for LockdownMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Integrity => write!(f, "integrity"),
            Self::Confidentiality => write!(f, "confidentiality"),
        }
    }
}

// ===========================================================================
// KernelLockdown — TPI parse helpers
// ===========================================================================

/// Nom path: find the opening `[`, then extract everything until `]`.
///
/// Input format: `none [integrity] confidentiality`
/// Extracts the token inside brackets (e.g., `"integrity"`).
fn parse_lockdown_path_a(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until("[")(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, mode) = take_until("]")(input)?;
    Ok((input, mode))
}

/// Imperative path: split on whitespace, find the `[token]` form, strip brackets.
///
/// Scans all whitespace-separated tokens for one that starts with `[` and ends
/// with `]`.  Returns the content between the brackets.
/// SSDF PW 4.1: bounds-safe slice indexing via checked length guard.
fn parse_lockdown_path_b(input: &str) -> io::Result<&str> {
    for token in input.split_whitespace() {
        if token.starts_with('[') && token.ends_with(']') {
            let len = token.len();
            // Guard: `[` + at least one char + `]` = minimum length 3.
            if len >= 3 {
                return Ok(&token[1..len - 1]);
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "No bracketed lockdown mode found",
    ))
}

/// Convert a kernel-canonical mode string to `LockdownMode`, failing closed
/// on any unrecognised value.
fn to_lockdown_mode(s: &str) -> io::Result<LockdownMode> {
    match s {
        "none" => Ok(LockdownMode::None),
        "integrity" => Ok(LockdownMode::Integrity),
        "confidentiality" => Ok(LockdownMode::Confidentiality),
        // SI-12: fixed error string — do not echo the unrecognised token to callers.
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unknown lockdown mode",
        )),
    }
}

// ===========================================================================
// KernelLockdown
// ===========================================================================

/// Kernel lockdown attribute node (`/sys/kernel/security/lockdown`).
///
/// Reads and TPI-parses the current kernel lockdown level from securityfs.
/// The file format uses bracketed notation to identify the active mode:
///
/// ```text
/// none [integrity] confidentiality
/// ```
///
/// Two independent parsers (nom declarative, imperative split) both extract
/// the bracketed token and agree on the resulting `LockdownMode`; if they
/// disagree the parse fails closed (NSA RTB RAIN).
///
/// Provenance is verified via fd-anchored `fstatfs` against `SECURITYFS_MAGIC`
/// before any bytes are read (NIST SP 800-53 SI-7).
///
/// NIST SP 800-53 CM-7: Least Functionality.
/// NIST SP 800-53 SC-39: Process Isolation.
/// NIST SP 800-53 SI-7: Software and Information Integrity.
/// NSA RTB RAIN: Non-bypassable, fail-closed TPI parsing.
pub struct KernelLockdown;

impl KernelFileSource for KernelLockdown {
    type Output = LockdownMode;
    const KOBJECT: &'static str = "securityfs";
    const ATTRIBUTE_NAME: &'static str = "lockdown";
    const DESCRIPTION: &'static str = "none       -- no lockdown; privileged userspace retains full kernel access\n\
         integrity  -- prevents modifications to kernel code and critical data\n\
         confidentiality -- additionally prevents extraction of kernel secrets";
    const KERNEL_NOTE: &'static str = "Read-only. Active mode shown in brackets: `none [integrity] confidentiality`. \
         Configured at boot via `lockdown=` kernel parameter or via IMA policy.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let s = std::str::from_utf8(data).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 lockdown data")
        })?;
        // Strip trailing newline for both parse paths.
        let s = s.trim_end();

        // Path A: nom declarative parser — locate and extract bracketed token.
        let mode_a = parse_lockdown_path_a(s)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Lockdown path A failure",
                )
            })
            .and_then(|(_, token)| to_lockdown_mode(token))?;

        // Path B: imperative split parser — independently locate bracketed token.
        let token_b = parse_lockdown_path_b(s)?;
        let mode_b = to_lockdown_mode(token_b)?;

        // TPI: fail closed on any disagreement between the two parse paths.
        // SI-12: log enum values only — never the raw input bytes.
        if mode_a != mode_b {
            log::error!(
                "TPI FAILURE: KernelLockdown parse disagreement ({mode_a:?} vs {mode_b:?})"
            );
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "RTB Redundancy Failure: lockdown mode mismatch",
            ));
        }

        #[cfg(debug_assertions)]
        log::debug!(
            "TPI: KernelLockdown dual-path validation completed in {} µs",
            start.elapsed().as_micros()
        );

        Ok(mode_a)
    }
}

impl StaticSource for KernelLockdown {
    const PATH: &'static str = "/sys/kernel/security/lockdown";
    const EXPECTED_MAGIC: FsType = SECURITYFS_MAGIC;
}
