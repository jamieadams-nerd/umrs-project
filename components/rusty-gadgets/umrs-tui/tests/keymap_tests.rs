// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for [`KeyMap`] default bindings and custom bind support.
//!
//! Verifies that every documented default key event maps to the expected
//! [`Action`], that unbound keys return `None`, and that `bind()` correctly
//! overrides both new and existing mappings.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use umrs_tui::keymap::{Action, KeyMap};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

const fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent::new(code, modifiers)
}

// ---------------------------------------------------------------------------
// Default bindings — Quit
// ---------------------------------------------------------------------------

#[test]
fn q_maps_to_quit() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Char('q'), KeyModifiers::NONE);
    assert_eq!(km.lookup(&ev), Some(Action::Quit), "'q' must map to Quit");
}

#[test]
fn esc_maps_to_quit() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Esc, KeyModifiers::NONE);
    assert_eq!(km.lookup(&ev), Some(Action::Quit), "Esc must map to Quit");
}

// ---------------------------------------------------------------------------
// Default bindings — NextTab
// ---------------------------------------------------------------------------

#[test]
fn tab_key_maps_to_next_tab() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Tab, KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::NextTab),
        "Tab must map to NextTab"
    );
}

#[test]
fn right_arrow_maps_to_next_tab() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Right, KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::NextTab),
        "Right arrow must map to NextTab"
    );
}

// ---------------------------------------------------------------------------
// Default bindings — PrevTab
// ---------------------------------------------------------------------------

#[test]
fn back_tab_with_shift_maps_to_prev_tab() {
    let km = KeyMap::new();
    let ev = key(KeyCode::BackTab, KeyModifiers::SHIFT);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::PrevTab),
        "Shift-Tab must map to PrevTab"
    );
}

#[test]
fn left_arrow_maps_to_prev_tab() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Left, KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::PrevTab),
        "Left arrow must map to PrevTab"
    );
}

// ---------------------------------------------------------------------------
// Default bindings — ScrollDown
// ---------------------------------------------------------------------------

#[test]
fn j_maps_to_scroll_down() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Char('j'), KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::ScrollDown),
        "'j' must map to ScrollDown"
    );
}

#[test]
fn down_arrow_maps_to_scroll_down() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Down, KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::ScrollDown),
        "Down arrow must map to ScrollDown"
    );
}

// ---------------------------------------------------------------------------
// Default bindings — ScrollUp
// ---------------------------------------------------------------------------

#[test]
fn k_maps_to_scroll_up() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Char('k'), KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::ScrollUp),
        "'k' must map to ScrollUp"
    );
}

#[test]
fn up_arrow_maps_to_scroll_up() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Up, KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::ScrollUp),
        "Up arrow must map to ScrollUp"
    );
}

// ---------------------------------------------------------------------------
// Default bindings — PageDown / PageUp
// ---------------------------------------------------------------------------

#[test]
fn page_down_key_maps_to_page_down() {
    let km = KeyMap::new();
    let ev = key(KeyCode::PageDown, KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::PageDown),
        "PageDown key must map to PageDown action"
    );
}

#[test]
fn page_up_key_maps_to_page_up() {
    let km = KeyMap::new();
    let ev = key(KeyCode::PageUp, KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::PageUp),
        "PageUp key must map to PageUp action"
    );
}

// ---------------------------------------------------------------------------
// Default bindings — Refresh
// ---------------------------------------------------------------------------

#[test]
fn r_maps_to_refresh() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Char('r'), KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::Refresh),
        "'r' must map to Refresh"
    );
}

// ---------------------------------------------------------------------------
// Unbound key
// ---------------------------------------------------------------------------

#[test]
fn unbound_key_returns_none() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Char('x'), KeyModifiers::NONE);
    assert_eq!(km.lookup(&ev), None, "unbound key 'x' must return None");
}

#[test]
fn unbound_key_with_ctrl_modifier_returns_none() {
    let km = KeyMap::new();
    // Ctrl-q is not in the default map (only bare 'q' is)
    let ev = key(KeyCode::Char('q'), KeyModifiers::CONTROL);
    assert_eq!(
        km.lookup(&ev),
        None,
        "Ctrl-q is distinct from bare 'q' and must return None"
    );
}

// ---------------------------------------------------------------------------
// Custom bind — new key
// ---------------------------------------------------------------------------

#[test]
fn custom_bind_new_key_is_retrievable() {
    let mut km = KeyMap::new();
    let ev = key(KeyCode::Char('x'), KeyModifiers::NONE);
    km.bind(ev, Action::Quit);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::Quit),
        "newly bound 'x' must return Quit"
    );
}

// ---------------------------------------------------------------------------
// Custom bind — override existing key
// ---------------------------------------------------------------------------

#[test]
fn custom_bind_overrides_existing_mapping() {
    let mut km = KeyMap::new();
    let ev = key(KeyCode::Char('q'), KeyModifiers::NONE);
    // Sanity: 'q' starts as Quit
    assert_eq!(km.lookup(&ev), Some(Action::Quit), "precondition");
    // Override
    km.bind(ev, Action::Refresh);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::Refresh),
        "bound 'q' to Refresh — lookup must return Refresh"
    );
}

// ---------------------------------------------------------------------------
// Default trait impl
// ---------------------------------------------------------------------------

#[test]
fn default_produces_same_bindings_as_new() {
    let km_new = KeyMap::new();
    let km_default = KeyMap::default();

    let ev_q = key(KeyCode::Char('q'), KeyModifiers::NONE);
    let ev_tab = key(KeyCode::Tab, KeyModifiers::NONE);
    let ev_r = key(KeyCode::Char('r'), KeyModifiers::NONE);

    assert_eq!(km_new.lookup(&ev_q), km_default.lookup(&ev_q));
    assert_eq!(km_new.lookup(&ev_tab), km_default.lookup(&ev_tab));
    assert_eq!(km_new.lookup(&ev_r), km_default.lookup(&ev_r));
}

// ---------------------------------------------------------------------------
// Default bindings — ShowHelp
// ---------------------------------------------------------------------------

#[test]
fn question_mark_maps_to_show_help() {
    let km = KeyMap::new();
    let ev = key(KeyCode::Char('?'), KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::ShowHelp),
        "'?' must map to ShowHelp"
    );
}

#[test]
fn f1_maps_to_show_help() {
    let km = KeyMap::new();
    let ev = key(KeyCode::F(1), KeyModifiers::NONE);
    assert_eq!(
        km.lookup(&ev),
        Some(Action::ShowHelp),
        "F1 must map to ShowHelp"
    );
}

// ---------------------------------------------------------------------------
// ShowHelp is a no-op in AuditCardState
// ---------------------------------------------------------------------------

#[test]
fn show_help_does_not_change_state() {
    use umrs_tui::app::AuditCardState;
    let mut state = AuditCardState::new(3);
    state.handle_action(&Action::ShowHelp);
    assert!(!state.should_quit, "ShowHelp must not quit");
    assert_eq!(
        state.active_tab, 0,
        "ShowHelp must not change the active tab"
    );
    assert_eq!(
        state.scroll_offset, 0,
        "ShowHelp must not change the scroll offset"
    );
}
