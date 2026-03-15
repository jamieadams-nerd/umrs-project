# Style Decision Record — rmf-methodology Collection
Generated: 2026-03-15
Owner: security-auditor agent

This record resolves tensions identified in the cross-reference map.
Entries here take precedence over any individual source document.

---

## SDR-001: Severity label mapping to SP 800-30 risk levels

**Tension:** UMRS audit reports use HIGH/MEDIUM/LOW severity. SP 800-30 uses a five-level semi-quantitative scale (Very High / High / Moderate / Low / Very Low). The mapping must be consistent.
**Sources involved:** SP800-30, UMRS audit report format (CLAUDE.md)
**Decision:** Map as follows:
- UMRS HIGH = SP 800-30 High or Very High impact to organizational operations, assets, or individuals
- UMRS MEDIUM = SP 800-30 Moderate impact, or High/Very High impact with low/very low likelihood
- UMRS LOW = SP 800-30 Low or Very Low impact at Tier 3, or indirect risk only
**Applies when:** Assigning severity to any UMRS audit finding
**Does not apply when:** Writing a formal SP 800-30 risk assessment document (use five-level scale there)
**Rationale:** The three-level UMRS scale must remain actionable; collapsing Very High and High into HIGH preserves urgency signaling without introducing a fifth severity tier.
**Status:** Resolved

---

## SDR-002: Citing SP 800-53A for assessment method evidence

**Tension:** When a UMRS audit finding identifies a missing or incorrect control citation, the remediation must specify what evidence an assessor would look for. SP 800-53A provides the authoritative evidence list per control.
**Sources involved:** SP800-53A, UMRS audit report format
**Decision:** In the "Recommended citation" and finding description fields, cite the SP 800-53 control identifier. When describing what an assessor would examine to verify the gap, use SP 800-53A assessment objects for the relevant method (Examine: system design documentation, security plan; Test: mechanisms implementing the control).
**Applies when:** Writing findings where the missing citation maps to a verifiable control
**Does not apply when:** The gap is a pure documentation style issue with no associated control requirement
**Rationale:** Grounds UMRS findings in auditor-reproducible evidence; a reviewer can use the SP 800-53A procedure to verify the finding independently.
**Status:** Resolved

---

## SDR-003: Tier attribution for UMRS audit findings

**Tension:** SP 800-39's three-tier model implies findings can be Tier 1 (org governance), Tier 2 (mission/business process), or Tier 3 (information system). UMRS audit reports are currently system-level (Tier 3) only.
**Sources involved:** SP800-39, UMRS audit report format
**Decision:** UMRS security auditor reports operate at Tier 3 by default. When a finding has cross-system or organizational implications (e.g., a systemic annotation gap pattern across all crates), add a "Tier Impact" note to the finding describing Tier 2/1 implications. Do not restructure reports to multi-tier format; add the note inline.
**Applies when:** A finding is not isolated to one file or module — it represents a pattern across the codebase
**Does not apply when:** The finding is isolated to a single file or function
**Rationale:** Keeps reports actionable for coders and tech-writers while giving the AO visibility into systemic risk.
**Status:** Resolved

---

## SDR-004: Authorization package terminology in reports

**Tension:** SP 800-37 defines specific artifact names (SSP, SAR, POA&M) that correspond to UMRS code documentation and audit reports. Using consistent terminology aids transition to formal RMF authorization.
**Sources involved:** SP800-37
**Decision:** Map UMRS artifacts as follows when referencing RMF context:
- UMRS audit report → Security Assessment Report (SAR) component
- UMRS finding with remediation → Plan of Action and Milestones (POA&M) entry
- UMRS crate-level doc comments (security properties) → System Security Plan (SSP) narrative
**Applies when:** Communicating findings to stakeholders with RMF context
**Does not apply when:** Writing internal report headers and finding blocks (use UMRS format there)
**Rationale:** Enables UMRS deliverables to be consumed directly by an AO or assessor working in an RMF context without translation.
**Status:** Resolved

---

## SDR-005: ODP treatment in annotation gap findings ⚠ PENDING

**Tension:** SP 800-53A states that undefined ODPs produce "other than satisfied" findings. UMRS code annotations cite control identifiers but do not specify ODP values (e.g., a citation to AC-2 does not specify the ODP for account review frequency). It is unclear whether UMRS annotations are expected to capture ODP values.
**Sources involved:** SP800-53A
**Options identified:**
1. Require ODP values in code annotations — consequence: annotations become very verbose; may not be appropriate for code-level docs
2. Accept control-identifier-only citations in code — consequence: an assessor must consult the SSP for ODP values; code and SSP must be consistent
**Recommended default:** Option 2 — accept control-identifier-only citations in code; note ODP values belong in the SSP, not inline code annotations.
**Status:** Pending — requires project owner input before this decision is binding.
**Agent interim behavior:** Apply recommended default; annotate output with [SDR-005 PENDING] when ODP completeness is relevant to a finding.
