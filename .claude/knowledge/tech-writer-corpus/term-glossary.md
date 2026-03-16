# Term Glossary — tech-writer-corpus
Generated: 2026-03-16

Terms are listed alphabetically. Source priority follows the collection's priority order.
Where sources conflict, the higher-priority source wins and the conflict is noted.

Priority order: MIL-STD-38784B > Federal Plain Language Guidelines > NIST Author Instructions > Common Criteria (CC:2022) > Google Developer Style Guide > Microsoft Writing Style Guide

---

## Active voice

**Definition:** A sentence construction in which the subject of the sentence is the actor — the person or system performing the action. Eliminates ambiguity about who is responsible for what.
**Source:** Federal Plain Language Guidelines, Section III
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** passive voice (in procedural steps)
**Usage notes:** Active voice is required in all UMRS procedure steps. Use passive voice only when no actor exists (the law itself is the actor) or when the identity of the actor is genuinely irrelevant to the instruction.

---

## allowlist

**Definition:** A list of approved entities (users, IP addresses, applications, etc.) that are permitted to access a resource or perform an action.
**Source:** NIST Author Instructions (inclusive terminology table, Feb 2025)
**Normative:** yes
**Synonyms / variants:** allowlist, allow list (both acceptable per Google; prefer closed form "allowlist")
**Deprecated forms:** whitelist, white list
**Usage notes:** When a codebase or established standard uses "whitelist," acknowledge it once in parentheses on first use, then use "allowlist" throughout.

---

## alt text

**Definition:** Alternative text attribute embedded in an image element that describes the content or intent of the image for users who cannot see it.
**Source:** NIST Author Instructions (Section 508 compliance); Google Developer Style Guide (accessibility)
**Normative:** yes (required by Section 508 of the Rehabilitation Act for federal agency electronic publications)
**Synonyms / variants:** alternative text, alt attribute
**Deprecated forms:** none
**Usage notes:** Every image in UMRS documentation must have alt text. If an image is purely decorative, use empty alt text. Do not describe the image literally — describe what it communicates.

---

## Evaluation Assurance Level (EAL)

**Definition:** A package of security assurance requirements (SARs) that defines a point on the CC assurance scale, from EAL1 (functionally tested) through EAL7 (formally verified design and tested). Each level builds on the previous one with increased rigor.
**Source:** Common Criteria CC:2022, Part 1 (ISO/IEC 15408-1)
**Normative:** yes (ISO/IEC 15408)
**Synonyms / variants:** EAL1–EAL7
**Deprecated forms:** none
**Usage notes:** When documenting evaluation targets for UMRS, specify the EAL level. Do not use EAL as a quality claim without specifying the scope of evaluation.

---

## Protection Profile (PP)

**Definition:** An implementation-independent set of security requirements for a category of products or systems (the TOE type) that meet specific consumer needs. A PP defines what security properties a product must have without specifying how.
**Source:** Common Criteria CC:2022, Part 1 (ISO/IEC 15408-1)
**Normative:** yes (ISO/IEC 15408)
**Synonyms / variants:** PP
**Deprecated forms:** none
**Usage notes:** Reference PPs by their formal identifier when citing evaluation requirements. UMRS may reference operating system PPs (e.g., OSPP) when documenting compliance posture.

---

## Security Functional Requirement (SFR)

**Definition:** A requirement for a security function of the TOE, expressed in a standardized language derived from CC Part 2 functional components. SFRs are the building blocks of security specifications in PPs and STs.
**Source:** Common Criteria CC:2022, Part 2 (ISO/IEC 15408-2)
**Normative:** yes (ISO/IEC 15408)
**Synonyms / variants:** SFR
**Deprecated forms:** none
**Usage notes:** SFR element text uses "shall" (not "must") per CC convention. The 11 SFR classes are: FAU (audit), FCS (crypto), FDP (data protection), FIA (identification/authentication), FMT (management), FPR (privacy), FPT (TSF protection), FRU (resource utilization), FTA (TOE access), FTP (trusted path), FCO (communication). UMRS maps most directly to FAU, FCS, FDP, FIA, and FPT.

---

## Security Target (ST)

**Definition:** A product-specific set of security requirements and specifications used as the basis for evaluation of an identified TOE. Unlike a PP (which is implementation-independent), an ST describes how a specific product meets its security objectives.
**Source:** Common Criteria CC:2022, Part 1 (ISO/IEC 15408-1)
**Normative:** yes (ISO/IEC 15408)
**Synonyms / variants:** ST
**Deprecated forms:** none
**Usage notes:** If UMRS ever undergoes CC evaluation, the ST is the primary document produced. STs must claim conformance to one or more PPs.

---

## Target of Evaluation (TOE)

**Definition:** The product or system (and its associated guidance documentation) that is the subject of a CC evaluation.
**Source:** Common Criteria CC:2022, Part 1 (ISO/IEC 15408-1)
**Normative:** yes (ISO/IEC 15408)
**Synonyms / variants:** TOE
**Deprecated forms:** none
**Usage notes:** When writing about what UMRS evaluates or protects, use precise language. The TOE is the specific product boundary, not the entire system.

---

## TSF (TOE Security Functionality)

**Definition:** The combined set of all hardware, software, and firmware of the TOE that must be relied upon for the correct enforcement of the security functional requirements (SFRs).
**Source:** Common Criteria CC:2022, Part 1 (ISO/IEC 15408-1)
**Normative:** yes (ISO/IEC 15408)
**Synonyms / variants:** TOE Security Functionality
**Deprecated forms:** none
**Usage notes:** In UMRS context, the TSF maps to the reference monitor concept — the SELinux kernel enforcement plus the UMRS library types that model and verify security labels.

---

## Caution (admonition)

**Definition (MIL-STD-38784B):** A notice in a procedure indicating conditions that may cause equipment damage or mission failure if the step is performed incorrectly. A Caution does not involve risk of injury or death (that is a Warning).
**Source:** MIL-STD-38784B, Rev B (2020)
**Normative:** normative for DoD TMs; guidance for UMRS AsciiDoc docs
**Synonyms / variants:** CAUTION (AsciiDoc admonition type)
**Deprecated forms:** none
**Usage notes:** In UMRS AsciiDoc documentation, `CAUTION` maps to: a step that is irreversible or security-critical but does not immediately destroy data or cause a breach. See SDR-004 for the full mapping.

---

## denylist

**Definition:** A list of entities (users, IP addresses, applications, etc.) that are explicitly prohibited from accessing a resource or performing an action.
**Source:** NIST Author Instructions (inclusive terminology table, Feb 2025)
**Normative:** yes
**Synonyms / variants:** deny list (both acceptable; prefer closed form "denylist")
**Deprecated forms:** blacklist, black list
**Usage notes:** Same acknowledgment pattern as allowlist: reference the deprecated term once in parentheses if it appears in existing code or standards, then use "denylist" throughout.

---

## descriptive link text

**Definition:** Link text that conveys the destination or purpose of the link to a reader who cannot see the surrounding context. Effective for both visual readers and screen reader users.
**Source:** Google Developer Style Guide (link-text page); NIST Author Instructions (Section 508)
**Normative:** yes (Section 508 requires meaningful hyperlink text)
**Synonyms / variants:** informative link text
**Deprecated forms:** "click here," "this document," "this link," "here"
**Usage notes:** For UMRS Antora xrefs, use either the page title or a descriptive phrase. Never use a bare URL as link text in prose.

---

## dirty dozen

**Definition:** The twelve words identified by the Federal Plain Language Guidelines as the bureaucratic terms most likely to weaken writing: addressees, assist/assistance, commence, implement, in accordance with, in order that, in the amount of, in the event of, it is, promulgate, this activity/command, utilize/utilization.
**Source:** Federal Plain Language Guidelines, simple-words guidance (plain-language-simple-words.md)
**Normative:** informative
**Synonyms / variants:** none
**Deprecated forms:** (the terms themselves are the deprecated forms — see Usage notes)
**Usage notes:** When drafting or editing UMRS docs, scan for any of these twelve terms. Replace with the plain alternative from the simple-words substitution table. Most common violations in technical writing: "utilize" → "use"; "implement" → "carry out" or "start"; "in accordance with" → "by," "following," or "per."

---

## F-pattern reading

**Definition:** The observed tendency of readers to scan web content in an F-shaped pattern, reading fully across the top, then partway across subsequent lines, and then scanning vertically down the left side.
**Source:** Microsoft Writing Style Guide (scannable-content.md)
**Normative:** informative
**Synonyms / variants:** F-shaped reading pattern
**Deprecated forms:** none
**Usage notes:** Informs where to place the most critical information in UMRS documentation: first sentence of a paragraph, beginning of a heading, and left side of tables. Put key identifiers at the start of table rows and list items.

---

## IMPORTANT (admonition)

**Definition (UMRS):** An AsciiDoc admonition type used when missing information would cause procedure failure, but where no physical harm, data loss, or security breach risk exists. One step above NOTE in urgency; one step below CAUTION.
**Source:** AsciiDoc documentation; UMRS STE Mode rules; SDR-004
**Normative:** guidance (UMRS project decision)
**Synonyms / variants:** none (not present in MIL-STD-38784B; an AsciiDoc-specific type)
**Deprecated forms:** none
**Usage notes:** Use IMPORTANT for prerequisite checks, dependency requirements, or configuration facts that will cause silent failure if overlooked.

---

## localization

**Definition:** The process of adapting a product or content — including text and other elements — to meet the language, cultural, and political expectations of a specific local market. Distinct from translation (changing the language only).
**Source:** Microsoft Writing Style Guide (global-communications.md)
**Normative:** informative
**Synonyms / variants:** l10n (abbreviation)
**Deprecated forms:** none
**Usage notes:** UMRS currently has i18n architecture for tool binaries. Localization guidance from this corpus (avoid idioms, culture-specific references, fragmented list items) applies when writing any user-visible strings.

---

## Note (admonition)

**Definition (MIL-STD-38784B):** A notice that provides supplementary information that is not a warning or caution — information the reader should know but that does not involve risk of injury, death, or equipment damage.
**Source:** MIL-STD-38784B, Rev B (2020)
**Normative:** normative for DoD TMs; guidance for UMRS AsciiDoc docs
**Synonyms / variants:** NOTE (AsciiDoc admonition type)
**Deprecated forms:** none
**Usage notes:** In UMRS, use NOTE for supplementary facts, cross-references, and "be aware that" statements that do not rise to the level of IMPORTANT, CAUTION, or WARNING.

---

## plain language

**Definition:** Grammatically correct and universally understood language that includes complete sentence structure and accurate word usage. Writing that is clear and to the point; not unprofessional writing or simplification.
**Source:** Federal Plain Language Guidelines, Introduction (statutory definition per Plain Writing Act of 2010, P.L. 111-274)
**Normative:** yes (statutory for federal agencies)
**Synonyms / variants:** clear writing, plain writing
**Deprecated forms:** none
**Usage notes:** Plain language is not dumbing down — it applies the same technical precision with simpler structure and vocabulary. All UMRS documentation must comply with plain language principles as a baseline.

---

## primary / secondary (in place of master / slave)

**Definition:** "Primary" refers to the authoritative or leading node, process, or resource. "Secondary" refers to the subordinate, replica, or standby counterpart.
**Source:** NIST Author Instructions (inclusive terminology table), Microsoft Writing Style Guide (bias-free-communication.md)
**Normative:** yes
**Synonyms / variants:** primary/replica; primary/subordinate (Microsoft); leader/follower
**Deprecated forms:** master/slave
**Usage notes:** Use "primary/secondary" in UMRS documentation. When referring to code, configuration, or external standards that use "master/slave," acknowledge the deprecated term in parentheses on first use.

---

## procedure

**Definition:** A sequence of numbered steps for accomplishing a task. Each step contains exactly one action and begins with an imperative verb.
**Source:** Google Developer Style Guide (procedures page); MIL-STD-38784B; UMRS STE Mode rules
**Normative:** yes (all three sources are normative for different aspects)
**Synonyms / variants:** step-by-step instructions, numbered steps
**Deprecated forms:** none
**Usage notes:** Introduce every procedure with a complete sentence (ending in colon or period). Use numbered lists. Do not use the heading as the introduction. Optional steps begin with "Optional:". Single-step procedures format as bulleted lists, not numbered.

---

## scan-first design

**Definition:** A document design philosophy that prioritizes enabling readers to quickly find and understand key information by scanning, before committing to full reading. Implemented through headings, short paragraphs, bullets, bold keywords, and front-loaded sentences.
**Source:** Microsoft Writing Style Guide (scannable-content.md)
**Normative:** guidance
**Synonyms / variants:** scannable content
**Deprecated forms:** none
**Usage notes:** Apply scan-first design in all UMRS documentation. Front-load the key term or action in every heading, list item, and first sentence.

---

## Section 508

**Definition:** Section 508 of the Rehabilitation Act of 1973 (as amended), which requires federal agencies to ensure that all electronic and information technology is accessible to people with disabilities.
**Source:** NIST Author Instructions; Google Developer Style Guide (accessibility page)
**Normative:** yes (statutory for federal agencies)
**Synonyms / variants:** 508 compliance
**Deprecated forms:** none
**Usage notes:** UMRS documentation must comply with Section 508. Key requirements: alt text for images, meaningful hyperlink text, simple table structures, semantic heading hierarchy (no skipped levels), sufficient color contrast, keyboard navigability.

---

## sentence case

**Definition:** Capitalization style in which only the first word of a phrase and proper nouns are capitalized; all other words are lowercase.
**Source:** Google Developer Style Guide (highlights); Microsoft Writing Style Guide (capitalization.md)
**Normative:** yes (both commercial guides agree)
**Synonyms / variants:** lowercase style, down style
**Deprecated forms:** title case (for headings); ALL CAPS (for emphasis)
**Usage notes:** All UMRS Antora headings use sentence case. This includes level-1 through level-4 headings. Product names and proper nouns that are always capitalized remain so.

---

## serial comma

**Definition:** A comma placed before the coordinating conjunction (and, or) in a list of three or more items. Also called the Oxford comma.
**Source:** Google Developer Style Guide (highlights); Microsoft Writing Style Guide (top-10-tips, item 8)
**Normative:** yes (both commercial guides require it)
**Synonyms / variants:** Oxford comma
**Deprecated forms:** omitting the comma before the final item in a list
**Usage notes:** Required in all UMRS documentation. Example: "labels, roles, and types" not "labels, roles and types."

---

## Technical Manual (TM)

**Definition (DoD):** A Department of Defense publication assigned a TM identification number and controlled by a TM management information system, or subject to requisition from an inventory control point. Governed by MIL-STD-38784B.
**Source:** MIL-STD-38784B, Rev B (2020)
**Normative:** yes (for DoD TMs)
**Synonyms / variants:** TM, technical publication
**Deprecated forms:** none
**Usage notes:** UMRS Antora documentation is not a formal TM in the MIL-STD sense. If UMRS ever produces a TM with a TM number for DoD delivery, full MIL-STD-38784B compliance becomes normative.

---

## Warning (admonition)

**Definition (MIL-STD-38784B):** A notice in a procedure indicating conditions that may cause death or serious injury if the step is performed incorrectly or omitted.
**Source:** MIL-STD-38784B, Rev B (2020)
**Normative:** normative for DoD TMs; guidance for UMRS AsciiDoc docs
**Synonyms / variants:** WARNING (AsciiDoc admonition type)
**Deprecated forms:** none
**Usage notes:** In UMRS AsciiDoc documentation, `WARNING` maps to: a step that could cause data loss, a security breach, or serious system damage if performed incorrectly. This is the highest-severity admonition. See SDR-004.

---

## Active voice

**Definition:** In an active sentence, the person or agency that is acting is the subject. The actor is named. "The company polluted the lake" (active) vs. "The lake was polluted by the company" (passive).
**Source:** Federal Plain Language Guidelines, Active Voice section
**Normative:** yes (required by Plain Writing Act)
**Synonyms / variants:** none
**Deprecated forms:** passive voice constructions — avoid in procedure steps and governance statements
**Usage notes:** All four sources agree: use active voice. Passive voice is permitted only when the actor is the law itself or when it genuinely does not matter who performs the action.

---

## Allowlist / denylist

**Definition:** Allowlist — a list of items that are explicitly permitted. Denylist — a list of items that are explicitly prohibited.
**Source:** Google Developer Style Guide (word list); NIST Author Instructions (inclusive terminology table)
**Normative:** yes (NIST canonical; Google required)
**Synonyms / variants:** allowlist, denylist
**Deprecated forms:** whitelist, blacklist — do not use
**Usage notes:** NIST is the highest-priority source for this decision. Use allowlist/denylist in all UMRS documentation. This applies in both narrative text and code documentation.
**NIST control reference:** none directly; inclusive terminology policy

---

## Caution (MIL-STD)

**Definition:** Highlights an essential operating or maintenance procedure, practice, condition, statement, etc., which, if not strictly observed, could result in damage to, or destruction of, equipment or loss of mission effectiveness.
**Source:** MIL-STD-38784B, §3.2.4
**Normative:** yes (within DoD TM context)
**Synonyms / variants:** CAUTION (in Antora/AsciiDoc usage)
**Deprecated forms:** none
**Usage notes:** In UMRS AsciiDoc, the `CAUTION` admonition maps to this MIL-STD definition. Use CAUTION when the action could degrade system integrity or cause recoverable damage. See SDR-007.

---

## Controlled Unclassified Information (CUI)

**Definition:** Information that the Government creates or possesses, or that an entity creates or possesses for or on behalf of the Government, that a law, regulation, or Government-wide policy requires or permits an agency to handle using safeguarding or dissemination controls.
**Source:** MIL-STD-38784B, §3.1 (acronym definition); DoDI 5200.48 (authoritative definition)
**Normative:** yes
**Synonyms / variants:** CUI
**Deprecated forms:** For-Official-Use-Only (FOUO) was the older term — superseded by CUI
**Usage notes:** CUI markings must appear in running heads/feet of any document containing CUI per MIL-STD-38784B §4.7.2.1.1 and §4.7.2.2.4. CUI category abbreviations are in `reference/pages/cui/`.

---

## Dirty dozen (plain language)

**Definition:** The twelve bureaucratic words most likely to weaken government writing, per the Federal Plain Language Simple Words list. They are: addressees, assist/assistance, commence, implement, in accordance with, in order that, in the amount of, in the event of, it is, promulgate, this activity/command, utilize/utilization.
**Source:** Federal Plain Language Guidelines, plain-language-simple-words.md
**Normative:** informative (guidance for better writing)
**Synonyms / variants:** none
**Deprecated forms:** all twelve — replace with plain alternatives (see simple-words table)
**Usage notes:** Agent should flag any of the dirty dozen found in prose during review. The preferred substitution for each is in `plain-language-simple-words.md`.

---

## Note (MIL-STD)

**Definition:** Highlights an essential operating or maintenance procedure, condition, or statement. Do not use notes in place of procedural steps.
**Source:** MIL-STD-38784B, §3.2.19
**Normative:** yes (within DoD TM context)
**Synonyms / variants:** NOTE (AsciiDoc/Antora)
**Deprecated forms:** none
**Usage notes:** Notes may precede or follow applicable text (unlike warnings and cautions which precede). Notes shall not contain procedural steps. In UMRS, `NOTE:` in AsciiDoc maps to this definition.

---

## Oxford comma

**Definition:** A comma placed before the conjunction (and, or) in a list of three or more items. "Android, iOS, and Windows" (with Oxford comma) vs. "Android, iOS and Windows" (without).
**Source:** Google Developer Style Guide (serial commas); Microsoft Writing Style Guide, Top 10 Tip 8
**Normative:** yes (both sources require it)
**Synonyms / variants:** serial comma
**Deprecated forms:** omitting the final comma — do not omit
**Usage notes:** Required in all UMRS documentation. Both primary style authorities agree.

---

## Plain language

**Definition:** Grammatically correct and universally understood language that includes complete sentence structure and accurate word usage. Plain language is not unprofessional writing or a method of "dumbing down" the reader.
**Source:** Federal Plain Language Guidelines, What Is Plain Language section
**Normative:** yes (Plain Writing Act of 2010)
**Synonyms / variants:** plain writing, clear writing
**Deprecated forms:** "simplified English" — avoid this term; it conflates plain language with controlled vocabulary systems like STE
**Usage notes:** Plain language is a statutory requirement for federal communications. UMRS documentation targets federal audiences and must comply.

---

## Primary / subordinate

**Definition:** Primary — the controlling or originating node, process, or system. Subordinate — the dependent or replica node, process, or system.
**Source:** Microsoft Writing Style Guide (bias-free-communication.md); NIST Author Instructions (inclusive terminology)
**Normative:** yes (NIST canonical; Microsoft required)
**Synonyms / variants:** primary/secondary, leader/follower, primary/replica
**Deprecated forms:** master/slave — do not use in UMRS documentation
**Usage notes:** Use primary/subordinate or primary/replica depending on the technical context. See SDR-010 for the exception when quoting verbatim technical specifications. NIST is the higher-priority source.

---

## Reading Grade Level (RGL)

**Definition:** A measure of the educational level required to read and understand a piece of writing. Calculated from sentence length and syllable count (Flesch-Kincaid and similar formulas). MIL-STD-38784B requires RGL 9 for technical manuals.
**Source:** MIL-STD-38784B, §4.8.3
**Normative:** yes (within DoD TM context)
**Synonyms / variants:** RGL (MIL-STD abbreviation); readability level
**Deprecated forms:** none
**Usage notes:** RGL 9 corresponds approximately to ninth-grade reading level. This is achieved through short sentences, common words, and active voice — the same principles reinforced by Plain Language guidelines.

---

## Sentence-style capitalization

**Definition:** Capitalization in which only the first word of a heading, title, or phrase and any proper nouns are capitalized. All other words are lowercase.
**Source:** Google Developer Style Guide (highlights); Microsoft Writing Style Guide (capitalization.md, top-10-tips tip 6)
**Normative:** yes (both primary style authorities agree)
**Synonyms / variants:** sentence case
**Deprecated forms:** title case in headings — do not use for Antora page headings or section headings
**Usage notes:** Apply sentence-style capitalization to all Antora AsciiDoc headings (= levels 1–5). Proper nouns and product names remain capitalized. Exception: MIL-STD formal TM submissions use all-caps chapter headings — this exception applies only to formal government deliverables.

---

## Serial comma

**Definition:** See Oxford comma.

---

## Shall / should / may / will (modal vocabulary)

**Definition:** In normative technical writing: "shall" = mandatory requirement; "should" = recommended but not required; "may" = permitted; "will" = declaration of purpose or simple futurity.
**Source:** MIL-STD-38784B, §4.8.8
**Normative:** yes (within MIL-STD scope)
**Synonyms / variants:** "must" is used instead of "shall" in plain language writing
**Deprecated forms:** none formally; but "shall" in descriptive prose is discouraged by PLAIN guidelines
**Usage notes:** See SDR-005. Use "shall/should/may" only in normative specification documents. Use present tense and "must" in descriptive and procedural documentation.

---

## Warning (MIL-STD)

**Definition:** Highlights an essential operating or maintenance procedure, practice, condition, statement, etc., which, if not strictly observed, could result in injury to, or death of, personnel or long-term health hazards.
**Source:** MIL-STD-38784B, §3.2.31
**Normative:** yes (within DoD TM context)
**Synonyms / variants:** WARNING (AsciiDoc/Antora)
**Deprecated forms:** none
**Usage notes:** In UMRS AsciiDoc, `WARNING` maps to this definition — adapted for security context: use when a reader's action could cause security policy violation, data loss, system compromise, or irreversible damage. Warnings precede cautions, which precede notes. See SDR-007.
