// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Package Substrate Phase
//!
//! Soft-gate phase that probes available package substrates (RPM, dpkg) to
//! derive an independent platform identity claim. On success, upgrades
//! confidence to `TrustLevel::SubstrateAnchored` (T3).
//!
//! ## Steps performed
//!
//! 1. Dispatch each registered `PackageProbe` in priority order (RPM first,
//!    dpkg second). The first probe that returns `parse_ok = true` and
//!    `identity.is_some()` is selected. All probe results are recorded in
//!    `evidence` regardless of success.
//!
//! 2. Before asserting T3, verify that SELinux enforce mode is active via
//!    `SecureReader::<SelinuxEnforce>::new().read()`. This implements the
//!    Biba integrity pre-check (RAG Finding 5): a system where SELinux is in
//!    permissive mode cannot fully constrain subject/object interactions,
//!    which reduces the authority of the package DB as an integrity anchor.
//!    If SELinux is not in enforce mode, the substrate identity is still
//!    returned, but confidence is downgraded by one tier and the deviation
//!    is recorded.
//!
//! 3. Verify the substrate identity meets the T3 threshold (`facts_count >= 2`).
//!    If it does not, confidence is not upgraded.
//!
//! ## Return value
//!
//! Returns `(Option<SubstrateIdentity>, Option<Box<dyn PackageProbe>>)`:
//! - The identity is `Some` if a probe succeeded and the T3 threshold was met.
//! - The probe handle is `Some` if a working probe was selected; it is passed
//!   to subsequent phases (`file_ownership`, `integrity_check`) so they can
//!   query the same probe without re-opening the DB.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Information System Component Inventory — the
//!   package substrate is the most authoritative non-kernel identity source.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — probing the
//!   package DB derives identity from the same record that tracks installed
//!   software provenance.
//! - **NIST SP 800-53 SI-3, SI-7**: SELinux enforce check (Biba pre-check)
//!   ensures MAC controls are active before T3 is asserted.
//! - **NSA RTB RAIN**: T3 is only asserted when ≥2 independent facts are
//!   corroborated and SELinux enforce is confirmed.

use crate::confidence::{ConfidenceModel, TrustLevel};
use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};
use crate::kattrs::{SecureReader, SelinuxEnforce, StaticSource as _};
use crate::os_identity::SubstrateIdentity;

use super::substrate::{DpkgProbe, PackageProbe, RpmProbe};

// ===========================================================================
// Phase entry point
// ===========================================================================

/// Run the package substrate phase.
///
/// Returns the selected substrate identity and probe handle.
///
/// NIST SP 800-53 CM-8, SA-12, SI-3, SI-7. NSA RTB RAIN.
pub(super) fn run(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> (Option<SubstrateIdentity>, Option<Box<dyn PackageProbe>>) {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = run_inner(evidence, confidence);

    #[cfg(debug_assertions)]
    log::debug!(
        "pkg_substrate: completed in {} µs",
        t0.elapsed().as_micros()
    );

    result
}

fn run_inner(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> (Option<SubstrateIdentity>, Option<Box<dyn PackageProbe>>) {
    // Build probe list in priority order: RPM first (primary target platform
    // is RHEL 10), dpkg second.
    let probes: Vec<Box<dyn PackageProbe>> = vec![
        Box::new(RpmProbe::new()),
        Box::new(DpkgProbe::new()),
    ];

    // Step 1: Dispatch probes; select the first successful one.
    let mut selected_identity: Option<SubstrateIdentity> = None;
    let mut selected_probe: Option<Box<dyn PackageProbe>> = None;

    for probe in probes {
        let result = probe.probe(evidence);

        if result.parse_ok {
            if let Some(identity) = result.identity {
                log::debug!(
                    "pkg_substrate: probe '{}' succeeded with {} facts",
                    result.probe_name,
                    identity.facts_count
                );
                selected_identity = Some(identity);
                selected_probe = Some(probe);
                break;
            }
        } else {
            log::debug!(
                "pkg_substrate: probe '{}' failed (parse_ok=false)",
                result.probe_name
            );
        }
    }

    let Some(identity) = selected_identity else {
        log::warn!("pkg_substrate: no probe succeeded — substrate identity unavailable");
        confidence.downgrade(
            TrustLevel::EnvAnchored,
            "no package substrate probe succeeded",
        );
        return (None, None);
    };

    // Step 2: T3 threshold check — ≥2 independent corroborating facts required.
    if !identity.meets_t3_threshold() {
        log::warn!(
            "pkg_substrate: identity has only {} fact(s) — T3 threshold not met (requires 2)",
            identity.facts_count
        );
        confidence.downgrade(
            TrustLevel::EnvAnchored,
            "substrate identity has fewer than 2 corroborating facts",
        );
        return (Some(identity), selected_probe);
    }

    // Step 3: Biba integrity pre-check — SELinux enforce mode verification.
    // (RAG Finding 5) T3 cannot be fully asserted on a permissive-mode system
    // because MAC controls do not constrain subject/object interactions,
    // reducing the authority of the package DB as an integrity anchor.
    let selinux_enforce_ok = check_selinux_enforce(evidence);

    if selinux_enforce_ok {
        confidence.upgrade(TrustLevel::SubstrateAnchored);
        log::debug!("pkg_substrate: confidence upgraded to SubstrateAnchored (T3)");
    } else {
        // SELinux not enforcing: substrate identity is still useful, but T3
        // cannot be fully asserted. Downgrade by one tier.
        log::warn!(
            "pkg_substrate: SELinux is not in enforce mode — \
             T3 degraded; confidence remains at EnvAnchored"
        );
        confidence.downgrade(
            TrustLevel::EnvAnchored,
            "SELinux enforce check failed — Biba integrity pre-check not satisfied",
        );
    }

    (Some(identity), selected_probe)
}

// ===========================================================================
// SELinux enforce check (Biba pre-check)
// ===========================================================================

/// Verify that SELinux is in enforce mode.
///
/// Returns `true` if enforce mode is confirmed; `false` if permissive,
/// disabled, or the read failed. Result is recorded in evidence.
///
/// NIST SP 800-53 SI-3, SI-7 — MAC enforcement.
/// RAG Finding 5: Biba integrity pre-check before asserting T3.
fn check_selinux_enforce(evidence: &mut EvidenceBundle) -> bool {
    match SecureReader::<SelinuxEnforce>::new().read() {
        Ok(state) => {
            use crate::kattrs::EnforceState;
            let enforcing = state == EnforceState::Enforcing;
            if enforcing {
                log::debug!("pkg_substrate: SELinux enforce confirmed (Biba pre-check passed)");
            } else {
                log::warn!(
                    "pkg_substrate: SELinux is not enforcing — Biba pre-check failed"
                );
            }
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::SysfsNode,
                opened_by_fd: true,
                path_requested: SelinuxEnforce::PATH.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: true,
                notes: vec![format!(
                    "selinux_enforce={:?}; biba_check={enforcing}",
                    state
                )],
            });
            enforcing
        }
        Err(e) => {
            log::warn!("pkg_substrate: could not read SELinux enforce state: {e}");
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::SysfsNode,
                opened_by_fd: true,
                path_requested: SelinuxEnforce::PATH.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["SELinux enforce read failed".to_owned()],
            });
            false
        }
    }
}
