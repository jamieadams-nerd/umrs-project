// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # Posture Indicators — Core Taxonomy Types
//!
//! Core indicator taxonomy types for the Kernel Security Posture Probe.
//!
//! Defines the typed vocabulary used throughout the posture module:
//! indicator identifiers, classification axes, impact tiers, and the
//! desired-value model that drives hardening assessment.
//!
//! ## Compliance
//!
//! NIST SP 800-53 AU-3: Audit record content — every indicator report carries a
//! typed `IndicatorId` rather than a raw string, enabling programmatic filtering
//! and machine-readable audit trails.
//!
//! NIST SP 800-53 CM-6: Configuration Settings — `IndicatorClass` distinguishes
//! runtime-effective from boot-persistent configuration sources, which is
//! essential for contradiction detection.
//!
//! NIST SP 800-53 CA-7: Continuous Monitoring — `AssuranceImpact` lets callers
//! prioritise monitoring effort by security relevance.
//!
//! NSA RTB: Compile-Time Path Binding — `IndicatorId` variants are an exhaustive
//! compile-time enumeration; no string-based dispatch is possible.

// ===========================================================================
// IndicatorId
// ===========================================================================

/// Unique identifier for each kernel security posture indicator.
///
/// Variants are grouped by security domain and ordered by catalog number for
/// stable iteration. The enum is `Copy` and `Hash` to support use as a map key
/// and efficient pass-by-value throughout the posture pipeline.
///
/// ## Variants:
///
/// *Kernel Self-Protection*
/// - `KptrRestrict` — `kernel.kptr_restrict`: restrict exposure of kernel pointers in
///   procfs and other interfaces.
/// - `RandomizeVaSpace` — `kernel.randomize_va_space`: ASLR level (0=off, 1=partial,
///   2=full).
/// - `UnprivBpfDisabled` — `kernel.unprivileged_bpf_disabled`: prevent unprivileged BPF
///   program loading (attack surface reduction).
/// - `PerfEventParanoid` — `kernel.perf_event_paranoid`: restrict perf_event_open() access
///   for unprivileged users.
/// - `YamaPtraceScope` — `kernel.yama.ptrace_scope`: YAMA ptrace restriction level.
/// - `DmesgRestrict` — `kernel.dmesg_restrict`: restrict dmesg access to privileged users.
/// - `KexecLoadDisabled` — `kernel.kexec_load_disabled`: disable the kexec_load() syscall
///   (prevents runtime kernel replacement).
/// - `Sysrq` — `kernel.sysrq`: SysRq key bitmask; 0 = fully disabled.
///
/// *Kernel Integrity*
/// - `ModulesDisabled` — `kernel.modules_disabled`: one-way latch preventing further kernel
///   module loading. Reuses `ModuleLoadLatch` from kattrs.
///
/// *Process Isolation*
/// - `UnprivUsernsClone` — `kernel.unprivileged_userns_clone`: block unprivileged user
///   namespace creation (reduces container escape surface).
///
/// *Filesystem Safety*
/// - `ProtectedSymlinks` — `fs.protected_symlinks`: prevent symlink following by non-owners
///   in sticky directories.
/// - `ProtectedHardlinks` — `fs.protected_hardlinks`: prevent hardlink creation to files
///   the caller does not own.
/// - `ProtectedFifos` — `fs.protected_fifos`: prevent privileged processes writing to
///   attacker-created FIFOs in sticky directories.
/// - `ProtectedRegular` — `fs.protected_regular`: prevent privileged processes writing to
///   attacker-created regular files in sticky directories.
/// - `SuidDumpable` — `fs.suid_dumpable`: controls whether SUID processes produce core
///   dumps.
///
/// *Boot-time / Kernel Cmdline*
/// - `Lockdown` — `lockdown=`: kernel lockdown level at boot. Reuses `KernelLockdown`
///   from kattrs.
/// - `ModuleSigEnforce` — `module.sig_enforce`: enforce kernel module signature
///   verification.
/// - `Mitigations` — `mitigations=`: umbrella cmdline flag governing CPU vulnerability
///   mitigations (spectre, meltdown, etc.).
/// - `Pti` — `pti=`: Page Table Isolation (Meltdown mitigation) override.
/// - `RandomTrustCpu` — `random.trust_cpu`: whether to trust the CPU hardware RNG for
///   early entropy seeding.
/// - `RandomTrustBootloader` — `random.trust_bootloader`: whether to trust the
///   bootloader-provided seed for early entropy.
///
/// *Special*
/// - `FipsEnabled` — `/proc/sys/crypto/fips_enabled`: FIPS 140-2/3 mode active. Reuses
///   `ProcFips` from kattrs.
///
/// *modprobe.d (Phase 2a)*
/// - `NfConntrackAcct` — `nf_conntrack acct`: connection tracking accounting for audit
///   trails. Configured via `options nf_conntrack acct=1`. Live value from
///   `/sys/module/nf_conntrack/parameters/acct`.
/// - `BluetoothBlacklisted` — `bluetooth` blacklisted in modprobe.d; Bluetooth stack is
///   an attack surface on servers.
/// - `UsbStorageBlacklisted` — `usb_storage` blacklisted in modprobe.d; USB mass storage
///   is a data exfiltration vector.
/// - `FirewireCoreBlacklisted` — `firewire_core` blacklisted in modprobe.d; FireWire DMA
///   attacks bypass memory protection.
/// - `ThunderboltBlacklisted` — `thunderbolt` blacklisted in modprobe.d; Thunderbolt DMA
///   attacks bypass memory protection.
///
/// *CPU Mitigation Sub-indicators (Phase 2b)* — each checks that a specific CVE
/// weakening override flag is ABSENT from `/proc/cmdline`
/// (`DesiredValue::CmdlineAbsent`). (NIST SP 800-53 SI-16; NSA RTB)
/// - `SpectreV2Off` — `spectre_v2=off` must be absent; disables Spectre v2 mitigation
///   (retpoline, IBRS, EIBRS).
/// - `SpectreV2UserOff` — `spectre_v2_user=off` must be absent; disables user-space
///   Spectre v2 mitigation (IBPB/STIBP).
/// - `MdsOff` — `mds=off` must be absent; disables MDS/RIDL/Fallout/ZombieLoad
///   mitigations (CVE-2018-12126 et al.).
/// - `TsxAsyncAbortOff` — `tsx_async_abort=off` must be absent; disables TSX Async Abort
///   mitigation (CVE-2019-11135).
/// - `L1tfOff` — `l1tf=off` must be absent; disables L1 Terminal Fault mitigation
///   (CVE-2018-3615/3620/3646).
/// - `RetbleedOff` — `retbleed=off` must be absent; disables RETBLEED mitigation
///   (CVE-2022-29900/29901).
/// - `SrbdsOff` — `srbds=off` must be absent; disables Special Register Buffer Data
///   Sampling mitigation (CVE-2020-0543).
/// - `NoSmtOff` — `nosmt=off` must be absent; re-enables SMT when `nosmt` was set,
///   weakening MDS, L1TF, and cross-HT attack mitigations.
///
/// *Kernel Core Dump (Phase 2b)*
/// - `CorePattern` — `kernel.core_pattern`: core dump disposition. Hardened state: value
///   begins with `|` (piped to a registered handler such as `systemd-coredump`). Validated
///   via TPI: structural (first byte is `|`) and semantic (handler path is a non-empty
///   absolute path); fail closed on disagreement. (NIST SP 800-53 SC-28, CM-6)
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: typed indicator identity for audit records — avoids
///   free-form strings that could be mis-parsed by downstream consumers.
/// - **NSA RTB**: Compile-Time Path Binding — the compiler enforces exhaustive handling
///   of all variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndicatorId {
    // ── Kernel Self-Protection ──────────────────────────────────────────
    KptrRestrict,
    RandomizeVaSpace,
    UnprivBpfDisabled,
    PerfEventParanoid,
    YamaPtraceScope,
    DmesgRestrict,
    KexecLoadDisabled,
    Sysrq,

    // ── Kernel Integrity ────────────────────────────────────────────────
    ModulesDisabled,

    // ── Process Isolation ───────────────────────────────────────────────
    UnprivUsernsClone,

    // ── Filesystem Safety ───────────────────────────────────────────────
    ProtectedSymlinks,
    ProtectedHardlinks,
    ProtectedFifos,
    ProtectedRegular,
    SuidDumpable,

    // ── Boot-time / kernel cmdline ──────────────────────────────────────
    Lockdown,
    ModuleSigEnforce,
    Mitigations,
    Pti,
    RandomTrustCpu,
    RandomTrustBootloader,

    // ── Special ─────────────────────────────────────────────────────────
    FipsEnabled,

    // ── modprobe.d (Phase 2a) ────────────────────────────────────────────
    NfConntrackAcct,
    BluetoothBlacklisted,
    UsbStorageBlacklisted,
    FirewireCoreBlacklisted,
    ThunderboltBlacklisted,

    // ── CPU mitigation sub-indicators (Phase 2b) ─────────────────────────
    // These indicators complement the umbrella `Mitigations` indicator by
    // checking individual per-CVE weakening flags on `/proc/cmdline`. Each
    // checks that the specific weakening override is ABSENT (present = hardening
    // failure). All are `KernelCmdline` class with `DesiredValue::CmdlineAbsent`.
    //
    // NIST SP 800-53 SI-16: Memory Protection — per-CVE mitigation overrides.
    // NSA RTB: CPU vulnerability mitigations must not be individually disabled.
    SpectreV2Off,
    SpectreV2UserOff,
    MdsOff,
    TsxAsyncAbortOff,
    L1tfOff,
    RetbleedOff,
    SrbdsOff,
    NoSmtOff,

    // ── Kernel core dump (Phase 2b) ──────────────────────────────────────
    CorePattern,
}

impl IndicatorId {
    /// Returns a stable, human-readable label for display in audit output.
    ///
    /// The label mirrors the sysctl key or kernel parameter name where
    /// applicable, providing a direct cross-reference to kernel documentation.
    #[must_use = "indicator labels are used for display and audit output"]
    pub const fn label(self) -> &'static str {
        match self {
            Self::KptrRestrict => "kernel.kptr_restrict",
            Self::RandomizeVaSpace => "kernel.randomize_va_space",
            Self::UnprivBpfDisabled => "kernel.unprivileged_bpf_disabled",
            Self::PerfEventParanoid => "kernel.perf_event_paranoid",
            Self::YamaPtraceScope => "kernel.yama.ptrace_scope",
            Self::DmesgRestrict => "kernel.dmesg_restrict",
            Self::KexecLoadDisabled => "kernel.kexec_load_disabled",
            Self::Sysrq => "kernel.sysrq",
            Self::ModulesDisabled => "kernel.modules_disabled",
            Self::UnprivUsernsClone => "kernel.unprivileged_userns_clone",
            Self::ProtectedSymlinks => "fs.protected_symlinks",
            Self::ProtectedHardlinks => "fs.protected_hardlinks",
            Self::ProtectedFifos => "fs.protected_fifos",
            Self::ProtectedRegular => "fs.protected_regular",
            Self::SuidDumpable => "fs.suid_dumpable",
            Self::Lockdown => "lockdown=",
            Self::ModuleSigEnforce => "module.sig_enforce",
            Self::Mitigations => "mitigations=",
            Self::Pti => "pti=",
            Self::RandomTrustCpu => "random.trust_cpu",
            Self::RandomTrustBootloader => "random.trust_bootloader",
            Self::FipsEnabled => "crypto.fips_enabled",
            Self::NfConntrackAcct => "modprobe:nf_conntrack/acct",
            Self::BluetoothBlacklisted => "modprobe:bluetooth/blacklisted",
            Self::UsbStorageBlacklisted => "modprobe:usb_storage/blacklisted",
            Self::FirewireCoreBlacklisted => "modprobe:firewire_core/blacklisted",
            Self::ThunderboltBlacklisted => "modprobe:thunderbolt/blacklisted",
            Self::SpectreV2Off => "spectre_v2=off",
            Self::SpectreV2UserOff => "spectre_v2_user=off",
            Self::MdsOff => "mds=off",
            Self::TsxAsyncAbortOff => "tsx_async_abort=off",
            Self::L1tfOff => "l1tf=off",
            Self::RetbleedOff => "retbleed=off",
            Self::SrbdsOff => "srbds=off",
            Self::NoSmtOff => "nosmt=off",
            Self::CorePattern => "kernel.core_pattern",
        }
    }
}

impl std::fmt::Display for IndicatorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

// ===========================================================================
// IndicatorClass
// ===========================================================================

/// How the indicator is persisted and where its live value is read from.
///
/// Distinguishes between runtime-effective and boot-persistent configuration
/// sources — essential for contradiction detection and for informing operators
/// which reboot/sysctl command would remediate a finding.
///
/// ## Variants:
///
/// - `Sysctl` — runtime sysctl: live value from `/proc/sys/*`, configured value from
///   the sysctl.d merge tree. Changes can take effect without a reboot.
/// - `KernelCmdline` — kernel command line: live value from `/proc/cmdline`, configured
///   value from bootloader entries (Phase 2). A reboot is required to change these.
/// - `SecurityFs` — security filesystem: live value from `/sys/kernel/security/`
///   (`SECURITYFS_MAGIC`). Controlled by kernel LSM state. Contradiction detection is
///   not applicable (Phase 1). Example: `Lockdown` reads `/sys/kernel/security/lockdown`.
///   (NSA RTB: paths verified against `SECURITYFS_MAGIC` at read time)
/// - `DistroManaged` — live value from a kernel interface, but the canonical configuration
///   channel is a distro tool (e.g., `fips-mode-setup`, `mokutil`). Configured value
///   discovery is distro-specific.
/// - `ModprobeConfig` — modprobe.d configured: live value from
///   `/sys/module/<mod>/parameters/` or inferred from module-directory presence;
///   configured value from the modprobe.d merge tree. Provenance-verified via
///   `SYSFS_MAGIC`. (NSA RTB: sysfs reads verified against `SYSFS_MAGIC` at read time)
///
/// ## Compliance
///
/// - **NIST SP 800-53 CM-6**: provenance of the effective value determines the
///   remediation path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorClass {
    Sysctl,
    KernelCmdline,
    SecurityFs,
    DistroManaged,
    ModprobeConfig,
}

// ===========================================================================
// AssuranceImpact
// ===========================================================================

/// Security impact tier for a posture indicator.
///
/// Ordered ascending (`Medium < High < Critical`) to support
/// `by_impact(min: AssuranceImpact)` filtering via `>=` comparison.
///
/// ## Variants:
///
/// - `Medium` — meaningful security improvement but limited blast radius.
/// - `High` — significant hardening; failure provides a useful attack primitive.
/// - `Critical` — foundational control; failure directly enables serious attacks
///   (kernel pointer leaks, full ASLR bypass, persistent module loading).
///
/// ## Compliance
///
/// - **NIST SP 800-53 CA-7**: impact tier drives prioritisation of monitoring, alerting,
///   and remediation effort.
/// - **NIST SP 800-53 RA-3**: impact tiers align with risk severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssuranceImpact {
    Medium,
    High,
    Critical,
}

// ===========================================================================
// DesiredValue
// ===========================================================================

/// The recommended value for a hardened system.
///
/// Expressed as a structured enum so that comparison logic is type-safe and
/// does not rely on string matching. The `meets` method implements the
/// hardening check for each variant.
///
/// ## Variants:
///
/// - `Exact(u32)` — live value must equal this integer exactly (e.g., `kptr_restrict = 2`).
/// - `AtLeast(u32)` — live value must be ≥ this threshold (e.g., `perf_event_paranoid >= 2`).
/// - `AtMost(u32)` — live value must be ≤ this threshold (e.g., `suid_dumpable <= 0`).
/// - `CmdlinePresent(&str)` — this token must be present in `/proc/cmdline`
///   (e.g., `module.sig_enforce=1`).
/// - `CmdlineAbsent(&str)` — this token must NOT be present in `/proc/cmdline`
///   (e.g., `mitigations=off`).
/// - `Custom` — evaluated by indicator-specific logic (e.g., `kernel.sysrq`). The
///   `meets_integer` and `meets_signed_integer` methods return `None` for this variant;
///   callers must invoke the indicator-specific validator instead.
///
/// ## Compliance
///
/// - **NIST SP 800-53 CM-6**: the desired value is the security baseline against which
///   live and configured values are measured.
/// - **NIST SP 800-53 SI-10**: `meets` rejects unrecognised values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesiredValue {
    Exact(u32),
    AtLeast(u32),
    AtMost(u32),
    CmdlinePresent(&'static str),
    CmdlineAbsent(&'static str),
    Custom,
}

impl DesiredValue {
    /// Test whether an integer live value meets this desired value.
    ///
    /// Returns `Some(true)` if the check passes, `Some(false)` if it fails,
    /// or `None` for variants that require non-integer input
    /// (`CmdlinePresent`, `CmdlineAbsent`, `Custom`).
    #[must_use = "hardening check result must be examined"]
    pub const fn meets_integer(&self, live: u32) -> Option<bool> {
        match self {
            Self::Exact(v) => Some(live == *v),
            Self::AtLeast(v) => Some(live >= *v),
            Self::AtMost(v) => Some(live <= *v),
            Self::CmdlinePresent(_) | Self::CmdlineAbsent(_) | Self::Custom => None,
        }
    }

    /// Test whether a signed integer live value meets this desired value.
    ///
    /// Used for sysctl nodes that legitimately emit negative values, such as
    /// `kernel.perf_event_paranoid` which can be `-1` (unrestricted). A
    /// negative value always fails `AtLeast(n)` for any non-negative `n`.
    ///
    /// Comparison is performed in `i64` to avoid overflow when the threshold
    /// `v` is large (e.g., close to `u32::MAX` widened to `i64`).
    ///
    /// Returns `None` for `CmdlinePresent`, `CmdlineAbsent`, and `Custom`.
    ///
    /// NIST SP 800-53 CA-7: must not discard valid kernel states; `-1` must
    /// produce `Some(false)` rather than a parse error.
    #[must_use = "hardening check result must be examined"]
    pub fn meets_signed_integer(&self, live: i32) -> Option<bool> {
        let live64 = i64::from(live);
        match self {
            Self::Exact(v) => Some(live64 == i64::from(*v)),
            Self::AtLeast(v) => Some(live64 >= i64::from(*v)),
            Self::AtMost(v) => Some(live64 <= i64::from(*v)),
            Self::CmdlinePresent(_) | Self::CmdlineAbsent(_) | Self::Custom => None,
        }
    }

    /// Test whether `/proc/cmdline` content meets this desired value.
    ///
    /// Returns `Some(true)` if the check passes, `Some(false)` if it fails,
    /// or `None` for variants that require integer input
    /// (`Exact`, `AtLeast`, `AtMost`, `Custom`).
    #[must_use = "hardening check result must be examined"]
    pub fn meets_cmdline(&self, cmdline: &str) -> Option<bool> {
        match self {
            Self::CmdlinePresent(token) => Some(cmdline_contains(cmdline, token)),
            Self::CmdlineAbsent(token) => Some(!cmdline_contains(cmdline, token)),
            Self::Exact(_) | Self::AtLeast(_) | Self::AtMost(_) | Self::Custom => None,
        }
    }
}

/// Check whether a cmdline string contains a given token as a whole word
/// (space-delimited).
///
/// Avoids false positives from prefix matches: `mitigations=off` should not
/// match `mitigations=off,nosmt`.
///
/// NIST SP 800-53 SI-10: Input Validation — token comparison is exact and
/// delimiter-aware.
fn cmdline_contains(cmdline: &str, token: &str) -> bool {
    cmdline.split_whitespace().any(|t| t == token)
}

// ===========================================================================
// LiveValue / ConfiguredValue wrappers
// ===========================================================================

/// The live (kernel-effective) value of a posture indicator.
///
/// Wraps the raw parsed value from the kernel interface. Integer values
/// are used for sysctl indicators; string values for cmdline and special
/// indicators.
///
/// `SignedInteger` is reserved for sysctl nodes that legitimately emit
/// negative values (e.g., `kernel.perf_event_paranoid = -1` means
/// "unrestricted for all users"). Using a distinct variant preserves the raw
/// signed value for display and audit output while enabling correct comparison
/// against unsigned thresholds via `DesiredValue::meets_signed_integer`.
///
/// ## Variants:
///
/// - `Integer(u32)` — a sysctl unsigned integer value.
/// - `SignedInteger(i32)` — a sysctl signed integer value for nodes that can emit negative
///   values (e.g., `kernel.perf_event_paranoid = -1` means "unrestricted"). A negative
///   value is a kernel-valid unhardened state; it must be represented faithfully.
///   (NIST SP 800-53 CA-7)
/// - `Text(String)` — a string value (cmdline token, lockdown mode, etc.).
/// - `Bool(bool)` — a boolean value (e.g., FIPS enabled, modules_disabled).
///
/// ## Compliance
///
/// - **NIST SP 800-53 CA-7**: accurate representation of all kernel-valid values is
///   required for reliable continuous monitoring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveValue {
    Integer(u32),
    SignedInteger(i32),
    Text(String),
    Bool(bool),
}

impl std::fmt::Display for LiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(v) => write!(f, "{v}"),
            Self::SignedInteger(v) => write!(f, "{v}"),
            Self::Text(s) => f.write_str(s),
            Self::Bool(b) => write!(f, "{b}"),
        }
    }
}

/// The configured (intended) value of a posture indicator, as read from the
/// sysctl.d merge tree or other persistence source.
///
/// Always a string because sysctl.d files store values as text.
///
/// ## Fields:
///
/// - `raw` — the raw string value from the configuration file.
/// - `source_file` — the file that last set this value (last-writer-wins in precedence
///   order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfiguredValue {
    pub raw: String,
    pub source_file: String,
}
