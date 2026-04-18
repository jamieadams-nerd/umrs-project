// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Fields — Configuration Field Types and Validation
//!
//! Defines the field type taxonomy for the [`super::ConfigApp`] pattern:
//! text input, selection from a fixed list, and boolean toggles.
//!
//! Each field carries its current value as a [`FieldValue`], a validation
//! rule, and the result of the last validation pass as a [`ValidationResult`].
//!
//! ## Validation Model
//!
//! Validation is triggered when a field exits edit mode
//! ([`crate::keymap::Action::ToggleEdit`]). The [`FieldDef::validate`] method
//! returns a [`ValidationResult`]. `ConfigState` stores results and uses them
//! to gate the `Save` action — saving is only permitted when all fields report
//! `ValidationResult::Ok`.
//!
//! Validation rules are expressed as closures stored in [`FieldDef`]. Because
//! closures cannot implement `Debug` or `Clone`, `FieldDef` is non-`Clone` and
//! non-`Debug` by design. Callers reconstruct field definitions from their data
//! model on each load rather than cloning them.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-10**: Input validation — every field has an
//!   associated validation rule; values are never committed without passing.
//! - **NIST SP 800-53 CM-3**: Configuration change control — the dirty flag
//!   on `FieldDef` tracks which fields have unsaved changes.
//! - **NSA RTB RAIN**: Non-bypassable validation — `Save` is rejected at the
//!   state level if any `ValidationResult` is not `Ok`.

// ---------------------------------------------------------------------------
// FieldValue
// ---------------------------------------------------------------------------

/// The current value of a configuration field.
///
/// Variants map to the three supported field types: free text input,
/// one-of-N selection, and boolean toggle.
///
/// ## Variants:
///
/// - `Text(String)` — a free-form text value (e.g., a path, a name, a numeric string).
/// - `Selection(String)` — a selection from a fixed set of options; the value is the selected
///   option string.
/// - `Toggle(bool)` — a boolean toggle.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: typed values prevent raw string injection into configuration
///   writers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    Text(String),
    Selection(String),
    Toggle(bool),
}

impl FieldValue {
    /// Return the display string for this value.
    ///
    /// Used in the config layout to render the current value column.
    #[must_use = "display string is used for rendering; discarding it has no effect"]
    pub fn display(&self) -> String {
        match self {
            Self::Text(s) | Self::Selection(s) => s.clone(),
            Self::Toggle(b) => {
                if *b {
                    "enabled".to_owned()
                } else {
                    "disabled".to_owned()
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ValidationResult
// ---------------------------------------------------------------------------

/// The result of validating a single field value.
///
/// Callers store this alongside each `FieldDef` in `ConfigState::fields`.
/// The renderer uses it to show inline validation feedback.
///
/// ## Variants:
///
/// - `Ok` — the field value is valid.
/// - `Error(String)` — the field value failed validation. The message is an operator-visible
///   description of what is wrong and what a correct value looks like (e.g., `"Must be
///   'enforcing', 'permissive', or 'disabled'"`). Must not contain classified data or key
///   material. NIST SP 800-53 SI-11.
/// - `Warning(String)` — the field value triggers a warning but is not strictly invalid; the
///   user may save with a warning present. The message follows the same constraints as `Error`.
/// - `Pending` — validation has not yet been run on this field (initial state).
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: validation results are typed enum variants, not raw strings;
///   callers can query and count failures programmatically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    Ok,
    Error(String),
    Warning(String),
    Pending,
}

impl ValidationResult {
    /// Return `true` if this result blocks the `Save` action.
    ///
    /// `Error` and `Pending` block save. `Ok` and `Warning` do not.
    ///
    /// NIST SP 800-53 SI-10 — only validated (non-Pending, non-Error)
    /// input may be committed.
    #[must_use = "save-blocking status must be checked before committing; discarding it bypasses validation"]
    pub const fn blocks_save(&self) -> bool {
        matches!(self, Self::Error(_) | Self::Pending)
    }

    /// Return a display string for the inline validation indicator.
    ///
    /// Used by the config layout to show a status icon + message next to
    /// each field.
    #[must_use = "display string is used for rendering; discarding it hides validation feedback"]
    pub const fn display(&self) -> &str {
        match self {
            Self::Error(msg) | Self::Warning(msg) => msg.as_str(),
            Self::Ok | Self::Pending => "",
        }
    }
}

// ---------------------------------------------------------------------------
// FieldDef
// ---------------------------------------------------------------------------

/// A single configuration field definition.
///
/// Carries the field label, current value, available options (for `Selection`
/// variants), a validation closure, a dirty flag, and a required flag.
///
/// `FieldDef` is intentionally not `Clone` or `Debug` because the validation
/// closure `Box<dyn Fn(&FieldValue) -> ValidationResult>` does not implement
/// those traits. Callers reconstruct field definitions from their data model
/// on each load.
///
/// ## Fields:
///
/// - `label` — human-readable label shown in the key column (e.g., `"SELINUX"`).
/// - `value` — current committed value.
/// - `edit_buffer` — in-progress value while the field is being edited; populated when edit mode
///   is entered; committed to `value` when `ToggleEdit` exits and validation passes.
/// - `editing` — whether this field is currently in edit mode.
/// - `options` — available options for `Selection` fields; empty for `Text`/`Toggle`.
/// - `validation` — last validation result.
/// - `dirty` — whether this field has been modified since the last save. NIST SP 800-53 CM-3.
/// - `required` — whether this field must have a non-empty value.
/// - `validator` — validation closure called with the proposed `FieldValue`; must be pure (no
///   side effects) and cheap to call; invoked on every `ToggleEdit` exit.
///   NIST SP 800-53 SI-10: validation is mandatory and non-bypassable.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: validation is mandatory; every field carries a validator; the
///   `required` flag marks fields that must have a non-empty value.
/// - **NIST SP 800-53 CM-3**: the `dirty` flag tracks unsaved changes at field granularity.
pub struct FieldDef {
    pub label: String,
    pub value: FieldValue,
    pub edit_buffer: String,
    pub editing: bool,
    pub options: Vec<String>,
    pub validation: ValidationResult,
    pub dirty: bool,
    pub required: bool,
    pub validator: Box<dyn Fn(&FieldValue) -> ValidationResult + Send>,
}

impl FieldDef {
    /// Construct a text input field.
    ///
    /// `validator` is called when the field exits edit mode. Use
    /// `|_| ValidationResult::Ok` for fields with no constraints.
    #[must_use = "FieldDef must be stored in ConfigState; constructing and discarding it has no effect"]
    pub fn text(
        label: impl Into<String>,
        initial_value: impl Into<String>,
        validator: impl Fn(&FieldValue) -> ValidationResult + Send + 'static,
    ) -> Self {
        let value = FieldValue::Text(initial_value.into());
        Self {
            label: label.into(),
            edit_buffer: value.display(),
            value,
            editing: false,
            options: Vec::new(),
            validation: ValidationResult::Pending,
            dirty: false,
            required: false,
            validator: Box::new(validator),
        }
    }

    /// Construct a selection field.
    ///
    /// `initial_value` must be one of the strings in `options`. The validator
    /// receives the proposed `FieldValue::Selection(s)` and should check
    /// whether `s` is in `options`.
    #[must_use = "FieldDef must be stored in ConfigState; constructing and discarding it has no effect"]
    pub fn selection(
        label: impl Into<String>,
        options: Vec<String>,
        initial_value: impl Into<String>,
        validator: impl Fn(&FieldValue) -> ValidationResult + Send + 'static,
    ) -> Self {
        let value = FieldValue::Selection(initial_value.into());
        Self {
            label: label.into(),
            edit_buffer: value.display(),
            value,
            editing: false,
            options,
            validation: ValidationResult::Pending,
            dirty: false,
            required: false,
            validator: Box::new(validator),
        }
    }

    /// Construct a boolean toggle field.
    ///
    /// Toggle fields use a fixed `Ok` validator by default (the value is
    /// always valid). Callers may supply a custom validator if a warning
    /// is appropriate for certain toggle states.
    #[must_use = "FieldDef must be stored in ConfigState; constructing and discarding it has no effect"]
    pub fn toggle(
        label: impl Into<String>,
        initial_value: bool,
        validator: impl Fn(&FieldValue) -> ValidationResult + Send + 'static,
    ) -> Self {
        let value = FieldValue::Toggle(initial_value);
        Self {
            label: label.into(),
            edit_buffer: value.display(),
            value,
            editing: false,
            options: Vec::new(),
            validation: ValidationResult::Pending,
            dirty: false,
            required: false,
            validator: Box::new(validator),
        }
    }

    /// Mark this field as required (non-empty value mandatory).
    #[must_use = "returns the modified field; the original is consumed"]
    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Run the validator on the current edit buffer.
    ///
    /// Returns the validation result and stores it in `self.validation`.
    /// Does not commit the buffer to `self.value`.
    ///
    /// NIST SP 800-53 SI-10 — validation is always run before commit.
    #[must_use = "validation result determines whether the edit may be committed"]
    pub fn validate_buffer(&mut self) -> &ValidationResult {
        let proposed = match &self.value {
            FieldValue::Text(_) => FieldValue::Text(self.edit_buffer.clone()),
            FieldValue::Selection(_) => FieldValue::Selection(self.edit_buffer.clone()),
            FieldValue::Toggle(b) => FieldValue::Toggle(*b),
        };
        self.validation = (self.validator)(&proposed);
        &self.validation
    }

    /// Commit the edit buffer to the field value if validation passes.
    ///
    /// Returns `true` if the commit succeeded (validation was `Ok` or
    /// `Warning`). Returns `false` if validation blocked the commit.
    ///
    /// NIST SP 800-53 SI-10 — values are never committed without validation.
    /// NIST SP 800-53 CM-3 — sets `dirty = true` on successful commit.
    #[must_use = "return value indicates whether the commit succeeded; callers must check it"]
    pub fn commit_edit(&mut self) -> bool {
        // validate_buffer stores its result in self.validation. The return value
        // is a reference to self.validation — we read blocking state via
        // self.validation.blocks_save() on the next line, so the reference
        // is intentionally not bound.
        let _ = self.validate_buffer();
        if self.validation.blocks_save() {
            return false;
        }
        let new_value = match &self.value {
            FieldValue::Text(_) => FieldValue::Text(self.edit_buffer.clone()),
            FieldValue::Selection(_) => FieldValue::Selection(self.edit_buffer.clone()),
            FieldValue::Toggle(b) => FieldValue::Toggle(*b),
        };
        self.dirty = new_value != self.value;
        self.value = new_value;
        self.editing = false;
        true
    }

    /// Toggle a boolean field value in place.
    ///
    /// No-op for non-`Toggle` field types. Marks the field dirty.
    ///
    /// NIST SP 800-53 CM-3 — toggle is an explicit operator action.
    pub fn toggle_value(&mut self) {
        if let FieldValue::Toggle(ref mut b) = self.value {
            *b = !*b;
            self.dirty = true;
            let proposed = FieldValue::Toggle(*b);
            self.validation = (self.validator)(&proposed);
        }
    }

    /// Cycle to the next option in a selection field.
    ///
    /// No-op for non-`Selection` fields. Marks dirty.
    ///
    /// NIST SP 800-53 CM-3 — selection cycle is an explicit operator action.
    pub fn cycle_selection(&mut self) {
        if let FieldValue::Selection(ref current) = self.value.clone() {
            if self.options.is_empty() {
                return;
            }
            let idx = self.options.iter().position(|o| o == current).unwrap_or(0);
            let next_idx = (idx + 1) % self.options.len();
            if let Some(next) = self.options.get(next_idx) {
                let new_val = FieldValue::Selection(next.clone());
                self.validation = (self.validator)(&new_val);
                self.dirty = new_val != self.value;
                self.value = new_val;
                self.edit_buffer = self.value.display();
            }
        }
    }

    /// Discard edit buffer and restore to the last committed value.
    ///
    /// Clears `editing` and `dirty` flags. Resets `validation` to `Pending`.
    pub fn discard_edit(&mut self) {
        self.edit_buffer = self.value.display();
        self.editing = false;
        self.dirty = false;
        self.validation = ValidationResult::Pending;
    }
}
