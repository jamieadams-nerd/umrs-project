// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # App — Audit Card Trait, State, and Supporting Types
//!
//! Defines the [`AuditCardApp`] trait that callers implement to feed data
//! into the audit card layout, plus [`AuditCardState`] which tracks mutable
//! UI state (active tab, scroll position, quit flag).
//!
//! ## Design
//!
//! The trait is object-safe: `render_audit_card` accepts `&dyn AuditCardApp`.
//! State is separate from data — the calling binary owns `AuditCardState`
//! and updates it in the event loop. The trait impl provides read-only data.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit record content — the trait requires a
//!   report name and subject, ensuring every card is self-identifying.
//! - **NSA RTB**: Security state is represented as typed enum variants
//!   (`StatusLevel`, `StyleHint`), never as raw strings.

use crate::keymap::Action;

// ---------------------------------------------------------------------------
// IndicatorValue
// ---------------------------------------------------------------------------

/// The state of a single security indicator, as read from the kernel.
///
/// Fail-closed: any read failure or unimplemented source maps to `Unavailable`,
/// never to a false `Active` or `Inactive` assertion.
///
/// NIST SP 800-53 SI-7: Software and Information Integrity — indicator values
/// are derived exclusively from provenance-verified kernel attribute reads.
/// NIST SP 800-53 CM-6: Configuration Settings — captures live kernel state,
/// not assumed configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "security indicator value must be inspected; discarding it hides the kernel state"]
pub enum IndicatorValue {
    /// Kernel attribute confirms the feature is active/enforcing.
    Active(String),
    /// Kernel attribute confirms the feature is inactive/permissive.
    Inactive(String),
    /// Source not yet implemented, or read failed — fail-closed default.
    Unavailable,
}

// ---------------------------------------------------------------------------
// SecurityIndicators
// ---------------------------------------------------------------------------

/// Snapshot of live kernel security indicators for the header indicator row.
///
/// Populated once per session by [`crate::indicators::read_security_indicators`]
/// and passed to the render path. Values are never mutated after construction.
///
/// All fields default to `IndicatorValue::Unavailable` — the fail-closed
/// baseline. A field becomes `Active` or `Inactive` only when a provenance-
/// verified kernel attribute read succeeds.
///
/// NIST SP 800-53 SI-7: Software and Information Integrity — all values
/// originate from `SecureReader`-gated kernel attribute reads.
/// NIST SP 800-53 CM-6: Configuration Settings — live kernel state, not
/// static configuration.
#[derive(Debug, Clone)]
pub struct SecurityIndicators {
    /// SELinux enforcement mode (`/sys/fs/selinux/enforce`).
    pub selinux_status: IndicatorValue,

    /// FIPS 140-2/3 cryptographic mode (`/proc/sys/crypto/fips_enabled`).
    pub fips_mode: IndicatorValue,

    /// Active LSM list (`/sys/kernel/security/lsm` — TODO: not yet implemented).
    pub active_lsm: IndicatorValue,

    /// Kernel lockdown level (`/sys/kernel/security/lockdown`).
    pub lockdown_mode: IndicatorValue,

    /// Secure Boot state (platform-specific — TODO: not yet implemented).
    pub secure_boot: IndicatorValue,
}

impl Default for SecurityIndicators {
    /// Construct a fully-unavailable indicator set — the fail-closed baseline.
    ///
    /// Callers should replace fields with live kernel reads where available.
    fn default() -> Self {
        Self {
            selinux_status: IndicatorValue::Unavailable,
            fips_mode: IndicatorValue::Unavailable,
            active_lsm: IndicatorValue::Unavailable,
            lockdown_mode: IndicatorValue::Unavailable,
            secure_boot: IndicatorValue::Unavailable,
        }
    }
}

// ---------------------------------------------------------------------------
// StyleHint
// ---------------------------------------------------------------------------

/// Visual emphasis hint for a data row value.
///
/// Maps to a foreground color in the theme. Callers use this to convey
/// semantic meaning (e.g., trust tier) without hard-coding color values.
///
/// NIST SP 800-53 AU-3 — security state is typed, not free-form strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleHint {
    /// Default foreground (white).
    Normal,

    /// Cyan highlight for attention.
    Highlight,

    /// Dimmed for secondary information.
    Dim,

    /// Green — trust verified / positive security outcome.
    TrustGreen,

    /// Yellow — trust degraded / advisory condition.
    TrustYellow,

    /// Red — trust failed / security concern.
    TrustRed,
}

// ---------------------------------------------------------------------------
// StatusLevel
// ---------------------------------------------------------------------------

/// Severity tier for the status bar.
///
/// Maps to a background color in the theme. Callers set the level to
/// communicate the current security posture at a glance.
///
/// NIST SP 800-53 AU-3, SI-5 — status must be visually unambiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    /// Informational — no action required.
    Info,

    /// Positive outcome — security posture is good.
    Ok,

    /// Advisory — degraded or uncertain state; review recommended.
    Warn,

    /// Error — security concern or pipeline failure; action required.
    Error,
}

// ---------------------------------------------------------------------------
// StatusMessage
// ---------------------------------------------------------------------------

/// A status bar message with an associated severity level.
///
/// The text is a short (≤80 char) summary suitable for single-line display.
/// Must not contain security labels, credentials, or classified data
/// (NIST SP 800-53 SI-12).
#[derive(Debug, Clone)]
pub struct StatusMessage {
    /// Severity tier controlling the background color.
    pub level: StatusLevel,

    /// Short display text. Must not contain sensitive data.
    pub text: String,
}

impl StatusMessage {
    /// Construct a new `StatusMessage`.
    #[must_use]
    pub fn new(level: StatusLevel, text: impl Into<String>) -> Self {
        Self {
            level,
            text: text.into(),
        }
    }
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self::new(StatusLevel::Info, "Ready")
    }
}

// ---------------------------------------------------------------------------
// TabDef
// ---------------------------------------------------------------------------

/// Definition of a single tab in the tab bar.
#[derive(Debug, Clone)]
pub struct TabDef {
    /// Display label shown in the tab bar.
    pub label: String,
}

impl TabDef {
    /// Construct a tab definition with the given label.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// DataRow
// ---------------------------------------------------------------------------

/// A single content item in the data panel.
///
/// `KeyValue` is the standard entry (key: value). `TwoColumn` renders two
/// independent key-value pairs side-by-side in the left and right half of
/// the panel area. `GroupTitle` adds a labelled section header (wired up in
/// Phase 4). `Separator` inserts a blank line.
///
/// ## Column pairing convention
///
/// Use `TwoColumn` when two logically independent but equally-weighted facts
/// belong on the same conceptual row (e.g., distro name vs. kernel version).
/// Use `KeyValue` for single facts, or facts that have subordinate detail
/// rows below them. `TwoColumn` is not appropriate when one fact is
/// subordinate to the other.
///
/// ## Trust boundary
///
/// Key and value strings are rendered verbatim. Callers must not include
/// security labels, credentials, or classified data in display strings
/// (NIST SP 800-53 SI-12).
///
/// NIST SP 800-53 AU-3 — every data item is labelled; no ambiguous blobs.
#[derive(Debug, Clone)]
pub enum DataRow {
    /// Standard single-column key-value row.
    KeyValue {
        /// Field name or label.
        key: String,
        /// Field value or description.
        value: String,
        /// Visual hint applied to the value column.
        style_hint: StyleHint,
    },

    /// Two key-value pairs rendered side-by-side.
    ///
    /// The panel area is split at the midpoint. The left pair occupies the
    /// left half; the right pair occupies the right half. Each half uses
    /// half the standard key column width.
    TwoColumn {
        /// Key for the left column.
        left_key: String,
        /// Value for the left column.
        left_value: String,
        /// Style hint for the left value.
        left_hint: StyleHint,
        /// Key for the right column.
        right_key: String,
        /// Value for the right column.
        right_value: String,
        /// Style hint for the right value.
        right_hint: StyleHint,
    },

    /// Section header rendered flush-left using the `group_title` theme style.
    ///
    /// ## Indentation convention
    ///
    /// Items that logically belong under a group title should be indented by
    /// the caller prepending `"  "` (two spaces) to the `key` string in
    /// subsequent `KeyValue` and `TwoColumn` rows. The library does not enforce
    /// or track indentation state — this is a presentation convention only.
    ///
    /// ## Semantic scope
    ///
    /// Group titles carry no semantic enforcement. They are visual organizers
    /// only. The caller is responsible for placing the correct rows under the
    /// correct group title. Misplacement does not produce a rendering error,
    /// but it may mislead an assessor who interprets the visual grouping as
    /// an accurate representation of data source boundaries.
    GroupTitle(String),

    /// Blank separator line.
    Separator,

    /// A fixed three-column table row for structured evidence display.
    ///
    /// Used for evidence chains and similar structured, labelled data. Columns
    /// are left-aligned; widths are fixed by the constants in `data_panel`.
    ///
    /// `style_hint` is applied to `col3` (the verification / outcome column),
    /// which conveys the security-relevant result at a glance. `col1` and
    /// `col2` are rendered with the standard key and value styles respectively.
    ///
    /// ## Trust Boundary
    ///
    /// Column strings are rendered verbatim. Callers must not include security
    /// labels, credentials, or classified data (NIST SP 800-53 SI-12). Path
    /// strings must be truncated to column width at the call site.
    ///
    /// NIST SP 800-53 AU-3 — evidence records are labelled and structured;
    /// each row names its type, source, and verification outcome.
    TableRow {
        /// Evidence type label (column 1 — `Evidence Type`).
        col1: String,
        /// Source path or identifier (column 2 — `Source`).
        col2: String,
        /// Verification outcome string (column 3 — `Verification`).
        col3: String,
        /// Visual hint applied to the verification column.
        style_hint: StyleHint,
    },

    /// Table column header row rendered with bold key styling across all columns.
    ///
    /// Emitted once per evidence group, immediately after the `GroupTitle` row
    /// and before the first `TableRow`. Provides column labels so the table is
    /// self-describing.
    ///
    /// NIST SP 800-53 AU-3 — every field in the evidence table is labelled.
    TableHeader {
        /// Label for column 1.
        col1: String,
        /// Label for column 2.
        col2: String,
        /// Label for column 3.
        col3: String,
    },
}

impl DataRow {
    /// Construct a `KeyValue` row with an explicit style hint.
    ///
    /// This is the primary constructor. Use `normal()` for unstyled rows.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled field from the audit card"]
    pub fn key_value(
        key: impl Into<String>,
        value: impl Into<String>,
        hint: StyleHint,
    ) -> Self {
        Self::KeyValue {
            key: key.into(),
            value: value.into(),
            style_hint: hint,
        }
    }

    /// Construct a `KeyValue` row with the given key, value, and style hint.
    ///
    /// Alias for [`DataRow::key_value`] — kept for call-site readability when
    /// the hint is already a named variable at the call site.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled field from the audit card"]
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        hint: StyleHint,
    ) -> Self {
        Self::key_value(key, value, hint)
    }

    /// Construct a `KeyValue` row with `Normal` style.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled field from the audit card"]
    pub fn normal(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::key_value(key, value, StyleHint::Normal)
    }

    /// Construct a blank `Separator` row.
    #[must_use = "DataRow::separator() must be pushed into the row list; \
                  discarding it omits visual spacing from the audit card"]
    pub const fn separator() -> Self {
        Self::Separator
    }

    /// Construct a `TwoColumn` row with explicitly styled left and right pairs.
    ///
    /// Use when two independently meaningful facts should share a display row.
    /// See the type-level documentation for the column pairing convention.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  labelled fields from the audit card"]
    pub fn two_column(
        left_key: impl Into<String>,
        left_value: impl Into<String>,
        left_hint: StyleHint,
        right_key: impl Into<String>,
        right_value: impl Into<String>,
        right_hint: StyleHint,
    ) -> Self {
        Self::TwoColumn {
            left_key: left_key.into(),
            left_value: left_value.into(),
            left_hint,
            right_key: right_key.into(),
            right_value: right_value.into(),
            right_hint,
        }
    }

    /// Construct a `GroupTitle` row with the given label string.
    ///
    /// Renders flush-left using the `group_title` theme style (bold white by
    /// default). Use to introduce a named section in the data panel. Follow
    /// with `KeyValue` or `TwoColumn` rows indented per the caller convention
    /// described in the `GroupTitle` variant documentation.
    #[must_use = "DataRow::group_title() must be pushed into the row list; \
                  discarding it omits a section header from the audit card"]
    pub fn group_title(title: impl Into<String>) -> Self {
        Self::GroupTitle(title.into())
    }

    /// Construct a `TableRow` with three column strings and a style hint.
    ///
    /// `col1` is the evidence type label; `col2` is the source path or
    /// identifier; `col3` is the verification outcome. `hint` is applied
    /// to `col3` to convey the security-relevant result at a glance.
    ///
    /// Column strings must not exceed the fixed column widths defined in
    /// `data_panel`. Callers are responsible for truncation before calling
    /// this constructor (NIST SP 800-53 SI-12).
    ///
    /// NIST SP 800-53 AU-3 — evidence rows are labelled and structured.
    #[must_use = "DataRow::table_row() must be pushed into the row list; \
                  discarding it silently omits an evidence record from the audit card"]
    pub fn table_row(
        col1: impl Into<String>,
        col2: impl Into<String>,
        col3: impl Into<String>,
        hint: StyleHint,
    ) -> Self {
        Self::TableRow {
            col1: col1.into(),
            col2: col2.into(),
            col3: col3.into(),
            style_hint: hint,
        }
    }

    /// Construct a `TableHeader` row with three column label strings.
    ///
    /// Rendered with bold key styling across all three columns. Emit once per
    /// evidence group, immediately after the `GroupTitle` row and before the
    /// first `TableRow`.
    ///
    /// NIST SP 800-53 AU-3 — table headers ensure the evidence display is
    /// self-labelling; no column requires external context to interpret.
    #[must_use = "DataRow::table_header() must be pushed into the row list; \
                  discarding it omits column labels from the evidence table"]
    pub fn table_header(
        col1: impl Into<String>,
        col2: impl Into<String>,
        col3: impl Into<String>,
    ) -> Self {
        Self::TableHeader {
            col1: col1.into(),
            col2: col2.into(),
            col3: col3.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// HeaderField
// ---------------------------------------------------------------------------

/// A single labelled field displayed in the header below the indicator row.
///
/// Callers use this to surface supplemental identification data — for example,
/// a tool version string, a run timestamp, or an assessment identifier — so
/// that every rendered card carries enough context to serve as a standalone
/// SP 800-53A Examine object.
///
/// ## Trust Boundary
///
/// `value` is rendered verbatim. The caller must ensure it does not contain
/// security labels, credentials, or classified data (NIST SP 800-53 SI-12).
/// This field is for identification metadata only, not policy data.
///
/// NIST SP 800-53 AU-3 — labelled header fields ensure every audit card is
/// self-identifying: report, host, subject, and supplemental context are all
/// present on every rendered frame.
#[derive(Debug, Clone)]
pub struct HeaderField {
    /// Display label for the left column (e.g., `"Version"`).
    pub label: String,

    /// Display value for the right column.
    ///
    /// Must not contain security labels, credentials, or classified data.
    pub value: String,

    /// Visual style hint applied to the value column.
    pub style_hint: StyleHint,
}

impl HeaderField {
    /// Construct a `HeaderField` with an explicit style hint.
    ///
    /// `label` is shown in the left column; `value` in the right column.
    /// The `hint` controls the foreground color of the value.
    #[must_use = "HeaderField must be stored and returned from header_fields(); \
                  discarding it silently omits identification data from the audit card"]
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        hint: StyleHint,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            style_hint: hint,
        }
    }

    /// Construct a `HeaderField` with `Normal` style.
    ///
    /// Convenience constructor for fields that need no emphasis.
    #[must_use = "HeaderField must be stored and returned from header_fields(); \
                  discarding it silently omits identification data from the audit card"]
    pub fn normal(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(label, value, StyleHint::Normal)
    }
}

// ---------------------------------------------------------------------------
// AuditCardApp trait
// ---------------------------------------------------------------------------

/// Data provider trait for the audit card layout.
///
/// Implement this on your application data struct. The layout engine calls
/// these methods on every frame draw — implementations must be cheap (no I/O,
/// no locking, no expensive computation).
///
/// NIST SP 800-53 AU-3 — every audit card surface is self-identifying
/// via `report_name` and `report_subject`.
pub trait AuditCardApp {
    /// Short name of the report displayed in the header (e.g., "OS Detection").
    fn report_name(&self) -> &'static str;

    /// Card title displayed in the header border.
    ///
    /// Override to customize; the default returns `"UMRS Audit Card"`.
    /// Returns `String` (not `&'static str`) so that binaries can supply
    /// a translated string via `i18n::tr()`.
    fn card_title(&self) -> String {
        "UMRS Audit Card".to_owned()
    }

    /// Subject of the report (e.g., a hostname, file path, or component name).
    fn report_subject(&self) -> &'static str;

    /// Ordered list of tab definitions. Must not be empty.
    fn tabs(&self) -> &[TabDef];

    /// Index of the currently active tab (0-based).
    ///
    /// The value returned here is informational for the header; the authoritative
    /// active tab index is held in [`AuditCardState::active_tab`].
    fn active_tab(&self) -> usize;

    /// Data rows to display for the given tab index.
    ///
    /// Called on every draw. Must be cheap — do not perform I/O here.
    fn data_rows(&self, tab_index: usize) -> Vec<DataRow>;

    /// Current status message to display in the status bar.
    fn status(&self) -> &StatusMessage;

    /// Supplemental identification fields rendered below the indicator row.
    ///
    /// Returns a slice of [`HeaderField`] values. These are appended to the
    /// header after the fixed rows (report, host, subject) and the security
    /// indicator row. If more fields are provided than the available header
    /// height allows, the renderer truncates with a `"…"` marker.
    ///
    /// The default implementation returns an empty slice — existing impls
    /// need not change, making this addition backward-compatible.
    ///
    /// NIST SP 800-53 AU-3 — supplemental fields extend audit card
    /// identification so each rendered card can serve as a standalone
    /// SP 800-53A Examine object (e.g., tool version, run timestamp).
    fn header_fields(&self) -> &[HeaderField] {
        &[]
    }
}

// ---------------------------------------------------------------------------
// AuditCardState
// ---------------------------------------------------------------------------

/// Mutable UI state for the audit card event loop.
///
/// Owned by the calling binary. Updated via [`handle_action`] in the event loop.
/// Passed alongside the immutable `AuditCardApp` impl to the render function.
///
/// NIST SP 800-53 AC-2 — `should_quit` drives clean session termination.
pub struct AuditCardState {
    /// Index of the currently displayed tab (0-based).
    pub active_tab: usize,

    /// Total number of tabs (set at construction; does not change).
    tab_count: usize,

    /// Current vertical scroll offset into the data panel.
    pub scroll_offset: usize,

    /// Set to `true` by [`Action::Quit`]; the event loop exits when this is `true`.
    pub should_quit: bool,
}

impl AuditCardState {
    /// Construct a new state for a card with `tab_count` tabs.
    ///
    /// `tab_count` must be at least 1. If 0 is passed it is clamped to 1.
    #[must_use]
    pub fn new(tab_count: usize) -> Self {
        Self {
            active_tab: 0,
            tab_count: tab_count.max(1),
            scroll_offset: 0,
            should_quit: false,
        }
    }

    /// Update state in response to an [`Action`].
    ///
    /// This is the single entry point for all keyboard-driven state changes.
    /// Call once per key event in the event loop.
    pub const fn handle_action(&mut self, action: &Action) {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::NextTab => {
                // Saturating modulo — wraps to 0 after the last tab.
                self.active_tab =
                    self.active_tab.saturating_add(1) % self.tab_count;
                self.scroll_offset = 0;
            }
            Action::PrevTab => {
                if self.active_tab == 0 {
                    self.active_tab = self.tab_count.saturating_sub(1);
                } else {
                    self.active_tab = self.active_tab.saturating_sub(1);
                }
                self.scroll_offset = 0;
            }
            Action::ScrollDown => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
            }
            Action::ScrollUp => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
            Action::PageDown => {
                self.scroll_offset = self.scroll_offset.saturating_add(10);
            }
            Action::PageUp => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
            }
            Action::Refresh => {
                // Refresh is application-defined; state itself has no refresh
                // behavior. Callers may use this signal to re-run detection.
            }
        }
    }

    /// Reset scroll offset to zero (e.g., after a data refresh).
    pub const fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
    }
}
