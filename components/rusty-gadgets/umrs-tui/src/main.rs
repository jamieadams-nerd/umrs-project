// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NIST 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
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
//! as an interactive ratatui audit card. Two tabs present the data:
//!
//! - **Tab 0 — OS Information**: `os-release` fields, substrate identity,
//!   boot ID.
//! - **Tab 1 — Trust / Evidence**: label trust classification, confidence
//!   tier, downgrade reasons, contradictions, evidence records.
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
use umrs_core::i18n;
use std::collections::BTreeMap;

use umrs_platform::detect::label_trust::LabelTrust;
use umrs_platform::detect::{DetectionError, DetectionResult, OsDetector};
use umrs_platform::evidence::{EvidenceRecord, SourceKind};
use umrs_platform::{Distro, OsFamily, TrustLevel};
use umrs_tui::app::{
    AuditCardApp, AuditCardState, DataRow, StatusLevel, StatusMessage,
    StyleHint, TabDef,
};
use umrs_tui::indicators::read_security_indicators;
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
/// NIST SP 800-53 CM-8, SI-7, AU-3.
struct OsDetectApp {
    tabs: Vec<TabDef>,
    os_info_rows: Vec<DataRow>,
    trust_rows: Vec<DataRow>,
    status: StatusMessage,
}

impl OsDetectApp {
    /// Build the app from a successful detection result.
    fn from_result(result: &DetectionResult) -> Self {
        let tabs = vec![
            TabDef::new("OS Information"),
            TabDef::new("Trust / Evidence"),
        ];

        let os_info_rows = build_os_info_rows(result);
        let trust_rows = build_trust_rows(result);
        let status = build_status(result);

        Self {
            tabs,
            os_info_rows,
            trust_rows,
            status,
        }
    }

    /// Build the app from a hard-gate detection failure.
    fn from_error(err: &DetectionError) -> Self {
        let tabs = vec![
            TabDef::new("OS Information"),
            TabDef::new("Trust / Evidence"),
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

        let status =
            StatusMessage::new(StatusLevel::Error, "Detection pipeline failed");

        Self {
            tabs,
            os_info_rows,
            trust_rows,
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
fn append_grouped_evidence(rows: &mut Vec<DataRow>, evidence: &[EvidenceRecord]) {
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
    let open_method = if rec.opened_by_fd { "fd" } else { "path" };
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

    // ── Detection ────────────────────────────────────────────────────────
    let app: OsDetectApp = match OsDetector::default().detect() {
        Ok(result) => {
            log::info!(
                "OS detection succeeded: {:?}",
                result.confidence.level()
            );
            OsDetectApp::from_result(&result)
        }
        Err(ref e) => {
            log::warn!("OS detection hard-gate failure: {e}");
            OsDetectApp::from_error(e)
        }
    };

    // ── UI state ─────────────────────────────────────────────────────────
    let mut state = AuditCardState::new(app.tabs().len());
    let keymap = KeyMap::default();
    let theme = Theme::default();
    let indicators = read_security_indicators();

    // ── Terminal setup ────────────────────────────────────────────────────
    let mut terminal = ratatui::init();

    // ── Event loop ───────────────────────────────────────────────────────
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_audit_card(f, f.area(), &app, &state, &indicators, &theme);
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
