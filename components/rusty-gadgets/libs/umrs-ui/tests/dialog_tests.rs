// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for the dialog module.
//!
//! Verifies the lifecycle, focus semantics, and mode invariants of
//! [`DialogState`], [`DialogMode`], and [`DialogFocus`]. No terminal
//! backend is required — these are pure state tests.

use umrs_ui::dialog::{DialogFocus, DialogMode, DialogState};

// ---------------------------------------------------------------------------
// dialog_state_info_starts_pending
// ---------------------------------------------------------------------------

/// `DialogState::info()` constructs a dialog with `response == None`.
///
/// Verifies the pending-response invariant: a newly created info dialog
/// has no response until the operator dismisses it.
#[test]
fn dialog_state_info_starts_pending() {
    let d = DialogState::info("System check complete.");
    assert!(
        d.response.is_none(),
        "info dialog response must start as None (pending)"
    );
    assert_eq!(d.mode, DialogMode::Info);
    assert_eq!(
        d.message, "System check complete.",
        "message must be stored verbatim"
    );
}

// ---------------------------------------------------------------------------
// dialog_mode_variants_are_distinct
// ---------------------------------------------------------------------------

/// All four `DialogMode` variants compare unequal to each other.
///
/// This confirms that mode checks in the render path (border color,
/// button labels) will never conflate different dialog kinds.
#[test]
fn dialog_mode_variants_are_distinct() {
    let modes =
        [DialogMode::Info, DialogMode::Error, DialogMode::SecurityWarning, DialogMode::Confirm];
    for (i, a) in modes.iter().enumerate() {
        for (j, b) in modes.iter().enumerate() {
            if i == j {
                assert_eq!(a, b, "mode[{i}] must equal itself");
            } else {
                assert_ne!(a, b, "mode[{i}] must differ from mode[{j}]");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// dialog_focus_toggle
// ---------------------------------------------------------------------------

/// `DialogFocus::toggle()` cycles Primary → Secondary → Primary.
///
/// This verifies the full cycle so that the Tab / arrow navigation in
/// two-button dialogs always returns to the starting state after two presses.
#[test]
fn dialog_focus_toggle() {
    let start = DialogFocus::Primary;
    let after_one = start.toggle();
    assert_eq!(
        after_one,
        DialogFocus::Secondary,
        "Primary.toggle() must return Secondary"
    );
    let after_two = after_one.toggle();
    assert_eq!(
        after_two,
        DialogFocus::Primary,
        "Secondary.toggle() must return Primary"
    );
}

// ---------------------------------------------------------------------------
// dialog_security_warning_defaults_to_cancel
// ---------------------------------------------------------------------------

/// `DialogState::security_warning()` sets default focus to `Secondary` (Cancel).
///
/// This is a mandatory fail-safe requirement (NIST SP 800-53 SC-5): a
/// reflexive Enter keypress on a DoD terminal must never confirm a
/// security-affecting action. The operator must actively move focus to
/// [OK] before pressing Enter.
#[test]
fn dialog_security_warning_defaults_to_cancel() {
    let d = DialogState::security_warning("SELinux policy reload requested.");
    assert_eq!(
        d.focused,
        DialogFocus::Secondary,
        "SecurityWarning dialog must default to Secondary (Cancel) focus \
         per fail-safe requirement NIST SP 800-53 SC-5"
    );
    assert_eq!(d.mode, DialogMode::SecurityWarning);
    assert!(d.response.is_none(), "response must start as None");
}

// ---------------------------------------------------------------------------
// dialog_confirm_defaults_to_cancel
// ---------------------------------------------------------------------------

/// `DialogState::confirm()` sets default focus to `Secondary` (No).
///
/// Mirrors the `SecurityWarning` fail-safe default: confirmation dialogs
/// also default to the dismissal button. The operator must actively choose
/// Yes / OK.
#[test]
fn dialog_confirm_defaults_to_cancel() {
    let d = DialogState::confirm("Reload the detection pipeline?");
    assert_eq!(
        d.focused,
        DialogFocus::Secondary,
        "Confirm dialog must default to Secondary (No) focus"
    );
    assert_eq!(d.mode, DialogMode::Confirm);
    assert!(d.response.is_none(), "response must start as None");
}

// ---------------------------------------------------------------------------
// dialog_info_has_primary_focus
// ---------------------------------------------------------------------------

/// `DialogState::info()` sets default focus to `Primary` (OK).
///
/// Info and Error dialogs have only one button. `Primary` is the logical
/// focus because it is the sole dismissal option — there is no Cancel.
#[test]
fn dialog_info_has_primary_focus() {
    let info = DialogState::info("Detection complete.");
    assert_eq!(
        info.focused,
        DialogFocus::Primary,
        "Info dialog must start with Primary focus (the sole button)"
    );

    let err = DialogState::error("Kernel attribute read failed.");
    assert_eq!(
        err.focused,
        DialogFocus::Primary,
        "Error dialog must start with Primary focus (the sole button)"
    );
}
