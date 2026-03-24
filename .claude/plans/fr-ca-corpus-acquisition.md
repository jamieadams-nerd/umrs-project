# Plan: French Canadian Corpus Acquisition

**Status:** Tier 1 complete; Tier 2 backburner (2026-03-23)
**Date:** 2026-03-23
**Tech Lead:** The Librarian (researcher)
**Consumer:** Simone (umrs-translator)
**ROADMAP Goals:** G9 (Five Eyes / M3 translations)
**LOE:** Medium (~2-3 sessions for structured databases, ~1-2 for prose corpora)
**Source:** `.claude/jamies_brain/improve-french.txt` + Simone's feedback

---

## Problem

The `french-lookup` skill currently searches GNU `.po` files that skew toward European
French (fr-FR). Canadian French (fr-CA) differs in vocabulary, tone, anglicism handling,
and bureaucratic register. Translation engines and open-source corpora default to France
French unless corrected. UMRS targets Five Eyes francophone operators — the register must
be Canadian federal/military, not Parisian academic.

Simone has identified specific terminology gaps where she coined terms without corpus
validation: "verrou dur" (hard gate), "palier de confiance" (trust tier), "posture"
(security posture), "indicateur de posture de securite" (security posture indicator).

## Acquisition Priority

### Tier 1 — Structured Terminology Databases (highest leverage, unblock french-lookup)

**1. Termium Plus (Government of Canada Translation Bureau)**
- URL: https://www.btb.termiumplus.gc.ca/
- Content: Federal bilingual terminology bank — security, IT, public administration, defense
- Why: Calibrated to federal bilingualism requirements; exactly UMRS's compliance register
- Format needed: Queryable terminology export or scraped entries for domains:
  `security`, `information technology`, `public administration`, `national defence`
- Delivery: Structured `.tsv` or similar for `french-lookup` skill integration
- Note: May require scraping the web interface; check if bulk download or API exists

**2. OQLF Grand dictionnaire terminologique (GDT)**
- URL: https://vitrinelinguistique.oqlf.gouv.qc.ca/
- Content: Quebec official terminology — informatique, administration, defense
- Why: Canonical fr-CA forms with domain tags; catches "France French drift"
- Format needed: Same as Termium — structured entries per domain
- Delivery: Structured `.tsv` for `french-lookup` skill integration
- Note: GDT has a web search interface; check for API or data export options

### Tier 2 — Prose Corpora (phrasing context, RAG collection)

**3. Government of Canada / Treasury Board IT Documentation**
- Sources:
  - Treasury Board of Canada Secretariat: https://www.canada.ca/en/treasury-board-secretariat.html
  - Shared Services Canada IT docs
  - Canadian Centre for Cyber Security (CCCS): https://www.cyber.gc.ca/
  - Look for bilingual EN/FR security policy, compliance, and IT guidance documents
- Why: Compliance-heavy, operator-facing, structured prose in exactly UMRS's register
- Format needed: RAG corpus (Chroma collection `gc-bilingual` or similar)
- Delivery: Download bilingual PDFs/HTML, ingest via rag-ingest skill
- Priority within tier: CCCS cybersecurity publications first (closest domain match)

**4. Canadian Armed Forces (CAF) Publications**
- Sources:
  - https://www.canada.ca/en/department-national-defence.html
  - Training manuals, operational guides, procedural docs (bilingual)
  - ITAR-free, publicly available publications only
- Why: High-pressure clarity language, minimal fluff, operational register
- Format needed: RAG corpus
- Delivery: Download and ingest
- Priority within tier: Lower than GC/Treasury Board but valuable for operational phrasing

## Integration Plan

### french-lookup skill enhancement
Once Tier 1 databases are acquired:
1. Add Termium Plus and GDT as priority search sources in the `french-lookup` skill
2. Search order becomes: Termium Plus → GDT → GNU coreutils → GNU other
3. Each lookup result should indicate source (Termium/GDT/GNU) so Simone can assess authority

### RAG collections
- `gc-bilingual` — Government of Canada IT/security publications
- `caf-bilingual` — CAF operational publications
- Both collections follow standard ingestion pipeline

### Validation pass
After acquisition, Simone runs a validation pass against her existing `vocabulary-fr_CA.md`
to confirm or correct terms she coined without corpus support:
- "verrou dur" (hard gate)
- "palier de confiance" (trust tier)
- "posture" (security posture)
- "indicateur de posture de securite" (security posture indicator)

## Simone's Additional Recommendation

Simone identified **Termium Plus** as a source Jamie's original notes missed. Her assessment:
"If the Librarian can choose only one structured terminology database, Termium Plus may be
more relevant to UMRS than GDT, because GDT's strength is Quebec commercial and administrative
language whereas Termium Plus is calibrated to federal bilingualism requirements — which is the
register UMRS targets."

Recommendation: acquire both. Termium Plus for federal/defense terms, GDT for Quebec
administrative and IT terms. The two together are authoritative.
