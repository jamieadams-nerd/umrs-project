# Accreditation Artifacts — Source Reference

Collection: `accreditation-artifacts`
Purpose: SSP/SAP/SAR document structures and accreditation process guidance for the security-auditor agent.
Plan: `.claude/plans/security-auditor-corpus.md` Phase 2
Created: 2026-03-15
Downloaded: 2026-03-15

## Documents in this collection

All documents downloaded 2026-03-15 via curl. See `refs/manifest.md` for full manifest entries
with SHA-256 checksums and provenance details.

**URL corrections applied during download**: Several original URLs contained S3 website
redirect stubs (83–85 bytes, `binary/octet-stream` with `x-amz-website-redirect-location`
header pointing to path + trailing slash). The actual file paths are at `/resources/documents/`
and `/resources/templates/`, not `/assets/resources/`. The SAR template was previously
listed at `/assets/resources/templates/` — corrected to `/resources/templates/`.

The 200-B and 200-C training PDFs (SAP Training, SAR Training) were removed from fedramp.gov
as part of the Rev5 reorganization. Both URLs return S3 redirect stubs. These files are no
longer available from the official source. The Rev5 SSP/SAP/SAR templates cover the same
structural ground.

### NIST SP 800-18 Rev. 1

| Field | Value |
|---|---|
| Full title | Guide for Developing Security Plans for Federal Information Systems |
| Issuing authority | NIST |
| Version | Rev 1 |
| Published | February 2006 |
| File | `sp800-18r1.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-18r1.pdf |
| SHA-256 | `1635870f7cac9c0606cddbf42a7e4d0d8d01c5ab8709d3667821a4cdeaddd506` |
| Size | 367K |
| Status | Downloaded 2026-03-15 |

Also copied to: `refs/nist/sp800-18r1.pdf`

### FedRAMP CSP Authorization Playbook v4.2

| Field | Value |
|---|---|
| Full title | FedRAMP CSP Authorization Playbook |
| Issuing authority | GSA / FedRAMP PMO |
| Version | 4.2 (November 17, 2025) |
| File | `fedramp-csp-authorization-playbook.pdf` |
| Source URL | https://www.fedramp.gov/resources/documents/CSP_Authorization_Playbook.pdf |
| SHA-256 | `6ae7066b90afdc6bc3a54836b6aa1ad59181cda4efef5bb63ed3cb2d0cb5b8e4` |
| Size | 1.5M |
| Status | Downloaded 2026-03-15 |

Also copied to: `refs/fedramp/fedramp-csp-authorization-playbook.pdf`

### FedRAMP Agency Authorization Playbook v4.1

| Field | Value |
|---|---|
| Full title | FedRAMP Agency Authorization Playbook |
| Issuing authority | GSA / FedRAMP PMO |
| Version | 4.1 (November 17, 2025) |
| File | `fedramp-agency-authorization-playbook.pdf` |
| Source URL | https://www.fedramp.gov/resources/documents/Agency_Authorization_Playbook.pdf |
| SHA-256 | `96fa3abc505e7b7aa89fa03d5595678824caaf99b8289a3d7e73c66e8a6cb87d` |
| Size | 928K |
| Status | Downloaded 2026-03-15 |

Also copied to: `refs/fedramp/fedramp-agency-authorization-playbook.pdf`

### FedRAMP SAP Training (PDF)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Plan (SAP) Training |
| Issuing authority | GSA / FedRAMP PMO |
| File | `fedramp-sap-training.pdf` |
| Source URL | https://www.fedramp.gov/assets/resources/training/200-B-FedRAMP-Training-Security-Assessment-Plan-SAP.pdf |
| Status | NOT AVAILABLE — removed from fedramp.gov in Rev5 reorganization (URL returns S3 redirect stub, 83 bytes) |

### FedRAMP SAR Training (PDF)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Report (SAR) Training |
| Issuing authority | GSA / FedRAMP PMO |
| File | `fedramp-sar-training.pdf` |
| Source URL | https://www.fedramp.gov/assets/resources/training/200-C-FedRAMP-Training-Security-Assessment-Report-SAR.pdf |
| Status | NOT AVAILABLE — removed from fedramp.gov in Rev5 reorganization (URL returns S3 redirect stub, 85 bytes) |

### FedRAMP SSP Template (DOCX)

| Field | Value |
|---|---|
| Full title | FedRAMP High/Moderate/Low/LI-SaaS Baseline System Security Plan Template |
| Issuing authority | GSA / FedRAMP PMO |
| Version | Rev 5 (last modified 2025-08-07) |
| File | `fedramp-ssp-template.docx` |
| Source URL | https://www.fedramp.gov/resources/templates/FedRAMP-High-Moderate-Low-LI-SaaS-Baseline-System-Security-Plan-(SSP).docx |
| SHA-256 | `e05d7fb0021cf42f7fe15eed5c21362e99a0182ccb6f428b1015babc59226c48` |
| Size | 215K |
| Status | Downloaded 2026-03-15 |

Converted to plain text for RAG ingestion:
- `fedramp-ssp-template.txt` — SHA-256: `e631914c8afbfad5603220c42ae13038c3b7e42ff9a542e1f7a2cdbb9e02166e` (151K)

Also copied to: `refs/fedramp/fedramp-ssp-template.docx`

### FedRAMP SAP Template (DOCX)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Plan Template |
| Issuing authority | GSA / FedRAMP PMO |
| Version | Rev 5 (last modified 2025-08-07) |
| File | `fedramp-sap-template.docx` |
| Source URL | https://www.fedramp.gov/resources/templates/FedRAMP-Security-Assessment-Plan-(SAP)-Template.docx |
| SHA-256 | `4d2f79e0577cb52a34e2eff1aad0290585c6e0af1ac1f6468d6ef0cba7fc6aae` |
| Size | 141K |
| Status | Downloaded 2026-03-15 |

Converted to plain text for RAG ingestion:
- `fedramp-sap-template.txt` — SHA-256: `004cbf245201cf229e236090716e5761969f77c43f3a964e663be6ff4f326fa4` (91K)

Also copied to: `refs/fedramp/fedramp-sap-template.docx`

### FedRAMP SAR Template (DOCX)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Report Template |
| Issuing authority | GSA / FedRAMP PMO |
| Version | Rev 5 (last modified 2025-08-07) |
| File | `fedramp-sar-template.docx` |
| Source URL | https://www.fedramp.gov/resources/templates/FedRAMP-Security-Assessment-Report-(SAR)-Template.docx |
| SHA-256 | `a779218bd12a5c52f26f8a5edd8896fcd8515801975e9a5f7c09f8228864b9a7` |
| Size | 146K |
| Status | Downloaded 2026-03-15 |

Converted to plain text for RAG ingestion:
- `fedramp-sar-template.txt` — SHA-256: `77f745835de5b2160bed708166d54d7da318794f6a46612dd60db1d3a80b1372` (93K)

Also copied to: `refs/fedramp/fedramp-sar-template.docx`

**URL correction**: Original SOURCE.md had `/assets/resources/templates/...` which is an S3
redirect stub. Correct URL is `/resources/templates/FedRAMP-Security-Assessment-Report-(SAR)-Template.docx`.

## Note on DOCX RAG Ingestion

The FedRAMP SSP/SAP/SAR templates are available only as .docx files (no PDF versions published
by FedRAMP). The three DOCX files have been converted to .txt using pandoc. The .txt files in
this directory are ready for RAG ingestion via ingest.py.

## FedRAMP Source Note

fedramp.gov is the official GSA program website. This is an approved source for
accreditation process artifacts (government website, publicly available, no registration required).
The "Security Assessment Framework" as a standalone document was replaced by the playbook series
in the Rev5 transition. The CSP Authorization Playbook v4.2 and Agency Authorization Playbook v4.1
are the current authoritative replacements.
