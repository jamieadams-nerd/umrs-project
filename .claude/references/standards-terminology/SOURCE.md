# Source: Standards Terminology — RAG Collection

## Collection Information

| Field | Value |
|---|---|
| Collection name | standards-terminology |
| Created | 2026-04-18 |
| Purpose | High-signal RAG collection for FHS, LSB, and systemd terminology pattern-matching |

## Files

These are copied from the Knox familiarization artifacts (term glossaries only — not the full corpora).

| File | Origin | Description |
|---|---|---|
| `fhs-2.3-term-glossary.md` | `.claude/knowledge/linux-fhs-2.3/term-glossary.md` | FHS 2.3 canonical terms |
| `lsb-5-term-glossary.md` | `.claude/knowledge/lsb-5/term-glossary.md` | LSB 5.0 canonical terms |
| `systemd-uid-gid-term-glossary.md` | `.claude/knowledge/systemd-uid-gid/term-glossary.md` | systemd UID/GID canonical terms |

## Design Rationale

Small, high-signal, search-oriented. Agents need pattern-match against FHS/LSB/systemd
terminology when citing section numbers or checking path correctness. The raw corpora are
large and Knox is already familiarized with them. Only the distilled glossaries are ingested.
