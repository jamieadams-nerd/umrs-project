// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # KeyMap — Keystroke to Action Mapping
//!
//! Decouples raw terminal key events from application logic. The [`KeyMap`]
//! struct maps [`crossterm::event::KeyEvent`] values to [`Action`] variants.
//!
//! ## Extending the Keymap
//!
//! Call `keymap.bind(key_event, action)` to add or override a mapping.
//! The defaults cover quit, tab navigation, and vertical scrolling.
//!
//! Extended actions for [`crate::viewer::ViewerApp`] (`Expand`, `Collapse`,
//! `Search`, `Back`) and [`crate::config::ConfigApp`] (`Save`, `Discard`,
//! `ToggleEdit`) are defined as variants in [`Action`] but are not bound
//! in the default keymap — callers add bindings with `KeyMap::bind()`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-12**: The quit action terminates the session cleanly —
//!   no half-written state is left behind.
//! - **NIST SP 800-53 SI-10**: Explicit operator input (`Save`, `Discard`,
//!   `ToggleEdit`) ensures configuration mutations are intentional and logged.

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// ---------------------------------------------------------------------------
// Action
// ---------------------------------------------------------------------------

/// User-visible actions triggered by keystrokes.
///
/// Each variant maps to a single logical operation in the audit card.
/// The keymap lookup returns `None` for unbound keys — callers silently
/// ignore unrecognized input.
///
/// ## Variants:
///
/// *Navigation*
///
/// - `Quit` — terminate the event loop and restore the terminal.
/// - `NextTab` — move to the next tab (wraps around).
/// - `PrevTab` — move to the previous tab (wraps around).
/// - `ScrollUp` — scroll the data area up one line.
/// - `ScrollDown` — scroll the data area down one line.
/// - `PageUp` — scroll the data area up one page.
/// - `PageDown` — scroll the data area down one page.
/// - `Refresh` — request a data refresh (re-run detection or reload source data).
///
/// *Dialog*
///
/// - `DialogConfirm` — confirm the active dialog (typically Enter or Y); the calling binary sets
///   `DialogState::response = Some(true)` in its event loop; callers must log the
///   acknowledgement for `SecurityWarning` and `Confirm` dialogs. NIST SP 800-53 SI-10, AU-2.
/// - `DialogCancel` — cancel or dismiss the active dialog (typically Esc or N); the calling
///   binary sets `DialogState::response = Some(false)`; for single-button dialogs, equivalent to
///   `DialogConfirm`. NIST SP 800-53 SI-10.
/// - `DialogToggleFocus` — move focus between buttons in a two-button dialog (Tab / Left /
///   Right); only meaningful for `SecurityWarning` and `Confirm` modes; the calling binary calls
///   `DialogFocus::toggle()`. NIST SP 800-53 SC-5.
/// - `ShowHelp` — open the in-TUI contextual help overlay for the current tab; bound to `?` and
///   `F1` by default; the calling binary creates a `DialogState::info(...)` with
///   context-appropriate help text. NIST SP 800-53 SA-5.
///
/// *ViewerApp actions (not bound in default keymap)*
///
/// - `Expand` — expand the currently selected tree node (Enter or Space); a no-op for
///   already-expanded nodes and leaf nodes. NIST SP 800-53 AC-3.
/// - `Collapse` — collapse the currently selected tree node; hides all descendants; no-op for
///   already-collapsed nodes and leaves.
/// - `Search` — activate the search/filter input bar in a viewer; bound to `/` by convention;
///   the viewer state transitions to search mode. NIST SP 800-53 AU-3.
/// - `Back` — navigate up one level in the tree hierarchy (Backspace); moves selection to the
///   parent; no-op at the root.
/// - `PanelSwitch` — switch focus between panels (Tree ↔ Detail in a viewer or equivalent);
///   typical binding is `Tab` / `Shift-Tab` overriding `NextTab`/`PrevTab`. NIST SP 800-53 AC-3.
///
/// *ConfigApp actions (not bound in default keymap)*
///
/// - `Save` — persist all in-progress edits to their backing store (Ctrl+S); callers must emit
///   a structured journald record and verify all validation results are clean.
///   NIST SP 800-53 CM-3, AU-2, SI-10.
/// - `Discard` — discard all in-progress edits and restore the last committed state (Ctrl+Z or
///   Esc); callers should present a confirmation dialog when dirty fields exist.
///   NIST SP 800-53 CM-3.
/// - `ToggleEdit` — enter or exit edit mode for the focused field (Enter on a field); when in
///   edit mode, commits the current buffer. NIST SP 800-53 SI-10.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-12**: session lifecycle (`Quit`) must be cleanly handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    // Navigation
    Quit,
    NextTab,
    PrevTab,
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    Refresh,

    // Dialog
    DialogConfirm,
    DialogCancel,
    DialogToggleFocus,
    ShowHelp,

    // ViewerApp actions
    Expand,
    Collapse,
    Search,
    Back,
    PanelSwitch,

    // ConfigApp actions
    Save,
    Discard,
    ToggleEdit,
}

// ---------------------------------------------------------------------------
// KeyMap
// ---------------------------------------------------------------------------

/// Maps terminal key events to [`Action`] variants.
///
/// Construct via `KeyMap::default()` for the standard bindings, then
/// use `bind()` to add or override mappings for a specific binary.
///
/// NIST SP 800-53 AC-12 — clean session termination on quit.
pub struct KeyMap {
    map: HashMap<KeyEvent, Action>,
}

impl KeyMap {
    /// Construct a `KeyMap` with the default bindings.
    ///
    /// Default bindings:
    ///
    /// | Key | Action |
    /// |---|---|
    /// | `q` / `Esc` | Quit |
    /// | `Tab` / `Right` | NextTab |
    /// | `Shift-Tab` / `Left` | PrevTab |
    /// | `j` / `Down` | ScrollDown |
    /// | `k` / `Up` | ScrollUp |
    /// | `PageDown` | PageDown |
    /// | `PageUp` | PageUp |
    /// | `r` | Refresh |
    /// | `Enter` | DialogConfirm |
    /// | `?` / `F1` | ShowHelp |
    #[must_use = "KeyMap must be used to process events; constructing and discarding it has no effect"]
    pub fn new() -> Self {
        let mut map = HashMap::new();

        // Quit
        map.insert(key(KeyCode::Char('q'), KeyModifiers::NONE), Action::Quit);
        map.insert(key(KeyCode::Esc, KeyModifiers::NONE), Action::Quit);

        // Tab navigation
        map.insert(key(KeyCode::Tab, KeyModifiers::NONE), Action::NextTab);
        map.insert(key(KeyCode::Right, KeyModifiers::NONE), Action::NextTab);
        map.insert(key(KeyCode::BackTab, KeyModifiers::SHIFT), Action::PrevTab);
        map.insert(key(KeyCode::Left, KeyModifiers::NONE), Action::PrevTab);

        // Vertical scroll
        map.insert(
            key(KeyCode::Char('j'), KeyModifiers::NONE),
            Action::ScrollDown,
        );
        map.insert(key(KeyCode::Down, KeyModifiers::NONE), Action::ScrollDown);
        map.insert(
            key(KeyCode::Char('k'), KeyModifiers::NONE),
            Action::ScrollUp,
        );
        map.insert(key(KeyCode::Up, KeyModifiers::NONE), Action::ScrollUp);
        map.insert(key(KeyCode::PageDown, KeyModifiers::NONE), Action::PageDown);
        map.insert(key(KeyCode::PageUp, KeyModifiers::NONE), Action::PageUp);

        // Refresh
        map.insert(key(KeyCode::Char('r'), KeyModifiers::NONE), Action::Refresh);

        // Dialog confirm — Enter dismisses an open dialog via the [OK] button.
        // The event loop maps DialogConfirm to dialog dismissal; when no dialog
        // is open this action reaches AuditCardState::handle_action which ignores it.
        map.insert(
            key(KeyCode::Enter, KeyModifiers::NONE),
            Action::DialogConfirm,
        );

        // Contextual help overlay
        map.insert(
            key(KeyCode::Char('?'), KeyModifiers::NONE),
            Action::ShowHelp,
        );
        map.insert(key(KeyCode::F(1), KeyModifiers::NONE), Action::ShowHelp);

        // ViewerApp — search activation (vim-style /)
        // Not bound by default in the AuditCardApp context but present in the
        // default map so callers do not need to re-bind it when constructing
        // a viewer.
        map.insert(key(KeyCode::Char('/'), KeyModifiers::NONE), Action::Search);

        // ViewerApp — navigate up in hierarchy (Backspace)
        map.insert(key(KeyCode::Backspace, KeyModifiers::NONE), Action::Back);

        // ViewerApp — expand / collapse (Enter = Expand, Ctrl+Space = Collapse)
        // Enter is already bound to DialogConfirm; viewer event loops must
        // disambiguate based on dialog state. Collapse is available for callers
        // to bind to a key of their choice.
        map.insert(key(KeyCode::Char(' '), KeyModifiers::NONE), Action::Expand);

        Self {
            map,
        }
    }

    /// Look up the action bound to a key event.
    ///
    /// Returns `None` for unrecognized keys. Callers should silently ignore
    /// `None` — unbound keys produce no effect.
    #[must_use = "the returned action must be applied to state"]
    pub fn lookup(&self, event: &KeyEvent) -> Option<Action> {
        self.map.get(event).copied()
    }

    /// Bind or override a key event to an action.
    ///
    /// Overwrites any existing binding for the same key event.
    pub fn bind(&mut self, event: KeyEvent, action: Action) {
        self.map.insert(event, action);
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Construct a [`KeyEvent`] from code and modifiers (no `unwrap`-free shorthand
/// exists in crossterm for this, so we centralise it here).
const fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent::new(code, modifiers)
}
