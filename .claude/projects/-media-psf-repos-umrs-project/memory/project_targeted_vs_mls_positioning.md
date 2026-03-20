---
name: Targeted vs MLS positioning — Phase 1 and Phase 2
description: UMRS Phase 1 (targeted policy) is about labeling fidelity and awareness, NOT enforcement. Phase 2 (MLS) adds mandatory enforcement. Critical framing for all documentation and outreach.
type: project
---

## Phase 1 (Targeted Policy) — What UMRS Claims

UMRS in targeted SELinux does NOT guarantee end-to-end enforcement of CUI handling. It provides:

- **Authoritative labeling** — all data is explicitly labeled, traceable, contextually understood
- **Operator awareness** — continuous, unambiguous signaling of sensitivity, handling restrictions, operational expectations
- **System-bound custody** — while data is within the system boundary, UMRS preserves integrity, visibility, and correctness of labeling and metadata
- **Minimal but meaningful controls** — type enforcement, controlled directories, vault flows where feasible

**Key principle:** "While information resides within the UMRS system boundary, the system is responsible for preserving the integrity, visibility, and correctness of its labeling and associated handling metadata. Once that information leaves the system boundary, responsibility for proper handling transfers to the recipient or downstream system."

**Why:** This is an accurate threat model, not a weakness. Control is strongest inside the boundary; trust degrades across transitions.

## Phase 2 (MLS Policy) — What Phase 2 Adds

- Mandatory enforcement
- Clearance-based access decisions
- Strong isolation

**The bridge:** "Phase 1 ensures the data knows what it is. Phase 2 ensures the system enforces who can touch it."

## How to Apply

- **ALL documentation, blog posts, and outreach** must reflect this distinction
- **Sage** must NEVER overclaim enforcement capabilities in targeted policy content
- **Rusty** should not implement enforcement semantics in Phase 1 code paths
- **Tech writers** must clearly distinguish Phase 1 capabilities from Phase 2 promises
- When describing UMRS value in Phase 1: awareness IS a control, labeling IS meaningful, boundary-based responsibility IS how real systems work

**Source:** `.claude/jamies_brain/target-mls.txt` (Jamie's foundational positioning, 2026-03-20)
