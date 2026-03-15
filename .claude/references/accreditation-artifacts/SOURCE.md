# Accreditation Artifacts — Source Reference

Collection: `accreditation-artifacts`
Purpose: SSP/SAP/SAR document structures and accreditation process guidance for the security-auditor agent.
Plan: `.claude/plans/security-auditor-corpus.md` Phase 2
Created: 2026-03-15

## Documents in this collection

All documents require manual download — outbound curl to external URLs is not available in the
researcher agent's current execution context. See `refs/manifest.md` for SHA-256 checksums
after download.

### NIST SP 800-18 Rev. 1

| Field | Value |
|---|---|
| Full title | Guide for Developing Security Plans for Federal Information Systems |
| Issuing authority | NIST |
| Version | Rev 1 |
| Published | February 2006 |
| File | `sp800-18r1.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-18r1.pdf |
| Status | Requires manual download |

### FedRAMP CSP Authorization Playbook v4.2

| Field | Value |
|---|---|
| Full title | FedRAMP CSP Authorization Playbook |
| Issuing authority | GSA / FedRAMP PMO |
| Version | 4.2 (November 17, 2025) |
| File | `fedramp-csp-authorization-playbook.pdf` |
| Source URL | https://www.fedramp.gov/assets/resources/documents/CSP_Authorization_Playbook.pdf |
| Status | Requires manual download |

### FedRAMP Agency Authorization Playbook v4.1

| Field | Value |
|---|---|
| Full title | FedRAMP Agency Authorization Playbook |
| Issuing authority | GSA / FedRAMP PMO |
| Version | 4.1 (November 17, 2025) |
| File | `fedramp-agency-authorization-playbook.pdf` |
| Source URL | https://www.fedramp.gov/resources/documents/Agency_Authorization_Playbook.pdf |
| Status | Requires manual download |

### FedRAMP SAP Training (PDF)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Plan (SAP) Training |
| Issuing authority | GSA / FedRAMP PMO |
| File | `fedramp-sap-training.pdf` |
| Source URL | https://www.fedramp.gov/assets/resources/training/200-B-FedRAMP-Training-Security-Assessment-Plan-SAP.pdf |
| Status | Requires manual download |

### FedRAMP SAR Training (PDF)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Report (SAR) Training |
| Issuing authority | GSA / FedRAMP PMO |
| File | `fedramp-sar-training.pdf` |
| Source URL | https://www.fedramp.gov/assets/resources/training/200-C-FedRAMP-Training-Security-Assessment-Report-SAR.pdf |
| Status | Requires manual download |

### FedRAMP SSP Template (DOCX)

| Field | Value |
|---|---|
| Full title | FedRAMP High/Moderate/Low/LI-SaaS Baseline System Security Plan Template |
| Issuing authority | GSA / FedRAMP PMO |
| Version | Rev 5 |
| File | `fedramp-ssp-template.docx` |
| Source URL | https://www.fedramp.gov/resources/templates/FedRAMP-High-Moderate-Low-LI-SaaS-Baseline-System-Security-Plan-(SSP).docx |
| Status | Requires manual download (DOCX format — note RAG ingestion may require conversion) |

### FedRAMP SAP Template (DOCX)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Plan Template |
| Issuing authority | GSA / FedRAMP PMO |
| Version | Rev 5 |
| File | `fedramp-sap-template.docx` |
| Source URL | https://www.fedramp.gov/resources/templates/FedRAMP-Security-Assessment-Plan-(SAP)-Template.docx |
| Status | Requires manual download (DOCX format — note RAG ingestion may require conversion) |

### FedRAMP SAR Template (DOCX)

| Field | Value |
|---|---|
| Full title | FedRAMP Security Assessment Report Template |
| Issuing authority | GSA / FedRAMP PMO |
| Version | Rev 5 (December 6, 2024) |
| File | `fedramp-sar-template.docx` |
| Source URL | https://www.fedramp.gov/assets/resources/templates/FedRAMP-Security-Assessment-Report-(SAR)-Template.docx |
| Status | Requires manual download (DOCX format — note RAG ingestion may require conversion) |

## Manual Download Instructions

```bash
# Create the target directory
mkdir -p .claude/references/accreditation-artifacts/

# NIST SP 800-18 Rev. 1 (approved NIST source)
curl -L -o .claude/references/accreditation-artifacts/sp800-18r1.pdf \
  "https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-18r1.pdf"
sha256sum .claude/references/accreditation-artifacts/sp800-18r1.pdf

# Also copy to refs/nist/ for the official manifest
cp .claude/references/accreditation-artifacts/sp800-18r1.pdf refs/nist/sp800-18r1.pdf

# FedRAMP CSP Authorization Playbook (PDF)
curl -L -o .claude/references/accreditation-artifacts/fedramp-csp-authorization-playbook.pdf \
  "https://www.fedramp.gov/assets/resources/documents/CSP_Authorization_Playbook.pdf"
sha256sum .claude/references/accreditation-artifacts/fedramp-csp-authorization-playbook.pdf

# FedRAMP Agency Authorization Playbook (PDF)
curl -L -o .claude/references/accreditation-artifacts/fedramp-agency-authorization-playbook.pdf \
  "https://www.fedramp.gov/resources/documents/Agency_Authorization_Playbook.pdf"
sha256sum .claude/references/accreditation-artifacts/fedramp-agency-authorization-playbook.pdf

# FedRAMP SAP Training PDF
curl -L -o .claude/references/accreditation-artifacts/fedramp-sap-training.pdf \
  "https://www.fedramp.gov/assets/resources/training/200-B-FedRAMP-Training-Security-Assessment-Plan-SAP.pdf"
sha256sum .claude/references/accreditation-artifacts/fedramp-sap-training.pdf

# FedRAMP SAR Training PDF
curl -L -o .claude/references/accreditation-artifacts/fedramp-sar-training.pdf \
  "https://www.fedramp.gov/assets/resources/training/200-C-FedRAMP-Training-Security-Assessment-Report-SAR.pdf"
sha256sum .claude/references/accreditation-artifacts/fedramp-sar-training.pdf

# FedRAMP SSP Template (DOCX)
curl -L -o ".claude/references/accreditation-artifacts/fedramp-ssp-template.docx" \
  "https://www.fedramp.gov/resources/templates/FedRAMP-High-Moderate-Low-LI-SaaS-Baseline-System-Security-Plan-(SSP).docx"

# FedRAMP SAP Template (DOCX)
curl -L -o ".claude/references/accreditation-artifacts/fedramp-sap-template.docx" \
  "https://www.fedramp.gov/resources/templates/FedRAMP-Security-Assessment-Plan-(SAP)-Template.docx"

# FedRAMP SAR Template (DOCX)
curl -L -o ".claude/references/accreditation-artifacts/fedramp-sar-template.docx" \
  "https://www.fedramp.gov/assets/resources/templates/FedRAMP-Security-Assessment-Report-(SAR)-Template.docx"
```

## Note on DOCX RAG Ingestion

The FedRAMP SSP/SAP/SAR templates are available only as .docx files (no PDF versions published
by FedRAMP). The existing `ingest.py` script does not handle .docx. Options:

1. Convert to PDF using LibreOffice headless before ingestion:
   `libreoffice --headless --convert-to pdf fedramp-ssp-template.docx`

2. Convert to plain text using pandoc:
   `pandoc fedramp-ssp-template.docx -o fedramp-ssp-template.txt`
   Then place the .txt file in this directory for ingestion.

3. Use the FedRAMP training PDFs (SAP Training, SAR Training) as PDF alternatives —
   they cover the same structural ground in a format ingest.py can handle.

Recommendation: Use option 2 (pandoc .txt conversion) for the templates, and also
ingest the training PDFs. The training PDFs are self-contained and cover the expected
structure of each document type.

## FedRAMP Source Note

fedramp.gov is the official GSA program website. This is an approved source for
accreditation process artifacts (government website, publicly available, no registration required).
The "Security Assessment Framework" as a standalone document was replaced by the playbook series
in the Rev5 transition. The CSP Authorization Playbook v4.2 and Agency Authorization Playbook v4.1
are the current authoritative replacements.
