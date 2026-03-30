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
/// NIST SP 800-53 AC-12 — session lifecycle (Quit) must be cleanly handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Terminate the event loop and restore the terminal.
    Quit,

    /// Move to the next tab (wraps around).
    NextTab,

    /// Move to the previous tab (wraps around).
    PrevTab,

    /// Scroll the data area up one line.
    ScrollUp,

    /// Scroll the data area down one line.
    ScrollDown,

    /// Scroll the data area up one page.
    PageUp,

    /// Scroll the data area down one page.
    PageDown,

    /// Request a data refresh (re-run detection or reload source data).
    Refresh,

    /// Confirm the active dialog (typically Enter or Y).
    ///
    /// The calling binary maps this action to the appropriate key events and
    /// sets `DialogState::response = Some(true)` in its event loop.
    ///
    /// NIST SP 800-53 SI-10, AU-2 — explicit operator confirmation; callers
    /// must log the acknowledgement for `SecurityWarning` and `Confirm` dialogs.
    DialogConfirm,

    /// Cancel or dismiss the active dialog (typically Esc or N).
    ///
    /// The calling binary sets `DialogState::response = Some(false)` in its
    /// event loop. For single-button dialogs (`Info`, `Error`), this is
    /// equivalent to `DialogConfirm` — both dismiss the dialog.
    ///
    /// NIST SP 800-53 SI-10 — explicit operator dismissal.
    DialogCancel,

    /// Move focus between buttons in a two-button dialog (Tab / Left / Right).
    ///
    /// Only meaningful for `SecurityWarning` and `Confirm` modes. The calling
    /// binary calls `DialogFocus::toggle()` on `DialogState::focused` in
    /// response to this action.
    ///
    /// NIST SP 800-53 SC-5 — focus navigation ensures the operator makes a
    /// deliberate choice before confirming a security-affecting action.
    DialogToggleFocus,

    /// Open the in-TUI contextual help overlay for the current tab.
    ///
    /// Bound to `?` and `F1` by default. The calling binary is responsible
    /// for creating a `DialogState::info(...)` with context-appropriate help
    /// text and displaying it via `render_dialog`.
    ///
    /// NIST SP 800-53 SA-5 — system documentation is accessible from within
    /// the tool; operators do not need an external reference guide.
    ShowHelp,

    // -----------------------------------------------------------------------
    // ViewerApp actions
    // -----------------------------------------------------------------------
    /// Expand the currently selected tree node (Enter or Space in viewer mode).
    ///
    /// When a collapsed node is selected, `Expand` opens it to reveal its
    /// children. When an already-expanded node is selected, this action is
    /// a no-op. Leaf nodes ignore `Expand`.
    ///
    /// Not bound in the default keymap — callers add bindings via `KeyMap::bind()`.
    ///
    /// NIST SP 800-53 AC-3 — navigation is gated on node type; leaf nodes do
    /// not expose a false "expand" affordance that might mislead operators.
    Expand,

    /// Collapse the currently selected tree node.
    ///
    /// Hides all descendants of the selected node. When the node is already
    /// collapsed or is a leaf, this action is a no-op.
    ///
    /// Not bound in the default keymap.
    Collapse,

    /// Activate the search/filter input bar in a viewer.
    ///
    /// Bound to `/` by convention (vim-style search activation). The viewer
    /// state transitions to search mode; subsequent character input populates
    /// the filter query.
    ///
    /// Not bound in the default keymap.
    ///
    /// NIST SP 800-53 AU-3 — search operates over visible data only; filtered
    /// results remain scoped to what is already permitted for display.
    Search,

    /// Navigate up one level in the tree hierarchy (Backspace).
    ///
    /// Moves selection to the parent of the currently selected node. At the
    /// root, this action is a no-op.
    ///
    /// Not bound in the default keymap.
    Back,

    // -----------------------------------------------------------------------
    // ConfigApp actions
    // -----------------------------------------------------------------------
    /// Persist all in-progress edits to their backing store (Ctrl+S).
    ///
    /// The calling binary is responsible for emitting a structured journald
    /// record identifying the changed fields, the operator identity, and the
    /// timestamp. `Save` must only commit validated fields — the caller must
    /// verify all validation results are clean before acting on this action.
    ///
    /// Not bound in the default keymap — callers add bindings via `KeyMap::bind()`.
    ///
    /// NIST SP 800-53 CM-3 — configuration changes require explicit operator
    /// action and must be logged with before/after values.
    /// NIST SP 800-53 AU-2 — the save event is an auditable operation.
    /// NIST SP 800-53 SI-10 — only validated input may be committed.
    Save,

    /// Discard all in-progress edits and restore the last committed state (Ctrl+Z or Esc).
    ///
    /// The calling binary should present a `DialogState::confirm(...)` when
    /// dirty fields exist, so the operator must explicitly confirm discard.
    /// An operator who accidentally hits Discard on a clean form should see
    /// no confirmation — the form is already at the committed state.
    ///
    /// Not bound in the default keymap.
    ///
    /// NIST SP 800-53 CM-3 — discard must be an explicit, confirmed action
    /// when unsaved edits are present.
    Discard,

    /// Enter or exit edit mode for the focused field (Enter on a field).
    ///
    /// When a field is focused but not in edit mode, `ToggleEdit` activates
    /// the in-place editor. When the field is already being edited,
    /// `ToggleEdit` commits the current buffer (equivalent to pressing Enter
    /// to accept the edited value).
    ///
    /// Not bound in the default keymap.
    ///
    /// NIST SP 800-53 SI-10 — field-level edit mode ensures input validation
    /// is applied at the moment the operator commits each field value.
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
        map.insert(
            key(KeyCode::PageDown, KeyModifiers::NONE),
            Action::PageDown,
        );
        map.insert(key(KeyCode::PageUp, KeyModifiers::NONE), Action::PageUp);

        // Refresh
        map.insert(
            key(KeyCode::Char('r'), KeyModifiers::NONE),
            Action::Refresh,
        );

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
