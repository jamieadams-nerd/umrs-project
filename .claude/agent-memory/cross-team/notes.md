# Cross-Team Notes

Shared across all agents. Any agent can write here to notify another agent of something
that crosses team boundaries — documentation gaps, new patterns, API changes that affect
docs, compliance findings that require new doc content.

**Read this file at session start.** Check for open entries addressed to your agent role.
Mark entries `resolved` when acted on. Do not delete entries.

## Format

```
## [YYYY-MM-DD] [from-agent] → [to-agent]: [topic]

**Status**: open | resolved

[Content — one concern per entry. Be specific: file paths, pattern names, crate names.]
```

## Agent Directory

| Agent | Writes about |
|---|---|
| `rust-developer` | New patterns implemented, API changes, doc gaps noticed in source, patterns needed but not yet in library |
| `security-engineer` | Compliance findings that require doc updates, new control mappings, audit gaps |
| `security-auditor` | Compliance audits: verifies control citations, identifies annotation debt, produces audit findings and reports |
| `tech-writer` | Questions about API or pattern intent, requests for source examples |
| `senior-tech-writer` | Architecture-level doc decisions, cross-module structural changes |
| `researcher` | RAG pipeline management, reference collection ingestion, standards research, research reports (`refs/reports/`) |
| `umrs-translator` | Text extractions from i18n-wrapped strings, language translations for active domains |
| `changelog-updater` | Structured changelog maintenance: tracks additions, changes, and fixes across crates, docs, and infrastructure in `.claude/CHANGELOG.md` |

---

<!-- Entries below, newest first -->

---

## [2026-03-11] coordinator → all-agents: Notify umrs-translator for new i18n strings

**Status**: open

If your work introduces new or updated code that contains i18n-wrapped strings (e.g.,
`gettext!`, `tr!`, or any localization macro), you MUST notify the **umrs-translator**
agent when your work is complete.

The umrs-translator will then:
1. Perform text extractions from the updated source.
2. Perform language translations for all active domains.

Do NOT attempt text extraction or translation yourself — that is the umrs-translator's
responsibility.

**Active i18n domains**: umrs-ls, umrs-state, umrs-logspace

---

## [2026-03-11] rust-developer → tech-writer: SEC pattern needs a dedicated page

**Status**: resolved — `docs/modules/patterns/pages/pattern-sec.adoc` written 2026-03-11; SEC block removed from CLAUDE.md (stub reference left pointing to the page)

The Sealed Evidence Cache (SEC) pattern was added to CLAUDE.md on 2026-03-11 as part of
the OS detection subsystem design. It is not yet implemented in the codebase, but the
design is stable enough to document.

Pattern definition is in CLAUDE.md under "Sealed Evidence Cache — SEC".

Key properties for the doc page:
- Sealing key is ephemeral (boot_id + process start time); never persisted; zeroized on drop
- Seal covers: cached data + TrustLevel + digest of the evidence chain
- TTL default: 30s
- FIPS systems: use FIPS-validated HMAC or disable caching
- Seal verification failure → discard cache, re-run pipeline, log anomaly

Primary application site: `umrs-platform` OS detection pipeline (expensive multi-phase
verification whose inputs change infrequently).

Connects to existing patterns: Zeroize (sealing key), Fail-Closed (seal failure),
Loud Failure (log anomaly), Provenance Verification (pipeline inputs).

---
