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

//! # umrs-uname — OS Detection Audit Card
//!
//! Runs the `umrs-platform` OS detection pipeline and displays the result
//! as an interactive ratatui audit card. Three tabs present the data:
//!
//! - **Tab 0 — OS Information**: `os-release` fields, platform identity,
//!   boot ID.
//! - **Tab 1 — Kernel Security**: live kernel security posture indicators,
//!   boot integrity state, cryptographic posture (FIPS), and hardening
//!   assessment from `PostureSnapshot`.
//! - **Tab 2 — Trust / Evidence**: label trust classification, confidence
//!   tier, downgrade reasons, contradictions, evidence records. Always last.
//!
//! Key bindings: `Tab`/`Right` = next tab, `Shift-Tab`/`Left` = prev tab,
//! `j`/`k` = scroll, `q`/`Esc` = quit.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Component inventory via platform identity.
//! - **NIST SP 800-53 SI-7**: Software integrity via label trust / T4 gate.
//! - **NIST SP 800-53 AU-3**: Evidence chain display for audit record content.

use std::time::Duration;

use crossterm::event::{self, Event};
use umrs_core::i18n;

use umrs_platform::detect::label_trust::LabelTrust;
use umrs_platform::detect::{DetectionError, DetectionResult, OsDetector};
use umrs_platform::evidence::{EvidenceRecord, SourceKind};
use umrs_platform::posture::{
    CATALOG_KERNEL_BASELINE, ConfiguredValue, ContradictionKind, IndicatorId,
    PostureSnapshot, annotate_live_value, lookup,
};
use umrs_platform::{Distro, KernelVersion, OsFamily, OsRelease, TrustLevel};
use umrs_ui::app::{
    AuditCardApp, AuditCardState, DataRow, HeaderContext, IndicatorValue,
    SecurityIndicators, StatusLevel, StatusMessage, StyleHint, TabDef,
};
use umrs_ui::dialog::{DialogState, render_dialog};
use umrs_ui::indicators::build_header_context;
use umrs_ui::keymap::{Action, KeyMap};
use umrs_ui::layout::render_audit_card;
use umrs_ui::theme::Theme;

// ---------------------------------------------------------------------------
// Display helpers
// ---------------------------------------------------------------------------

const fn trust_level_label(level: TrustLevel) -> &'static str {
    match level {
        TrustLevel::Untrusted => "T0 — Untrusted",
        TrustLevel::KernelAnchored => "T1 — Kernel Anchored",
        TrustLevel::EnvAnchored => "T2 — Environment Anchored",
        TrustLevel::SubstrateAnchored => "T3 — Platform Verified",
        TrustLevel::IntegrityAnchored => "T4 — Integrity Anchored",
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
            "Platform identity verified; >= 2 independent package facts confirmed."
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

/// Map a `SourceKind` to a plain-English display label for the evidence table.
///
/// These labels are operator-facing. Each label names the evidence type in
/// terms an operator can relate to the actual source on the system, without
/// requiring knowledge of internal type names.
const fn source_kind_label(kind: &SourceKind) -> &'static str {
    match kind {
        SourceKind::Procfs => "Kernel runtime (/proc)",
        SourceKind::RegularFile => "Configuration file",
        SourceKind::PackageDb => "Package database",
        SourceKind::SymlinkTarget => "Symlink target",
        SourceKind::SysfsNode => "Kernel attributes (/sys)",
        SourceKind::StatfsResult => "Filesystem identity",
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
/// - Tab 0 — OS Information: `os-release` fields, platform identity, boot ID.
/// - Tab 1 — Kernel Security: boot integrity, cryptographic posture, kernel
///   self-protection, process isolation, filesystem hardening, and module
///   restrictions — populated from a live `PostureSnapshot`.
/// - Tab 2 — Trust / Evidence: label trust classification (pinned summary at
///   top), confidence tier, downgrade reasons, contradictions, and a scrollable
///   evidence chain below. Always the last (rightmost) tab — UMRS convention.
///
/// Tab 2 uses the split-panel layout: `trust_summary_rows` are pinned via
/// `pinned_rows()` and always visible; `trust_evidence_rows` are the scrollable
/// evidence chain returned by `data_rows()`.
///
/// NIST SP 800-53 CM-8, SI-7, AU-3, CA-7.
struct OsDetectApp {
    tabs: Vec<TabDef>,
    os_info_rows: Vec<DataRow>,
    trust_summary_rows: Vec<DataRow>,
    trust_evidence_rows: Vec<DataRow>,
    kernel_security_summary_rows: Vec<DataRow>,
    kernel_security_rows: Vec<DataRow>,
    status: StatusMessage,
}

impl OsDetectApp {
    /// Build the app from a successful detection result.
    ///
    /// `ctx` is passed in for header indicators and kernel version string.
    /// `snap` provides the full `PostureSnapshot` for the Kernel Security tab.
    fn from_result(
        result: &DetectionResult,
        ctx: &HeaderContext,
        snap: &PostureSnapshot,
    ) -> Self {
        // Tab order: OS Information → Kernel Security → Trust / Evidence.
        // Trust/Evidence is intentionally last — it is the deepest-dive tab
        // (evidence chain) and operators typically start with OS identity and
        // kernel posture before reviewing the full evidence chain.
        // Convention: Trust/Evidence is always the rightmost (last) tab.
        let tabs = vec![
            TabDef::new(i18n::tr("OS Information")),
            TabDef::new(i18n::tr("Kernel Security")),
            TabDef::new(i18n::tr("Trust / Evidence")),
        ];

        let os_info_rows = build_os_info_rows(result);
        let trust_summary_rows = build_trust_summary_rows(result);
        let trust_evidence_rows = build_trust_evidence_rows(result);
        let kernel_security_summary_rows =
            build_kernel_security_summary_rows(snap, &ctx.kernel_version);
        let kernel_security_rows =
            build_kernel_security_rows(snap, &ctx.indicators);
        let status = build_status(result);

        Self {
            tabs,
            os_info_rows,
            trust_summary_rows,
            trust_evidence_rows,
            kernel_security_summary_rows,
            kernel_security_rows,
            status,
        }
    }

    /// Build the app from a hard-gate detection failure.
    ///
    /// `ctx` is passed in for header indicators and kernel version string.
    /// `snap` provides the full `PostureSnapshot` for the Kernel Security tab —
    /// kernel posture data is available independently of OS detection.
    fn from_error(
        err: &DetectionError,
        ctx: &HeaderContext,
        snap: &PostureSnapshot,
    ) -> Self {
        // Error description — does not include variable kernel data
        // (NIST SP 800-53 SI-12 — no sensitive data in user-visible errors).
        let description = match err {
            DetectionError::ProcfsNotReal => {
                i18n::tr("Hard gate: procfs is not real procfs")
            }
            DetectionError::PidCoherenceFailed {
                ..
            } => i18n::tr("Hard gate: PID coherence broken"),
            DetectionError::KernelAnchorIo(_) => {
                i18n::tr("Hard gate: I/O error during kernel anchor")
            }
        };

        let os_info_rows = vec![
            DataRow::new(
                i18n::tr("Status"),
                i18n::tr("Detection pipeline failed"),
                StyleHint::TrustRed,
            ),
            DataRow::new(i18n::tr("Reason"), description, StyleHint::TrustRed),
        ];

        let trust_summary_rows = vec![
            DataRow::new(
                i18n::tr("Trust Level"),
                i18n::tr("T0 — Untrusted"),
                StyleHint::TrustRed,
            ),
            DataRow::new(
                i18n::tr("Reason"),
                i18n::tr("Hard gate failure aborted pipeline"),
                StyleHint::TrustRed,
            ),
        ];

        let trust_evidence_rows = Vec::new();

        let kernel_security_summary_rows =
            build_kernel_security_summary_rows(snap, &ctx.kernel_version);
        let kernel_security_rows =
            build_kernel_security_rows(snap, &ctx.indicators);
        let status = StatusMessage::new(
            StatusLevel::Error,
            i18n::tr("Detection pipeline failed"),
        );

        // Tab order: OS Information → Kernel Security → Trust / Evidence.
        // Trust/Evidence is always the rightmost (last) tab — convention
        // shared across all UMRS tools that use the audit card layout.
        let tabs = vec![
            TabDef::new(i18n::tr("OS Information")),
            TabDef::new(i18n::tr("Kernel Security")),
            TabDef::new(i18n::tr("Trust / Evidence")),
        ];

        Self {
            tabs,
            os_info_rows,
            trust_summary_rows,
            trust_evidence_rows,
            kernel_security_summary_rows,
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
        // Tab order: 0=OS Information, 1=Kernel Security, 2=Trust/Evidence.
        match tab_index {
            0 => self.os_info_rows.clone(),
            1 => self.kernel_security_rows.clone(),
            2 => self.trust_evidence_rows.clone(),
            _ => vec![DataRow::normal("(no data)", "(invalid tab index)")],
        }
    }

    fn pinned_rows(&self, tab_index: usize) -> Vec<DataRow> {
        // Tab order: 0=OS Information, 1=Kernel Security, 2=Trust/Evidence.
        match tab_index {
            1 => self.kernel_security_summary_rows.clone(),
            2 => self.trust_summary_rows.clone(),
            _ => Vec::new(),
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

    // Platform identity (package substrate)
    if let Some(sub) = &result.substrate_identity {
        rows.push(DataRow::new(
            i18n::tr("Platform Family"),
            i18n::tr(family_label(&sub.family)),
            StyleHint::Highlight,
        ));
        if let Some(distro) = &sub.distro {
            rows.push(DataRow::normal(
                i18n::tr("Platform Distro"),
                distro_label(distro),
            ));
        }
        if let Some(ver) = &sub.version_id {
            rows.push(DataRow::normal(
                i18n::tr("Platform Version"),
                ver.clone(),
            ));
        }
        rows.push(DataRow::normal(
            i18n::tr("Platform Facts"),
            sub.facts_count.to_string(),
        ));
        rows.push(DataRow::normal(
            i18n::tr("Probe Used"),
            sub.probe_used.to_owned(),
        ));
    } else {
        rows.push(DataRow::new(
            i18n::tr("Platform Identity"),
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

/// Build the pinned (fixed) summary rows for the Trust / Evidence tab.
///
/// These rows are displayed in a fixed pane above the scrollable evidence chain.
/// They always remain visible regardless of scroll position, so the operator
/// can always see the top-level trust classification while reviewing evidence.
///
/// Includes: label trust, confidence tier, downgrade reasons, and contradictions.
/// Evidence records are in the scrollable section (`build_trust_evidence_rows`).
///
/// NIST SP 800-53 AU-3 — critical trust classification is always visible.
/// NIST SP 800-53 SI-7 — trust level is derived from evidence, not asserted.
fn build_trust_summary_rows(result: &DetectionResult) -> Vec<DataRow> {
    let mut rows = Vec::new();

    // Label trust classification — the top-level finding.
    // i18n: label key and value both wrapped for translator discovery.
    let (lt_label, lt_hint) = label_trust_display(&result.label_trust);
    rows.push(DataRow::new(i18n::tr("Label Trust"), lt_label, lt_hint));

    // For IntegrityVerifiedButContradictory, emit the full contradiction text
    // as a secondary row. The primary label is shortened to fit on one line;
    // the complete detail is preserved here for assessment use.
    // NIST SP 800-53 AU-3 / SI-7 — audit records and integrity findings must be complete.
    if let LabelTrust::IntegrityVerifiedButContradictory {
        contradiction,
    } = &result.label_trust
    {
        rows.push(DataRow::key_value(
            "",
            contradiction.clone(),
            StyleHint::TrustRed,
        ));
    }

    // Confidence level — the numeric trust tier with plain-language description.
    // Note: trust_level_label/description are const fn helpers returning
    // &'static str; the translation wraps the returned value at the call site.
    let level = result.confidence.level();
    rows.push(DataRow::new(
        i18n::tr("Trust Tier"),
        i18n::tr(trust_level_label(level)),
        trust_level_hint(level),
    ));
    rows.push(DataRow::new(
        i18n::tr("Description"),
        i18n::tr(trust_level_description(level)),
        StyleHint::Dim,
    ));

    // Downgrade reasons — positive framing when none exist.
    // A downgrade reason means a check that would have elevated trust
    // could not be confirmed. "None" means full trust was retained.
    if result.confidence.downgrade_reasons.is_empty() {
        rows.push(DataRow::new(
            i18n::tr("Downgrade Reasons"),
            i18n::tr("No downgrade — full trust retained"),
            StyleHint::TrustGreen,
        ));
    } else {
        rows.push(DataRow::new(
            i18n::tr("Downgrade Reasons"),
            result.confidence.downgrade_reasons.len().to_string(),
            StyleHint::TrustYellow,
        ));
        rows.push(DataRow::key_value(
            "",
            i18n::tr("Each reason below prevented a trust tier upgrade."),
            StyleHint::Dim,
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

    // Contradictions — two independent evidence sources disagreed.
    // A contradiction means the same fact was asserted differently by two
    // separate checks. This may indicate tampering or a misconfigured system.
    if result.confidence.contradictions.is_empty() {
        rows.push(DataRow::new(
            i18n::tr("Contradictions"),
            i18n::tr("None detected"),
            StyleHint::TrustGreen,
        ));
        rows.push(DataRow::key_value(
            "",
            i18n::tr(
                "All evidence sources agreed \u{2014} no conflicting identity assertions detected.",
            ),
            StyleHint::Dim,
        ));
    } else {
        rows.push(DataRow::new(
            i18n::tr("Contradictions"),
            result.confidence.contradictions.len().to_string(),
            StyleHint::TrustRed,
        ));
        rows.push(DataRow::key_value(
            "",
            i18n::tr(
                "Two independent sources reported conflicting values. \
                 Review each pair — this may indicate tampering.",
            ),
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

    // Evidence record count — always visible in the summary pane so the
    // operator can correlate the trust tier with the volume of evidence
    // reviewed, without needing to scroll to the bottom of the evidence chain.
    let evidence_count = result.evidence.records().len();
    rows.push(DataRow::new(
        i18n::tr("Evidence Records"),
        evidence_count.to_string(),
        StyleHint::Normal,
    ));

    rows
}

/// Build the scrollable evidence chain rows for the Trust / Evidence tab.
///
/// Contains only the evidence records (the actual files and kernel nodes read).
/// The single `TableHeader` row appears once at the top of this section.
/// Records are then grouped by evidence type with a `GroupTitle` separator
/// but no repeated header row.
///
/// Separated from the trust summary so summary rows can be pinned (fixed)
/// while evidence rows scroll independently.
///
/// NIST SP 800-53 AU-3 — evidence records are labelled, grouped, and structured.
/// NIST SP 800-53 SI-12 — no raw kernel values or security labels in display.
fn build_trust_evidence_rows(result: &DetectionResult) -> Vec<DataRow> {
    let mut rows = Vec::new();

    let evidence = result.evidence.records();

    if !evidence.is_empty() {
        append_grouped_evidence(&mut rows, evidence);
    }

    rows
}

/// Append the Contradictions row and its one-line explanation to `rows`.
///
/// A contradiction on the Kernel Security tab means the running kernel and the
/// persisted configuration do not agree on a hardening setting. This can
/// indicate tampering, a missed sysctl apply (`sysctl -p`), or an ephemeral
/// runtime change that was never persisted to a drop-in file under
/// `/etc/sysctl.d/`. The explanation row beneath the count tells the operator
/// what the contradiction type means so they can act without consulting
/// external documentation.
///
/// Always shown — a zero count is a positive signal.
///
/// NIST SP 800-53 CA-7 — contradiction count surfaces configuration drift
/// in the always-visible summary pane so the assessor cannot miss it.
/// NIST SP 800-53 CM-6 — live/configured disagreements are a configuration
/// management finding that must be investigated before CUI processing.
fn append_kernel_contradiction_rows(rows: &mut Vec<DataRow>, count: usize) {
    rows.push(DataRow::separator());
    let (value, hint) = if count == 0 {
        (i18n::tr("None"), StyleHint::TrustGreen)
    } else {
        (
            format!(
                "{count} \u{2014} configuration/kernel disagreements detected"
            ),
            StyleHint::TrustRed,
        )
    };
    rows.push(DataRow::key_value_highlighted(
        i18n::tr("Contradictions"),
        value,
        hint,
    ));
    if count == 0 {
        rows.push(DataRow::key_value(
            "",
            i18n::tr("No disagreements between running kernel and persisted configuration."),
            StyleHint::Dim,
        ));
    } else {
        rows.push(DataRow::key_value(
            "",
            i18n::tr(
                "The running kernel and persisted configuration disagree on one or more \
                 settings. Drift means intended hardening is not active; hotfixes mean \
                 current hardening will be lost on reboot.",
            ),
            StyleHint::TrustRed,
        ));
    }
}

/// Produce the catalog baseline comparison row for the Kernel Security summary.
///
/// Compares the running kernel (`kernel_version` from `uname(2)`) against the
/// `CATALOG_KERNEL_BASELINE` constant that records which kernel the indicator
/// catalog targets. Returns a `DataRow` with one of three messages:
///
/// - Running newer than baseline → `StyleHint::Warn` advisory.
/// - Running matches baseline → `StyleHint::TrustGreen` confirmation.
/// - Running older than baseline (rare) → `StyleHint::TrustRed` warning.
/// - Either string fails to parse as `MAJOR.MINOR.PATCH` → `StyleHint::Dim`
///   neutral note; no false alarm on parse failure (fail-open for display).
///
/// This comparison is advisory only — it guides the operator in judging
/// whether the indicator catalog is current for their kernel. It does not
/// gate access or change the posture score.
///
/// NIST SP 800-53 CA-7: Continuous Monitoring — catalog currency is a
/// precondition for meaningful posture assessment.
fn catalog_baseline_row(kernel_version: &str) -> DataRow {
    let running = kernel_version.parse::<KernelVersion>();
    let baseline = CATALOG_KERNEL_BASELINE.parse::<KernelVersion>();

    match (running, baseline) {
        (Err(_), _) | (_, Err(_)) => DataRow::key_value(
            i18n::tr("Catalog Baseline"),
            format!("baseline {CATALOG_KERNEL_BASELINE}"),
            StyleHint::Dim,
        ),
        (Ok(r), Ok(b)) if r > b => DataRow::key_value(
            i18n::tr("Catalog Baseline"),
            format!(
                "{r} is newer than catalog baseline ({b}) \u{2014} \
                 some indicators may have changed"
            ),
            StyleHint::TrustYellow,
        ),
        (Ok(r), Ok(b)) if r == b => DataRow::key_value(
            i18n::tr("Catalog Baseline"),
            i18n::tr("Catalog baseline matches running kernel"),
            StyleHint::TrustGreen,
        ),
        (Ok(r), Ok(b)) => DataRow::key_value(
            i18n::tr("Catalog Baseline"),
            format!(
                "{r} is older than catalog baseline ({b}) \u{2014} \
                 update your kernel"
            ),
            StyleHint::TrustRed,
        ),
    }
}

/// Build the pinned (fixed) summary rows for the Kernel Security tab.
///
/// Displayed in a fixed pane above the scrollable indicator list, always
/// visible regardless of scroll position. Provides an at-a-glance hardening
/// score so the operator can assess overall posture before reviewing details.
///
/// Summary contents:
/// - Kernel version (always visible without scrolling — NIST SP 800-53 CM-8)
/// - Catalog baseline comparison — running kernel vs catalog target version
/// - Total indicators readable from this kernel
/// - Count of indicators that meet the hardened baseline
/// - Count of indicators that do not meet the hardened baseline
/// - Count with no assessment (custom or unreadable)
///
/// NIST SP 800-53 CA-7: Continuous Monitoring — top-level posture score is
/// always visible so the assessor cannot miss the overall finding.
/// NIST SP 800-53 CM-8: Component inventory — running kernel version always
/// visible in the summary pane.
fn build_kernel_security_summary_rows(
    snap: &PostureSnapshot,
    kernel_version: &str,
) -> Vec<DataRow> {
    // Kernel version — always visible without scrolling.
    // Operators correlate posture findings with the specific kernel under
    // assessment. NIST SP 800-53 CM-8: component inventory.
    //
    // Catalog baseline row — immediately below the running version so the
    // operator can see at a glance whether the indicator definitions are
    // current for this kernel. NIST SP 800-53 CA-7: Continuous Monitoring.
    //
    // Provenance note — makes clear to the operator that every value below
    // reflects the live running kernel state, not a saved configuration or
    // cached snapshot.
    let mut rows = vec![
        DataRow::key_value_highlighted(
            i18n::tr("Kernel Version"),
            kernel_version.to_owned(),
            StyleHint::Highlight,
        ),
        catalog_baseline_row(kernel_version),
        DataRow::key_value(
            "",
            i18n::tr(
                "All values below are read live from the running kernel via /proc and /sys.",
            ),
            StyleHint::Dim,
        ),
        DataRow::separator(),
    ];

    let readable = snap.readable_count();

    let mut hardened: usize = 0;
    let mut not_hardened: usize = 0;
    let mut no_assessment: usize = 0;

    for report in &snap.reports {
        if report.live_value.is_none() {
            continue; // not readable — skip from scoring
        }
        match report.meets_desired {
            Some(true) => hardened = hardened.saturating_add(1),
            Some(false) => not_hardened = not_hardened.saturating_add(1),
            None => no_assessment = no_assessment.saturating_add(1),
        }
    }

    // Single consolidated indicators line — replaces separate Hardened /
    // Not Hardened rows to save two summary lines. "Readable" is the count
    // that could actually be probed; the gap to total is already surfaced by
    // the No Assessment line below, so "X of Y total" is redundant noise.
    // All-hardened case gets a checkmark. Mixed case shows the split so
    // the operator immediately knows remediation scope.
    // NIST SP 800-53 CM-6 — posture summary is co-located with the detail.
    let indicators_value = if not_hardened == 0 {
        format!("{readable} readable \u{2014} all hardened \u{2713}")
    } else {
        // Compute the percentage of readable indicators that are not hardened.
        // Use saturating arithmetic; readable is always >= not_hardened by
        // construction, but protect against zero-divide defensively.
        let pct = if readable > 0 {
            not_hardened.saturating_mul(100) / readable
        } else {
            0
        };
        format!(
            "{readable} readable \u{2014} {hardened} hardened, \
             {not_hardened} not hardened ({pct}%)"
        )
    };
    let indicators_hint = if not_hardened > 0 {
        StyleHint::TrustRed
    } else {
        StyleHint::TrustGreen
    };
    rows.push(DataRow::key_value_highlighted(
        i18n::tr("Indicators"),
        indicators_value,
        indicators_hint,
    ));

    if no_assessment > 0 {
        rows.push(DataRow::new(
            i18n::tr("No Assessment"),
            no_assessment.to_string(),
            StyleHint::Dim,
        ));
    }

    // Contradictions — see helper for display and explanation logic.
    append_kernel_contradiction_rows(&mut rows, snap.contradictions().count());

    rows.push(DataRow::separator());

    // Curated note — blank line above and below for visual breathing room.
    rows.push(DataRow::key_value(
        "",
        i18n::tr(
            "Curated indicators selected to give you the clearest view of \
             your system's security posture. Items marked in red can be \
             hardened — see each indicator's recommended setting below.",
        ),
        StyleHint::Dim,
    ));
    rows.push(DataRow::separator());

    rows
}

/// Build the Kernel Security tab rows from a live `PostureSnapshot`.
///
/// Organises all probed indicators into six purpose-based groups. Groups with
/// no readable indicator data are omitted entirely — an empty group with only
/// `"(not probed)"` entries is noise. Kernel version is shown in the pinned
/// summary pane above, not repeated here.
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
) -> Vec<DataRow> {
    let mut rows = Vec::new();

    append_boot_integrity_group(&mut rows, snap, indicators);
    append_indicator_group(
        &mut rows,
        "CRYPTOGRAPHIC POSTURE",
        "Verifies government-validated cryptography and correct entropy \
         sourcing. Failures here mean sensitive operations may use \
         unvalidated algorithms.",
        snap,
        &[
            (IndicatorId::FipsEnabled, "fips_enabled"),
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
        "Controls that hide kernel internals from unprivileged processes. \
         Weak settings let attackers locate exploitable code and bypass ASLR.",
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
        "Controls how much one process can see or interfere with another. \
         Weak settings allow credential theft across sibling processes.",
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
        "Closes privilege escalation paths through the filesystem. \
         Absent controls allow symlink and hardlink attacks in world-writable directories.",
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
        "Verifies high-risk kernel modules are blocked from loading. \
         USB, FireWire, and Thunderbolt are primary data exfiltration and DMA attack vectors.",
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
        ],
    );
    append_indicator_group(
        &mut rows,
        "NETWORK AUDITING",
        "Controls that enable traffic accounting for anomaly detection \
         and forensic reconstruction. Without these, network audit logs \
         lack the volume data needed to identify exfiltration.",
        snap,
        &[(IndicatorId::NfConntrackAcct, "nf_conntrack acct")],
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
    // `ModulesDisabled` is here rather than CRYPTOGRAPHIC POSTURE because its
    // threat model is boot-time kernel surface freeze — it prevents rootkits
    // and SELinux bypass by blocking module loading after boot, which is a
    // tamper-resistance control, not a cryptographic primitive.
    let boot_indicators: &[(IndicatorId, &str)] = &[
        (IndicatorId::Lockdown, "lockdown"),
        (IndicatorId::KexecLoadDisabled, "kexec_load_disabled"),
        (IndicatorId::ModuleSigEnforce, "module.sig_enforce"),
        (IndicatorId::ModulesDisabled, "modules_disabled"),
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
            // Two-space indent — membership in BOOT INTEGRITY group.
            // Use IndicatorRow so the description aligns with the other
            // boot-integrity indicators. Description is the same as for
            // IndicatorId::Lockdown since this is the same semantic check.
            Some(DataRow::indicator_row(
                "  lockdown (header)",
                lv,
                lookup(IndicatorId::Lockdown).map_or("", |d| d.description),
                lh,
            ))
        }
    };

    if !group_rows.is_empty() || fallback.is_some() {
        rows.push(DataRow::group_title(i18n::tr("BOOT INTEGRITY")));
        // Description row: empty key → italic word-wrapped text in data_panel.
        rows.push(DataRow::key_value(
            "",
            i18n::tr(
                "Verifies the kernel loaded in a tamper-resistant state \
                 and cannot be silently replaced at runtime.",
            ),
            StyleHint::Dim,
        ));
        // Blank line between description and first indicator for visual breathing room.
        rows.push(DataRow::separator());
        rows.extend(group_rows);
        if let Some(r) = fallback {
            rows.push(r);
        }
        rows.push(DataRow::separator());
    }
}

/// Build display rows for a named group of posture indicators.
///
/// For each `(IndicatorId, label)` pair, looks up the `IndicatorReport` in the
/// snapshot. Indicators with a readable `live_value` are rendered as
/// `DataRow::IndicatorRow` entries styled by their hardening outcome. Indicators
/// with no live value (`live_value: None`) are silently skipped — the caller
/// is responsible for deciding whether to omit the group entirely.
///
/// Each indicator is followed by a dim description explaining what it controls
/// and why the operator should care. When the indicator is **not** hardened
/// (`meets_desired = Some(false)`), a `[ Recommended: <value> ]` line is
/// appended so operators know the target setting without a reference guide.
///
/// Returns an empty `Vec` when no indicator in the group has a readable value,
/// allowing the caller to suppress the group header.
///
/// NIST SP 800-53 SI-11: degraded signals are skipped, not fabricated.
/// NIST SP 800-53 SA-5: inline documentation reduces operator reliance on
/// external reference guides.
/// NIST SP 800-53 CM-6: remediation guidance accompanies each failing setting.
fn indicator_group_rows(
    snap: &PostureSnapshot,
    signals: &[(IndicatorId, &str)],
) -> Vec<DataRow> {
    let mut rows: Vec<DataRow> = Vec::new();
    for (id, label) in signals {
        let Some(report) = snap.get(*id) else {
            continue;
        };
        let Some(live) = &report.live_value else {
            continue;
        };
        let hint = meets_desired_hint(report.meets_desired);
        // Prefix the value with a ✓ / ✗ / ? symbol so the hardening
        // assessment is visible without relying solely on color. This
        // ensures the indicator is readable in NO_COLOR mode (WCAG 1.4.1).
        //   ✓ (U+2713) — meets hardened baseline
        //   ✗ (U+2717) — does not meet hardened baseline
        //   ? (U+003F) — no assessment (unreadable or custom type)
        let raw_value = annotate_live_value(*id, live);
        let display_value = match report.meets_desired {
            Some(true) => format!("\u{2713} {raw_value}"),
            Some(false) => format!("\u{2717} {raw_value}"),
            None => format!("? {raw_value}"),
        };
        // Recommendation: shown only for unhardened (red) indicators.
        // Green indicators pass `None` — no remediation guidance needed.
        let recommendation = if report.meets_desired == Some(false) {
            lookup(*id).and_then(|d| d.recommended)
        } else {
            None
        };

        // Contradiction: carry the kind to the render layer so it applies
        // the correct style and ⚠ symbol.
        //
        // NIST SP 800-53 CA-7 — contradiction kind is a typed finding;
        // surfacing it per-indicator lets the assessor pinpoint exactly
        // which setting has a drift event without scanning a separate list.
        let contradiction: Option<ContradictionKind> = report.contradiction;

        // Configured-value line: shown when a persisted configured value exists.
        //
        // NIST SP 800-53 CM-6 — source attribution for the configured value
        // allows the operator to locate and correct the config file.
        let configured_line: Option<String> =
            report.configured_value.as_ref().map(
                |ConfiguredValue {
                     raw,
                     source_file,
                 }| {
                    format!("Configured: {raw} (from {source_file})")
                },
            );

        rows.push(DataRow::indicator_row_full(
            format!("  {label}"),
            display_value,
            lookup(*id).map_or("", |d| d.description),
            recommendation,
            contradiction,
            configured_line,
            hint,
        ));
    }
    rows
}

/// Append a named indicator group to the row list.
///
/// Inserts a one-line plain-language description below the group title so
/// operators understand the purpose of each group without a reference guide.
/// Skips the group header and separator entirely when no indicator in `signals`
/// has a readable live value — an empty group is not shown.
///
/// NIST SP 800-53 CM-6 — configuration settings are labelled with their
/// security purpose so assessors can evaluate them without external context.
fn append_indicator_group(
    rows: &mut Vec<DataRow>,
    title: &'static str,
    description: &'static str,
    snap: &PostureSnapshot,
    signals: &[(IndicatorId, &str)],
) {
    let group_rows = indicator_group_rows(snap, signals);
    if !group_rows.is_empty() {
        rows.push(DataRow::group_title(i18n::tr(title)));
        // Description row: empty key → italic word-wrapped text in data_panel.
        rows.push(DataRow::key_value(
            "",
            i18n::tr(description),
            StyleHint::Dim,
        ));
        // Blank line between description and first indicator for visual breathing room.
        rows.push(DataRow::separator());
        rows.extend(group_rows);
        rows.push(DataRow::separator());
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
/// - `Enabled(s)` → value string + `StyleHint::TrustGreen`
/// - `Disabled(s)` → value string + `StyleHint::TrustYellow`
/// - `Unavailable` → `"unavailable"` + `StyleHint::Dim`
fn indicator_to_display(value: &IndicatorValue) -> (String, StyleHint) {
    match value {
        IndicatorValue::Enabled(s) => (s.clone(), StyleHint::TrustGreen),
        IndicatorValue::Disabled(s) => (s.clone(), StyleHint::TrustYellow),
        IndicatorValue::Unavailable => {
            ("unavailable".to_owned(), StyleHint::Dim)
        }
    }
}

/// Encode a byte slice as a lowercase hexadecimal string.
///
/// Uses `write!` with pre-allocated capacity to avoid the per-byte allocation
/// that `format!("{b:02x}")` + `.collect::<String>()` would incur.
fn bytes_to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut hex = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        // write! to a String is infallible; the error arm is unreachable.
        let _ = write!(hex, "{b:02x}");
    }
    hex
}

/// Pipeline execution order for evidence group display.
///
/// Groups are emitted in this fixed order so assessors follow the trust-elevation
/// narrative: kernel runtime checks first (highest trust), then filesystem
/// identity, configuration files, package database, and finally symlink targets.
/// Any `SourceKind` variant absent from the collected evidence is silently skipped.
///
/// NIST SP 800-53 AU-3 — evidence ordering supports coherent audit trail review.
const PIPELINE_ORDER: &[SourceKind] = &[
    SourceKind::Procfs,
    SourceKind::SysfsNode,
    SourceKind::StatfsResult,
    SourceKind::RegularFile,
    SourceKind::PackageDb,
    SourceKind::SymlinkTarget,
];

/// Append evidence records grouped by source kind to the row list.
///
/// Records are grouped by `SourceKind` and emitted in pipeline execution order
/// (kernel runtime → kernel attributes → filesystem identity → config files →
/// package database → symlink targets) so assessors follow the trust-elevation
/// narrative without needing to reorder rows mentally.
///
/// The `TableHeader` is emitted once at the top of the entire evidence section,
/// not repeated for every group. Each group is introduced by a `GroupTitle`
/// followed by `TableRow` entries. Groups are separated by a blank `Separator`.
///
/// When a T4 integrity record carries a SHA-256 digest (`EvidenceRecord::sha256`)
/// or a package-database reference digest (`EvidenceRecord::pkg_digest`), a
/// supplementary `KeyValue` row is appended immediately below the evidence row
/// so an assessor recording a T4 finding has the digest reference inline.
///
/// No raw kernel values or security-label data appear in the display strings
/// (NIST SP 800-53 SI-12). Evidence type labels are plain English
/// (e.g., `"Kernel runtime (/proc)"`) so operators do not need prior knowledge
/// of internal type names to interpret the evidence chain.
///
/// NIST SP 800-53 AU-3 — evidence rows are labelled and structured.
/// NIST SP 800-53 SI-7 — SHA-256 and package digests support integrity verification.
fn append_grouped_evidence(
    rows: &mut Vec<DataRow>,
    evidence: &[EvidenceRecord],
) {
    // Collect records into per-kind buckets. SourceKind does not implement Hash,
    // so we iterate PIPELINE_ORDER for collection — one O(n) pass per kind,
    // which is acceptable given the small number of kinds (6) and the typical
    // evidence bundle size (< 30 records).
    let kind_records: Vec<Vec<&EvidenceRecord>> = PIPELINE_ORDER
        .iter()
        .map(|kind| {
            evidence.iter().filter(|rec| &rec.source_kind == kind).collect()
        })
        .collect();

    // Single header row at the top of the entire evidence section.
    // Column headers appear once, not once per group, to reduce visual noise.
    rows.push(DataRow::table_header(
        i18n::tr("Evidence Type"),
        i18n::tr("Source"),
        i18n::tr("Verification"),
    ));
    // Blank line below the sticky column-label bar, before the first evidence
    // group title. The header is rendered as a fixed top bar by data_panel;
    // this separator provides visual breathing room between the bar and
    // the first GroupTitle row in the scrollable content below it.
    rows.push(DataRow::separator());

    let mut first_group = true;
    for (kind, records) in PIPELINE_ORDER.iter().zip(kind_records.iter()) {
        if records.is_empty() {
            continue;
        }
        if !first_group {
            rows.push(DataRow::separator());
        }
        first_group = false;

        let group_label = source_kind_label(kind);
        rows.push(DataRow::group_title(i18n::tr(group_label)));

        for rec in records {
            rows.push(DataRow::table_row(
                // col1: source kind (same as the group label — keeps the row
                // readable in isolation even when scrolled away from the title).
                i18n::tr(source_kind_label(&rec.source_kind)),
                // col2: full path — dynamic column width computed at render
                // time by data_panel::TableWidths::from_rows ensures the column
                // is wide enough without pre-truncation. data_panel still
                // clips at clip_pad time if the computed width is exceeded.
                // NIST SP 800-53 SI-12 — no sensitive data in display strings.
                rec.path_requested.clone(),
                // col3: structured verification outcome including magic label.
                evidence_verification_str(rec),
                evidence_style_hint(rec),
            ));

            // T4 integrity: surface the SHA-256 digest inline so an assessor
            // recording a T4 finding has the reference digest without leaving
            // the TUI. NIST SP 800-53 SI-7 / AU-3.
            if let Some(digest) = &rec.sha256 {
                let hex = bytes_to_hex(digest);
                rows.push(DataRow::key_value(
                    String::new(),
                    format!("sha256: {hex}"),
                    StyleHint::Dim,
                ));
            }

            // Package-database reference digest — shows the algorithm and
            // hex-encoded value stored in the RPM/dpkg database for this path.
            if let Some(pkg) = &rec.pkg_digest {
                let alg_label = match &pkg.algorithm {
                    umrs_platform::evidence::DigestAlgorithm::Sha256 => {
                        "sha256"
                    }
                    umrs_platform::evidence::DigestAlgorithm::Sha512 => {
                        "sha512"
                    }
                    umrs_platform::evidence::DigestAlgorithm::Md5 => {
                        "md5 (weak)"
                    }
                    umrs_platform::evidence::DigestAlgorithm::Unknown(s) => {
                        s.as_str()
                    }
                };
                let hex = bytes_to_hex(&pkg.value);
                rows.push(DataRow::key_value(
                    String::new(),
                    format!("pkg digest ({alg_label}): {hex}"),
                    StyleHint::Dim,
                ));
            }
        }
    }
}

/// Map an `EvidenceRecord` to a structured verification outcome string.
///
/// Returns a minimal, status-code-style string that an assessor can read and
/// act on. Unicode check (✓ U+2713) and cross (✗ U+2717) mark positive and
/// negative outcomes.
///
/// When the record was opened by file descriptor, the string includes the
/// filesystem magic label that was confirmed via `fstatfs(2)` provenance
/// verification, so an SP 800-53A assessor can identify the exact examination
/// method without consulting secondary documentation:
///
/// - `Procfs` → `"✓ ok (fd, PROC_MAGIC)"` — kernel confirmed `PROC_SUPER_MAGIC`
/// - `SysfsNode` → `"✓ ok (fd, SYS_MAGIC)"` — kernel confirmed `SYSFS_MAGIC`
/// - `StatfsResult` → `"✓ ok (fd, statfs)"` — filesystem identity via `statfs(2)`
/// - `RegularFile`, `PackageDb`, `SymlinkTarget` → `"✓ ok (fd)"` — fd-opened, no magic
///
/// Path-opened records (`opened_by_fd = false`) do not receive a magic label
/// because provenance verification is not performed on path-opened reads.
///
/// NIST SP 800-53 AU-3 — verification strings identify the outcome and method.
fn evidence_verification_str(rec: &EvidenceRecord) -> String {
    if rec.opened_by_fd {
        // Include the provenance verification method so an assessor knows
        // what filesystem magic check was performed, not just that an fd was used.
        let method_detail = match rec.source_kind {
            SourceKind::Procfs => "fd, PROC_MAGIC",
            SourceKind::SysfsNode => "fd, SYS_MAGIC",
            SourceKind::StatfsResult => "fd, statfs",
            SourceKind::RegularFile
            | SourceKind::PackageDb
            | SourceKind::SymlinkTarget => "fd",
        };
        if rec.parse_ok {
            format!("\u{2713} ok ({method_detail})")
        } else {
            format!("\u{2717} FAIL ({method_detail})")
        }
    } else if rec.parse_ok {
        "\u{2713} ok (path)".to_owned()
    } else {
        "\u{2717} FAIL (path)".to_owned()
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
            ..
        } => (
            // Shorter display form so the label fits without truncation.
            // The full contradiction description is emitted as a secondary
            // row in build_trust_summary_rows — see the call site.
            "Verified w/ Contradiction — T4 integrity + conflict".to_owned(),
            StyleHint::TrustRed,
        ),
    }
}

fn build_status(result: &DetectionResult) -> StatusMessage {
    match result.confidence.level() {
        TrustLevel::IntegrityAnchored => {
            StatusMessage::new(StatusLevel::Ok, i18n::tr("Integrity Anchored"))
        }
        TrustLevel::SubstrateAnchored => {
            StatusMessage::new(StatusLevel::Info, i18n::tr("Platform Verified"))
        }
        TrustLevel::EnvAnchored => StatusMessage::new(
            StatusLevel::Warn,
            format!(
                "{} — {}",
                i18n::tr(trust_level_label(TrustLevel::EnvAnchored)),
                i18n::tr(trust_level_description(TrustLevel::EnvAnchored))
            ),
        ),
        TrustLevel::KernelAnchored => StatusMessage::new(
            StatusLevel::Warn,
            format!(
                "{} — {}",
                i18n::tr(trust_level_label(TrustLevel::KernelAnchored)),
                i18n::tr(trust_level_description(TrustLevel::KernelAnchored))
            ),
        ),
        TrustLevel::Untrusted => StatusMessage::new(
            StatusLevel::Error,
            i18n::tr("Untrusted — no kernel anchor"),
        ),
    }
}

// ---------------------------------------------------------------------------
// In-TUI help text
// ---------------------------------------------------------------------------

/// Return contextual help text for the given tab index.
///
/// Help is presented via a `DialogState::info(...)` overlay when the operator
/// presses `?` or `F1`. Each tab gets a concise explanation covering what the
/// tab shows, how to interpret colors, and what action to take on findings.
///
/// Tab indices are stable by construction order in `OsDetectApp::from_result`:
/// - 0 → OS Information
/// - 1 → Kernel Security
/// - 2 → Trust / Evidence (always last — convention for all UMRS audit cards)
///
/// An unknown index falls back to a generic navigation hint.
///
/// NIST SP 800-53 SA-5 — inline system documentation reduces operator
/// reliance on external reference guides during assessment.
const fn help_text_for_tab(tab_index: usize) -> &'static str {
    match tab_index {
        0 => {
            "OS Information\n\
             Shows identity fields from /etc/os-release, platform identity,\n\
             and boot ID. These fields identify the system under assessment.\n\
             \n\
             Navigation: Tab / Shift-Tab = switch tabs  j/k = scroll\n\
             \n\
             Press Enter, Esc, or q to close this help."
        }
        1 => {
            "Kernel Security\n\
             Shows live kernel security posture from /proc and /sys.\n\
             \n\
             Symbols: \u{2713} = hardened  \u{2717} = not hardened  ? = unavailable\n\
             Colors:  green   = hardened  red  = not hardened  dim = unavailable\n\
             \n\
             Groups:\n\
             BOOT INTEGRITY      — lockdown, kexec, module sig, modules_disabled\n\
             CRYPTOGRAPHIC POSTURE — FIPS mode and entropy sources\n\
             KERNEL SELF-PROTECTION — ASLR, kptr, BPF, ptrace, perf\n\
             PROCESS ISOLATION   — user namespaces, sysrq, suid dumps\n\
             FILESYSTEM HARDENING — symlink and hardlink protections\n\
             MODULE RESTRICTIONS — blacklisted kernel modules\n\
             NETWORK AUDITING    — nf_conntrack accounting\n\
             \n\
             Red rows do not meet the hardened baseline — review each\n\
             indicator's description for risk context and remediation\n\
             priority before CUI processing.\n\
             \n\
             Contradictions (running kernel vs persisted config disagree):\n\
             \u{26a0} DRIFT         — config says hardened but kernel is not;\n\
                                  intended hardening is not active\n\
             \u{26a0} NOT PERSISTED — kernel is hardened now but config will not\n\
                                  keep it after reboot\n\
             \u{26a0} UNVERIFIABLE  — config exists but kernel value could not\n\
                                  be read to confirm\n\
             \n\
             Press Enter, Esc, or q to close this help."
        }
        2 => {
            "Trust / Evidence\n\
             \n\
             TOP (Summary — always visible):\n\
             Trust tier, downgrade reasons, and contradictions.\n\
             'No downgrade' means all trust checks passed.\n\
             Any contradiction requires manual review — it may indicate\n\
             tampering or a misconfigured system.\n\
             Contradictions here are OS detection contradictions: two\n\
             independent sources (e.g., /etc/os-release vs package DB)\n\
             reported conflicting values for the same fact. This is\n\
             distinct from kernel/config contradictions on the Kernel\n\
             Security tab.\n\
             \n\
             Trust tiers: T0=Untrusted  T1=Kernel Anchored  T2=Env Anchored\n\
             T3=Platform Verified  T4=Integrity Anchored\n\
             \n\
             BOTTOM (Evidence Chain — scrollable):\n\
             Actual files and kernel nodes read during detection.\n\
             Check mark = parsed successfully.\n\
             \n\
             Evidence types:\n\
               Kernel runtime (/proc)    — from /proc (procfs)\n\
               Kernel attributes (/sys)  — from /sys (sysfs)\n\
               Configuration file        — from /etc or filesystem\n\
               Package database          — from RPM/dpkg package DB\n\
               Symlink target            — symlink destination\n\
               Filesystem identity       — from statfs() syscall\n\
             \n\
             Verification codes:\n\
               fd          — file opened by file descriptor (safer than path)\n\
               PROC_MAGIC  — procfs verified via fstatfs() filesystem magic\n\
               SYS_MAGIC   — sysfs verified via fstatfs() filesystem magic\n\
               statfs      — filesystem identity confirmed via statfs()\n\
             These confirm the source is the real kernel filesystem,\n\
             not a crafted file at the same path.\n\
             \n\
             Press Enter, Esc, or q to close this help."
        }
        _ => {
            "Press Tab / Shift-Tab to switch tabs.\n\
             Press j/k or Up/Down to scroll.\n\
             Press q or Esc to quit.\n\
             \n\
             Press Enter, Esc, or q to close this help."
        }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    // ── i18n ─────────────────────────────────────────────────────────────
    // Initialize gettext catalog for the "umrs-ui" domain. Must be called
    // before any i18n::tr() calls. Falls back to the msgid if no catalog
    // is found — no error surfaced to the user.
    i18n::init("umrs-ui");

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

    // ── Detection ────────────────────────────────────────────────────────
    let app: OsDetectApp = match OsDetector::default().detect() {
        Ok(result) => {
            log::info!(
                "OS detection succeeded: {:?}",
                result.confidence.level()
            );
            // Populate os_name from the detection result now that it is available.
            ctx.os_name = os_name_from_release(result.os_release.as_ref());
            OsDetectApp::from_result(&result, &ctx, &snap)
        }
        Err(ref e) => {
            log::warn!("OS detection hard-gate failure: {e}");
            OsDetectApp::from_error(e, &ctx, &snap)
        }
    };

    // ── UI state ─────────────────────────────────────────────────────────
    let mut state = AuditCardState::new(app.tabs().len());
    let keymap = KeyMap::default();
    let theme = Theme::default();

    // ── Dialog state ──────────────────────────────────────────────────────
    // `None` = no dialog visible. `Some(d)` = dialog is rendered over the card.
    // The dialog is used exclusively for the informational help overlay (? / F1).
    // Presence in the Option is the sole visibility signal — no separate flag.
    // NIST SP 800-53 AC-2 — explicit lifecycle; no implicit global modal state.
    let mut help_dialog: Option<DialogState> = None;

    // ── Terminal setup ────────────────────────────────────────────────────
    let mut terminal = ratatui::init();

    // ── Event loop ───────────────────────────────────────────────────────
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_audit_card(f, f.area(), &app, &state, &ctx, &theme);
            render_dialog(f, f.area(), help_dialog.as_ref(), &theme);
        }) {
            log::error!("terminal draw error: {e}");
            break;
        }

        match event::poll(Duration::from_millis(250)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => {
                    if let Some(action) = keymap.lookup(&key) {
                        match action {
                            // Dialog-dismissal actions clear an open help dialog.
                            // Quit (Esc / q) also dismisses the dialog when one
                            // is open rather than quitting — the operator is
                            // clearly interacting with the dialog, not exiting.
                            Action::DialogConfirm
                            | Action::DialogCancel
                            | Action::Quit
                                if help_dialog.is_some() =>
                            {
                                help_dialog = None;
                            }
                            // ShowHelp opens the contextual help overlay for the
                            // current tab. If a dialog is already open, dismiss it.
                            Action::ShowHelp => {
                                if help_dialog.is_some() {
                                    help_dialog = None;
                                } else {
                                    let text =
                                        help_text_for_tab(state.active_tab);
                                    help_dialog = Some(DialogState::info(text));
                                }
                            }
                            // When a dialog is open, suppress all navigation
                            // actions — the operator must dismiss the dialog first.
                            _ if help_dialog.is_some() => {}
                            // Normal navigation.
                            _ => {
                                state.handle_action(&action);
                            }
                        }
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
