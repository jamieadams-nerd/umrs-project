---
name: Phase 1 vs Phase 2 Positioning — UMRS Capability Guardrails
description: Foundational positioning rules for what UMRS claims in targeted policy (Phase 1) vs MLS (Phase 2). Hard constraints on outreach content. Read before writing any post.
type: feedback
---

Source document: `.claude/jamies_brain/target-mls.txt`
Date internalized: 2026-03-20

## The Foundational Distinction

Phase 1 (Targeted SELinux Policy) is not a weak version of Phase 2. It is a different thing doing a different job.

**Phase 1 ensures the data knows what it is.**
**Phase 2 ensures the system enforces who can touch it.**

Never conflate these. Never use Phase 2 language to describe Phase 1 behavior.

---

## What Phase 1 IS

UMRS in targeted policy provides:

- **Authoritative labeling** — all data is explicitly labeled, traceable, and contextually understood
- **Operator awareness** — continuous, unambiguous signaling of sensitivity, handling restrictions, and operational expectations (categories, caveats, dissemination controls)
- **System-bound custody integrity** — while data resides within the UMRS boundary, the system is responsible for preserving correctness and visibility of labels and metadata
- **Minimal meaningful controls** — type enforcement, controlled directories where feasible
- **Boundary-based responsibility** — UMRS is authoritative within its boundary; responsibility transfers to the recipient/downstream system when data leaves

---

## What Phase 1 IS NOT

Phase 1 does NOT provide:

- End-to-end enforcement of CUI handling requirements
- Mandatory access control based on clearance levels
- Strong isolation between sensitivity levels
- Enforcement beyond the system boundary
- Guarantee that CUI is "protected" in the full enforcement sense

**Why this matters:** These are Phase 2 (MLS policy) capabilities. Claiming them for Phase 1 is an overclaim that destroys trust with the exact technical audience UMRS needs to reach.

---

## The Responsibility Model (Verbatim from target-mls.txt)

> While information resides within the UMRS system boundary, the system is responsible for preserving the integrity, visibility, and correctness of its labeling and associated handling metadata. Once that information leaves the system boundary, responsibility for proper handling transfers to the recipient or downstream system.

**Tagline-level:** "Authoritative labeling within the boundary. Responsible handling beyond it."

---

## Why Phase 1 Is Still Valuable (The Justification Argument)

1. **Perfect enforcement is not required to provide value.** Even without MLS-level enforcement, UMRS eliminates unknown data sensitivity, reduces accidental mishandling, and provides consistent machine-readable semantics. That is a meaningful uplift over typical environments.

2. **Awareness is a control — and often the missing one.** In most real-world environments, data is copied with no markings, files lose context immediately, and operators are forced to guess sensitivity. UMRS ensures the operator is never blind to what the data is. This is a legitimate NIST-aligned control.

3. **Boundary-based responsibility is how real systems work.** Even high-assurance systems have strongest control inside the boundary and degrading trust across transitions. This is an accurate threat model, not a weakness.

4. **Phase 1 creates the foundation for Phase 2.** The labeling semantics, operator awareness, and data hygiene built in Phase 1 are prerequisites for Phase 2 MLS enforcement. The architecture is deliberate.

---

## Content Guardrail Rules (Operational)

### Safe to claim for Phase 1:
- "UMRS ensures the data knows what it is" — YES
- "Awareness is a control — and often the missing one" — YES
- "Authoritative labeling within the boundary" — YES
- "Operator is never blind to what the data is" — YES
- "Labeling fidelity and system-bound custody controls" — YES
- "High-assurance reference implementation for classification awareness" — YES

### NEVER claim for Phase 1:
- "UMRS enforces who can touch it" — NO (Phase 2)
- "UMRS guarantees CUI protection" — NO (overclaim)
- "Mandatory access control" when describing targeted policy — NO
- "Strong isolation" between sensitivity levels — NO
- "Clearance-based access" — NO

### How to frame limitations honestly:
- "Phase 1 does not enforce; it ensures awareness" — the model language
- "Phase 2 will add mandatory enforcement and clearance-based access decisions"
- Admitting this clearly BUILDS trust; hiding it destroys it with the technical audience

---

**Why:** Jamie's foundational positioning document establishes this distinction as architectural, not accidental. The two phases do fundamentally different things. Overclaiming Phase 1 as enforcement would mislead exactly the security engineers and auditors UMRS most needs to reach — people who will immediately see through inaccurate claims and will not return.

**How to apply:** Before publishing any post, run Phase 1/2 claims through this checklist. If a sentence implies enforcement capability that targeted SELinux policy does not provide, it is wrong and must be corrected.
