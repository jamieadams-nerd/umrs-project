// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for the ConfigApp pattern.
//!
//! Covers: FieldDef construction and validation, FieldValue display,
//! ValidationResult blocking, ConfigState action handling, dirty tracking,
//! save gating, and discard behavior.

use umrs_ui::Action;
use umrs_ui::config::fields::{FieldDef, FieldValue, ValidationResult};
use umrs_ui::config::{ConfigState, ConfigStateEvent};

// ---------------------------------------------------------------------------
// FieldValue
// ---------------------------------------------------------------------------

#[test]
fn field_value_text_display() {
    let v = FieldValue::Text("enforcing".to_owned());
    assert_eq!(v.display(), "enforcing");
}

#[test]
fn field_value_toggle_true_display() {
    let v = FieldValue::Toggle(true);
    assert_eq!(v.display(), "enabled");
}

#[test]
fn field_value_toggle_false_display() {
    let v = FieldValue::Toggle(false);
    assert_eq!(v.display(), "disabled");
}

#[test]
fn field_value_selection_display() {
    let v = FieldValue::Selection("permissive".to_owned());
    assert_eq!(v.display(), "permissive");
}

// ---------------------------------------------------------------------------
// ValidationResult
// ---------------------------------------------------------------------------

#[test]
fn validation_result_ok_does_not_block_save() {
    assert!(!ValidationResult::Ok.blocks_save());
}

#[test]
fn validation_result_warning_does_not_block_save() {
    assert!(!ValidationResult::Warning("msg".to_owned()).blocks_save());
}

#[test]
fn validation_result_error_blocks_save() {
    assert!(ValidationResult::Error("bad".to_owned()).blocks_save());
}

#[test]
fn validation_result_pending_blocks_save() {
    assert!(ValidationResult::Pending.blocks_save());
}

#[test]
fn validation_result_error_display_returns_message() {
    let r = ValidationResult::Error("must be numeric".to_owned());
    assert_eq!(r.display(), "must be numeric");
}

#[test]
fn validation_result_warning_display_returns_message() {
    let r = ValidationResult::Warning("unusual value".to_owned());
    assert_eq!(r.display(), "unusual value");
}

#[test]
fn validation_result_ok_display_is_empty() {
    assert!(ValidationResult::Ok.display().is_empty());
}

#[test]
fn validation_result_pending_display_is_empty() {
    assert!(ValidationResult::Pending.display().is_empty());
}

// ---------------------------------------------------------------------------
// FieldDef — text field
// ---------------------------------------------------------------------------

const fn always_ok(_v: &FieldValue) -> ValidationResult {
    ValidationResult::Ok
}

fn require_non_empty(v: &FieldValue) -> ValidationResult {
    match v {
        FieldValue::Text(s) | FieldValue::Selection(s) if s.is_empty() => {
            ValidationResult::Error("value must not be empty".to_owned())
        }
        _ => ValidationResult::Ok,
    }
}

#[test]
fn field_def_text_initial_value_stored() {
    let field = FieldDef::text("SELINUX", "enforcing", always_ok);
    assert_eq!(field.value, FieldValue::Text("enforcing".to_owned()));
    assert_eq!(field.label, "SELINUX");
}

#[test]
fn field_def_text_initial_validation_is_pending() {
    let field = FieldDef::text("F", "v", always_ok);
    assert_eq!(field.validation, ValidationResult::Pending);
}

#[test]
fn field_def_validate_buffer_ok() {
    let mut field = FieldDef::text("F", "hello", always_ok);
    let result = field.validate_buffer();
    assert_eq!(*result, ValidationResult::Ok);
}

#[test]
fn field_def_validate_buffer_error() {
    let mut field = FieldDef::text("F", "", require_non_empty);
    let result = field.validate_buffer();
    assert!(matches!(result, ValidationResult::Error(_)));
}

#[test]
fn field_def_commit_edit_succeeds_on_ok_validation() {
    let mut field = FieldDef::text("F", "old", always_ok);
    field.edit_buffer = "new".to_owned();
    field.editing = true;
    let committed = field.commit_edit();
    assert!(committed);
    assert_eq!(field.value, FieldValue::Text("new".to_owned()));
    assert!(field.dirty);
    assert!(!field.editing);
}

#[test]
fn field_def_commit_edit_fails_on_error_validation() {
    let mut field = FieldDef::text("F", "old", require_non_empty);
    field.edit_buffer = String::new(); // empty — will fail
    field.editing = true;
    let committed = field.commit_edit();
    assert!(!committed, "commit must fail when validation returns Error");
    // Value is unchanged.
    assert_eq!(field.value, FieldValue::Text("old".to_owned()));
}

#[test]
fn field_def_no_dirty_when_value_unchanged() {
    let mut field = FieldDef::text("F", "same", always_ok);
    field.edit_buffer = "same".to_owned();
    field.editing = true;
    let _ = field.commit_edit();
    assert!(!field.dirty, "committing the same value must not set dirty");
}

#[test]
fn field_def_discard_edit_restores_value() {
    let mut field = FieldDef::text("F", "original", always_ok);
    field.edit_buffer = "changed".to_owned();
    field.editing = true;
    field.discard_edit();
    assert_eq!(field.edit_buffer, "original");
    assert!(!field.editing);
    assert!(!field.dirty);
    assert_eq!(field.validation, ValidationResult::Pending);
}

// ---------------------------------------------------------------------------
// FieldDef — toggle field
// ---------------------------------------------------------------------------

#[test]
fn field_def_toggle_initial_value() {
    let field = FieldDef::toggle("ENABLED", true, always_ok);
    assert_eq!(field.value, FieldValue::Toggle(true));
}

#[test]
fn field_def_toggle_value_flips() {
    let mut field = FieldDef::toggle("T", false, always_ok);
    field.toggle_value();
    assert_eq!(field.value, FieldValue::Toggle(true));
    assert!(field.dirty);
}

#[test]
fn field_def_toggle_value_flips_back() {
    let mut field = FieldDef::toggle("T", true, always_ok);
    field.toggle_value();
    field.toggle_value();
    assert_eq!(field.value, FieldValue::Toggle(true));
}

// ---------------------------------------------------------------------------
// FieldDef — selection field
// ---------------------------------------------------------------------------

fn make_selinux_field() -> FieldDef {
    let options = vec![
        "enforcing".to_owned(),
        "permissive".to_owned(),
        "disabled".to_owned(),
    ];
    let valid_options = options.clone();
    FieldDef::selection("SELINUX", options, "enforcing", move |v| {
        if let FieldValue::Selection(s) = v {
            if valid_options.contains(s) {
                return ValidationResult::Ok;
            }
        }
        ValidationResult::Error("invalid mode".to_owned())
    })
}

#[test]
fn field_def_selection_initial_value() {
    let field = make_selinux_field();
    assert_eq!(field.value, FieldValue::Selection("enforcing".to_owned()));
}

#[test]
fn field_def_selection_cycle_advances() {
    let mut field = make_selinux_field();
    field.cycle_selection();
    assert_eq!(field.value, FieldValue::Selection("permissive".to_owned()));
}

#[test]
fn field_def_selection_cycle_wraps() {
    let mut field = make_selinux_field();
    field.cycle_selection(); // enforcing → permissive
    field.cycle_selection(); // permissive → disabled
    field.cycle_selection(); // disabled → enforcing (wrap)
    assert_eq!(field.value, FieldValue::Selection("enforcing".to_owned()));
}

#[test]
fn field_def_required_builder() {
    let field = FieldDef::text("F", "v", always_ok).required();
    assert!(field.required);
}

// ---------------------------------------------------------------------------
// ConfigState — construction
// ---------------------------------------------------------------------------

#[test]
fn config_state_new_defaults() {
    let state = ConfigState::new(2);
    assert_eq!(state.focused_field, 0);
    assert_eq!(state.active_tab, 0);
    assert!(!state.should_quit);
    assert!(state.fields.is_empty());
}

#[test]
fn config_state_not_dirty_when_empty() {
    let state = ConfigState::new(1);
    assert!(!state.is_dirty());
}

// ---------------------------------------------------------------------------
// ConfigState — dirty tracking
// ---------------------------------------------------------------------------

fn build_state_with_fields() -> ConfigState {
    let mut state = ConfigState::new(1);
    state.fields.push(FieldDef::text("F1", "v1", always_ok));
    state.fields.push(FieldDef::text("F2", "v2", always_ok));
    state
}

#[test]
fn config_state_dirty_when_field_is_dirty() {
    let mut state = build_state_with_fields();
    state.fields[0].dirty = true;
    assert!(state.is_dirty());
}

#[test]
fn config_state_mark_saved_clears_dirty() {
    let mut state = build_state_with_fields();
    state.fields[0].dirty = true;
    state.fields[1].dirty = true;
    state.mark_saved();
    assert!(!state.is_dirty());
}

#[test]
fn config_state_discard_all_clears_edits() {
    let mut state = build_state_with_fields();
    state.fields[0].edit_buffer = "changed".to_owned();
    state.fields[0].editing = true;
    state.fields[0].dirty = true;
    state.discard_all();
    assert!(!state.is_dirty());
    assert!(!state.fields[0].editing);
}

// ---------------------------------------------------------------------------
// ConfigState — save gating
// ---------------------------------------------------------------------------

#[test]
fn can_save_true_when_all_fields_ok() {
    let mut state = build_state_with_fields();
    // Run all validators to move from Pending to Ok. Summary not needed here.
    let _ = state.validate_all();
    assert!(state.can_save());
}

#[test]
fn can_save_false_when_any_field_pending() {
    let state = build_state_with_fields();
    // All fields start in Pending state.
    assert!(!state.can_save(), "Pending fields must block save");
}

#[test]
fn can_save_false_when_any_field_has_error() {
    let mut state = ConfigState::new(1);
    state.fields.push(FieldDef::text("F", "", require_non_empty));
    let _ = state.validate_all();
    assert!(!state.can_save());
}

// ---------------------------------------------------------------------------
// ConfigState — validate_all
// ---------------------------------------------------------------------------

#[test]
fn validate_all_returns_all_fields_valid_on_clean_state() {
    let mut state = build_state_with_fields();
    let summary = state.validate_all();
    assert_eq!(summary, "all fields valid");
}

#[test]
fn validate_all_returns_error_count_on_failures() {
    let mut state = ConfigState::new(1);
    state.fields.push(FieldDef::text("F1", "", require_non_empty));
    state.fields.push(FieldDef::text("F2", "", require_non_empty));
    let summary = state.validate_all();
    assert!(
        summary.contains('2'),
        "summary must report 2 errors; got: {summary}"
    );
}

// ---------------------------------------------------------------------------
// ConfigState — handle_action
// ---------------------------------------------------------------------------

#[test]
fn quit_action_sets_should_quit_and_returns_quit_event() {
    let mut state = build_state_with_fields();
    let event = state.handle_action(Action::Quit);
    assert_eq!(event, ConfigStateEvent::Quit);
    assert!(state.should_quit);
}

#[test]
fn next_tab_action_advances_tab() {
    let mut state = ConfigState::new(2);
    let event = state.handle_action(Action::NextTab);
    assert_eq!(event, ConfigStateEvent::Redraw);
    assert_eq!(state.active_tab, 1);
}

#[test]
fn prev_tab_action_wraps() {
    let mut state = ConfigState::new(2);
    let event = state.handle_action(Action::PrevTab);
    assert_eq!(event, ConfigStateEvent::Redraw);
    assert_eq!(
        state.active_tab, 1,
        "PrevTab at 0 must wrap to tab_count - 1"
    );
}

#[test]
fn scroll_down_action_moves_focus() {
    let mut state = build_state_with_fields();
    assert_eq!(state.focused_field, 0);
    let event = state.handle_action(Action::ScrollDown);
    assert_eq!(event, ConfigStateEvent::Redraw);
    assert_eq!(state.focused_field, 1);
}

#[test]
fn scroll_up_at_zero_stays_at_zero() {
    let mut state = build_state_with_fields();
    let event = state.handle_action(Action::ScrollUp);
    assert_eq!(event, ConfigStateEvent::Redraw);
    assert_eq!(state.focused_field, 0);
}

#[test]
fn save_action_returns_save_event_when_valid_and_dirty() {
    let mut state = build_state_with_fields();
    let _ = state.validate_all();
    state.fields[0].dirty = true; // make dirty
    let event = state.handle_action(Action::Save);
    assert_eq!(
        event,
        ConfigStateEvent::Save,
        "Save action on clean, dirty form must return Save event"
    );
}

#[test]
fn save_action_returns_none_when_not_dirty() {
    let mut state = build_state_with_fields();
    let _ = state.validate_all();
    // not dirty
    let event = state.handle_action(Action::Save);
    assert_eq!(
        event,
        ConfigStateEvent::None,
        "Save on clean (not dirty) form must return None"
    );
}

#[test]
fn save_action_returns_none_when_validation_blocks() {
    let mut state = ConfigState::new(1);
    state.fields.push(FieldDef::text("F", "", require_non_empty));
    state.fields[0].dirty = true;
    // Do not validate — field stays Pending.
    let event = state.handle_action(Action::Save);
    assert_eq!(
        event,
        ConfigStateEvent::None,
        "Save must be blocked when field is in Pending/Error state"
    );
}

#[test]
fn discard_action_returns_confirm_event_when_dirty() {
    let mut state = build_state_with_fields();
    state.fields[0].dirty = true;
    let event = state.handle_action(Action::Discard);
    assert_eq!(event, ConfigStateEvent::DiscardConfirm);
}

#[test]
fn discard_action_returns_none_when_clean() {
    let mut state = build_state_with_fields();
    let event = state.handle_action(Action::Discard);
    assert_eq!(
        event,
        ConfigStateEvent::None,
        "Discard on clean form must return None"
    );
}

#[test]
fn toggle_edit_enters_edit_mode() {
    let mut state = build_state_with_fields();
    assert!(!state.fields[0].editing);
    let _ = state.handle_action(Action::ToggleEdit);
    assert!(state.fields[0].editing);
}

#[test]
fn toggle_edit_exits_edit_mode_and_commits() {
    let mut state = build_state_with_fields();
    let _ = state.handle_action(Action::ToggleEdit); // enter edit
    state.fields[0].edit_buffer = "new_value".to_owned();
    let _ = state.handle_action(Action::ToggleEdit); // exit edit, commit
    assert!(!state.fields[0].editing);
    assert_eq!(
        state.fields[0].value,
        FieldValue::Text("new_value".to_owned())
    );
}

#[test]
fn dialog_cancel_exits_edit_mode_without_commit() {
    let mut state = build_state_with_fields();
    let _ = state.handle_action(Action::ToggleEdit); // enter edit
    state.fields[0].edit_buffer = "changed_but_cancelled".to_owned();
    let event = state.handle_action(Action::DialogCancel);
    assert_eq!(event, ConfigStateEvent::Redraw);
    assert!(!state.fields[0].editing);
    // Value unchanged (discard_edit restores to original buffer).
    assert_eq!(state.fields[0].value, FieldValue::Text("v1".to_owned()));
}

// ---------------------------------------------------------------------------
// ConfigState — validation_summary
// ---------------------------------------------------------------------------

#[test]
fn validation_summary_all_valid() {
    let mut state = build_state_with_fields();
    let _ = state.validate_all();
    assert_eq!(state.validation_summary(), "all fields valid");
}

#[test]
fn validation_summary_counts_errors() {
    let mut state = ConfigState::new(1);
    state.fields.push(FieldDef::text("F", "", require_non_empty));
    let _ = state.validate_all();
    let summary = state.validation_summary();
    assert!(
        summary.contains('1'),
        "expected '1' in summary, got: {summary}"
    );
    assert!(
        summary.contains("error"),
        "expected 'error' in summary, got: {summary}"
    );
}

// ---------------------------------------------------------------------------
// ConfigHeaderContext
// ---------------------------------------------------------------------------

#[test]
fn config_header_context_fields_stored() {
    use umrs_ui::ConfigHeaderContext;
    let ctx = ConfigHeaderContext::new(
        "SELinux Config",
        "/etc/selinux/config",
        "all fields valid",
    );
    assert_eq!(ctx.tool_name, "SELinux Config");
    assert_eq!(ctx.config_target, "/etc/selinux/config");
    assert_eq!(ctx.validation_summary, "all fields valid");
}
