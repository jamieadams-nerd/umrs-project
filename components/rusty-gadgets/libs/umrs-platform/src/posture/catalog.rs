// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Static indicator catalog — compile-time array of `IndicatorDescriptor` entries.
//!
//! Every kernel security posture indicator known to Phase 1, Phase 2a, and
//! Phase 2b is described here as a `const` value. The catalog is the single
//! authoritative source for indicator metadata: paths, sysctl keys, desired
//! values, impact tiers, and rationale text.
//!
//! ## Design
//!
//! The catalog is a `const` Rust array rather than an external file to satisfy
//! the Compile-Time Path Binding Rule: the compiler verifies every path, key,
//! and desired-value expression at build time. There is no runtime I/O error
//! path for catalog access, no deserialization surface, and no risk of a
//! substituted data file.
//!
//! When a new indicator is added, the change must pass code review — there is
//! no silent data-file edit path. This property is important for auditability
//! under NIST SP 800-218 SSDF PW.4.
//!
//! ## Compliance
//!
//! NIST SP 800-53 CM-6: Configuration Settings — the catalog defines the security
//! baseline against which runtime state is measured.
//! NIST SP 800-53 CA-7: Continuous Monitoring — catalog entries enumerate the
//! complete set of monitored security controls.
//! NIST SP 800-218 SSDF PW.4: Compile-time binding of security-relevant constants.
//! NSA RTB: Compile-Time Path Binding — paths and expected values are
//! compiler-verified, not runtime-parsed.

use super::indicator::{AssuranceImpact, DesiredValue, IndicatorClass, IndicatorId};

// ===========================================================================
// Catalog baseline version
// ===========================================================================

/// The kernel release version against which this catalog's indicator
/// definitions were authored and validated.
///
/// This is the `major.minor.patch` prefix of the kernel release string —
/// no distribution suffix, no architecture suffix. The running kernel's
/// release string (from `uname(2)`) is compared against this value at
/// display time so operators can see whether the catalog is current for
/// their kernel.
///
/// Update this constant whenever the catalog is reviewed and updated
/// against a new kernel release. The value must be a valid semver-style
/// `"MAJOR.MINOR.PATCH"` string (three dot-separated decimal integers).
///
/// NIST SP 800-53 CM-6: Configuration Settings — operators must know
/// whether the running kernel is covered by the current indicator baseline.
/// NIST SP 800-53 CA-7: Continuous Monitoring — catalog currency is a
/// precondition for meaningful posture assessment.
pub const CATALOG_KERNEL_BASELINE: &str = "6.12.0";

// ===========================================================================
// IndicatorDescriptor
// ===========================================================================

/// Compile-time catalog entry describing one kernel security posture indicator.
///
/// All instances are `const` and live in the `INDICATORS` array. No heap
/// allocation is required to access the catalog.
///
/// NIST SP 800-53 CM-6: each descriptor captures the security baseline
/// (desired value) alongside its rationale and NIST control citation.
/// NIST SP 800-53 AU-3: `nist_controls` provides the audit control mapping
/// needed for compliance evidence generation.
/// NIST SP 800-53 CA-2: `cce` provides assessment evidence traceability to
/// the RHEL 10 STIG SCAP content where a direct mapping exists.
pub struct IndicatorDescriptor {
    /// The typed indicator identifier.
    pub id: IndicatorId,
    /// How this indicator is persisted and read.
    pub class: IndicatorClass,
    /// The kernel pseudo-filesystem path from which the live value is read.
    /// For `KernelCmdline` indicators this is `/proc/cmdline`.
    pub live_path: &'static str,
    /// The sysctl key used to look up this indicator in sysctl.d merge output.
    /// `None` for cmdline and special indicators that have no sysctl key.
    pub sysctl_key: Option<&'static str>,
    /// The hardened desired value for this indicator.
    pub desired: DesiredValue,
    /// Security impact of this indicator not meeting its desired value.
    pub impact: AssuranceImpact,
    /// Short human-readable label suitable for column headers and compact display.
    ///
    /// Typically 2–4 words. Consumers should prefer `label` over truncating
    /// `rationale` for space-constrained output such as TUI columns or audit
    /// report tables.
    ///
    /// NIST SP 800-53 AU-3 — audit records must be identifiable by a concise,
    /// stable label that does not require parsing the full rationale.
    pub label: &'static str,
    /// One-sentence rationale for the desired value.
    pub rationale: &'static str,
    /// Applicable NIST SP 800-53 and NSA RTB control references.
    pub nist_controls: &'static str,
    /// CCE identifier from the RHEL 10 STIG, if this indicator has a SCAP
    /// equivalent. Format: `CCE-NNNNN-N`. `None` for indicators that exceed
    /// STIG coverage (UMRS-only hardening checks with no direct STIG analog).
    ///
    /// NIST SP 800-53 CA-2: assessment evidence traceability — links this
    /// indicator to its authoritative SCAP content identifier for use in
    /// automated compliance reporting and STIG gap analysis.
    pub cce: Option<&'static str>,
    /// Multi-sentence description explaining what this indicator controls and
    /// what the security impact is if it is not hardened.
    ///
    /// Intended for operator-facing display in TUI, CLI, and assessment reports.
    /// Empty string for indicators that do not yet have display-layer coverage
    /// (e.g., CPU mitigation sub-indicators pending Phase 2b TUI integration).
    ///
    /// NIST SP 800-53 SA-5: system documentation — inline descriptions reduce
    /// operator reliance on external reference guides.
    pub description: &'static str,
    /// Operator-facing remediation guidance shown when the indicator does not
    /// meet the desired baseline. `None` when no single remediation action is
    /// defined (e.g., indicators with site-policy-dependent desired values, or
    /// CPU mitigation sub-indicators whose remediation is context-dependent).
    ///
    /// NIST SP 800-53 CM-6: configuration settings — remediation guidance
    /// accompanies each failing configuration finding.
    pub recommended: Option<&'static str>,
}

// ===========================================================================
// INDICATORS — static catalog
// ===========================================================================

/// Static catalog of all kernel security posture indicators (Phase 1–2b).
///
/// Ordered by `IndicatorId` grouping (kernel self-protection → integrity →
/// process isolation → filesystem safety → boot-time → special).
///
/// NIST SP 800-53 CA-7: the catalog is the enumerated monitoring scope.
/// NIST SP 800-53 CM-6: each entry encodes the security baseline.
pub static INDICATORS: &[IndicatorDescriptor] = &[
    // ── Kernel Self-Protection ───────────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::KptrRestrict,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/kptr_restrict",
        sysctl_key: Some("kernel.kptr_restrict"),
        desired: DesiredValue::Exact(2),
        impact: AssuranceImpact::Critical,
        label: "Kernel Pointer Restriction",
        rationale: "Level 2 blocks kernel pointer exposure in all contexts, \
                    preventing KASLR bypass via /proc leaks.",
        nist_controls: "NIST 800-53 CM-6(a), SC-30, SC-30(2), SC-30(5); \
                        NSA RTB: minimized information disclosure",
        cce: Some("CCE-88686-1"),
        description: "Hides kernel pointer addresses from /proc and logs. Knowing \
                      where kernel code lives in memory is the first step in most \
                      kernel exploits.",
        recommended: Some("2 (hidden from all users)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::RandomizeVaSpace,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/randomize_va_space",
        sysctl_key: Some("kernel.randomize_va_space"),
        desired: DesiredValue::Exact(2),
        impact: AssuranceImpact::Critical,
        label: "ASLR",
        rationale: "Full ASLR (level 2) randomises stack, mmap, and heap bases, \
                    making memory-corruption exploits significantly harder.",
        nist_controls: "NIST 800-53 CM-6(a), SC-30, SC-30(2); \
                        NSA RTB: exploit mitigation",
        cce: Some("CCE-87876-9"),
        description: "ASLR randomizes stack, heap, and library addresses on every \
                      process start. Without it, memory-corruption exploits can jump \
                      to known addresses reliably.",
        recommended: Some("2 (full ASLR)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::UnprivBpfDisabled,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/unprivileged_bpf_disabled",
        sysctl_key: Some("kernel.unprivileged_bpf_disabled"),
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::High,
        label: "Unprivileged BPF",
        rationale: "Unprivileged BPF programs have been a recurring source of \
                    kernel exploits; restricting to CAP_BPF/CAP_SYS_ADMIN reduces attack surface.",
        nist_controls: "NIST 800-53 AC-6, SC-7(10); NSA RTB: attack surface reduction",
        cce: Some("CCE-89405-5"),
        description: "Prevents unprivileged users from loading BPF programs. The BPF \
                      JIT and verifier are complex attack surfaces responsible for many \
                      kernel privilege escalation CVEs.",
        recommended: Some("1 (restricted to CAP_BPF)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::PerfEventParanoid,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/perf_event_paranoid",
        sysctl_key: Some("kernel.perf_event_paranoid"),
        desired: DesiredValue::AtLeast(2),
        impact: AssuranceImpact::High,
        label: "Perf Event Access",
        rationale: "Level >=2 restricts perf_event_open() to privileged users, \
                    blocking side-channel attacks and profiling-based ASLR leaks.",
        nist_controls: "NIST 800-53 AC-6; NSA RTB: information disclosure prevention",
        cce: Some("CCE-90142-1"),
        description: "Limits access to CPU performance counters. Performance counters \
                      can be used as side-channels to leak information across processes \
                      or bypass ASLR by inferring memory layout.",
        recommended: Some("2 (restricted)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::YamaPtraceScope,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/yama/ptrace_scope",
        sysctl_key: Some("kernel.yama.ptrace_scope"),
        desired: DesiredValue::AtLeast(1),
        impact: AssuranceImpact::High,
        label: "Ptrace Scope",
        rationale: "YAMA scope >=1 restricts ptrace to parent/child relationships, \
                    preventing credential extraction from sibling processes.",
        nist_controls: "NIST 800-53 SC-7(10), AC-6; NSA RTB: process isolation",
        cce: Some("CCE-88785-1"),
        description: "Controls which processes can attach to another via ptrace. \
                      Without restriction, a compromised process can extract passwords \
                      and private keys from every sibling process.",
        recommended: Some("1 (children only)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::DmesgRestrict,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/dmesg_restrict",
        sysctl_key: Some("kernel.dmesg_restrict"),
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::Medium,
        label: "Dmesg Access",
        rationale: "Restricts dmesg to CAP_SYSLOG, preventing unprivileged users \
                    from reading kernel addresses and capability-related messages.",
        nist_controls: "NIST 800-53 SI-11(a), SI-11(b); \
                        NSA RTB: information disclosure prevention",
        cce: Some("CCE-89000-4"),
        description: "Prevents unprivileged users from reading the kernel message \
                      buffer. dmesg often contains hardware addresses and capability \
                      messages useful for kernel exploit development.",
        recommended: Some("1 (restricted to CAP_SYSLOG)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::KexecLoadDisabled,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/kexec_load_disabled",
        sysctl_key: Some("kernel.kexec_load_disabled"),
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::Critical,
        label: "Kexec Load",
        rationale: "Disabling kexec_load() prevents runtime kernel replacement \
                    by root, preserving boot-time integrity guarantees (Secure Boot, IMA).",
        nist_controls: "NIST 800-53 CM-6, SI-7; NSA RTB: boot integrity",
        cce: Some("CCE-89232-3"),
        description: "Prevents loading a new kernel image at runtime. Without this, \
                      an attacker with root can replace the running kernel without a \
                      reboot, bypassing Secure Boot.",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::Sysrq,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/sysrq",
        sysctl_key: Some("kernel.sysrq"),
        // Custom: bitmask semantics are site-policy-dependent.
        // Default hardened check: value == 0 (fully disabled).
        // Sites permitting restricted sysrq (e.g., 176 = sync+remount+reboot)
        // should document their policy and apply a custom validator.
        desired: DesiredValue::Custom,
        impact: AssuranceImpact::Medium,
        label: "SysRq Key",
        rationale: "SysRq key can bypass access controls at the console; \
                    fully disabling (0) is safest on production servers.",
        nist_controls: "NIST 800-53 AC-3, CM-7; NSA RTB: attack surface reduction",
        cce: None,
        description: "Controls which Magic SysRq key combinations are active. On a \
                      system with console access, SysRq can kill security processes or \
                      force a reboot without authentication.",
        recommended: Some("0 (fully disabled)"),
    },
    // ── Kernel Integrity ─────────────────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::ModulesDisabled,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/modules_disabled",
        sysctl_key: Some("kernel.modules_disabled"),
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::Critical,
        label: "Module Load Latch",
        rationale: "One-way latch: once set, no further kernel modules can be \
                    loaded, locking the kernel attack surface for the lifetime of the boot.",
        nist_controls: "NIST 800-53 CM-7, SI-7; NSA RTB: minimised attack surface",
        cce: None,
        description: "One-way latch: once set, no new kernel modules can be loaded. \
                      Prevents rootkits and SELinux bypass by freezing the kernel \
                      attack surface after boot.",
        recommended: Some("1 (loading locked)"),
    },
    // ── Process Isolation ────────────────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::UnprivUsernsClone,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/unprivileged_userns_clone",
        sysctl_key: Some("kernel.unprivileged_userns_clone"),
        desired: DesiredValue::Exact(0),
        impact: AssuranceImpact::High,
        label: "User Namespaces",
        rationale: "Unprivileged user namespaces are a primary container-escape \
                    vector; blocking them (0) reduces the attack surface significantly.",
        nist_controls: "NIST 800-53 SC-39, CM-7; NSA RTB: process isolation",
        cce: None,
        description: "Prevents unprivileged users from creating user namespaces. \
                      Unprivileged user namespaces are one of the most commonly \
                      exploited kernel features for container escapes.",
        recommended: Some("0 (restricted to root)"),
    },
    // ── Filesystem Safety ────────────────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::ProtectedSymlinks,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/fs/protected_symlinks",
        sysctl_key: Some("fs.protected_symlinks"),
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::High,
        label: "Symlink Protection",
        rationale: "Prevents TOCTOU symlink attacks in world-writable sticky \
                    directories (e.g., /tmp); following symlinks owned by others is blocked.",
        nist_controls: "NIST 800-53 AC-6(1), CM-6(a); NSA RTB: filesystem hardening",
        cce: Some("CCE-88796-8"),
        description: "Prevents following symlinks in world-writable directories when \
                      owned by someone other than the caller. Blocks classic TOCTOU \
                      attacks using /tmp symlinks.",
        recommended: Some("1 (protected)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::ProtectedHardlinks,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/fs/protected_hardlinks",
        sysctl_key: Some("fs.protected_hardlinks"),
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::High,
        label: "Hardlink Protection",
        rationale: "Prevents creation of hardlinks to files the caller does not \
                    own, blocking privilege escalation via SUID binaries.",
        nist_controls: "NIST 800-53 AC-6(1), CM-6(a); NSA RTB: filesystem hardening",
        cce: Some("CCE-86689-7"),
        description: "Prevents creating hard links to files the caller does not own. \
                      Without this, an attacker can hardlink a SUID binary into a \
                      directory they control.",
        recommended: Some("1 (protected)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::ProtectedFifos,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/fs/protected_fifos",
        sysctl_key: Some("fs.protected_fifos"),
        desired: DesiredValue::Exact(2),
        impact: AssuranceImpact::Medium,
        label: "FIFO Protection",
        rationale: "Level 2 prevents privileged processes from writing to FIFOs \
                    in world-writable sticky directories that they do not own.",
        nist_controls: "NIST 800-53 SI-10, CM-7; NSA RTB: filesystem hardening",
        cce: None,
        description: "Prevents privileged processes from writing to FIFOs they do not \
                      own in sticky directories. Blocks data interception via \
                      predictable pipe names in /tmp.",
        recommended: Some("2 (fully protected)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::ProtectedRegular,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/fs/protected_regular",
        sysctl_key: Some("fs.protected_regular"),
        desired: DesiredValue::Exact(2),
        impact: AssuranceImpact::Medium,
        label: "Regular File Protection",
        rationale: "Level 2 prevents privileged processes from writing to regular \
                    files in world-writable sticky directories that they do not own.",
        nist_controls: "NIST 800-53 SI-10, CM-7; NSA RTB: filesystem hardening",
        cce: None,
        description: "Prevents privileged processes from writing to regular files they \
                      do not own in sticky directories. Blocks replacement of trusted \
                      files via predictable paths in /tmp.",
        recommended: Some("2 (fully protected)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::SuidDumpable,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/fs/suid_dumpable",
        sysctl_key: Some("fs.suid_dumpable"),
        desired: DesiredValue::Exact(0),
        impact: AssuranceImpact::High,
        label: "SUID Core Dumps",
        rationale: "Disabling core dumps for SUID processes (0) prevents \
                    credential and key material extraction via coredump files.",
        nist_controls: "NIST 800-53 SC-28, SI-12; NSA RTB: information disclosure prevention",
        // No direct STIG CCE for fs.suid_dumpable. Related STIG rules that address
        // coredump prevention via different mechanisms:
        //   CCE-88330-6 (disable_users_coredumps, PAM limits layer)
        //   CCE-88732-3 (coredump_disable_storage, systemd layer)
        // UMRS checks the kernel sysctl directly.
        cce: None,
        description: "Controls whether SUID and privileged processes produce core dumps. \
                      Core dumps can contain encryption keys, passwords, and session \
                      tokens from process memory.",
        recommended: Some("0 (no core dumps)"),
    },
    // ── Boot-time / kernel cmdline ───────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::Lockdown,
        // SecurityFs: live value from /sys/kernel/security/lockdown (SECURITYFS_MAGIC),
        // not /proc/cmdline. The lockdown LSM state is read via KernelLockdown::read()
        // which verifies SECURITYFS_MAGIC. Contradiction detection is not applicable
        // in Phase 1 — configured-value discovery for LSM state is deferred.
        class: IndicatorClass::SecurityFs,
        live_path: "/sys/kernel/security/lockdown",
        sysctl_key: None,
        desired: DesiredValue::CmdlinePresent("lockdown=integrity"),
        impact: AssuranceImpact::Critical,
        label: "Kernel Lockdown",
        rationale: "Kernel lockdown at integrity level prevents runtime kernel \
                    modification by root, preserving Secure Boot guarantees.",
        nist_controls: "NIST 800-53 CM-7, SI-7; NSA RTB: boot integrity",
        cce: None,
        description: "Kernel lockdown LSM restricts operations that let root modify the \
                      running kernel. Without it, boot-time integrity checks can be \
                      bypassed after the system is up.",
        recommended: Some("integrity or confidentiality (lockdown active)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::ModuleSigEnforce,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlinePresent("module.sig_enforce=1"),
        impact: AssuranceImpact::Critical,
        label: "Module Signature",
        rationale: "Enforcing module signatures at boot prevents loading of \
                    unsigned kernel modules, complementing lockdown mode.",
        nist_controls: "NIST 800-53 SI-7, CM-7; NSA RTB: boot integrity",
        // No direct STIG CCE for module.sig_enforce=1 cmdline enforcement.
        // UMRS proactive check exceeds STIG baseline (STIG audits reactively via
        // audit rules CCE-89982-3, CCE-88638-2, CCE-90172-8 instead).
        cce: None,
        description: "Requires all kernel modules to be cryptographically signed. \
                      Without this, any code can be loaded as a kernel module, \
                      defeating lockdown and enabling rootkits.",
        recommended: Some("module.sig_enforce=1 in cmdline"),
    },
    IndicatorDescriptor {
        id: IndicatorId::Mitigations,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        // The hardened check is: `mitigations=off` must be ABSENT.
        desired: DesiredValue::CmdlineAbsent("mitigations=off"),
        impact: AssuranceImpact::Critical,
        label: "CPU Mitigations",
        rationale: "CPU vulnerability mitigations (Spectre, Meltdown, MDS, etc.) \
                    must not be disabled; `mitigations=off` is a critical weakening flag.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "Umbrella switch: when mitigations=off is present, ALL CPU \
                      vulnerability mitigations (Spectre, Meltdown, MDS) are disabled \
                      at once. Should never be present in production.",
        recommended: Some("remove mitigations=off from cmdline"),
    },
    IndicatorDescriptor {
        id: IndicatorId::Pti,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        // Hardened: `pti=off` must be absent (PTI enabled by default).
        desired: DesiredValue::CmdlineAbsent("pti=off"),
        impact: AssuranceImpact::High,
        label: "Page Table Isolation",
        rationale: "Page Table Isolation (Meltdown mitigation) must not be \
                    explicitly disabled; `pti=off` removes kernel address space protection.",
        nist_controls: "NIST 800-53 SI-16; NSA RTB: CPU vulnerability mitigations",
        cce: Some("CCE-88971-7"),
        description: "Page Table Isolation mitigates Meltdown (CVE-2017-5754). When \
                      pti=off is present, any process can read arbitrary kernel memory \
                      on affected CPUs.",
        recommended: Some("remove pti=off from cmdline"),
    },
    IndicatorDescriptor {
        id: IndicatorId::RandomTrustCpu,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        // On FIPS systems: random.trust_cpu=off is preferred (NIST SP 800-90B).
        // We flag as a finding if trust_cpu=on is present on a FIPS system.
        // Phase 1: simple check — CmdlineAbsent("random.trust_cpu=on") is
        // conservative; Phase 2 can integrate FIPS cross-check.
        desired: DesiredValue::CmdlineAbsent("random.trust_cpu=on"),
        impact: AssuranceImpact::Medium,
        label: "CPU RNG Trust",
        rationale: "Trusting CPU RNG unconditionally may not satisfy NIST SP 800-90B; \
                    RHEL 10 defaults to not trusting it.",
        nist_controls: "NIST 800-53 SC-12; NIST SP 800-90B entropy requirements",
        cce: None,
        description: "Controls whether the kernel unconditionally trusts the CPU RNG \
                      (RDRAND). Trusting it exclusively risks weak key generation if \
                      the CPU RNG is compromised or backdoored.",
        recommended: Some("remove random.trust_cpu=on from cmdline"),
    },
    IndicatorDescriptor {
        id: IndicatorId::RandomTrustBootloader,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("random.trust_bootloader=on"),
        impact: AssuranceImpact::Medium,
        label: "Bootloader RNG Trust",
        rationale: "Trusting bootloader-provided entropy requires a verified \
                    boot chain; absent Secure Boot attestation, the seed is untrusted.",
        nist_controls: "NIST 800-53 SC-12, SI-7; NIST SP 800-90B entropy requirements",
        cce: None,
        description: "Controls whether the kernel trusts entropy from the bootloader. \
                      Without a verified boot chain, a predictable seed can weaken all \
                      cryptographic key generation during that boot.",
        recommended: Some("remove random.trust_bootloader=on from cmdline"),
    },
    // ── Special ──────────────────────────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::FipsEnabled,
        class: IndicatorClass::DistroManaged,
        live_path: "/proc/sys/crypto/fips_enabled",
        sysctl_key: Some("crypto.fips_enabled"),
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::Critical,
        label: "FIPS Mode",
        rationale: "FIPS 140-2/3 mode enforces validated cryptographic primitives \
                    and is a mandatory baseline for DoD/government deployments.",
        nist_controls: "NIST 800-53 SC-13, SC-28; FIPS 140-2/140-3; CMMC SC.L2-3.13.10",
        // CCE-89085-5 covers configure_crypto_policy (policy layer); UMRS checks
        // the kernel enforcement state directly — nearest STIG anchor, not exact equivalent.
        // CCE-89085-5 (RHEL 10 STIG, scap-security-guide 2026-03-17)
        cce: Some("CCE-89085-5"),
        description: "FIPS 140-2/3 mode restricts the system to NIST-validated \
                      cryptographic algorithms. Required for DoD and federal deployments \
                      processing CUI.",
        recommended: Some("1 (FIPS mode active)"),
    },
    // ── modprobe.d (Phase 2a) ─────────────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::NfConntrackAcct,
        class: IndicatorClass::ModprobeConfig,
        live_path: "/sys/module/nf_conntrack/parameters/acct",
        sysctl_key: None,
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::Medium,
        label: "Conntrack Accounting",
        rationale: "Connection tracking accounting (acct=1) enables per-connection \
                    byte/packet counters used by audit and firewall logging tools.",
        nist_controls: "NIST 800-53 AU-12, CM-6; NSA RTB: audit trail completeness",
        cce: None,
        description: "Enables per-connection byte and packet counters in netfilter. \
                      Without it, network audit logs lack the traffic volume data \
                      needed for anomaly detection and forensic reconstruction.",
        recommended: Some("1 (accounting enabled)"),
    },
    IndicatorDescriptor {
        id: IndicatorId::BluetoothBlacklisted,
        class: IndicatorClass::ModprobeConfig,
        live_path: "/sys/module/bluetooth",
        sysctl_key: None,
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::High,
        label: "Bluetooth",
        rationale: "The Bluetooth stack is a large attack surface on servers; \
                    blacklisting prevents accidental or malicious module loading.",
        nist_controls: "NIST 800-53 AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7; \
                        NSA RTB: attack surface reduction; CMMC CM.L2-3.4.6",
        cce: Some("CCE-87455-2"),
        description: "Bluetooth stack is blacklisted and cannot load. The Bluetooth \
                      protocol stack is large, historically vulnerability-prone, and \
                      serves no purpose on server infrastructure.",
        recommended: Some("blacklist bluetooth in modprobe.d"),
    },
    IndicatorDescriptor {
        id: IndicatorId::UsbStorageBlacklisted,
        class: IndicatorClass::ModprobeConfig,
        live_path: "/sys/module/usb_storage",
        sysctl_key: None,
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::High,
        label: "USB Storage",
        rationale: "USB mass storage is a primary data exfiltration vector on \
                    classified and government systems; blacklisting prevents \
                    mounting of untrusted external media.",
        nist_controls: "NIST 800-53 CM-6(a), CM-7(a), CM-7(b), MP-7; \
                        NSA RTB: media protection; CMMC MP.L2-3.8.7",
        cce: Some("CCE-89301-6"),
        description: "USB mass storage module is blacklisted. USB storage is a primary \
                      data exfiltration vector — blacklisting prevents copying data \
                      even if someone physically connects a drive.",
        recommended: Some("blacklist usb_storage in modprobe.d"),
    },
    IndicatorDescriptor {
        id: IndicatorId::FirewireCoreBlacklisted,
        class: IndicatorClass::ModprobeConfig,
        live_path: "/sys/module/firewire_core",
        sysctl_key: None,
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::High,
        label: "FireWire",
        rationale: "FireWire DMA can bypass memory protection; blacklisting \
                    prevents direct memory access attacks via physical FireWire \
                    ports.",
        nist_controls: "NIST 800-53 SI-7, CM-7; NSA RTB: physical attack surface \
                        reduction",
        cce: None,
        description: "FireWire module is blacklisted. FireWire uses DMA with direct \
                      access to system RAM, allowing an attacker with physical access \
                      to read and write arbitrary memory.",
        recommended: Some("blacklist firewire_core in modprobe.d"),
    },
    IndicatorDescriptor {
        id: IndicatorId::ThunderboltBlacklisted,
        class: IndicatorClass::ModprobeConfig,
        live_path: "/sys/module/thunderbolt",
        sysctl_key: None,
        desired: DesiredValue::Exact(1),
        impact: AssuranceImpact::High,
        label: "Thunderbolt",
        rationale: "Thunderbolt DMA can bypass IOMMU protections on some hardware; \
                    blacklisting prevents DMA-based attacks via Thunderbolt ports.",
        nist_controls: "NIST 800-53 SI-7, CM-7; NSA RTB: physical attack surface \
                        reduction; CMMC CM.L2-3.4.6",
        cce: None,
        description: "Thunderbolt module is blacklisted. Like FireWire, Thunderbolt \
                      uses DMA that can bypass IOMMU protections and expose system \
                      memory to physical-access attackers.",
        recommended: Some("blacklist thunderbolt in modprobe.d"),
    },
    // ── CPU mitigation sub-indicators (Phase 2b) ─────────────────────────
    // These complement the umbrella `Mitigations` indicator (which checks for
    // the global `mitigations=off` flag) by checking each per-CVE weakening
    // override individually. An operator who disables a specific mitigation
    // without using the umbrella flag will be caught here.
    //
    // All entries: KernelCmdline, /proc/cmdline, CmdlineAbsent.
    IndicatorDescriptor {
        id: IndicatorId::SpectreV2Off,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("spectre_v2=off"),
        impact: AssuranceImpact::High,
        label: "Spectre v2",
        rationale: "Explicitly disabling Spectre v2 mitigation exposes the system to \
                    branch-predictor injection attacks between processes and the kernel.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::SpectreV2UserOff,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("spectre_v2_user=off"),
        impact: AssuranceImpact::Medium,
        label: "Spectre v2 User",
        rationale: "Disabling user-space Spectre v2 mitigation prevents processes \
                    from opting in to IBPB/STIBP protection via prctl, increasing \
                    cross-process speculation exposure.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::MdsOff,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("mds=off"),
        impact: AssuranceImpact::High,
        label: "MDS Mitigation",
        rationale: "Disabling MDS mitigation exposes the system to RIDL/Fallout/\
                    ZombieLoad attacks (CVE-2018-12126 et al.) that leak kernel and \
                    hypervisor memory across fill-buffer boundaries.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::TsxAsyncAbortOff,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("tsx_async_abort=off"),
        impact: AssuranceImpact::Medium,
        label: "TSX Async Abort",
        rationale: "Disabling TAA mitigation exposes Intel systems with TSX to \
                    CVE-2019-11135, which leaks data via asynchronous TSX aborts.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::L1tfOff,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("l1tf=off"),
        impact: AssuranceImpact::High,
        label: "L1TF Mitigation",
        rationale: "Disabling L1TF mitigation exposes Intel processors to L1 Terminal \
                    Fault (CVE-2018-3615/3620/3646), which leaks L1 cache data across \
                    VM and process boundaries.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::RetbleedOff,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("retbleed=off"),
        impact: AssuranceImpact::High,
        label: "RETBLEED",
        rationale: "Disabling RETBLEED mitigation exposes the kernel to \
                    CVE-2022-29900/29901, allowing return-address speculation attacks \
                    that bypass retpoline on affected CPUs.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::SrbdsOff,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("srbds=off"),
        impact: AssuranceImpact::Medium,
        label: "SRBDS",
        rationale: "Disabling SRBDS mitigation exposes Intel processors to \
                    CVE-2020-0543, which leaks RNG output from special registers \
                    via sampling attacks.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    IndicatorDescriptor {
        id: IndicatorId::NoSmtOff,
        class: IndicatorClass::KernelCmdline,
        live_path: "/proc/cmdline",
        sysctl_key: None,
        desired: DesiredValue::CmdlineAbsent("nosmt=off"),
        impact: AssuranceImpact::Medium,
        label: "SMT Re-enable",
        rationale: "Re-enabling SMT when the kernel was booted with nosmt weakens \
                    MDS, L1TF, and cross-HT speculation attack mitigations that \
                    depend on SMT being disabled.",
        nist_controls: "NIST 800-53 SI-16, SC-39; NSA RTB: CPU vulnerability mitigations",
        cce: None,
        description: "",
        recommended: None,
    },
    // ── Kernel core dump (Phase 2b) ──────────────────────────────────────
    IndicatorDescriptor {
        id: IndicatorId::CorePattern,
        class: IndicatorClass::Sysctl,
        live_path: "/proc/sys/kernel/core_pattern",
        sysctl_key: Some("kernel.core_pattern"),
        // Hardened check is string-based: value must begin with `|`.
        // This uses Custom because neither Exact(u32) nor CmdlinePresent/Absent
        // can express "string must begin with `|`". The CorePattern validator
        // in reader.rs implements TPI (two-path independence) for this check.
        desired: DesiredValue::Custom,
        impact: AssuranceImpact::High,
        label: "Core Pattern",
        rationale: "A core_pattern beginning with `|` routes dumps to a registered \
                    handler (e.g., systemd-coredump), enabling audit, compression, \
                    and access control. A raw path writes process memory directly to \
                    the filesystem with no handler accountability.",
        nist_controls: "NIST 800-53 SC-7(10), SC-28, CM-6, AU-9; NSA RTB: information \
                        disclosure prevention; CMMC SC.L2-3.13.10",
        cce: Some("CCE-86714-3"),
        description: "",
        recommended: None,
    },
];

// ===========================================================================
// Catalog lookup
// ===========================================================================

/// Look up an indicator descriptor by its typed identifier.
///
/// Searches the static `INDICATORS` array for the entry whose `id` matches
/// the supplied `IndicatorId`. Returns `None` only if the id is not present
/// in the catalog — which would indicate a programming error, since the
/// catalog is intended to cover every `IndicatorId` variant.
///
/// Callers that need both the description and recommendation for display
/// purposes should call `lookup()` once and access both fields from the
/// returned descriptor, rather than calling separate helper functions.
///
/// NIST SP 800-53 SA-5: system documentation — centralised lookup ensures
/// all display consumers access the same authoritative descriptor text.
/// NIST SP 800-53 CM-6: configuration settings — remediation guidance is
/// accessible to any reporting consumer via this function.
#[must_use = "catalog lookup result should be used for display or assessment"]
pub fn lookup(id: IndicatorId) -> Option<&'static IndicatorDescriptor> {
    INDICATORS.iter().find(|d| d.id == id)
}
