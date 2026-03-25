// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # config_selinux — ConfigApp Usage Example
//!
//! Demonstrates the `ConfigApp` / `ConfigState` pattern by building a
//! mock SELinux configuration editor. Like the viewer example, this drives
//! the state machine directly without a real terminal.
//!
//! ## What this shows
//!
//! - Implementing `ConfigApp` on a data struct
//! - Defining `FieldDef` with typed values and validation closures
//! - Using `handle_action` to enter edit mode, commit values, and save
//! - The dirty flag, `can_save()` gate, and `mark_saved()` lifecycle
//! - `DiffEntry` generation for before/after change review
//!
//! ## Running
//!
//! ```bash
//! cargo run -p umrs-ui --example config_selinux
//! ```

use std::collections::HashMap;

use umrs_ui::app::{StatusLevel, StatusMessage, TabDef};
use umrs_ui::config::diff::DiffEntry;
use umrs_ui::config::fields::{FieldDef, FieldValue, ValidationResult};
use umrs_ui::config::{ConfigApp, ConfigHeaderContext, ConfigState, ConfigStateEvent};
use umrs_ui::Action;

// ---------------------------------------------------------------------------
// Data struct
// ---------------------------------------------------------------------------

/// Represents the committed (last saved) SELinux configuration.
struct SelinuxConfigApp {
    title: String,
    tabs: Vec<TabDef>,
    /// Last saved values, keyed by field label.
    committed: HashMap<String, String>,
}

impl SelinuxConfigApp {
    fn new() -> Self {
        let mut committed = HashMap::new();
        committed.insert("SELINUX".to_owned(), "enforcing".to_owned());
        committed.insert("SELINUXTYPE".to_owned(), "targeted".to_owned());
        committed.insert("AUDIT_ENABLED".to_owned(), "enabled".to_owned());

        Self {
            title: "SELinux Configuration Editor".to_owned(),
            tabs: vec![TabDef::new("Config"), TabDef::new("Help")],
            committed,
        }
    }
}

impl ConfigApp for SelinuxConfigApp {
    fn card_title(&self) -> &str {
        &self.title
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn status(&self) -> StatusMessage {
        StatusMessage {
            level: StatusLevel::Ok,
            text: "Ready".to_owned(),
        }
    }

    fn config_header(&self) -> ConfigHeaderContext {
        ConfigHeaderContext::new(
            "SELinux Config Editor",
            "/etc/selinux/config",
            "all fields valid",
        )
    }

    fn committed_values(&self) -> HashMap<String, String> {
        self.committed.clone()
    }
}

// ---------------------------------------------------------------------------
// Validators
// ---------------------------------------------------------------------------

fn validate_selinux_mode(v: &FieldValue) -> ValidationResult {
    let valid = ["enforcing", "permissive", "disabled"];
    match v {
        FieldValue::Selection(s) if valid.contains(&s.as_str()) => {
            if s == "disabled" {
                ValidationResult::Warning(
                    "Disabling SELinux requires a reboot and reduces system security".to_owned(),
                )
            } else {
                ValidationResult::Ok
            }
        }
        _ => ValidationResult::Error(
            "Must be 'enforcing', 'permissive', or 'disabled'".to_owned(),
        ),
    }
}

fn validate_selinux_type(v: &FieldValue) -> ValidationResult {
    let valid = ["targeted", "mls", "minimum"];
    match v {
        FieldValue::Selection(s) if valid.contains(&s.as_str()) => ValidationResult::Ok,
        _ => ValidationResult::Error(
            "Must be 'targeted', 'mls', or 'minimum'".to_owned(),
        ),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Populate the three SELinux fields with typed values and validators.
fn populate_fields(state: &mut ConfigState) {
    state.fields.push(
        FieldDef::selection(
            "SELINUX",
            vec![
                "enforcing".to_owned(),
                "permissive".to_owned(),
                "disabled".to_owned(),
            ],
            "enforcing",
            validate_selinux_mode,
        )
        .required(),
    );
    state.fields.push(
        FieldDef::selection(
            "SELINUXTYPE",
            vec![
                "targeted".to_owned(),
                "mls".to_owned(),
                "minimum".to_owned(),
            ],
            "targeted",
            validate_selinux_type,
        )
        .required(),
    );
    state.fields.push(FieldDef::toggle(
        "AUDIT_ENABLED",
        true,
        |_| ValidationResult::Ok,
    ));
}

/// Demonstrate entering edit mode, cycling a selection, and committing.
fn demo_edit_cycle(state: &mut ConfigState) {
    println!("--- Entering edit mode on SELINUX field ---");
    let event = state.handle_action(Action::ToggleEdit);
    println!("Event: {event:?}");
    println!("  fields[0].editing = {}", state.fields[0].editing);

    state.fields[0].cycle_selection();
    println!("  After cycle: {}", state.fields[0].value.display());

    println!("--- Committing edit ---");
    let event = state.handle_action(Action::ToggleEdit);
    println!("Event: {event:?}");
    println!("  fields[0].value = {}", state.fields[0].value.display());
    println!("  fields[0].dirty = {}", state.fields[0].dirty);
    println!("  fields[0].validation = {:?}", state.fields[0].validation);
    println!("  is_dirty(): {}", state.is_dirty());
    println!();
}

/// Build diff entries and print before/after table.
fn demo_diff(app: &SelinuxConfigApp, state: &ConfigState) {
    println!("--- Diff (before → after) ---");
    let committed = app.committed_values();
    let diff_entries: Vec<DiffEntry> = state
        .fields
        .iter()
        .map(|f| {
            let before = committed.get(&f.label).cloned().unwrap_or_default();
            DiffEntry::new(f.label.clone(), before, f.value.display())
        })
        .collect();
    println!("  {:<20}  {:<20}  After", "Field", "Before");
    for entry in &diff_entries {
        let changed_marker = if entry.is_changed() { " *" } else { "" };
        println!(
            "  {:<20}  {:<20}  {}{}",
            entry.label, entry.before, entry.after, changed_marker
        );
    }
    println!();
}

/// Attempt a save and simulate writing to disk.
fn demo_save(state: &mut ConfigState) {
    println!("--- Save ---");
    println!("  can_save(): {}", state.can_save());
    let event = state.handle_action(Action::Save);
    println!("  Event: {event:?}");

    if event == ConfigStateEvent::Save && state.can_save() {
        println!("  [Simulated] Writing /etc/selinux/config ...");
        state.mark_saved();
        println!("  mark_saved() called");
        println!("  is_dirty() after save: {}", state.is_dirty());
    }
    println!();
}

/// Demonstrate discard on a clean form then on a dirty form.
fn demo_discard(state: &mut ConfigState) {
    println!("--- Discard on clean form ---");
    let event = state.handle_action(Action::Discard);
    println!("  Event: {event:?} (expected None — nothing to discard)");
    println!();

    println!("--- Toggle AUDIT_ENABLED ---");
    state.fields[2].toggle_value();
    println!(
        "  AUDIT_ENABLED = {}  (dirty: {})",
        state.fields[2].value.display(),
        state.fields[2].dirty
    );
    println!();

    println!("--- Discard when dirty (AUDIT_ENABLED changed) ---");
    let event = state.handle_action(Action::Discard);
    println!("  Event: {event:?} (expected DiscardConfirm)");
    if event == ConfigStateEvent::DiscardConfirm {
        state.discard_all();
        println!("  discard_all() called");
        println!("  is_dirty(): {}", state.is_dirty());
    }
    println!();
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let app = SelinuxConfigApp::new();
    let mut state = ConfigState::new(app.tabs().len());
    populate_fields(&mut state);

    println!("=== SELinux Configuration Editor — ConfigApp Example ===\n");

    // Initial state: all fields Pending.
    println!("Initial state:");
    println!("  is_dirty():  {}", state.is_dirty());
    println!("  can_save():  {}", state.can_save());
    println!();

    // Run all validators.
    let summary = state.validate_all();
    println!("After validate_all(): {summary}");
    println!("  can_save():  {}", state.can_save());
    println!();

    // Print field state.
    println!("Field values:");
    for field in &state.fields {
        println!(
            "  {:<20} = {}  (dirty: {}, validation: {:?})",
            field.label,
            field.value.display(),
            field.dirty,
            field.validation
        );
    }
    println!();

    demo_edit_cycle(&mut state);
    demo_diff(&app, &state);
    demo_save(&mut state);
    demo_discard(&mut state);

    // Header context.
    let ctx = app.config_header();
    println!("=== Header Context ===");
    println!("Tool:    {}", ctx.tool_name);
    println!("Target:  {}", ctx.config_target);
    println!("Status:  {}", ctx.validation_summary);

    println!("\n=== Example complete ===");
}
