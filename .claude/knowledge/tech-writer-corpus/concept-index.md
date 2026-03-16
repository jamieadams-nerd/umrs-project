# Concept Index — tech-writer-corpus
Generated: 2026-03-16

---

## MIL-STD-38784B

**Full title:** Department of Defense Standard Practice: General Style and Format Requirements for Technical Manuals
**Source:** `gov-standards/MIL-STD-38784B.pdf` + `gov-standards/MIL-STD-38784B-MANUAL-DOWNLOAD.md`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
MIL-STD-38784B governs DoD technical manual structure, format, and delivery for installation, operation, maintenance, and logistics support publications. Revision B (November 2020) added CUI marking requirements throughout. The standard defines the warning/caution/note hierarchy, procedure step format, document organization (TOC placement, lists of illustrations and tables, alphabetical index), and SGML/DTD requirements for electronic delivery.

### Key concepts introduced
- **Technical Manual (TM)** — a DoD publication assigned a TM identification number and controlled by a TM management information system
- **Warning** — a procedural notice about conditions that may cause death or serious injury
- **Caution** — a procedural notice about conditions that may cause equipment damage or mission failure
- **Note** — supplementary information that is neither a warning nor a caution
- **CUI marking** — controlled unclassified information designation requirements (added Rev B)
- **SGML/DTD** — markup requirements for electronic data delivery of TMs

### Governs these writing tasks
- Choosing between WARNING, CAUTION, and NOTE admonitions — hierarchy and scope
- Admonition placement (adjacent to the step they apply to, not at section end)
- Organizing technical documents (TOC start page, list-of-illustrations rules, index column headers)
- Marking any UMRS document that contains CUI

### Related documents in corpus
- `federal-plain-language-guidelines` — plain language requirements apply to TMs; MIL-STD governs structure
- `google/procedures` — supplementary procedure guidance for developer doc context
- `microsoft/procedures-instructions` — supplementary for UI-centric step format

---

## Federal Plain Language Guidelines

**Full title:** Federal Plain Language Guidelines, Revision 1
**Source:** `gov-standards/federal-plain-language-guidelines.md`
**Type:** regulatory standard
**Normative weight:** normative (statutory basis: Plain Writing Act of 2010, P.L. 111-274)

### Coverage
The Federal Plain Language Guidelines establish the statutory requirement for federal agencies to communicate in clear, simple, meaningful, jargon-free language. Coverage includes audience identification, document organization, active voice, sentence length, word choice, and design features (bullets, tables, headers, white space). The guidelines apply to documents explaining government benefits, services, or compliance requirements.

### Key concepts introduced
- **Plain language** — grammatically correct, universally understood language; not simplification
- **Active voice** — the person or agency acting is the subject; eliminates ambiguity about responsibility
- **Target sentence length** — average 15–20 words
- **Design features** — bullets, tables, headers, and white space as comprehension aids
- **Audience identification** — identify your reader first; address multiple audiences separately
- **Abbreviation policy** — minimize abbreviations; find simplified names instead

### Governs these writing tasks
- Prose style in all UMRS documentation
- Word choice (familiar, concrete, short words preferred)
- Sentence structure in procedural steps
- Using tables when information has a consistent structure across multiple items
- Abbreviation decisions (define on first use; prefer simplified names)

### Related documents in corpus
- `MIL-STD-38784B` — structure companion; Plain Language governs prose quality within that structure
- `google/tone` — commercial equivalent for developer docs
- `microsoft/top-10-tips-style-voice` — commercial distillation of the same plain language principles
- `nist-author-instructions` — references Plain Writing Act; extends requirements to NIST publications

---

## NIST Technical Series Publications Author Instructions

**Full title:** NIST Technical Series Publications Author Instructions
**Source:** `gov-standards/nist-author-instructions.md`
**Type:** regulatory standard
**Normative weight:** normative (for NIST-published documents)

### Coverage
This document governs preparation, organization, and submission of NIST technical publications. It covers document structure (required components ordered per ANSI/NISO Z39.18), heading levels and numbering, reference format, table and figure conventions, Section 508 (accessibility) compliance, plain language requirements, SI units, disclaimers, and copyright. Includes a table of canonical NIST inclusive terminology guidance with before/after examples.

### Key concepts introduced
- **PubID and DOI** — required identifiers for all NIST Technical Series publications
- **ANSI/NISO Z39.18-2005** — governs scientific and technical report organization; basis for NIST document structure
- **Section 508** — Rehabilitation Act requirement that federal agency electronic content be accessible to people with disabilities
- **Four-level heading hierarchy** — 1. / 1.1. / 1.1.1. / 1.1.1.1. — maximum in NIST documents
- **Alt text** — required for all images and visually dependent tables
- **Inclusive terminology (NIST canonical)** — allowlist/denylist (not whitelist/blacklist), primary/secondary (not master/slave), "older adult" (not "elderly")
- **Disclaimer requirements** — required for trade/product name mentions in NIST publications

### Governs these writing tasks
- Structuring long-form reference and architecture documents
- Heading numbering in formal reference documents
- Inclusive terminology — NIST is the highest-priority authority for these decisions in UMRS docs
- Accessibility compliance: alt text for images, simple table structure, meaningful link text
- Citation format for NIST publications referenced in Rust doc comments

### Related documents in corpus
- `federal-plain-language-guidelines` — referenced directly; statutory baseline
- `google/accessibility` — complementary web-context accessibility guidance
- `microsoft/bias-free-communication` — reinforcing inclusive terminology from commercial source

---

## Google Developer Documentation Style Guide

**Full title:** Google Developer Documentation Style Guide
**Source:** `style-guides/google/` (12 files)
**Type:** style guide
**Normative weight:** guidance (project-specific rules take precedence; Merriam-Webster and Chicago Manual are tertiary)

### Coverage
The primary commercial style authority for developer documentation. Covers voice and tone, grammar (second person, active voice, present tense, sentence structure), formatting (bold, italics, code font, sentence-case headings, serial comma), procedure structure, link text, code in text, accessibility, inclusive documentation, and a developer-specific word list. The guide is prescriptive but explicitly permits deviation when it serves readers better.

### Key concepts introduced
- **Conversational tone** — friendly and professional without being frivolous; knowledgeable-friend register
- **Second person** — "you" not "we" in procedures; direct address
- **Sentence case** — all headings and document titles
- **Serial comma** — required (Oxford comma)
- **Conditions before instructions** — place the "if" clause before the imperative
- **Procedure introductory sentence** — complete sentence (colon or period) before numbered steps; do not simply repeat the heading
- **Optional step format** — "Optional:" prefix with colon; not "(Optional)"
- **Sub-steps** — lowercase letters (a, b, c) for first level; Roman numerals (i, ii, iii) for second level
- **Code font scope** — filenames, paths, class names, method names, env variables, command output, language keywords, HTTP verbs, HTTP status codes
- **Descriptive link text** — never "click here" or "this document"; use page title or descriptive phrase
- **Ableist language** — avoid "crazy," "blind to," "cripple," "dummy," "sanity check"
- **allowlist / denylist** — preferred over whitelist / blacklist

### Governs these writing tasks
- Default style for all UMRS developer-facing documentation (devel/ module)
- Procedure step format: numbered, imperative verb, one action per step
- Code-in-text formatting decisions
- Link text in Antora xrefs
- Tone calibration for developer content
- Inclusive terminology

### Related documents in corpus
- `federal-plain-language-guidelines` — regulatory baseline; Google guide is the developer-doc refinement
- `microsoft/` — strong agreement on most principles; different emphasis on contractions and formality
- `nist-author-instructions` — inclusive terminology alignment; NIST is higher-priority authority on these

---

## Microsoft Writing Style Guide

**Full title:** Microsoft Writing Style Guide
**Source:** `style-guides/microsoft/` (13 files)
**Type:** style guide
**Normative weight:** guidance

### Coverage
Covers voice, tone, and formatting for technology writing. Signature principle: crisp minimalism (warm and relaxed, crisp and clear, ready to lend a hand). Covers the Top 10 style tips, brand voice principles, step-by-step procedure writing, heading structure and parallel form, list formatting (bulleted, numbered, term lists), scannable content design, sentence-style capitalization, word choice (simple words, one term per concept), contractions, bias-free communication, and global/localization guidance.

### Key concepts introduced
- **Brand voice** — warm and relaxed, crisp and clear, ready to lend a hand
- **Sentence-style capitalization** — first word and proper nouns only; explicit rule: never title-case headings
- **Contractions** — encouraged (it's, you'll, we're) for approachable tone
- **Procedure step limit** — seven steps maximum per procedure, preferably fewer
- **Right angle bracket notation** — `>` for sequential menu selections with spaces: **File > New > Document**
- **Term list format** — **Bold term**. Definition sentence.
- **Scan-first design** — write for scanning first, reading second; front-load keywords
- **F-pattern reading** — most attention goes to upper-left; put important content first
- **primary/subordinate** — preferred over master/slave
- **perimeter network** — preferred over DMZ in general writing

### Governs these writing tasks
- Heading structure: sentence case, parallel form, run-in headings, no back-to-back headings without text
- List format: when to use bulleted vs. numbered, term list syntax, punctuation rules
- Procedure format for operator-facing documentation (operations/ module)
- Scannable content design across all modules
- Capitalization audits
- Bias-free terminology validation

### Related documents in corpus
- `google/` — strong agreement on sentence case, active voice, serial comma; Microsoft encourages contractions; Google does not discourage them but does not emphasize them
- `federal-plain-language-guidelines` — same plain-language root; Microsoft is more conversational
- `nist-author-instructions` — NIST is more formal; Microsoft register is warmer

---

## Plain Language Individual Section Files (GSA GitHub Archive)

**Files:** `plain-language-active-voice.md`, `plain-language-simple-words.md`, `plain-language-short-sections.md`, `plain-language-organize.md`, `plain-language-conversational.md`, `plain-language-words-index.md`
**Source:** GSA GitHub archive (https://github.com/GSA/plainlanguage.gov), CC0 public domain
**Type:** regulatory standard (chapter-level extracts)
**Normative weight:** normative (same statutory basis as the full Federal Plain Language Guidelines)

### Coverage
Chapter-level extracts providing detailed guidance on: active voice identification and use, the canonical simple-words substitution table, short sections and visual chunking, information organization (chronological order for processes), conversational verbs as the fuel of writing, and the words index (which links to `plain-language-simple-words.md`).

### Key concepts introduced
- **Dirty dozen** — the twelve bureaucratic words most likely to weaken writing: "addressees," "assist/assistance," "commence," "implement," "in accordance with," "in order that," "in the amount of," "in the event of," "it is," "promulgate," "this activity/command," "utilize/utilization"
- **Simple words substitution table** — over 200 word pairs: preferred plain form vs. bureaucratic equivalent
- **Short sections** — a long section cannot be meaningfully summarized in a heading; break it
- **Passive voice markers** — "to be" form + past participle; identify by looking for these two features
- **Verbs as fuel** — strong, specific verbs drive clarity; ensure it is clear who does what

### Governs these writing tasks
- Word choice audits on any draft (check against the simple-words table)
- Identifying and correcting passive voice in procedure steps
- Chunking long introductions into headed sub-sections
- Ensuring every procedure step names an explicit actor

### Related documents in corpus
- `federal-plain-language-guidelines` — the summary; these files provide the chapter detail
- All style guides — all reinforce the same word-choice and sentence-structure principles

---

## Stub / Invalid / Metadata Files (non-substantive)

**Files:** `plain-language-sentences.md`, `plain-language-words.md`, `FederalPLGuidelines.pdf`, `everyspec-38784b-page.md`, `gov-standards/SOURCE.md`, `style-guides/google/SOURCE.md`, `style-guides/microsoft/SOURCE.md`

These files are SPA-fetch failures (empty shells), superseded placeholders, HTML index pages, or source-tracking metadata. They contain no guidance content. For plain language word and sentence guidance, use `plain-language-simple-words.md` and `plain-language-active-voice.md`.
