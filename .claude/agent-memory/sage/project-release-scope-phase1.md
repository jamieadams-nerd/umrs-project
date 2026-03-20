---
name: Phase 1 Release Scope — What Sage Can Write About Now
description: Defines the concrete first-release feature set that Sage is authorized to describe in outreach content. Hard guardrails on what to include and what to defer.
type: project
---

First release scope (Jamie's directive, 2026-03-20):

- CUI labeling on a system running targeted SELinux policy
- Basic security-focused tool overlay: umrs-uname, umrs-ls, umrs-stat
- Base software stack
- RHEL 10 deployment guide

**Why:** Jamie wants a concrete, releasable milestone. Labeling is a new concept for most operators. Getting the label story right — data knows what it is — is the Phase 1 value proposition before Phase 2 enforcement is built.

**How to apply:** Every blog post, abstract, or outreach piece about "what UMRS does" must scope to this list. Do NOT mention IMA/EVM in Phase 1 content. Do not imply enforcement is in scope — labeling and awareness is the story. See `phase1-phase2-positioning.md` for the full positioning guardrails.

Tool naming note: Jamie is overlaying classic Linux tools with security-focused versions. The pattern is deliberate — familiar commands, security-enriched output. umrs-ls shows you what SELinux sees. umrs-stat shows you the security context. umrs-uname shows you the platform posture. This is the pitch.

Next release (Phase 2 preview, for future use only):
- More tools
- Environment scrubbing (flagged as "hot" by Sage in prior session)
- IMA/EVM enhanced assurance
- MLS policy enforcement
