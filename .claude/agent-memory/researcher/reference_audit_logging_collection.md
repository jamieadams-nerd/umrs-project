---
name: Audit Logging Research Collection
description: Location and purpose of the audit-logging RAG corpus acquired 2026-04-02
type: reference
---

Collection directory: `/DEVELOPMENT/umrs-project/.claude/research/audit-logging/`

Seven documents acquired 2026-04-02 for UMRS audit logging architecture research:

- `cc-part2-rev1-2022.pdf` — Common Criteria Part 2 Rev1 2022; FAU (Security Audit) functional component requirements
- `3gpp-ts28111-v18.1.0.pdf` — 3GPP TS 28.111 v18.1.0; fault management alarm model and severity taxonomy
- `rfc4765-idmef.txt` — RFC 4765 IDMEF; XML data model for intrusion detection event exchange
- `cef-implementation-standard.pdf` — CEF Implementation Standard (ArcSight 8.4); SIEM wire format
- `dfxml-2012.pdf` — DFXML paper (Garfinkel 2012); forensic XML schema for file/volume provenance
- `case-ontology-intro.html` — CASE Ontology intro; cyber-investigation provenance ontology
- `itu-x721.pdf` — ITU-T X.721 (02/92); GDMO managed object class definitions underlying 3GPP alarm model

All files verified with SHA-256. Full checksums in SOURCE.md.

**ITU note:** The dologin_pub.asp URL for X.721 delivered the actual document without authentication
on 2026-04-02. This may not be reproducible. Fallback: https://www.itu.int/rec/T-REC-X.721/en
then select 02/1992 edition → PDF (free).

This collection is NOT yet RAG-ingested. Familiarization status: none. Decision pending on
whether to ingest (large corpus, multi-agent search) or familiarize only.
