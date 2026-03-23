# Henri Corpus Acquisition Summary

**Session:** 2026-03-23
**Agent:** The Librarian (T. Librarian)
**Mission:** Acquire Canadian federal policy documents for Henri's regulatory corpus.

---

## What Was Acquired

### Priority 1 — TBS Policy (COMPLETE)

All four TBS policy documents acquired in both official languages:

| Document | EN | FR | Notes |
|---|---|---|---|
| Directive on Security Management (2019) | 77K | 91K | pandoc-rendered; full text present |
| Standard on Security Categorization (2019) | 6.3K | 7.8K | Clean extraction via Python parser |
| Policy on Government Security (2019, amended 2025) | 36K | 45K | Includes 2025-01-06 amendments |
| Directive on Privacy Practices (2024) | 59K | 74K | Replaces 2022 version + 2010 PIA directive |

### Priority 2 — CCCS Guidance (BLOCKED)

cyber.gc.ca is unreachable due to FIPS TLS EMS incompatibility on RHEL10.
All CCCS documents require manual download. See corpus-index.md for direct URLs.

Key finding: ITSP.10.222 does not exist in the CCCS catalog. The correct PBMM reference is ITSG-33 Annex 4A Profile 1.

### Priority 3 — Legislation (COMPLETE)

All three acts acquired in both official languages as full consolidated text:

| Act | EN | FR | Current to |
|---|---|---|---|
| Access to Information Act (A-1) | 210K | 217K | 2026-03-02 (amended 2025-06-02) |
| Privacy Act (P-21) | 131K | 137K | 2026-03-02 (amended 2025-06-02) |
| Official Languages Act (O-3.01) | 140K | 136K | 2026-03-02 (amended 2025-06-20) |

### Terminology TSV

65 bilingual term pairs covering:
- Security classification levels (Top Secret / Très secret through Protected A / Protégé A)
- Privacy vocabulary (PIB/FRP, PIA/EFVP, personal information/renseignements personnels)
- Access rights terminology (right of access/droit d'accès, exemption/exception, etc.)
- Official languages terms (language of work/langue de travail, significant demand/demande importante)
- Governance vocabulary (deputy head/administrateur général, CSO/agent de sécurité du ministère)
- Key institutional names (Treasury Board/Conseil du Trésor, etc.)

---

## Technical Discoveries

### TBS WAF Behavior
- Some TBS document IDs are blocked by the WAF without the `Accept-Language` header.
- The `&section=html` parameter (print-friendly view) reduces JavaScript dependency and improves extraction.
- A Python-based HTML extractor was written and saved at `.claude/agent-memory/henri/extract_pol.py` — reuse this for future TBS document acquisition.

### cyber.gc.ca TLS Block
- Confirmed: cyber.gc.ca requires EMS (Extended Master Secret) in TLS handshake.
- RHEL10 FIPS mode does not support EMS — this is a policy conflict, not a configuration error.
- Resolution: Use a non-FIPS system or a browser proxy for CCCS document retrieval.
- Alternative: Open Government Portal (open.canada.ca) may host mirrored PDFs without this restriction.

### laws-lois.justice.gc.ca
- Works without special headers.
- Use `FullText.html` (EN) / `TexteComplet.html` (FR) for complete consolidated act text.
- Acts are large (250–430KB HTML) but clean — full statutory text with all amendments.

---

## What Henri Can Do With This Corpus

1. **Security classification analysis:** Full EN/FR vocabulary for Protégé A/B/C and classifié categories; impact level definitions; aggregation considerations.

2. **Privacy compliance checks:** Complete Privacy Act text + TBS Directive on Privacy Practices; PIB requirements; PIA triggers; breach notification requirements.

3. **Access to information analysis:** ATIA exemptions (ss.13-26); proactive publication requirements (Part 2); complaint/investigation process.

4. **Official languages obligations:** Bilingual service delivery triggers; language of work rights; Treasury Board responsibilities; Commissioner oversight.

5. **Security governance mapping:** Directive on Security Management appendices (IT, physical, BCM, IM, contracts, event management, awareness).

6. **Bilingual drafting support:** 65-entry terminology TSV with source citations for accurate EN/FR terminology in policy documents.

---

## Pending Actions

1. **MANUAL DOWNLOAD required** for CCCS ITSG-33 documents — see corpus-index.md for URLs. Henri should flag when a browser session is available.

2. **Cleanup** of `tmp-*.html` files (2MB) after corpus ingestion confirmation.

3. **GC Security Control Profile for Cloud** (canada.ca) — available but URL too long for current bash sandbox. Fetch separately: https://www.canada.ca/en/government/system/digital-government/digital-government-innovations/cloud-services/government-canada-security-control-profile-cloud-based-it-services.html

4. **Re-process directive files** — the two 2019 Directive on Security Management files have pandoc div artifacts from the print-friendly fetch path. The policy text is fully present but future re-extraction with the Python parser would produce cleaner markdown.

---

## File Inventory

```
.claude/agent-memory/henri/corpus/
├── tbs-directive-security-mgmt-en.md       (77K)
├── tbs-directive-security-mgmt-fr.md       (91K)
├── tbs-standard-security-categorization-en.md  (6.3K)
├── tbs-standard-security-categorization-fr.md  (7.8K)
├── tbs-policy-gov-security-en.md           (36K)
├── tbs-policy-gov-security-fr.md           (45K)
├── tbs-directive-privacy-en.md             (59K)
├── tbs-directive-privacy-fr.md             (74K)
├── atia-en.md                              (210K)
├── atia-fr.md                              (217K)
├── privacy-act-en.md                       (131K)
├── privacy-act-fr.md                       (137K)
├── ola-en.md                               (140K)
├── ola-fr.md                               (136K)
├── henri-tbs-terms-fr_CA.tsv               (65 term pairs)
├── extract_pol.py                          (reusable HTML extractor)
└── tmp-*.html                              (temp files — pending cleanup)

.claude/agent-memory/henri/
├── corpus-index.md                         (full document index)
└── corpus-acquisition-summary.md           (this file)
```

Total substantive content: ~1.3MB across 14 policy/legislation markdown files.
