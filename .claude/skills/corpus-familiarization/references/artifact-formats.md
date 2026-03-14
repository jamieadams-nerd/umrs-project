# Artifact Format Schemas

Exact schemas for the four knowledge artifacts produced during a
familiarization pass. Agents must follow these formats exactly — the
tech-writer agent and other consumers depend on consistent structure.

---

## Artifact 1: `concept-index.md`

```markdown
# Concept Index — <Collection Name>
Generated: <ISO date>

---

## <Document Short Identifier>

**Full title:** <Official document title>  
**Source:** <URL or file path>  
**Type:** <style guide | regulatory standard | domain reference | supplemental>  
**Normative weight:** <normative | informative | guidance>

### Coverage
<2–4 sentences describing what the document covers. Your own words.
Do not quote. Describe scope, not content.>

### Key concepts introduced
- <Term or concept> — <one-line definition>
- <Term or concept> — <one-line definition>
- ...

### Governs these writing tasks
- <Specific task or decision the agent should consult this document for>
- ...

### Related documents in corpus
- <Document identifier> — <nature of relationship>
- ...

---
```

Repeat the block for every document in the collection. Do not skip any document.

---

## Artifact 2: `cross-reference-map.md`

```markdown
# Cross-Reference Map — <Collection Name>
Generated: <ISO date>

---

## Agreements

### <Topic>
**Documents in agreement:** <Doc A>, <Doc B>  
**Shared guidance:** <What they agree on, in one sentence>

---

## Tensions

### <Topic of tension>
**Documents in conflict:** <Doc A> vs. <Doc B>  
**Doc A position:** <What Doc A says>  
**Doc B position:** <What Doc B says>  
**Nature of conflict:** <contradictory | context-dependent | scope difference>  
**Resolution:** See `style-decision-record.md` → <entry name>
  _or_
**Resolution:** <Pending — requires project owner input>

---

## Chains (deference relationships)

### <Topic>
**Primary:** <Doc A>  
**Defers to:** <Doc B> for <specific scope>  
**Agent behavior:** Consult <Doc B> first for <scope>, then <Doc A> for remaining guidance.

---

## Gaps

### <Gap topic>
**Not covered by:** any document in this collection  
**Agent behavior:** Flag to user when this topic arises. Do not invent guidance.
  _or_
**Agent behavior:** Apply <named principle> as a reasonable default; note the gap.
```

---

## Artifact 3: `style-decision-record.md`

```markdown
# Style Decision Record — <Project Name>
Generated: <ISO date>
Owner: <Agent name or "Pending project owner review">

This record resolves tensions identified in the cross-reference map.
Entries here take precedence over any individual source document.

---

## SDR-001: <Decision topic>

**Tension:** <Brief description of the conflict>  
**Sources involved:** <Doc A>, <Doc B>  
**Decision:** <What the agent should do>  
**Applies when:** <Context conditions — always | specific document type | specific audience>  
**Does not apply when:** <Exceptions>  
**Rationale:** <Why this decision, in one sentence>  
**Status:** <Resolved | Pending — awaiting project owner input>

---
```

### Placeholder format for unresolved decisions

```markdown
## SDR-NNN: <Decision topic> ⚠ PENDING

**Tension:** <Brief description>  
**Sources involved:** <Doc A>, <Doc B>  
**Options identified:**
  1. <Option A> — consequence: <...>
  2. <Option B> — consequence: <...>
**Recommended default:** <Option X> — rationale: <...>  
**Status:** Pending — requires project owner input before this decision is binding.  
**Agent interim behavior:** Apply recommended default; annotate output with
  `[SDR-NNN PENDING]` so the project owner can review.
```

---

## Artifact 4: `term-glossary.md`

```markdown
# Term Glossary — <Collection Name>
Generated: <ISO date>

Terms are listed alphabetically. Source priority follows the collection's
priority order. Where sources conflict, the higher-priority source wins
and the conflict is noted.

---

## <Term — canonical spelling and capitalization>

**Definition:** <Verbatim from source if normative; paraphrased if descriptive>  
**Source:** <Document identifier>, Section <X.X>  
**Normative:** <yes | no>  
**Synonyms / variants:** <list, or "none">  
**Deprecated forms:** <list what NOT to use, or "none">  
**Usage notes:** <Context-specific meaning, constraints, or common misuse — or omit if none>  
**NIST control reference:** <e.g., AC-3, AU-12 — or omit if not applicable>

---
```

### Mandatory glossary entries

The following must always be present when the domain-references collection
is processed, populated verbatim from their normative sources:

**From NIST SP 800-53:**
- All control family names (AC, AU, CM, IA, SC, SI, etc.)
- Controlled Unclassified Information (CUI)
- Security Control, Security Control Baseline, Overlay
- Authorization Boundary, System Security Plan (SSP)

**From MIL-STD-38784A:**
- Warning (exact definition from the standard)
- Caution (exact definition from the standard)
- Note (exact definition from the standard)

**From Common Criteria:**
- Security Functional Requirement (SFR)
- Security Target (ST)
- Protection Profile (PP)
- Evaluation Assurance Level (EAL)

**From SELinux / RHEL:**
- Multi-Level Security (MLS)
- Multi-Category Security (MCS)
- Security context
- Type Enforcement (TE)
- Sensitivity label
- Category
