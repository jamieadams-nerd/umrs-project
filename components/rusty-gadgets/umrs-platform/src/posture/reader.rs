// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Live-value readers for kernel security posture signals.
//!
//! Every sysctl signal routes through the existing `SecureReader` engine
//! defined in `kattrs::traits`. Provenance verification (fd-anchored `fstatfs`
//! against `PROC_SUPER_MAGIC`) is non-bypassable: it is encoded in the
//! `StaticSource::read()` default method.
//!
//! ## Pattern: Hand-Written Reference + Declarative Macro
//!
//! `KptrRestrict` is written out fully by hand as an auditor reference.
//! All other sysctl-integer signals are stamped out by `define_sysctl_signal!`,
//! which expands to identical boilerplate. Any auditor can verify the macro
//! expansion produces the same structure as the hand-written reference.
//!
//! ## Reuse Strategy
//!
//! Three signals already have complete `KernelFileSource + StaticSource`
//! implementations in `kattrs`:
//!
//! | Signal | Existing type | How reused |
//! |---|---|---|
//! | `FipsEnabled` | `ProcFips` | Called directly via `ProcFips::read()` |
//! | `ModulesDisabled` | `ModuleLoadLatch` | Called directly via `ModuleLoadLatch::read()` |
//! | `Lockdown` | `KernelLockdown` | Called directly via `KernelLockdown::read()` |
//!
//! No duplication: posture uses the existing types; it does not redefine them.
//!
//! ## Compliance
//!
//! NIST 800-53 SI-7: Software and Information Integrity — all live reads use
//! provenance-verified paths; no raw `File::open` on `/proc/` or `/sys/`.
//! NSA RTB RAIN: Non-Bypassable — `StaticSource::read()` routes through
//! `SecureReader::execute_read`, which cannot be skipped.
//! NSA RTB: Compile-Time Path Binding — `PATH` and `EXPECTED_MAGIC` are
//! associated `const` items; the compiler verifies them.

use nix::sys::statfs::{FsType, PROC_SUPER_MAGIC};
use std::io;

use crate::kattrs::traits::{KernelFileSource, StaticSource};

// ===========================================================================
// KptrRestrict — hand-written auditor reference
// ===========================================================================

/// Kernel pointer restriction level (`/proc/sys/kernel/kptr_restrict`).
///
/// Controls whether kernel pointers are exposed in procfs interfaces like
/// `/proc/kallsyms` and `/proc/modules`.
///
/// | Value | Behaviour |
/// |---|---|
/// | 0 | Pointers visible to all users |
/// | 1 | Pointers hidden from non-CAP_SYSLOG users |
/// | 2 | Pointers hidden from all users (including root) |
///
/// Hardened baseline: `2`. This is the fully auditor-verified, hand-written
/// implementation; all other sysctl-integer signals follow the same structure
/// and are stamped out by `define_sysctl_signal!`.
///
/// NIST 800-53 SI-7: reading from provenance-verified procfs.
/// NIST 800-53 SC-39: information hiding prevents KASLR bypass.
/// NSA RTB RAIN: Non-bypassable via `StaticSource::read()`.
pub struct KptrRestrict;

impl KernelFileSource for KptrRestrict {
    type Output = u32;

    const KOBJECT: &'static str = "proc/sys/kernel";
    const ATTRIBUTE_NAME: &'static str = "kptr_restrict";
    const DESCRIPTION: &'static str = "0 -- kernel pointers exposed\n\
         1 -- pointers hidden from unprivileged users (CAP_SYSLOG required)\n\
         2 -- pointers hidden from all users including root";
    const KERNEL_NOTE: &'static str = "Changed from 1→2 at boot by many hardening guides. \
         Requires PROC_SUPER_MAGIC verification.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let result = parse_sysctl_u32(data);

        #[cfg(debug_assertions)]
        log::debug!(
            "posture: KptrRestrict parse completed in {} µs, result={:?}",
            start.elapsed().as_micros(),
            result
        );

        result
    }
}

impl StaticSource for KptrRestrict {
    const PATH: &'static str = "/proc/sys/kernel/kptr_restrict";
    const EXPECTED_MAGIC: FsType = PROC_SUPER_MAGIC;
}

// ===========================================================================
// parse_sysctl_u32 — shared parser for all sysctl integer signals
// ===========================================================================

/// Parse an unsigned sysctl integer value from kernel-format bytes (`"2\n"` → `2`).
///
/// Sysctl nodes uniformly emit ASCII decimal followed by a newline.
/// This function handles the trimming and parsing in one place so the
/// macro-generated types all reference it.
///
/// For sysctl nodes that can legitimately emit negative values (e.g.,
/// `kernel.perf_event_paranoid = -1`), use `parse_sysctl_i32` instead.
///
/// NIST 800-53 SI-10: Input Validation — rejects non-numeric content
/// rather than silently defaulting.
pub fn parse_sysctl_u32(data: &[u8]) -> io::Result<u32> {
    let s = std::str::from_utf8(data).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "sysctl: non-UTF8 data")
    })?;
    s.trim().parse::<u32>().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "sysctl: non-integer value")
    })
}

/// Parse a signed sysctl integer value from kernel-format bytes (`"-1\n"` → `-1`).
///
/// Used for sysctl nodes that legitimately emit negative values, such as
/// `kernel.perf_event_paranoid` where `-1` means "unrestricted for all users".
/// Using `parse_sysctl_u32` on such nodes would return `Err` for `-1`,
/// silently degrading the signal to `live_value: None` and giving a
/// false-assurance picture (Finding 1).
///
/// NIST 800-53 CA-7: must not discard valid kernel states; `-1` is a
/// legitimate (unhardened) value that must be represented, not erased.
/// NIST 800-53 SI-10: Input Validation — rejects non-numeric content.
pub fn parse_sysctl_i32(data: &[u8]) -> io::Result<i32> {
    let s = std::str::from_utf8(data).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "sysctl: non-UTF8 data")
    })?;
    s.trim().parse::<i32>().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "sysctl: non-integer value")
    })
}

// ===========================================================================
// define_sysctl_signal! — declarative macro for boilerplate reduction
// ===========================================================================

/// Stamp out a `KernelFileSource + StaticSource` implementation for a sysctl
/// integer node, routing through `parse_sysctl_u32` and `PROC_SUPER_MAGIC`.
///
/// The expansion is structurally identical to the hand-written `KptrRestrict`
/// above. An auditor can verify this by inspecting the `KptrRestrict`
/// implementation and confirming the macro body matches it.
///
/// # Usage
///
/// ```ignore
/// define_sysctl_signal!(
///     TypeName,
///     "/proc/sys/path/to/attr",
///     "kobject.name",
///     "attr_name",
///     "Description text.",
///     "Optional kernel note."
/// );
/// ```
macro_rules! define_sysctl_signal {
    (
        $type_name:ident,
        $path:literal,
        $kobject:literal,
        $attr_name:literal,
        $description:literal,
        $kernel_note:literal
    ) => {
        /// Sysctl integer signal — stamped out by `define_sysctl_signal!`.
        /// See `KptrRestrict` for the hand-written auditor reference.
        ///
        /// NIST 800-53 SI-7 / NSA RTB RAIN: provenance-verified via `StaticSource::read()`.
        pub struct $type_name;

        impl $crate::kattrs::traits::KernelFileSource for $type_name {
            type Output = u32;
            const KOBJECT: &'static str = $kobject;
            const ATTRIBUTE_NAME: &'static str = $attr_name;
            const DESCRIPTION: &'static str = $description;
            const KERNEL_NOTE: &'static str = $kernel_note;

            fn parse(data: &[u8]) -> ::std::io::Result<Self::Output> {
                #[cfg(debug_assertions)]
                let start = ::std::time::Instant::now();

                let result = $crate::posture::reader::parse_sysctl_u32(data);

                #[cfg(debug_assertions)]
                ::log::debug!(
                    "posture: {} parse completed in {} µs, result={:?}",
                    stringify!($type_name),
                    start.elapsed().as_micros(),
                    result
                );

                result
            }
        }

        impl $crate::kattrs::traits::StaticSource for $type_name {
            const PATH: &'static str = $path;
            const EXPECTED_MAGIC: ::nix::sys::statfs::FsType =
                ::nix::sys::statfs::PROC_SUPER_MAGIC;
        }
    };
}

// Sysctl integer signals — ordered by catalog grouping.
// Each expansion is identical in structure to the hand-written KptrRestrict.

define_sysctl_signal!(
    RandomizeVaSpace,
    "/proc/sys/kernel/randomize_va_space",
    "proc/sys/kernel",
    "randomize_va_space",
    "0 -- ASLR disabled\n1 -- partial ASLR\n2 -- full ASLR (stack, mmap, heap)",
    "RHEL 10 default: 2. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    UnprivBpfDisabled,
    "/proc/sys/kernel/unprivileged_bpf_disabled",
    "proc/sys/kernel",
    "unprivileged_bpf_disabled",
    "0 -- unprivileged BPF allowed\n1 -- restricted to CAP_BPF/CAP_SYS_ADMIN",
    "Requires kernel 5.12+. Verified via PROC_SUPER_MAGIC."
);

// PerfEventParanoid uses a hand-written implementation (not the macro) because
// the kernel legitimately emits -1 for this node ("unrestricted for all users").
// parse_sysctl_u32 would return Err on -1, silently degrading the signal to
// live_value: None and producing a false-assurance picture. parse_sysctl_i32 is
// used instead, and the Output type is i32. See Finding 1 in the security review.
//
// NIST 800-53 CA-7: a system with perf_event_paranoid=-1 is unhardened and must
// produce meets_desired=Some(false), not a missing signal.

/// `kernel.perf_event_paranoid` — signed sysctl reader.
///
/// This node can legitimately emit `-1` (unrestricted access for all users),
/// which is a valid but unhardened state. Unlike unsigned sysctl readers,
/// this implementation uses `parse_sysctl_i32` so that `-1` is represented as
/// `LiveValue::SignedInteger(-1)` with `meets_desired: Some(false)` rather
/// than being silently discarded as a parse error.
///
/// NIST 800-53 CA-7: accurate monitoring requires representing all
/// kernel-valid states, including negative ones.
/// NIST 800-53 SI-7: provenance-verified via PROC_SUPER_MAGIC.
/// NSA RTB RAIN: Non-bypassable via `StaticSource::read()`.
pub struct PerfEventParanoid;

impl KernelFileSource for PerfEventParanoid {
    type Output = i32;

    const KOBJECT: &'static str = "proc/sys/kernel";
    const ATTRIBUTE_NAME: &'static str = "perf_event_paranoid";
    const DESCRIPTION: &'static str = "-1 -- no restriction (unrestricted for all users)\n\
         0 -- restrict to CAP_PERFMON\n\
         1 -- restrict user counters\n\
         2 -- restrict all to CAP_PERFMON\n\
         3 -- disallow all (some distros)";
    const KERNEL_NOTE: &'static str = "Hardened baseline: >=2. Kernel can emit -1 (unhardened). \
         Verified via PROC_SUPER_MAGIC.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let result = parse_sysctl_i32(data);

        #[cfg(debug_assertions)]
        log::debug!(
            "posture: PerfEventParanoid parse completed in {} µs, result={:?}",
            start.elapsed().as_micros(),
            result
        );

        result
    }
}

impl StaticSource for PerfEventParanoid {
    const PATH: &'static str = "/proc/sys/kernel/perf_event_paranoid";
    const EXPECTED_MAGIC: nix::sys::statfs::FsType = PROC_SUPER_MAGIC;
}

define_sysctl_signal!(
    YamaPtraceScope,
    "/proc/sys/kernel/yama/ptrace_scope",
    "proc/sys/kernel/yama",
    "ptrace_scope",
    "0 -- classic ptrace\n1 -- restricted (parent/children only)\n\
     2 -- admin only\n3 -- no ptrace",
    "Requires CONFIG_SECURITY_YAMA. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    DmesgRestrict,
    "/proc/sys/kernel/dmesg_restrict",
    "proc/sys/kernel",
    "dmesg_restrict",
    "0 -- dmesg readable by all\n1 -- restricted to CAP_SYSLOG",
    "RHEL 10 default: 1. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    KexecLoadDisabled,
    "/proc/sys/kernel/kexec_load_disabled",
    "proc/sys/kernel",
    "kexec_load_disabled",
    "0 -- kexec_load() permitted\n1 -- kexec_load() permanently disabled",
    "One-way latch; cannot be re-enabled without reboot. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    SysrqRaw,
    "/proc/sys/kernel/sysrq",
    "proc/sys/kernel",
    "sysrq",
    "Bitmask: 0=disabled, 1=all, 176=sync+remount+reboot",
    "Bitmask semantics are site-policy-dependent; see DesiredValue::Custom. \
     Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    UnprivUsernsClone,
    "/proc/sys/kernel/unprivileged_userns_clone",
    "proc/sys/kernel",
    "unprivileged_userns_clone",
    "0 -- unprivileged user namespace creation blocked\n1 -- allowed",
    "RHEL 10 default: 0. Node may be absent on kernels without this patch. \
     Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    ProtectedSymlinks,
    "/proc/sys/fs/protected_symlinks",
    "proc/sys/fs",
    "protected_symlinks",
    "0 -- no protection\n1 -- symlink following restricted in sticky dirs",
    "RHEL 10 default: 1. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    ProtectedHardlinks,
    "/proc/sys/fs/protected_hardlinks",
    "proc/sys/fs",
    "protected_hardlinks",
    "0 -- no protection\n1 -- hardlinks to unowned files blocked",
    "RHEL 10 default: 1. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    ProtectedFifos,
    "/proc/sys/fs/protected_fifos",
    "proc/sys/fs",
    "protected_fifos",
    "0 -- no protection\n1 -- CAP_DAC_READ_SEARCH required\n\
     2 -- blocked for all in sticky world-writable dirs",
    "RHEL 10 default: 1 or 2 depending on policy. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    ProtectedRegular,
    "/proc/sys/fs/protected_regular",
    "proc/sys/fs",
    "protected_regular",
    "0 -- no protection\n1 -- CAP_DAC_READ_SEARCH required\n\
     2 -- blocked for all in sticky world-writable dirs",
    "RHEL 10 default: 1 or 2 depending on policy. Verified via PROC_SUPER_MAGIC."
);

define_sysctl_signal!(
    SuidDumpable,
    "/proc/sys/fs/suid_dumpable",
    "proc/sys/fs",
    "suid_dumpable",
    "0 -- no core dumps for SUID/SGID processes\n\
     1 -- core dumps enabled (file owned by process uid)\n\
     2 -- core dumps enabled (root-readable only)",
    "Hardened baseline: 0. Verified via PROC_SUPER_MAGIC."
);

// ===========================================================================
// CmdlineReader — /proc/cmdline single-read cache for snapshot collection
// ===========================================================================

/// Reads `/proc/cmdline` once and provides token lookup for all cmdline signals.
///
/// During `PostureSnapshot::collect()`, all cmdline signals share a single
/// read of `/proc/cmdline` so the file is opened only once per snapshot.
///
/// The reader validates the `/proc/` prefix and verifies `PROC_SUPER_MAGIC`
/// before consuming any bytes — identical provenance guarantees to the sysctl
/// readers.
///
/// NIST 800-53 SI-7: provenance-verified read.
/// NSA RTB RAIN: Non-bypassable path through `SecureReader`.
pub struct CmdlineReader {
    content: String,
}

impl CmdlineReader {
    /// Read `/proc/cmdline` through the provenance-verified `ProcfsText` path.
    ///
    /// NIST 800-53 SI-7: provenance-verified read of `/proc/cmdline`.
    pub fn read() -> io::Result<Self> {
        use crate::kattrs::procfs::ProcfsText;
        use crate::kattrs::traits::SecureReader;
        use std::path::PathBuf;

        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let node = ProcfsText::new(PathBuf::from("/proc/cmdline"))?;
        let content =
            SecureReader::<ProcfsText>::new().read_generic_text(&node)?;

        #[cfg(debug_assertions)]
        log::debug!(
            "posture: CmdlineReader read /proc/cmdline in {} µs",
            start.elapsed().as_micros()
        );

        Ok(Self {
            content,
        })
    }

    /// Return the full cmdline string (trimmed of trailing whitespace/newline).
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.content.trim_end()
    }

    /// Test whether the given token is present as a whitespace-delimited word.
    #[must_use]
    pub fn contains_token(&self, token: &str) -> bool {
        self.as_str().split_whitespace().any(|t| t == token)
    }
}

// ===========================================================================
// BootIdReader — /proc/sys/kernel/random/boot_id
// ===========================================================================

/// Reads the kernel boot ID from `/proc/sys/kernel/random/boot_id`.
///
/// The boot ID is a UUID generated at boot time; it changes on every reboot.
/// `PostureSnapshot` records it to anchor the snapshot to a specific boot
/// instance, enabling callers to detect whether a snapshot is stale after
/// a reboot.
///
/// Implemented independently of the `detect` module as specified in the plan.
///
/// NIST 800-53 AU-3: event content — boot ID provides temporal anchor for
/// audit records.
pub struct BootIdReader;

impl BootIdReader {
    /// Read the boot ID string from `/proc/sys/kernel/random/boot_id`.
    ///
    /// Returns `Ok(Some(id))` on success, `Ok(None)` if the node is absent,
    /// or `Err` on I/O or provenance failure.
    pub fn read() -> io::Result<Option<String>> {
        use crate::kattrs::procfs::ProcfsText;
        use crate::kattrs::traits::SecureReader;
        use std::path::PathBuf;

        let path = PathBuf::from("/proc/sys/kernel/random/boot_id");
        let node = ProcfsText::new(path)?;

        match SecureReader::<ProcfsText>::new().read_generic_text(&node) {
            Ok(s) => Ok(Some(s.trim_end().to_owned())),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
}

// ===========================================================================
// read_live_sysctl — dispatch helper for snapshot collection
// ===========================================================================

/// Read the live unsigned integer value of a sysctl signal.
///
/// The dispatch uses a match over `SignalId` variants so the compiler can
/// verify exhaustiveness. Each arm calls the corresponding type's
/// provenance-verified read path.
///
/// Returns `Ok(None)` for signals not handled via this function (signed
/// sysctl, cmdline, and special signals). Returns `Ok(Some(v))` on success,
/// `Err` on I/O failure.
///
/// For `PerfEventParanoid`, which can emit signed values, use
/// `read_live_sysctl_signed` instead.
///
/// NIST 800-53 SI-7: all reads provenance-verified through `StaticSource`.
#[must_use = "live sysctl read result must be examined"]
pub fn read_live_sysctl(
    id: crate::posture::signal::SignalId,
) -> io::Result<Option<u32>> {
    use crate::kattrs::traits::StaticSource;
    use crate::posture::signal::SignalId;

    match id {
        SignalId::KptrRestrict => KptrRestrict::read().map(Some),
        SignalId::RandomizeVaSpace => RandomizeVaSpace::read().map(Some),
        SignalId::UnprivBpfDisabled => UnprivBpfDisabled::read().map(Some),
        SignalId::YamaPtraceScope => YamaPtraceScope::read().map(Some),
        SignalId::DmesgRestrict => DmesgRestrict::read().map(Some),
        SignalId::KexecLoadDisabled => KexecLoadDisabled::read().map(Some),
        SignalId::Sysrq => SysrqRaw::read().map(Some),
        SignalId::UnprivUsernsClone => UnprivUsernsClone::read().map(Some),
        SignalId::ProtectedSymlinks => ProtectedSymlinks::read().map(Some),
        SignalId::ProtectedHardlinks => ProtectedHardlinks::read().map(Some),
        SignalId::ProtectedFifos => ProtectedFifos::read().map(Some),
        SignalId::ProtectedRegular => ProtectedRegular::read().map(Some),
        SignalId::SuidDumpable => SuidDumpable::read().map(Some),
        // PerfEventParanoid uses a signed reader — handled by read_live_sysctl_signed.
        // Non-sysctl signals handled elsewhere.
        SignalId::PerfEventParanoid
        | SignalId::ModulesDisabled
        | SignalId::Lockdown
        | SignalId::ModuleSigEnforce
        | SignalId::Mitigations
        | SignalId::Pti
        | SignalId::RandomTrustCpu
        | SignalId::RandomTrustBootloader
        | SignalId::FipsEnabled => Ok(None),
    }
}

/// Read the live signed integer value for sysctl signals that emit negative values.
///
/// Currently handles `PerfEventParanoid`, which can emit `-1` ("unrestricted
/// for all users"). All other signals pass through as `Ok(None)`.
///
/// NIST 800-53 CA-7: represents the complete set of kernel-valid states,
/// including negative values, so no valid unhardened state is silently discarded.
/// NIST 800-53 SI-7: provenance-verified via `StaticSource`.
#[must_use = "live signed sysctl read result must be examined"]
pub fn read_live_sysctl_signed(
    id: crate::posture::signal::SignalId,
) -> io::Result<Option<i32>> {
    use crate::kattrs::traits::StaticSource;
    use crate::posture::signal::SignalId;

    match id {
        SignalId::PerfEventParanoid => PerfEventParanoid::read().map(Some),
        _ => Ok(None),
    }
}

// ===========================================================================
// read_lockdown_live — live value for Lockdown signal
// ===========================================================================

/// Read the live lockdown mode for the `Lockdown` signal.
///
/// Delegates to `KernelLockdown::read()` which uses TPI parsing and
/// verifies `SECURITYFS_MAGIC`. Returns `Ok(None)` if securityfs is not
/// mounted.
///
/// NIST 800-53 SI-7: provenance-verified via SECURITYFS_MAGIC.
pub fn read_lockdown_live()
-> io::Result<Option<crate::kattrs::security::LockdownMode>> {
    use crate::kattrs::security::KernelLockdown;
    use crate::kattrs::traits::StaticSource;

    match KernelLockdown::read() {
        Ok(mode) => Ok(Some(mode)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}
