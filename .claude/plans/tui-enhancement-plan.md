# TUI Enhancement Plan — Audit Card Template

**Date:** 2026-03-15
**Author:** rust-developer agent
**Status:** Phases 3, 5, 6, 7, 8 COMPLETE (2026-03-15). Phases 1, 2, 4 not started.
**ROADMAP Goals:** G5 (Security Tools), G8 (High-Assurance Patterns)
**Input:** `.claude/jamies_brain/tui-feedback.md`
**Display grouping reference:** `.claude/references/capability-matrix-domains.md` (Jamie's 7-domain capability matrix)

---

## Executive Summary

The `umrs-tui` library crate (at `components/rusty-gadgets/umrs-tui/`) needs seven
enhancements to make audit cards function as structured security-posture reports. This plan
phases all seven changes following Jamie's specified implementation order. Each phase
documents affected files, type-level API changes, backward compatibility impact, and
required test coverage.

The security-auditor agent is called out as a primary consumer of these interfaces.
This plan is written to provide enough design detail for the auditor to evaluate the security
properties of each change before implementation begins.

---

## Current Architecture Summary

Before reading the phases, reviewers should understand the current module boundaries:

| Module | Role |
|---|---|
| `app.rs` | `AuditCardApp` trait, `AuditCardState`, `DataRow`, `StyleHint`, `StatusLevel`, `StatusMessage`, `TabDef` |
| `header.rs` | Renders report name, hostname, subject inside a bordered block. No security indicator data. |
| `theme.rs` | All `ratatui::style::Style` values for every panel element. |
| `layout.rs` | Master render function. Splits terminal into header row / tab bar / data panel / status bar. |
| `data_panel.rs` | Single vertical stream of `DataRow` entries. Fixed key column width (20 chars). |
| `tabs.rs`, `status_bar.rs`, `keymap.rs` | Self-contained; minor or no changes in this plan. |

Binary consumers today: `main.rs` (OS detect TUI), `bin/file_stat.rs` (file stat TUI).
Both construct their own `AuditCardApp` impl using library types without extension hooks.

### Trust Boundary Note

The library renders data supplied by the caller. The template itself has no authority to make
security decisions. Its job is to render data faithfully and ensure that security state is
represented as typed enum variants — never as raw strings that could be fabricated or
misread. This boundary is preserved in all phases below.

---

## Phase 1 — Enhance Header with Security Indicators

### Goal

Add a pre-populated security indicator row to the header. Indicators cover:
SELinux status, FIPS mode, active LSM, kernel lockdown mode, and secure boot state.

### Design

**New type in `app.rs`:**

```rust
/// Pre-populated security posture indicators shown in the header.
///
/// The template populates this at startup from `umrs-platform` kattrs
/// where data is available. Fields that cannot be read (e.g., because
/// the kernel node does not exist) are represented as `Unavailable`.
///
/// NIST SP 800-53 SI-7, CM-6 — kernel-sourced posture flags are displayed
/// without interpretation; the caller may not override values read from
/// the kernel.
#[derive(Debug, Clone)]
pub struct SecurityIndicators {
    pub selinux_status: IndicatorValue,
    pub fips_mode:      IndicatorValue,
    pub active_lsm:     IndicatorValue,
    pub lockdown_mode:  IndicatorValue,
    pub secure_boot:    IndicatorValue,
}

/// The value of a single security posture indicator.
///
/// NIST SP 800-53 SI-3, CM-6 — absence of a readable value is a distinct
/// state (Unavailable), not silently collapsed into a default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndicatorValue {
    /// Kernel/platform value successfully read.
    Active(String),
    /// Indicator is not active or explicitly disabled.
    Inactive(String),
    /// Kernel node could not be read (permission, not present, etc.).
    Unavailable,
}
```

**Population function in a new module `indicators.rs`:**

A standalone function `fn read_security_indicators() -> SecurityIndicators` reads from
`umrs-platform` kattrs using the established `SecureReader` pipeline. Each indicator maps
to a known kernel attribute path. Fields that fail to read return `IndicatorValue::Unavailable`
(fail-closed — do not guess or default to "active").

Indicator sources:
- `selinux_status` — `/sys/fs/selinux/enforce` (kattr, already in `umrs-platform`)
- `fips_mode` — `/proc/sys/crypto/fips_enabled` (kattr, `ProcFips`)
- `active_lsm` — `/sys/kernel/security/lsm` (sysfs text, may need new kattr)
- `lockdown_mode` — `/sys/kernel/security/lockdown` (sysfs text, may need new kattr)
- `secure_boot` — `/sys/firmware/efi/efivars/SecureBoot-…` or `/proc/sys/kernel/secure_boot`
  (platform-specific; stub if node not present)

If a kattr is not yet implemented in `umrs-platform`, return `IndicatorValue::Unavailable`
for that slot. **Do not** read the raw file with `File::open` — all reads must go through
`SecureReader`. If the path does not yet exist in `umrs-platform`, stub the field with
`Unavailable` and file a follow-up task.

**`header.rs` changes — two-column readable layout:**

`render_header` gains an additional argument: `indicators: &SecurityIndicators`.

**DESIGN RULE (Jamie, 2026-03-15):** Do NOT use the compressed bracket format:
```
NO: [SEL:enforcing] [FIPS:active] [LSM:?] [LKD:none] [SB:?]
```
This is unreadable and operator-hostile.

**Instead:** Security indicators are rendered as full key-value lines grouped with
related information, using a two-column layout where the right side carries additional
context. Like information is grouped for at-a-glance comprehension.

**Terminology (security-auditor review, 2026-03-15 — NIST SP 800-53A / OSCAL aligned):**
- "Report" → **Assessment** — SP 800-53A normative term; "Report" is informal
- "Subject" → **Scope** — SAR Section 2.3 term; assessors recognize it immediately
- "Checked" → **Assessed** — consistent with SP 800-53A language
- Display labels may be terse; JSON keys use normative OSCAL terms (see JSON section)

**Mandatory header fields (every tool, every run):**

| Display Label | JSON Key | Source | Purpose |
|---|---|---|---|
| Assessment | `assessment_type` | Caller-supplied | Names what was assessed |
| Scope | `assessment_object` | Caller-supplied | What was examined (SP 800-53A) |
| Assessed | `assessed_at` | System clock (ISO-8601) | CA-7 timestamped checks |
| Tool | `tool.name` + `tool.version` | Compile-time constants | SA-11 traceability |
| Host | `hostname` | System | Identifies assessed system |
| Boot ID | `boot_id` | `/proc/sys/kernel/random/boot_id` | Journald correlation, CA-7 |
| System ID | `system_uuid` | `/sys/class/dmi/id/product_uuid` | Cross-run correlation |
| Kernel | `kernel_version` | `uname` | Platform context |
| SELinux | `selinux` | kattr | Enforcement posture |
| FIPS | `fips_mode` | kattr | Crypto compliance |
| LSM | `active_lsm` | kattr | Security framework |
| Lockdown | `lockdown_mode` | kattr | Kernel lockdown |

**Target layout (6 rows — must fit in the header alongside the wizard art):**

```
Assessment : OS Detection                           Boot ID   : a3f7c2d1-...
Scope      : Platform Identity and Integrity        Assessed  : 2026-03-15 14:32
Host       : goldeneye                              Kernel    : 6.12.0-211.el10
Tool       : umrs-os-detect 0.3.1                   System ID : <UUID>
SELinux    : Enabled (Enforcing) / Targeted         LSM       : selinux
FIPS       : Active                                 Lockdown  : integrity
```

**Design constraints:**
- Header must uniquely identify this system and the type of assessment
- At a glance, it should give easily understood information
- Like information grouped together, easy to read
- Right column is optional — **be width-aware**: the file-stat report launched by
  umrs-ls will be narrower. When terminal width is insufficient, fall back to
  single-column layout. Minimum viable header = left column only
- All header fields must map cleanly to structured JSON for future `--json` report export
- 6 rows is the target — do not exceed this without Jamie's approval

**JSON export structure (OSCAL-aligned — future `--json` output):**

```json
{
  "assessment_type": "OS Detection",
  "assessment_object": "Platform Identity and Integrity",
  "assessed_at": "2026-03-15T14:32:00Z",
  "tool": { "name": "umrs-os-detect", "version": "0.3.1" },
  "system": {
    "hostname": "goldeneye",
    "system_uuid": "...",
    "boot_id": "a3f7c2d1-...",
    "kernel_version": "6.12.0-211.el10"
  },
  "security_posture": {
    "selinux": "Enabled (Enforcing) / Targeted",
    "fips_mode": "Active",
    "active_lsm": "selinux",
    "lockdown_mode": "integrity"
  }
}
```

Each indicator value is styled via a new theme field (see Phase 5 / theme additions).
A direct `IndicatorValue → Style` mapping is used to color active vs. inactive vs.
unavailable states. Full words ("Enabled", "Active", "integrity") — no abbreviations.

**Header height adjustment:**

`layout.rs` defines `HEADER_HEIGHT` as `WIZARD_SMALL.height + 2`. With the added indicator
row, the header needs one additional line. Change to `WIZARD_SMALL.height + 3` or make the
header height dynamic (preferred: keep it a constant, set to `WIZARD_SMALL.height + 3`).

### Files Modified

- `src/app.rs` — add `SecurityIndicators`, `IndicatorValue`
- `src/indicators.rs` — new module; `read_security_indicators()` function
- `src/header.rs` — `render_header` gains `indicators: &SecurityIndicators` arg
- `src/layout.rs` — `render_audit_card` gains `indicators: &SecurityIndicators` arg;
  passes it through; updates `HEADER_HEIGHT` constant
- `src/theme.rs` — new style fields for indicator active / inactive / unavailable colors
- `src/lib.rs` — add `pub mod indicators`; re-export `SecurityIndicators`, `IndicatorValue`

### Backward Compatibility

`render_audit_card`'s signature gains one parameter. All existing callers (`main.rs`,
`bin/file_stat.rs`, and the test mock) must be updated. This is a **breaking change within
the workspace**. Since all consumers are in the same workspace and there are no external
crate users yet, this is acceptable — update all callers in the same PR.

### Tests Needed

- `tests/indicators_tests.rs` (new file):
  - `security_indicators_all_unavailable()` — verify `SecurityIndicators::default()` or a
    stub returns `Unavailable` for all fields when kernel nodes are not present (testable
    with a mock that skips actual reads)
  - `indicator_value_variants_are_distinct()` — `Active != Inactive != Unavailable`
  - `indicator_value_active_contains_string()` — round-trip the inner string

---

## Phase 2 — Header Extensibility (Main.rs Supplemental Fields)

### Goal

Allow each binary (`main.rs`, `file_stat.rs`, future binaries) to supply additional
key-value pairs displayed in the header, below the fixed security indicators row.

### Design

**New type in `app.rs`:**

```rust
/// A single supplemental header field supplied by the calling binary.
///
/// Supplements the fixed fields (report name, host, subject, security indicators).
/// The binary supplies these to communicate card-specific identity information
/// without modifying library code.
///
/// NIST SP 800-53 AU-3 — every field in the header is labelled.
#[derive(Debug, Clone)]
pub struct HeaderField {
    /// Short label (e.g., "Target", "Mode", "Policy").
    pub label: String,
    /// Display value. Must not contain security labels or credentials.
    pub value: String,
    /// Visual hint for the value column.
    pub style_hint: StyleHint,
}

impl HeaderField {
    #[must_use = "HeaderField must be stored and passed to the render function"]
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        hint: StyleHint,
    ) -> Self { ... }

    #[must_use = "HeaderField must be stored and passed to the render function"]
    pub fn normal(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(label, value, StyleHint::Normal)
    }
}
```

**Trait change in `app.rs`:**

Add a new method to `AuditCardApp` with a default implementation returning an empty slice:

```rust
/// Supplemental header fields supplied by this specific card.
///
/// Rendered below the security indicator row. Return an empty slice (default)
/// if no supplemental fields are needed. Fields are rendered in order.
///
/// NIST SP 800-53 AU-3.
fn header_fields(&self) -> &[HeaderField] {
    &[]
}
```

Because this has a default implementation, the change is **backward compatible** — existing
`AuditCardApp` impls continue to compile without modification.

**`header.rs` changes:**

`render_header` renders `app.header_fields()` below the security indicators row. Each field
uses the same key/value layout as the existing report/host/subject rows. If there are many
supplemental fields and the header block would overflow its height, **truncate with a `…`
indicator** rather than wrapping or panicking (fail gracefully).

**`main.rs` example usage:**

```rust
fn header_fields(&self) -> &[HeaderField] {
    &self.extra_header_fields  // pre-built Vec<HeaderField>
}
```

### Files Modified

- `src/app.rs` — add `HeaderField`; add `header_fields()` method with default `&[]`
- `src/header.rs` — render supplemental fields from `app.header_fields()`
- `src/lib.rs` — re-export `HeaderField`

### Backward Compatibility

No breaking changes. Default implementation returns `&[]`. Existing impls unaffected.

### Tests Needed

- `tests/trait_impl_tests.rs` (extend existing file):
  - `header_fields_default_returns_empty_slice()` — mock impl without override returns `&[]`
  - `header_fields_override_returns_custom_fields()` — mock impl overrides and returns 2 fields
  - `header_field_new_sets_all_fields()` — round-trip test
  - `header_field_normal_sets_normal_hint()` — convenience constructor sets `StyleHint::Normal`

---

## Phase 3 — Two-Column Layout in Dynamic Data Area

**Status: COMPLETE (2026-03-15)**

### Goal

The data panel currently renders a single vertical stream. Add left/right column support
so data can be placed side-by-side, using horizontal terminal space efficiently.

### Design

**`DataRow` becomes an enum:**

This is the most significant API change in the plan. The current `DataRow` struct is
replaced by a `DataRow` enum:

```rust
/// A single content item in the data panel.
///
/// `KeyValue` is the standard entry (key: value). `TwoColumn` renders two
/// independent key-value pairs side-by-side in the left and right half of
/// the panel area. `GroupTitle` adds a labelled section header (Phase 4).
/// `Separator` inserts a blank line.
///
/// NIST SP 800-53 AU-3 — every data item is labelled.
#[derive(Debug, Clone)]
pub enum DataRow {
    /// Standard single-column key-value row.
    KeyValue {
        key: String,
        value: String,
        style_hint: StyleHint,
    },
    /// Two key-value pairs rendered side-by-side.
    TwoColumn {
        left_key: String,
        left_value: String,
        left_hint: StyleHint,
        right_key: String,
        right_value: String,
        right_hint: StyleHint,
    },
    /// Group section title (Phase 4).
    GroupTitle(String),
    /// Blank separator line.
    Separator,
}
```

**Reasoning for enum approach over `add_row(Column::Left, ...)` API:**

The feedback document sketched a builder-style column API. However, making `DataRow` an enum
is superior for this codebase because:

1. `data_rows()` returns `Vec<DataRow>` — the trait method is already defined and object-safe.
   A builder that pairs rows implicitly across separate calls would require mutable state in
   the render function, breaking the current clean model.
2. `TwoColumn` makes it explicit and self-contained at the call site — no pairing ambiguity.
3. Keeps `data_rows()` a pure function with no side effects.
4. All variants are matchable — the security-auditor can inspect exactly what was rendered.

**Constructors on `DataRow`:**

All current constructor functions become associated functions on the enum:

```rust
impl DataRow {
    #[must_use = "..."]
    pub fn key_value(key: impl Into<String>, value: impl Into<String>, hint: StyleHint) -> Self
    pub fn normal(key: impl Into<String>, value: impl Into<String>) -> Self
    pub fn separator() -> Self
    pub fn two_column(
        left_key: impl Into<String>, left_value: impl Into<String>, left_hint: StyleHint,
        right_key: impl Into<String>, right_value: impl Into<String>, right_hint: StyleHint,
    ) -> Self
}
```

**`data_panel.rs` changes:**

`render_data_panel` pattern-matches on `DataRow` variants:

- `KeyValue` → current single-column rendering (unchanged)
- `TwoColumn` → split the panel area horizontally at the midpoint; render left half and
  right half independently using the same key/value style. Key column width in each half
  is half of `KEY_COL_WIDTH`.
- `GroupTitle` → Phase 4
- `Separator` → blank line (current behavior)

The scrollbar logic remains unchanged — total row count drives scroll math.

### Files Modified

- `src/app.rs` — `DataRow` becomes an enum; all constructors updated
- `src/data_panel.rs` — match on `DataRow` variants; add two-column rendering path
- `src/lib.rs` — no change to re-exports (enum variants are accessed via `DataRow::…`)
- All callers: `main.rs`, `bin/file_stat.rs` — update `DataRow::new(...)` to `DataRow::key_value(...)`
  or `DataRow::normal(...)` (same names, same semantics — mechanical rename)

### Backward Compatibility

**This is a breaking API change.** The `DataRow` struct fields (`key`, `value`, `style_hint`)
are no longer directly accessible — they are wrapped in an enum variant. All existing call
sites must migrate to the new constructors. Because all consumers are in-workspace, this is
managed in one pass.

**Migration path for existing call sites:**

- `DataRow::new(k, v, h)` → `DataRow::key_value(k, v, h)`
- `DataRow::normal(k, v)` → `DataRow::normal(k, v)` (same name, behavior preserved)
- `DataRow::separator()` → `DataRow::separator()` (unchanged)
- Direct field access `row.key`, `row.value`, `row.style_hint` in `data_panel.rs` → match arm

Existing tests in `tests/data_types_tests.rs` must be updated to use the new constructors
and match the enum structure. Test *coverage* must remain equivalent — no tests dropped.

### Tests Needed

- Update `tests/data_types_tests.rs`:
  - Rename `data_row_new_sets_key_value_and_hint()` → tests `DataRow::key_value(...)`
  - All existing assertions remain valid with field access updated for enum match
- Add `data_row_two_column_sets_all_fields()` — verify all 6 fields round-trip correctly
- Add `data_row_separator_variant_is_separator()` — `matches!(DataRow::separator(), DataRow::Separator)`

---

## Phase 4 — Group Title Support

### Goal

Organize related posture data with labeled section headers. Flush left, items indented one
space, text-only styling (no boxes, no ASCII decoration).

### Design

`DataRow::GroupTitle(String)` is already added in Phase 3 as a variant. Phase 4 wires it up.

**`DataRow` constructor:**

```rust
impl DataRow {
    #[must_use = "..."]
    pub fn group_title(title: impl Into<String>) -> Self {
        DataRow::GroupTitle(title.into())
    }
}
```

**`data_panel.rs` changes:**

`GroupTitle` arm in the match renders the title string using `theme.group_title` style.
No border, no box — a single styled `Span`. The title takes one display row.

**Indentation of items under a group title:**

Items indented with one leading space is a *caller convention*, not enforced by the library.
The caller prepends `" "` to the key string in `DataRow::key_value(" key", ...)`. This keeps
the library simple and policy-free. The doc comment on `GroupTitle` must document this
convention explicitly.

Alternative considered: automatic indentation tracking in the render function. Rejected
because it requires stateful rendering (the render function would need to know whether the
current row follows a group title). Stateless rendering is simpler and more auditable.

**Theme additions (Phase 5 handles the theme module; these fields are needed by Phase 4):**

```rust
pub struct Theme {
    // ... existing fields ...
    /// Group section title text (bold, subtle color — see default).
    pub group_title: Style,
    /// Security indicator active state (green).
    pub indicator_active: Style,
    /// Security indicator inactive state (dim).
    pub indicator_inactive: Style,
    /// Security indicator unavailable (dark gray).
    pub indicator_unavailable: Style,
}
```

### Files Modified

- `src/app.rs` — add `DataRow::group_title()` constructor
- `src/data_panel.rs` — add `GroupTitle` match arm
- `src/theme.rs` — add `group_title`, `indicator_*` style fields (see Phase 5)
- `src/lib.rs` — no changes

### Tests Needed

- `tests/data_types_tests.rs`:
  - `data_row_group_title_stores_string()` — `DataRow::group_title("SELINUX")` has expected inner string

---

## Phase 5 — Theme-Based Styling for Group Titles and Indicators

**Status: COMPLETE (2026-03-15)**

### Goal

All visual styling for the new elements (group titles, security indicators) is defined in
`theme.rs`. No inline color values in `header.rs` or `data_panel.rs`.

### Design

Complete the `Theme` struct with the fields called out in Phase 4:

```rust
impl Default for Theme {
    fn default() -> Self {
        Self {
            // ... existing defaults ...

            // Group titles: bold white (subtle, no color splash).
            group_title: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),

            // Security indicators
            indicator_active: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            indicator_inactive: Style::default()
                .fg(Color::DarkGray),
            indicator_unavailable: Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        }
    }
}
```

**`IndicatorValue → Style` helper:**

Add a free function or method alongside `style_hint_color` and `status_bg_color`:

```rust
/// Map an [`IndicatorValue`] to the appropriate theme style.
#[must_use = "style is used for rendering; discarding it has no effect"]
pub fn indicator_style<'a>(value: &IndicatorValue, theme: &'a Theme) -> &'a Style {
    match value {
        IndicatorValue::Active(_) => &theme.indicator_active,
        IndicatorValue::Inactive(_) => &theme.indicator_inactive,
        IndicatorValue::Unavailable => &theme.indicator_unavailable,
    }
}
```

### Files Modified

- `src/theme.rs` — add `group_title`, `indicator_active`, `indicator_inactive`,
  `indicator_unavailable` fields and defaults; add `indicator_style()` helper

### Tests Needed

- `tests/theme_tests.rs` (extend existing):
  - `theme_default_has_group_title_style()` — `Theme::default().group_title` is not the `Default` zero style
  - `indicator_style_active_returns_active_style()`, `_inactive_`, `_unavailable_` — coverage
    of the new `indicator_style()` function path

---

## Phase 6 — Evidence Tab Grouping

**Status: COMPLETE (2026-03-15)**

### Goal

The evidence chain tab displays `EvidenceRecord` entries grouped by subsystem (source kind),
in a scrollable table-formatted layout. Currently in `main.rs`, evidence is rendered as a
flat list of abbreviated key-value rows.

### Design

This is primarily a **binary-level change** in `main.rs`, not a library-level change.

**Library addition — table row type:**

Add a new `DataRow` variant for table-formatted rows. This gives the evidence display a
distinct visual identity from plain key-value data:

```rust
/// A fixed three-column table row for structured evidence display.
///
/// Used for evidence chains and similar structured data. Columns are
/// left-aligned; widths are fixed by the theme/layout.
///
/// NIST SP 800-53 AU-3 — evidence records are labelled and structured.
TwoColumnTable {
    col1: String,
    col2: String,
    col3: String,
    style_hint: StyleHint,
}
```

And a header-row variant:

```rust
/// A table column header row (bold key style across all columns).
TableHeader {
    col1: String,
    col2: String,
    col3: String,
}
```

Constructors:

```rust
impl DataRow {
    pub fn table_row(c1: impl Into<String>, c2: impl Into<String>,
                     c3: impl Into<String>, hint: StyleHint) -> Self
    pub fn table_header(c1: impl Into<String>, c2: impl Into<String>,
                        c3: impl Into<String>) -> Self
}
```

**Column widths for evidence table:**

Evidence table columns: `Evidence Type` (20 chars), `Source` (24 chars), `Verification`
(remainder). These are constants in `data_panel.rs`. The render path for `TwoColumnTable`
formats each column with fixed width and clips to avoid overflow.

**`main.rs` `build_trust_rows()` refactor:**

Replace the current flat evidence rendering with a grouped structure:

1. Group `EvidenceRecord` entries by `SourceKind`.
2. For each group: emit `DataRow::group_title(group_name)`, then a `DataRow::table_header(...)`,
   then one `DataRow::table_row(...)` per record in the group.
3. Emit `DataRow::separator()` between groups.

Grouping is done by building a `BTreeMap<SourceKind, Vec<&EvidenceRecord>>` (or equivalent
deterministic ordering) before constructing the row list. `SourceKind` must implement `Ord`
or records are grouped by a manually determined display order (not sorted dynamically).

**Evidence status indicators:**

Each evidence row uses a minimal status indicator in the verification column. Keep it
simple — content over decoration:

- `✓ verified` — green (`StyleHint::TrustGreen`) — evidence checked and valid
- `✓ validated` — green — secondary confirmation passed
- `✗ failed` — red (`StyleHint::TrustRed`) — verification failed
- `✗ unsigned` — red — expected signature missing
- `? inconclusive` — yellow (`StyleHint::TrustYellow`) — could not determine

The existing `StyleHint` enum already has `TrustGreen`, `TrustYellow`, `TrustRed` —
use these directly. Do not add new variants for evidence styling. The checkmark/X
characters come from `umrs_core::console::symbols::icons` if available, or use
Unicode `✓` (U+2713) and `✗` (U+2717) directly.

**Design rule:** Minimal color, minimal decoration. The content (what was checked,
what source, what result) carries the message. Color reinforces — it does not replace.

**Display field length constraints:**

Paths are truncated to column width. `parse_ok` maps to `"✓ verified"` / `"✗ FAIL"`.
No security labels or raw kernel values in the display strings — NIST SP 800-53 SI-12.

### Files Modified

- `src/app.rs` — add `TwoColumnTable`, `TableHeader` variants and constructors
- `src/data_panel.rs` — add match arms for the new variants; add column-width constants
- `src/main.rs` — refactor `build_trust_rows()` to use grouped, table-formatted rows
- `src/lib.rs` — no change

### Tests Needed

- `tests/data_types_tests.rs`:
  - `data_row_table_row_stores_three_columns()` — round-trip all three column strings
  - `data_row_table_header_stores_three_columns()` — same for header variant

---

## Phase 7 — Placeholder Kernel Security Tab **[COMPLETE 2026-03-15]**

### Goal

Add a second tab to `main.rs` (OS detect TUI) for kernel security configuration. Content
is placeholder data for now; the tab structure must be correct and extensible.

### Design

This is a **binary-level change** in `main.rs` only. No library changes required.

**Tab definition change in `main.rs`:**

```rust
let tabs = vec![
    TabDef::new("OS Information"),
    TabDef::new("Trust / Evidence"),
    TabDef::new("Kernel Security"),  // new
];
```

**`AuditCardState` construction:**

```rust
let mut state = AuditCardState::new(app.tabs().len()); // becomes 3
```

**`data_rows()` match arm for tab 2:**

```rust
2 => self.kernel_security_rows.clone(),
```

**`build_kernel_security_rows()` in `main.rs`:**

A new private function that builds placeholder rows using `DataRow::group_title` and
`DataRow::key_value`. Initial content:

```
KERNEL LOCKDOWN
 mode            : (not yet probed)

MODULE LOADING
 restrictions    : (not yet probed)

FIPS STATE
 fips_enabled    : (from header indicators)

SECURE BOOT
 state           : (not yet probed)
```

Where a value is available from `SecurityIndicators` (Phase 1), it is populated. Where it
is not yet probed, the string `"(not yet probed)"` is used with `StyleHint::Dim`.

**`OsDetectApp` struct gains:**

```rust
struct OsDetectApp {
    tabs: Vec<TabDef>,
    os_info_rows: Vec<DataRow>,
    trust_rows: Vec<DataRow>,
    kernel_security_rows: Vec<DataRow>,  // new
    status: StatusMessage,
}
```

### Files Modified

- `src/main.rs` — add tab 3, `build_kernel_security_rows()`, update `from_result()` and
  `from_error()`, update `data_rows()` match

### Tests Needed

No new library tests. Consider adding a test in `tests/trait_impl_tests.rs` that the
mock `AuditCardApp` handles a third tab without panicking — extend the mock to 3 tabs.

---

## Phase 8 — Dialog API **[COMPLETE 2026-03-15]**

### Goal

Provide a centered modal dialog overlay callable from `main.rs`. Four modes:
Info, Error, SecurityWarning, Confirm. Focus management for two-button dialogs.

### Design

The dialog is a self-contained render function plus a state type — no new trait methods.

```rust
/// Mutable state for an active dialog.
///
/// Created by the caller when a dialog should appear. Passed into
/// `render_dialog()` alongside the normal `AuditCardState`.
/// `response` is `None` until the user confirms or cancels.
///
/// NIST SP 800-53 AC-2 — dialog state lifecycle is explicit; there is no
/// implicit global modal state.
pub struct DialogState {
    /// The dialog is visible when this is true.
    pub visible: bool,
    /// User response: None = pending, Some(true) = confirmed, Some(false) = cancelled.
    pub response: Option<bool>,
    /// The message displayed to the user.
    pub message: String,
    /// The mode controlling button labels and styling.
    pub mode: DialogMode,
    /// Currently focused button (for Y/N and Cancel/OK modes).
    pub focused: DialogFocus,
}

/// Visual and interaction mode for a dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogMode {
    /// Informational — single Esc/Enter to dismiss; response always `Some(true)`.
    Info,
    /// Error condition — single dismiss; styled with error background.
    Error,
    /// Security-serious — two-button (Cancel / OK); security-warning styling.
    SecurityWarning,
    /// Confirmation — two-button (No / Yes).
    Confirm,
}

/// Which button has focus in a two-button dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogFocus {
    Primary,    // "OK" / "Yes"
    Secondary,  // "Cancel" / "No"
}
```

**New render function in a new module `dialog.rs`:**

```rust
/// Render a modal dialog box centered in `area`.
///
/// The dialog is rendered on top of the existing content — the caller
/// must call this *after* `render_audit_card()` so it overlays the card.
/// Uses ratatui `Clear` widget to blank the region before drawing.
///
/// NIST SP 800-53 AC-2 — dialog must be dismissed explicitly; there is
/// no timeout or auto-dismiss behavior.
pub fn render_dialog(
    frame: &mut Frame,
    area: Rect,
    state: &DialogState,
    theme: &Theme,
)
```

**Width calculation:**

Dialog width = `message.len().max(40).min(area.width as usize - 8)` chars, horizontally
centered. This ensures it is never stubby (minimum 40) and never wider than the terminal.

**`Action` additions for dialog interaction:**

Add to `keymap.rs`:

```rust
pub enum Action {
    // ... existing ...
    /// Confirm the active dialog (Enter / Y).
    DialogConfirm,
    /// Cancel or dismiss the active dialog (Esc / N).
    DialogCancel,
    /// Move focus within a two-button dialog (Tab / Left / Right).
    DialogToggleFocus,
}
```

The calling binary decides which keys invoke these actions. The library provides the
variants; the caller binds them.

**`AuditCardState` gains `dialog: Option<DialogState>`:**

Or the caller manages `DialogState` independently — preferred, because it avoids coupling
`AuditCardState` to dialog lifetime. The calling binary owns `Option<DialogState>` and
passes it to `render_dialog()`. The library does not manage the presence or absence of
dialogs.

**Theme additions for dialogs:**

```rust
pub struct Theme {
    // ...
    pub dialog_info_border: Style,
    pub dialog_error_border: Style,
    pub dialog_security_border: Style,
    pub dialog_button_focused: Style,
    pub dialog_button_unfocused: Style,
    pub dialog_title: Style,
    pub dialog_message: Style,
}
```

### Files Modified

- `src/dialog.rs` — new module; `DialogState`, `DialogMode`, `DialogFocus`, `render_dialog()`
- `src/app.rs` — add `DialogState`, `DialogMode`, `DialogFocus` types (or re-export from dialog)
- `src/keymap.rs` — add `DialogConfirm`, `DialogCancel`, `DialogToggleFocus` action variants
- `src/theme.rs` — add `dialog_info_border`, `dialog_error_border`, `dialog_security_border`,
  `dialog_button_focused`, `dialog_button_unfocused`, `dialog_title`, `dialog_message` fields
- `src/lib.rs` — add `pub mod dialog`; re-export dialog types

### Backward Compatibility

Additive only. No existing signatures change. The caller opts in by creating a `DialogState`
and calling `render_dialog()` after `render_audit_card()`.

### Tests Needed

- `tests/dialog_tests.rs` (new file):
  - `dialog_state_default_is_not_visible()` — initial state has `visible: false`
  - `dialog_mode_variants_are_distinct()` — all four modes compare differently
  - `dialog_focus_toggle()` — Primary → Secondary → Primary
  - `dialog_info_mode_response_is_always_true()` — Info dismiss sets `Some(true)`
  - `dialog_confirm_mode_starts_pending()` — response is `None` until user acts
  - `dialog_min_width_enforced()` — width never below 40 chars

---

## Cross-Cutting Constraints

### No `unsafe`, No `unwrap()`

All new code must comply with `#![forbid(unsafe_code)]` and `#![deny(clippy::unwrap_used)]`.
New I/O paths for indicator reads (Phase 1) must use `?` or explicit `match`.

### Clippy Pedantic + Nursery Clean

`cargo xtask clippy` must pass with zero warnings after each phase. Particular attention:
- Any new `usize as u16` cast in `layout.rs` needs `#[allow(clippy::cast_possible_truncation)]`
  with a comment explaining the value is bounded.
- New `const fn` candidates (enum helper functions) must be `const`.

### Compliance Annotations

All new public types and functions need NIST/RTB annotations. Specifically:
- `SecurityIndicators`, `IndicatorValue` → `NIST SP 800-53 SI-7, CM-6`
- `HeaderField` → `NIST SP 800-53 AU-3`
- `DataRow::GroupTitle`, `TwoColumn`, `TwoColumnTable`, `TableHeader` → `NIST SP 800-53 AU-3`
- `DialogState`, `DialogMode` → `NIST SP 800-53 AC-2`
- `indicators.rs` module → `NIST SP 800-53 SI-7, CM-6, CM-7`

### All Existing Tests Must Pass

No test may be deleted. Tests in `data_types_tests.rs` must be updated for the `DataRow`
enum change (Phase 3) but must cover the same invariants.

### `#[must_use]` with Message Strings

All new public functions returning `Result`, `Option`, or a security-relevant type need
`#[must_use = "..."]`. This includes `HeaderField::new`, `DataRow::two_column`,
`DialogState` constructors.

### Pattern Execution Measurement

Phase 1's `read_security_indicators()` is a provenance-verified I/O path — it qualifies
for Pattern Execution Measurement. Wrap the body in:

```rust
#[cfg(debug_assertions)]
let _t = std::time::Instant::now();
// ... body ...
#[cfg(debug_assertions)]
log::debug!("Pattern: SecurityIndicators read completed in {} µs", _t.elapsed().as_micros());
```

---

## Security-Auditor Review Points

The security-auditor agent should evaluate the following when reviewing implementations
produced from this plan:

1. **Phase 1 — Indicator population**: Confirm all reads go through `SecureReader`.
   No `File::open("/sys/...")` or `File::open("/proc/...")` in `indicators.rs`.

2. **Phase 1 — Fail-closed behavior**: `IndicatorValue::Unavailable` must be the default
   when a read fails. No fallback to a "looks active" default.

3. **Phase 2 — Header fields**: Confirm `header_fields()` return value flows only to
   ratatui `Span` for display. It must not be used in any trust or policy decision.

4. **Phase 3 — `DataRow` enum migration**: Confirm all call sites are updated and no
   field access bypasses the enum match (which would be a compile error, but worth
   verifying the migration is complete).

5. **Phase 6 — Evidence display**: Confirm that path strings are truncated to display
   width and that no raw kernel values (e.g., full buffer contents) appear in the
   evidence table values.

6. **Dialog design (future)**: The dialog's `SecurityWarning` mode should be reviewed
   for whether the styling is visually unambiguous enough for a DoD operator under
   time pressure.

7. **Compliance annotation coverage**: Verify all new `pub` items carry NIST/RTB
   citations. The auditor's existing annotation review process applies.

---

## File Change Summary

| File | Phases | Change Type |
|---|---|---|
| `src/app.rs` | 1, 2, 3, 4, 6 | Additive + breaking (DataRow) |
| `src/indicators.rs` | 1 | New file |
| `src/header.rs` | 1, 2 | Breaking (new args) |
| `src/layout.rs` | 1 | Breaking (new arg), constant update |
| `src/theme.rs` | 4+5 | Additive only |
| `src/data_panel.rs` | 3, 4, 6 | Additive match arms |
| `src/lib.rs` | 1, 2 | Additive re-exports, new pub mod |
| `src/main.rs` | 3, 6, 7 | All three phases touch this file |
| `src/bin/file_stat.rs` | 1, 3 | Must update for breaking changes |
| `tests/data_types_tests.rs` | 3, 4, 6 | Update + extend |
| `tests/trait_impl_tests.rs` | 2, 7 | Extend |
| `tests/indicators_tests.rs` | 1 | New file |
| `tests/theme_tests.rs` | 5 | Extend |
| `src/dialog.rs` | 8 | New file |
| `src/keymap.rs` | 8 | Add dialog action variants |
| `tests/dialog_tests.rs` | 8 | New file |

---

## Implementation Sequence

Jamie's specified order maps to the phases above:

1. Phase 1 (header security indicators) — `indicators.rs` + `header.rs` + `layout.rs`
2. Phase 2 (header extensibility) — `app.rs` `HeaderField` + `header_fields()` default method
3. Phase 3 (two-column layout) — `DataRow` enum, `data_panel.rs` match, all call sites
4. Phase 4 (group titles) — `DataRow::GroupTitle` variant active in `data_panel.rs`
5. Phase 5 (theme styling) — `theme.rs` additions (partly needed by Phases 1+4 already)
6. Phase 6 (evidence tab grouping) — `main.rs` `build_trust_rows` refactor + new variants
7. Phase 7 (kernel security placeholder tab) — `main.rs` only
8. Phase 8 (dialog API) — `dialog.rs` + `keymap.rs` + `theme.rs`

Phases 1 and 5 are partially interleaved: the `theme.rs` indicator style fields are needed
by Phase 1's header render code. The developer should add those theme fields during Phase 1
even though Phase 5 is listed later. Phase 5 then only adds the `group_title` style.

---

## Documentation Sync

On completion, a `doc-sync:` task must be created for the `tech-writer` covering:

- `umrs-tui` public API surface: new types `SecurityIndicators`, `IndicatorValue`,
  `HeaderField`, updated `DataRow` enum
- Documentation pages that may contain code snippets referencing `DataRow` struct fields
  (the enum migration is a breaking change visible in examples)
- Any Antora page that describes the audit card layout (header height, column behavior)

---

## Future Phases (from ROADMAP — not yet designed)

### Phase 9 — Findings Tab (ROADMAP enhancement)

When the security posture isn't good, a dedicated **Findings** tab should indicate what
was wrong and the related security control. Each finding should:
- Name the failed check
- Cite the specific security control (e.g., NIST SP 800-53 CM-6)
- Indicate severity
- Be structured data (`SecurityObservation` enum variants, not strings)

This ties into the assessment engine (G4) — findings are the bridge between tool output
and formal assessment reports.

### Phase 10 — Security Control Text Pop-Up (ROADMAP enhancement)

When viewing a finding, the operator should be able to pop up a meaningful snippet of the
security control text so they understand *why* the control isn't satisfied.

**Level-of-effort estimate needed before implementation.** Considerations:
- Does NOT need to be the entire control text — a meaningful snippet is sufficient
- Source data: could come from the RAG corpus (NIST collection) or a compiled-in lookup table
- The Dialog API (Phase 8) provides the overlay mechanism
- Scope question: how many controls do we need to cover? All of SP 800-53? Just the ones
  UMRS tools can assess? Start with the subset relevant to posture checks.
- Jamie flagged this as potentially enormous — estimate first, build second

**Dependencies:** Phase 8 (Dialog API), Phase 9 (Findings Tab), security-auditor input on
which controls to include and what constitutes a "meaningful snippet."

---

## ROADMAP Alignment

- **G5 (Security Tools)**: Audit cards become proper security-posture reports. The header
  shows enforcement state at a glance. Evidence is grouped and readable. This directly
  serves the auditor use case.
- **G8 (High-Assurance Patterns)**: The `DataRow` enum, typed `IndicatorValue`,
  `SecurityIndicators` population via `SecureReader`, and the `DialogMode` enum all
  demonstrate the "security findings as data" and "validate at construction" patterns from
  the pattern library.
