---
name: corpus-familiarization
description: >
  Perform a structured familiarization pass over RAG reference material so an
  agent knows what it knows. Use this skill whenever an agent has newly ingested
  reference material into the RAG corpus and needs to build active knowledge of
  it — not just passive retrieval. Trigger when the user says anything like
  "familiarize yourself with the corpus", "read through the reference material",
  "build your knowledge base", "scan the new docs", "do a familiarization pass",
  or "learn what's in the RAG". Also trigger proactively after any corpus
  ingestion event. The skill reads source documents from
  .claude/reference/<collection>/, processes them systematically, and produces
  four structured knowledge artifacts that belong in the agent's always-on
  context rather than in retrieval.
---

# Corpus Familiarization Skill

Builds active, always-on knowledge from RAG source material. Produces four
artifacts that function as the agent's internal map of what it knows — so it
can reason about the corpus before issuing any retrieval query.

**Source material location:** `.claude/reference/<collection>/`  
**Output location:** `.claude/knowledge/<collection>/`  
**Run:** Once after initial ingestion, then after any significant corpus update.

---

## Why this exists

RAG is pull-only. An agent retrieves only what it already knows to ask for.
Without a familiarization pass, blind spots in the agent's prior knowledge
become permanent blind spots — the corpus sits inert.

The four artifacts produced here are loaded into system-prompt context (not
retrieved), so the agent always knows *what* it knows and *where* to look
before formulating any retrieval query.

---

## Step 1 — Inventory the collection

1. List all files under `.claude/reference/<collection>/`.
2. Record: filename, approximate size, document type (style guide, standard,
   reference manual, regulatory, technical doc).
3. Group files into logical clusters if multiple collections are present.
4. Output a working inventory — do not skip any file.

If no collection is specified by the user, process all collections found
under `.claude/reference/`.

---

## Step 2 — Read and process each document

For each file in the inventory:

1. Read the full document. Do not skim. If the document exceeds context limits,
   process it in sequential chunks, maintaining continuity across chunks.
2. After reading, hold the content in working memory and proceed to Step 3
   before moving to the next document.
3. Track position in the inventory — do not skip documents.

**Processing order:** High-priority documents first. See
`references/priority-order.md` for the priority table for UMRS collections.
For collections not listed there, process in this order:
   - Regulatory / standards documents (NIST, MIL-STD, FIPS, CC)
   - Domain reference (vendor docs, SELinux, RHEL)
   - Style guides
   - Supplemental

---

## Step 3 — Produce the four knowledge artifacts

After processing all documents, write the following four files to
`.claude/knowledge/<collection>/`. These are the always-on artifacts.

### Artifact 1: `concept-index.md`

For each document processed, one entry containing:
- Document name and short identifier
- What it covers (2–4 sentences, your own words)
- Key terms and concepts it introduces (bulleted list)
- What writing tasks or agent decisions it governs
- Related documents in the corpus (cross-references)

See `references/artifact-formats.md` for the exact schema.

### Artifact 2: `cross-reference-map.md`

A structured map of relationships across documents:
- **Agreements** — where two or more documents reinforce the same guidance
- **Tensions** — where documents conflict or require a judgment call
  (e.g., MIL-STD-38784A warning language vs. Plain Language Guidelines
  brevity requirements — both valid, context-dependent)
- **Chains** — where one document defers to another
  (e.g., Google Style Guide defers to Chicago Manual on grammar questions)
- **Gaps** — topics the corpus does not cover that the agent should flag
  when encountered

This map is the agent's conflict-resolution reference. When two sources
disagree, consult this artifact first.

### Artifact 3: `style-decision-record.md`

Project-specific resolutions to cross-reference tensions. Captures:
- The tension or choice point
- Which source takes precedence for this project, and why
- Any context conditions that change the ruling
  (e.g., "use Plain Language for operator procedures; use CC structured
  English for Security Target SFR descriptions")

Seed this artifact with decisions derivable from the corpus alone. Leave
explicit placeholders for decisions that require project owner input.

See `references/artifact-formats.md` for the schema and placeholder format.

### Artifact 4: `term-glossary.md`

Canonical terminology extracted from the domain reference documents.
For each term:
- Canonical spelling and capitalization (authoritative source wins)
- Definition (verbatim from source where the source is normative;
  paraphrased where the source is descriptive)
- Source document and section
- Synonyms or deprecated variants to avoid
- Usage notes if the term has context-specific meaning

Priority sources for canonical terms (in order):
1. NIST SP 800-53 / 800-171 (control names, control families)
2. CMMC Assessment Guide (practice identifiers)
3. Common Criteria (SFR component names)
4. MIL-STD-38784A (warning/caution/note definitions)
5. RHEL / SELinux documentation (product-specific terminology)
6. Style guides (writing-process terminology)

When sources conflict on a term, log it in the cross-reference map and
apply the priority order above.

---

## Step 4 — Write the collection summary

Write `.claude/knowledge/<collection>/README.md` containing:
- Collection name and date of familiarization pass
- Document count and total coverage
- One-paragraph summary of what the collection covers
- List of the four artifact files with one-line descriptions
- Any significant gaps or open questions flagged during processing

---

## Step 5 — Report to the user

Report completion with:
- Documents processed (count and list)
- Artifacts written (paths)
- Top 3–5 notable findings: significant tensions found, important gaps,
  anything that will materially affect how the agent writes

Do not summarize the entire corpus in the report — the artifacts exist for
that. The report is a completion notice, not a document dump.

---

## Maintenance

Re-run this skill (or the relevant steps) when:
- New documents are added to a collection
- A document is updated to a new version
- A cross-reference tension is resolved by a project decision
  (update `style-decision-record.md` only)

For incremental updates (single new document), run Steps 2–3 for that
document only, then update all four artifacts for any entries that changed.

---

## Reference files

- `references/priority-order.md` — Processing priority for UMRS collections
- `references/artifact-formats.md` — Exact schemas for the four artifact types

