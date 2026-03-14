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
//! NIST 800-53 CA-7: Continuous Monitoring — the snapshot is the atomic unit
//! of posture assessment, anchored to a specific boot instance via `boot_id`.
//! NIST 800-53 AU-3: Audit Record Content — `SignalReport` carries typed,
//! structured findings rather than free-form strings.
//! NIST 800-53 CM-6: Configuration Settings — contradiction detection compares
//! live vs. configured values from the sysctl.d merge tree.

use std::time::SystemTime;

use crate::posture::catalog::{SIGNALS, SignalDescriptor};
use crate::posture::configured::SysctlConfig;
use crate::posture::contradiction::{self, ContradictionKind};
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
/// NIST 800-53 AU-3: structured finding record.
/// NIST 800-53 CM-6: live vs. configured comparison.
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
/// NIST 800-53 CA-7: Continuous Monitoring — atomic posture assessment unit.
/// NIST 800-53 AU-3: temporal anchor via `collected_at` and `boot_id`.
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
    /// NIST 800-53 CA-7: produces the posture assessment record.
    /// NIST 800-53 CM-6: contradiction detection via sysctl.d merge.
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

        let reports: Vec<SignalReport> = SIGNALS
            .iter()
            .map(|desc| collect_one(desc, cmdline.as_ref(), &sysctl_config))
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
    #[must_use]
    pub fn readable_count(&self) -> usize {
        self.reports.iter().filter(|r| r.live_value.is_some()).count()
    }

    /// Number of signals whose live value meets the desired hardened value.
    #[must_use]
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
) -> SignalReport {
    log::debug!(
        "posture: reading signal {:?} from {}",
        desc.id,
        desc.live_path
    );

    let (live_value, meets_desired) = read_live(desc, cmdline);

    let configured_value = read_configured(desc, sysctl_config);

    let configured_meets: Option<bool> =
        configured_value.as_ref().and_then(|cv| {
            contradiction::evaluate_configured_meets(&cv.raw, &desc.desired)
        });

    let contradiction =
        contradiction::classify(meets_desired, configured_meets);

    log::debug!(
        "posture: {:?} live={:?} meets={:?} configured={:?} contradiction={:?}",
        desc.id,
        live_value,
        meets_desired,
        configured_value.as_ref().map(|c| &c.raw),
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
    }
}

/// Read a sysctl integer or boolean signal.
fn read_live_sysctl_signal(
    desc: &'static SignalDescriptor,
) -> (Option<LiveValue>, Option<bool>) {
    match desc.id {
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
/// NIST 800-53 SC-28: minimise retention of boot parameter content.
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
/// NIST 800-53 SI-7: provenance-verified via SECURITYFS_MAGIC.
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

// ===========================================================================
// read_configured — configured value lookup
// ===========================================================================

/// Look up the configured value for a signal from the sysctl.d merge tree.
///
/// Returns `None` for cmdline signals (Phase 1 defers bootloader reading)
/// and for signals with no sysctl key.
fn read_configured(
    desc: &'static SignalDescriptor,
    sysctl_config: &SysctlConfig,
) -> Option<ConfiguredValue> {
    // Cmdline and SecurityFs configured values are deferred to Phase 2.
    // SecurityFs signals (e.g., Lockdown) are controlled by kernel LSM state,
    // not by sysctl.d or bootloader cmdline key=value pairs.
    if matches!(
        desc.class,
        SignalClass::KernelCmdline | SignalClass::SecurityFs
    ) {
        return None;
    }

    let key = desc.sysctl_key?;
    sysctl_config.get(key)
}
