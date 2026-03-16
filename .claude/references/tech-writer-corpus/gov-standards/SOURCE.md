# Government & Defense Writing Standards — Source Tracking

**Collection:** tech-writer-corpus/gov-standards
**Type:** RAG augmentation material
**Retrieved:** 2026-03-16

## Files

### nist-author-instructions.md

| Field | Value |
|-------|-------|
| Full title | NIST Technical Series Publications Author Instructions |
| Issuing authority | NIST (National Institute of Standards and Technology) |
| Source URL | https://www.nist.gov/nist-research-library/nist-technical-series-publications-author-instructions |
| Last updated | February 19, 2025 |
| Retrieval method | curl + pandoc HTML-to-markdown (2026-03-16) |
| Lines | 740 |
| SHA-256 | eaac00d6fa832fe51d06a7d67d5245b3bef5dd3d978597462fb86ae060442de6 |
| Status | Downloaded — verbatim page content |

---

### MIL-STD-38784B.pdf

| Field | Value |
|-------|-------|
| Full title | DoD Standard Practice: General Style and Format Requirements for Technical Manuals |
| Document number | MIL-STD-38784B |
| Issuing authority | Department of Defense |
| Date | 16 November 2020 |
| Source URL | https://everyspec.com/MIL-STD/MIL-STD-10000-and-Up/download.php?spec=MIL-STD-38784B.057086.pdf |
| Source site | everyspec.com (Jamie-approved for unclassified DoD specs, 2026-03-16) |
| File size | 3.95 MB |
| SHA-256 | d7072a44cbeaabd3404b3a215fd26a6261558db61ed4b93a2004d8005e518e33 |
| Status | Downloaded |

**Note:** The placeholder file `MIL-STD-38784B-MANUAL-DOWNLOAD.md` is now superseded.
The official source is ASSIST (quicksearch.dla.mil, CAC required). everyspec.com was
explicitly approved by Jamie as a source for unclassified DoD specifications.

---

### federal-plain-language-guidelines.md

| Field | Value |
|-------|-------|
| Full title | Federal Plain Language Guidelines (Revision 1, May 2011) |
| Issuing authority | Plain Language Action and Information Network (PLAIN) |
| Statutory authority | Plain Writing Act of 2010 (P.L. 111-274) |
| Source URL | https://plainlanguage.gov/media/FederalPLGuidelines.pdf |
| Retrieval method | WebSearch summaries (2026-03-16 — original PDF URL now redirects to digital.gov) |
| Status | WebSearch-derived summary content; authoritative text in GitHub archive below |

**PDF availability:** The original PDF URL (plainlanguage.gov/media/FederalPLGuidelines.pdf)
permanently redirects to https://digital.gov/guides/plain-language. The PDF is no longer
served directly. Original content is fully preserved in the GSA GitHub archive.

---

### FederalPLGuidelines.pdf

**Status: INVALID — not a PDF.** This file contains the HTML redirect response from
plainlanguage.gov. The PDF is no longer available at that URL (301 to digital.gov).
Do not use this file. See the GitHub-sourced plain language files below.

---

### Plain Language Guidelines (GSA GitHub Archive)

Source repository: https://github.com/GSA/plainlanguage.gov
License: CC0 1.0 Universal (public domain dedication)
Retrieved: 2026-03-16 via curl from raw.githubusercontent.com

| File | Source path in repo | SHA-256 | Lines |
|------|--------------------|---------| ------|
| plain-language-active-voice.md | _pages/guidelines/conversational/use-active-voice.md | f1f45e977f2f1ada94c429344d966df622bfe39cc1b0e23a91c7384857ee2d7b | 78 |
| plain-language-simple-words.md | _pages/guidelines/words/use-simple-words-phrases.md | cede672a89387c9041ad755e8b1241ec769388103fe012a2d40abc1cd341c452 | 251 |
| plain-language-short-sections.md | _pages/guidelines/concise/write-short-sections.md | (not recorded) | 56 |
| plain-language-organize.md | _pages/guidelines/organize/index.md | (not recorded) | 10 |
| plain-language-conversational.md | _pages/guidelines/conversational/index.md | (not recorded) | 11 |
| plain-language-words-index.md | _pages/guidelines/words/index.md | (not recorded) | 39 |

**Note:** The plainlanguage.gov web pages are JavaScript-rendered SPAs — curl returns empty
shells. The GitHub archive is the authoritative source for machine-readable content.
Files plain-language-guidelines-index.md, plain-language-words.md, plain-language-sentences.md
are stub/redirect notes from failed SPA fetches.

---

### everyspec-38784b-page.md

| Field | Value |
|-------|-------|
| Content | everyspec.com page for MIL-STD-38784B (HTML-to-markdown) |
| Purpose | Reference page showing revision history and download links |
| Source URL | http://everyspec.com/MIL-STD/MIL-STD-10000-and-Up/MIL-STD-38784B_57086/ |
| Status | Supplementary — the actual PDF is in MIL-STD-38784B.pdf |

---

## Update Check

- NIST Author Instructions: Check https://www.nist.gov/nist-research-library/nist-technical-series-publications-author-instructions for updates.
- MIL-STD-38784B: Dated 16 Nov 2020 (Rev B). Check ASSIST for superseding documents.
- Federal Plain Language Guidelines: Stable since May 2011. Check https://github.com/GSA/plainlanguage.gov for updates.
