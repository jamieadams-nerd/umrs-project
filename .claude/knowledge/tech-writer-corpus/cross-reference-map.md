# Cross-Reference Map — tech-writer-corpus
Generated: 2026-03-16

---

## Agreements

### Active voice in procedures
**Documents in agreement:** Federal Plain Language Guidelines, Google Developer Style Guide, Microsoft Writing Style Guide, MIL-STD-38784B (implied by imperative step format)
**Shared guidance:** All four sources require active voice in procedural steps. The actor must be explicit in every instruction.

### Sentence case for headings
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Use sentence case for all headings and document titles — capitalize only the first word and proper nouns.

### Serial comma (Oxford comma)
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** In a list of three or more items, include a comma before the conjunction.

### Numbered lists for procedures
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide, MIL-STD-38784B, Federal Plain Language Guidelines
**Shared guidance:** Use numbered lists for sequential steps; use bulleted lists for non-sequential items.

### One action per step
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide, MIL-STD-38784B (implied)
**Shared guidance:** Each numbered step contains exactly one action. Split multi-action steps into separate steps.

### Introduce lists with a complete sentence
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** A list must be preceded by a complete introductory sentence or a fragment ending in a colon. Never use a partial sentence that is completed by the list items.

### Descriptive link text
**Documents in agreement:** Google Developer Style Guide, NIST Author Instructions (meaningful hyperlink text per Section 508), Microsoft Writing Style Guide (scan-first design)
**Shared guidance:** Link text must be descriptive and meaningful without surrounding context. Never use "click here" or "this document."

### Avoid jargon and abbreviations
**Documents in agreement:** Federal Plain Language Guidelines, Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Define technical terms on first use. Prefer simplified names over abbreviations. Avoid unexplained jargon.

### Inclusive terminology: allowlist/denylist
**Documents in agreement:** NIST Author Instructions (canonical), Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Use "allowlist" and "denylist" (not whitelist/blacklist) in all new writing. When established code uses the old term, acknowledge it once in parentheses, then use the preferred term.

### Inclusive terminology: primary/secondary
**Documents in agreement:** NIST Author Instructions, Microsoft Writing Style Guide, Google Developer Style Guide
**Shared guidance:** Use "primary/secondary" or "primary/replica" instead of "master/slave."

### Short sentences and paragraphs
**Documents in agreement:** Federal Plain Language Guidelines (15–20 words target), Google Developer Style Guide (fewer than 26 words per sentence for accessibility), Microsoft Writing Style Guide (crisp minimalism)
**Shared guidance:** Keep sentences short. Break long paragraphs into shorter ones with white space.

### Accessibility: alt text for images
**Documents in agreement:** NIST Author Instructions (Section 508 compliance), Google Developer Style Guide
**Shared guidance:** Every image must have alt text that accurately describes the content or intent.

### Code font for code-related text
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Put filenames, paths, commands, class names, method names, environment variables, and language keywords in code font.

---

## Tensions

### Contractions in technical documentation
**Documents in conflict:** Microsoft Writing Style Guide vs. NIST Author Instructions / MIL-STD-38784B
**Microsoft position:** Encourage contractions (it's, you'll, we're, let's) as a primary voice element.
**NIST/MIL-STD position:** No explicit prohibition, but the formal register of NIST publications and DoD TMs does not employ contractions.
**Google position:** Permits contractions; does not discourage or actively promote them.
**Nature of conflict:** context-dependent
**Resolution:** See `style-decision-record.md` → SDR-001

### "Please" in instructions
**Documents in conflict:** Google Developer Style Guide vs. implicit colloquial usage
**Google position:** Do not use "please" in instructions — overdoes politeness and adds no information.
**Microsoft position:** Not addressed explicitly; "warm and relaxed" voice could tolerate "please" occasionally.
**Nature of conflict:** context-dependent; Google is explicit, Microsoft is silent
**Resolution:** See `style-decision-record.md` → SDR-002

### Formality register for operations documentation
**Documents in conflict:** MIL-STD-38784B (DoD formal) vs. Google/Microsoft (conversational)
**MIL-STD-38784B position:** Formal technical manual structure; imperative steps; no conversational elements.
**Google/Microsoft position:** Friendly, conversational, approachable; second person throughout.
**Nature of conflict:** scope difference — MIL-STD targets printed DoD TMs; Google/Microsoft target web-based developer docs
**Resolution:** See `style-decision-record.md` → SDR-003

### Warning/Caution/Note admonition definitions
**Documents in conflict:** MIL-STD-38784B vs. Google Developer Style Guide vs. UMRS STE Mode rules
**MIL-STD-38784B position:** Strict hierarchy — Warning (injury/death) > Caution (equipment/mission) > Note (supplementary). Prescribes exact placement and formatting.
**Google position:** Uses Note, Caution, Warning as notice types without the same strict DoD hierarchy.
**UMRS STE Mode position:** AsciiDoc admonition syntax (NOTE, WARNING, CAUTION, IMPORTANT); four types, not three.
**Nature of conflict:** scope difference — MIL-STD is normative for DoD TMs; UMRS uses AsciiDoc's four-type set
**Resolution:** See `style-decision-record.md` → SDR-004

### Sentence length limits
**Documents in conflict:** Federal Plain Language Guidelines vs. Google accessibility guidance
**Plain Language position:** Average of 15–20 words per sentence.
**Google accessibility position:** Fewer than 26 words per sentence.
**Nature of conflict:** different upper bounds for different purposes; not contradictory
**Resolution:** Apply 15–20 words as the UMRS procedural target. Use 26 words as an absolute ceiling for conceptual and explanatory content.

### Second person ("you") vs. third person
**Documents in conflict:** Google Developer Style Guide vs. NIST Author Instructions
**Google position:** Always use second person "you" in documentation.
**NIST position:** Plain Language encourages "you" and "we"; formal NIST publications often use third person for technical descriptions.
**Nature of conflict:** audience and document type drive the choice
**Resolution:** See `style-decision-record.md` → SDR-005

---

## Chains (deference relationships)

### Google Developer Style Guide → Merriam-Webster / Chicago Manual of Style
**Primary:** Google Developer Style Guide
**Defers to:** Merriam-Webster for word spellings and usage; Chicago Manual of Style for grammar and punctuation questions not in the guide
**Agent behavior:** Consult Google guide first. If not covered, consult Microsoft guide. If still not covered, consult Merriam-Webster or Chicago Manual.

### Microsoft Writing Style Guide → Merriam-Webster / Chicago Manual of Style
**Primary:** Microsoft Writing Style Guide
**Defers to:** Merriam-Webster Dictionary and The Chicago Manual of Style
**Agent behavior:** Same chain as Google; both commercial guides share the same tertiary references.

### NIST Author Instructions → Plain Writing Act / ANSI/NISO Z39.18 / Section 508
**Primary:** NIST Author Instructions
**Defers to:** Plain Writing Act of 2010 for plain language requirements; ANSI/NISO Z39.18-2005 for document structure; Section 508 for accessibility
**Agent behavior:** For UMRS documents that cite or reference NIST standards, treat NIST conventions as authoritative and commercial guides as supplementary.

### UMRS STE Mode rules → this corpus
**Primary:** UMRS STE Mode rules (`.claude/rules/ste_mode.md`)
**Defers to:** MIL-STD-38784B for Warning/Caution/Note hierarchy; Federal Plain Language Guidelines for prose quality
**Agent behavior:** STE Mode rules take precedence for UMRS procedural content. Use this corpus to fill gaps in STE Mode guidance.

---

## Gaps

### Formal security documentation (auditor reports, findings, SSPs)
**Not covered by:** any document in this collection
**Agent behavior:** When writing formal security findings or audit reports, flag the gap to the user. Apply Plain Language principles as the closest available guidance.

### AsciiDoc-specific formatting conventions
**Not covered by:** any document in this collection
**Agent behavior:** This corpus covers HTML, Markdown, and Word/PDF formatting. Apply the semantic intent from this corpus to AsciiDoc equivalents (code font → `+monospace+`, bold → `*bold*`). Consult existing UMRS `.adoc` files for project conventions.

### Rust API documentation style (rustdoc)
**Not covered by:** any document in this collection
**Agent behavior:** Apply Google guide code-in-text guidance for naming conventions. Apply CLAUDE.md rules for Rust-specific annotation requirements. Flag rustdoc-specific questions to the user.

### Diagram and figure caption conventions
**Not substantially covered by:** any document in this collection
**Agent behavior:** Use NIST figure caption conventions as the default (concise captions; detail in body text). Flag complex diagram labeling questions to the user.

### MIL-STD-38784B full text
**Limitation:** The PDF was downloaded and is at `gov-standards/MIL-STD-38784B.pdf` but was not directly readable in this familiarization pass (binary PDF format). Key concepts extracted from the summary file.
**Agent behavior:** For detailed MIL-STD-38784B questions (exact warning format, specific section requirements), note that the PDF exists and may need to be read directly via RAG query. Key concepts (Warning > Caution > Note hierarchy; procedure format; CUI marking) are captured in the concept index.

---

## Agreements

### Active voice
**Documents in agreement:** Federal Plain Language Guidelines, Google Developer Style Guide, Microsoft Writing Style Guide, MIL-STD-38784B (§4.8.2)
**Shared guidance:** All four sources require active voice. Name the actor. Use second-person imperative for procedures.

### Sentence case for headings
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Both require sentence case (capitalize first word and proper nouns only) for all document titles and section headings.

### Serial (Oxford) comma
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Both require a comma before the final item in a list of three or more.

### Simple, direct verbs
**Documents in agreement:** Federal Plain Language Guidelines, Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** All three prefer short, familiar, concrete words. Avoid "utilize," "facilitate," "implement," "leverage" — use "use," "help," "carry out," "apply" instead.

### Numbered lists for sequences
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide, MIL-STD-38784B
**Shared guidance:** Use numbered lists for ordered steps and sequences; use bulleted lists for unordered items.

### Introductory sentence before a list or procedure
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Introduce lists and procedures with a complete sentence or a fragment ending with a colon.

### Parallel structure in lists and headings
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** All items in a list and all headings at the same level must use the same grammatical structure.

### Avoid condescending simplicity markers
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Do not use "simply," "just," "easily," "obviously" — these are condescending and frequently inaccurate.

### Conditions before instructions
**Documents in agreement:** Google Developer Style Guide (explicit), MIL-STD-38784B (implied by procedure format)
**Shared guidance:** State the context or condition before describing the action. "To start X, click Y" not "Click Y to start X."

### One action per step
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide, MIL-STD-38784B (§4.7.11.5.3)
**Shared guidance:** Each step in a procedure contains one action. Split combined actions into separate steps.

### Bias-free and gender-neutral language
**Documents in agreement:** Google Developer Style Guide (inclusive-documentation.md), Microsoft Writing Style Guide (bias-free-communication.md)
**Shared guidance:** Use gender-neutral alternatives; avoid master/slave; singular "they" is acceptable.

### Writing for a global audience
**Documents in agreement:** Google Developer Style Guide, Microsoft Writing Style Guide
**Shared guidance:** Use simple verbs; avoid idioms, cultural references, and ambiguous language to support translation and global readers.

---

## Tensions

### Tone formality: conversational vs. formal
**Documents in conflict:** Google Developer Style Guide + Microsoft Writing Style Guide vs. MIL-STD-38784B
**Google/Microsoft position:** Conversational, friendly, warm. Use contractions. Informal is fine. "Write like you speak."
**MIL-STD position:** Formal, precise. Technical content in "language free of vague and ambiguous terms." No contractions mentioned.
**Nature of conflict:** context-dependent
**Resolution:** See `style-decision-record.md` → SDR-001

### Sentence length target
**Documents in conflict:** Federal Plain Language Guidelines vs. MIL-STD-38784B
**PLAIN position:** Average 15–20 words per sentence.
**MIL-STD position:** Sentences shall be short and concise; Reading Grade Level 9 target; no specific word count.
**Nature of conflict:** scope difference (PLAIN provides a measurable target; MIL-STD is qualitative)
**Resolution:** See `style-decision-record.md` → SDR-002

### Contractions in procedural text
**Documents in conflict:** Microsoft Writing Style Guide (encourage contractions) vs. MIL-STD-38784B (no guidance; formal register implied)
**Microsoft position:** Use contractions ("it's," "you'll") — makes text natural and friendly.
**MIL-STD position:** Silent; formal register of DoD TMs implies contractions are inappropriate.
**Nature of conflict:** context-dependent
**Resolution:** See `style-decision-record.md` → SDR-003

### Number of procedure steps
**Documents in conflict:** Microsoft Writing Style Guide vs. MIL-STD-38784B
**Microsoft position:** Limit to seven steps, preferably fewer.
**MIL-STD position:** No maximum step count; max four levels of substep depth. Steps may continue across pages.
**Nature of conflict:** scope difference (Microsoft is editorial guidance; MIL-STD is physical formatting)
**Resolution:** See `style-decision-record.md` → SDR-004

### Single-step procedure format
**Documents in conflict:** Google Developer Style Guide vs. Microsoft Writing Style Guide
**Google position:** Use a bullet for single-step procedures (not a numbered list).
**Microsoft position:** Use a bullet for single-step procedures (consistent with numbered format otherwise).
**Nature of conflict:** Agreement, not a true conflict. Both say: use a bullet for one step.
**Resolution:** No tension. Both sources agree. Use a bullet.

### "Shall" vs. present tense for requirements
**Documents in conflict:** MIL-STD-38784B vs. Federal Plain Language Guidelines + Google + Microsoft
**MIL-STD position:** "Shall" for binding requirements; "should/may" for nonmandatory; "will" for declaration or futurity.
**PLAIN/Google/Microsoft position:** Use present tense ("the system requires," "you must") rather than shall/should constructs; plain English prefers "must" over "shall."
**Nature of conflict:** context-dependent
**Resolution:** See `style-decision-record.md` → SDR-005

### Heading style: title case vs. sentence case
**Documents in conflict:** MIL-STD-38784B vs. Google + Microsoft
**MIL-STD position:** Parts, chapters, sections, appendices headings SHALL be all capital letters (§4.7.5). Primary sideheads are in capital letters and underscored.
**Google/Microsoft position:** Sentence case for all document titles and section headings.
**Nature of conflict:** context-dependent (MIL-STD governs formal print TMs; Google/Microsoft govern web/developer docs)
**Resolution:** See `style-decision-record.md` → SDR-006

### Warning/Caution/Note vs. NOTE/WARNING/IMPORTANT
**Documents in conflict:** MIL-STD-38784B vs. Google Developer Style Guide
**MIL-STD position:** Strict three-tier hierarchy: WARNING (injury/death risk), CAUTION (equipment damage risk), NOTE (informational). Placement rules are specific.
**Google position:** Four notice types: Note, Caution, Warning, Important. No life-safety severity distinction explicitly.
**Nature of conflict:** scope difference (MIL-STD is designed for physical safety; Google for software docs)
**Resolution:** See `style-decision-record.md` → SDR-007

### Abbreviation handling
**Documents in conflict:** Federal Plain Language Guidelines vs. MIL-STD-38784B
**PLAIN position:** Abbreviations are a "menace to prose"; minimize them; use simplified names instead.
**MIL-STD position:** Spell out first use in each chapter/section/part; then abbreviation is acceptable; all abbreviations defined in foreword/intro.
**Nature of conflict:** scope difference (PLAIN aims to reduce abbreviations; MIL-STD manages them)
**Resolution:** See `style-decision-record.md` → SDR-008

---

## Chains (deference relationships)

### Google → Microsoft
**Primary:** Google Developer Style Guide
**Defers to:** Microsoft Writing Style Guide for topics not explicitly covered
**Agent behavior:** Consult Google Style Guide first for developer documentation. When Google provides no explicit guidance, apply Microsoft Style Guide.

### Google → Chicago Manual of Style
**Primary:** Google Developer Style Guide
**Defers to:** Chicago Manual of Style on grammar questions not covered
**Agent behavior:** For grammar edge cases not resolved by Google or Microsoft, consult Chicago Manual conventions (agent knowledge, not in corpus).

### NIST Author Instructions → Federal Plain Language Guidelines
**Primary:** NIST Author Instructions
**Defers to:** Federal Plain Language Guidelines (both share Plain Writing Act basis)
**Agent behavior:** NIST documents must comply with PLAIN. When writing NIST-register content, apply PLAIN guidelines as baseline.

### MIL-STD-38784B → US Government Printing Office Style Manual
**Primary:** MIL-STD-38784B (§4.8)
**Defers to:** US Government Printing Office Style Manual for capitalization, punctuation, compounding of words, and spelling of nontechnical words
**Agent behavior:** For capitalization and spelling questions in MIL-STD-register content, apply GPO Style Manual conventions (not in corpus — flag to user if needed).

### MIL-STD-38784B → DoD Dictionary of Military and Associated Terms
**Primary:** MIL-STD-38784B (§4.8.6)
**Defers to:** DoD Dictionary for military terminology
**Agent behavior:** When writing DoD-register content, military terms must match the DoD Dictionary definition. Flag when a UMRS term conflicts with a DoD Dictionary definition.

---

## Gaps

### AsciiDoc-specific formatting conventions
**Not covered by:** any document in this collection
**Agent behavior:** Apply Google/Microsoft general formatting principles as reasonable defaults. For AsciiDoc-specific questions (macros, admonitions, xrefs), rely on Antora/AsciiDoc documentation. Flag to user when a corpus-derived decision conflicts with Antora mechanics.

### Rustdoc and API reference writing conventions
**Not covered by:** any document in this collection (Google mentions API references briefly)
**Agent behavior:** Apply Google Developer Style Guide method documentation conventions ("Gets the...", "Checks whether...", "Sets the...") as the closest applicable guidance. Note the gap when writing new rustdoc content.

### Antora navigation and cross-reference conventions
**Not covered by:** any document in this collection
**Agent behavior:** This is UMRS project-specific. Use knowledge from CLAUDE.md and MEMORY.md; do not extrapolate from corpus.

### Security classification marking conventions (digital/web formats)
**Not covered in detail by:** any document except MIL-STD-38784B (which covers print/PDF TMs)
**Agent behavior:** Apply MIL-STD-38784B classification marking placement rules as the closest applicable standard. Flag when digital format (HTML/AsciiDoc) creates ambiguity that print rules do not resolve.

### Diataxis framework and modular documentation
**Not covered by:** any document in this collection
**Agent behavior:** These are UMRS-project-specific frameworks. Use CLAUDE.md and MEMORY.md guidance; do not extrapolate from corpus.

### Common Criteria structured English for SFR descriptions
**Not covered by:** any document in this collection (Common Criteria Parts 1 & 2 are listed in priority-order.md as desired but not acquired)
**Agent behavior:** Flag to user if UMRS documentation requires CC-format Security Functional Requirements. Do not attempt to write SFR text without the CC corpus.
