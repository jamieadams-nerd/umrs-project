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

use umrs_platform::posture::ContradictionKind;

use crate::keymap::Action;

// ---------------------------------------------------------------------------
// IndicatorValue
// ---------------------------------------------------------------------------

/// The state of a single security indicator, as read from the kernel.
///
/// Fail-closed: any read failure or unimplemented source maps to `Unavailable`,
/// never to a false `Enabled` or `Disabled` assertion.
///
/// ## Variants:
///
/// - `Enabled(String)` — kernel attribute confirms the feature is enabled/enforcing.
/// - `Disabled(String)` — kernel attribute confirms the feature is disabled/permissive.
/// - `Unavailable` — source not yet implemented, or read failed; fail-closed default.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: Software and Information Integrity — indicator values are derived
///   exclusively from provenance-verified kernel attribute reads.
/// - **NIST SP 800-53 CM-6**: Configuration Settings — captures live kernel state, not assumed
///   configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "security indicator value must be inspected; discarding it hides the kernel state"]
pub enum IndicatorValue {
    Enabled(String),
    Disabled(String),
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
/// baseline. A field becomes `Enabled` or `Disabled` only when a provenance-
/// verified kernel attribute read succeeds.
///
/// ## Fields:
///
/// - `selinux_status` — SELinux enforcement mode (`/sys/fs/selinux/enforce`).
/// - `fips_mode` — FIPS 140-2/3 cryptographic mode (`/proc/sys/crypto/fips_enabled`).
/// - `active_lsm` — active LSM list (`/sys/kernel/security/lsm` — TODO: not yet implemented).
/// - `lockdown_mode` — kernel lockdown level (`/sys/kernel/security/lockdown`).
/// - `secure_boot` — Secure Boot state (platform-specific — TODO: not yet implemented).
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: Software and Information Integrity — all values originate from
///   `SecureReader`-gated kernel attribute reads.
/// - **NIST SP 800-53 CM-6**: Configuration Settings — live kernel state, not static
///   configuration.
#[derive(Debug, Clone)]
pub struct SecurityIndicators {
    pub selinux_status: IndicatorValue,
    pub fips_mode: IndicatorValue,
    pub active_lsm: IndicatorValue,
    pub lockdown_mode: IndicatorValue,
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
// HeaderContext
// ---------------------------------------------------------------------------

/// System-identification and security posture snapshot for the header panel.
///
/// Constructed once per session by [`crate::indicators::build_header_context`]
/// and passed into [`crate::layout::render_audit_card`]. All fields are
/// immutable after construction; the header render path receives a shared
/// reference.
///
/// ## Design
///
/// Separating system-identification data from the `AuditCardApp` trait keeps
/// the trait focused on application data (rows, tabs, status) while this
/// struct holds fields that are the same for every card on the same system:
/// hostname, kernel version, boot ID, OS name, architecture, system UUID,
/// and security posture indicators.
///
/// The `assessed_at` field is the ISO-8601 timestamp captured at detection
/// time, satisfying the CA-7 requirement that each Examine object carry a
/// collection timestamp. The `tool_name` and `tool_version` fields allow
/// an assessor to trace evidence to a specific tool version (SA-11).
///
/// ## Trust Boundary
///
/// All values are display-only. `hostname`, `kernel_version`, and
/// `architecture` come from `uname(2)` and are not trust-relevant assertions.
/// `boot_id` comes from `/proc/sys/kernel/random/boot_id` via
/// `ProcfsText` + `SecureReader`. `os_name` is supplied by the calling binary
/// from the OS detection pipeline; it is display-only and not a policy input.
/// `system_uuid` comes from `/sys/class/dmi/id/product_uuid` via `SysfsText`;
/// set to `"unavailable"` on read failure (non-UEFI systems, permission errors).
/// `indicators` are populated via provenance-verified kattr reads.
///
/// ## Fields:
///
/// - `indicators` — live kernel security posture indicators.
/// - `tool_name` — tool name (the binary that produced this audit card); typically
///   `env!("CARGO_PKG_NAME")` from the calling binary.
/// - `tool_version` — tool version (the binary version that produced this audit card); typically
///   `env!("CARGO_PKG_VERSION")`.
/// - `assessed_at` — ISO-8601 timestamp of when detection was run; captured at tool startup
///   before the detection pipeline runs; satisfies CA-7 requirement for timestamped collection
///   events.
/// - `hostname` — system hostname (display-only, from `uname(2)`); not a trust-relevant
///   assertion; used only to label which host the card was collected on.
/// - `kernel_version` — kernel release string (display-only, from `uname(2)`); provides platform
///   context for the assessor.
/// - `architecture` — CPU architecture (display-only, from `uname(2)` machine field); provides
///   hardware context for the assessor (e.g., `"aarch64"`, `"x86_64"`); not a trust-relevant
///   assertion.
/// - `boot_id` — boot ID from `/proc/sys/kernel/random/boot_id` (display-only); used for
///   journald log correlation (CA-7); set to `"unavailable"` if the procfs read fails.
/// - `system_uuid` — system UUID from `/sys/class/dmi/id/product_uuid` (display-only); used for
///   cross-run correlation; set to `"unavailable"` if the sysfs read fails (non-UEFI systems,
///   permission errors).
/// - `os_name` — operating system display name (supplied by the calling binary); populated from
///   `PRETTY_NAME` in `/etc/os-release` when available, otherwise composed from `NAME` and
///   `VERSION_ID`; display-only, not a trust-relevant assertion.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: every header field is labelled and sourced; the header uniquely
///   identifies the host, tool, and collection time.
/// - **NIST SP 800-53 CA-7**: `assessed_at` timestamps each collection event.
/// - **NIST SP 800-53 SA-11**: `tool_name` and `tool_version` provide traceability to the
///   specific tool version that collected the evidence.
#[derive(Debug, Clone)]
pub struct HeaderContext {
    pub indicators: SecurityIndicators,
    pub tool_name: String,
    pub tool_version: String,
    pub assessed_at: String,
    pub hostname: String,
    pub kernel_version: String,
    pub architecture: String,
    pub boot_id: String,
    pub system_uuid: String,
    pub os_name: String,
}

// ---------------------------------------------------------------------------
// StyleHint
// ---------------------------------------------------------------------------

/// Visual emphasis hint for a data row value.
///
/// Maps to a foreground color in the theme. Callers use this to convey
/// semantic meaning (e.g., trust tier) without hard-coding color values.
///
/// ## Variants:
///
/// - `Normal` — default foreground (white).
/// - `Highlight` — cyan highlight for attention.
/// - `Dim` — dimmed for secondary information.
/// - `TrustGreen` — green; trust verified / positive security outcome.
/// - `TrustYellow` — yellow; trust degraded / advisory condition.
/// - `TrustRed` — red; trust failed / security concern.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: security state is typed, not free-form strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleHint {
    Normal,
    Highlight,
    Dim,
    TrustGreen,
    TrustYellow,
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
/// ## Variants:
///
/// - `Info` — informational; no action required.
/// - `Ok` — positive outcome; security posture is good.
/// - `Warn` — advisory; degraded or uncertain state; review recommended.
/// - `Error` — security concern or pipeline failure; action required.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: status must be visually unambiguous.
/// - **NIST SP 800-53 SI-5**: security alert processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    Info,
    Ok,
    Warn,
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
///
/// ## Fields:
///
/// - `level` — severity tier controlling the background color.
/// - `text` — short display text; must not contain sensitive data.
#[derive(Debug, Clone)]
pub struct StatusMessage {
    pub level: StatusLevel,
    pub text: String,
}

impl StatusMessage {
    /// Construct a new `StatusMessage`.
    #[must_use = "dropping the constructed StatusMessage discards the message before it is displayed"]
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
///
/// ## Fields:
///
/// - `label` — display label shown in the tab bar.
#[derive(Debug, Clone)]
pub struct TabDef {
    pub label: String,
}

impl TabDef {
    /// Construct a tab definition with the given label.
    #[must_use = "dropping the constructed TabDef discards the tab definition before it is registered"]
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
/// Phase 4). `Separator` inserts a blank line. `IndicatorRow` is a
/// multi-line layout for kernel security indicators: key + value on the first
/// line, description wrapped on subsequent lines (indented to align under the
/// value), and an implicit blank line after.
///
/// ## Column pairing convention
///
/// Use `TwoColumn` when two logically independent but equally-weighted facts
/// belong on the same conceptual row (e.g., distro name vs. kernel version).
/// Use `KeyValue` for single facts, or facts that have subordinate detail
/// rows below them. `TwoColumn` is not appropriate when one fact is
/// subordinate to the other.
/// Use `IndicatorRow` for kernel security indicators where the description
/// must appear directly beneath the key-value pair, indented to align under
/// the value text.
///
/// ## Trust boundary
///
/// Key and value strings are rendered verbatim. Callers must not include
/// security labels, credentials, or classified data in display strings
/// (NIST SP 800-53 SI-12).
///
/// ## Variants:
///
/// - `KeyValue { key, value, style_hint, highlight_key }` — standard single-column key-value
///   row. `key` is the field name or label; `value` is the field value; `style_hint` is the
///   visual hint applied to the value column; `highlight_key` — when `true`, the key label is
///   rendered with the header field style (bright cyan) instead of the dim-cyan data key style.
///   Use for summary rows where the key should match the header area — e.g., `"Kernel Version"`.
/// - `TwoColumn { left_key, left_value, left_hint, right_key, right_value, right_hint }` — two
///   key-value pairs rendered side-by-side; the panel area is split at the midpoint. Each half
///   uses half the standard key column width.
/// - `GroupTitle(String)` — section header rendered flush-left using the `group_title` theme
///   style. Items under the title should be indented by prepending `"  "` (two spaces) to the
///   `key` in subsequent rows. Group titles are visual organizers only — no semantic enforcement.
/// - `Separator` — blank separator line.
/// - `IndicatorRow { key, value, description, recommendation, contradiction, configured_line,
///   style_hint }` — multi-line indicator row for the Kernel Security tab. `key` is the
///   indicator name (may include leading spaces for group indentation). `value` is the live
///   kernel value string (already translated by the caller). `description` is a plain-language
///   description of what the indicator controls; omitted when empty. `recommendation` — when
///   `Some`, rendered as a dim italic `[ Recommended: <value> ]` line; `None` for hardened
///   indicators. `contradiction` — when `Some`, a `⚠` marker line is rendered; `BootDrift` uses
///   `TrustRed`, `EphemeralHotfix` uses `TrustYellow`, `SourceUnavailable` uses `Dim`; the `⚠`
///   symbol ensures visibility without relying on color (WCAG 1.4.1). `configured_line` — when
///   `Some`, a dim line in the format `"Configured: <raw> (from <source_file>)"`. `style_hint`
///   is applied to the value string. The key column width is computed dynamically from all
///   `IndicatorRow` entries. An implicit trailing blank line provides visual separation.
///   NIST SP 800-53 AU-3, CM-6, CA-7.
/// - `TableRow { col1, col2, col3, style_hint }` — fixed three-column table row. `col1` is the
///   evidence type label; `col2` is the source path or identifier; `col3` is the verification
///   outcome. `style_hint` is applied to `col3`. Column strings must not exceed fixed widths;
///   callers are responsible for truncation. NIST SP 800-53 AU-3.
/// - `TableHeader { col1, col2, col3 }` — table column header row rendered with bold key styling
///   across all columns. Emitted once per evidence group before the first `TableRow`.
///   NIST SP 800-53 AU-3.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: every data item is labelled; no ambiguous blobs.
/// - **NIST SP 800-53 SI-12**: key, value, and description strings are rendered verbatim;
///   callers must not include security labels, credentials, or classified data.
#[derive(Debug, Clone)]
pub enum DataRow {
    KeyValue {
        key: String,
        value: String,
        style_hint: StyleHint,
        highlight_key: bool,
    },

    TwoColumn {
        left_key: String,
        left_value: String,
        left_hint: StyleHint,
        right_key: String,
        right_value: String,
        right_hint: StyleHint,
    },

    GroupTitle(String),

    Separator,

    IndicatorRow {
        key: String,
        value: String,
        description: &'static str,
        recommendation: Option<&'static str>,
        contradiction: Option<ContradictionKind>,
        configured_line: Option<String>,
        style_hint: StyleHint,
    },

    TableRow {
        col1: String,
        col2: String,
        col3: String,
        style_hint: StyleHint,
    },

    TableHeader {
        col1: String,
        col2: String,
        col3: String,
    },
}

impl DataRow {
    /// Construct a `KeyValue` row with an explicit style hint.
    ///
    /// This is the primary constructor. Use `normal()` for unstyled rows.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled field from the audit card"]
    pub fn key_value(key: impl Into<String>, value: impl Into<String>, hint: StyleHint) -> Self {
        Self::KeyValue {
            key: key.into(),
            value: value.into(),
            style_hint: hint,
            highlight_key: false,
        }
    }

    /// Construct a `KeyValue` row with a highlighted (bright cyan) key label.
    ///
    /// Identical to [`DataRow::key_value`] but renders the key column using
    /// the header field style (bright cyan) instead of the dim-cyan data key
    /// style. Use for summary rows where the key should stand out visually to
    /// match the header area label styling — e.g., `"Kernel Version"` and
    /// `"Indicators"` in the kernel security pinned summary.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled field from the audit card"]
    pub fn key_value_highlighted(
        key: impl Into<String>,
        value: impl Into<String>,
        hint: StyleHint,
    ) -> Self {
        Self::KeyValue {
            key: key.into(),
            value: value.into(),
            style_hint: hint,
            highlight_key: true,
        }
    }

    /// Construct a `KeyValue` row with the given key, value, and style hint.
    ///
    /// Alias for [`DataRow::key_value`] — kept for call-site readability when
    /// the hint is already a named variable at the call site.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled field from the audit card"]
    pub fn new(key: impl Into<String>, value: impl Into<String>, hint: StyleHint) -> Self {
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

    /// Construct an `IndicatorRow` for a kernel security indicator.
    ///
    /// `key` is the indicator name (typically prefixed with two spaces for
    /// group indentation). `value` is the already-translated live value string.
    /// `description` is the plain-language security explanation; pass `""` to
    /// omit description lines. `hint` controls the value foreground color.
    ///
    /// The data panel computes the key column width dynamically from all
    /// `IndicatorRow` entries in the row list — callers do not need to specify
    /// or coordinate column widths.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled indicator from the audit card"]
    pub fn indicator_row(
        key: impl Into<String>,
        value: impl Into<String>,
        description: &'static str,
        hint: StyleHint,
    ) -> Self {
        Self::IndicatorRow {
            key: key.into(),
            value: value.into(),
            description,
            recommendation: None,
            contradiction: None,
            configured_line: None,
            style_hint: hint,
        }
    }

    /// Construct an `IndicatorRow` with an optional recommended-value annotation.
    ///
    /// When `recommendation` is `Some`, a dim italic `[ Recommended: ... ]`
    /// line is rendered below any configured-value line. Pass `None` for green
    /// (already hardened) indicators — no recommendation line is shown.
    ///
    /// NIST SP 800-53 CM-6 — remediation guidance accompanies each failing
    /// configuration setting.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled indicator from the audit card"]
    pub fn indicator_row_recommended(
        key: impl Into<String>,
        value: impl Into<String>,
        description: &'static str,
        recommendation: Option<&'static str>,
        hint: StyleHint,
    ) -> Self {
        Self::IndicatorRow {
            key: key.into(),
            value: value.into(),
            description,
            recommendation,
            contradiction: None,
            configured_line: None,
            style_hint: hint,
        }
    }

    /// Construct an `IndicatorRow` with contradiction and configured-value display.
    ///
    /// Extends `indicator_row_recommended` with two additional optional fields:
    ///
    /// - `contradiction` — when `Some`, a `⚠` marker line is rendered below
    ///   the description. `BootDrift` uses `TrustRed`; `EphemeralHotfix` uses
    ///   `TrustYellow`; `SourceUnavailable` uses `Dim`. The `⚠` symbol ensures
    ///   visibility without relying on color (WCAG 1.4.1 / NO_COLOR).
    /// - `configured_line` — when `Some`, a dim line showing the configured
    ///   value and its source file is rendered below the contradiction marker.
    ///
    /// Contradiction is rendered before the recommendation because it describes
    /// an active kernel/config disagreement — more urgent than a hardening gap.
    ///
    /// NIST SP 800-53 CA-7 — contradiction findings surface live/configured
    /// drift continuously so assessors cannot miss configuration management gaps.
    /// NIST SP 800-53 CM-6 — configured-value source attribution supports audit.
    #[must_use = "DataRow must be pushed into the row list; discarding it omits \
                  a labelled indicator from the audit card"]
    pub fn indicator_row_full(
        key: impl Into<String>,
        value: impl Into<String>,
        description: &'static str,
        recommendation: Option<&'static str>,
        contradiction: Option<ContradictionKind>,
        configured_line: Option<String>,
        hint: StyleHint,
    ) -> Self {
        Self::IndicatorRow {
            key: key.into(),
            value: value.into(),
            description,
            recommendation,
            contradiction,
            configured_line,
            style_hint: hint,
        }
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
// ColumnLayout
// ---------------------------------------------------------------------------

/// Layout mode for a data panel tab.
///
/// Controls whether the tab renders its data in a single full-width column
/// (the default) or in two independent side-by-side columns. Two-column mode
/// is intended for tabs that have logically distinct data groups of roughly
/// equal depth — it makes better use of terminal width and lets operators
/// compare related information without scrolling.
///
/// When `TwoColumn` is returned by [`AuditCardApp::column_layout`], the
/// data panel calls [`AuditCardApp::data_rows_left`] and
/// [`AuditCardApp::data_rows_right`] instead of `data_rows`. The two
/// column slices scroll together using the shared [`AuditCardState::scroll_offset`].
///
/// ## Variants:
///
/// - `Full` — single full-width column; default behavior, backward-compatible.
/// - `TwoColumn` — two independent vertical columns side-by-side, each 50% of the panel width;
///   left and right rows are supplied by separate trait methods.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: structured presentation ensures every labelled field remains
///   visible and legible regardless of layout mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnLayout {
    #[default]
    Full,
    TwoColumn,
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
/// ## Fields:
///
/// - `label` — display label for the left column (e.g., `"Version"`).
/// - `value` — display value for the right column; must not contain security labels, credentials,
///   or classified data (NIST SP 800-53 SI-12).
/// - `style_hint` — visual style hint applied to the value column.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: labelled header fields ensure every audit card is self-identifying;
///   report, host, subject, and supplemental context are all present on every rendered frame.
#[derive(Debug, Clone)]
pub struct HeaderField {
    pub label: String,
    pub value: String,
    pub style_hint: StyleHint,
}

impl HeaderField {
    /// Construct a `HeaderField` with an explicit style hint.
    ///
    /// `label` is shown in the left column; `value` in the right column.
    /// The `hint` controls the foreground color of the value.
    #[must_use = "HeaderField must be stored and returned from header_fields(); \
                  discarding it silently omits identification data from the audit card"]
    pub fn new(label: impl Into<String>, value: impl Into<String>, hint: StyleHint) -> Self {
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

    /// Pinned (non-scrollable) summary rows rendered at the top of the data panel.
    ///
    /// Pinned rows are rendered in a fixed area above the scrollable evidence
    /// chain. They remain visible at all times regardless of scroll position.
    /// The scrollable area shrinks by the number of lines occupied by pinned rows.
    ///
    /// Use this for per-tab summary information that an operator must always
    /// be able to see (e.g., trust tier, overall status) while scrolling
    /// through detailed evidence below.
    ///
    /// The default implementation returns an empty `Vec` — tabs without a
    /// summary section are unaffected. This method allocates only when a tab
    /// has pinned content.
    ///
    /// ## Design note
    ///
    /// Pinned rows are rendered using the same `build_row_line` path as scrollable
    /// rows. They do not support internal scrolling. Keep pinned content concise
    /// (fewer than 10 rows) so the scrollable area retains sufficient height.
    ///
    /// NIST SP 800-53 AU-3 — critical trust classification is always visible,
    /// ensuring the assessor cannot miss the top-level finding while reviewing
    /// detailed evidence.
    fn pinned_rows(&self, _tab_index: usize) -> Vec<DataRow> {
        Vec::new()
    }

    /// Column layout mode for the given tab index.
    ///
    /// When [`ColumnLayout::TwoColumn`] is returned, the data panel renders
    /// the tab in two independent side-by-side columns. The left column is
    /// populated by [`Self::data_rows_left`] and the right column by
    /// [`Self::data_rows_right`]. Both columns scroll together via the shared
    /// scroll offset in [`AuditCardState`].
    ///
    /// The default returns [`ColumnLayout::Full`] — the single-column layout
    /// used by all existing implementations. Override only for tabs that have
    /// two logically independent data groups of roughly equal depth.
    ///
    /// `data_rows()` is ignored when `column_layout()` returns `TwoColumn`.
    ///
    /// NIST SP 800-53 AU-3 — layout mode does not affect data completeness;
    /// every labelled field is always rendered regardless of column mode.
    fn column_layout(&self, _tab_index: usize) -> ColumnLayout {
        ColumnLayout::Full
    }

    /// Data rows for the left column when the tab uses two-column layout.
    ///
    /// Called by the data panel when [`Self::column_layout`] returns
    /// [`ColumnLayout::TwoColumn`] for this tab. Ignored in single-column mode.
    ///
    /// The default returns an empty `Vec` — single-column tabs are unaffected.
    ///
    /// NIST SP 800-53 AU-3 — left-column rows are labelled key-value fields;
    /// no data is rendered as ambiguous free-form text.
    fn data_rows_left(&self, _tab_index: usize) -> Vec<DataRow> {
        Vec::new()
    }

    /// Data rows for the right column when the tab uses two-column layout.
    ///
    /// Called by the data panel when [`Self::column_layout`] returns
    /// [`ColumnLayout::TwoColumn`] for this tab. Ignored in single-column mode.
    ///
    /// The default returns an empty `Vec` — single-column tabs are unaffected.
    ///
    /// NIST SP 800-53 AU-3 — right-column rows are labelled key-value fields;
    /// no data is rendered as ambiguous free-form text.
    fn data_rows_right(&self, _tab_index: usize) -> Vec<DataRow> {
        Vec::new()
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
/// ## Fields:
///
/// - `active_tab` — index of the currently displayed tab (0-based).
/// - `tab_count` (private) — total number of tabs; set at construction and does not change.
/// - `scroll_offset` — current vertical scroll offset into the data panel.
/// - `should_quit` — set to `true` by `Action::Quit`; the event loop exits when `true`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-2**: `should_quit` drives clean session termination.
pub struct AuditCardState {
    pub active_tab: usize,
    tab_count: usize,
    pub scroll_offset: usize,
    pub should_quit: bool,
}

impl AuditCardState {
    /// Construct a new state for a card with `tab_count` tabs.
    ///
    /// `tab_count` must be at least 1. If 0 is passed it is clamped to 1.
    #[must_use = "dropping the constructed AuditCardState discards the initialized card state"]
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
            Action::Refresh
            | Action::DialogConfirm
            | Action::DialogCancel
            | Action::DialogToggleFocus
            | Action::ShowHelp
            // ViewerApp and ConfigApp actions — not relevant to AuditCardState.
            // ViewerApp: Expand, Collapse, Search, Back, PanelSwitch.
            // ConfigApp: Save, Discard, ToggleEdit.
            // These variants exist in Action for the shared keymap; AuditCardState
            // ignores them rather than forwarding to unused code paths.
            | Action::Expand
            | Action::Collapse
            | Action::Search
            | Action::Back
            | Action::PanelSwitch
            | Action::Save
            | Action::Discard
            | Action::ToggleEdit => {
                // These actions have no effect on AuditCardState itself.
                //
                // Refresh is application-defined — callers use this signal to
                // re-run detection; AuditCardState carries no refresh behavior.
                //
                // Dialog actions are handled by the calling binary's event loop.
                // The caller owns Option<DialogState> and mutates it directly
                // (response, focused). AuditCardState has no dialog lifecycle
                // authority — there is no implicit global modal state.
                //
                // ShowHelp is handled by the calling binary's event loop.
                // The binary creates a DialogState::info(...) with tab-specific
                // help text and renders it via render_dialog. AuditCardState
                // has no help-text authority — callers supply the content.
                //
                // NIST SP 800-53 AC-2 — explicit lifecycle; no hidden modal state.
            }
        }
    }

    /// Reset scroll offset to zero (e.g., after a data refresh).
    pub const fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
    }
}
