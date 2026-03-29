# Term Glossary — oscal-schemas
Generated: 2026-03-28

Terms are listed alphabetically. Source priority: OSCAL schemas (normative) over FedRAMP
profiles (normative extensions). Where terms appear in both, OSCAL schema definition wins.

---

## Assessment Log

**Definition:** A log of all assessment-related actions taken as part of executing an
assessment plan or assessment event. Each entry requires a UUID and start timestamp.
**Source:** AR-SCHEMA, `assessment-log` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** Distinct from the `risk-log` which tracks risk response actions. An
assessment log records what the assessor *did*; a risk log records what was *done about* a risk.

---

## Assessment Plan (SAP)

**Definition:** A Security Assessment Plan — an OSCAL document that defines the scope,
schedule, and methods for a security assessment. Imports an SSP to bound its scope.
**Source:** AP-SCHEMA, `assessment-plan` assembly
**Normative:** yes
**Synonyms / variants:** SAP
**Deprecated forms:** none
**Usage notes:** Required predecessor to an AR. UMRS will need a minimal stub SAP to produce
OSCAL-valid AR output. The `import-ssp` field is required in the schema.

---

## Assessment Results (SAR)

**Definition:** A Security Assessment Report — an OSCAL document containing observations,
findings, risks, and assessment log entries from one or more assessment events.
**Source:** AR-SCHEMA, `assessment-results` assembly
**Normative:** yes
**Synonyms / variants:** SAR, AR
**Deprecated forms:** none
**Usage notes:** Top-level document requires: uuid, metadata, import-ap, results. The `results`
array contains one entry per distinct assessment run.

---

## Back-Matter

**Definition:** A collection of resource citations and attachments referenced by other parts
of the OSCAL document. Each resource has a UUID and may carry descriptions, document-ids,
citations, rlinks, and base64-encoded content.
**Source:** All schemas (shared assembly)
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** Used to embed or reference external evidence files, FedRAMP logo, reference
documents. Not security-critical for UMRS initial implementation.

---

## By-Component

**Definition:** A component-level narrative describing how a specific system component
satisfies a control implementation requirement or statement.
**Source:** SSP-SCHEMA, `by-component` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** Links a component UUID to an implementation description. This is the primary
location for tool-level control satisfaction claims in an SSP.

---

## Catalog

**Definition:** A structured, organized collection of security control information. Controls
are grouped by family and may contain enhancements (sub-controls).
**Source:** CATALOG-SCHEMA, `catalog` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**NIST control reference:** The NIST SP 800-53 Rev 5 control catalog is the normative source
for all control definitions used in UMRS.

---

## CCE (Common Configuration Enumeration)

**Definition:** A reference identifier in the RHEL 10 STIG SCAP content that maps a specific
configuration check to its authoritative NIST control. Format: `CCE-NNNNN-N`.
**Source:** UMRS `catalog.rs` (UMRS-specific field, not an OSCAL standard term)
**Normative:** no (UMRS convention)
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** CCE identifiers are carried in `IndicatorDescriptor.cce`; in OSCAL output
they are encoded as namespaced props or links. Not a native OSCAL field.
**NIST control reference:** CA-2 (assessment evidence traceability)

---

## Component

**Definition:** A defined component that can be part of an implemented system. Type must be
one of: interconnection, software, hardware, service, policy, physical, process-procedure,
plan, guidance, standard, validation.
**Source:** COMPONENT-SCHEMA, `defined-component` assembly
**Normative:** yes
**Synonyms / variants:** system-component (in SSP context)
**Deprecated forms:** none
**Usage notes:** UMRS would be described as type `software`. Each crate or major capability
could be a separate component.

---

## Control ID

**Definition:** A machine-readable token identifying a specific control or control enhancement.
Format: lowercase family abbreviation + hyphen + number (e.g., `si-7`, `au-10`, `cm-6`).
Enhancement format: base-control + period + enhancement number (e.g., `si-7.1`).
**Source:** CATALOG-SCHEMA (TokenDatatype), used in all schemas
**Normative:** yes
**Synonyms / variants:** control-id, with-id
**Deprecated forms:** Uppercase forms (SI-7) are used in prose/doc comments but lowercase
hyphenated tokens are required in OSCAL JSON.
**Usage notes:** UMRS doc comment citations use uppercase (e.g., `NIST SP 800-53 SI-7`);
OSCAL JSON uses lowercase (e.g., `"control-id": "si-7"`). Both forms are correct in their
respective contexts.

---

## Finding

**Definition:** An assessor's conclusion describing an individual security finding, linking
one or more observations to a control objective target with a satisfied/not-satisfied status.
**Source:** AR-SCHEMA, `finding` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** Required fields: uuid, title, description, target (finding-target). Optional:
origins, related-observations, related-risks, implementation-statement-uuid. A finding without
related-observations is valid but unusual in automated assessment.
**NIST control reference:** CA-7 (continuous monitoring), AU-3 (audit record content)

---

## Finding Target

**Definition:** The assessment target of a finding — binds the finding to either a specific
control statement (`statement-id`) or a control objective (`objective-id`) with a satisfaction
state.
**Source:** AR-SCHEMA, `finding-target` assembly (titled "Objective Status" in schema)
**Normative:** yes
**Synonyms / variants:** Objective Status (schema title)
**Deprecated forms:** none
**Usage notes:** Required fields: type (`statement-id` or `objective-id`), target-id (the
token), status.state (`satisfied` or `not-satisfied`). Optional: implementation-status.
The `target-id` must reference a token defined in the SSP control-implementation.

---

## Import-AP

**Definition:** A reference (href URI) from an Assessment Results document back to its
governing Assessment Plan. Required field in the AR top-level assembly.
**Source:** AR-SCHEMA, `import-ap` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** UMRS AR output must include a valid `import-ap.href`. For automated
continuous assessment, a canonical AP URI can be used (e.g., a relative reference to a
co-located SAP file).

---

## Implemented Requirement

**Definition:** A per-control record in an SSP describing how the system satisfies one
specific control. Requires uuid and control-id; all other fields optional.
**Source:** SSP-SCHEMA, `implemented-requirement` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** The `control-id` token must match a control selected in the profile
referenced by `import-profile`. UMRS SSP contributions would populate implemented-requirements
for SI-7, SI-10, SI-11, CM-6, CA-7, and AU-3 at minimum.

---

## Observation

**Definition:** An atomic evidence record describing one observed security condition. The raw
evidence unit from which findings are derived.
**Source:** AR-SCHEMA and POAM-SCHEMA (shared assembly), `observation` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** Required fields: uuid, description, methods, collected (timestamp). Optional:
title, props, links, origins, subjects, relevant-evidence, types, expires. The `methods`
array uses the enum `EXAMINE | INTERVIEW | TEST | UNKNOWN`; automated tool output uses `TEST`.
**NIST control reference:** CA-7 (continuous monitoring), AU-3 (audit record content)

---

## Observation Method

**Definition:** How an observation was made. Enum: `EXAMINE` (document review), `INTERVIEW`
(personnel interview), `TEST` (automated or manual test), `UNKNOWN`.
**Source:** AR-SCHEMA, `methods` field in observation
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** UMRS automated checks use `TEST`. If an indicator is manually reviewed, use `EXAMINE`.

---

## Observation Type

**Definition:** The nature of an observation for filtering purposes. Enum: `ssp-statement-issue`,
`control-objective`, `mitigation`, `finding`, `historic`. Extensible via TokenDatatype.
**Source:** AR-SCHEMA, `types` field in observation
**Normative:** yes (enum values); extensible
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** For UMRS automated observations: use `finding` for Risk-kind, `ssp-statement-issue` for Warning-kind, `control-objective` for Good-kind (positive evidence). Multiple types permitted.

---

## Origin Actor

**Definition:** The source of a finding or observation. Type must be one of: `tool`,
`assessment-platform`, `party`. Requires actor-uuid referencing the tool/party definition.
**Source:** AR-SCHEMA, `origin-actor` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** For UMRS-generated observations, use type `tool` with actor-uuid referencing
the UMRS component definition. This is the OSCAL foundation for AU-10 non-repudiation at HIGH.

---

## POA&M Item

**Definition:** An individual plan of action and milestones entry representing a tracked
remediation action. Requires title and description.
**Source:** POAM-SCHEMA, `poam-item` assembly
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** Links to related-findings, related-observations, related-risks by UUID reference.
Note that uuid is optional on the poam-item itself in schema (unlike most OSCAL assemblies).

---

## Profile

**Definition:** A control selection overlay that imports one or more catalogs, selects a
subset of controls, and applies parameter modifications. FedRAMP provides profiles for LOW,
MODERATE, and HIGH impact tiers.
**Source:** PROFILE-SCHEMA, `profile` assembly
**Normative:** yes
**Synonyms / variants:** baseline profile, control baseline
**Deprecated forms:** none
**Usage notes:** SSP `import-profile` should reference the FedRAMP profile appropriate for
the system's authorization tier. UMRS targets FedRAMP MODERATE at minimum.

---

## Resolved Profile Catalog

**Definition:** A catalog produced by resolving a profile against its source catalog — all
selected controls appear with full text, resolved parameter values, and any profile additions.
FedRAMP provides resolved-profile catalogs for each tier.
**Source:** FEDRAMP-*-RESOLVED (instances of CATALOG-SCHEMA)
**Normative:** yes
**Synonyms / variants:** resolved catalog
**Deprecated forms:** none
**Usage notes:** Use resolved catalogs (not raw profiles) to read full control text. The
resolved catalog contains FedRAMP FR (additional requirement) parts alongside NIST control text.

---

## Result

**Definition:** One assessment run within an Assessment Results document. Carries the
observations, findings, risks, and assessment log from a discrete assessment event. Requires
uuid, title, description, start, reviewed-controls.
**Source:** AR-SCHEMA, `result` assembly
**Normative:** yes
**Synonyms / variants:** assessment result
**Deprecated forms:** none
**Usage notes:** For UMRS continuous assessment: each `PostureSnapshot` produces one `result`
with `start` = snapshot timestamp. `reviewed-controls` must list the controls being assessed.

---

## Risk

**Definition:** An identified risk to the system, including status, characterizations,
mitigating factors, remediations, and a risk log.
**Source:** AR-SCHEMA and POAM-SCHEMA (shared assembly), `risk` assembly
**Normative:** yes
**Synonyms / variants:** identified risk
**Deprecated forms:** none
**Usage notes:** Risk status uses `risk-status` field (open, closed, investigating, remediating,
deviation-requested, deviation-approved). Not every Risk-kind UMRS observation needs an OSCAL
risk; reserve for findings with active remediation tracking.
**NIST control reference:** RA-3 (risk assessment), CA-5 (POA&M)

---

## Security Assessment Plan

See Assessment Plan (SAP).

---

## Security Assessment Report

See Assessment Results (SAR).

---

## Statement ID

**Definition:** A human-oriented identifier for a specific part of a control statement
within the SSP. Format follows the control-id + part name convention (e.g., `si-7_smt.a`).
**Source:** SSP-SCHEMA (`statement-id` field), CATALOG-SCHEMA (part `id` attributes)
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** Finding targets that use `type: "statement-id"` must reference tokens that
appear in the SSP `control-implementation.implemented-requirements[].statements[].statement-id`.
The token format derives from the catalog part naming convention.

---

## System Security Plan (SSP)

**Definition:** A machine-readable document describing a system's security posture, including
system characteristics, boundary, information types, CIA impact levels, and per-control
implementation narratives.
**Source:** SSP-SCHEMA, `system-security-plan` assembly; reference: NIST SP 800-18
**Normative:** yes
**Synonyms / variants:** SSP
**Deprecated forms:** none
**Usage notes:** Required fields: uuid, metadata, import-profile, system-characteristics,
system-implementation, control-implementation. UMRS would produce an SSP contribution
describing its components and the controls it satisfies within the system boundary.
**NIST control reference:** PL-2 (system security and privacy plans)

---

## UUID (Universally Unique Identifier)

**Definition:** A machine-oriented, globally unique identifier used to reference OSCAL
assemblies within and across documents. Must be a valid UUID (8-4-4-4-12 hex format).
**Source:** All schemas (UUIDDatatype)
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**Usage notes:** OSCAL requires per-subject UUID assignment — the same real-world object
should use the same UUID across revisions of a document. For UMRS: each indicator in the
catalog should have a stable UUID so findings can be tracked across assessment runs.
