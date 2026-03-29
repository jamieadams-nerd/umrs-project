# Cross-Reference Map — oscal-schemas
Generated: 2026-03-28

---

## Agreements

### Observation is the atomic evidence unit
**Documents in agreement:** AR-SCHEMA, POAM-SCHEMA, AP-SCHEMA
**Shared guidance:** The `observation` assembly is the same definition in all three documents;
observations are the atomic evidence unit that findings and POA&M items point back to.

### Control IDs use the same token format across all documents
**Documents in agreement:** CATALOG-SCHEMA, PROFILE-SCHEMA, SSP-SCHEMA, AR-SCHEMA, POAM-SCHEMA
**Shared guidance:** Control identifiers (e.g., `si-7`, `au-10`, `cm-6`) are lowercase
hyphenated tokens; this is consistent across every document type and all FedRAMP profiles.

### UUID is mandatory for every addressable object
**Documents in agreement:** All schemas
**Shared guidance:** Every assembly that can be cross-referenced carries a UUID. This is
consistent across AR, SSP, POA&M, SAP, component, and catalog schemas.

### FedRAMP profiles use OSCAL 1.0.4 internally but schemas are 1.1.2
**Documents in agreement:** FEDRAMP-*-PROFILE, FEDRAMP-*-RESOLVED
**Shared guidance:** All FedRAMP profile files declare `"oscal-version": "1.0.4"` in their
metadata despite the schema collection being 1.1.2. The FedRAMP PMO has not yet updated
their baselines to OSCAL 1.1.2. Tooling must handle this version mismatch.

---

## Tensions

### SI-7 tier placement: UMRS cites SI-7 broadly vs. FedRAMP requires it only at MODERATE+
**Documents in conflict:** FEDRAMP-LOW-PROFILE vs. UMRS source code annotations
**FEDRAMP-LOW-PROFILE position:** SI-7 is not selected at LOW; only SI-1, SI-2, SI-3, SI-4, SI-5, SI-12
**UMRS position:** `SecurityObservation::TpiDisagreement` and `ImaHashPresent` cite SI-7; posture catalog entries cite SI-7 for IMA-related indicators
**Nature of conflict:** scope difference — SI-7 is correct for MODERATE and HIGH deployments; the UMRS annotations are accurate for the intended deployment tier (MODERATE/HIGH/DoD)
**Resolution:** See `style-decision-record.md` → SDR-001

### AU-10 tier placement: UMRS does not implement non-repudiation but HIGH requires it
**Documents in conflict:** FEDRAMP-HIGH-PROFILE vs. current UMRS AR output capability
**FEDRAMP-HIGH-PROFILE position:** AU-10 (non-repudiation) is required at HIGH; requires cryptographic binding of actions to identities
**UMRS position:** Origin-actor tracking exists in the AR schema (type: `tool`) but UMRS does not currently produce cryptographically signed observations
**Nature of conflict:** gap — UMRS produces observations but lacks AU-10 compliant signing
**Resolution:** See `style-decision-record.md` → SDR-002

### POA&M item vs. Finding vs. Observation: UMRS uses a flat SecurityObservation enum
**Documents in conflict:** AR-SCHEMA / POAM-SCHEMA (three-tier model) vs. UMRS (two-tier Good/Warning/Risk)
**AR/POAM position:** Distinct tiers: Observation (raw evidence) → Finding (assessor conclusion) → Risk (identified risk) → POA&M item (remediation tracking). Each tier is a separate assembly with different required fields.
**UMRS position:** `SecurityObservation` variants with `ObservationKind` (Good/Warning/Risk) collapse evidence and conclusion into a single type; no separate finding or risk tier
**Nature of conflict:** structural — UMRS's current model maps cleanly to OSCAL observation but requires a translation layer to produce findings and risks
**Resolution:** See `style-decision-record.md` → SDR-003

### FedRAMP resolved catalogs include FedRAMP-added guidance parts not in NIST 800-53
**Documents in conflict:** FEDRAMP-*-RESOLVED vs. NIST SP 800-53 Rev 5 (not in this collection)
**FEDRAMP position:** Adds `_fr`, `_fr_smt`, `_fr_gdn` parts alongside standard NIST parts; these carry FedRAMP-specific requirements that go beyond the base NIST control
**NIST position:** Base control text does not include FedRAMP additions
**Nature of conflict:** additive — FedRAMP adds requirements, does not contradict NIST; but implementations must satisfy both layers
**Resolution:** When writing SSP narratives, address both the NIST statement part and the FedRAMP FR parts. See `style-decision-record.md` → SDR-004

---

## Chains (deference relationships)

### AR → AP → SSP chain
**Primary:** AR-SCHEMA
**Defers to:** AP-SCHEMA for assessment scope definition, then SSP-SCHEMA for system context
**Agent behavior:** To produce a valid AR, an AP must exist first; to produce an AP, an SSP must exist first. For UMRS automated assessment, a minimal stub AP and stub SSP are prerequisites for OSCAL-valid AR output.

### Profile → Catalog chain
**Primary:** PROFILE-SCHEMA (FedRAMP profiles)
**Defers to:** CATALOG-SCHEMA (NIST SP 800-53 catalog) for control definitions
**Agent behavior:** When resolving a control-id referenced in an SSP or AR, look it up in the resolved catalog for the applicable tier, not the raw profile.

### POA&M → AR chain (by UUID reference)
**Primary:** POAM-SCHEMA
**Defers to:** AR-SCHEMA for observation and risk UUIDs that poam-items reference
**Agent behavior:** POA&M items reference observations and risks by UUID; those UUIDs must exist in an assessment results document. UMRS cannot produce a stand-alone POA&M from tool output alone without first generating an AR.

---

## Gaps

### OSCAL has no native severity/priority field on observations
**Not covered by:** AR-SCHEMA, POAM-SCHEMA
**Situation:** OSCAL observations carry `types` and `methods` but no standardized severity level (Critical/High/Medium/Low). Severity is typically conveyed via props with a namespace-qualified name (e.g., `risk-level` prop). No standard prop name is defined in the schema.
**Agent behavior:** When designing UMRS AR output, define a project-namespaced prop for severity. Flag to user that severity representation is not standardized in OSCAL 1.1.2. Recommended approach: `prop name="risk-level" ns="https://umrs.example/ns" value="high"`.

### No native mapping from OSCAL observation to CCE identifier
**Not covered by:** AR-SCHEMA, CATALOG-SCHEMA
**Situation:** UMRS `IndicatorDescriptor.cce` carries CCE identifiers (RHEL 10 STIG SCAP content). OSCAL observations have no standard field for CCE. The link can be expressed via a `prop` or `link` with appropriate relation.
**Agent behavior:** Use `link rel="related" href="cce:CCE-NNNNN-N"` or a namespaced prop to carry CCE in OSCAL output. This is a gap in the standard that the project must handle with a convention.

### OSCAL AR schema does not define a machine-readable pass/fail status at the observation level
**Not covered by:** AR-SCHEMA at the observation level
**Situation:** Observations do not carry a pass/fail status — that is reserved for `finding-target.status.state` in findings. UMRS `ObservationKind` (Good/Warning/Risk) has no direct analog at the observation tier; it would need to be encoded as a prop or expressed by promoting to a finding.
**Agent behavior:** For UMRS Risk-kind observations, promote to a finding with `status.state = "not-satisfied"`. For Good-kind observations, use `finding-target.status.state = "satisfied"`. For Warning-kind, create a finding with `status.reason = "other"` and a remarks explanation.

### FedRAMP profiles reference OSCAL 1.0.4 but collection schemas are 1.1.2
**Not covered by:** Any document in this collection with resolution guidance
**Situation:** The FedRAMP files declare `oscal-version: 1.0.4` but UMRS is using 1.1.2 schemas. Differences between these versions may cause validation failures.
**Agent behavior:** Flag version mismatch when validating FedRAMP profiles against 1.1.2 schemas. Do not assume backward compatibility without testing. Recommend checking FedRAMP automation GitHub for 1.1.2 updates.

### No OSCAL document type covers continuous/automated assessment natively
**Not covered by:** Any single document
**Situation:** OSCAL was designed for point-in-time assessments. Continuous assessment (UMRS's use case) requires generating AR documents repeatedly. There is no standard pattern for incremental AR updates or time-series observation aggregation.
**Agent behavior:** Design UMRS AR output as point-in-time snapshots keyed by `boot_id` and timestamp. Each PostureSnapshot becomes one AR result object with its own start/end timestamps. Cross-snapshot trending is a UMRS-layer concern, not an OSCAL concern.
