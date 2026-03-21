---
name: tui-review-patterns
description: Recurring patterns and unresolved findings from umrs-tui review rounds v1, v2, Phase 6 (2026-03-20/21)
type: project
---

## Persistent Unresolved Items (as of Phase 6, 2026-03-21)

These were open in v2 and confirmed still open in Phase 6:

1. **C-1 / C-7v2** — `main.rs` line 1891: help text "Red rows require remediation before CUI processing" is too absolute. Fix: "Red rows do not meet the hardened baseline — review each indicator's description for risk context and remediation priority before CUI processing." Coder owner. MEDIUM.

2. **C-2 / C-15v2** — `main.rs` line 1805 (`label_trust_display`): `IntegrityVerifiedButContradictory` contradiction string `.take(48)` leaves only 13 usable chars after the 35-char prefix. Fix: emit a secondary empty-key KeyValue row with the full text, consistent with downgrade reasons pattern. Coder owner. MEDIUM. **Blocks release.**

3. **C-3 / C-T3-STATUS** — `main.rs` line 1820: `SubstrateAnchored` (T3) uses `StatusLevel::Info` (blue) while data panel renders T3 green. Fix: `StatusLevel::Ok`. LOW.

## Patterns to Check in Future TUI Reviews

- `label_trust_display()` — check all variant arms for truncation of security-relevant strings
- Status bar level (StatusLevel) must match the StyleHint used for the same trust state in data panel
- Help text assertions about CUI processing must be hedged, not absolute
- Contradiction markers: BootDrift=Red, EphemeralHotfix=Yellow, SourceUnavailable=Dim — verify this mapping survives refactors

## Assessment Value Patterns (Phase 6 findings)

- Indicator descriptions follow: "what the setting controls + what the adversary gains if it fails" — check this pattern when new indicators are added
- Recommended value strings should be operator-readable (not raw integers) — always include parenthetical
- `nist_controls` field in catalog uses abbreviated `NIST 800-53` form — this is rule-compliant per the display-string exemption; do not flag it

**Why:** Rapid-fire rounds of review on this tool; patterns repeat across sessions.
**How to apply:** At start of any TUI review session, check these items first.
