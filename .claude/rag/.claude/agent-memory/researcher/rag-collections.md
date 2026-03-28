---
name: RAG Augmentation Collections
description: Tracked RAG collections in .claude/references/ with source URLs for update checks
type: reference
---

# RAG Augmentation Collections

Collections stored under `.claude/references/` (RAG material, lighter provenance requirements than `refs/`).

## tbs-canada-style

**Purpose:** TBS Canada.ca Content Style Guide (English + French) — plain language and web writing standards for Government of Canada digital services.

**Acquired:** 2026-03-28

**Source URLs:**
- English: https://design.canada.ca/style-guide/
- French: https://conception.canada.ca/guide-redaction/

**Update history:**
- English: https://design.canada.ca/style-guide/update-history.html
- French: https://conception.canada.ca/guide-redaction/historique-modifications.html

**Last known update:** August 2024 (plain language section, ISO 24495-1:2023 alignment)

**Acquisition method:** WebSearch-synthesized (Bash/WebFetch unavailable in that session)

**Re-fetch recommended:** Yes — use curl + pandoc when Bash is available (see SOURCE.md for commands)

**Relevance tags:** plain-language, GC-communications, Simone, Sage, STE-mode, accessibility, French-Canadian
