// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::unreadable_literal)]

//! # umrs-tui — OS Detection Audit Card
//!
//! Runs the `umrs-platform` OS detection pipeline and displays the result
//! as an interactive ratatui audit card. Three tabs present the data:
//!
//! - **Tab 0 — OS Information**: `os-release` fields, substrate identity,
//!   boot ID.
//! - **Tab 1 — Trust / Evidence**: label trust classification, confidence
//!   tier, downgrade reasons, contradictions, evidence records.
//! - **Tab 2 — Kernel Security**: live kernel security posture indicators,
//!   boot integrity state, cryptographic posture (FIPS), and hardening
//!   assessment from `PostureSnapshot`.
//!
//! Key bindings: `Tab`/`Right` = next tab, `Shift-Tab`/`Left` = prev tab,
//! `j`/`k` = scroll, `q`/`Esc` = quit.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Component inventory via substrate identity.
//! - **NIST SP 800-53 SI-7**: Software integrity via label trust / T4 gate.
//! - **NIST SP 800-53 AU-3**: Evidence chain display for audit record content.

use std::time::Duration;

use crossterm::event::{self, Event};
use std::collections::BTreeMap;
use umrs_core::i18n;

use umrs_platform::detect::label_trust::LabelTrust;
use umrs_platform::detect::{DetectionError, DetectionResult, OsDetector};
use umrs_platform::evidence::{EvidenceRecord, SourceKind};
use umrs_platform::posture::{IndicatorId, LiveValue, PostureSnapshot};
use umrs_platform::{Distro, OsFamily, OsRelease, TrustLevel};
use umrs_tui::app::{
    AuditCardApp, AuditCardState, DataRow, HeaderContext, IndicatorValue,
    SecurityIndicators, StatusLevel, StatusMessage, StyleHint, TabDef,
};
use umrs_tui::indicators::{build_header_context, read_system_uuid};
use umrs_tui::keymap::KeyMap;
use umrs_tui::layout::render_audit_card;
use umrs_tui::theme::Theme;

// ---------------------------------------------------------------------------
// Display helpers
// ---------------------------------------------------------------------------

const fn trust_level_label(level: TrustLevel) -> &'static str {
    match level {
        TrustLevel::Untrusted => "T0 — Untrusted",
        TrustLevel::KernelAnchored => "T1 — KernelAnchored",
        TrustLevel::EnvAnchored => "T2 — EnvAnchored",
        TrustLevel::SubstrateAnchored => "T3 — SubstrateAnchored",
        TrustLevel::IntegrityAnchored => "T4 — IntegrityAnchored",
    }
}

const fn trust_level_description(level: TrustLevel) -> &'static str {
    match level {
        TrustLevel::Untrusted => "No kernel anchor established.",
        TrustLevel::KernelAnchored => {
            "procfs verified via PROC_SUPER_MAGIC + PID coherence."
        }
        TrustLevel::EnvAnchored => {
            "Mount topology cross-checked (mountinfo vs statfs)."
        }
        TrustLevel::SubstrateAnchored => {
            "Package substrate parsed; identity from >= 2 facts."
        }
        TrustLevel::IntegrityAnchored => {
            "os-release ownership + installed digest verified."
        }
    }
}

const fn trust_level_hint(level: TrustLevel) -> StyleHint {
    match level {
        TrustLevel::Untrusted => StyleHint::TrustRed,
        TrustLevel::KernelAnchored | TrustLevel::EnvAnchored => {
            StyleHint::TrustYellow
        }
        TrustLevel::SubstrateAnchored | TrustLevel::IntegrityAnchored => {
            StyleHint::TrustGreen
        }
    }
}

const fn source_kind_label(kind: &SourceKind) -> &'static str {
    match kind {
        SourceKind::Procfs => "procfs",
        SourceKind::RegularFile => "regular-file",
        SourceKind::PackageDb => "package-db",
        SourceKind::SymlinkTarget => "symlink-target",
        SourceKind::SysfsNode => "sysfs",
        SourceKind::StatfsResult => "statfs",
    }
}

/// Derive the OS display name from an `OsRelease` value.
///
/// Prefers `NAME + VERSION_ID` (e.g., "CentOS Stream 10") for brevity.
/// Falls back to bare `NAME` if `VERSION_ID` is absent. Returns
/// `"unavailable"` when `os_release` is `None`.
///
/// `PRETTY_NAME` is intentionally not used — it often includes codenames
/// or parenthetical suffixes (e.g., "CentOS Stream 10 (Coughlan)") that
/// are too long for the header's fixed-width left column.
///
/// This is display-only — not a trust-relevant assertion.
fn os_name_from_release(rel: Option<&OsRelease>) -> String {
    let Some(rel) = rel else {
        return "unavailable".to_owned();
    };
    if let Some(ver) = &rel.version_id {
        return format!("{} {}", rel.name.as_str(), ver.as_str());
    }
    rel.name.as_str().to_owned()
}

fn distro_label(distro: &Distro) -> String {
    match distro {
        Distro::Rhel => "RHEL".to_owned(),
        Distro::Fedora => "Fedora".to_owned(),
        Distro::CentOs => "CentOS".to_owned(),
        Distro::AlmaLinux => "AlmaLinux".to_owned(),
        Distro::RockyLinux => "Rocky Linux".to_owned(),
        Distro::Debian => "Debian".to_owned(),
        Distro::Ubuntu => "Ubuntu".to_owned(),
        Distro::Kali => "Kali Linux".to_owned(),
        Distro::Other(s) => s.clone(),
    }
}

const fn family_label(family: &OsFamily) -> &'static str {
    match family {
        OsFamily::RpmBased => "RPM-based",
        OsFamily::DpkgBased => "dpkg-based",
        OsFamily::PacmanBased => "pacman-based",
        OsFamily::Unknown => "unknown",
    }
}

// ---------------------------------------------------------------------------
// OsDetectApp
// ---------------------------------------------------------------------------

/// Audit card data source backed by the OS detection pipeline.
///
/// Constructed once; detection is not re-run on refresh (the result is
/// immutable after construction). The `status` field is mutable so the
/// caller can update it to reflect the detection outcome.
///
/// Three tabs are presented:
/// - Tab 0 — OS Information: `os-release` fields, substrate identity, boot ID.
/// - Tab 1 — Trust / Evidence: label trust classification, confidence tier,
///   downgrade reasons, contradictions, evidence records.
/// - Tab 2 — Kernel Security: boot integrity, cryptographic posture, kernel
///   self-protection, process isolation, filesystem hardening, and module
///   restrictions — populated from a live `PostureSnapshot`.
///
/// NIST SP 800-53 CM-8, SI-7, AU-3, CA-7.
struct OsDetectApp {
    tabs: Vec<TabDef>,
    os_info_rows: Vec<DataRow>,
    trust_rows: Vec<DataRow>,
    kernel_security_rows: Vec<DataRow>,
    status: StatusMessage,
}

impl OsDetectApp {
    /// Build the app from a successful detection result.
    ///
    /// `ctx` is passed in for header indicators and kernel version string.
    /// `snap` provides the full `PostureSnapshot` for the Kernel Security tab.
    /// `system_uuid` is the DMI product UUID (display-only; may be "unavailable"
    /// if the sysfs read failed or root was not available).
    fn from_result(
        result: &DetectionResult,
        ctx: &HeaderContext,
        snap: &PostureSnapshot,
        system_uuid: &str,
    ) -> Self {
        let tabs = vec![
            TabDef::new("OS Information"),
            TabDef::new("Trust / Evidence"),
            TabDef::new("Kernel Security"),
        ];

        let os_info_rows = build_os_info_rows(result);
        let trust_rows = build_trust_rows(result);
        let kernel_security_rows = build_kernel_security_rows(
            snap,
            &ctx.indicators,
            &ctx.kernel_version,
            result.os_release.as_ref(),
            system_uuid,
        );
        let status = build_status(result);

        Self {
            tabs,
            os_info_rows,
            trust_rows,
            kernel_security_rows,
            status,
        }
    }

    /// Build the app from a hard-gate detection failure.
    ///
    /// `ctx` is passed in for header indicators and kernel version string.
    /// `snap` provides the full `PostureSnapshot` for the Kernel Security tab —
    /// kernel posture data is available independently of OS detection.
    /// `system_uuid` is the DMI product UUID (display-only).
    fn from_error(
        err: &DetectionError,
        ctx: &HeaderContext,
        snap: &PostureSnapshot,
        system_uuid: &str,
    ) -> Self {
        let tabs = vec![
            TabDef::new("OS Information"),
            TabDef::new("Trust / Evidence"),
            TabDef::new("Kernel Security"),
        ];

        // Error description — does not include variable kernel data
        // (NIST SP 800-53 SI-12 — no sensitive data in user-visible errors).
        let description = match err {
            DetectionError::ProcfsNotReal => {
                "Hard gate: procfs is not real procfs".to_owned()
            }
            DetectionError::PidCoherenceFailed {
                ..
            } => "Hard gate: PID coherence broken".to_owned(),
            DetectionError::KernelAnchorIo(_) => {
                "Hard gate: I/O error during kernel anchor".to_owned()
            }
        };

        let os_info_rows = vec![
            DataRow::new(
                "Status",
                "Detection pipeline failed",
                StyleHint::TrustRed,
            ),
            DataRow::new("Reason", description, StyleHint::TrustRed),
        ];

        let trust_rows = vec![
            DataRow::new("Trust level", "T0 — Untrusted", StyleHint::TrustRed),
            DataRow::new(
                "Reason",
                "Hard gate failure aborted pipeline",
                StyleHint::TrustRed,
            ),
        ];

        let kernel_security_rows = build_kernel_security_rows(
            snap,
            &ctx.indicators,
            &ctx.kernel_version,
            None,
            system_uuid,
        );
        let status =
            StatusMessage::new(StatusLevel::Error, "Detection pipeline failed");

        Self {
            tabs,
            os_info_rows,
            trust_rows,
            kernel_security_rows,
            status,
        }
    }
}

impl AuditCardApp for OsDetectApp {
    fn report_name(&self) -> &'static str {
        "OS Detection"
    }

    fn report_subject(&self) -> &'static str {
        "Platform Identity and Integrity"
    }

    fn card_title(&self) -> String {
        i18n::tr("OS Detection Audit")
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn active_tab(&self) -> usize {
        // Authoritative tab is in AuditCardState; this is a hint only.
        0
    }

    fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
        match tab_index {
            0 => self.os_info_rows.clone(),
            1 => self.trust_rows.clone(),
            2 => self.kernel_security_rows.clone(),
            _ => vec![DataRow::normal("(no data)", "(invalid tab index)")],
        }
    }

    fn status(&self) -> &StatusMessage {
        &self.status
    }
}

// ---------------------------------------------------------------------------
// Row builders
// ---------------------------------------------------------------------------

fn build_os_info_rows(result: &DetectionResult) -> Vec<DataRow> {
    let mut rows = Vec::new();

    // os-release fields
    if let Some(rel) = &result.os_release {
        rows.push(DataRow::new(
            "ID",
            rel.id.as_str().to_owned(),
            StyleHint::Highlight,
        ));
        rows.push(DataRow::normal("NAME", rel.name.as_str().to_owned()));
        if let Some(ver) = &rel.version_id {
            rows.push(DataRow::normal("VERSION_ID", ver.as_str().to_owned()));
        }
        if let Some(pn) = &rel.pretty_name {
            rows.push(DataRow::normal("PRETTY_NAME", pn.as_str().to_owned()));
        }
        if let Some(cpe) = &rel.cpe_name {
            rows.push(DataRow::normal("CPE_NAME", cpe.as_str().to_owned()));
        }
    } else {
        rows.push(DataRow::new(
            "os-release",
            i18n::tr("not available"),
            StyleHint::TrustYellow,
        ));
    }

    rows.push(DataRow::separator());

    // Substrate identity
    if let Some(sub) = &result.substrate_identity {
        rows.push(DataRow::new(
            i18n::tr("substrate family"),
            family_label(&sub.family).to_owned(),
            StyleHint::Highlight,
        ));
        if let Some(distro) = &sub.distro {
            rows.push(DataRow::normal(
                i18n::tr("substrate distro"),
                distro_label(distro),
            ));
        }
        if let Some(ver) = &sub.version_id {
            rows.push(DataRow::normal("substrate version", ver.clone()));
        }
        rows.push(DataRow::normal(
            "substrate facts",
            sub.facts_count.to_string(),
        ));
        rows.push(DataRow::normal("probe used", sub.probe_used.to_owned()));
    } else {
        rows.push(DataRow::new(
            "substrate identity",
            i18n::tr("not available"),
            StyleHint::TrustYellow,
        ));
    }

    rows.push(DataRow::separator());

    // Boot ID
    if let Some(boot) = &result.boot_id {
        rows.push(DataRow::normal("boot_id", boot.clone()));
    } else {
        rows.push(DataRow::new(
            "boot_id",
            i18n::tr("not available"),
            StyleHint::Dim,
        ));
    }

    rows
}

fn build_trust_rows(result: &DetectionResult) -> Vec<DataRow> {
    let mut rows = Vec::new();

    // Label trust
    let (lt_label, lt_hint) = label_trust_display(&result.label_trust);
    rows.push(DataRow::new(i18n::tr("label trust"), lt_label, lt_hint));

    rows.push(DataRow::separator());

    // Confidence level
    let level = result.confidence.level();
    rows.push(DataRow::new(
        i18n::tr("trust level"),
        trust_level_label(level).to_owned(),
        trust_level_hint(level),
    ));
    rows.push(DataRow::new(
        "description",
        trust_level_description(level).to_owned(),
        StyleHint::Dim,
    ));

    rows.push(DataRow::separator());

    // Downgrade reasons
    if result.confidence.downgrade_reasons.is_empty() {
        rows.push(DataRow::new(
            i18n::tr("downgrade reasons"),
            "none",
            StyleHint::TrustGreen,
        ));
    } else {
        rows.push(DataRow::new(
            i18n::tr("downgrade reasons"),
            result.confidence.downgrade_reasons.len().to_string(),
            StyleHint::TrustYellow,
        ));
        for (i, reason) in
            result.confidence.downgrade_reasons.iter().enumerate()
        {
            let idx = i.saturating_add(1);
            rows.push(DataRow::new(
                format!("  [{idx}]"),
                reason.clone(),
                StyleHint::Dim,
            ));
        }
    }

    rows.push(DataRow::separator());

    // Contradictions
    if result.confidence.contradictions.is_empty() {
        rows.push(DataRow::new(
            i18n::tr("contradictions"),
            "none",
            StyleHint::TrustGreen,
        ));
    } else {
        rows.push(DataRow::new(
            i18n::tr("contradictions"),
            result.confidence.contradictions.len().to_string(),
            StyleHint::TrustRed,
        ));
        for (i, con) in result.confidence.contradictions.iter().enumerate() {
            let idx = i.saturating_add(1);
            let desc: String = con.description.chars().take(64).collect();
            rows.push(DataRow::new(
                format!("  [{idx}] {} vs {}", con.source_a, con.source_b),
                desc,
                StyleHint::TrustRed,
            ));
        }
    }

    rows.push(DataRow::separator());

    // Evidence records — grouped by source kind and displayed as a table.
    // Groups are ordered deterministically by their label string (BTreeMap).
    // NIST SP 800-53 AU-3 — evidence is labelled, grouped, and structured.
    // NIST SP 800-53 SI-12 — no raw kernel values or security labels in display.
    let evidence = result.evidence.records();
    rows.push(DataRow::new(
        i18n::tr("evidence records"),
        evidence.len().to_string(),
        StyleHint::Normal,
    ));

    if !evidence.is_empty() {
        rows.push(DataRow::separator());
        append_grouped_evidence(&mut rows, evidence);
    }

    rows
}

/// Append kernel identity preamble rows to the Kernel Security tab row list.
///
/// Emits: OS release label (if available), version ID, kernel version,
/// system UUID (DMI), and active LSM indicator. A separator follows.
///
/// These are informational rows — none are hardening assertions. They provide
/// component identity context (NIST SP 800-53 CM-8) so the operator can
/// correlate the security posture data with the specific OS and hardware.
fn append_kernel_identity_rows(
    rows: &mut Vec<DataRow>,
    indicators: &SecurityIndicators,
    kernel_version: &str,
    os_release: Option<&OsRelease>,
    system_uuid: &str,
) {
    // OS stability context — from os-release fields; display-only.
    if let Some(rel) = os_release {
        let os_id = rel.id.as_str().to_owned();
        let os_label = if let Some(pn) = &rel.pretty_name {
            pn.as_str().to_owned()
        } else if let Some(ver) = &rel.version_id {
            format!("{} {}", rel.name.as_str(), ver.as_str())
        } else {
            rel.name.as_str().to_owned()
        };
        rows.push(DataRow::key_value("os release", os_label, StyleHint::Dim));
        if let Some(ver) = &rel.version_id {
            rows.push(DataRow::key_value(
                "version id",
                format!("{os_id} {}", ver.as_str()),
                StyleHint::Dim,
            ));
        }
        rows.push(DataRow::separator());
    }

    // Kernel version from uname(2) — display-only, not a hardening assertion.
    rows.push(DataRow::key_value(
        "kernel version",
        kernel_version.to_owned(),
        StyleHint::Dim,
    ));

    // System UUID from /sys/class/dmi/id/product_uuid — display-only.
    // Readable only by root; "unavailable" on non-root or non-UEFI systems.
    rows.push(DataRow::key_value(
        "system uuid",
        system_uuid.to_owned(),
        StyleHint::Dim,
    ));

    // Active LSM — no kattr type for /sys/kernel/security/lsm yet.
    // Uses the header indicator value; returns Unavailable until implemented.
    let (lsm_val, lsm_hint) = indicator_to_display(&indicators.active_lsm);
    rows.push(DataRow::key_value("active lsm", lsm_val, lsm_hint));

    rows.push(DataRow::separator());
}

/// Build the Kernel Security tab rows from a live `PostureSnapshot`.
///
/// Organises all probed indicators into six purpose-based groups. Groups with
/// no readable indicator data are omitted entirely — an empty group with only
/// `"(not probed)"` entries is noise. Kernel version appears as an
/// informational row at the top, not as its own group.
///
/// Indicator styling follows the hardening assessment from the snapshot:
/// - `meets_desired = Some(true)` → `TrustGreen` (hardened)
/// - `meets_desired = Some(false)` → `TrustRed` (not hardened)
/// - `meets_desired = None` (unreadable) → `Dim`
///
/// For the lockdown indicator, the header's `SecurityIndicators` value is
/// used as a cross-reference since the `Lockdown` indicator in the snapshot
/// carries the same value via a different read path.
///
/// NIST SP 800-53 CA-7: Continuous Monitoring — all rendered values are
/// sourced from a single atomic posture snapshot.
/// NIST SP 800-53 CM-6: Configuration Settings — live kernel values rendered
/// without interpretation or transformation.
/// NIST SP 800-53 SI-11: Error Handling — unreadable indicators display as Dim
/// rather than propagating errors to the display layer.
fn build_kernel_security_rows(
    snap: &PostureSnapshot,
    indicators: &SecurityIndicators,
    kernel_version: &str,
    os_release: Option<&OsRelease>,
    system_uuid: &str,
) -> Vec<DataRow> {
    let mut rows = Vec::new();

    append_kernel_identity_rows(
        &mut rows,
        indicators,
        kernel_version,
        os_release,
        system_uuid,
    );

    append_boot_integrity_group(&mut rows, snap, indicators);
    append_indicator_group(
        &mut rows,
        "CRYPTOGRAPHIC POSTURE",
        snap,
        &[
            (IndicatorId::FipsEnabled, "fips_enabled"),
            (IndicatorId::ModulesDisabled, "modules_disabled"),
            (IndicatorId::RandomTrustCpu, "random.trust_cpu"),
            (
                IndicatorId::RandomTrustBootloader,
                "random.trust_bootloader",
            ),
        ],
    );
    append_indicator_group(
        &mut rows,
        "KERNEL SELF-PROTECTION",
        snap,
        &[
            (IndicatorId::RandomizeVaSpace, "randomize_va_space"),
            (IndicatorId::KptrRestrict, "kptr_restrict"),
            (IndicatorId::UnprivBpfDisabled, "unprivileged_bpf_disabled"),
            (IndicatorId::PerfEventParanoid, "perf_event_paranoid"),
            (IndicatorId::YamaPtraceScope, "yama.ptrace_scope"),
            (IndicatorId::DmesgRestrict, "dmesg_restrict"),
        ],
    );
    append_indicator_group(
        &mut rows,
        "PROCESS ISOLATION",
        snap,
        &[
            (IndicatorId::UnprivUsernsClone, "unprivileged_userns_clone"),
            (IndicatorId::Sysrq, "sysrq"),
            (IndicatorId::SuidDumpable, "suid_dumpable"),
        ],
    );
    append_indicator_group(
        &mut rows,
        "FILESYSTEM HARDENING",
        snap,
        &[
            (IndicatorId::ProtectedSymlinks, "protected_symlinks"),
            (IndicatorId::ProtectedHardlinks, "protected_hardlinks"),
            (IndicatorId::ProtectedFifos, "protected_fifos"),
            (IndicatorId::ProtectedRegular, "protected_regular"),
        ],
    );
    append_indicator_group(
        &mut rows,
        "MODULE RESTRICTIONS",
        snap,
        &[
            (IndicatorId::BluetoothBlacklisted, "bluetooth (blacklisted)"),
            (
                IndicatorId::UsbStorageBlacklisted,
                "usb_storage (blacklisted)",
            ),
            (
                IndicatorId::FirewireCoreBlacklisted,
                "firewire_core (blacklisted)",
            ),
            (
                IndicatorId::ThunderboltBlacklisted,
                "thunderbolt (blacklisted)",
            ),
            (IndicatorId::NfConntrackAcct, "nf_conntrack acct"),
        ],
    );

    // Remove trailing separator if present.
    if matches!(rows.last(), Some(DataRow::Separator)) {
        rows.pop();
    }

    rows
}

/// Append the BOOT INTEGRITY group to the row list.
///
/// Lockdown is the primary boot-integrity indicator. If the posture snapshot
/// could not read the securityfs node (kernel without CONFIG_SECURITY_LOCKDOWN),
/// the header indicator value is used as a fallback so the row is never silently
/// absent on systems where lockdown is otherwise visible in the header.
///
/// Other boot-integrity indicators (kexec, module sig, mitigations, PTI) are
/// appended from the snapshot. The group is omitted only if every indicator is
/// unreadable AND the fallback indicator is also unavailable.
fn append_boot_integrity_group(
    rows: &mut Vec<DataRow>,
    snap: &PostureSnapshot,
    indicators: &SecurityIndicators,
) {
    // Snapshot-sourced boot-integrity indicators.
    let boot_indicators: &[(IndicatorId, &str)] = &[
        (IndicatorId::Lockdown, "lockdown"),
        (IndicatorId::KexecLoadDisabled, "kexec_load_disabled"),
        (IndicatorId::ModuleSigEnforce, "module.sig_enforce"),
        (IndicatorId::Mitigations, "mitigations"),
        (IndicatorId::Pti, "pti"),
    ];
    let group_rows = indicator_group_rows(snap, boot_indicators);

    // If lockdown was not in the snapshot, fall back to the header indicator.
    let has_lockdown =
        snap.get(IndicatorId::Lockdown).is_some_and(|r| r.live_value.is_some());
    let fallback: Option<DataRow> = if has_lockdown {
        None
    } else {
        let (lv, lh) = indicator_to_display(&indicators.lockdown_mode);
        if matches!(lh, StyleHint::Dim) {
            None // header indicator also unavailable — skip
        } else {
            Some(DataRow::key_value("lockdown (header)", lv, lh))
        }
    };

    if !group_rows.is_empty() || fallback.is_some() {
        rows.push(DataRow::group_title("BOOT INTEGRITY"));
        rows.extend(group_rows);
        if let Some(r) = fallback {
            rows.push(r);
        }
        rows.push(DataRow::separator());
    }
}

/// Append a named indicator group to the row list.
///
/// Skips the group header and separator entirely when no indicator in `signals`
/// has a readable live value — an empty group is not shown.
fn append_indicator_group(
    rows: &mut Vec<DataRow>,
    title: &'static str,
    snap: &PostureSnapshot,
    signals: &[(IndicatorId, &str)],
) {
    let group_rows = indicator_group_rows(snap, signals);
    if !group_rows.is_empty() {
        rows.push(DataRow::group_title(title));
        rows.extend(group_rows);
        rows.push(DataRow::separator());
    }
}

/// Build display rows for a named group of posture indicators.
///
/// For each `(IndicatorId, label)` pair, looks up the `IndicatorReport` in the
/// snapshot. Indicators with a readable `live_value` are rendered as
/// `DataRow::key_value` rows styled by their hardening outcome. Indicators
/// with no live value (`live_value: None`) are silently skipped — the caller
/// is responsible for deciding whether to omit the group entirely.
///
/// Returns an empty `Vec` when no indicator in the group has a readable value,
/// allowing the caller to suppress the group header.
///
/// NIST SP 800-53 SI-11: degraded signals are skipped, not fabricated.
fn indicator_group_rows(
    snap: &PostureSnapshot,
    signals: &[(IndicatorId, &str)],
) -> Vec<DataRow> {
    let mut rows = Vec::new();
    for (id, label) in signals {
        let Some(report) = snap.get(*id) else {
            continue;
        };
        let Some(ref live) = report.live_value else {
            // Indicator was not readable on this kernel — omit the row.
            continue;
        };
        let hint = meets_desired_hint(report.meets_desired);
        rows.push(DataRow::key_value(
            label.to_owned(),
            format_live_value(live),
            hint,
        ));
    }
    rows
}

/// Map a `LiveValue` to a concise display string for the Kernel Security tab.
///
/// Integer values are rendered as their decimal string. Boolean values are
/// rendered as `"enabled"` / `"disabled"` for operator clarity rather than
/// Rust's `true` / `false`. Text values are passed through unchanged.
fn format_live_value(live: &LiveValue) -> String {
    match live {
        LiveValue::Integer(v) => v.to_string(),
        LiveValue::SignedInteger(v) => v.to_string(),
        LiveValue::Bool(true) => "enabled".to_owned(),
        LiveValue::Bool(false) => "disabled".to_owned(),
        LiveValue::Text(s) => s.clone(),
    }
}

/// Map a hardening assessment to the appropriate `StyleHint`.
///
/// - `Some(true)` → `TrustGreen` — indicator meets the hardened baseline.
/// - `Some(false)` → `TrustRed` — indicator does not meet the hardened baseline.
/// - `None` → `Dim` — the assessment could not be computed (unreadable or
///   custom indicator type).
const fn meets_desired_hint(meets: Option<bool>) -> StyleHint {
    match meets {
        Some(true) => StyleHint::TrustGreen,
        Some(false) => StyleHint::TrustRed,
        None => StyleHint::Dim,
    }
}

/// Convert an [`IndicatorValue`] to a display string and [`StyleHint`] pair.
///
/// Used by [`build_kernel_security_rows`] to render live kernel indicator
/// values in the Kernel Security tab with the same semantic styling as the
/// header indicator row.
///
/// - `Active(s)` → value string + `StyleHint::TrustGreen`
/// - `Inactive(s)` → value string + `StyleHint::TrustYellow`
/// - `Unavailable` → `"unavailable"` + `StyleHint::Dim`
fn indicator_to_display(value: &IndicatorValue) -> (String, StyleHint) {
    match value {
        IndicatorValue::Active(s) => (s.clone(), StyleHint::TrustGreen),
        IndicatorValue::Inactive(s) => (s.clone(), StyleHint::TrustYellow),
        IndicatorValue::Unavailable => {
            ("unavailable".to_owned(), StyleHint::Dim)
        }
    }
}

/// Append evidence records grouped by source kind to the row list.
///
/// Records are collected into a `BTreeMap` keyed by the source-kind display
/// label so groups are emitted in a stable, deterministic order. Within each
/// group, records appear in pipeline-append order (the bundle is append-only).
///
/// Each group is introduced by a `GroupTitle`, followed by a `TableHeader`,
/// followed by one `TableRow` per record. Groups are separated by a
/// `Separator`. No raw kernel values or security-label data appear in the
/// display strings (NIST SP 800-53 SI-12).
///
/// NIST SP 800-53 AU-3 — evidence rows are labelled and structured.
fn append_grouped_evidence(
    rows: &mut Vec<DataRow>,
    evidence: &[EvidenceRecord],
) {
    // Build a BTreeMap<group_label, Vec<record_ref>> for deterministic ordering.
    let mut groups: BTreeMap<&'static str, Vec<&EvidenceRecord>> =
        BTreeMap::new();
    for rec in evidence {
        groups
            .entry(source_kind_label(&rec.source_kind))
            .or_default()
            .push(rec);
    }

    let mut first_group = true;
    for (group_label, records) in &groups {
        if !first_group {
            rows.push(DataRow::separator());
        }
        first_group = false;

        rows.push(DataRow::group_title(group_label.to_uppercase()));
        rows.push(DataRow::table_header(
            "Evidence Type",
            "Source",
            "Verification",
        ));

        for rec in records {
            rows.push(DataRow::table_row(
                // col1: source kind (same as the group label — keeps the row
                // readable in isolation even when scrolled away from the title).
                source_kind_label(&rec.source_kind),
                // col2: path truncated to TABLE_COL2_WIDTH (24 chars).
                // Paths are clipped here; data_panel clips at render time too,
                // providing defence-in-depth against display overflow.
                // NIST SP 800-53 SI-12 — no sensitive data in display strings.
                rec.path_requested.chars().take(24).collect::<String>(),
                // col3: structured verification outcome.
                evidence_verification_str(rec),
                evidence_style_hint(rec),
            ));
        }
    }
}

/// Map an `EvidenceRecord` to a structured verification outcome string.
///
/// Returns a minimal, status-code-style string that an assessor can read and
/// act on. Unicode check (✓ U+2713) and cross (✗ U+2717) mark positive and
/// negative outcomes. The open-by-fd flag and source kind provide enough
/// context for independent verification without exposing raw kernel data.
///
/// NIST SP 800-53 AU-3 — verification strings identify the outcome and method.
fn evidence_verification_str(rec: &EvidenceRecord) -> String {
    let open_method = if rec.opened_by_fd {
        "fd"
    } else {
        "path"
    };
    if rec.parse_ok {
        format!("\u{2713} ok ({open_method})")
    } else {
        format!("\u{2717} FAIL ({open_method})")
    }
}

/// Map an `EvidenceRecord` to the appropriate `StyleHint` for the
/// verification column.
///
/// - Parse succeeded → `TrustGreen`
/// - Parse failed → `TrustRed`
const fn evidence_style_hint(rec: &EvidenceRecord) -> StyleHint {
    if rec.parse_ok {
        StyleHint::TrustGreen
    } else {
        StyleHint::TrustRed
    }
}

fn label_trust_display(trust: &LabelTrust) -> (String, StyleHint) {
    match trust {
        LabelTrust::UntrustedLabelCandidate => (
            "UntrustedLabelCandidate — do not use for policy".to_owned(),
            StyleHint::TrustRed,
        ),
        LabelTrust::LabelClaim => (
            "LabelClaim — structurally valid; integrity unconfirmed".to_owned(),
            StyleHint::TrustYellow,
        ),
        LabelTrust::TrustedLabel => (
            "TrustedLabel — T4: ownership + digest verified".to_owned(),
            StyleHint::TrustGreen,
        ),
        LabelTrust::IntegrityVerifiedButContradictory {
            contradiction,
        } => {
            let desc: String = contradiction.chars().take(48).collect();
            (
                format!("IntegrityVerifiedButContradictory: {desc}"),
                StyleHint::TrustRed,
            )
        }
    }
}

fn build_status(result: &DetectionResult) -> StatusMessage {
    match result.confidence.level() {
        TrustLevel::IntegrityAnchored => {
            StatusMessage::new(StatusLevel::Ok, "Integrity Anchored")
        }
        TrustLevel::SubstrateAnchored => {
            StatusMessage::new(StatusLevel::Info, "Substrate Anchored")
        }
        TrustLevel::EnvAnchored => StatusMessage::new(
            StatusLevel::Warn,
            format!(
                "{} — {}",
                trust_level_label(TrustLevel::EnvAnchored),
                trust_level_description(TrustLevel::EnvAnchored)
            ),
        ),
        TrustLevel::KernelAnchored => StatusMessage::new(
            StatusLevel::Warn,
            format!(
                "{} — {}",
                trust_level_label(TrustLevel::KernelAnchored),
                trust_level_description(TrustLevel::KernelAnchored)
            ),
        ),
        TrustLevel::Untrusted => StatusMessage::new(
            StatusLevel::Error,
            "Untrusted — no kernel anchor",
        ),
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    // ── i18n ─────────────────────────────────────────────────────────────
    // Initialize gettext catalog for the "umrs-tui" domain. Must be called
    // before any i18n::tr() calls. Falls back to the msgid if no catalog
    // is found — no error surfaced to the user.
    i18n::init("umrs-tui");

    // ── Logging ──────────────────────────────────────────────────────────
    // Best-effort journald logger. Failures are silently ignored — a TUI
    // should not write to stderr (would corrupt the terminal state).
    if let Ok(logger) = systemd_journal_logger::JournalLog::new() {
        // Ignore install error — another logger may already be set.
        let _ = logger.install();
        log::set_max_level(log::LevelFilter::Info);
    }

    // ── Header context ───────────────────────────────────────────────────
    // Build before detection so that live security indicators (lockdown mode,
    // FIPS state) and the kernel version string are available to the Kernel
    // Security tab row builder. Detection result is independent of ctx.
    // os_name is initially "unavailable" and updated after detection completes.
    let mut ctx = build_header_context(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        "unavailable",
    );

    // ── Posture snapshot ─────────────────────────────────────────────────
    // Collect all kernel security posture indicators once before building the app.
    // The snapshot is independent of OS detection — it reads directly from
    // kernel nodes via the provenance-verified SecureReader engine.
    // Used exclusively to populate the Kernel Security tab.
    //
    // NIST SP 800-53 CA-7: Continuous Monitoring — posture collected at startup.
    let snap = PostureSnapshot::collect();
    log::info!(
        "posture snapshot: {}/{} indicators readable",
        snap.readable_count(),
        snap.reports.len()
    );

    // ── System UUID ───────────────────────────────────────────────────────
    // Read once here for the Kernel Security tab. Requires root on most kernels;
    // returns "unavailable" if the sysfs read fails. Display-only.
    let system_uuid = read_system_uuid();

    // ── Detection ────────────────────────────────────────────────────────
    let app: OsDetectApp = match OsDetector::default().detect() {
        Ok(result) => {
            log::info!(
                "OS detection succeeded: {:?}",
                result.confidence.level()
            );
            // Populate os_name from the detection result now that it is available.
            ctx.os_name = os_name_from_release(result.os_release.as_ref());
            OsDetectApp::from_result(&result, &ctx, &snap, &system_uuid)
        }
        Err(ref e) => {
            log::warn!("OS detection hard-gate failure: {e}");
            OsDetectApp::from_error(e, &ctx, &snap, &system_uuid)
        }
    };

    // ── UI state ─────────────────────────────────────────────────────────
    let mut state = AuditCardState::new(app.tabs().len());
    let keymap = KeyMap::default();
    let theme = Theme::default();

    // ── Terminal setup ────────────────────────────────────────────────────
    let mut terminal = ratatui::init();

    // ── Event loop ───────────────────────────────────────────────────────
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_audit_card(f, f.area(), &app, &state, &ctx, &theme);
        }) {
            log::error!("terminal draw error: {e}");
            break;
        }

        match event::poll(Duration::from_millis(250)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => {
                    if let Some(action) = keymap.lookup(&key) {
                        state.handle_action(&action);
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    log::warn!("event read error: {e}");
                }
            },
            Ok(false) => {}
            Err(e) => {
                log::warn!("event poll error: {e}");
            }
        }

        if state.should_quit {
            break;
        }
    }

    // ── Terminal teardown ─────────────────────────────────────────────────
    ratatui::restore();
}
