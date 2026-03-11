# umrs-platform: OS Detection Subsystem Design

## Module Tree

```
components/rusty-gadgets/umrs-platform/src/
└── detect/
    ├── mod.rs                  // public API: OsDetector, DetectionResult
    ├── evidence.rs             // EvidenceRecord, EvidenceBundle, SourceKind
    ├── confidence.rs           // TrustLevel (T0–T4), Contradiction, ConfidenceModel
    ├── phase0_kernel.rs        // procfs anchor, PID coherence, boot_id
    ├── phase1_env.rs           // namespace IDs, mountinfo, statfs dual-check
    ├── phase2_candidate.rs     // os-release path resolution, perms sanity
    ├── phase3_substrate.rs     // package DB probes (rpm/dpkg), trait PackageProbe
    ├── phase4_ownership.rs     // file ownership query via selected probe
    ├── phase5_digest.rs        // installed digest fetch + SHA-256 comparison
    ├── phase6_parse.rs         // strict os-release field parsing → typed OsRelease
    ├── substrate/
    │   ├── mod.rs              // PackageProbe trait, ProbeResult
    │   ├── rpm.rs              // RPM DB probe (RHEL, Fedora, CentOS)
    │   └── dpkg.rs             // dpkg status probe (Debian, Ubuntu)
    └── types/
        ├── mod.rs
        ├── os_release.rs       // typed OsRelease fields (not String soup)
        ├── label_trust.rs      // LabelTrust enum
        └── os_identity.rs      // OsFamily, Distro, KernelRelease, Arch
```

---

## Key Types

### `confidence.rs`

```rust
// NIST SP 800-53 SA-9, CM-6 — trust levels must be explicit and auditable
// RTB: every claim must be traceable to a kernel-anchored evidence source

/// Monotonically increasing trust tier.
/// Confidence can only downgrade, never upgrade silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// T0: No kernel anchor. Procfs unverified or inaccessible.
    Untrusted,
    /// T1: procfs verified via fstatfs(PROC_SUPER_MAGIC) + PID coherence gate.
    KernelAnchored,
    /// T2: Mount topology cross-checked (mountinfo ↔ statfs). Env reality known.
    EnvAnchored,
    /// T3: Package substrate parsed; identity derived from ≥2 independent facts.
    SubstrateAnchored,
    /// T4: os-release target ownership + installed digest verified.
    IntegrityAnchored,
}

#[derive(Debug, Clone)]
pub struct Contradiction {
    pub source_a: &'static str,
    pub source_b: &'static str,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ConfidenceModel {
    pub level: TrustLevel,
    pub contradictions: Vec<Contradiction>,
    pub downgrade_reasons: Vec<String>,
}

impl ConfidenceModel {
    pub fn downgrade(&mut self, to: TrustLevel, reason: impl Into<String>) {
        if to < self.level {
            self.level = to;
            self.downgrade_reasons.push(reason.into());
        }
    }
}
```

---

### `evidence.rs`

```rust
// NIST SP 800-53 AU-10 — non-repudiation; every read is recorded with provenance
// NIST SP 800-53 SI-7 — software and information integrity

use std::os::unix::fs::MetadataExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceKind {
    Procfs,
    RegularFile,
    PackageDb,
    SymlinkTarget,
    SysfsNode,
}

/// Full provenance record for one artifact consumed during detection.
/// Never returned as a naked value — always wrapped in EvidenceRecord.
#[derive(Debug, Clone)]
pub struct EvidenceRecord {
    pub source_kind:    SourceKind,
    pub opened_by_fd:   bool,
    pub path_requested: String,
    pub path_resolved:  Option<String>,  // if symlink
    pub st_dev:         Option<u64>,
    pub st_ino:         Option<u64>,
    pub st_mode:        Option<u32>,
    pub st_uid:         Option<u32>,
    pub st_gid:         Option<u32>,
    pub st_nlink:       Option<u64>,
    pub st_size:        Option<u64>,
    pub st_mtime:       Option<i64>,
    pub fs_magic:       Option<u64>,
    pub sha256:         Option<[u8; 32]>,
    pub pkg_digest:     Option<PkgDigest>,
    pub parse_ok:       bool,
    pub notes:          Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PkgDigest {
    pub algorithm: DigestAlgorithm,
    pub value:     Vec<u8>,   // raw bytes from package DB
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigestAlgorithm {
    Sha256,
    Sha512,
    Md5,   // legacy RPM — record but flag as weak
    Unknown(String),
}

/// Accumulated evidence from all phases.
#[derive(Debug, Default, Clone)]
pub struct EvidenceBundle {
    pub records: Vec<EvidenceRecord>,
}

impl EvidenceBundle {
    pub fn push(&mut self, record: EvidenceRecord) {
        self.records.push(record);
    }
}
```

---

### `types/os_release.rs`

```rust
// All fields are typed — no String soup.
// NIST SP 800-53 CM-8 — system component inventory must be accurate and typed.

use std::fmt;

/// Strongly typed representation of /etc/os-release or /usr/lib/os-release.
/// Fields are Option<T> — absence is explicit, not defaulted.
#[derive(Debug, Clone)]
pub struct OsRelease {
    /// ID= — machine-readable distro identifier. e.g. "rhel", "debian"
    pub id:             OsId,
    /// ID_LIKE= — space-separated parent distros. Optional.
    pub id_like:        Option<Vec<OsId>>,
    /// NAME= — human-readable distro name. Validated non-empty.
    pub name:           OsName,
    /// VERSION_ID= — machine-readable version string.
    pub version_id:     Option<VersionId>,
    /// VERSION= — human-readable version, may include codename.
    pub version:        Option<OsVersion>,
    /// VERSION_CODENAME= — e.g. "bookworm"
    pub version_codename: Option<Codename>,
    /// PRETTY_NAME= — display string.
    pub pretty_name:    Option<OsName>,
    /// HOME_URL= — validated URL format only.
    pub home_url:       Option<ValidatedUrl>,
    /// CPE_NAME= — NIST NVD CPE identifier if present.
    pub cpe_name:       Option<CpeName>,
    /// VARIANT_ID= — e.g. "server", "workstation"
    pub variant_id:     Option<VariantId>,
    /// BUILD_ID= — immutable image build identifier.
    pub build_id:       Option<BuildId>,
    /// ANSI_COLOR= — terminal color hint. Informational only.
    pub ansi_color:     Option<String>,
}

// ── Newtype wrappers with validation at construction ──────────────────────

/// Validated OS identifier: lowercase alphanumeric + hyphens only, non-empty, ≤64 chars.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsId(String);

impl OsId {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidId(s.to_owned()));
        }
        if !s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_') {
            return Err(OsReleaseParseError::InvalidId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

/// Human-readable OS name: non-empty, ≤256 chars, valid UTF-8.
#[derive(Debug, Clone)]
pub struct OsName(String);

impl OsName {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 256 {
            return Err(OsReleaseParseError::InvalidName(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
}

/// VERSION_ID: digits + dots + optional tilde, ≤32 chars.
/// e.g. "10.0", "22.04", "9"
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionId(String);

impl VersionId {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 32 {
            return Err(OsReleaseParseError::InvalidVersionId(s.to_owned()));
        }
        if !s.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '~' || c == '-') {
            return Err(OsReleaseParseError::InvalidVersionId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

/// Codename: lowercase alpha + hyphens, ≤64 chars.
#[derive(Debug, Clone)]
pub struct Codename(String);

impl Codename {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidCodename(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
}

/// CPE name: must start with "cpe:/" or "cpe:2.3:", ≤256 chars.
#[derive(Debug, Clone)]
pub struct CpeName(String);

impl CpeName {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.len() > 256 || (!s.starts_with("cpe:/") && !s.starts_with("cpe:2.3:")) {
            return Err(OsReleaseParseError::InvalidCpe(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

/// URL: minimal validation — must start with https:// or http://, ≤512 chars.
#[derive(Debug, Clone)]
pub struct ValidatedUrl(String);

impl ValidatedUrl {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.len() > 512 || (!s.starts_with("https://") && !s.starts_with("http://")) {
            return Err(OsReleaseParseError::InvalidUrl(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
}

/// OsVersion: human-readable, ≤128 chars.
#[derive(Debug, Clone)]
pub struct OsVersion(String);

impl OsVersion {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 128 {
            return Err(OsReleaseParseError::InvalidVersion(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
}

/// VariantId: lowercase alphanumeric + hyphens, ≤64 chars.
#[derive(Debug, Clone)]
pub struct VariantId(String);

impl VariantId {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 64 {
            return Err(OsReleaseParseError::InvalidVariantId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
}

/// BuildId: printable ASCII, ≤128 chars.
#[derive(Debug, Clone)]
pub struct BuildId(String);

impl BuildId {
    pub fn parse(s: &str) -> Result<Self, OsReleaseParseError> {
        let s = s.trim().trim_matches('"');
        if s.is_empty() || s.len() > 128 || !s.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
            return Err(OsReleaseParseError::InvalidBuildId(s.to_owned()));
        }
        Ok(Self(s.to_owned()))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum OsReleaseParseError {
    #[error("invalid ID field: {0}")]          InvalidId(String),
    #[error("invalid NAME field: {0}")]        InvalidName(String),
    #[error("invalid VERSION_ID field: {0}")] InvalidVersionId(String),
    #[error("invalid VERSION field: {0}")]     InvalidVersion(String),
    #[error("invalid codename: {0}")]          InvalidCodename(String),
    #[error("invalid CPE name: {0}")]          InvalidCpe(String),
    #[error("invalid URL: {0}")]               InvalidUrl(String),
    #[error("invalid VARIANT_ID: {0}")]        InvalidVariantId(String),
    #[error("invalid BUILD_ID: {0}")]          InvalidBuildId(String),
    #[error("duplicate key: {0}")]             DuplicateKey(String),
    #[error("non-UTF8 content")]               NonUtf8,
    #[error("line too long: {0} bytes")]       LineTooLong(usize),
    #[error("required field missing: {0}")]   MissingRequired(String),
}
```

---

### `types/label_trust.rs`

```rust
// RTB: trust assertions must be explicit, graded, and non-forgeable from context alone.

/// The trust level assigned to the os-release label after all phases complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelTrust {
    /// Permissions failed sanity check or file is unowned by any package.
    /// May be parsed for informational use only. Never used for policy decisions.
    UntrustedLabelCandidate,

    /// Parsed successfully but integrity not verified.
    /// Owner unknown, digest unavailable, or T3 not reached.
    LabelClaim,

    /// T4 reached: owned by package, digest matches installed.
    /// Content also corroborates substrate identity.
    TrustedLabel,

    /// T4 integrity passed but content contradicts substrate.
    /// Rare. Recorded as anomaly; treated as untrusted for policy.
    IntegrityVerifiedButContradictory {
        contradiction: String,
    },
}
```

---

### `types/os_identity.rs`

```rust
// NIST SP 800-53 CM-8 — component inventory, typed identification.

/// High-level OS family from substrate probe, not from os-release claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsFamily {
    RpmBased,
    DpkgBased,
    PacmanBased,
    Unknown,
}

/// Specific distro derived from package substrate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Distro {
    Rhel,
    Fedora,
    CentOs,
    AlmaLinux,
    RockyLinux,
    Debian,
    Ubuntu,
    Kali,
    Other(String),
}

/// Kernel release string validated from dual source:
/// uname(2) syscall ↔ /proc/sys/kernel/osrelease
#[derive(Debug, Clone)]
pub struct KernelRelease {
    pub release: String,    // e.g. "5.14.0-503.23.1.el9_5.x86_64"
    pub corroborated: bool, // true if both sources agreed
}

/// CPU architecture from kernel — ELF e_machine cross-checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArch {
    X86_64,
    Aarch64,
    Riscv64,
    Unknown(u16),  // ELF e_machine raw value
}

/// Substrate-derived identity (T3+), independent of os-release content.
#[derive(Debug, Clone)]
pub struct SubstrateIdentity {
    pub family:      OsFamily,
    pub distro:      Option<Distro>,
    pub version_id:  Option<String>,   // e.g. "10" from RHEL release package
    pub facts_count: u8,               // number of independent corroborating facts (≥2 for T3)
    pub probe_used:  &'static str,     // "rpm" | "dpkg" | "pacman"
}
```

---

### `substrate/mod.rs` — `PackageProbe` trait

```rust
// NIST SP 800-53 CM-7 — least functionality; pluggable, bounded substrate probes.

use crate::detect::{
    evidence::{EvidenceBundle, EvidenceRecord},
    types::os_identity::SubstrateIdentity,
};
use std::path::Path;

/// Result of attempting to open and parse a package manager database.
#[derive(Debug)]
pub struct ProbeResult {
    pub probe_name:              &'static str,
    pub parse_ok:                bool,
    pub can_query_ownership:     bool,
    pub can_verify_digest:       bool,
    pub identity:                Option<SubstrateIdentity>,
    pub evidence:                EvidenceRecord,
}

/// Ownership claim for a specific file path.
#[derive(Debug, Clone)]
pub struct FileOwnership {
    pub package_name:    String,
    pub package_version: String,
    pub evidence_trail:  Vec<String>,    // DB records that proved ownership
}

/// Installed digest for a specific file path from package DB.
#[derive(Debug, Clone)]
pub struct InstalledDigest {
    pub path:      String,
    pub algorithm: crate::detect::evidence::DigestAlgorithm,
    pub value:     Vec<u8>,
}

/// Pluggable package substrate probe.
/// Each implementation must:
///   - secure-open its DB roots (fstatfs sanity check)
///   - parse minimally to prove real DB
///   - require ≥2 independent facts before asserting identity
///   - never invoke external commands
pub trait PackageProbe: Send + Sync {
    /// Attempt to open and validate the package DB.
    /// Returns ProbeResult regardless of success — caller decides.
    fn probe(&self, evidence: &mut EvidenceBundle) -> ProbeResult;

    /// Query which package owns the file at (dev, inode, path).
    /// Returns None if unowned or probe lacks capability.
    fn query_ownership(
        &self,
        dev: u64,
        ino: u64,
        path: &Path,
    ) -> Option<FileOwnership>;

    /// Fetch expected installed digest for a path from the DB.
    /// Returns None if not available.
    fn installed_digest(&self, path: &Path) -> Option<InstalledDigest>;
}
```

---

### `mod.rs` — Public API

```rust
// Top-level public interface for the OS detection subsystem.
// NIST SP 800-53 SA-8, CM-6, SI-7

use crate::detect::{
    confidence::ConfidenceModel,
    evidence::EvidenceBundle,
    types::{label_trust::LabelTrust, os_identity::SubstrateIdentity, os_release::OsRelease},
};

/// Final output of the detection pipeline.
#[derive(Debug)]
pub struct DetectionResult {
    /// Identity derived purely from substrate (T3+), independent of os-release.
    pub substrate_identity: Option<SubstrateIdentity>,

    /// Parsed os-release fields, strongly typed.
    /// Present even at LabelClaim level.
    pub os_release: Option<OsRelease>,

    /// Trust level assigned to the os-release label.
    pub label_trust: LabelTrust,

    /// Boot session UUID from /proc/sys/kernel/random/boot_id.
    /// All evidence is bound to this session.
    pub boot_id: Option<String>,

    /// Final confidence tier and any contradictions.
    pub confidence: ConfidenceModel,

    /// Full provenance record for every artifact consumed.
    pub evidence: EvidenceBundle,
}

/// Orchestrates all detection phases in order.
/// Phase failures downgrade confidence; only hard gates abort.
pub struct OsDetector {
    /// Max bytes for a single bounded read (default: 65536).
    pub max_read_bytes: usize,
    /// Max bytes for mountinfo read (default: 4_194_304 = 4 MiB).
    pub max_mountinfo_bytes: usize,
    /// Max line length in os-release (default: 512).
    pub max_line_len: usize,
}

impl Default for OsDetector {
    fn default() -> Self {
        Self {
            max_read_bytes:      65_536,
            max_mountinfo_bytes: 4_194_304,
            max_line_len:        512,
        }
    }
}

impl OsDetector {
    /// Run all detection phases.
    ///
    /// # Errors
    /// Returns Err only on hard-gate failure:
    ///   - procfs is not real procfs (T0 — cannot proceed)
    ///   - PID coherence broken (kernel channel corrupted)
    ///
    /// All other failures result in confidence downgrade, not Err.
    pub fn detect(&self) -> Result<DetectionResult, DetectionError> {
        let mut evidence = EvidenceBundle::default();
        let mut confidence = ConfidenceModel {
            level: crate::detect::confidence::TrustLevel::Untrusted,
            contradictions: Vec::new(),
            downgrade_reasons: Vec::new(),
        };

        // Phase 0 — kernel anchor (hard gates)
        let boot_id = phase0_kernel::run(&mut evidence, &mut confidence)?;

        // Phase 1 — environment reality
        phase1_env::run(&mut evidence, &mut confidence, self.max_mountinfo_bytes);

        // Phase 2 — os-release candidate resolution
        let candidate = phase2_candidate::run(&mut evidence, &mut confidence);

        // Phase 3 — substrate probe
        let substrate = phase3_substrate::run(&mut evidence, &mut confidence);

        // Phase 4 — ownership
        let ownership = if let (Some(ref c), Some(ref s)) = (&candidate, &substrate) {
            phase4_ownership::run(c, s, &mut evidence, &mut confidence)
        } else {
            None
        };

        // Phase 5 — digest verification
        let digest_ok = if let (Some(ref c), Some(ref s), Some(_)) =
            (&candidate, &substrate, &ownership)
        {
            phase5_digest::run(c, s, &mut evidence, &mut confidence, self.max_read_bytes)
        } else {
            false
        };

        // Phase 6 — parse + corroborate
        let (os_release, label_trust) = if let Some(ref c) = candidate {
            phase6_parse::run(
                c,
                substrate.as_ref(),
                digest_ok,
                &mut evidence,
                &mut confidence,
                self.max_read_bytes,
                self.max_line_len,
            )
        } else {
            (None, LabelTrust::UntrustedLabelCandidate)
        };

        Ok(DetectionResult {
            substrate_identity: substrate,
            os_release,
            label_trust,
            boot_id,
            confidence,
            evidence,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DetectionError {
    #[error("procfs is not real procfs — cannot establish kernel anchor")]
    ProcfsNotReal,
    #[error("PID coherence broken: syscall={syscall} procfs={procfs}")]
    PidCoherenceFailed { syscall: u32, procfs: u32 },
    #[error("I/O error during kernel anchor: {0}")]
    KernelAnchorIo(#[from] std::io::Error),
}
```

---

## Syscall Strategy

No libc, no FFI, no `uname()` via libc. Use `rustix` (safe syscall wrapper, no libc dependency):

```toml
# Cargo.toml for umrs-platform
[dependencies]
rustix = { version = "0.38", features = ["fs", "process", "thread", "system"] }
sha2  = { version = "0.10", default-features = false }
thiserror = "1"
# zeroize only for types that hold secrets (TPM, attestation — future)
# zeroize = { version = "1", features = ["derive"] }
```

`rustix` gives you:
- `rustix::fs::openat2` with `ResolveFlags`
- `rustix::fs::fstatfs` → `FsType` comparison
- `rustix::process::getpid()`
- `rustix::fs::statx`
- `rustix::fs::readlinkat`

No libc. No FFI surface. No `uname()` via libc — read `/proc/sys/kernel/osrelease` and `/proc/version` directly and cross-check.

---

## Numeric Caps (Phase Budget)

| Resource              | Cap         | Rationale                               |
|-----------------------|-------------|-----------------------------------------|
| Single file read      | 64 KiB      | os-release, proc files — never large   |
| mountinfo read        | 4 MiB       | Busy containers can have many mounts   |
| os-release line len   | 512 bytes   | No legitimate field is longer          |
| boot_id read          | 128 bytes   | UUID is 36 chars + newline             |
| /proc/self/stat read  | 8 KiB       | Bounded kernel output                  |
| Namespace link read   | 256 bytes   | `mnt:[4026531840]` format              |
| Symlink target read   | 512 bytes   | os-release symlink target              |
| Max packages scanned  | 10,000      | Abort substrate scan after this        |

---

## NIST / CMMC / RTB Annotations (per phase)

| Phase | Control Citations |
|-------|-------------------|
| Phase 0 — Kernel anchor | NIST SI-7 (integrity verification), SA-9 (external system services — TCB boundary), CM-6 |
| Phase 1 — Mount topology | NIST CM-8 (inventory accuracy), SC-39 (process isolation) |
| Phase 2 — Candidate resolution | NIST SI-7, AC-6 (least privilege — check perms before trust) |
| Phase 3 — Substrate probe | NIST CM-8, SA-12 (supply chain — pkg DB as provenance) |
| Phase 4 — Ownership | NIST SI-7, AU-10 (non-repudiation of file ownership) |
| Phase 5 — Digest | NIST SI-7, SC-28 (integrity at rest), CMMC L2 SI.1.210 |
| Phase 6 — Parse + corroborate | NIST CM-6, CM-8, SI-2 (flaw remediation — version accuracy matters) |
| Evidence bundle | NIST AU-3, AU-10 (audit record content, non-repudiation) |

---

## File Layout Summary

```
detect/
├── mod.rs              → OsDetector, DetectionResult, DetectionError  [public API]
├── evidence.rs         → EvidenceRecord, EvidenceBundle, SourceKind, DigestAlgorithm
├── confidence.rs       → TrustLevel, Contradiction, ConfidenceModel
├── phase0_kernel.rs    → procfs gate, PID coherence, boot_id
├── phase1_env.rs       → namespace IDs, mountinfo parse, statfs dual-check
├── phase2_candidate.rs → os-release path resolution, perms sanity, symlink policy
├── phase3_substrate.rs → PackageProbe dispatch, family decision logic
├── phase4_ownership.rs → file ownership query
├── phase5_digest.rs    → installed digest fetch, SHA-256 compute, compare
├── phase6_parse.rs     → strict field-by-field parse → OsRelease, corroboration
├── substrate/
│   ├── mod.rs          → PackageProbe trait, ProbeResult, FileOwnership, InstalledDigest
│   ├── rpm.rs          → RPM BDB/SQLite probe
│   └── dpkg.rs         → dpkg status file probe
└── types/
    ├── mod.rs
    ├── os_release.rs   → OsRelease + all newtypes + OsReleaseParseError
    ├── label_trust.rs  → LabelTrust enum
    └── os_identity.rs  → OsFamily, Distro, KernelRelease, CpuArch, SubstrateIdentity
```
