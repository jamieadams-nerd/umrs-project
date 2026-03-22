// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for [`AuditCardState`] and [`Action`] state machine.
//!
//! Covers construction invariants, tab navigation (including wrapping),
//! scroll operations (including saturation at zero), and the reset_scroll
//! helper. All tests exercise pure logic — no terminal backend required.

use umrs_ui::app::AuditCardState;
use umrs_ui::keymap::Action;

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

#[test]
fn new_sets_active_tab_to_zero() {
    let state = AuditCardState::new(3);
    assert_eq!(state.active_tab, 0, "active_tab must start at 0");
}

#[test]
fn new_sets_scroll_offset_to_zero() {
    let state = AuditCardState::new(3);
    assert_eq!(state.scroll_offset, 0, "scroll_offset must start at 0");
}

#[test]
fn new_sets_should_quit_to_false() {
    let state = AuditCardState::new(3);
    assert!(!state.should_quit, "should_quit must start as false");
}

#[test]
fn new_with_zero_tab_count_clamps_to_one() {
    // tab_count is private, but we can verify correct behaviour by cycling
    // PrevTab from tab 0 — with clamp=1, it must stay at 0.
    let mut state = AuditCardState::new(0);
    state.handle_action(&Action::PrevTab);
    assert_eq!(
        state.active_tab, 0,
        "clamped tab_count=1 means PrevTab from 0 stays at 0"
    );
}

// ---------------------------------------------------------------------------
// Quit
// ---------------------------------------------------------------------------

#[test]
fn quit_action_sets_should_quit() {
    let mut state = AuditCardState::new(3);
    state.handle_action(&Action::Quit);
    assert!(state.should_quit, "Quit must set should_quit = true");
}

#[test]
fn quit_does_not_change_active_tab() {
    let mut state = AuditCardState::new(3);
    state.active_tab = 1;
    state.handle_action(&Action::Quit);
    assert_eq!(state.active_tab, 1, "Quit must not change active_tab");
}

// ---------------------------------------------------------------------------
// Tab navigation — NextTab
// ---------------------------------------------------------------------------

#[test]
fn next_tab_increments_active_tab() {
    let mut state = AuditCardState::new(3);
    state.handle_action(&Action::NextTab);
    assert_eq!(state.active_tab, 1);
}

#[test]
fn next_tab_wraps_from_last_to_zero() {
    let mut state = AuditCardState::new(3);
    // Advance to last tab
    state.handle_action(&Action::NextTab);
    state.handle_action(&Action::NextTab);
    assert_eq!(state.active_tab, 2, "sanity: should be at last tab");
    // One more NextTab must wrap
    state.handle_action(&Action::NextTab);
    assert_eq!(state.active_tab, 0, "NextTab must wrap from last tab to 0");
}

#[test]
fn next_tab_three_full_cycles_returns_to_zero() {
    let mut state = AuditCardState::new(3);
    for _ in 0..9 {
        state.handle_action(&Action::NextTab);
    }
    assert_eq!(
        state.active_tab, 0,
        "3 full cycles of 3 tabs must land back on tab 0"
    );
}

#[test]
fn next_tab_resets_scroll_offset() {
    let mut state = AuditCardState::new(3);
    state.scroll_offset = 7;
    state.handle_action(&Action::NextTab);
    assert_eq!(
        state.scroll_offset, 0,
        "NextTab must reset scroll_offset to 0"
    );
}

// ---------------------------------------------------------------------------
// Tab navigation — PrevTab
// ---------------------------------------------------------------------------

#[test]
fn prev_tab_from_zero_wraps_to_last() {
    let mut state = AuditCardState::new(3);
    state.handle_action(&Action::PrevTab);
    assert_eq!(
        state.active_tab, 2,
        "PrevTab from tab 0 must wrap to last tab (2)"
    );
}

#[test]
fn prev_tab_from_middle_decrements() {
    let mut state = AuditCardState::new(3);
    state.active_tab = 2;
    state.handle_action(&Action::PrevTab);
    assert_eq!(state.active_tab, 1, "PrevTab from tab 2 must go to tab 1");
}

#[test]
fn prev_tab_from_one_goes_to_zero() {
    let mut state = AuditCardState::new(3);
    state.active_tab = 1;
    state.handle_action(&Action::PrevTab);
    assert_eq!(state.active_tab, 0, "PrevTab from tab 1 must go to tab 0");
}

#[test]
fn prev_tab_resets_scroll_offset() {
    let mut state = AuditCardState::new(3);
    state.active_tab = 2;
    state.scroll_offset = 5;
    state.handle_action(&Action::PrevTab);
    assert_eq!(
        state.scroll_offset, 0,
        "PrevTab must reset scroll_offset to 0"
    );
}

// ---------------------------------------------------------------------------
// Scroll — ScrollDown / ScrollUp
// ---------------------------------------------------------------------------

#[test]
fn scroll_down_increments_offset_by_one() {
    let mut state = AuditCardState::new(1);
    state.handle_action(&Action::ScrollDown);
    assert_eq!(state.scroll_offset, 1);
}

#[test]
fn scroll_down_accumulates() {
    let mut state = AuditCardState::new(1);
    for _ in 0..5 {
        state.handle_action(&Action::ScrollDown);
    }
    assert_eq!(state.scroll_offset, 5);
}

#[test]
fn scroll_up_decrements_offset_by_one() {
    let mut state = AuditCardState::new(1);
    state.scroll_offset = 3;
    state.handle_action(&Action::ScrollUp);
    assert_eq!(state.scroll_offset, 2);
}

#[test]
fn scroll_up_at_zero_stays_at_zero() {
    let mut state = AuditCardState::new(1);
    assert_eq!(state.scroll_offset, 0, "precondition");
    state.handle_action(&Action::ScrollUp);
    assert_eq!(
        state.scroll_offset, 0,
        "ScrollUp at zero must saturate — must not underflow"
    );
}

// ---------------------------------------------------------------------------
// Scroll — PageDown / PageUp
// ---------------------------------------------------------------------------

#[test]
fn page_down_increments_offset_by_ten() {
    let mut state = AuditCardState::new(1);
    state.handle_action(&Action::PageDown);
    assert_eq!(state.scroll_offset, 10);
}

#[test]
fn page_up_decrements_offset_by_ten() {
    let mut state = AuditCardState::new(1);
    state.scroll_offset = 15;
    state.handle_action(&Action::PageUp);
    assert_eq!(state.scroll_offset, 5);
}

#[test]
fn page_up_from_less_than_ten_saturates_at_zero() {
    let mut state = AuditCardState::new(1);
    state.scroll_offset = 3;
    state.handle_action(&Action::PageUp);
    assert_eq!(
        state.scroll_offset, 0,
        "PageUp must saturate at 0 when offset < 10"
    );
}

#[test]
fn page_up_at_zero_stays_at_zero() {
    let mut state = AuditCardState::new(1);
    state.handle_action(&Action::PageUp);
    assert_eq!(state.scroll_offset, 0, "PageUp at zero must not underflow");
}

// ---------------------------------------------------------------------------
// Refresh
// ---------------------------------------------------------------------------

#[test]
fn refresh_does_not_change_active_tab() {
    let mut state = AuditCardState::new(3);
    state.active_tab = 2;
    state.handle_action(&Action::Refresh);
    assert_eq!(state.active_tab, 2, "Refresh must not change active_tab");
}

#[test]
fn refresh_does_not_change_scroll_offset() {
    let mut state = AuditCardState::new(3);
    state.scroll_offset = 4;
    state.handle_action(&Action::Refresh);
    assert_eq!(
        state.scroll_offset, 4,
        "Refresh must not change scroll_offset"
    );
}

#[test]
fn refresh_does_not_set_should_quit() {
    let mut state = AuditCardState::new(3);
    state.handle_action(&Action::Refresh);
    assert!(!state.should_quit, "Refresh must not set should_quit");
}

// ---------------------------------------------------------------------------
// reset_scroll
// ---------------------------------------------------------------------------

#[test]
fn reset_scroll_sets_offset_to_zero() {
    let mut state = AuditCardState::new(1);
    state.scroll_offset = 42;
    state.reset_scroll();
    assert_eq!(
        state.scroll_offset, 0,
        "reset_scroll must zero scroll_offset"
    );
}

#[test]
fn reset_scroll_when_already_zero_is_idempotent() {
    let mut state = AuditCardState::new(1);
    state.reset_scroll();
    assert_eq!(state.scroll_offset, 0);
}
