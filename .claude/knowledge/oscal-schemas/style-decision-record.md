# Style Decision Record — UMRS OSCAL Integration
Generated: 2026-03-28
Owner: security-auditor (initial pass); Pending project owner review for SDR-002, SDR-004

This record resolves tensions identified in the cross-reference map. Entries here take
precedence over any individual source document for UMRS integration decisions.

---

## SDR-001: SI-7 citation scope for UMRS annotations

**Tension:** UMRS source code cites SI-7 on IMA-related observations and integrity checks.
FedRAMP LOW does not require SI-7. Is the citation appropriate?
**Sources involved:** FEDRAMP-LOW-PROFILE, FEDRAMP-MODERATE-PROFILE, UMRS source code
**Decision:** SI-7 citations in UMRS source code are correct and should be retained. UMRS
targets MODERATE and HIGH deployment tiers where SI-7 is required. The LOW baseline is not
the target. Annotations should additionally note the tier applicability when the distinction
matters.
**Applies when:** Reviewing or adding SI-7 citations in UMRS source code
**Does not apply when:** Generating output for a LOW-tier customer deployment (flag SI-7
features as above-baseline for that customer)
**Rationale:** UMRS is a high-assurance platform explicitly designed for DoD/MODERATE+ environments; LOW-tier control mapping would misrepresent its security posture
**Status:** Resolved

---

## SDR-002: AU-10 non-repudiation gap ⚠ PENDING

**Tension:** FedRAMP HIGH requires AU-10 (non-repudiation), which in OSCAL AR maps to
cryptographically bound origin-actor tracing on observations. UMRS currently produces
`SecurityObservation` records with no cryptographic signature binding.
**Sources involved:** FEDRAMP-HIGH-PROFILE, AR-SCHEMA (origin-actor)
**Options identified:**
  1. Accept the gap — document that UMRS addresses AU-10 at MODERATE (not HIGH) and note the gap in the SSP
  2. Add a signing capability to UMRS AR output — each observation gets a cryptographic origin token (tool certificate or HMAC)
  3. Defer to the platform's audit subsystem (auditd) for AU-10 coverage; UMRS does not need to independently satisfy AU-10
**Recommended default:** Option 3 (defer to auditd) — rationale: AU-10 non-repudiation for system-level events is an auditd/kernel responsibility; UMRS assessment output can reference the auditd trail via a link in the AR back-matter
**Status:** Pending — requires project owner input before this decision is binding.
**Agent interim behavior:** Apply recommended default (option 3); annotate output with
  `[SDR-002 PENDING]` in any UMRS OSCAL design document so the project owner can review.

---

## SDR-003: UMRS SecurityObservation to OSCAL three-tier mapping

**Tension:** UMRS uses a flat `SecurityObservation` enum with `ObservationKind` (Good/Warning/Risk).
OSCAL separates observation (evidence), finding (conclusion), and risk (threat to system) into
distinct assemblies with different required fields.
**Sources involved:** AR-SCHEMA, POAM-SCHEMA, UMRS `observations.rs`
**Decision:** Map UMRS concepts to OSCAL tiers as follows:
  - Every `SecurityObservation` instance → one OSCAL `observation` (with method `TEST`, type `finding`)
  - `ObservationKind::Risk` observations → additionally promoted to an OSCAL `finding` with `target.status.state = "not-satisfied"`
  - `ObservationKind::Warning` observations → promoted to a `finding` with `target.status.state = "not-satisfied"` and `status.reason = "other"`, remarks explaining the conditional nature
  - `ObservationKind::Good` observations → optionally promoted to a `finding` with `target.status.state = "satisfied"` (positive evidence of control satisfaction)
  - OSCAL `risk` assembly is reserved for Risk-kind observations that also have a `risk-log` and remediation timeline; not all Risk findings need a risk object
  - OSCAL `poam-item` is generated only for Risk-kind findings with an assigned remediation owner and deadline
**Applies when:** Designing UMRS OSCAL AR output, building the assessment-engine
**Does not apply when:** Internal UMRS processing — the Rust types remain as-is; translation happens at the OSCAL serialization boundary only
**Rationale:** Preserves UMRS's clean internal model while producing fully OSCAL-valid output; the mapping is deterministic and automatable
**Status:** Resolved (pending implementation by assessment-engine)

---

## SDR-004: FedRAMP FR parts must be addressed alongside NIST control text

**Tension:** FedRAMP resolved catalogs add `_fr`, `_fr_smt`, `_fr_gdn` parts with additional
requirements beyond the base NIST 800-53 control. SSP narratives that address only the NIST
statement part will fail FedRAMP assessment.
**Sources involved:** FEDRAMP-MODERATE-RESOLVED, FEDRAMP-HIGH-RESOLVED (FR part content)
**Decision:** When writing SSP control narratives for UMRS, always check the resolved catalog
for the target tier and address both the NIST statement parts and the FedRAMP FR parts. In
OSCAL SSP by-component descriptions, reference both the `_smt` and `_fr_smt` statement IDs.
**Applies when:** Writing SSP control implementation narratives for any FedRAMP-tier deployment
**Does not apply when:** Writing against the raw NIST 800-53 catalog (non-FedRAMP context)
**Rationale:** FedRAMP assessors will evaluate FR parts; missing them in the SSP will generate findings
**Status:** Pending — requires project owner input on which FR parts apply to UMRS scope (some FR parts address cloud-provider responsibilities, not tool-developer responsibilities).

---

## SDR-005: Observation severity representation in OSCAL

**Tension:** UMRS `ObservationKind` (Good/Warning/Risk) has no direct field in OSCAL observation.
The schema uses `types` and `methods` but not severity.
**Sources involved:** AR-SCHEMA (no native severity field)
**Decision:** Use a namespaced `prop` to carry severity in UMRS-generated OSCAL observations:
  `prop name="umrs-severity" ns="https://umrs-project.example/oscal-ns/1.0" value="risk|warning|good"`
  Additionally, for Risk observations map to `types: ["finding"]` and for Warning map to `types: ["ssp-statement-issue"]`.
**Applies when:** Serializing UMRS observations to OSCAL AR format
**Does not apply when:** Internal Rust processing
**Rationale:** Namespaced props are the OSCAL-idiomatic extension mechanism; using a standard namespace ensures other tools can ignore but not corrupt the property
**Status:** Resolved (convention established; implementation deferred to assessment-engine sprint)

---

## SDR-006: CCE identifier representation in OSCAL output

**Tension:** UMRS `IndicatorDescriptor.cce` carries CCE identifiers with no standard OSCAL
field to receive them.
**Sources involved:** AR-SCHEMA (no native CCE field), UMRS `catalog.rs`
**Decision:** Carry CCE identifiers in OSCAL observations as links with `rel="related"` and
href in the form `https://access.redhat.com/security/cve/CCE-NNNNN-N` or the canonical
SCAP content URI. Also duplicate as a namespaced prop for machine readability:
  `prop name="cce" ns="https://umrs-project.example/oscal-ns/1.0" value="CCE-88686-1"`
**Applies when:** Serializing IndicatorDescriptor entries with non-None cce fields to OSCAL
**Does not apply when:** Observations derived from SecurityObservation (no CCE mapping exists for those)
**Rationale:** Links preserve the authoritative SCAP reference; props allow automated querying without URI resolution
**Status:** Resolved (convention established)
