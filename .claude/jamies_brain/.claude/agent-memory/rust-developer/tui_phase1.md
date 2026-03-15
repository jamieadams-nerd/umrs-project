---
name: umrs-tui TUI Phase 1 — Security Indicator Header Row
description: What was implemented in Phase 1, key design decisions, and what's deferred
type: project
---

Phase 1 of the TUI enhancement plan (`.claude/plans/tui-enhancement-plan.md`) is complete.

**Why:** Audit cards need a compact header row showing live kernel security posture
(SELinux, FIPS, lockdown, LSM, Secure Boot) sourced from SecureReader kernel reads.

**What was delivered:**

- `app.rs`: `IndicatorValue` enum (Active/Inactive/Unavailable) and `SecurityIndicators` struct
- `indicators.rs`: `read_security_indicators()` reads SelinuxEnforce, ProcFips, KernelLockdown via StaticSource; debug-mode timing log
- `theme.rs`: `indicator_active/inactive/unavailable` style fields; `indicator_style()` const fn helper
- `header.rs`: `render_header()` now takes `indicators: &SecurityIndicators`; builds 5-badge indicator row
- `layout.rs`: `render_audit_card()` now takes `indicators: &SecurityIndicators`; `HEADER_HEIGHT` +1 for indicator row
- `lib.rs`: `pub mod indicators`; re-exports `IndicatorValue`, `SecurityIndicators`, `read_security_indicators`
- Callers updated: `main.rs`, `bin/file_stat.rs`, `examples/show_logo.rs`
- `tests/indicators_tests.rs`: 5 tests covering default/unavailable contract, variant distinctness, inner string round-trip, field assignment

**Deferred (TODO comments in indicators.rs):**
- `active_lsm` — `/sys/kernel/security/lsm` is securityfs, needs a dedicated StaticSource type in umrs-platform
- `secure_boot` — UEFI efivars path, platform-specific, no kattr type yet

**How to apply:** When continuing Phase 2+ work, indicators snapshot is already threaded everywhere.
`read_security_indicators()` is cheap (3 kernel reads) and called once before the event loop.
