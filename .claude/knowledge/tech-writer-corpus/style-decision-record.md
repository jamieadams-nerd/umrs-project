# Style Decision Record — UMRS Project
Generated: 2026-03-16
Owner: tech-writer (pending project owner review for flagged items)

This record resolves tensions identified in the cross-reference map. Entries here take precedence over any individual source document for UMRS documentation.

---

## SDR-001: Contractions in technical documentation

**Tension:** Microsoft Writing Style Guide actively encourages contractions (it's, you'll, we're). NIST Author Instructions and MIL-STD-38784B use a formal register that excludes contractions.
**Sources involved:** Microsoft Writing Style Guide, NIST Author Instructions, MIL-STD-38784B, Google Developer Style Guide
**Decision:** Contractions are permitted in developer-facing documentation (devel/ module, patterns/ module, umrs-tools/ module). Contractions are not used in formal reference documents (reference/ module, compliance-framework pages) or in content that will be submitted as part of a formal NIST or DoD publication.
**Applies when:** devel/, patterns/, umrs-tools/, operations/ (operator guide prose sections)
**Does not apply when:** reference/ compliance pages, architecture/ formal design rationale, any content destined for formal government submission
**Rationale:** Operator-facing and developer-facing content benefits from the warmer Microsoft register. Auditor-facing and compliance-referencing content maintains the formal register expected by that audience.
**Status:** Resolved

---

## SDR-002: "Please" in instructions

**Tension:** Google Developer Style Guide explicitly prohibits "please" in procedures. Microsoft does not address it but encourages warmth.
**Sources involved:** Google Developer Style Guide, Microsoft Writing Style Guide
**Decision:** Do not use "please" in procedure steps. This aligns with the Google guide and with UMRS STE Mode rules (which use imperative verbs). "Please" adds no information and can slow operators.
**Applies when:** All numbered procedure steps in any UMRS module
**Does not apply when:** Non-procedural introductory prose (a sentence like "Please note the prerequisite below" is acceptable in a paragraph but not in a numbered step)
**Rationale:** UMRS targets security operators in high-stakes environments. Step language must be direct.
**Status:** Resolved

---

## SDR-003: Formality register for operations documentation

**Tension:** MIL-STD-38784B's formal DoD technical manual register vs. Google/Microsoft's conversational developer register.
**Sources involved:** MIL-STD-38784B, Google Developer Style Guide, Microsoft Writing Style Guide
**Decision:** UMRS operations and deployment documentation uses the UMRS STE Mode register: imperative steps, active voice, sentence case headings, AsciiDoc admonitions — not the full DoD formal TM structure. Introductory and conceptual paragraphs in the same documents use the Google/Microsoft conversational register (second person, direct language). The full MIL-STD-38784B structure (SGML, service-specific formatting, TOC on right-hand page) does not apply to UMRS Antora docs.
**Applies when:** All UMRS Antora documentation
**Does not apply when:** If UMRS produces an actual DoD TM (TM identification number, ICS/TDMA systems) — that would require full MIL-STD-38784B compliance; flag to project owner
**Rationale:** UMRS is not producing printed DoD TMs. The Antora-based documentation system has its own formatting conventions. MIL-STD principles (one action per step, Warning > Caution > Note hierarchy) apply but not MIL-STD's typographic and structural format requirements.
**Status:** Resolved

---

## SDR-004: Warning/Caution/Note admonition definitions

**Tension:** MIL-STD-38784B defines a three-tier hierarchy (Warning, Caution, Note) with strict scope definitions. AsciiDoc (and UMRS STE Mode) uses a four-type set: NOTE, TIP, IMPORTANT, WARNING, CAUTION.
**Sources involved:** MIL-STD-38784B, UMRS STE Mode rules, Google Developer Style Guide
**Decision:** UMRS uses AsciiDoc admonition types. Map them to MIL-STD intent as follows:
- `WARNING` — use when a step could cause data loss, security breach, or system damage (maps to MIL-STD Warning for DoD-severity events, or Caution for equipment-level events)
- `CAUTION` — use when a step is irreversible or security-critical but not immediately destructive
- `NOTE` — use for supplementary information the operator should know
- `IMPORTANT` — use when missing the information would cause procedure failure, but no physical or data risk exists
Do not use MIL-STD definitions verbatim; apply the spirit of the hierarchy within AsciiDoc constraints.
**Applies when:** All UMRS Antora procedure documentation
**Does not apply when:** Generating actual DoD TM content
**Rationale:** AsciiDoc is the rendering format. The four-type model is more granular than MIL-STD's three types and provides useful additional precision for UMRS docs.
**Status:** Resolved

---

## SDR-005: Second person ("you") vs. third person

**Tension:** Google Developer Style Guide mandates "you" throughout. NIST publications often use third person in technical descriptions.
**Sources involved:** Google Developer Style Guide, NIST Author Instructions, Federal Plain Language Guidelines
**Decision:** Use second person ("you") throughout all developer-facing and operator-facing UMRS documentation. Use third person only when describing a system's behavior (e.g., "The daemon reads the policy file at startup") where "you" would be awkward or incorrect. Do not use third person in procedure steps.
**Applies when:** All UMRS Antora documentation
**Does not apply when:** Describing system behavior (the system is the actor, not the operator); formal security documentation submitted to a government body
**Rationale:** UMRS targets developers and operators who are doing something. Second person is clearer and matches the Plain Language guidance to "address one person, not a group."
**Status:** Resolved

---

## SDR-006: Inclusive terminology authority

**Tension:** NIST Author Instructions (normative, updated Feb 2025) and Google Developer Style Guide (living web guide) may diverge over time on specific inclusive terms.
**Sources involved:** NIST Author Instructions, Google Developer Style Guide, Microsoft Writing Style Guide
**Decision:** When NIST and the commercial guides agree, use the shared term. When they diverge, NIST takes precedence for UMRS documentation (government audience, compliance context). When only the commercial guides address a term and NIST is silent, use the Google guide as primary and Microsoft as secondary.
**Applies when:** All UMRS documentation
**Rationale:** UMRS operates in a government/DoD compliance context. NIST authority is appropriate for this environment.
**Status:** Resolved

---

## SDR-007: Sentence length targets

**Tension:** Federal Plain Language Guidelines target 15–20 words average. Google accessibility guidance sets 26 words as a ceiling.
**Sources involved:** Federal Plain Language Guidelines, Google Developer Style Guide (accessibility page)
**Decision:**
- Procedure steps: maximum 20 words per sentence (apply STE Mode limit of 20 words)
- Conceptual and explanatory prose: target 15–20 words; absolute ceiling 26 words
- Architecture and rationale sections: may exceed 20 words when technical precision requires it; never exceed 30 words
**Applies when:** All UMRS Antora documentation
**Rationale:** Shorter sentences reduce cognitive load for operators under time pressure. The STE Mode rule is already in place; this decision extends the spirit of that rule to non-procedural prose.
**Status:** Resolved

---

## SDR-008: Code font for command-line utility names

**Tension:** Google Developer Style Guide says command-line utility names (gcloud, kubectl, etc.) should use code font when they appear in prose. This is sometimes omitted in informal writing.
**Sources involved:** Google Developer Style Guide (code-in-text page)
**Decision:** Always use code font for command-line utility names when they appear in prose. This includes: `sestatus`, `semanage`, `restorecon`, `ausearch`, `cargo`, `rustup`, `make`, `systemctl`, and all other CLI tools referenced in UMRS documentation.
**Applies when:** All UMRS Antora documentation
**Rationale:** Code font signals to the reader that the term is verbatim text to be entered or recognized; it also improves scannability.
**Status:** Resolved

---

## SDR-009: "Optional:" step prefix format

**Tension:** Google style specifies "Optional:" (with colon, not in parentheses). Some existing UMRS docs may use "(Optional)".
**Sources involved:** Google Developer Style Guide
**Decision:** Use "Optional:" (with colon, no parentheses) at the start of optional procedure steps. Correct any existing "(Optional)" occurrences when editing those documents.
**Applies when:** All UMRS Antora procedure steps
**Rationale:** The colon form is cleaner, consistent with Google style, and matches how other step prefixes (e.g., "Note:") work in AsciiDoc prose.
**Status:** Resolved

This record resolves tensions identified in the cross-reference map.
Entries here take precedence over any individual source document.

---

## SDR-001: Tone formality — conversational vs. formal

**Tension:** Google and Microsoft style guides call for conversational, friendly tone with contractions. MIL-STD-38784B implies a formal register for technical documentation.
**Sources involved:** Google Developer Style Guide, Microsoft Writing Style Guide, MIL-STD-38784B
**Decision:** Apply document type and audience context.
- Architecture and concept pages (`architecture/`, `security-concepts/`): conversational but precise; no contractions.
- Developer documentation (`devel/`, `patterns/`): conversational; contractions acceptable.
- Operational procedures (`deployment/`, `operations/`): direct and precise; no contractions in numbered steps; conversational in introductory paragraphs.
- Any documentation destined for formal government submission or DoD review: MIL-STD formal register; no contractions; "shall/should/may" modal vocabulary.
**Applies when:** All UMRS documentation.
**Does not apply when:** Internal-only scratch files, comments, or agent notes.
**Rationale:** UMRS serves three audiences (new engineers, security auditors, potential adopters). Engineers and adopters benefit from conversational framing. Auditors and government reviewers expect formal register. Separate by document type rather than trying to satisfy both in one passage.
**Status:** Resolved

---

## SDR-002: Sentence length target — 15–20 words (PLAIN) vs. qualitative (MIL-STD)

**Tension:** Federal Plain Language Guidelines specifies average 15–20 words per sentence. MIL-STD-38784B requires short and concise sentences with RGL 9 but gives no word count.
**Sources involved:** Federal Plain Language Guidelines, MIL-STD-38784B
**Decision:** Use 15–20 words as the practical ceiling. In procedural steps, target 10–15 words per step (STE Mode rule). In architectural explanation paragraphs, 20 words is the upper limit for any single sentence. Split sentences that exceed 25 words.
**Applies when:** All UMRS documentation.
**Does not apply when:** Technical terms or code literals within a sentence that cannot be shortened without losing precision. Do not force-split a sentence that contains a required verbatim term.
**Rationale:** The 15–20 word target from PLAIN is measurable and aligns with MIL-STD's readability intent. Applying a concrete number makes editing decisions consistent across the writing team.
**Status:** Resolved

---

## SDR-003: Contractions in procedural text

**Tension:** Microsoft Writing Style Guide encourages contractions. MIL-STD-38784B is silent but implies a formal register.
**Sources involved:** Microsoft Writing Style Guide, MIL-STD-38784B
**Decision:** No contractions in numbered procedure steps. Contractions are acceptable in introductory paragraphs, conceptual explanations, and developer-facing content. Never in formal compliance statements, NIST-register content, or any passage explicitly citing a control requirement.
**Applies when:** Numbered procedure steps in all modules.
**Does not apply when:** Introductory paragraphs, architecture explanations, devel/ tutorials.
**Rationale:** Procedure steps are read in high-stakes operational contexts. Contractions reduce precision. Introductory paragraphs set tone and can be warmer. This matches existing STE Mode rules.
**Status:** Resolved

---

## SDR-004: Procedure step count — 7-step limit (Microsoft) vs. no limit (MIL-STD)

**Tension:** Microsoft Writing Style Guide limits procedures to seven steps. MIL-STD-38784B defines no maximum step count but limits substep depth to four levels.
**Sources involved:** Microsoft Writing Style Guide, MIL-STD-38784B
**Decision:** Target seven or fewer main steps per procedure. When a procedure requires more than seven steps, divide it into sub-procedures with headings (Prerequisites, Part 1: Configure X, Part 2: Verify Y). Never use more than four levels of substep depth. Apply the four-level depth limit from MIL-STD as a hard ceiling.
**Applies when:** All UMRS procedures in `deployment/`, `operations/`, `devel/`.
**Does not apply when:** Reference material listing multiple independent commands where each "step" is a standalone lookup item rather than a sequential action.
**Rationale:** Seven steps is the practical limit for short-term memory. Longer procedures should be structured as multiple named parts rather than a single long list. The MIL-STD depth limit prevents nested-step hell.
**Status:** Resolved

---

## SDR-005: "Shall" vs. present tense for requirements

**Tension:** MIL-STD-38784B uses "shall/should/may/will" modal vocabulary with defined meanings. Federal Plain Language Guidelines and commercial style guides prefer plain English: "must" over "shall," present tense over modal constructs.
**Sources involved:** MIL-STD-38784B, Federal Plain Language Guidelines, Google Developer Style Guide
**Decision:** Context-dependent by document type.
- Normative statements in UMRS specifications (any document that will be reviewed as a requirements document): use "shall" (mandatory), "should" (recommended), "may" (permitted), "will" (declaration of purpose). This matches MIL-STD and NIST SP vocabulary.
- Descriptive and procedural documentation (architecture/, devel/, operations/, deployment/): use present tense ("the system verifies," "the operator must," "you configure"). Avoid "shall" in non-normative text — it reads as overreaching.
- NIST control citations in doc comments: use "must" for the reader-facing statement and cite the NIST control separately.
**Applies when:** All UMRS documentation.
**Does not apply when:** Verbatim quotations from standards where "shall" appears in the source.
**Rationale:** Mixing modal vocabulary confuses readers. "Shall" in a tutorial reads as authoritative demand; "must" reads as a practical requirement. Separating normative from descriptive documents allows each to use the vocabulary their audience expects.
**Status:** Resolved

---

## SDR-006: Heading style — all caps (MIL-STD) vs. sentence case (Google/Microsoft)

**Tension:** MIL-STD-38784B requires all-capital letters for chapter, section, and appendix headings, and capital letters + underscoring for primary sideheads. Google and Microsoft require sentence case.
**Sources involved:** MIL-STD-38784B, Google Developer Style Guide, Microsoft Writing Style Guide
**Decision:** Sentence case for all UMRS Antora documentation. MIL-STD heading rules apply to print TMs and do not apply to web-rendered AsciiDoc content. UMRS documentation is an Antora-based documentation site, not a DoD Technical Manual submission.
**Applies when:** All Antora AsciiDoc pages.
**Does not apply when:** Any document explicitly prepared as a DoD TM submission, DoD deliverable, or formal government document. In those cases, apply MIL-STD-38784B heading rules.
**Rationale:** UMRS produces web documentation, not print TMs. Sentence case is the correct choice for the medium and the audience.
**Status:** Resolved

---

## SDR-007: WARNING/CAUTION/NOTE hierarchy — severity levels

**Tension:** MIL-STD-38784B defines a strict three-tier hierarchy: WARNING (injury/death), CAUTION (equipment damage), NOTE (informational). The severity definitions drive placement and emphasis. Google Style Guide and Antora use NOTE, CAUTION, WARNING, IMPORTANT without explicit severity definitions.
**Sources involved:** MIL-STD-38784B, Google Developer Style Guide
**Decision:** Apply MIL-STD semantic definitions to UMRS AsciiDoc admonition usage:
- `WARNING` — Use when the reader's action could cause data loss, system compromise, security policy violation, or irreversible damage. Equivalent to MIL-STD WARNING + CAUTION combined for a software/security context.
- `CAUTION` — Use when a specific action could produce unexpected results or degrade system integrity in a recoverable way.
- `NOTE` — Use for supplementary information that does not affect safety or correctness.
- `IMPORTANT` — Use for information the reader must not overlook; stronger than NOTE but lower risk than CAUTION.
- `TIP` — Optional; use sparingly for non-critical shortcuts or best practices.
**Applies when:** All UMRS Antora documentation admonitions.
**Does not apply when:** Formal government TM submissions — use MIL-STD's exact three-tier vocabulary and placement rules.
**Rationale:** UMRS operates in a high-assurance security environment. The severity distinction matters. "WARNING" should not be used for minor reminders. Calibrating admonition levels to MIL-STD severity intent maintains the signal-to-noise ratio that security operators depend on.
**Status:** Resolved

---

## SDR-008: Abbreviation handling — minimize (PLAIN) vs. manage (MIL-STD)

**Tension:** Federal Plain Language Guidelines treats abbreviations as a "menace to prose" and recommends avoiding them. MIL-STD-38784B accepts abbreviations after first use (spelled out in each chapter/section) with all abbreviations defined in the foreword.
**Sources involved:** Federal Plain Language Guidelines, MIL-STD-38784B
**Decision:** Apply MIL-STD's define-on-first-use rule across UMRS documentation, with an added constraint: do not introduce abbreviations for terms used fewer than three times in a page. For terms used frequently throughout a document, define on first use and use the abbreviation consistently thereafter. For cross-module documents, re-define on first use per module (don't assume the reader carries context from a prior page). Maintain the Approved Terminology List at `.claude/agent-memory/doc-team/approved_terminology.md` as the canonical source.
**Applies when:** All UMRS documentation.
**Does not apply when:** Reference tables and glossary entries, which list both the full term and abbreviation.
**Rationale:** Security documentation contains many necessary abbreviations (NIST, CUI, MLS, MCS, TPI, etc.). Avoiding them entirely would make prose unreadable. The MIL-STD approach of defining on first use balances clarity with efficiency.
**Status:** Resolved

---

## SDR-009: Persona address in architecture and concept pages

**Tension:** Google and Microsoft recommend second person ("you") throughout. Federal Plain Language Guidelines recommends "you" and "we." UMRS architecture pages and security concept explanations traditionally use third person ("the system," "the operator," "the developer") for authority and precision.
**Sources involved:** Google Developer Style Guide, Microsoft Writing Style Guide, Federal Plain Language Guidelines
**Options identified:**
  1. Second person throughout — consequence: warms the tone but can feel awkward in formal design rationale ("You must consider the reference monitor...")
  2. Third person for architecture, second person for procedures and tutorials — consequence: consistent with current UMRS practice; requires explicit per-page decisions
  3. Mixed by audience section within a page — consequence: complex to maintain; risk of inconsistency
**Decision:** Option 2 — third person for `architecture/` and `security-concepts/` explanations; second person for `devel/`, `deployment/`, `operations/` procedures.
**Rationale:** Consistent with current UMRS practice. Architecture and security-concepts pages describe system properties with authority; procedures and tutorials address the reader directly.
**Status:** Resolved (approved by Jamie, 2026-03-16)

---

## SDR-010: Inclusive language in security domain terms

**Tension:** Microsoft Writing Style Guide recommends replacing "master/slave" with "primary/subordinate" and "DMZ" with "perimeter network." Some UMRS technical contexts use these terms (e.g., in SELinux policy discussions, network architecture). Changing terms in normative security contexts may reduce clarity for security professionals.
**Sources involved:** Microsoft Writing Style Guide (bias-free-communication.md)
**Options identified:**
  1. Apply Microsoft's replacements consistently — consequence: more inclusive; may confuse readers expecting standard security terminology
  2. Use inclusive terms in narrative/explanation; use standard terms in formal technical specifications — consequence: two-tier vocabulary; manageable with clear policy
  3. Keep standard terms throughout — consequence: does not comply with Microsoft/Google inclusive language guidance
**Decision:** Option 2 — use inclusive terms in architecture and developer documentation; use standard terms when the term is a technical identifier (e.g., in SELinux configuration examples where the exact term matters). When quoting a specification verbatim that uses deprecated terms, quote exactly and add an editorial note.
**Rationale:** Two-tier approach respects both inclusive language standards and technical precision. Security professionals expect standard terms in technical specifications; editorial notes bridge the gap.
**Status:** Resolved (approved by Jamie, 2026-03-16)

---

## SDR-011: SFR element prose register (Common Criteria)

**Tension:** CC structured English for Security Functional Requirements uses a formal register with specific notation: bold element text, italics for operations (assignment, selection, refinement, iteration), "shall" modal verb. This conflicts with Plain Language Guidelines (use "must", avoid formal register) and UMRS STE mode (approved verb list, short sentences).
**Sources involved:** Common Criteria CC:2022 Parts 1 & 2, Federal Plain Language Guidelines, STE Mode rules
**Decision:** Two registers coexist. When writing or citing CC SFR elements verbatim, use CC structured English (bold, italics, "shall"). When explaining what an SFR means in UMRS context, use plain language and UMRS conventions ("must", active voice, short sentences). The two registers serve different reader purposes and must not be blended within the same paragraph.
**Applies when:** Any documentation that cites or describes CC SFRs, Security Targets, or Protection Profiles
**Does not apply when:** General UMRS documentation that does not reference CC constructs
**Rationale:** CC structured English is a normative format defined by ISO/IEC 15408. Rewriting SFR elements into plain language would change their meaning and break traceability to the standard. The surrounding explanation prose is where plain language applies.
**Status:** Resolved (2026-03-16)
