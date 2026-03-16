// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Point-in-time kernel security posture snapshot.
//!
//! `PostureSnapshot` is the primary user-facing type. It collects every
//! signal in the static catalog, reads live and configured values, evaluates
//! hardening status, and records any contradictions.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use umrs_platform::posture::{PostureSnapshot, SignalId, AssuranceImpact};
//!
//! let snap = PostureSnapshot::collect();
//! println!("{}/{} signals hardened", snap.hardened_count(), snap.readable_count());
//!
//! for report in snap.findings() {
//!     println!("{}: live={:?}", report.descriptor.id, report.live_value);
//! }
//! ```
//!
//! ## Compliance
//!
//! NIST SP 800-53 CA-7: Continuous Monitoring — the snapshot is the atomic unit
//! of posture assessment, anchored to a specific boot instance via `boot_id`.
//! NIST SP 800-53 AU-3: Audit Record Content — `SignalReport` carries typed,
//! structured findings rather than free-form strings.
//! NIST SP 800-53 CM-6: Configuration Settings — contradiction detection compares
//! live vs. configured values from the sysctl.d merge tree.

use std::time::SystemTime;

use crate::posture::catalog::{SIGNALS, SignalDescriptor};
use crate::posture::configured::{SysctlConfig, configured_cmdline};
use crate::posture::contradiction::{self, ContradictionKind};
use crate::posture::fips_cross::FipsCrossCheck;
use crate::posture::modprobe::ModprobeConfig;
use crate::posture::reader::{BootIdReader, CmdlineReader};
use crate::posture::signal::{
    AssuranceImpact, ConfiguredValue, DesiredValue, LiveValue, SignalClass,
    SignalId,
};

// ===========================================================================
// SignalReport
// ===========================================================================

/// The result of reading and evaluating one security posture signal.
///
/// Contains the live (kernel) value, the configured (sysctl.d) value,
/// the hardening assessment, and any contradiction classification.
///
/// NIST SP 800-53 AU-3: structured finding record.
/// NIST SP 800-53 CM-6: live vs. configured comparison.
#[must_use = "signal reports carry security posture findings — do not discard"]
pub struct SignalReport {
    /// Static catalog entry for this signal.
    pub descriptor: &'static SignalDescriptor,
    /// The value currently active in the kernel, or `None` if unreadable.
    pub live_value: Option<LiveValue>,
    /// The configured value from the persistence layer, or `None` if absent.
    pub configured_value: Option<ConfiguredValue>,
    /// Whether the live value meets the desired hardened value.
    /// `None` if the live value was unreadable.
    pub meets_desired: Option<bool>,
    /// Contradiction classification, if live and configured values disagree.
    pub contradiction: Option<ContradictionKind>,
}

// ===========================================================================
// PostureSnapshot
// ===========================================================================

/// Point-in-time snapshot of all kernel security posture signals.
///
/// Constructed via `PostureSnapshot::collect()`, which reads every signal
/// in the static catalog and produces a `SignalReport` for each.
///
/// The snapshot is anchored to a specific boot instance via `boot_id`
/// (read from `/proc/sys/kernel/random/boot_id`). If `boot_id` changes
/// between two snapshots, the comparison is cross-boot and may reflect
/// expected deltas.
///
/// NIST SP 800-53 CA-7: Continuous Monitoring — atomic posture assessment unit.
/// NIST SP 800-53 AU-3: temporal anchor via `collected_at` and `boot_id`.
#[must_use = "posture snapshots contain security findings — do not discard"]
pub struct PostureSnapshot {
    /// All signal reports, one per catalog entry, in catalog order.
    pub reports: Vec<SignalReport>,
    /// Wall-clock time when this snapshot was collected.
    pub collected_at: SystemTime,
    /// Kernel boot ID (`/proc/sys/kernel/random/boot_id`), if readable.
    pub boot_id: Option<String>,
}

impl PostureSnapshot {
    /// Collect all signals from the static catalog and produce a snapshot.
    ///
    /// Reads `/proc/sys/*` nodes via provenance-verified `SecureReader` paths,
    /// `/proc/cmdline` once (shared across all cmdline signals), and the
    /// sysctl.d merge tree for configured values.
    ///
    /// Individual signal read failures are captured in the report's
    /// `live_value: None` field rather than propagated as errors — the
    /// snapshot degrades gracefully when kernel nodes are absent.
    ///
    /// NIST SP 800-53 CA-7: produces the posture assessment record.
    /// NIST SP 800-53 CM-6: contradiction detection via sysctl.d merge.
    #[must_use = "posture snapshot contains security findings — examine before discarding"]
    pub fn collect() -> Self {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let collected_at = SystemTime::now();

        // Read boot_id independently — no dependency on the detect module.
        let boot_id = match BootIdReader::read() {
            Ok(id) => id,
            Err(e) => {
                log::warn!("posture: boot_id read failed: {e}");
                None
            }
        };

        // Read /proc/cmdline once; shared across all cmdline signals.
        let cmdline = match CmdlineReader::read() {
            Ok(r) => Some(r),
            Err(e) => {
                log::warn!("posture: /proc/cmdline read failed: {e}");
                None
            }
        };

        // Load sysctl.d configured values once for the entire snapshot.
        let sysctl_config = SysctlConfig::load();

        // Load modprobe.d configured values once for all modprobe signals.
        let modprobe_config = ModprobeConfig::load();

        // Load the bootloader configured cmdline once for all KernelCmdline
        // signals. Phase 2b: reads BLS entries from /boot/loader/entries/.
        // Returns None on systems without BLS (containers, non-RHEL), which
        // disables configured-cmdline contradiction detection gracefully.
        let configured_boot_cmdline = configured_cmdline();

        let reports: Vec<SignalReport> = SIGNALS
            .iter()
            .map(|desc| {
                collect_one(
                    desc,
                    cmdline.as_ref(),
                    &sysctl_config,
                    &modprobe_config,
                    configured_boot_cmdline.as_deref(),
                )
            })
            .collect();

        let readable =
            reports.iter().filter(|r| r.live_value.is_some()).count();
        let hardened =
            reports.iter().filter(|r| r.meets_desired == Some(true)).count();

        #[cfg(debug_assertions)]
        log::debug!(
            "posture: PostureSnapshot collected {readable}/{} signals in {} µs ({hardened} hardened)",
            reports.len(),
            start.elapsed().as_micros()
        );

        Self {
            reports,
            collected_at,
            boot_id,
        }
    }

    /// Iterator over all signal reports in catalog order.
    #[must_use = "signal report iterator must be consumed to examine posture findings"]
    pub fn iter(&self) -> impl Iterator<Item = &SignalReport> {
        self.reports.iter()
    }

    /// Iterator over signals that do NOT meet their desired hardened value.
    ///
    /// Excludes signals whose live value could not be read (`meets_desired == None`).
    /// Use `iter()` and filter manually to include unreadable signals.
    #[must_use = "findings iterator carries unhardened signals — examine each report"]
    pub fn findings(&self) -> impl Iterator<Item = &SignalReport> {
        self.reports.iter().filter(|r| r.meets_desired == Some(false))
    }

    /// Iterator over signals with a live/configured contradiction.
    #[must_use = "contradictions iterator carries configuration management gaps — examine each report"]
    pub fn contradictions(&self) -> impl Iterator<Item = &SignalReport> {
        self.reports.iter().filter(|r| r.contradiction.is_some())
    }

    /// Iterator over signals meeting a minimum assurance impact threshold.
    ///
    /// Returns reports whose `descriptor.impact >= min`.
    #[must_use = "impact-filtered iterator carries security findings — examine each report"]
    pub fn by_impact(
        &self,
        min: AssuranceImpact,
    ) -> impl Iterator<Item = &SignalReport> {
        self.reports.iter().filter(move |r| r.descriptor.impact >= min)
    }

    /// Number of signals whose live value was successfully read.
    #[must_use = "readable_count feeds operator summary and audit metrics — discarding hides signal availability"]
    pub fn readable_count(&self) -> usize {
        self.reports.iter().filter(|r| r.live_value.is_some()).count()
    }

    /// Number of signals whose live value meets the desired hardened value.
    #[must_use = "hardened_count feeds operator summary and audit metrics — discarding hides hardening posture"]
    pub fn hardened_count(&self) -> usize {
        self.reports.iter().filter(|r| r.meets_desired == Some(true)).count()
    }

    /// Look up the report for a specific signal by ID.
    ///
    /// Returns `None` if the signal is not in the catalog (should not happen
    /// in practice — the catalog is exhaustive for Phase 1 signals).
    #[must_use = "signal lookup result must be examined"]
    pub fn get(&self, id: SignalId) -> Option<&SignalReport> {
        self.reports.iter().find(|r| r.descriptor.id == id)
    }
}

// ===========================================================================
// collect_one — single-signal collection logic
// ===========================================================================

/// Collect the report for one signal.
///
/// Reads the live value, looks up the configured value, evaluates whether
/// the live value meets the desired baseline, and classifies any contradiction.
fn collect_one(
    desc: &'static SignalDescriptor,
    cmdline: Option<&CmdlineReader>,
    sysctl_config: &SysctlConfig,
    modprobe_config: &ModprobeConfig,
    configured_boot_cmdline: Option<&str>,
) -> SignalReport {
    log::debug!(
        "posture: reading signal {:?} from {}",
        desc.id,
        desc.live_path
    );

    let (live_value, meets_desired) = read_live(desc, cmdline);

    let configured_value = read_configured(
        desc,
        sysctl_config,
        cmdline,
        live_value.as_ref(),
        modprobe_config,
        configured_boot_cmdline,
    );

    // For KernelCmdline signals, configured_meets is evaluated via token-based
    // matching on the BLS options string, not through evaluate_configured_meets().
    // The BLS options string is not an integer and not the "blacklisted" sentinel,
    // so evaluate_configured_meets() would return None for it — which would
    // silently suppress BootDrift and EphemeralHotfix detection for all cmdline
    // signals. Instead, apply DesiredValue::meets_cmdline() directly to the raw
    // BLS options string.
    //
    // NIST SP 800-53 CA-7: BootDrift/EphemeralHotfix must fire when the BLS
    // options line disagrees with /proc/cmdline on a security token.
    // NIST SP 800-53 CM-6: configured persistence layer for cmdline signals is
    // the BLS options line, not a sysctl.d integer value.
    let configured_meets: Option<bool> = if desc.class == SignalClass::KernelCmdline {
        // For KernelCmdline signals, configured_meets is derived from token
        // presence in the BLS options string (configured_boot_cmdline).
        // If configured_boot_cmdline is None (BLS unavailable), configured_meets
        // is None — no contradiction can be detected (graceful degrade).
        configured_boot_cmdline
            .and_then(|opts| desc.desired.meets_cmdline(opts))
    } else {
        configured_value.as_ref().and_then(|cv| {
            contradiction::evaluate_configured_meets(&cv.raw, &desc.desired)
        })
    };

    let contradiction =
        contradiction::classify(meets_desired, configured_meets);

    // Gate the summary log behind debug_assertions to prevent raw configured
    // values from leaking in release builds when debug logging is enabled on
    // DoD/CUI systems during troubleshooting. Configured values for sysctl.d,
    // modprobe.d, and FIPS indicators are suppressed from the production log
    // path; signal IDs, hardening status, and contradiction kind are safe to log.
    // NIST SP 800-53 SI-11; NSA RTB Error Discipline.
    #[cfg(debug_assertions)]
    log::debug!(
        "posture: {:?} live={:?} meets={:?} configured={:?} contradiction={:?}",
        desc.id,
        live_value,
        meets_desired,
        configured_value.as_ref().map(|c| &c.raw),
        contradiction
    );
    // Release-mode debug log: live_value is intentionally included because
    // current Text-valued signals store only compile-time catalog tokens (e.g.,
    // "module.sig_enforce=1", "absent"), not raw kernel output. If a future
    // signal stores kernel-supplied text in LiveValue::Text (e.g., a raw sysfs
    // string), this log line must be gated under #[cfg(debug_assertions)] for
    // that signal to maintain Error Information Discipline.
    // NIST SP 800-53 SI-11; NSA RTB Error Discipline.
    #[cfg(not(debug_assertions))]
    log::debug!(
        "posture: {:?} live={:?} meets={:?} contradiction={:?}",
        desc.id,
        live_value,
        meets_desired,
        contradiction
    );

    SignalReport {
        descriptor: desc,
        live_value,
        configured_value,
        meets_desired,
        contradiction,
    }
}

// ===========================================================================
// read_live — live value dispatch
// ===========================================================================

/// Read the live value of a signal and evaluate whether it meets the desired value.
///
/// Returns `(Some(value), Some(meets))` on success, `(None, None)` if the
/// signal's kernel node is absent or unreadable.
fn read_live(
    desc: &'static SignalDescriptor,
    cmdline: Option<&CmdlineReader>,
) -> (Option<LiveValue>, Option<bool>) {
    match desc.class {
        SignalClass::Sysctl => read_live_sysctl_signal(desc),
        SignalClass::KernelCmdline => read_live_cmdline_signal(desc, cmdline),
        SignalClass::SecurityFs => read_live_security_fs(desc),
        SignalClass::DistroManaged => read_live_distro_managed(desc),
        SignalClass::ModprobeConfig => read_live_modprobe(desc),
    }
}

/// Read a sysctl integer or boolean signal.
fn read_live_sysctl_signal(
    desc: &'static SignalDescriptor,
) -> (Option<LiveValue>, Option<bool>) {
    match desc.id {
        SignalId::CorePattern => {
            // CorePattern is Sysctl-class but returns a String, not a u32.
            // TPI classification is applied in read_live_core_pattern.
            match crate::posture::reader::read_live_core_pattern() {
                Ok(Some((kind, raw))) => {
                    use crate::posture::reader::CorePatternKind;
                    let meets =
                        Some(kind == CorePatternKind::ManagedHandler);
                    log::debug!(
                        "posture: CorePattern: kind={kind:?} meets={meets:?}"
                    );
                    (Some(LiveValue::Text(raw)), meets)
                }
                Ok(None) => {
                    log::debug!("posture: CorePattern: kernel node absent");
                    (None, None)
                }
                Err(e) => {
                    log::debug!("posture: CorePattern read failed: {e}");
                    (None, None)
                }
            }
        }
        SignalId::ModulesDisabled => {
            use crate::kattrs::procfs::ModuleLoadLatch;
            use crate::kattrs::traits::StaticSource;
            match ModuleLoadLatch::read() {
                Ok(v) => {
                    let meets = desc.desired.meets_integer(u32::from(v));
                    (Some(LiveValue::Bool(v)), meets)
                }
                Err(e) => {
                    log::debug!("posture: ModulesDisabled read failed: {e}");
                    (None, None)
                }
            }
        }
        // PerfEventParanoid uses a signed reader (can emit -1 = "unrestricted").
        SignalId::PerfEventParanoid => {
            match crate::posture::reader::read_live_sysctl_signed(
                SignalId::PerfEventParanoid,
            ) {
                Ok(Some(v)) => {
                    let meets = desc.desired.meets_signed_integer(v);
                    (Some(LiveValue::SignedInteger(v)), meets)
                }
                Ok(None) => {
                    // read_live_sysctl_signed always handles PerfEventParanoid.
                    debug_assert!(
                        false,
                        "PerfEventParanoid must be handled by signed reader"
                    );
                    log::warn!(
                        "posture: PerfEventParanoid: signed reader returned None unexpectedly"
                    );
                    (None, None)
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    log::debug!(
                        "posture: PerfEventParanoid: kernel node absent"
                    );
                    (None, None)
                }
                Err(e) => {
                    log::debug!("posture: PerfEventParanoid read failed: {e}");
                    (None, None)
                }
            }
        }
        id => {
            // All remaining Sysctl-class signals return u32.
            match crate::posture::reader::read_live_sysctl(id) {
                Ok(Some(v)) => {
                    let meets = if desc.desired == DesiredValue::Custom {
                        // Sysrq: default hardened check is value == 0.
                        Some(v == 0)
                    } else {
                        desc.desired.meets_integer(v)
                    };
                    (Some(LiveValue::Integer(v)), meets)
                }
                Ok(None) => {
                    // A Sysctl-class signal fell through read_live_sysctl without
                    // a matching arm. This is a catalog/reader mismatch that
                    // should be caught in debug builds.
                    debug_assert!(
                        false,
                        "Sysctl-class signal {id:?} not dispatched by read_live_sysctl — \
                         catalog/reader mismatch"
                    );
                    log::warn!(
                        "posture: Sysctl-class signal {id:?} returned Ok(None) from \
                         read_live_sysctl — catalog/reader mismatch"
                    );
                    (None, None)
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    log::debug!("posture: signal {id:?}: kernel node absent");
                    (None, None)
                }
                Err(e) => {
                    log::debug!("posture: signal {id:?} read failed: {e}");
                    (None, None)
                }
            }
        }
    }
}

/// Read a kernel cmdline signal.
///
/// Stores only the matched token (or `"absent"` for `CmdlineAbsent` tokens)
/// as the `LiveValue`, not the full cmdline string. This avoids repeated heap
/// allocation of the full cmdline per signal and limits exposure of potentially
/// sensitive boot parameters in the snapshot.
///
/// NIST SP 800-53 SC-28: minimise retention of boot parameter content.
fn read_live_cmdline_signal(
    desc: &'static SignalDescriptor,
    cmdline: Option<&CmdlineReader>,
) -> (Option<LiveValue>, Option<bool>) {
    let Some(reader) = cmdline else {
        log::debug!("posture: cmdline reader unavailable for {:?}", desc.id);
        return (None, None);
    };

    let cmdline_str = reader.as_str();
    let meets = desc.desired.meets_cmdline(cmdline_str);

    // Store only the relevant token, not the full cmdline string.
    // For both CmdlinePresent and CmdlineAbsent: store the token text if the token
    // is present in the cmdline, or "absent" if it is not. This records what was
    // observed without retaining the entire cmdline content per signal.
    let token_value = match &desc.desired {
        DesiredValue::CmdlinePresent(token)
        | DesiredValue::CmdlineAbsent(token) => {
            if reader.contains_token(token) {
                token.to_string()
            } else {
                "absent".to_owned()
            }
        }
        // Non-cmdline desired values should not reach here for KernelCmdline signals.
        _ => cmdline_str.to_owned(),
    };

    (Some(LiveValue::Text(token_value)), meets)
}

/// Read a SecurityFs signal (currently only `Lockdown`).
///
/// Routes to `read_lockdown_live()` for the `Lockdown` signal. Any unknown
/// `SecurityFs`-class signal degrades gracefully to `(None, None)`.
///
/// NIST SP 800-53 SI-7: provenance-verified via SECURITYFS_MAGIC.
fn read_live_security_fs(
    desc: &'static SignalDescriptor,
) -> (Option<LiveValue>, Option<bool>) {
    match desc.id {
        SignalId::Lockdown => {
            match crate::posture::reader::read_lockdown_live() {
                Ok(Some(mode)) => {
                    use crate::kattrs::security::LockdownMode;
                    // Desired: lockdown=integrity means at least Integrity level.
                    let meets = Some(mode >= LockdownMode::Integrity);
                    (Some(LiveValue::Text(mode.to_string())), meets)
                }
                Ok(None) => {
                    log::debug!("posture: Lockdown: securityfs node absent");
                    (None, None)
                }
                Err(e) => {
                    log::debug!("posture: Lockdown read failed: {e}");
                    (None, None)
                }
            }
        }
        id => {
            log::debug!("posture: unknown SecurityFs signal {id:?}");
            (None, None)
        }
    }
}

/// Read a distro-managed signal (currently only `FipsEnabled`).
fn read_live_distro_managed(
    desc: &'static SignalDescriptor,
) -> (Option<LiveValue>, Option<bool>) {
    match desc.id {
        SignalId::FipsEnabled => {
            use crate::kattrs::procfs::ProcFips;
            use crate::kattrs::traits::StaticSource;
            match ProcFips::read() {
                Ok(v) => {
                    let meets = desc.desired.meets_integer(u32::from(v));
                    (Some(LiveValue::Bool(v)), meets)
                }
                Err(e) => {
                    log::debug!("posture: FipsEnabled read failed: {e}");
                    (None, None)
                }
            }
        }
        id => {
            log::debug!("posture: unknown DistroManaged signal {id:?}");
            (None, None)
        }
    }
}

/// Read a modprobe.d-configured signal live value from sysfs.
///
/// For blacklist signals: reads module directory presence from sysfs as the
/// live check. Module absent = `Bool(true)` (blacklist effective). Module
/// present = `Bool(false)` (module loaded, blacklist not effective).
///
/// For parameter signals (`NfConntrackAcct`): reads
/// `/sys/module/<mod>/parameters/<param>` via `SysfsText` + `SYSFS_MAGIC`,
/// but only if the module is loaded (Trust Gate). Returns `(None, None)` if
/// module is not loaded.
///
/// NIST SP 800-53 SI-7: provenance-verified sysfs reads via SYSFS_MAGIC.
/// NIST SP 800-53 CM-6: Trust Gate — module must be loaded to read parameters.
fn read_live_modprobe(
    desc: &'static SignalDescriptor,
) -> (Option<LiveValue>, Option<bool>) {
    use crate::posture::modprobe::{is_module_loaded, read_module_param};

    match desc.id {
        SignalId::NfConntrackAcct => {
            // Trust Gate: only read if nf_conntrack is loaded.
            if !is_module_loaded("nf_conntrack") {
                log::debug!(
                    "posture: modprobe cross-check: nf_conntrack not loaded \
                     — live value unavailable (Trust Gate)"
                );
                return (None, None);
            }
            match read_module_param("nf_conntrack", "acct") {
                Ok(Some(raw)) => {
                    let parsed = raw.trim().parse::<u32>().ok();
                    if let Some(v) = parsed {
                        let meets = desc.desired.meets_integer(v);
                        log::debug!(
                            "posture: modprobe cross-check: \
                             nf_conntrack acct: live={v} meets={meets:?}"
                        );
                        (Some(LiveValue::Integer(v)), meets)
                    } else {
                        log::debug!(
                            "posture: modprobe cross-check: \
                             nf_conntrack acct: non-integer live value '{raw}'"
                        );
                        (None, None)
                    }
                }
                Ok(None) => {
                    log::debug!(
                        "posture: modprobe cross-check: \
                         nf_conntrack/acct: parameter node absent"
                    );
                    (None, None)
                }
                Err(e) => {
                    log::debug!(
                        "posture: modprobe cross-check: \
                         nf_conntrack/acct: sysfs read failed: {e}"
                    );
                    (None, None)
                }
            }
        }
        // Blacklist signals: module-directory presence is the live check.
        // Module absent → blacklist effective (Bool(true) = hardened).
        // Module present → blacklist not effective (Bool(false) = unhardened).
        id @ (SignalId::BluetoothBlacklisted
        | SignalId::UsbStorageBlacklisted
        | SignalId::FirewireCoreBlacklisted
        | SignalId::ThunderboltBlacklisted) => {
            let module_name = module_name_for_blacklist_signal(id);
            let loaded = is_module_loaded(module_name);
            // Blacklist hardened = module NOT loaded.
            let hardened = !loaded;
            log::debug!(
                "posture: modprobe cross-check: {} blacklisted={} \
                 loaded={} → {}",
                module_name,
                !loaded,
                loaded,
                if hardened {
                    "PASS (module absent)"
                } else {
                    "FAIL (module present)"
                }
            );
            // desired=Exact(1) means "blacklist effective" (Bool(true) = hardened).
            (Some(LiveValue::Bool(hardened)), Some(hardened))
        }
        id => {
            log::debug!("posture: unknown ModprobeConfig signal {id:?}");
            (None, None)
        }
    }
}

/// Map a blacklist `SignalId` to the corresponding kernel module name.
///
/// Used to derive the `/sys/module/<name>/` path for module-load detection.
const fn module_name_for_blacklist_signal(id: SignalId) -> &'static str {
    match id {
        SignalId::BluetoothBlacklisted => "bluetooth",
        SignalId::UsbStorageBlacklisted => "usb_storage",
        SignalId::FirewireCoreBlacklisted => "firewire_core",
        SignalId::ThunderboltBlacklisted => "thunderbolt",
        // All other IDs are not blacklist signals; this function is only
        // called from the blacklist match arm above.
        _ => "unknown",
    }
}

// ===========================================================================
// read_configured — configured value lookup
// ===========================================================================

/// Look up the configured value for a signal from the appropriate source.
///
/// - `Sysctl` + `DistroManaged` (sysctl key present): sysctl.d merge tree.
/// - `DistroManaged` `FipsEnabled`: FIPS cross-check via `FipsCrossCheck`.
/// - `ModprobeConfig`: modprobe.d merge tree.
/// - `KernelCmdline`: BLS bootloader entry `options` line (Phase 2b).
/// - `SecurityFs`: no sysctl.d / cmdline configured value (not applicable).
///
/// NIST SP 800-53 CM-6: configured-value lookup from the full set of
/// persistence sources.
fn read_configured(
    desc: &'static SignalDescriptor,
    sysctl_config: &SysctlConfig,
    cmdline: Option<&CmdlineReader>,
    live_value: Option<&LiveValue>,
    modprobe_config: &ModprobeConfig,
    configured_boot_cmdline: Option<&str>,
) -> Option<ConfiguredValue> {
    // SecurityFs LSM signals have no sysctl.d / cmdline configured value.
    if desc.class == SignalClass::SecurityFs {
        return None;
    }

    match desc.class {
        SignalClass::ModprobeConfig => {
            read_configured_modprobe(desc, modprobe_config)
        }
        SignalClass::DistroManaged if desc.id == SignalId::FipsEnabled => {
            read_configured_fips(cmdline, live_value)
        }
        SignalClass::Sysctl | SignalClass::DistroManaged => {
            let key = desc.sysctl_key?;
            sysctl_config.get(key)
        }
        SignalClass::KernelCmdline => {
            read_configured_boot_cmdline(desc, configured_boot_cmdline)
        }
        SignalClass::SecurityFs => None,
    }
}

/// Look up configured value for a `KernelCmdline`-class signal from the
/// bootloader-configured cmdline.
///
/// The `configured_boot_cmdline` argument is the `options` line from the most
/// likely active BLS entry, as read by `bootcmdline::read_configured_cmdline()`.
/// If `None` (BLS not available, no entries found), returns `None` — no
/// configured cmdline value is available and no contradiction will be detected.
///
/// The raw BLS options string is stored as-is for operator display and audit
/// output. Contradiction detection for `KernelCmdline` signals does NOT go
/// through `evaluate_configured_meets()` — it uses a dedicated token-based path
/// in `collect_one()` that calls `DesiredValue::meets_cmdline()` directly on
/// the BLS options string. This is the correct path for `CmdlinePresent` and
/// `CmdlineAbsent` desired values.
///
/// The configured value uses `/boot/loader/entries/` as the `source_file`
/// sentinel — the exact entry filename is not recorded here to keep the
/// interface simple.
///
/// NIST SP 800-53 CM-6: bootloader `options` line is the persistence layer for
/// cmdline security tokens.
/// NIST SP 800-53 CA-7: enables `EphemeralHotfix`/`BootDrift` detection for
/// cmdline signals (`ModuleSigEnforce`, `Mitigations`, `Pti`, etc.).
fn read_configured_boot_cmdline(
    _desc: &'static SignalDescriptor,
    configured_boot_cmdline: Option<&str>,
) -> Option<ConfiguredValue> {
    let boot_opts = configured_boot_cmdline?;

    // Store the full BLS options line as the raw configured value for operator
    // display and audit output. This string is not an integer and not the
    // "blacklisted" sentinel, so evaluate_configured_meets() returns None for it.
    //
    // Contradiction detection for KernelCmdline signals is handled via a
    // dedicated token-based path in collect_one(): configured_meets is computed
    // by calling DesiredValue::meets_cmdline(boot_opts) rather than routing
    // through evaluate_configured_meets(). This produces correct BootDrift and
    // EphemeralHotfix results when the BLS options line disagrees with
    // /proc/cmdline on a security token.
    Some(ConfiguredValue {
        raw: boot_opts.to_owned(),
        source_file: "/boot/loader/entries/".to_owned(),
    })
}

/// Look up configured value for a modprobe.d signal.
///
/// For blacklist signals: returns `Some(ConfiguredValue { raw: "blacklisted", ... })`
/// if the module is in the blacklist map.
/// For parameter signals: returns the configured `options` value.
fn read_configured_modprobe(
    desc: &'static SignalDescriptor,
    modprobe_config: &ModprobeConfig,
) -> Option<ConfiguredValue> {
    use crate::posture::modprobe::blacklist_configured_value;

    match desc.id {
        SignalId::NfConntrackAcct => {
            let cv = modprobe_config.get_option("nf_conntrack", "acct");
            if let Some(ref c) = cv {
                // Log the source path only; suppress the raw configured value in
                // release builds to maintain Error Information Discipline on
                // DoD/CUI systems. NIST SP 800-53 SI-11; NSA RTB Error Discipline.
                #[cfg(debug_assertions)]
                {
                    let raw = &c.raw;
                    let src = &c.source_file;
                    log::debug!(
                        "posture: modprobe cross-check: nf_conntrack acct: \
                         configured={raw} source={src}"
                    );
                }
                #[cfg(not(debug_assertions))]
                log::debug!(
                    "posture: modprobe cross-check: nf_conntrack acct: source={}",
                    c.source_file
                );
            }
            cv
        }
        id @ (SignalId::BluetoothBlacklisted
        | SignalId::UsbStorageBlacklisted
        | SignalId::FirewireCoreBlacklisted
        | SignalId::ThunderboltBlacklisted) => {
            let module_name = module_name_for_blacklist_signal(id);
            let cv = blacklist_configured_value(module_name, modprobe_config);
            if let Some(ref c) = cv {
                let src = &c.source_file;
                log::debug!(
                    "posture: modprobe cross-check: {module_name} blacklisted \
                     source={src}"
                );
            } else {
                log::debug!(
                    "posture: modprobe cross-check: {module_name} not found in \
                     modprobe.d blacklist"
                );
            }
            cv
        }
        id => {
            log::debug!(
                "posture: read_configured_modprobe: unknown ModprobeConfig \
                 signal {id:?}"
            );
            None
        }
    }
}

/// Evaluate the FIPS configured-value via the cross-check module.
///
/// Implements the Trust Gate: only invokes FIPS cross-check if the live
/// FIPS value was successfully read. Returns the cross-check's
/// `ConfiguredValue` summary for insertion into `SignalReport`.
///
/// NIST SP 800-53 CM-6: Trust Gate — config reads gated on live availability.
/// NIST SP 800-218 SSDF PW.4: pattern timing in debug builds.
fn read_configured_fips(
    cmdline: Option<&CmdlineReader>,
    live_value: Option<&LiveValue>,
) -> Option<ConfiguredValue> {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    // Trust Gate: only run cross-check if live FIPS value was readable.
    let live_fips_readable = live_value.is_some();

    // Extract cmdline fips=1 token from the shared CmdlineReader.
    let cmdline_has_fips1 = cmdline.map(|r| r.contains_token("fips=1"));

    let cross_check =
        FipsCrossCheck::evaluate(live_fips_readable, cmdline_has_fips1);
    let result = cross_check.as_configured_value();

    #[cfg(debug_assertions)]
    {
        let elapsed = start.elapsed().as_micros();
        let raw_display = result.as_ref().map(|c| &c.raw);
        log::debug!(
            "posture: FIPS cross-check: read_configured completed in \
             {elapsed} µs, result={raw_display:?}"
        );
    }

    result
}
