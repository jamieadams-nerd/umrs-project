---
name: Phase 1 / Phase 2 Positioning
description: UMRS Phase 1 (targeted policy) vs Phase 2 (MLS) positioning — labeling fidelity vs mandatory enforcement; foundational distinction for all code, comments, and docs
type: project
---

## Core Distinction

**Phase 1 (Targeted Policy):** Labeling fidelity + operator awareness + custody integrity.
NOT about enforcement. The system is responsible for ensuring all data is correctly labeled,
visible, and contextually understood while under system control.

**Phase 2 (MLS):** Mandatory enforcement + clearance-based access decisions + strong isolation.
Builds on Phase 1's labeling semantics.

One-liner: *Phase 1 ensures the data knows what it is. Phase 2 ensures the system enforces
who can touch it.*

## Responsibility Model

**In-boundary (UMRS system):**
- Correctly label all artifacts (CUI categories, markings, caveats)
- Persistently associate metadata (xattrs, sidecars, receipts)
- Present clear visual and programmatic indicators
- Prevent silent ambiguity (no unlabeled or contextless data)
- Maintain chain-of-custody signals while under system control
- Enforce minimal but meaningful controls where feasible

**Out-of-boundary (exported, transmitted):**
- Receiving entity assumes responsibility for interpreting markings and applying handling procedures

## Why This Matters for Code

- Comments, doc strings, and compliance annotations must NOT claim enforcement that targeted
  policy does not provide
- Phase 1 work is about awareness controls: AC-16 (security attributes), AU-3 (audit content),
  PM-22 (data governance) — not AC-4 (information flow enforcement, which is Phase 2)
- The trust boundary of Phase 1 is the system boundary — we are authoritative inside it

## Tagline

"Authoritative labeling within the boundary. Responsible handling beyond it."

**Why:** Source: `.claude/jamies_brain/target-mls.txt` — Jamie's foundational positioning argument
**How to apply:** Any new module, type, or API that touches CUI handling must be framed in terms
of labeling/awareness/custody — NOT enforcement/access control — unless Phase 2 explicitly starts.
