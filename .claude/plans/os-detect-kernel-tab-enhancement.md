# Plan: OS Detection Tool & Kernel Security Tab Enhancement

**Date:** 2026-03-15
**Status:** Phase 0 in progress (IRS indicator definitions)
**ROADMAP Goals:** G1 (Platform Awareness), G5 (Security Tools), G8 (High-Assurance Patterns)
**Owner:** All-team effort
**Justification:** NIST SP 800-53A (assessment clarity), CLIG guidelines (operator UX),
CMMC SC.L2-3.13.10 (CUI system monitoring requires understandable output)

---

## Problem Statement

The Kernel Security tab displays 25+ indicators across 6 groups with cryptic names,
raw numeric values, and no explanatory context. An operator sees `kptr_restrict : 2`
and has no idea what it means or whether it's good. Even trained auditors need a
reference sheet. The tool produces correct data but fails to communicate it.

This is a CUI system tool. If the operator can't understand the output, the tool
has no assessment value — regardless of how technically correct the underlying probes are.

---

## Team Roles

| Agent | Alias | Responsibility |
|---|---|---|
| security-auditor | The IRS | Defines what each indicator means, what "good" looks like, NIST/CMMC control mapping |
| rust-developer | Rusty | Implements display descriptions, value translations, UX improvements in code |
| tech-writer | Von Neumann | Writes the operations reference guide in Antora docs |
| senior-tech-writer | The Imprimatur | Reviews and approves final documentation |
| security-engineer | *(unassigned)* | Reviews deployment context for indicator relevance |

---

## Phase 0 — Indicator Definition Reference (The IRS)

**Status:** In progress

The IRS produces the authoritative indicator reference table:

For every indicator:
- Plain language description (what is this?)
- What "good" looks like (expected hardened value)
- What "bad" looks like (insecure/default value)
- Why it matters (1 sentence — what's the risk?)
- NIST SP 800-53 control mapping
- CMMC practice mapping where applicable

For every group:
- 1-2 sentence plain language explanation of what this group covers

For trust evidence tab:
- What each column means
- What ✓/✗/? mean
- How to interpret trust tiers (T0→T3)
- Better phrasing for "downgrade reasons: none"

**Output:** `.claude/agent-memory/security-auditor/indicator-definitions-plain-language.md`

---

## Phase 1 — Value Translation (Rusty)

**Depends on:** Phase 0 complete

Translate raw numeric values into human-readable text:

- `kptr_restrict : 2` → `kptr_restrict : 2 (restricted)` or `Kernel pointer visibility : restricted`
- `randomize_va_space : 2` → `ASLR : full randomization`
- Boolean values already show `enabled`/`disabled` — verify all are covered
- Values that are integers with specific meanings get parenthetical translations

**Design decision needed:** Do we show the raw value AND the translation, or just the
translation? Raw values help advanced users; translations help everyone else.
Jamie to decide.

---

## Phase 2 — Indicator Descriptions (Rusty)

**Depends on:** Phase 0 complete

Add brief explanatory text to each indicator row. Options:

**Option A — Inline description:**
```
kptr_restrict          : 2 (restricted) — hides kernel pointers from /proc
```

**Option B — TwoColumn with description on right:**
```
kptr_restrict : 2 (restricted)          Hides kernel pointers from /proc
```

**Option C — Group-level descriptions only:**
```
KERNEL SELF-PROTECTION
  Controls that prevent exploitation of kernel vulnerabilities

  kptr_restrict     : 2 (restricted)
  randomize_va_space : 2 (full ASLR)
```

Jamie to decide which option. Option C is cleanest; Option A is most self-documenting.

---

## Phase 3 — Trust Evidence Tab UX (Rusty)

**Depends on:** Phase 0 (trust evidence definitions)

- Reword "downgrade reasons: none" to positive framing
- Add trust ladder visualization (T0→T1→T2→T3 progression)
- Consider fourth evidence column for trust tier contribution
- Add brief explanation of what contradictions mean
- Ensure evidence tab is scrollable with all content visible

---

## Phase 4 — In-TUI Help (Rusty)

**Depends on:** Phase 8 Dialog API (DONE), Phase 0 definitions

Use the Dialog API (Info mode) to provide contextual help:

- `?` or `F1` key opens help overlay for the current tab
- Kernel Security tab help: explains what the groups mean, what colors mean
- Trust Evidence tab help: explains the evidence chain, trust tiers, verification symbols
- Help text sourced from The IRS's Phase 0 definitions

---

## Phase 5 — Operations Reference Guide (Von Neumann)

**Depends on:** Phases 1-3 complete (code matches what docs describe)

Von Neumann writes the authoritative reference in `docs/modules/operations/` or
`docs/modules/umrs-tools/`:

- Every indicator documented with The IRS's definitions
- Every group explained
- Screenshots or text captures of the TUI output
- "What to do when you see red" — operator response procedures
- Trust tier explanation with Mermaid diagrams
- Header field justification table (with NIST/CMMC/OSCAL citations)
- Contradiction explanation with flow chart (Mermaid)

The Imprimatur reviews and approves before publication.

---

## Phase 6 — Assessment Value Review (The IRS)

**Depends on:** Phases 1-5 complete

The IRS runs the enhanced tool and evaluates:

- Is every indicator understandable without external reference?
- Does the color coding consistently indicate security posture?
- Can an auditor produce a finding from this output?
- Does the evidence tab satisfy SP 800-53A Examine requirements?
- Are CMMC practices adequately covered for CUI system assessment?

Findings feed back to Rusty for final adjustments.

---

## Success Criteria

1. An operator with basic Linux knowledge can read the Kernel Security tab and
   understand every field without a reference guide
2. An auditor can use the output to produce assessment findings with control citations
3. The trust evidence tab tells a clear story: what was checked, what passed, what failed
4. The operations reference guide in Antora is complete and matches the live tool
5. `make docs` passes clean
6. All tests pass, clippy clean
