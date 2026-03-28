# Agent Evolution — Quick Reference Highlights

**Purpose:** At-a-glance research data for Jamie's AI study.
**Full reports:** `.claude/reports/*-evolution-study-*.md`
**Skill:** `/agent-evolution-study <agent-name>` to generate new studies

---

## Herb (Security Auditor) — Studied 2026-03-22

**Study period:** 2026-03-11 — 2026-03-21 (11 days)
**Reports produced:** 18+
**Knowledge acquisition events:** 5
**Key inflection points:** 3

### Inflection Points
1. **[2026-03-15] RMF corpus (SP 800-37/53A/30/39)** → Stopped reviewing code for bugs; started reviewing systems for assessment readiness. First portfolio-level finding.
2. **[2026-03-17-18] SCAP/STIG familiarization** → Generic NIST citations became CCE-specific work instructions with cross-agent orchestration.
3. **[2026-03-20-21] All corpora converging** → Default lens is now "can an assessor use this output?" Reviews include positive findings and prioritized remediation.

### Capability Trajectory
| Dimension | Pre-Corpus (3/11) | Post-RMF (3/15) | Post-SCAP (3/17) | Mature (3/20+) |
|---|---|---|---|---|
| Finding type | Code bugs | Assessment gaps | CCE-specific ops | Assessment value |
| Framework | Control catalog | Assessment procedures | STIG + controls | Risk taxonomy |
| Communication | Flat list | Executive summary | Tiered + work instructions | ACCURATE/CONCERN/ERROR |
| Cross-agent | None | Portfolio observations | Orchestrates remediation | Release-readiness calls |

### One-Line Thesis
Knowledge acquisition produces qualitative capability shifts — not more findings, but fundamentally different kinds of findings.

### Why This Matters (Jamie's framing)
The traceability is the point. You can point to a specific date, a specific document, and a specific behavioral change. That's not something you get from a model trained on more data. The RAG architecture combined with the structured familiarization sequence created an **audit trail of intellectual development** — you can trace exactly when Herb learned SP 800-53A assessment procedures, and exactly when "can an assessor use this?" first appeared in his reviews. That's remarkable, unusual, and the core differentiator from both generic swarms and base-model improvements.

---

## Rusty (Rust Developer) — NOT YET STUDIED

Data available: task log entries from 2026-03-11+, reports, code commits, plan authorship.
Candidate inflection points: CPU corpus ingestion, TUI/CLI corpus, auditor feedback loops.

## Elena (Senior Tech Writer) — NOT YET STUDIED

Data available: task log, doc reviews, Antora restructure work, style decisions.
Candidate inflection points: doc-arch corpus, STE mode adoption, admonition hierarchy.

## Sage (Outreach) — NOT YET STUDIED

Data available: blog posts, content policy evolution, scoring rubric, strategic briefs.
Candidate inflection points: Jamie's philosophical input, post-publication feedback.

## Simone (Translator) — NOT YET STUDIED

Data available: corpus pipeline, french-lookup skill, terminology decisions.
Candidate inflection points: GNU translation corpus ingestion, Canadian French scoping.

## The Librarian (Researcher) — NOT YET STUDIED

Data available: RAG ingestions, collection manifests, research briefs.
Candidate inflection points: each new collection → downstream agent capability.
