// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # OS Identity — Substrate-Derived Platform Identity Types
//!
//! Typed representations of OS family, distribution, kernel release, CPU
//! architecture, and the composite substrate identity derived by the package
//! probe phase. All types here are derived from the package substrate
//! (RPM DB, dpkg status), not from the self-reported `os-release` label.
//!
//! Key exported types: `OsFamily`, `Distro`, `KernelRelease`, `KernelVersion`,
//! `CpuArch`, `SubstrateIdentity`.
//!
//! This separation is intentional: the substrate provides an independent
//! identity claim that can be compared against (and used to corroborate or
//! contradict) what `os-release` asserts. Neither source is trusted in
//! isolation.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Information System Component Inventory —
//!   component identity must be accurate, typed, and independently verifiable.
//!   These types represent the inventory-level facts derived from the package
//!   substrate, which is the most authoritative non-kernel identity source.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — package
//!   substrate probes derive identity from the same database that tracks
//!   installed software provenance. A mismatch between substrate identity
//!   and `os-release` is a supply chain integrity signal.
//! - **NSA RTB**: identity must be derived from multiple independent facts.
//!   `SubstrateIdentity::facts_count` records how many independent facts
//!   were corroborated; T3 (`SubstrateAnchored`) requires ≥2.

use std::fmt;
use std::str::FromStr;

use thiserror::Error;

// ===========================================================================
// OsFamily
// ===========================================================================

/// High-level OS family, derived from the package substrate probe.
///
/// This is the family classification used to select the correct
/// [`crate::detect::substrate`] probe implementation. It is derived from
/// which package DB was successfully opened and parsed — not from any
/// self-reported string.
///
/// NIST SP 800-53 CM-8 — component inventory, substrate-derived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsFamily {
    /// RPM-based distribution (RHEL, Fedora, CentOS, AlmaLinux, RockyLinux).
    RpmBased,

    /// dpkg-based distribution (Debian, Ubuntu, Kali).
    DpkgBased,

    /// Pacman-based distribution (Arch Linux and derivatives).
    PacmanBased,

    /// No recognised package substrate was found or successfully probed.
    Unknown,
}

// ===========================================================================
// Distro
// ===========================================================================

/// Specific distribution identity, derived from package substrate evidence.
///
/// `Other(String)` captures distributions not enumerated here. The inner
/// string is the substrate-derived identifier (e.g., from the release package
/// name), not the `os-release` `ID=` field.
///
/// NIST SP 800-53 CM-8, SA-12.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Distro {
    /// Red Hat Enterprise Linux.
    Rhel,

    /// Fedora Linux.
    Fedora,

    /// CentOS Stream or CentOS Linux.
    CentOs,

    /// AlmaLinux OS.
    AlmaLinux,

    /// Rocky Linux.
    RockyLinux,

    /// Debian GNU/Linux.
    Debian,

    /// Ubuntu.
    Ubuntu,

    /// Kali Linux.
    Kali,

    /// A distribution not enumerated above, identified by a substrate-derived string.
    Other(String),
}

// ===========================================================================
// KernelRelease
// ===========================================================================

/// Kernel release string, optionally corroborated from two independent sources.
///
/// On a well-functioning system, `uname(2)` (via `rustix::system::uname`) and
/// `/proc/sys/kernel/osrelease` (via `ProcfsText`) must agree. When they agree,
/// `corroborated` is `true` and the release string can be trusted. When they
/// disagree, `corroborated` is `false` and the discrepancy is recorded in the
/// `EvidenceBundle`.
///
/// NIST SP 800-53 CM-8 — system component inventory; the kernel version is
/// a required inventory field. Corroboration from two sources strengthens the
/// claim.
#[derive(Debug, Clone)]
pub struct KernelRelease {
    /// The kernel release string (e.g., `"5.14.0-503.23.1.el9_5.x86_64"`).
    pub release: String,

    /// `true` if both `uname(2)` and `/proc/sys/kernel/osrelease` agreed on
    /// this string. `false` if only one source was available or they disagreed.
    pub corroborated: bool,
}

// ===========================================================================
// KernelVersion
// ===========================================================================

/// Parse error returned when a kernel release string does not contain a
/// recognisable `MAJOR.MINOR.PATCH` prefix.
///
/// NIST SP 800-53 SI-10 — input validation; construction fails on malformed
/// input rather than silently defaulting.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("not a valid MAJOR.MINOR.PATCH kernel version: {0:?}")]
pub struct KernelVersionParseError(String);

/// Parsed `MAJOR.MINOR.PATCH` version triple extracted from a kernel release
/// string (e.g., `"6.12.0"` from `"6.12.0-211.el10.aarch64"`).
///
/// The struct is intentionally minimal — it captures only the version triple
/// needed for catalog currency comparisons. Distribution suffixes, build
/// tags, and architecture suffixes are discarded.
///
/// Construction is via `FromStr`, which parses the leading `MAJOR.MINOR.PATCH`
/// portion and rejects strings that do not begin with three dot-separated
/// decimal integers. `Display` renders the canonical `"MAJOR.MINOR.PATCH"`
/// form, with no suffix.
///
/// `Ord` and `PartialOrd` are derived — comparison is lexicographic on the
/// `(major, minor, patch)` triple, which matches version ordering.
///
/// ## Usage
///
/// ```
/// use umrs_platform::os_identity::KernelVersion;
/// let running: KernelVersion = "6.12.0-211.el10.aarch64".parse().unwrap();
/// let baseline: KernelVersion = "6.12.0".parse().unwrap();
/// assert!(running >= baseline);
/// ```
///
/// NIST SP 800-53 CM-8 — typed kernel version for component inventory.
/// NIST SP 800-53 CA-7 — catalog currency check uses this type to compare
/// the running kernel against the baseline the indicator catalog targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KernelVersion {
    /// Linux kernel major version number.
    pub major: u32,
    /// Linux kernel minor version number.
    pub minor: u32,
    /// Linux kernel patch-level version number.
    pub patch: u32,
}

impl FromStr for KernelVersion {
    type Err = KernelVersionParseError;

    /// Parse a kernel release string into a `KernelVersion`.
    ///
    /// Accepts any string whose first three dot-separated tokens are decimal
    /// integers, e.g.:
    /// - `"6.12.0"` — plain triple
    /// - `"6.12.0-211.el10.aarch64"` — with distribution suffix (suffix ignored)
    /// - `"5.14.0-503.23.1.el9_5.x86_64"` — RHEL 9 style
    ///
    /// Returns `KernelVersionParseError` if the string does not start with
    /// three dot-separated decimal integers. Fails closed — no partial result
    /// on error.
    ///
    /// NIST SP 800-53 SI-10 — input validated at construction; callers receive
    /// a `Result`, not a silently degraded default.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || KernelVersionParseError(s.chars().take(64).collect());

        // Strip any suffix starting at the first '-' to isolate the semver
        // portion (e.g., "6.12.0-211.el10.aarch64" → "6.12.0").
        let version_part = s.split('-').next().ok_or_else(err)?;

        let mut parts = version_part.splitn(4, '.');
        let major = parts.next().ok_or_else(err)?.parse::<u32>().map_err(|_| err())?;
        let minor = parts.next().ok_or_else(err)?.parse::<u32>().map_err(|_| err())?;
        let patch = parts.next().ok_or_else(err)?.parse::<u32>().map_err(|_| err())?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl fmt::Display for KernelVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// ===========================================================================
// CpuArch
// ===========================================================================

/// CPU architecture identifier sourced from `uname(2)`.
///
/// The primary source is `uname(2)` (the `machine` field). The type stores
/// raw ELF `e_machine` values in `Unknown(u16)` for audit records when the
/// architecture is not enumerated here. Cross-checking `uname(2)` against the
/// ELF `e_machine` field in a known binary is the caller's responsibility —
/// this type does not perform the cross-check itself (NIST SP 800-53 CM-8).
///
/// `Unknown(u16)` preserves the raw ELF `e_machine` value for audit records
/// when the architecture is not in this enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArch {
    /// `x86_64` / `amd64` — ELF `e_machine` = 62 (`EM_X86_64`).
    X86_64,

    /// `aarch64` / `arm64` — ELF `e_machine` = 183 (`EM_AARCH64`).
    Aarch64,

    /// `riscv64` — ELF `e_machine` = 243 (`EM_RISCV`).
    Riscv64,

    /// An architecture not enumerated above. The inner value is the raw ELF
    /// `e_machine` field, preserved for audit records.
    Unknown(u16),
}

// ===========================================================================
// SubstrateIdentity
// ===========================================================================

/// Composite platform identity derived from the package substrate probe.
///
/// This struct accumulates all substrate-derived identity facts during Phase 3
/// (`pkg_substrate`). It is independent of any `os-release` self-report —
/// the fields here are derived from the package database alone.
///
/// A `SubstrateIdentity` with `facts_count >= 2` satisfies the T3
/// (`SubstrateAnchored`) trust tier requirement. Callers must not assert T3
/// with fewer than two corroborating facts.
///
/// NIST SP 800-53 CM-8 — component inventory; SA-12 — supply chain risk.
/// NSA RTB — identity must be derived from multiple independent facts.
#[derive(Debug, Clone)]
pub struct SubstrateIdentity {
    /// High-level OS family derived from which package substrate was probed.
    pub family: OsFamily,

    /// Specific distribution identity, if the probe could determine it.
    pub distro: Option<Distro>,

    /// Version identifier from the release package in the substrate
    /// (e.g., `"10"` from `redhat-release-10.0-1.el10.aarch64`).
    /// This is independent of the `VERSION_ID=` field in `os-release`.
    pub version_id: Option<String>,

    /// Number of independent corroborating facts gathered from the substrate.
    ///
    /// Must reach ≥2 before T3 (`SubstrateAnchored`) can be asserted.
    /// Incremented exclusively via [`SubstrateIdentity::add_fact`], which uses
    /// `saturating_add(1)` to prevent overflow in release builds regardless of
    /// the workspace `overflow-checks` setting (ANSSI Secure Rust Coding Guide:
    /// integer overflow must be handled explicitly rather than relying on
    /// debug-mode overflow checks, which are absent in release builds).
    pub facts_count: u8,

    /// The probe implementation that produced this identity.
    /// One of `"rpm"`, `"dpkg"`, `"pacman"`, or `"unknown"`.
    pub probe_used: &'static str,
}

impl SubstrateIdentity {
    /// Increment `facts_count` by one, saturating at `u8::MAX`.
    ///
    /// Uses `saturating_add(1)` explicitly — this is the correct operation
    /// regardless of whether `overflow-checks = true` is set in the release
    /// profile. A saturated count (255) correctly communicates "many facts"
    /// and never wraps to zero.
    ///
    /// ANSSI Rust Secure Coding Guide — checked/saturating arithmetic MUST be
    /// used for all integer operations on security-relevant values.
    pub const fn add_fact(&mut self) {
        self.facts_count = self.facts_count.saturating_add(1);
    }

    /// Return `true` if at least two independent facts have been corroborated.
    ///
    /// T3 (`SubstrateAnchored`) requires `facts_count >= 2`.
    #[must_use]
    pub const fn meets_t3_threshold(&self) -> bool {
        self.facts_count >= 2
    }
}
