# High-Assurance Documentation Writing Guide — Plan

**Created:** 2026-03-16
**Status:** Unblocked — SCAP ingestion complete; ready for execution
**ROADMAP Goals:** G1 (Documentation Excellence), G10 (AI Transparency)
**Agent:** senior-tech-writer (primary author), tech-writer (reviewer)
**Audience:** External agents, new contributors, other projects adopting similar practices

---

## Vision

Produce a standalone, shareable writing guide that teaches any AI agent or human writer
how to produce documentation for a high-assurance, heavily audited, NIST/CCE/CMMC-referenced
system. This guide captures the nuances that make high-assurance documentation fundamentally
different from standard technical writing.

The goal is: hand this guide to someone else's agent on a different project and they
immediately write documentation that meets our standard.

---

## Why This Is Different

Standard technical writing guides cover tone, structure, and clarity. They do not cover:

- **Citation discipline** — when and how to cite NIST SP 800-53, CCE, CMMC, NSA RTB controls
- **Admonition hierarchy** — MIL-STD-38784B adapted for software (consequence determines level)
- **Audience layering** — writing for developers, operators, AND security auditors simultaneously
- **Compliance annotation placement** — module-level vs. type-level vs. not-needed
- **Verifiable claims only** — no hedging, no aspirational language, every security claim traceable
- **STE for procedures** — Simplified Technical English with approved verb lists
- **Inclusive terminology with technical precision** — two-tier vocabulary (inclusive in narrative,
  standard terms in specifications, editorial notes for verbatim quotes)
- **Cross-reference density** — NIST → CCE → STIG → code → documentation round-tripping
- **Diataxis + modular documentation** — Red Hat-style module typing within Diataxis framework
- **Threat-aware writing** — error messages, log output, and examples must not leak sensitive data
- **Writing for auditors** — what auditors look for, how to structure evidence, what "verifiable" means

---

## Planned Structure

### Part 1 — Foundations
- What makes high-assurance documentation different
- The three audiences and how to serve all three
- Diataxis taxonomy adapted for security documentation
- Red Hat modular documentation patterns (concept/procedure/reference)

### Part 2 — Citation and Compliance
- NIST SP 800-53 citation format and placement rules
- CCE cross-referencing (from SCAP/STIG corpus)
- NSA RTB principle citations
- CMMC domain and level citations
- NIST SP 800-218 SSDF practice citations
- FIPS 140-2/140-3 citations
- When NOT to cite (simple accessors, display impls)

### Part 3 — Writing Rules
- Admonition hierarchy (MIL-STD-38784B adapted)
- STE mode for procedures (approved verbs, sentence length, active voice)
- Terminology control (approved term list, consistency rules)
- Inclusive language with technical precision (SDR-010 two-tier rule)
- Persona address by module (SDR-009: third person for architecture, second for procedures)
- Contractions by audience (SDR-001)

### Part 4 — Security-Aware Writing
- Error information discipline — what not to include in examples
- Threat-aware documentation — examples that don't teach attacks
- Verifiable claims only — no hedging language
- Writing for Common Criteria (SFR descriptions, Security Target prose)
- Debug log documentation — value suppression on CUI/DoD systems

### Part 5 — Tooling and Process
- Antora mechanics (navigation, xrefs, component descriptors)
- Build validation (`make docs` gate)
- Doc-ops checklist
- RAG corpus usage (rag-query, doc-arch skills)
- Style decision records (SDR process)

### Part 6 — Reference
- Complete approved terminology list
- Complete approved verb list (STE)
- Admonition decision tree
- Citation format quick reference
- Style decision record index

---

## Sources to Draw From

All of these exist and have been ingested/familiarized:

- `.claude/rules/ste_mode.md` — STE writing rules
- `.claude/rules/admonition_hierarchy.md` — MIL-STD admonition hierarchy
- `.claude/rules/rust_design_rules.md` — citation format, comment discipline
- `.claude/knowledge/tech-writer-corpus/style-decision-record.md` — all SDRs
- `.claude/knowledge/tech-writer-corpus/term-glossary.md` — canonical terms
- `.claude/knowledge/tech-writer-corpus/cross-reference-map.md` — source tensions
- `.claude/agents/senior-tech-writer.md` — agent definition with all style rules
- `.claude/agents/tech-writer.md` — agent definition
- `CLAUDE.md` — compliance annotation rules, architectural review triggers
- RAG `tech-writer-corpus` — Google, Microsoft, MIL-STD, NIST, Plain Language, CC
- RAG `scap-stig` (when ingested) — CCE cross-referencing patterns

---

## Output Format

The guide should be:
1. **Self-contained** — no dependencies on UMRS-specific files; portable to other projects
2. **AsciiDoc** — lives in `docs/modules/devel/pages/` for our use, but exportable
3. **Structured for agents** — clear rules, decision trees, examples; not narrative prose
4. **Versioned** — include a revision date and source corpus version

---

## Definition of Done

- [ ] All 6 parts drafted by senior-tech-writer
- [ ] Reviewed by tech-writer for completeness and accuracy
- [ ] Reviewed by security-auditor for citation accuracy
- [ ] `make docs` passes
- [ ] Guide is self-contained — can be copied to another project and used immediately
- [ ] Jamie reviews and approves for external sharing

---

## DO NOT START YET

This is a placeholder. Execute after:
1. SCAP/STIG corpus ingested and all agents familiarized
2. CC corpus familiarization complete for both writers
3. Current admonition fixes and Phase 2b doc-sync landed
