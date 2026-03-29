# Knowledge Base — oscal-schemas
Familiarization pass completed: 2026-03-28
Agent: security-auditor

---

## Collection summary

15 files, approximately 468,000 lines of JSON. The collection covers two categories:

**OSCAL v1.1.2 JSON schemas (8 files):** Machine-readable format definitions for the full
OSCAL document suite — catalog, profile, SSP, component definition, assessment plan,
assessment results, POA&M, and the combined complete schema. These are the normative format
specifications from NIST (usnistgov).

**FedRAMP Rev 5 baselines (6 files):** Official GSA FedRAMP control selection profiles and
their resolved-profile catalogs for LOW, MODERATE, and HIGH impact tiers. These define which
NIST SP 800-53 controls are required at each tier and carry FedRAMP-specific parameter values
and additional requirements.

A `SOURCE.md` file tracks provenance and update procedures.

---

## Knowledge artifacts

| File | Description |
|---|---|
| `concept-index.md` | Per-document entry: coverage, key concepts, writing tasks governed, related documents |
| `cross-reference-map.md` | Agreements, tensions, deference chains, and gaps across the collection |
| `style-decision-record.md` | UMRS-specific resolutions for OSCAL integration decisions (6 SDRs) |
| `term-glossary.md` | Canonical OSCAL and FedRAMP terminology with definitions and usage notes |

---

## Notable findings from this pass

### Finding 1: SI-7, SI-10, SI-11 are MODERATE-required — UMRS already cites them correctly
UMRS source code citations for SI-7 (TPI disagreement, IMA), SI-10 (dual-parser validation),
and SI-11 (parse failure observations) are precisely aligned with the FedRAMP MODERATE control
set. The project is annotating to the right tier.

### Finding 2: AU-10 (non-repudiation) is HIGH-only and UMRS has a gap
AU-10 is required at FedRAMP HIGH but absent at MODERATE. UMRS currently produces
observations without cryptographic origin binding. For a HIGH authorization, this is a gap.
The AR schema's `origin-actor` structure (type: tool, actor-uuid) is the OSCAL mechanism for
AU-10 compliance, but it requires a verifiable tool identity (certificate or signed artifact).
SDR-002 tracks this as pending project owner decision.

### Finding 3: UMRS's flat SecurityObservation model needs a translation layer for OSCAL
OSCAL uses three distinct tiers: observation → finding → risk. UMRS uses a single
`SecurityObservation` type with polarity tagging. The mapping is clean and automatable (SDR-003),
but the assessment-engine must implement it explicitly. No UMRS code currently produces
OSCAL-shaped output.

### Finding 4: FedRAMP profiles declare OSCAL 1.0.4 against 1.1.2 schemas — version mismatch
All six FedRAMP baseline files declare `"oscal-version": "1.0.4"` while the schema collection
is 1.1.2. Schema validation of FedRAMP files against 1.1.2 schemas may surface compatibility
warnings. The FedRAMP automation repository has not released 1.1.2 versions as of the
collection retrieval date (2026-03-23).

### Finding 5: OSCAL has no native severity field — UMRS needs a namespace convention
OSCAL observations carry `types` and `methods` but no severity (Critical/High/Medium/Low).
SDR-005 establishes a namespaced prop convention for UMRS. This is not a blocker but must
be implemented before UMRS AR output is consumed by external tools.

---

## Open questions requiring project owner input

- SDR-002: How to satisfy AU-10 at HIGH — defer to auditd, or add UMRS-layer signing?
- SDR-004: Which FedRAMP FR (additional requirement) parts apply to UMRS's scope vs. cloud provider scope?
