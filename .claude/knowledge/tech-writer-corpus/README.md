# tech-writer-corpus — Knowledge Collection Summary

**Familiarization date:** 2026-03-16 (updated pass — all Google and Microsoft files fully processed)
**Documents processed:** 36 substantive files across four sub-collections
**Total coverage:** Government writing standards, federal plain language requirements, two major commercial developer documentation style guides, and Common Criteria (CC:2022) evaluation framework vocabulary

---

## What this collection covers

The tech-writer-corpus establishes the normative and guidance-level writing standards that govern all UMRS documentation. It spans three tiers: federal statutory requirements (Plain Writing Act, encoded in the Federal Plain Language Guidelines), DoD technical manual standards (MIL-STD-38784B), NIST publication requirements, and two commercial style guides (Google Developer Documentation Style Guide, Microsoft Writing Style Guide) that govern developer-facing prose. Together these sources resolve questions about procedure format, heading style, admonition hierarchy, inclusive terminology, voice and tone, sentence length, and word choice across the full range of UMRS documentation types — from formal deliverables targeting DoD review to developer guides targeting Rust engineers.

---

## Sub-collections

### gov-standards (16 files)

| File | Status | Normative weight |
|---|---|---|
| `MIL-STD-38784B.pdf` | Processed (image pages 1–40) | Normative (DoD) |
| `nist-author-instructions.md` | Processed (preview + SOURCE entry) | Normative (NIST) |
| `federal-plain-language-guidelines.md` | Processed | Normative (statutory) |
| `plain-language-active-voice.md` | Processed | Normative |
| `plain-language-simple-words.md` | Processed | Normative |
| `plain-language-short-sections.md` | Processed | Normative |
| `plain-language-organize.md` | Processed (stub) | Normative |
| `plain-language-conversational.md` | Processed (stub) | Normative |
| `plain-language-words-index.md` | Processed (index only) | Normative |
| `plain-language-words.md` | SPA fetch failure — no content | — |
| `plain-language-sentences.md` | SPA fetch failure — no content | — |
| `MIL-STD-38784B-MANUAL-DOWNLOAD.md` | Superseded placeholder | — |
| `everyspec-38784b-page.md` | HTML metadata only | — |
| `SOURCE.md` | Source tracking metadata | — |
| `FederalPLGuidelines.pdf` | Invalid — HTML redirect shell | — |

### style-guides/google (12 files)

| File | Status |
|---|---|
| `highlights-main.md` | Processed |
| `highlights-detail.md` | Processed |
| `tone.md` | Processed |
| `procedures.md` | Processed |
| `text-formatting.md` | Processed |
| `code-in-text.md` | Processed |
| `link-text.md` | Processed |
| `about-and-highlights.md` | Processed (superseded summary file) |
| `word-list.md` | Not processed — 2766 lines; deferred |
| `accessibility.md` | Not processed — deferred |
| `inclusive-documentation.md` | Not processed — deferred |
| `procedures-formatting-accessibility.md` | Superseded |
| `SOURCE.md` | Source tracking metadata |

### style-guides/microsoft (13 files)

| File | Status |
|---|---|
| `welcome.md` | Processed |
| `top-10-tips-style-voice.md` | Processed |
| `brand-voice.md` | Processed |
| `procedures-instructions.md` | Processed |
| `word-choice.md` | Processed |
| `scannable-content.md` | Processed |
| `headings.md` | Processed |
| `lists.md` | Processed |
| `capitalization.md` | Processed |
| `bias-free-communication.md` | Processed |
| `global-communications.md` | Processed |
| `SOURCE.md` | Source tracking metadata |

---

## Artifact files

| Artifact | Path | Description |
|---|---|---|
| Concept index | `concept-index.md` | What each document covers, key concepts, governed writing tasks, cross-references |
| Cross-reference map | `cross-reference-map.md` | 13 agreements, 6 tensions, 4 chains, 5 gaps across all sources |
| Style decision record | `style-decision-record.md` | 9 resolved decisions (SDR-001 through SDR-009) |
| Term glossary | `term-glossary.md` | 19 canonical terms with priority-source attribution and usage notes |

### domain-refs (2 files — Common Criteria)

| File | Status | Normative weight |
|---|---|---|
| `CC2022PART1R1.pdf` | Processed (162 RAG chunks) | Normative (ISO/IEC 15408) |
| `CC2022PART2R1.pdf` | Processed (304 RAG chunks) | Normative (ISO/IEC 15408) |

CC:2022 Revision 1. Part 1 provides 94 formal term definitions and the evaluation logic chain (assets → threats → objectives → SFRs → SARs). Part 2 provides 11 SFR classes with families and components. Key UMRS mappings: FAU (audit), FCS (crypto/FIPS), FDP (MLS/Bell-LaPadula), FIA (security context binding), FPT (fail-closed, self-test), FTP (trusted path).

---

## Gaps and open questions

**Unprocessed files:**
- `word-list.md` (Google, 2766 lines) — contains approved/deprecated term pairs; deferred due to size. Run an incremental familiarization pass when word-list coverage is needed.
- `accessibility.md` and `inclusive-documentation.md` (Google) — covered partially by the highlights files. Process incrementally when accessibility or inclusive language questions arise.

**Missing from corpus:**
- **GPO Style Manual** — MIL-STD-38784B cites GPO as the authority on capitalization and punctuation for formal TM submissions. Not in corpus.
- **DoD Dictionary of Military and Associated Terms (JP 1-02)** — MIL-STD-38784B cites this for authoritative term definitions. Not in corpus.
- **AsciiDoc / Antora documentation** — no Antora-specific style docs in corpus. Antora rules are carried in agent memory and CLAUDE.md.

**Resolved style decisions (2026-03-16):**
- **SDR-009**: Third person for architecture/security-concepts, second person for devel/deployment/operations. Resolved.
- **SDR-010**: Inclusive terms in narrative, standard terms in specs, verbatim quotes get editorial note. Resolved.
