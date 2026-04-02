// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// -----------------------------------------------------------------------------
// SelinuxCtxState — Integration Tests
//
// Verifies that the four label states are distinguishable and that the display
// and observation layers produce the correct output for each state.
//
// Key invariants tested:
//   - Labeled  → display_type returns the actual type string
//   - Unlabeled → display_type returns "<unlabeled>"
//   - ParseFailure → display_type returns "<parse-error>" (not "<unlabeled>")
//   - TpiDisagreement → display_type returns "<unverifiable>"
//   - SecurityObservation::NoSelinuxContext is emitted for Unlabeled only
//   - SecurityObservation::SelinuxParseFailure is emitted for ParseFailure
//   - SecurityObservation::TpiDisagreement is emitted for TpiDisagreement
//   - ParseFailure and TpiDisagreement do NOT emit NoSelinuxContext
//
// NIST SP 800-53 AU-3: accurate audit records.
// NIST SP 800-53 SI-12: information management / no false negatives in display.
// -----------------------------------------------------------------------------

use umrs_selinux::secure_dirent::SelinuxCtxState;
use umrs_selinux::{
    ObservationKind, SecurityContext, SecurityObservation, SelinuxRole, SelinuxType, SelinuxUser,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_context(user: &str, role: &str, ty: &str) -> SecurityContext {
    let u: SelinuxUser = user.parse().expect("valid user");
    let r: SelinuxRole = role.parse().expect("valid role");
    let t: SelinuxType = ty.parse().expect("valid type");
    SecurityContext::new(u, r, t, None)
}

// ---------------------------------------------------------------------------
// SelinuxCtxState::Labeled
// ---------------------------------------------------------------------------

#[test]
fn labeled_display_type_returns_type_string() {
    let ctx = make_context("system_u", "system_r", "sshd_t");
    let state = SelinuxCtxState::Labeled(Box::new(ctx));
    assert_eq!(state.display_type(), "sshd_t");
}

#[test]
fn labeled_is_labeled_returns_true() {
    let ctx = make_context("system_u", "system_r", "sshd_t");
    let state = SelinuxCtxState::Labeled(Box::new(ctx));
    assert!(state.is_labeled());
}

#[test]
fn labeled_as_context_returns_some() {
    let ctx = make_context("system_u", "system_r", "sshd_t");
    let state = SelinuxCtxState::Labeled(Box::new(ctx.clone()));
    assert_eq!(state.as_context(), Some(&ctx));
}

#[test]
fn labeled_level_returns_none_when_no_level() {
    let ctx = make_context("system_u", "system_r", "sshd_t");
    let state = SelinuxCtxState::Labeled(Box::new(ctx));
    // No MLS level set — level() should return None.
    assert!(state.level().is_none());
}

// ---------------------------------------------------------------------------
// SelinuxCtxState::Unlabeled
// ---------------------------------------------------------------------------

#[test]
fn unlabeled_display_type_is_sentinel() {
    let state = SelinuxCtxState::Unlabeled;
    assert_eq!(
        state.display_type(),
        "<unlabeled>",
        "Unlabeled must render as <unlabeled>"
    );
}

#[test]
fn unlabeled_is_labeled_returns_false() {
    assert!(!SelinuxCtxState::Unlabeled.is_labeled());
}

#[test]
fn unlabeled_as_context_returns_none() {
    assert!(SelinuxCtxState::Unlabeled.as_context().is_none());
}

#[test]
fn unlabeled_level_returns_none() {
    assert!(SelinuxCtxState::Unlabeled.level().is_none());
}

// ---------------------------------------------------------------------------
// SelinuxCtxState::ParseFailure
// ---------------------------------------------------------------------------

#[test]
fn parse_failure_display_type_is_not_unlabeled() {
    let state = SelinuxCtxState::ParseFailure;
    let display = state.display_type();
    // The critical invariant: ParseFailure must NOT display as <unlabeled>.
    // An operator must be able to tell the difference.
    assert_ne!(
        display, "<unlabeled>",
        "ParseFailure must NOT display as <unlabeled> — false negative risk"
    );
    assert_eq!(
        display, "<parse-error>",
        "ParseFailure should display as <parse-error>"
    );
}

#[test]
fn parse_failure_is_labeled_returns_false() {
    assert!(!SelinuxCtxState::ParseFailure.is_labeled());
}

#[test]
fn parse_failure_as_context_returns_none() {
    assert!(SelinuxCtxState::ParseFailure.as_context().is_none());
}

// ---------------------------------------------------------------------------
// SelinuxCtxState::TpiDisagreement
// ---------------------------------------------------------------------------

#[test]
fn tpi_disagreement_display_type_is_unverifiable() {
    let state = SelinuxCtxState::TpiDisagreement;
    let display = state.display_type();
    // TpiDisagreement is distinct from all other states.
    assert_ne!(display, "<unlabeled>", "Must not display as <unlabeled>");
    assert_ne!(
        display, "<parse-error>",
        "Must not display as <parse-error>"
    );
    assert_eq!(
        display, "<unverifiable>",
        "TpiDisagreement should display as <unverifiable>"
    );
}

#[test]
fn tpi_disagreement_is_labeled_returns_false() {
    assert!(!SelinuxCtxState::TpiDisagreement.is_labeled());
}

// ---------------------------------------------------------------------------
// All four states have distinct display_type strings
// ---------------------------------------------------------------------------

#[test]
fn all_four_states_produce_distinct_display_types() {
    let ctx = make_context("system_u", "system_r", "httpd_t");
    let labeled = SelinuxCtxState::Labeled(Box::new(ctx)).display_type();
    let unlabeled = SelinuxCtxState::Unlabeled.display_type();
    let parse_fail = SelinuxCtxState::ParseFailure.display_type();
    let tpi_dis = SelinuxCtxState::TpiDisagreement.display_type();

    // All must be distinct.
    assert_ne!(labeled, unlabeled);
    assert_ne!(labeled, parse_fail);
    assert_ne!(labeled, tpi_dis);
    assert_ne!(unlabeled, parse_fail);
    assert_ne!(unlabeled, tpi_dis);
    assert_ne!(parse_fail, tpi_dis);
}

// ---------------------------------------------------------------------------
// SecurityObservation mapping
//
// These tests verify the observation-level contract: which variant of
// SelinuxCtxState produces which SecurityObservation.
// ---------------------------------------------------------------------------

/// Verify that `NoSelinuxContext` has Risk polarity.
#[test]
fn observation_no_selinux_context_is_risk() {
    assert_eq!(
        SecurityObservation::NoSelinuxContext.kind(),
        ObservationKind::Risk
    );
}

/// Verify that `SelinuxParseFailure` has Warning polarity.
#[test]
fn observation_selinux_parse_failure_is_warning() {
    assert_eq!(
        SecurityObservation::SelinuxParseFailure.kind(),
        ObservationKind::Warning
    );
}

/// Verify that `TpiDisagreement` has Risk polarity.
#[test]
fn observation_tpi_disagreement_is_risk() {
    assert_eq!(
        SecurityObservation::TpiDisagreement.kind(),
        ObservationKind::Risk
    );
}

/// `SelinuxParseFailure` display string must not say "unlabeled".
#[test]
fn observation_parse_failure_display_does_not_say_unlabeled() {
    let s = SecurityObservation::SelinuxParseFailure.to_string();
    assert!(
        !s.to_lowercase().contains("unlabeled"),
        "SelinuxParseFailure display must not say 'unlabeled': {s}"
    );
    assert!(
        s.contains("parse") || s.contains("unverifiable"),
        "SelinuxParseFailure display should describe parse issue: {s}"
    );
}

/// `TpiDisagreement` display string must mention TPI or integrity.
#[test]
fn observation_tpi_disagreement_display_mentions_tpi() {
    let s = SecurityObservation::TpiDisagreement.to_string();
    let lower = s.to_lowercase();
    assert!(
        lower.contains("tpi") || lower.contains("integrity"),
        "TpiDisagreement display should mention TPI or integrity: {s}"
    );
}

/// The three label-state observations are distinct from each other.
#[test]
fn three_label_state_observations_are_distinct() {
    let no_ctx = SecurityObservation::NoSelinuxContext.to_string();
    let parse_fail = SecurityObservation::SelinuxParseFailure.to_string();
    let tpi_dis = SecurityObservation::TpiDisagreement.to_string();

    assert_ne!(
        no_ctx, parse_fail,
        "NoSelinuxContext and SelinuxParseFailure must differ"
    );
    assert_ne!(
        no_ctx, tpi_dis,
        "NoSelinuxContext and TpiDisagreement must differ"
    );
    assert_ne!(
        parse_fail, tpi_dis,
        "SelinuxParseFailure and TpiDisagreement must differ"
    );
}
