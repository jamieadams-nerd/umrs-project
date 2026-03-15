---
name: accreditation-artifacts collection summary
description: Overview of the accreditation-artifacts RAG corpus — what documents it contains, their authority, and how to use them in audit work
type: reference
---

# Accreditation Artifacts — Corpus Summary

**Collection**: `accreditation-artifacts`
**Chunks**: 405
**Familiarization date**: 2026-03-15
**Status**: Familiarized — ready for use in plan reviews and audit work

## Documents in This Collection

| Document | Authority | Version | Purpose in audit work |
|---|---|---|---|
| NIST SP 800-18 Rev. 1 | NIST | Feb 2006 | Canonical SSP structure, roles, system boundary analysis |
| FedRAMP CSP Authorization Playbook | GSA/FedRAMP PMO | v4.2 (2025-11-17) | Authorization phases, authorization package content, SSP/SAP/SAR writing guidance |
| FedRAMP Agency Authorization Playbook | GSA/FedRAMP PMO | v4.1 (2025-11-17) | Agency AO perspective, ConMon requirements, designation lifecycle |
| FedRAMP SSP Template (Rev 5) | GSA/FedRAMP PMO | Rev 5 (2025-08-07) | Required SSP sections A-Q, mandatory appendices |
| FedRAMP SAP Template (Rev 5) | GSA/FedRAMP PMO | Rev 5 (2025-08-07) | Scope, methodology, penetration testing, sampling |
| FedRAMP SAR Template (Rev 5) | GSA/FedRAMP PMO | Rev 5 (2025-08-07) | Risk Exposure Table, SRTM, finding documentation |

**Note**: SAP Training (200-B) and SAR Training (200-C) PDFs are no longer available on fedramp.gov
(removed in Rev5 reorganization). The Rev5 templates cover the same structural ground.

**Note**: FedRAMP is transitioning to FedRAMP 20x. The Rev5 process is expected to cease operations
at end of FY27. These documents cover the current Rev5 agency authorization path.

## Source file locations
- PDFs: `.claude/references/accreditation-artifacts/` and `refs/fedramp/`
- Templates (converted to text): `.claude/references/accreditation-artifacts/*.txt`
- Provenance: `SOURCE.md` in the collection directory with SHA-256 checksums
