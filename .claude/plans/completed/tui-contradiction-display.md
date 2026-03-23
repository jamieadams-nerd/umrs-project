# TUI Contradiction Display — Implementation Plan

**Date:** 2026-03-20
**Status:** COMPLETE (2026-03-21). Kernel baseline added. Phase 6 passed. All Medium findings resolved.
**ROADMAP Goals:** G5 (Security Tools), G3 (Posture Assessment)
**Triggered by:** Jamie session 2026-03-20 — "did we check what the configuration file says it wanted?"

---

## Problem Statement

The `PostureSnapshot` and `IndicatorReport` types carry full contradiction detection
(configured vs live value comparison) but the TUI kernel security tab displayed none of it.
An operator had no way to see:

- Whether a sysctl.d/modprobe.d config existed for an indicator
- Whether the config disagreed with the running kernel
- Whether a runtime fix would survive a reboot

Both contradiction directions are operationally critical:

| Direction | Kind | Risk |
|---|---|---|
| Config says hardened, kernel is not | `BootDrift` | **Security failure** — intended hardening is not active |
| Kernel is hardened, config is not | `EphemeralHotfix` | **Persistence failure** — hardening lost on reboot |
| Config exists, kernel node unreadable | `SourceUnavailable` | **Verification gap** — cannot confirm posture |

## Implementation (COMPLETE — Rusty, 2026-03-20)

### Changes made

1. **`umrs-tui/src/app.rs`** — Extended `DataRow::IndicatorRow` with `contradiction: Option<ContradictionKind>` and `configured_line: Option<String>`. New `indicator_row_full()` constructor.

2. **`umrs-tui/src/data_panel.rs`** — Renders contradiction markers below each indicator:
   - `BootDrift` → `TrustRed` + `⚠ DRIFT: config says hardened, kernel is not`
   - `EphemeralHotfix` → `TrustYellow` + `⚠ NOT PERSISTED: hardened now, lost after reboot`
   - `SourceUnavailable` → `Dim` + `⚠ UNVERIFIABLE`
   - Shows configured value + source file when available
   - Uses `⚠` symbol for NO_COLOR / WCAG 1.4.1 compliance

3. **`umrs-tui/src/main.rs`** — `indicator_group_rows()` now reads `report.contradiction` and `report.configured_value`. Summary pinned rows show contradiction count.

4. **`umrs-tui/tests/data_types_tests.rs`** — Updated for new fields.

### Build status

Rusty reports: fmt, clippy, test all pass.

## Still TODO (deferred — post-review)

### A. Kernel baseline version message (Jamie feedback, same session)

The current pinned summary message "Tested baseline: see kernel security catalog for update guidance" is unclear. Replace with:

- Compare running kernel version against catalog baseline version
- If running kernel is newer: "Your kernel (6.12.0-211) is newer than catalog baseline (6.12.0) — some indicators may have changed"
- If running kernel matches: "Catalog baseline matches running kernel"
- If running kernel is older (rare): flag it

This requires the catalog to expose its baseline kernel version as a constant.

### B. Mermaid flow diagram (Jamie request)

Create a Mermaid diagram showing the contradiction detection flow:

```
Live value read → Configured value read → classify() → ContradictionKind → TUI display
                                                     ↘ None (no contradiction)
```

Target: `docs/modules/patterns/pages/` or `docs/modules/devel/pages/`

### C. Review and commit

Jamie will review the working tree changes and commit when ready.
