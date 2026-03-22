---
name: Intern review findings — outreach and narrative implications
description: What Nora and Finn's differentiated reviews demonstrate about the AI development model
type: project
---

Nora produced 13 findings (1 HIGH, 4 MEDIUM, 8 LOW) from a first-time user perspective.
Finn produced 18 findings (4 HIGH, 7 MEDIUM, 7 LOW) from an operator perspective.

**The important observation:** The reviews are genuinely differentiated. Nora focused on
conceptual clarity gaps (what trust tiers imply, why verification codes exist).
Finn focused on operational completeness gaps (what to DO with a DRIFT finding, go/no-go
thresholds, escalation paths). These are distinct problem spaces arising from distinct
roles. An intern playing "security operator" would not have produced this split.

**What Finn's HIGHs reveal:**
Finn's four HIGH findings all converge on the same gap: the tool accurately describes
conditions but does not connect them to operational decisions. DRIFT with no investigation
path. Trust tiers with no go/no-go threshold. "May indicate tampering" with no next step.
This is the gap between a diagnostic instrument and a tool that operators can act on.

**The design principle Jamie established in response:** Surface the finding, steer direction,
do not dictate site procedure. This is the right call — and the interns surfaced the need
to make that choice explicit.

**For outreach content:** This session is a real demonstration of the AI development model
working. Two agents with distinct knowledge profiles reviewing the same artifact and
producing complementary, non-overlapping findings. That is the thesis of Jamie's AI study.
Blog post material: "What happens when you give the intern two different job descriptions?"

**Source documents:**
- `.claude/reports/nora-help-text-review-2026-03-22.md`
- `.claude/reports/finn-help-text-review-2026-03-22.md`
- `docs/modules/ai-transparency/pages/final-cast-and-crew.adoc` (intern bios)
