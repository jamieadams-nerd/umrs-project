# Tech-Writer Agent RAG Corpus Acquisition Procedures

**Document type:** Agent operating procedures  
**Applies to:** Researcher agent  
**Purpose:** Acquire style and domain reference materials for the tech-writer agent RAG corpus  
**Prerequisite:** Researcher agent has ingestion pipeline operational and knows what to do with downloaded content.

---

## Section 1 — Style Guide Acquisition

### 1.1 Google Developer Documentation Style Guide

**Source:** `https://developers.google.com/style`  
**Format:** Web (crawlable HTML)  
**Priority:** High

1. Crawl `https://developers.google.com/style` recursively, following all in-domain links under the `/style/` path prefix.
2. Capture all subpages including highlights, voice and tone, text-formatting summary, and word list sections.
3. Exclude pages under `/tech-writing/` (course material — separate corpus if desired).
4. Store as structured markdown per page, preserving heading hierarchy.

---

### 1.2 Microsoft Writing Style Guide

**Source:** `https://learn.microsoft.com/en-us/style-guide/`  
**Format:** Web (crawlable HTML)  
**Priority:** High

1. Crawl `https://learn.microsoft.com/en-us/style-guide/` recursively, following all links within the `/style-guide/` path prefix.
2. Prioritize sections on: word choice, grammar, procedures, bias-free communication, and the A–Z word list.
3. Store as structured markdown per page, preserving heading hierarchy and all example tables (Use this / Not this).

---

### 1.3 Apple Style Guide

**Source:** `https://support.apple.com/guide/applestyleguide/`  
**Format:** Downloadable PDF (243 pages) and web  
**Priority:** Medium

1. Navigate to `https://support.apple.com/guide/applestyleguide/`.
2. Download the PDF version.
3. Extract full text from the PDF, preserving section structure.
4. Pay particular attention to: international audience guidance, writing for translation, and the glossary sections.

---

### 1.4 DigitalOcean Writing Guidelines

**Source:** `https://www.digitalocean.com/community/tutorials/digitalocean-s-technical-writing-guidelines`  
**Format:** Web (single page)  
**Priority:** Medium

1. Fetch the page at the URL above.
2. Capture the full text including the tutorial structure framework and procedure-writing guidance.
3. Also fetch: `https://github.com/do-community/do-article-templates` — download the raw markdown templates from this repository.

---

### 1.5 Mailchimp Content Style Guide

**Source:** `https://styleguide.mailchimp.com/`  
**Format:** Web (crawlable HTML, openly licensed)  
**Priority:** Medium

1. Crawl `https://styleguide.mailchimp.com/` recursively, following all in-domain links.
2. Capture all sections. Prioritize: Writing Goals and Principles, Voice and Tone, Grammar and Mechanics, and Writing for Accessibility.
3. Note: Licensed for reuse with attribution — confirm attribution metadata is stored with the corpus entry.

---

## Section 2 — Government and Defense Standards Acquisition

### 2.1 MIL-STD-38784A — DoD Technical Manual Style and Format

**Source:** `https://everyspec.com/MIL-STD/MIL-STD-10000-and-Up/MIL-STD-38784A_36449/`  
**Format:** PDF download  
**Priority:** High — primary reference for procedures, warnings, and cautions

1. Navigate to the everyspec.com page above.
2. Download the MIL-STD-38784A PDF.
3. Extract full text, preserving all section numbers, table structures, and the example appendices.
4. This document governs the format and language requirements for warnings, cautions, and notes within procedures. The tech-writer agent must internalize these distinctions.

---

### 2.2 NIST Technical Series Publications Author Instructions

**Source:** `https://www.nist.gov/nist-research-library/nist-technical-series-publications-author-instructions`  
**Format:** Web (single page)  
**Priority:** High — governs how NIST-adjacent documentation is structured and written

1. Fetch the full page at the URL above.
2. Capture all guidance including: Plain Writing Act compliance, Section 508 accessibility requirements, equation and figure formatting, and deprecated terminology guidance.
3. This establishes the register and precision expected in documentation targeting the Five Eyes / federal security community.

---

### 2.3 Plain Language Guidelines (Federal)

**Source:** `https://www.plainlanguage.gov/guidelines/`  
**Format:** Web (crawlable HTML)  
**Priority:** High — mandated baseline for all federal-adjacent documentation

1. Crawl `https://www.plainlanguage.gov/guidelines/` recursively, following all links within the `/guidelines/` path prefix.
2. Capture all subsections including: words, sentences, organization, design, and testing guidance.
3. Also fetch the quick reference summary at `https://www.plainlanguage.gov/media/FederalPLGuidelines.pdf` and download the PDF.

---

## Section 3 — Domain Reference Acquisition

These are content-domain references, not style guides. They ensure the tech-writer agent uses correct terminology when writing about the security domain.

### 3.1 NIST SP 800-53 Rev 5 — Security and Privacy Controls

**Source:** `https://csrc.nist.gov/publications/detail/sp/800/53/rev-5/final`  
**Format:** PDF  
**Priority:** Critical — primary control vocabulary reference

1. Navigate to the CSRC page above.
2. Download the final PDF of SP 800-53 Rev 5.
3. Extract and index the control family summaries, control names, and control text. Control numbers (e.g., AC-1, AU-12) must be preserved exactly as printed.
4. Do not summarize control text — preserve verbatim for exact reference by the tech-writer agent.

---

### 3.2 NIST SP 800-171 Rev 3 — Protecting CUI

**Source:** `https://csrc.nist.gov/publications/detail/sp/800/171/rev-3/final`  
**Format:** PDF  
**Priority:** High — direct relevance to CUI documentation requirements

1. Navigate to the CSRC page above.
2. Download the final PDF.
3. Extract requirement identifiers and requirement text. Preserve numbering exactly.

---

### 3.3 CMMC Assessment Guide — Level 2

**Source:** `https://dodcio.defense.gov/CMMC/`  
**Format:** PDF (linked from the DoD CIO CMMC page)  
**Priority:** High — defines vocabulary used by the Five Eyes procurement community

1. Navigate to `https://dodcio.defense.gov/CMMC/`.
2. Locate and download the current Level 2 Assessment Guide PDF.
3. Extract practice identifiers, practice descriptions, and assessment objective language. Preserve all identifiers verbatim.

---

### 3.4 Red Hat Enterprise Linux Security Guide (RHEL 10)

**Source:** `https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/`  
**Format:** Web (crawlable HTML) and PDF  
**Priority:** High — establishes correct operator-facing terminology for SELinux, IMA, dm-crypt

1. Navigate to the RHEL 10 documentation index above.
2. Identify and crawl the Security Guide (or equivalent security-focused guide for RHEL 10).
3. Prioritize chapters covering: SELinux, MLS, IMA/IMA appraisal, LUKS/dm-crypt, and audit subsystem configuration.
4. Preserve all command syntax blocks and configuration file examples verbatim — these define the authoritative naming conventions the tech-writer agent must use.

---

### 3.5 SELinux Project Documentation

**Source:** `https://selinuxproject.org/page/Main_Page`  
**Format:** Web (crawlable HTML / wiki)  
**Priority:** Medium

1. Crawl the SELinux Project wiki, following links from the main page.
2. Prioritize: the Notebook (selinuxproject.org/page/SELinux_Notebook), MLS/MCS sections, and policy language reference.
3. The Notebook is the primary conceptual reference — ensure full capture of all chapters.

---

### 3.6 Common Criteria — CC Portal Writing Guidance

**Source:** `https://www.commoncriteriaportal.org/`  
**Format:** PDF downloads  
**Priority:** Medium — establishes the formal register for security target and protection profile writing

1. Navigate to `https://www.commoncriteriaportal.org/cc/`.
2. Download the current versions of:
   - Common Criteria for Information Technology Security Evaluation, Part 1 (Introduction and general model)
   - Common Criteria for Information Technology Security Evaluation, Part 2 (Security functional components)
3. Extract the SFR (Security Functional Requirement) naming conventions and the structured English used in functional component descriptions. This register is distinct from plain-language writing and the tech-writer agent must recognize when to apply it.

---

## Section 4 — Supplemental Reference

### 4.1 NASA Technical Report Writing Guidance

**Source:** `https://ntrs.nasa.gov/`  
**Format:** PDF (search required)  
**Priority:** Low-Medium — strong model for safety-critical procedure writing

1. Search `https://ntrs.nasa.gov/` for: `technical writing style guide`.
2. Identify and download NASA/TM or similar technical memoranda focused on writing standards for technical reports and procedures.
3. Prioritize documents covering: procedure clarity, safety-critical warning language, and documentation for operators in high-consequence environments.

---

## Acquisition Checklist

| Item | Section | Priority | Format | Status |
|------|---------|----------|--------|--------|
| Google Developer Style Guide | 1.1 | High | Web crawl | — |
| Microsoft Writing Style Guide | 1.2 | High | Web crawl | — |
| Apple Style Guide | 1.3 | Medium | PDF + Web | — |
| DigitalOcean Guidelines | 1.4 | Medium | Web + GitHub | — |
| Mailchimp Style Guide | 1.5 | Medium | Web crawl | — |
| MIL-STD-38784A | 2.1 | High | PDF | — |
| NIST Author Instructions | 2.2 | High | Web | — |
| Plain Language Guidelines | 2.3 | High | Web + PDF | — |
| NIST SP 800-53 Rev 5 | 3.1 | Critical | PDF | — |
| NIST SP 800-171 Rev 3 | 3.2 | High | PDF | — |
| CMMC Level 2 Assessment Guide | 3.3 | High | PDF | — |
| RHEL 10 Security Guide | 3.4 | High | Web/PDF | — |
| SELinux Project Notebook | 3.5 | Medium | Web crawl | — |
| Common Criteria Parts 1 & 2 | 3.6 | Medium | PDF | — |
| NASA Writing Guidance | 4.1 | Low-Med | PDF | — |

