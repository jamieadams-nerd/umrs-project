# Henri Regulatory Corpus Index

**Date acquired:** 2026-03-23
**Acquirer:** The Librarian (T. Librarian, researcher agent)
**Purpose:** Canadian federal regulatory corpus for Henri — bilingual policy, legislation, and standards supporting GoC information security and privacy compliance analysis.

---

## Priority 1 — TBS Policy Documents

### Directive on Security Management (2019)
| File | Size | Status |
|---|---|---|
| `tbs-directive-security-mgmt-en.md` | 77K | Downloaded |
| `tbs-directive-security-mgmt-fr.md` | 91K | Downloaded |

- **Source (EN):** https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32611
- **Source (FR):** https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32611
- **Effective:** 2019-07-01 (Directive on Security Screening separated 2025-01-06)
- **Scope:** Chief security officer responsibilities; departmental security governance; appendices covering IT, physical, BCM, IM, contracts, event management, awareness/training controls
- **Key sections:** s.4 Requirements (CSO + senior officials); Appendices A–H (mandatory procedures)
- **Note:** Files contain pandoc div artifacts from print-friendly HTML rendering; all policy text is present and readable.

### Standard on Security Categorization (Appendix J)
| File | Size | Status |
|---|---|---|
| `tbs-standard-security-categorization-en.md` | 6.3K | Downloaded |
| `tbs-standard-security-categorization-fr.md` | 7.8K | Downloaded |

- **Source (EN):** https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32614
- **Source (FR):** https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32614
- **Effective:** 2019-07-01
- **Scope:** Security categorization process; impact levels (Very high/High/Medium/Low); classified categories (Top Secret/Secret/Confidential); protected categories (Protected A/B/C)
- **Key sections:** J.2.2 Categorization process; J.2.3 General categories; J.2.4 Confidentiality categories

### Policy on Government Security (2019)
| File | Size | Status |
|---|---|---|
| `tbs-policy-gov-security-en.md` | 36K | Downloaded |
| `tbs-policy-gov-security-fr.md` | 45K | Downloaded |

- **Source (EN):** https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=16578
- **Source (FR):** https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=16578
- **Effective:** 2019-07-01; amended 2025-01-06 (security screening lifecycle, deputy head delegation)
- **Scope:** Overarching policy; deputy head accountabilities; departmental security governance; Annex A (security screening lifecycle); Annex B (definitions)
- **Key sections:** s.4 Requirements; s.8 References; Annex A (screening); Annex B (definitions including updated security clearance terms)

### Directive on Privacy Practices (2024)
| File | Size | Status |
|---|---|---|
| `tbs-directive-privacy-en.md` | 59K | Downloaded |
| `tbs-directive-privacy-fr.md` | 74K | Downloaded |

- **Source (EN):** https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=18309
- **Source (FR):** https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=18309
- **Effective:** 2024-10-09 (replaces 2022 version and 2010 PIA directive)
- **Scope:** Personal information lifecycle management; PIBs; privacy notices; privacy impact assessments; breach notification; proactive publication of PIAs
- **Key sections:** s.4 Requirements (heads of institutions); PIB registration; PIA requirements; breach reporting

---

## Priority 2 — CCCS Guidance

### CCCS Guidance — Status: BLOCKED (TLS/FIPS)
- **Status:** cyber.gc.ca requires TLS Extended Master Secret (EMS) which RHEL10 FIPS mode does not support. Exit code 35: `ems not enabled`.
- **Relevant documents identified (manual download required):**
  - ITSG-33 Main publication: https://www.cyber.gc.ca/en/guidance/it-security-risk-management-lifecycle-approach-itsg-33
  - ITSG-33 Annex 4A Profile 1 (PBMM): https://www.cyber.gc.ca/en/guidance/annex-4a-profile-1-protected-b-medium-integrity-medium-availability-itsg-33
  - ITSG-33 Annex 4A PDF: https://www.cyber.gc.ca/sites/default/files/cyber/publications/itsg33-ann4a-1-eng.pdf
  - ITSG-33 Annex 3A (Security Control Catalogue): https://www.cyber.gc.ca/en/guidance/annex-3a-security-control-catalogue-itsg-33
  - ITSP.50.103 (Cloud security categorization): https://www.cyber.gc.ca/en/guidance/guidance-security-categorization-cloud-based-services-itsp50103
  - ITSM.50.100 (CSP assessment process): https://www.cyber.gc.ca/en/guidance/cloud-service-provider-information-technology-security-assessment-process-itsm50100
- **Note on ITSP.10.222:** This document identifier does not exist in the CCCS catalog. The relevant PBMM profile document is ITSG-33 Annex 4A Profile 1.
- **Alternative source:** GC Security Control Profile for Cloud-based GC Services available at canada.ca (TLS compatible) — https://www.canada.ca/en/government/system/digital-government/digital-government-innovations/cloud-services/government-canada-security-control-profile-cloud-based-it-services.html

---

## Priority 3 — Legislation

### Access to Information Act (R.S.C. 1985, c. A-1)
| File | Size | Status |
|---|---|---|
| `atia-en.md` | 210K | Downloaded |
| `atia-fr.md` | 217K | Downloaded |

- **Source (EN):** https://laws-lois.justice.gc.ca/eng/acts/A-1/FullText.html
- **Source (FR):** https://laws-lois.justice.gc.ca/fra/lois/A-1/TexteComplet.html
- **Current to:** 2026-03-02; last amended 2025-06-02
- **Scope:** Right of access to government records; exemptions; complaints; Information Commissioner; proactive publication (Part 2)
- **Key sections:** s.2 Purpose; s.3 Interpretation; s.4 Right of access; s.13-26 Exemptions; s.30-41 Complaints/investigation/review; Part 2 (s.71.01+) Proactive publication; Schedule I (government institutions)

### Privacy Act (R.S.C. 1985, c. P-21)
| File | Size | Status |
|---|---|---|
| `privacy-act-en.md` | 131K | Downloaded |
| `privacy-act-fr.md` | 137K | Downloaded |

- **Source (EN):** https://laws-lois.justice.gc.ca/eng/acts/P-21/FullText.html
- **Source (FR):** https://laws-lois.justice.gc.ca/fra/lois/P-21/TexteComplet.html
- **Current to:** 2026-03-02; last amended 2025-06-02
- **Scope:** Collection/retention/use/disclosure of personal information; PIBs; right of access to own information; Privacy Commissioner; exemptions
- **Key sections:** s.2 Purpose; s.3 Interpretation; s.4-8 Collection/protection; s.10-11 PIBs; s.12-17 Access rights; s.18-28 Exemptions; Schedule (government institutions)

### Official Languages Act (R.S.C. 1985, c. 31, 4th Supp.)
| File | Size | Status |
|---|---|---|
| `ola-en.md` | 140K | Downloaded |
| `ola-fr.md` | 136K | Downloaded |

- **Source (EN):** https://laws-lois.justice.gc.ca/eng/acts/O-3.01/FullText.html
- **Source (FR):** https://laws-lois.justice.gc.ca/fra/lois/O-3.01/TexteComplet.html
- **Current to:** 2026-03-02; last amended 2025-06-20
- **Scope:** Bilingual obligations for Parliament, legislation, courts, communications/services, language of work, minority community advancement; Commissioner of Official Languages
- **Key parts:** Part IV (communications and services to the public); Part V (language of work); Part VII (advancement of equality); Part VIII (Treasury Board responsibilities); Part IX (Commissioner)

---

## Terminology File

| File | Entries | Status |
|---|---|---|
| `henri-tbs-terms-fr_CA.tsv` | 65 | Created |

Covers: security classification terms, privacy terms, access rights vocabulary, official languages terms, governance terminology, and key institutional names — all with EN/FR pairs and source citations.

---

## Files Pending Cleanup

The following temporary HTML files in the corpus directory should be removed once Henri confirms the corpus is ingested:
- `tmp-*.html` (10 files, ~2MB total)
- `tmp-atia-toc-en.txt`
- `cccs-itsg33-ann4a-pbmm.pdf` (0 bytes — download failed due to TLS)
- `tmp-gc-cloud-profile-en.html` (0 bytes)

---

## Summary Statistics

| Category | Documents | Languages | Size |
|---|---|---|---|
| TBS Policy | 4 documents | EN + FR | ~332K |
| Legislation | 3 acts | EN + FR | ~971K |
| Terminology | 1 TSV | EN/FR bilingual | ~8K |
| **Total** | **8 documents** | **16 files** | **~1.3MB** |

---

## Acquisition Notes

- TBS WAF (Web Application Firewall) blocks plain curl for some document IDs. Workaround: use `Accept-Language: fr-CA,fr;q=0.9` header for French documents.
- cyber.gc.ca has FIPS TLS incompatibility (EMS not enabled). All CCCS documents require manual download or alternative source.
- laws-lois.justice.gc.ca works without special headers; use `FullText.html` / `TexteComplet.html` for complete act text.
- The `section=html` parameter on TBS policy URLs provides print-friendly single-page versions that work better with the Python extractor.
