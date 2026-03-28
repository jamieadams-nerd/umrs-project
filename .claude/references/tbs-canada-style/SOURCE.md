# TBS Canada Content Style Guide — Collection Source Record

## Collection

**Name:** TBS Canada.ca Content Style Guide (English + French)
**Purpose:** Plain language and web content writing standards for Government of Canada digital services. Relevant to UMRS documentation quality, operator-facing content design, and plain language compliance for any GC-adjacent deliverables.
**Acquired:** 2026-03-28
**Status:** RAG augmentation material — not subject to SHA-256 provenance chain

---

## Files

### English Version

| Field | Value |
|---|---|
| File | `canada-content-style-guide-en.md` |
| Source URL | https://design.canada.ca/style-guide/ |
| Issuing authority | Treasury Board of Canada Secretariat (TBS) |
| Language | English |
| Last major update | August 2024 (plain language section restructured) |
| Acquisition method | WebSearch-synthesized from official Canada.ca pages |
| Acquisition date | 2026-03-28 |

### French Version

| Field | Value |
|---|---|
| File | `canada-content-style-guide-fr.md` |
| Source URL | https://conception.canada.ca/guide-redaction/ |
| Issuing authority | Secrétariat du Conseil du Trésor du Canada (SCT) |
| Language | French (fr-CA) |
| Last major update | Août 2024 (section sur le langage clair restructurée) |
| Acquisition method | WebSearch-synthesized from official Canada.ca pages |
| Acquisition date | 2026-03-28 |

---

## Acquisition Notes

**Method:** Bash (`curl`) and WebFetch were unavailable in this session due to permission constraints. Content was synthesized from WebSearch results against official `canada.ca` and `design.canada.ca` sources. Both the English and French versions draw from the same authoritative source domain.

**Canonical redirect:** The guide was previously hosted at:
- `https://www.canada.ca/en/treasury-board-secretariat/services/government-communications/canada-content-style-guide.html` (English)
- `https://www.canada.ca/fr/secretariat-conseil-tresor/services/communications-gouvernementales/guide-redaction-contenu-canada.html` (French)

Both now redirect to the design system subdomain:
- English: `https://design.canada.ca/style-guide/`
- French: `https://conception.canada.ca/guide-redaction/`

**Completeness note:** The synthesized files represent the full section structure and principal guidance of the guide as of August 2024. The before/after makeover examples and some detailed subsection elaborations are summarized rather than verbatim. For the most current verbatim text, retrieve directly from the canonical URLs above.

**Recommended re-fetch:** Use the following when Bash access is available to get the live HTML:

```bash
# English
curl -L -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://design.canada.ca/style-guide/" \
  | pandoc -f html -t markdown -o canada-content-style-guide-en.md

# French
curl -L -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  -H "Accept-Language: fr-CA,fr;q=0.9" \
  "https://conception.canada.ca/guide-redaction/" \
  | pandoc -f html -t markdown -o canada-content-style-guide-fr.md
```

---

## Update History Reference

Full update history (all changes since May 2023):
- English: https://design.canada.ca/style-guide/update-history.html
- French: https://conception.canada.ca/guide-redaction/historique-modifications.html

Key 2024 changes:
- **August 2024:** Plain language section (2.0) restructured. Aligned with ISO 24495-1:2023. Readability formulas removed from recommendations. New "Getting Started" guidance added.
- **Coordinated update:** TBS Guidelines on Making Communications Products and Activities Accessible updated simultaneously to remove readability formula recommendations.

---

## Relevance to UMRS

| Area | Relevance |
|---|---|
| UMRS documentation | Plain language rules apply to all operator-facing documentation, especially deployment and runbook procedures |
| Simone (French tech writer agent) | Authoritative source for GC French writing conventions, inclusive language, and typography (guillemets, espace insécable, etc.) |
| Sage (English tech writer agent) | Active voice, one idea per sentence, short sentences — directly applicable to STE mode compliance |
| TUI/CLI output | Operator messages and labels should follow the directness and brevity principles in Section 2 |
| Accessibility | Section 6 (images/alt text) and WCAG references apply to any UMRS web-facing material |

---

## Policy Mandate

This guide is mandatory under the **Directive on the Management of Communications** for all GC departments. It is not advisory. For UMRS documentation targeting GC deployment contexts, these rules carry compliance weight.

ISO standard cross-reference: **ISO 24495-1:2023** (Plain language — Part 1: Governing principles and guidelines)
