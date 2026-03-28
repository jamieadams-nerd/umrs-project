# Concept Index — oscal-schemas
Generated: 2026-03-28

---

## AR-SCHEMA

**Full title:** OSCAL Assessment Results (SAR) Model — JSON Schema
**Source:** `.claude/references/oscal-schemas/oscal_assessment-results_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the machine-readable format for Security Assessment Reports. Covers the full hierarchy
from document envelope (uuid, metadata, import-ap) down to atomic observations, findings, and
risks. Also defines assessment-log entries, origin tracing, and reviewed-controls linkage.

### Key concepts introduced
- **assessment-results** — top-level envelope; requires uuid, metadata, import-ap, results array
- **result** — one assessment run; requires uuid, title, description, start, reviewed-controls
- **observation** — atomic evidence unit; requires uuid, description, methods, collected timestamp
- **finding** — assessor conclusion linking observation(s) to a control target; requires uuid, title, description, target
- **finding-target** — binds a finding to a `statement-id` or `objective-id`; carries `status.state` = `satisfied | not-satisfied`
- **risk** — identified risk with status, characterizations, mitigating-factors, remediations, risk-log
- **origin** — source of a finding or observation; type enum: `tool | assessment-platform | party`
- **observation methods** — `EXAMINE | INTERVIEW | TEST | UNKNOWN`
- **observation types** — `ssp-statement-issue | control-objective | mitigation | finding | historic`
- **import-ap** — required reference back to the governing assessment plan (href only)
- **assessment-log** — time-stamped action log with logged-by and related-tasks

### Governs these writing tasks
- Designing UMRS OSCAL output for the assessment-engine (planned)
- Mapping `SecurityObservation` and `IndicatorReport` to OSCAL observations
- Understanding what fields are mandatory vs. optional for minimal-valid output
- Determining how to link automated tool findings to control objectives
- Understanding how risks feed into POA&M items

### Related documents in corpus
- POAM-SCHEMA — risk and observation structures are shared definitions; a POA&M item can reference observations from an AR
- SSP-SCHEMA — finding-target references `statement-id` values defined in the SSP control-implementation
- AP-SCHEMA — AR imports its governing AP via `import-ap`
- FEDRAMP-HIGH-PROFILE — defines which controls have AU-10 (non-repudiation) at HIGH tier; relevant to observation origin tracing

---

## SSP-SCHEMA

**Full title:** OSCAL System Security Plan (SSP) Model — JSON Schema
**Source:** `.claude/references/oscal-schemas/oscal_ssp_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the machine-readable SSP format per NIST SP 800-18. Covers system characteristics
(identity, sensitivity level, information types, authorization boundary), system implementation
(components, inventory, users, interconnections), and control implementation (per-control
statements, by-component satisfaction narrative). References NIST SP 800-60 for information
type categorization.

### Key concepts introduced
- **system-security-plan** — top-level envelope; requires uuid, metadata, import-profile, system-characteristics, system-implementation, control-implementation
- **import-profile** — href reference to the governing FedRAMP or NIST baseline profile
- **system-characteristics** — requires system-ids, system-name, description, system-information, status, authorization-boundary
- **security-sensitivity-level** — FIPS-199 string (e.g., "moderate")
- **system-information** — information-types with CIA impact levels; categorization system uses NIST SP 800-60 URI
- **security-impact-level** — overall C/I/A impact designation
- **control-implementation** — array of `implemented-requirement` entries keyed by control-id
- **implemented-requirement** — per-control satisfaction record; requires uuid and control-id; optional statements and by-components
- **statement** — addresses a specific control statement part (statement-id)
- **by-component** — component-level satisfaction narrative; links component-uuid to implementation description

### Governs these writing tasks
- Understanding what a UMRS-generated SSP contribution would look like for SELinux or posture controls
- Designing component definitions for UMRS tool components (software type)
- Understanding how FedRAMP profile selection feeds into SSP control scope
- Knowing what `statement-id` tokens look like (used in AR finding-target)

### Related documents in corpus
- AR-SCHEMA — findings reference SSP statement-ids via implementation-statement-uuid
- AP-SCHEMA — assessment plan imports SSP to derive assessment scope
- COMPONENT-SCHEMA — components defined here; reusable via component-definition
- FEDRAMP-MODERATE-PROFILE — defines the control set that must appear in implemented-requirements for FedRAMP Moderate

---

## POAM-SCHEMA

**Full title:** OSCAL Plan of Action and Milestones (POA&M) Model — JSON Schema
**Source:** `.claude/references/oscal-schemas/oscal_poam_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the POA&M format required by FedRAMP and NIST RMF. The POA&M shares observation, risk,
and finding structures with the AR (they are the same assembly definitions). The distinctive
element is `poam-item`, which represents a tracked remediation action linked to findings and
risks. The top-level document requires only uuid, metadata, and poam-items — the SSP import and
system-id are optional.

### Key concepts introduced
- **plan-of-action-and-milestones** — top-level envelope; requires uuid, metadata, poam-items
- **poam-item** — a tracked remediation action; requires title and description; links to related-findings, related-observations, related-risks via UUID references
- **local-definitions** — allows component and inventory definitions when no SSP is available
- **import-ssp** — optional SSP reference (required by FedRAMP in practice but not in schema)
- **poam-item origins** — same actor type enum as AR (tool, assessment-platform, party)

### Governs these writing tasks
- Designing UMRS output path from automated finding → POA&M item
- Understanding the minimum valid POA&M structure (uuid + metadata + poam-items is sufficient)
- Understanding that poam-items reference findings/observations by UUID (not by control-id directly)

### Related documents in corpus
- AR-SCHEMA — observations and risks defined there feed into poam-items here
- SSP-SCHEMA — optional SSP import for context
- FEDRAMP-MODERATE-PROFILE — CA-5 (POA&M) is required at all FedRAMP tiers

---

## AP-SCHEMA

**Full title:** OSCAL Security Assessment Plan (SAP) Model — JSON Schema
**Source:** `.claude/references/oscal-schemas/oscal_assessment-plan_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the Security Assessment Plan format. An SAP imports an SSP to bound the assessment
scope, defines objectives-and-methods, assessment-subjects, and tasks. AR documents must
reference a governing SAP via `import-ap`. For continuous automated assessment (UMRS use case),
a minimal SAP can be generated to satisfy the import requirement.

### Key concepts introduced
- **assessment-plan** — top-level envelope; requires uuid, metadata, import-ssp, reviewed-controls
- **reviewed-controls** — explicit list of controls within scope of this assessment
- **assessment-subjects** — what is being assessed (components, inventory-items, users, etc.)
- **assessment-assets** — tools and platforms used to perform the assessment
- **objectives-and-methods** — local objective definitions with assessment methods (EXAMINE/TEST/INTERVIEW)
- **tasks** — discrete assessment activities with timing and responsibility

### Governs these writing tasks
- Understanding what a UMRS-generated SAP stub needs to contain to allow AR generation
- Knowing that `import-ssp` is required (not optional) in the AP schema
- Designing continuous monitoring workflows where AP, AR, and POA&M form a chain

### Related documents in corpus
- AR-SCHEMA — AR imports AP via `import-ap.href`
- SSP-SCHEMA — AP imports SSP via `import-ssp`

---

## CATALOG-SCHEMA

**Full title:** OSCAL Control Catalog Model — JSON Schema
**Source:** `.claude/references/oscal-schemas/oscal_catalog_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the format for control catalogs (e.g., NIST SP 800-53 Rev 5). A catalog organizes
controls into groups, with each control containing parts (statement text, objectives, guidance),
parameters, and links. The FedRAMP resolved-profile catalogs are instances of this schema with
full control text included.

### Key concepts introduced
- **catalog** — collection of groups and/or controls; requires uuid and metadata
- **group** — organizational container (e.g., a control family like AC, AU, CM)
- **control** — individual control with id, title, params, parts, links, controls (enhancements)
- **part** — named section of control text; part name conventions: `_smt` (statement), `_gdn` (guidance), `_obj` (objective)
- **parameter** — value placeholder within control text; resolved by profiles
- **back-matter** — resource citations and references

### Governs these writing tasks
- Understanding control-id token format (e.g., `si-7`, `au-10`, `cm-6`) for use in SSP/AR/profile
- Understanding control part naming conventions for statement-id and objective-id references
- Understanding that resolved-profile catalogs are self-contained with all text and parameters resolved

### Related documents in corpus
- PROFILE-SCHEMA — profiles select controls from catalogs
- FEDRAMP-*-RESOLVED — resolved catalog instances; use these to read full control text
- SSP-SCHEMA — implemented-requirement control-id tokens reference catalog control IDs

---

## PROFILE-SCHEMA

**Full title:** OSCAL Profile Model — JSON Schema
**Source:** `.claude/references/oscal-schemas/oscal_profile_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the format for control selection overlays (profiles). A profile imports one or more
catalogs or profiles, selects controls via `include-controls` with-ids lists, applies parameter
modifications via `set-parameters`, and merges result into a resolved baseline. FedRAMP baseline
profiles are instances of this schema.

### Key concepts introduced
- **profile** — top-level envelope with imports, merge, and modify sections
- **imports** — catalog or profile references with include-controls / exclude-controls filters
- **with-ids** — explicit control-id list for selection
- **merge.as-is** — preserve original catalog structure in resolved output
- **modify.set-parameters** — FedRAMP-specific parameter constraints (e.g., "at least annually")
- **add/alter** — mechanisms for adding or modifying control parts

### Governs these writing tasks
- Understanding how FedRAMP selects its control subset from NIST SP 800-53
- Knowing that `import-profile` in SSP should reference a profile (or resolved catalog) URI

### Related documents in corpus
- CATALOG-SCHEMA — profiles import from catalogs
- FEDRAMP-*-PROFILE — concrete profile instances
- SSP-SCHEMA — SSP `import-profile` references a profile

---

## COMPONENT-SCHEMA

**Full title:** OSCAL Component Definition Model — JSON Schema
**Source:** `.claude/references/oscal-schemas/oscal_component_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines reusable component definitions that describe how specific products, services, or
configurations satisfy controls. Components carry control-implementations which describe
which controls the component satisfies and how. UMRS tool components (software type) could
be described here.

### Key concepts introduced
- **component-definition** — collection of defined-components and capabilities
- **defined-component** — requires uuid, type, title, description; type enum includes `software`, `hardware`, `service`, `policy`, `validation`
- **capability** — grouping of components; useful for describing UMRS as a security capability
- **control-implementation** — per-component array of implemented-requirement entries
- **protocols** — network protocol descriptions for interconnection components

### Governs these writing tasks
- Defining UMRS as an OSCAL component (type: `software`) with control satisfaction claims
- Describing UMRS capability (integrity checking, posture monitoring) as a named capability
- Reusing component definitions across SSPs

### Related documents in corpus
- SSP-SCHEMA — system-implementation references component UUIDs from component definitions
- AR-SCHEMA — result local-definitions can inline components when no SSP is available

---

## COMPLETE-SCHEMA

**Full title:** OSCAL Complete Schema — All Models Combined
**Source:** `.claude/references/oscal-schemas/oscal_complete_schema.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
A single schema file that combines all OSCAL model definitions. Useful for tooling that needs
to validate any OSCAL document type without selecting the specific schema. Contains the same
definitions as the individual schemas; no additional concepts.

### Key concepts introduced
- No unique concepts; aggregates all individual schema definitions

### Governs these writing tasks
- Validating OSCAL output when the document type is determined at runtime

### Related documents in corpus
- All individual schemas — this is their union

---

## FEDRAMP-LOW-PROFILE

**Full title:** FedRAMP Rev 5 LOW Baseline Profile
**Source:** `.claude/references/oscal-schemas/fedramp_rev5_LOW-baseline_profile.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Selects the FedRAMP LOW impact control subset from NIST SP 800-53 Rev 5. SI family at LOW
includes only SI-1, SI-2, SI-3, SI-4, SI-5, SI-12. Notably absent: SI-6, SI-7 (software
integrity), SI-10, SI-11. AU family at LOW includes AU-1 through AU-12 but not AU-10
(non-repudiation). CA-5 (POA&M) is required.

### Key concepts introduced
- LOW tier control set — minimum control selection for cloud systems processing low-impact data
- OSCAL profile version `fedramp2.1.0-oscal1.0.4` (note: references OSCAL 1.0.4 not 1.1.2)
- Parameter constraints at LOW tier (e.g., policy review "at least every 3 years")

### Governs these writing tasks
- Determining which NIST controls UMRS must cover at minimum for a FedRAMP LOW system
- Identifying that SI-7, SI-10, AU-10 are above LOW tier (MODERATE+)

### Related documents in corpus
- FEDRAMP-LOW-RESOLVED — full control text for LOW selections
- FEDRAMP-MODERATE-PROFILE — adds SI-6, SI-7, SI-10, SI-11, AU-10 among others

---

## FEDRAMP-MODERATE-PROFILE

**Full title:** FedRAMP Rev 5 MODERATE Baseline Profile
**Source:** `.claude/references/oscal-schemas/fedramp_rev5_MODERATE-baseline_profile.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Selects the FedRAMP MODERATE impact control subset. Adds significant SI and AU controls over
LOW: SI-6 (security function verification), SI-7 (software/information integrity), SI-7.1,
SI-7.7, SI-10 (input validation), SI-11 (error handling), SI-16, AU-10 absent at MODERATE
(AU-10 is HIGH only). CM-6, CM-6.1 required. SC-13 (cryptographic protection) required.
CA-5, CA-7, CA-7.1, CA-7.4 required (continuous monitoring).

### Key concepts introduced
- MODERATE control set adds ~100+ controls over LOW
- SI-7 (software and information integrity) required at MODERATE — directly relevant to UMRS IMA integration
- SI-10 (input validation) required at MODERATE — directly relevant to UMRS dual-parser TPI pattern
- SI-11 (error handling) required at MODERATE — relevant to UMRS `SecurityObservation::SelinuxParseFailure`
- SI-6 (security function verification) required at MODERATE — relevant to UMRS posture contradiction detection
- SA-11 (developer testing), SA-15 (development process) required
- SR-2 through SR-12 (supply chain risk management) required

### Governs these writing tasks
- Understanding the target compliance tier for UMRS (MODERATE is the primary target)
- Mapping UMRS features to MODERATE-required controls
- Knowing that AU-10 is not required at MODERATE (it is HIGH-only)

### Related documents in corpus
- FEDRAMP-HIGH-PROFILE — adds AU-10, more SI-7 enhancements, additional CA controls
- FEDRAMP-MODERATE-RESOLVED — full control text

---

## FEDRAMP-HIGH-PROFILE

**Full title:** FedRAMP Rev 5 HIGH Baseline Profile
**Source:** `.claude/references/oscal-schemas/fedramp_rev5_HIGH-baseline_profile.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Selects the FedRAMP HIGH impact control subset. Adds AU-10 (non-repudiation), AU-12.1,
AU-12.3, AU-9.2, AU-9.3 over MODERATE. SI-7 enhancements expanded: SI-7.1, SI-7.2, SI-7.5,
SI-7.7, SI-7.15. Additional CA controls: CA-2.2, CA-7 enhancements. This is the tier most
relevant to UMRS's DoD/classified context.

### Key concepts introduced
- HIGH tier adds AU-10 (non-repudiation) — requires proof that an action occurred and who performed it
- AU-10 parameter constraint: `organization-defined personnel or roles` must be specified
- SI-7 at HIGH has more enhancements (code authentication, binary analysis)
- HIGH tier is the relevant baseline for DoD/CUI/classified systems

### Governs these writing tasks
- Designing UMRS audit output with non-repudiation properties (origin-actor tracing in AR)
- Understanding the full SI-7 implementation requirements at HIGH
- Knowing that AU-10 requires explicit role identification in assessment results

### Related documents in corpus
- AR-SCHEMA — origin-actor in observations/findings maps to AU-10 non-repudiation requirement
- FEDRAMP-HIGH-RESOLVED — full control text including AU-10 parameter values

---

## FEDRAMP-LOW-RESOLVED

**Full title:** FedRAMP Rev 5 LOW Baseline — Resolved Profile Catalog
**Source:** `.claude/references/oscal-schemas/fedramp_rev5_LOW-baseline-resolved-profile_catalog.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Full control text for all LOW baseline selections, with FedRAMP parameter values resolved.
Contains SI-4 monitoring objectives, AU-3 audit record content requirements, CM-6 configuration
settings objectives. Useful as the authoritative text source for any LOW-tier control.

### Key concepts introduced
- Resolved catalog format: controls appear with full statement, guidance, objective, and FedRAMP-specific additions
- FedRAMP adds its own guidance parts (`_fr`, `_fr_smt`, `_fr_gdn`) alongside NIST control text

### Governs these writing tasks
- Reading exact control statement text for SI-4, AU-3, CM-6 at LOW tier

### Related documents in corpus
- CATALOG-SCHEMA — format definition for resolved catalogs
- FEDRAMP-LOW-PROFILE — the profile whose resolution produced this catalog

---

## FEDRAMP-MODERATE-RESOLVED

**Full title:** FedRAMP Rev 5 MODERATE Baseline — Resolved Profile Catalog
**Source:** `.claude/references/oscal-schemas/fedramp_rev5_MODERATE-baseline-resolved-profile_catalog.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Full control text for all MODERATE baseline selections (5.0 MB). Includes complete SI-7, SI-10,
SI-11, SI-6 text with FedRAMP guidance additions. This is the primary reference for UMRS
control implementation narratives.

### Key concepts introduced
- SI-7 full statement: employ integrity verification tools for software, firmware, and information
- SI-10 full statement: check validity of information inputs for accuracy, completeness, valid values
- SI-11 full statement: generate error messages that provide information necessary to corrective actions without revealing sensitive information
- FedRAMP additional requirement (FR) sections added to controls

### Governs these writing tasks
- Writing UMRS SSP narratives for SI-7, SI-10, SI-11 at MODERATE
- Verifying that UMRS TPI dual-parser addresses SI-10 requirements
- Verifying that UMRS `SelinuxParseFailure` observation addresses SI-11 requirements

### Related documents in corpus
- CATALOG-SCHEMA — format definition
- FEDRAMP-MODERATE-PROFILE — the profile whose resolution produced this catalog

---

## FEDRAMP-HIGH-RESOLVED

**Full title:** FedRAMP Rev 5 HIGH Baseline — Resolved Profile Catalog
**Source:** `.claude/references/oscal-schemas/fedramp_rev5_HIGH-baseline-resolved-profile_catalog.json`
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Full control text for all HIGH baseline selections (5.9 MB, largest file). Includes AU-10
non-repudiation full text with FedRAMP guidance. Contains more SI-7 enhancements than MODERATE.
This is the reference for DoD/classified-tier UMRS deployments.

### Key concepts introduced
- AU-10 full statement: provide non-repudiation of information transferred between and within external and internal systems
- AU-10 FedRAMP guidance: requires cryptographic binding of actions to identities
- SI-7.2: notify designated personnel upon discovery of discrepancies during integrity verification

### Governs these writing tasks
- Designing AU-10 compliant audit trails (cryptographic origin binding in AR observations)
- Writing HIGH-tier SSP narratives for UMRS

### Related documents in corpus
- AR-SCHEMA — AU-10 compliance maps to origin-actor with cryptographic tool binding
- FEDRAMP-HIGH-PROFILE — the profile whose resolution produced this catalog
