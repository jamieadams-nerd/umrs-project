---
name: OSCAL Integration Knowledge
description: OSCAL schema structure, FedRAMP tier control mappings, and UMRS-to-OSCAL translation conventions. Load when auditing OSCAL output or assessment-engine design.
type: project
---

## OSCAL document chain for UMRS

SAP (Assessment Plan) → SAR (Assessment Results) → POA&M
Each AR result requires an import-ap reference. Each AP requires an import-ssp reference.
UMRS will need stub SAP and stub SSP documents before producing OSCAL-valid AR output.

## Required fields for minimal valid AR output

- assessment-results: uuid, metadata, import-ap, results[]
- result: uuid, title, description, start, reviewed-controls
- observation: uuid, description, methods (e.g., ["TEST"]), collected (timestamp)
- finding: uuid, title, description, target (finding-target with type, target-id, status.state)

**Why:** These are the schema-required fields with no defaults. Missing any of them fails validation.
**How to apply:** When reviewing assessment-engine design, verify the output struct covers all required fields before optional ones.

## UMRS SecurityObservation → OSCAL mapping (SDR-003)

- Every SecurityObservation → one OSCAL observation (method: TEST, type varies by kind)
- Risk-kind → also a finding with status.state = "not-satisfied"
- Warning-kind → also a finding with status.state = "not-satisfied", reason = "other"
- Good-kind → optionally a finding with status.state = "satisfied" (positive evidence)
- OSCAL risk assembly only for Risk-kind with active remediation tracking
- OSCAL poam-item only for Risk-kind findings with assigned owner + deadline

**Why:** OSCAL separates evidence (observation) from conclusion (finding) from tracking (risk/poam-item). UMRS's flat model maps cleanly but needs explicit translation at the serialization boundary.

## FedRAMP tier relevance for UMRS-cited controls

| Control | LOW | MODERATE | HIGH |
|---------|-----|----------|------|
| SI-7 (software integrity) | No | Yes | Yes (expanded) |
| SI-10 (input validation) | No | Yes | Yes |
| SI-11 (error handling) | No | Yes | Yes |
| SI-6 (security fn verification) | No | Yes | Yes |
| AU-10 (non-repudiation) | No | No | Yes |
| CM-6 (configuration settings) | Yes | Yes | Yes |
| AU-3 (audit record content) | Yes | Yes | Yes |
| CA-7 (continuous monitoring) | Yes | Yes | Yes |
| CA-5 (POA&M) | Yes | Yes | Yes |

**Why:** UMRS targets MODERATE/HIGH; SI-7/SI-10/SI-11 citations are correct at that tier. AU-10 is HIGH-only and UMRS has a gap (no cryptographic origin binding). See SDR-002.

## FedRAMP version mismatch

FedRAMP profiles declare oscal-version: 1.0.4 but UMRS uses 1.1.2 schemas.
The FedRAMP automation repo (GSA/fedramp-automation) has not released 1.1.2 baseline files as of 2026-03-23.

## OSCAL severity convention (SDR-005)

No native severity field in OSCAL. UMRS convention:
prop name="umrs-severity" ns="https://umrs-project.example/oscal-ns/1.0" value="risk|warning|good"

## CCE in OSCAL (SDR-006)

No native CCE field. UMRS convention:
prop name="cce" ns="https://umrs-project.example/oscal-ns/1.0" value="CCE-88686-1"
Plus link rel="related" for the authoritative SCAP URI.

## Knowledge artifacts location

Full schemas knowledge base: `/DEVELOPMENT/umrs-project/.claude/knowledge/oscal-schemas/`
- concept-index.md — per-document coverage and key concepts
- cross-reference-map.md — agreements, tensions, gaps
- style-decision-record.md — 6 SDRs with UMRS integration decisions
- term-glossary.md — canonical OSCAL/FedRAMP terminology
