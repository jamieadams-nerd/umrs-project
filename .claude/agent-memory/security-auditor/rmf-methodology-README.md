# rmf-methodology Collection — Familiarization Summary
Familiarization date: 2026-03-15
Agent: security-auditor

---

## Collection overview

Four NIST documents covering the full Risk Management Framework (RMF) and its supporting methodologies. Together they establish the normative vocabulary for risk assessment, control selection, control assessment, and authorization that governs UMRS security audit work.

**Document count:** 4
**Total pages read:** ~330 pages across all four documents

| Document | Source | Pages |
|---|---|---|
| NIST SP 800-37 Rev. 2 | refs/nist/sp800-37r2.pdf | ~96 |
| NIST SP 800-53A Rev. 5 | refs/nist/sp800-53Ar5.pdf | ~49 (methodology + AC family; catalog is ~450 pages total) |
| NIST SP 800-30 Rev. 1 | refs/nist/sp800-30r1.pdf | ~40 |
| NIST SP 800-39 | refs/nist/sp800-39.pdf | ~88 |

---

## Collection purpose

Provides the security auditor with active knowledge of:
1. The RMF lifecycle and where security annotation audits fit within it (Assess step, Task A-3)
2. The assessment procedure structure that makes audit findings reproducible by external assessors
3. The risk characterization vocabulary needed to assign severity levels in audit reports
4. The organizational risk management context that determines when a finding escalates beyond Tier 3

---

## Artifact files

- `concept-index.md` — per-document entries covering scope, key concepts, governing tasks, and cross-references
- `cross-reference-map.md` — agreements, tensions, deference chains, and gaps across the four documents
- `style-decision-record.md` — five project-specific resolutions including severity mapping and tier attribution
- `term-glossary.md` — canonical definitions for ~25 RMF/assessment terms including all mandatory entries

---

## Key connections to UMRS audit work

**Annotation gap → SAR finding:** A missing or incorrect control citation in UMRS source code is an "other than satisfied" finding on the assessment objective for the relevant control (typically SA-11, PL-2, or the specific control being cited). The SP 800-53A Examine method lists "system design documentation" as an assessment object — UMRS code doc comments are part of this.

**Severity calibration:** UMRS HIGH severity corresponds to SP 800-30 High or Very High impact. A HIGH finding is one where the missing or incorrect claim directly affects an authorization, classification, or enforcement decision — exactly what would drive an AO's risk determination.

**POA&M mapping:** Every UMRS finding with a remediation owner maps to a POA&M entry. Findings assigned to "coder" are implementation deficiencies; findings assigned to "tech-writer" are documentation deficiencies. Both belong in the authorization package.

**Ongoing authorization:** The Monitor step's Task M-2 (ongoing assessments) is the RMF process that UMRS audit reports directly support. Regular security audits of the codebase are the automated/ongoing assessment mechanism for UMRS.

---

## Notable gaps and open questions

1. **SP 800-53A catalog coverage:** Only the methodology and AC family were read. The AU, CM, IA, SC, SI families (most relevant to UMRS annotations) follow identical structure. Retrieve specific procedures from RAG when exact object lists are needed for a finding.

2. **FIPS 199 / SP 800-60 not in collection:** System categorization methodology is referenced but not available here. Do not derive impact levels without consulting these documents.

3. **SDR-005 pending:** Whether code-level annotations must capture ODP values is unresolved. Current position: control-identifier-only citations are acceptable; ODP values belong in SSP.

4. **Privacy risk methodology (IR 8062):** Not in this collection. Treat all UMRS audit work as security-focused; flag if privacy risk assessment questions arise.
