# Cross-Reference Map — rmf-methodology
Generated: 2026-03-15

---

## Agreements

### Risk assessment feeds authorization decisions
**Documents in agreement:** SP800-37, SP800-30, SP800-39
**Shared guidance:** Risk assessments conducted per SP 800-30 are a required input to the RMF Authorize step (task R-2) and to SP 800-39's Respond component; the AO's risk determination is not a substitute for a documented risk assessment.

### Continuous monitoring replaces point-in-time authorization
**Documents in agreement:** SP800-37, SP800-39
**Shared guidance:** Both documents describe ongoing authorization as the mature end state; the continuous monitoring strategy produces real-time risk information that allows the AO to maintain authorization without a fixed expiration date.

### Assessment findings produce two possible outcomes
**Documents in agreement:** SP800-53A, SP800-37
**Shared guidance:** Both documents use "satisfied" / "other than satisfied" as the only valid assessment findings; SP 800-53A defines these terms; SP 800-37 task A-3 cites SP 800-53A as the governing procedure source.

### Risk response options are consistent
**Documents in agreement:** SP800-30, SP800-39
**Shared guidance:** Both define the same five risk response options: accept, avoid, mitigate, share, transfer. SP 800-39 provides the organizational context; SP 800-30 provides the operational procedure.

### ODP values must be defined for controls to be assessable
**Documents in agreement:** SP800-53A, SP800-37
**Shared guidance:** SP 800-53A states that undefined ODPs produce "other than satisfied" findings; SP 800-37 Select step task S-2 (tailoring) explicitly requires organizations to assign ODP values.

---

## Tensions

### Scope of risk assessment: system-level vs. organizational
**Documents in conflict:** SP800-30 vs. SP800-39
**SP800-30 position:** Provides methodology for conducting a risk assessment — primarily operationalized at the system level (Tier 3), though applicable at all tiers.
**SP800-39 position:** Emphasizes that risk assessment must occur at all three tiers simultaneously; system-level risk assessments alone are insufficient for organizational risk management.
**Nature of conflict:** scope difference — SP 800-30 is a tool; SP 800-39 is the governance framework that determines when and where to use it.
**Resolution:** Apply SP 800-39's three-tier framing when determining which stakeholders must be informed of a finding. Use SP 800-30 tables and scales to characterize the risk itself.

### Assessor independence: required vs. discretionary for privacy
**Documents in conflict:** SP800-37 (Task A-1) vs. OMB A-130 (referenced by SP 800-37)
**SP800-37 position:** Assessor independence is required for security control assessments; the AO determines the level of independence required.
**OMB A-130 position (cited in SP 800-37 footnote):** An independent evaluation of the privacy program is not required; organizations may choose to employ independent privacy assessments.
**Nature of conflict:** context-dependent — security vs. privacy domains have different independence requirements.
**Resolution:** For UMRS audit work (security controls), treat assessor independence as required. Privacy control assessments may use the senior agency official for privacy as assessor.

### Assessment reuse: efficiency vs. currency
**Documents in conflict:** SP800-53A vs. SP800-37 (Monitor step)
**SP800-53A position:** Prior assessment results (including from CCEVS, CMVP, system developers) may be reused if they meet established reuse criteria.
**SP800-37 Monitor position:** Ongoing assessments must be conducted continuously at organization-defined frequency; stale assessment results cannot substitute for current monitoring.
**Nature of conflict:** context-dependent — reuse is appropriate for initial authorization; ongoing authorization requires current results.
**Resolution:** Reuse is appropriate during the RMF Assess step to reduce duplication. For Monitor step tasks M-2 (ongoing assessments), current evidence is required.

---

## Chains (deference relationships)

### Risk assessment methodology
**Primary:** SP800-39 (frames what risk assessment must accomplish at each tier)
**Defers to:** SP800-30 for the specific methodology, scales, tables, and determination procedures
**Agent behavior:** Consult SP 800-30 for any question about how to conduct a risk assessment or characterize a risk level. Consult SP 800-39 for questions about scope, tier, and organizational context.

### Control assessment procedures
**Primary:** SP800-37 (RMF Assess step tasks A-1 through A-6)
**Defers to:** SP800-53A for the actual assessment procedures (methods, objects, depth, coverage) for each control
**Agent behavior:** SP 800-37 describes the process; SP 800-53A provides the content. Always cite SP 800-53A when specifying what must be examined, interviewed, or tested for a specific control.

### System categorization
**Primary:** SP800-37 (task C-1)
**Defers to:** FIPS 199 (categorization methodology) and SP 800-60 (information type to impact level mapping)
**Agent behavior:** SP 800-37 mandates categorization; FIPS 199 and SP 800-60 define how to do it. These documents are outside this collection but are normatively required.

---

## Gaps

### FIPS 199 and SP 800-60 (categorization methodology)
**Not covered by:** any document in this collection
**Agent behavior:** Flag to user when categorization methodology questions arise. SP 800-37 mandates FIPS 199 categorization but the methodology itself is not in this collection. Do not invent guidance on impact level determination.

### SP 800-128 (security-focused configuration management)
**Not covered by:** any document in this collection
**Agent behavior:** SP 800-37 Monitor task M-1 references SP 800-128 for configuration management. When auditing configuration management controls (CM family), note that SP 800-128 provides deeper procedural guidance not available in this collection.

### Privacy risk assessment methodology (IR 8062)
**Not covered by:** any document in this collection
**Agent behavior:** SP 800-37 references IR 8062 for privacy risk assessment methodology (alongside SP 800-30 for security). Privacy risk factors are not covered in this collection. Flag when privacy-specific risk questions arise.

### NIST Cybersecurity Framework (CSF) mapping
**Not covered by:** any document in this collection
**Agent behavior:** SP 800-37 Authorize and Monitor tasks reference NIST CSF constructs (ID.RA, RS.AN, etc.). The CSF is not in this collection. Apply SP 800-53 control citations for UMRS audit work; note CSF alignment only where SP 800-37 explicitly calls it out.

### Specific control family assessment procedures beyond AC
**Not covered by:** SP 800-53A pages read (pp.1-49 cover only through AC-02 enhancements)
**Agent behavior:** The SP 800-53A catalog is very large (~450 pages of tables). The security auditor has read the full methodology and AC family as representative. For AU, CM, IA, SC, SI control family assessment procedures, the structure is identical to what was read — use the same Examine/Interview/Test pattern with control-specific objects. Retrieve specific AU/CM/SC/SI procedures from the RAG if exact object lists are needed.
