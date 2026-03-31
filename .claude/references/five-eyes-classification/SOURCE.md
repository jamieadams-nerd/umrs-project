# Five Eyes Classification Collection — Source Record

## Collection

**Name:** Five Eyes Classification Policies
**Purpose:** Official security classification and categorization policies from Five Eyes partner
nations. Primary use: mapping Canadian Protected A/B/C tiers to US CUI equivalents, informing
`CANADIAN-PROTECTED.json` and `setrans.conf` label design, and providing Simone with authoritative
bilingual terminology for classification markings.
**Directory:** `.claude/references/five-eyes-classification/`
**Acquired:** 2026-03-24 (initial); 2026-03-31 (SOURCE.md and manifest entries added)
**Status:** Partial — TBS PGS (EN/FR) and UK GSCP downloaded; NZISM downloaded;
  Australia PSPF Policy 8 not yet acquired (backburner); TBS DOSM Appendix J EN/FR downloaded 2026-03-31;
  TBS DOSM (id=32611) not yet downloaded (lower priority — Appendix J has the Protected A/B/C content).
**RAG ingestion:** Not required for this collection — these are reference documents used
  directly by Henri (security policy) and Simone (French terminology). Familiarization-only.

---

## Files

### TBS Policy on Government Security — English

| Field | Value |
|---|---|
| File | `tbs-policy-gov-security-en.md` |
| Source URL | https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=16578 |
| Issuing authority | Treasury Board of Canada Secretariat (TBS) |
| Language | English |
| Policy ID | 16578 |
| Effective date | July 1, 2019 |
| Last amended | January 6, 2025 (security screening definitions added) |
| Acquisition method | curl with Firefox UA via `tbs-sct.canada.ca` (TLS WORKS — confirmed in MEMORY.md) |
| Acquisition date | 2026-03-24 |
| File size | 36,193 bytes |
| SHA-256 | ⚠ Not yet computed — run `sha256sum tbs-policy-gov-security-en.md` |

### TBS Policy on Government Security — French

| Field | Value |
|---|---|
| File | `tbs-policy-gov-security-fr.md` |
| Source URL | https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=16578 |
| Issuing authority | Secrétariat du Conseil du Trésor du Canada (SCT) |
| Language | French (fr-CA) |
| Policy ID | 16578 |
| Effective date | 1er juillet 2019 |
| Last amended | 6 janvier 2025 |
| Acquisition method | curl with Firefox UA + Accept-Language: fr-CA header |
| Acquisition date | 2026-03-24 |
| File size | 45,790 bytes |
| SHA-256 | ⚠ Not yet computed — run `sha256sum tbs-policy-gov-security-fr.md` |

### TBS Directive on Security Management (DOSM) — English

| Field | Value |
|---|---|
| File | `tbs-dosm-en.md` (target filename when downloaded) |
| Source URL | https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32611 |
| Issuing authority | Treasury Board of Canada Secretariat (TBS) |
| Language | English |
| Policy ID | 32611 |
| Effective date | July 1, 2019 |
| Status | ⚠ NOT YET DOWNLOADED — see fetch instructions below |

### TBS Directive on Security Management (DOSM) — French

| Field | Value |
|---|---|
| File | `tbs-dosm-fr.md` (target filename when downloaded) |
| Source URL | https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32611 |
| Issuing authority | Secrétariat du Conseil du Trésor du Canada (SCT) |
| Language | French (fr-CA) |
| Policy ID | 32611 |
| Status | ⚠ NOT YET DOWNLOADED — see fetch instructions below |

### TBS DOSM Appendix J — Standard on Security Categorization — English

| Field | Value |
|---|---|
| File | `tbs-dosm-appendix-j-security-categorization-en.html` |
| Source URL | https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32614 |
| Issuing authority | Treasury Board of Canada Secretariat (TBS) |
| Language | English |
| Policy ID | 32614 |
| Effective date | July 1, 2019 |
| Content | Defines Protected A, Protected B, Protected C tiers by injury type |
| Acquisition method | Firefox "Save Complete Web Page" (WAF blocks curl/WebFetch) |
| Acquisition date | 2026-03-31 |
| Status | ✓ Downloaded |

### TBS DOSM Appendix J — Standard on Security Categorization — French

| Field | Value |
|---|---|
| File | `tbs-dosm-appendix-j-security-categorization-fr.html` |
| Source URL | https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32614 |
| Issuing authority | Secrétariat du Conseil du Trésor du Canada (SCT) |
| Language | French (fr-CA) |
| Policy ID | 32614 |
| Content | Norme sur la catégorisation de la sécurité — définit Protégé A, B, C |
| Acquisition method | Firefox "Save Complete Web Page" (WAF blocks curl/WebFetch) |
| Acquisition date | 2026-03-31 |
| Status | ✓ Downloaded |

### UK Government Security Classification Policy (GSCP)

| Field | Value |
|---|---|
| File | `uk-gscp-june2023.pdf` |
| Source URL | https://www.gov.uk/government/publications/government-security-classifications (Cabinet Office) |
| Issuing authority | Cabinet Office, UK Government |
| Language | English |
| Version | June 2023 |
| Acquisition date | 2026-03-24 |
| File size | 293,469 bytes |
| SHA-256 | ⚠ Not yet computed — run `sha256sum uk-gscp-june2023.pdf` |

### New Zealand Information Security Manual (NZISM) 3.7

| Field | Value |
|---|---|
| File | `nzism-3.7.pdf` |
| Source URL | https://www.nzism.gcsb.govt.nz/ |
| Issuing authority | Government Communications Security Bureau (GCSB), New Zealand |
| Language | English |
| Version | 3.7 |
| Acquisition date | 2026-03-24 |
| File size | 3,369,360 bytes |
| SHA-256 | ⚠ Not yet computed — run `sha256sum nzism-3.7.pdf` |

---

## Outstanding Acquisitions

### Priority 1: TBS DOSM and Appendix J (Protected A/B/C definitions)

The Policy on Government Security (PGS) defines governance obligations. The **Protected A/B/C
tier definitions** are in Appendix J of the Directive on Security Management (DOSM). This is
the document Henri and Simone need for classification tier mapping.

Fetch commands (tbs-sct.canada.ca curl works with Firefox UA per MEMORY.md):

```bash
# Directive on Security Management — English
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32611&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-en.md'

# Directive on Security Management — French
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  -H "Accept-Language: fr-CA,fr;q=0.9" \
  "https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32611&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-fr.md'

# Appendix J: Standard on Security Categorization — English (PRIORITY: has Protected A/B/C)
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32614&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-appendix-j-security-categorization-en.md'

# Appendix J: Norme sur la catégorisation de la sécurité — French (PRIORITY)
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  -H "Accept-Language: fr-CA,fr;q=0.9" \
  "https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32614&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-appendix-j-security-categorization-fr.md'
```

After each download: verify the file is not empty and contains "Protected" (EN) or "Protégé" (FR)
content, then compute SHA-256 and update SOURCE.md and refs-manifest.md.

### Priority 2: Compute checksums for already-downloaded files

```bash
cd /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/
sha256sum tbs-policy-gov-security-en.md \
          tbs-policy-gov-security-fr.md \
          uk-gscp-june2023.pdf \
          nzism-3.7.pdf
```

Record results in this SOURCE.md and in refs-manifest.md.

### Priority 3: Australia PSPF Policy 8

The Protective Security Policy Framework (PSPF) Policy 8 contains Table 30 — the definitive
Five Eyes classification equivalency table. Source: `protectivesecurity.gov.au`.

URL: https://www.protectivesecurity.gov.au/publications-library/policy-8-sensitive-and-classified-information

Fetch when Bash is available:
```bash
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://www.protectivesecurity.gov.au/publications-library/policy-8-sensitive-and-classified-information" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/au-pspf-policy-8.md'
```

Note: `protectivesecurity.gov.au` source approval pending — check with Jamie before fetching.

---

## Document Relationships

```
Policy on Government Security (PGS) — id=16578
  └── Directive on Security Management (DOSM) — id=32611
        ├── Appendix J: Standard on Security Categorization — id=32614
        │     └── Defines: Protected A, Protected B, Protected C, Classified (Confidential/Secret/Top Secret)
        ├── Appendix A: Standard on Physical Security
        ├── Appendix B: Standard on IT Security
        └── [other appendices]
```

The **PGS** (already downloaded) establishes governance obligations and defines security management
structure. The **DOSM** (needed) provides mandatory procedures. **Appendix J** (needed, highest
priority) contains the classification tier definitions that map directly to UMRS label design.

---

## Content Notes

### PGS Coverage

The downloaded PGS files cover:
- Policy objectives and expected results (§3)
- Requirements for deputy heads (§4)
- Government-wide security roles (§5)
- Appendix A: Security control categories (screening, IT, physical, BCM, IM, contracts, events, training)
- Appendix B: Full definitions glossary (security clearance, classification, compromise, etc.)

The PGS does NOT define Protected A/B/C tiers — that is Appendix J of the DOSM.

### What Appendix J Contains (from search confirmation)

- Information is "Protected" when unauthorized disclosure causes injury outside the national interest
- Information is "Classified" when unauthorized disclosure causes injury to the national interest
- Protected A: injury to an individual or organization (limited)
- Protected B: serious injury to an individual or organization
- Protected C: extremely grave injury (comparable to Classified; rarely used)
- Classified tiers: Confidential → Secret → Top Secret (injury to national interest, escalating)

---

## Relevance to UMRS

| Area | Relevance |
|---|---|
| `CANADIAN-PROTECTED.json` | Appendix J definitions are authoritative for Protected A/B/C MCS category assignments (`c300`–`c399`) |
| `setrans.conf` | Translation entries for Canadian labels must match Appendix J terminology exactly |
| Simone (French tech writer) | FR versions provide canonical bilingual terminology; "Protégé A/B/C" vs "Protected A/B/C" |
| Henri (security policy agent) | Authoritative source for GoC classification policy citations |
| Five Eyes mapping | PGS + Appendix J complete the Canada column in the Five Eyes equivalency matrix |
| `i18n_l10n_rules.md` compliance | Confirms "PROTEGE A" (with accent) is correct; provides Treasury Board source citation |
