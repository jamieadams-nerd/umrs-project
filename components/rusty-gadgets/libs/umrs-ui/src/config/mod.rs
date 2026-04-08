// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Config — Interactive Configuration Editing Pattern
//!
//! Provides the [`ConfigApp`] trait and [`ConfigState`] for building
//! interactive configuration editor TUI tools within the UMRS platform.
//!
//! ## Intended Consumers
//!
//! - SELinux label assignment tools
//! - `setrans.conf` editing interface
//! - System security policy configuration (future)
//!
//! ## Usage Pattern
//!
//! 1. Implement [`ConfigApp`] on your data struct. Provide `tabs()`,
//!    `status()`, `card_title()`, `config_header()`, and `committed_values()`.
//! 2. Create a [`ConfigState`] with `ConfigState::new(tab_count)`.
//! 3. Populate `state.fields` with your [`fields::FieldDef`] instances.
//! 4. Call [`layout::render_config`] inside `terminal.draw(...)`.
//! 5. Feed [`crate::keymap::KeyMap`] events into `state.handle_action(...)`.
//! 6. When `handle_action` returns `ConfigStateEvent::Save`, call
//!    `app.commit(state)` to persist the changes and emit an audit record.
//!
//! ## Header Context
//!
//! The config header is **config-contextual**, not security-posture:
//! - Tool name
//! - Config file path or target resource
//! - Modified indicator (dirty flag)
//! - Validation status summary
//!
//! ## Save Gate
//!
//! The `Save` action is only permitted when all fields pass validation.
//! `ConfigState::can_save()` checks this; the layout reflects the gating
//! by showing/hiding the `^S: save` key hint.
//!
//! ## Audit Obligation
//!
//! The library has no logging dependency. Callers MUST emit a structured
//! journald record when a `Save` is committed, including: the tool name,
//! operator identity, timestamp, and before/after values for each changed
//! field. See `DiffEntry` for the authoritative source of before/after data.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-3**: Configuration change control — explicit
//!   operator action required for every save; diff view before commit.
//! - **NIST SP 800-53 SI-10**: Input validation — every field has a
//!   validator; values are never committed without passing validation.
//! - **NIST SP 800-53 AU-2**: The save event is an auditable operation;
//!   callers must log it.
//! - **NIST SP 800-53 AC-12**: Discard action cleanly restores prior state.
//! - **NSA RTB RAIN**: Non-bypassable save gate — `Save` is rejected at the
//!   state level when any field has a blocking validation result.

pub mod diff;
pub mod fields;
pub mod layout;

pub use layout::render_config;

use std::collections::HashMap;

use crate::app::{StatusMessage, TabDef};
use crate::keymap::Action;

use fields::{FieldDef, ValidationResult};

// ---------------------------------------------------------------------------
// ConfigHeaderContext
// ---------------------------------------------------------------------------

/// Context for the configuration editor header panel.
///
/// Config-contextual: identifies the tool, target resource, and current
/// validation state. Does not include kernel security posture indicators.
///
/// NIST SP 800-53 AU-3 — header fields ensure every rendered frame carries
/// sufficient identification for audit record dating and scope.
/// NIST SP 800-53 CM-3 — the `validation_summary` communicates whether the
/// current form state is committable.
#[derive(Debug, Clone)]
pub struct ConfigHeaderContext {
    /// Name of the configuration tool (e.g., `"SELinux Config Editor"`).
    pub tool_name: String,

    /// The path or identifier of the configuration target
    /// (e.g., `"/etc/selinux/config"` or `"setrans.conf"`).
    pub config_target: String,

    /// A one-line validation summary shown in the header
    /// (e.g., `"all fields valid"` or `"2 errors"`).
    ///
    /// Callers compute this from `ConfigState::validation_summary()` or
    /// provide their own format.
    pub validation_summary: String,
}

impl ConfigHeaderContext {
    /// Construct a `ConfigHeaderContext`.
    #[must_use = "ConfigHeaderContext must be returned from config_header(); discarding it leaves the header empty"]
    pub fn new(
        tool_name: impl Into<String>,
        config_target: impl Into<String>,
        validation_summary: impl Into<String>,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            config_target: config_target.into(),
            validation_summary: validation_summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// ConfigStateEvent
// ---------------------------------------------------------------------------

/// Events returned by `ConfigState::handle_action` that require caller action.
///
/// Most actions return `ConfigStateEvent::Redraw` (re-render needed) or
/// `ConfigStateEvent::None` (no change). `Save` and `DiscardConfirm` require
/// the caller to perform I/O or show a confirmation dialog.
///
/// NIST SP 800-53 CM-3 — save and discard are explicitly surfaced as events
/// so callers cannot accidentally miss them.
/// NIST SP 800-53 AU-2 — the `Save` event signals the caller to emit an
/// audit record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "ConfigStateEvent must be handled; discarding it may skip a required save or audit action"]
pub enum ConfigStateEvent {
    /// No state change occurred; no re-render needed.
    None,

    /// State changed; the caller should re-render.
    Redraw,

    /// The operator requested a save. The caller must: verify `state.can_save()`,
    /// perform I/O, emit an audit record, and call `state.mark_saved()`.
    ///
    /// NIST SP 800-53 CM-3 / AU-2.
    Save,

    /// The operator requested a discard while dirty fields exist. The caller
    /// should show a confirmation dialog (`DialogState::confirm(...)`) and
    /// call `state.discard_all()` if confirmed.
    ///
    /// When no dirty fields exist, `Discard` does not produce this event —
    /// the form is already at the committed state.
    ///
    /// NIST SP 800-53 CM-3.
    DiscardConfirm,

    /// The operator pressed Quit.
    ///
    /// If the form is dirty, the caller should show a confirmation dialog
    /// before setting `should_quit = true`.
    Quit,
}

// ---------------------------------------------------------------------------
// ConfigApp trait
// ---------------------------------------------------------------------------

/// Trait for interactive configuration editor TUI tools.
///
/// Implement this trait on your data struct to plug into the config layout.
/// The trait is object-safe — `render_config` accepts `&dyn ConfigApp`.
///
/// ## Invariants
///
/// - `tabs()` must return at least one entry.
/// - `config_header()` must return a context with a non-empty `tool_name`.
/// - `committed_values()` must return the last committed (saved) values for
///   all fields by label. Used by the diff view to compute before/after.
///
/// NIST SP 800-53 CM-3 — the trait requires `committed_values()` so the diff
/// view can always show what will change.
/// NIST SP 800-53 AU-2 — callers must emit audit records on every save event.
pub trait ConfigApp {
    /// The title displayed in the header border.
    fn card_title(&self) -> &str;

    /// The tab definitions for this config editor.
    fn tabs(&self) -> &[TabDef];

    /// The current status message for the status bar.
    fn status(&self) -> StatusMessage;

    /// The header context for the config header panel.
    ///
    /// Called once per frame. The `validation_summary` field should be
    /// computed from `ConfigState::validation_summary()` before passing to
    /// `render_config`.
    ///
    /// NIST SP 800-53 AU-3 — header context carries tool identity and target.
    fn config_header(&self) -> ConfigHeaderContext;

    /// Return the last committed (saved) values for all fields, keyed by
    /// field label.
    ///
    /// Used by the diff view to compute before/after comparisons. Values must
    /// match what was last written to the backing store. If a field has never
    /// been saved, its label should be absent from the map (treated as "new").
    ///
    /// NIST SP 800-53 CM-3 — before/after comparison requires the authoritative
    /// committed state from the backing store, not from state.
    fn committed_values(&self) -> HashMap<String, String>;
}

// ---------------------------------------------------------------------------
// ConfigState
// ---------------------------------------------------------------------------

/// Mutable UI state for a [`ConfigApp`] session.
///
/// Owns the field definitions, focus state, and dirty tracking. Separate from
/// the app data struct so the event loop can mutate state while holding an
/// immutable reference to the app.
///
/// NIST SP 800-53 CM-3 — dirty tracking and save-gating are enforced here,
/// not left to the caller.
/// NSA RTB RAIN — `can_save()` must return `true` before `Save` is emitted;
/// the state enforces this.
pub struct ConfigState {
    /// Configuration field definitions (ordered for display).
    ///
    /// Populate after constructing `ConfigState::new()`. Each `FieldDef`
    /// carries its current value, edit state, validation result, and dirty flag.
    pub fields: Vec<FieldDef>,

    /// Index of the currently focused field.
    pub focused_field: usize,

    /// Active tab index.
    pub active_tab: usize,

    /// Total number of tabs.
    tab_count: usize,

    /// Signal to the event loop that the application should terminate.
    pub should_quit: bool,
}

impl ConfigState {
    /// Construct a new `ConfigState` with no fields and `tab_count` tabs.
    ///
    /// Add fields to `state.fields` after construction.
    #[must_use = "ConfigState must be used in the event loop; constructing and discarding it has no effect"]
    pub fn new(tab_count: usize) -> Self {
        Self {
            fields: Vec::new(),
            focused_field: 0,
            active_tab: 0,
            tab_count: tab_count.max(1),
            should_quit: false,
        }
    }

    /// Return `true` if any field has unsaved changes.
    ///
    /// NIST SP 800-53 CM-3 — dirty state determines whether `Discard`
    /// requires confirmation and whether the dirty indicator is shown.
    #[must_use = "dirty state must be checked before allowing quit without confirmation"]
    pub fn is_dirty(&self) -> bool {
        self.fields.iter().any(|f| f.dirty)
    }

    /// Return `true` if all fields pass validation and the form may be saved.
    ///
    /// A field with `ValidationResult::Pending` blocks save because it has
    /// not been validated yet. Use `validate_all()` to run all validators
    /// before checking `can_save()`.
    ///
    /// NIST SP 800-53 SI-10, NSA RTB RAIN — save is gated on this method;
    /// callers must check it before acting on a `ConfigStateEvent::Save`.
    #[must_use = "save permission must be checked before performing I/O; discarding it bypasses the validation gate"]
    pub fn can_save(&self) -> bool {
        self.fields.iter().all(|f| !f.validation.blocks_save())
    }

    /// Run all field validators in place.
    ///
    /// Call before checking `can_save()` to ensure `Pending` fields are
    /// evaluated. Returns a validation summary string for display.
    ///
    /// NIST SP 800-53 SI-10 — all fields are validated before save.
    #[must_use = "summary string is used for display; discarding it hides validation results"]
    pub fn validate_all(&mut self) -> String {
        let mut errors = 0usize;
        let mut warnings = 0usize;
        for field in &mut self.fields {
            // validate_buffer stores its result in field.validation.
            // The returned reference is to that same field — we read the
            // result on the next line via field.validation directly.
            let _ = field.validate_buffer();
            match &field.validation {
                ValidationResult::Error(_) => errors = errors.saturating_add(1),
                ValidationResult::Warning(_) => {
                    warnings = warnings.saturating_add(1);
                }
                _ => {}
            }
        }
        if errors == 0 && warnings == 0 {
            "all fields valid".to_owned()
        } else if errors == 0 {
            format!("{warnings} warning(s)")
        } else {
            format!("{errors} error(s)")
        }
    }

    /// Return a validation summary string without running validators.
    ///
    /// Reads existing `ValidationResult` from each field. Use `validate_all()`
    /// to first run all validators.
    #[must_use = "summary is used for header display; discarding it leaves the header stale"]
    pub fn validation_summary(&self) -> String {
        let errors = self
            .fields
            .iter()
            .filter(|f| matches!(f.validation, ValidationResult::Error(_)))
            .count();
        let warnings = self
            .fields
            .iter()
            .filter(|f| matches!(f.validation, ValidationResult::Warning(_)))
            .count();
        if errors == 0 && warnings == 0 {
            "all fields valid".to_owned()
        } else if errors == 0 {
            format!("{warnings} warning(s)")
        } else {
            format!("{errors} error(s)")
        }
    }

    /// Mark all fields as clean (not dirty) after a successful save.
    ///
    /// Call this after the caller has persisted the field values to the
    /// backing store.
    ///
    /// NIST SP 800-53 CM-3 — dirty flag is cleared only after confirmed
    /// persistence, not before.
    pub fn mark_saved(&mut self) {
        for field in &mut self.fields {
            field.dirty = false;
        }
    }

    /// Discard all unsaved edits for all fields.
    ///
    /// Restores every field to its last committed value and clears dirty
    /// and editing flags.
    ///
    /// NIST SP 800-53 CM-3 — discard must restore the committed state.
    pub fn discard_all(&mut self) {
        for field in &mut self.fields {
            field.discard_edit();
        }
    }

    /// Handle an [`Action`] and return the appropriate [`ConfigStateEvent`].
    ///
    /// NIST SP 800-53 CM-3 — explicit action-to-event mapping; no implicit
    /// state mutation.
    /// NSA RTB RAIN — `Save` event is only emitted when `can_save()` is true.
    pub fn handle_action(&mut self, action: Action) -> ConfigStateEvent {
        match action {
            Action::Quit => {
                self.should_quit = true;
                ConfigStateEvent::Quit
            }
            Action::NextTab => {
                self.active_tab = (self.active_tab + 1) % self.tab_count;
                ConfigStateEvent::Redraw
            }
            Action::PrevTab => {
                self.active_tab = self.active_tab.checked_sub(1).unwrap_or(self.tab_count - 1);
                ConfigStateEvent::Redraw
            }
            Action::ScrollUp => {
                self.move_focus_up();
                ConfigStateEvent::Redraw
            }
            Action::ScrollDown => {
                self.move_focus_down();
                ConfigStateEvent::Redraw
            }
            Action::ToggleEdit => {
                self.toggle_edit_mode();
                ConfigStateEvent::Redraw
            }
            Action::DialogConfirm => {
                // Enter on a focused field enters or commits edit mode.
                self.toggle_edit_mode();
                ConfigStateEvent::Redraw
            }
            Action::Save => {
                if self.can_save() && self.is_dirty() {
                    ConfigStateEvent::Save
                } else {
                    // Nothing to save or validation blocks.
                    ConfigStateEvent::None
                }
            }
            Action::Discard => {
                if self.is_dirty() {
                    ConfigStateEvent::DiscardConfirm
                } else {
                    // Nothing to discard.
                    ConfigStateEvent::None
                }
            }
            Action::DialogCancel => {
                // Escape exits edit mode without committing.
                if let Some(field) = self.fields.get_mut(self.focused_field) {
                    if field.editing {
                        field.discard_edit();
                        return ConfigStateEvent::Redraw;
                    }
                }
                ConfigStateEvent::None
            }
            // Actions not relevant to config mode.
            Action::Expand
            | Action::Collapse
            | Action::Search
            | Action::Back
            | Action::PanelSwitch
            | Action::PageUp
            | Action::PageDown
            | Action::Refresh
            | Action::DialogToggleFocus
            | Action::ShowHelp => ConfigStateEvent::None,
        }
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    const fn move_focus_up(&mut self) {
        if self.focused_field > 0 {
            self.focused_field -= 1;
        }
    }

    const fn move_focus_down(&mut self) {
        let max = self.fields.len().saturating_sub(1);
        if self.focused_field < max {
            self.focused_field += 1;
        }
    }

    fn toggle_edit_mode(&mut self) {
        if let Some(field) = self.fields.get_mut(self.focused_field) {
            if field.editing {
                // Exit edit mode: commit the buffer. The return value
                // indicates success/failure; here we let the validation
                // result remain in field.validation for the renderer to display.
                // The field stays in editing=false regardless — the caller
                // reads field.validation directly for inline feedback.
                let _ = field.commit_edit();
            } else {
                // Enter edit mode: populate the edit buffer from the current value.
                field.edit_buffer = field.value.display();
                field.editing = true;
            }
        }
    }
}
