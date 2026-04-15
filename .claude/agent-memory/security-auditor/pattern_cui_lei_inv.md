---
name: CUI LEI/INV anti-pattern distribution
description: Where the CUI//LEI/INV subcategory anti-pattern appears across the repo and what was fixed
type: project
---

The `CUI//LEI/INV` anti-pattern (INV incorrectly subordinated to LEI) was found in 21 actionable locations as of 2026-04-15 audit. Full report: `.claude/reports/code/2026-04-15-cui-lei-inv-audit.md`.

**Why:** LEI and INV are both independent NARA categories. `CUI//INV` is the correct standalone form; `CUI//INV/LEI` alphabetized if both are genuinely present.

**How to apply:** On any future audit touching CUI marking strings, grep for `LEI/INV` as well as `CUI//LEI`. The blog files use `LEI/INV` as a colloquial shorthand (not a banner string) — flag those too as they reinforce the wrong mental model.

Highest-priority unfixed items as of 2026-04-15:
- `components/rusty-gadgets/libs/umrs-selinux/config/setrans.conf:136` — active, operators see this
- `components/rusty-gadgets/umrs-label/config/us/labels-backup.json:673` — active catalog backup, has incorrect `parent_group: "LEI"` field too
- `help/umrs-ls/C/concept-grouping.page:50` and fr_CA equivalent — user-facing help
- `docs/sage/blogs/blog-cui-sign-lock.adoc` and `.md` — public blog
