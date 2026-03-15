---
name: umrs-tui HeaderContext and Phase 1 Header Redesign
description: HeaderContext type, build_header_context(), and the two-column header layout implemented in Phase 1 of the TUI Enhancement Plan
type: project
---

## Phase 1 TUI Enhancement — Implemented 2026-03-15

**Fact:** Phase 1 of the TUI Enhancement Plan is complete. The header was redesigned from
a single-column "Report/Host/Subject + badge row" layout to a two-column six-row layout
with OSCAL-aligned terminology.

**Why:** The security-auditor required OSCAL/SP 800-53A terminology and full-word indicators
(not compressed badge format). HeaderContext was added to carry system identification fields.

**How to apply:** Use `build_header_context(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))`
in every binary before the event loop. Pass `&ctx` to `render_audit_card` instead of `&indicators`.

### New Type: `HeaderContext` (in `app.rs`)

Fields: `indicators: SecurityIndicators`, `tool_name`, `tool_version`, `assessed_at`,
`hostname`, `kernel_version`, `boot_id`, `system_uuid` (all `String`).

### New Function: `build_header_context` (in `indicators.rs`)

Reads: security indicators + uname(hostname, kernel) + boot_id (ProcfsText) + system_uuid (SysfsText).
Fail-closed: on any read error the field is set to `"unavailable"`.

### `render_audit_card` signature change

Was: `(frame, area, app, state, &SecurityIndicators, theme)`
Now: `(frame, area, app, state, &HeaderContext, theme)`

All callers updated: `main.rs`, `bin/file_stat.rs`, `examples/show_logo.rs`.

### Header Layout

Two-column when terminal width >= 90, single-column when narrower.
Six rows: Assessment/Boot ID | Scope/Assessed | Host/Kernel | Tool/System ID | SELinux/LSM | FIPS/Lockdown.
Terminology: "Assessment" (was Report), "Scope" (was Subject), "Assessed" (was Checked).

### Implementation Notes

- `i18n::tr()` returns `String` — must store as locals before passing `&str` to helpers
- Gregorian calendar algorithm in `format_assessed_at` uses all-`i64` arithmetic to avoid
  clippy `cast_possible_truncation` — uses `i64::try_from(u64)` for the epoch-day conversion
- `two_col_line` uses `{left_label:<LABEL_WIDTH$} :` format (inlined variable for clippy)
- `read_security_indicators()` still exists and is still re-exported — backward compat
