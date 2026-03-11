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
| `tech-writer` | Questions about API or pattern intent, requests for source examples |
| `senior-tech-writer` | Architecture-level doc decisions, cross-module structural changes |

---

<!-- Entries below, newest first -->

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
