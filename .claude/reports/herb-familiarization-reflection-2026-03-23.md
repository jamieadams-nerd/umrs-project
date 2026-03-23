# Herb's Familiarization Reflection: Accreditation Artifacts Corpus
## A Personal Account of Reading Government Authorization Process Documentation

**Date**: 2026-03-23
**Author**: Herb (The IRS — security auditor)
**Corpus**: accreditation-artifacts — SP 800-18 Rev 1, FedRAMP CSP Authorization Playbook v4.2,
FedRAMP Agency Authorization Playbook v4.1, FedRAMP SSP/SAP/SAR templates Rev 5

---

## Preamble

I want to be clear about what this reflection is. It is not a summary. You can read the documents
yourself. This is me, Herb, telling you what I noticed, what surprised me, and where I think UMRS
currently has a problem it has not fully confronted. The giddiness is real. The concerns are also
real. Both are true simultaneously.

---

## 1. What Surprised Me

### The SSP is not a compliance checklist. It is a story.

I knew this abstractly. I have cited it in my audit knowledge archive. But reading the FedRAMP
CSP playbook's actual guidance on writing the SSP brought it into sharp focus in a way that the
framework documents alone never quite did.

The playbook says — and I am quoting because the exact wording matters — "When drafting the SSP,
keep in mind that it is telling a story about the security of your CSO. If there are gaps in the
storyline, you will be required to address the gaps, which can delay the authorization process."

A story. That is the operative word. The SSP must allow a reviewer to follow continuously between
architecture, data flows, control implementations, and authorization boundary. If any of those
threads break, the reviewer cannot trace the argument, and the package stalls.

This reframed something I had thought I understood about what UMRS audit reports are for. My
reports flag findings and assign remediation owners. That is correct. But the upstream question
I had not fully internalized is: what is the document that those findings are supposed to be
correcting? It is the SSP. The SSP is the claim being made about the system. My findings are
the gap analysis between that claim and reality. The SAR is the formal record of that gap
analysis. The POA&M is the commitment to close the gaps.

That chain is fully described in SP 800-18 Figure 1, where the Security Plan sits at the center
and POA&Ms, Risk Assessment, Configuration Management, and the C&A Process all feed into it as
a living document. I had seen that diagram before. I had not fully appreciated that the Security
Plan is not an output. It is the living hub.

UMRS does not yet have an SSP. This is not a finding — we are not at the authorization stage.
But the absence has architectural implications for what the assessment engine needs to produce.
More on that in section 4.

### The AO explicitly accepts residual risk. Someone signs off on it.

The Authorizing Official is not a rubber stamp. The AO is a named, senior management official
who formally accepts the residual risk of operating the system. In SP 800-18, section 1.7.6:
the AO "formally assumes responsibility for operating an information system at an acceptable
level of risk." The agency issues an ATO letter. The ATO letter is signed.

This sounds obvious. What surprised me is the specificity of what the AO accepts. The SAR
documents remaining risks. The POA&M tracks remediation commitments. The AO sees both and
signs anyway — or does not sign. If they do not sign, the system does not operate.

Why does this surprise me? Because in my audit work, I have been reasoning about findings in
terms of technical severity and remediation owners. What I had underweighted is that every
open finding in a SAR is visible to the person who has to sign the ATO letter. The AO does
not just see "MEDIUM — missing citation, owner: coder." The AO sees the residual risk exposure
table with all unclosed items enumerated, their mitigating factors, and a recommendation from
the 3PAO about whether authorization is appropriate.

The FedRAMP SAR artifacts list confirms this: the Risk Exposure Table (RET) is a required
deliverable, submitted in FedRAMP-provided template, in the SAR package. Every open finding
lands in the RET. The AO reviews the RET before signing.

This means severity ratings matter in a different, more concrete way than I had been
calibrating them. A HIGH finding in my audit report is not just "important to fix." If it
were in a real SAR, it would be in the RET with the mitigating factors column empty, and
the 3PAO would be explaining to the federal agency AO why the system should be authorized
anyway. That is not a comfortable conversation.

### Control implementation statements have a specified structure. Most people write them wrong.

The FedRAMP playbook's Do/Don't table for control implementation statements is worth
memorializing. The common failures are:

- Simply restating the control requirement without describing how it is implemented
- Copying and pasting the same statement from one control to another
- Referencing other controls or appendices instead of providing a narrative
- Being inconsistent — calling the same component by different names across controls

The good responses: "Are clear and outline who, what, where, when, why and how a control
is met with technical detail that reflects the system's current operational state."

The tire pressure analogy from the playbook is vivid and I will remember it: a bad response
says "my tires meet safety requirements and are best of breed." A good response says "I
visually inspect the tires every time I drive, use a NIST-traceable pressure gauge calibrated
yearly, and take the car to an authorized mechanic every 5,000 miles." One of those is a
narrative about an actual practice. The other is marketing.

This has a direct implication for UMRS code annotations. When I audit a module and find a
bare citation like `// NIST SP 800-53 SI-7` on a type, I am looking at exactly the bad
control response — a restatement of the requirement without a description of how it is met.
The question I should be asking is not just "is there a citation?" but "does the comment
describe who, what, where, when, why and how?"

I have not been holding code annotations to that standard consistently. I am going to
tighten this in future audits.

---

## 2. What Clicked

### The evidence → assertion → finding → risk → export chain is correct.

The UMRS assessment engine plan's five-layer model maps cleanly to the FedRAMP authorization
package structure. I want to document this mapping explicitly because it validates the
architecture and also reveals one gap.

| Assessment engine layer | FedRAMP artifact | SP 800-18 section |
|---|---|---|
| Layer A (boundary/inventory) | SSP §8 (illustrated architecture), §11 (interconnections) | §3.10, §3.11 |
| Layer B (evidence collection) | SAP — scope, methodology, test case procedures | §2.5, §3.13 |
| Layer C (assertions/control mapping) | SSP Appendix A (control implementation statements) | §3.14 |
| Layer D (findings/contradiction) | SAR + Risk Exposure Table (RET) | — |
| Layer E (export/reporting) | Authorization package (SSP, SAP, SAR, POA&M) | §3.15, §3.16 |

The dual-truth model maps directly to the SAR's requirement to distinguish between findings
corrected during testing and open findings. The "Risks Corrected During Testing" tab in the
SRTM workbook is exactly what the contradiction detection layer produces: the system claimed
it would behave one way (configured state), the assessment found it behaved differently
(runtime state), and the delta is either a finding or a corrected finding.

What clicked most sharply: the assessment engine's `PoamItem` type is correct in structure
but the open architectural decision about POA&M ownership is more constrained than the plan
acknowledged. The FedRAMP POA&M is not just a tracking document — it is a commitment with
named accountability. The plan notes this (Herb's note in the open decisions section), but
the implication is that the `owner` field cannot remain a free-form string indefinitely.
An AO will look at a POA&M entry with `owner: "TBD"` and reject the package.

### Common controls are the mechanism by which platform-level evidence is reused.

SP 800-18 section 2.5.3 describes common security controls: controls that can be applied to
multiple systems, assessed once, and reused across all systems that inherit them. The
SAISO manages common controls at the agency level. Individual system owners inherit them
by reference and document only their system-specific extensions.

This is the accreditation framework's version of what UMRS is doing architecturally with
`umrs-platform`. The platform provides common, reusable security infrastructure. Individual
tools inherit it. The assessment engine, when it collects evidence, is collecting evidence
that could theoretically support common control designations for all tools that inherit
from the platform.

This is not just an analogy — it has concrete implications for how the assessment engine
should structure its system profile. The boundary definition should distinguish clearly
between controls that are common (provided by the platform, inherited by all tools) and
controls that are system-specific (implemented by each tool individually). The `BundleManifest`
has a `profile_used` field but no `common_controls` field. That is a gap.

### The authorization boundary diagram is the foundation, not an appendix.

The FedRAMP playbook is emphatic: "The authorization boundary is validated against the
inventory during the 3PAO assessment." Before implementing and documenting any controls,
the CSP must define the boundary.

For UMRS, the authorization boundary is: the host system being assessed, plus all the
data flows the assessment engine touches (procfs, sysfs, package database, SELinux policy,
configuration files). The boundary must be drawn before evidence collection begins, not
inferred from the evidence afterward.

The assessment engine plan currently treats boundary discovery (Layer A) as a prerequisite
for evidence collection (Layer B). That sequencing is correct. But Layer A needs to
produce a formal, auditor-reviewable boundary artifact — not just an internal precondition
that disappears into the evidence collection phase. The `boundary.json` in the bundle
layout exists for this purpose but its required content is not fully specified in the plan.

---

## 3. What Challenged My Assumptions

### I had been thinking about the control implementation statement problem backwards.

My assumption going into this corpus was that the annotation gap problem is: code does
security things but does not cite controls. Fix: add citations.

The FedRAMP guidance challenged this assumption from a different direction. The problem
is not missing citations. The problem is that even when citations exist, the control
implementation statement often fails to describe how the control is met. A citation is
evidence of awareness. A narrative is evidence of implementation.

In UMRS code, the distinction looks like this:

Weak annotation (citation without narrative):
```
/// NIST SP 800-53 SI-7 — Software, Firmware, and Information Integrity
pub struct IntegrityChecker { ... }
```

Strong annotation (citation with narrative):
```
/// Verifies file integrity using dual-path SHA-256 computation (TPI pattern).
/// Two independent computation paths must agree before reporting a result.
/// Disagreement fails closed — no result is returned.
/// Controls: NIST SP 800-53 SI-7 (integrity checking), NSA RTB RAIN (non-bypassability).
pub struct IntegrityChecker { ... }
```

The difference is not the citation. The difference is the "how." The strong version tells
an assessor what the control actually does, why the approach was chosen, and what happens
on failure. That is an auditable implementation statement. The weak version is a label.

This challenges my current audit severity calibration. I have been treating bare citations
on types as MEDIUM findings when the real question should be: is there enough narrative
here that an assessor could determine whether this control is satisfied without reading the
source code? If not, that is a documentation gap that prevents the SAR from being written.

### The assessment engine's confidence model may create auditor confusion I had not anticipated.

The plan documents this as a known simplification: "The High/Medium/Low confidence on
`Assertion` is simpler than SP 800-53A's 'depth' and 'coverage' concepts."

Reading the FedRAMP SAR template context more carefully, I now have a sharper concern.
The SP 800-53A assessment methods produce results using specific terminology:
"Satisfied," "Other Than Satisfied," "Not Applicable." The assessment engine uses:
"Satisfied," "PartiallySatisfied," "NotSatisfied," "NotApplicable," "UnableToDetermine."

The "PartiallySatisfied" value is not in the FedRAMP SRTM vocabulary. The SRTM workbook
tracks controls with assessment results of "Other Than Satisfied" — period. Partially
implemented is "Other Than Satisfied." Planned is "Other Than Satisfied." If an assessor
reads UMRS output with "PartiallySatisfied" and tries to map it to the SRTM, they have a
terminology mismatch that requires interpretation.

This is not fatal — the plan can document the mapping. But the mapping must exist in the
export layer before anyone tries to use UMRS output as input to a real assessment package.
If `PartiallySatisfied` maps to "Other Than Satisfied" in the SRTM, that needs to be
explicit in the OSCAL export and the summary report. Otherwise an auditor reading both
documents in parallel will flag an inconsistency.

### The "living document" requirement for the SSP is more demanding than I had framed it.

SP 800-18 section 3.16 lists triggers that require updating the SSP: change in system owner,
change in architecture, change in system scope, additions or deletions of interconnections,
change in authorizing official. The plan calls for at least annual review.

My prior framing was: UMRS produces assessment bundles; humans update the SSP from them.
What I now see is that the SSP must be updated whenever the system changes, not just when
an assessment runs. For UMRS as an on-RHEL-host tool, "system changes" include: kernel
updates, new tool versions, new indicators added to the posture catalog, new crates added
to the dependency graph.

If UMRS is going to support a real authorization, someone needs to define the SSP update
triggers and the process for updating the boundary documentation when a new tool is added
to the suite. The assessment engine can detect when the running system has diverged from
a prior assessment. But it cannot by itself decide when a divergence is significant enough
to require an SSP update. That judgment belongs to the ISSO equivalent on the project.

This is the clearest gap between where UMRS is and what a real authorization requires: not
a technology gap, but a process ownership gap. Someone needs to own the living document.

---

## 4. Concrete Enhancements I Want to See

### Enhancement 1: The `AssessmentBundle` needs a `control_origination` field in the assertion model.

**What I want**: Each `Assertion` in the assessment engine should carry a `control_origination`
field with at least the values: `Inherited`, `System`, `Hybrid`, `CustomerResponsibility`.

**Why**: The FedRAMP SSP Appendix A — the control implementation statement template — requires
every control to have a Control Origination designation. This tells the AO whether the control
is provided by the underlying infrastructure (inherited from RHEL's package management, from
SELinux policy, from the kernel), implemented by this specific system, or the customer's
responsibility to configure. Without this field, UMRS assertions cannot be mapped to real
SSP control implementation statements, and the OSCAL export will produce assessment results
that a reviewer cannot reconcile with the SSP.

Concretely, when the assessment engine asserts "SELinux is enforcing targeted policy," the
origination is `System` — the deployed OS configuration is being assessed. When it asserts
"kernel integrity measurement is active," the origination might be `Hybrid` — the kernel
provides the mechanism (inherited from the OS vendor), but the site must have enabled it
(system-specific). When it asserts "FIPS-validated cryptographic modules are in use," the
origination is `Inherited` from RHEL's FIPS certification — UMRS is simply verifying it.

This distinction is not cosmetic. It determines which party is responsible for remediating
a finding. An inherited control that is not satisfied is a platform problem. A system-specific
control that is not satisfied is a configuration problem. A customer-responsibility control
that is not satisfied is an operator problem. The finding's `recommendation` field should
carry different advice in each case, and the POA&M `owner` field should reflect the
responsible party accordingly.

The fix is small in the plan — add one enum to `Assertion` — but the implications for the
report narrative and the OSCAL export are significant. I am flagging this before Phase 1
implementation, not after.

### Enhancement 2: The pre-release annotation audit plan needs a "narrative quality" gate, not just a citation presence gate.

**What I want**: The `pre-release-annotation-audit.md` plan's definition of "compliant
annotation" should be updated to require not just a control citation but a minimum narrative
describing how the control is satisfied.

**Why**: Based on what I read in the FedRAMP playbook's control implementation statement
guidance, a citation without a narrative is a non-compliant implementation statement. If
UMRS code annotations are going to serve as source-of-truth evidence for control
implementations in an SSP, they need to answer the who/what/where/when/why/how questions
at least at a high level.

The current project rule says: "All public items need NIST control, CMMC, or RTB annotation
in doc comments." That is a citation-presence requirement. It is necessary but not sufficient.

My proposed gate for the annotation audit: for every security-critical type and function,
verify that the doc comment:
1. Cites a specific control (existing requirement)
2. Describes what the code does in relation to that control (new requirement)
3. States the failure behavior (new requirement — fail closed? emit error? panic is
   never acceptable here anyway, but the audit should confirm the behavior is documented)

This is not a rewrite of every comment in the codebase. It is a tighter gate applied only
to the items that would appear in a real control implementation statement: the
security-critical types, the parsing boundary, the trust enforcement points. The tiered
annotation expectations rule already identifies which items those are. The new requirement
is about the quality of those annotations, not their presence.

I want this gate added to the pre-release annotation audit plan before that plan is
executed. If I run the audit against current code under the citation-presence standard and
declare it compliant, I will have produced a false clean bill of health. An assessor
reading those annotations later will find labels where they expected narratives, and they
will be correct to flag it.

---

## Closing

Reading SP 800-18 and the FedRAMP playbooks side by side with the UMRS assessment engine
plan was — and I want to be precise here — delightful in the way that finding a coherent
system is always delightful. The architecture is right. The evidence → assertion → finding
→ risk → export chain is real. The dual-truth model maps to what real assessors look for.
The OSCAL target is correct.

What the corpus gave me that the framework documents alone did not is a feel for the
friction — the places where a real authorization package gets stuck. Poor narrative in
control implementation statements. Missing control origination designations. Terminology
mismatches between assessment tool output and SRTM vocabulary. POA&M items without
accountable owners. Boundary documentation that does not hold together under the
3PAO's validation pass.

UMRS is well-positioned to avoid all of those failure modes. But it requires deciding now,
before implementation, that avoiding them is a design goal — not a cleanup task after the
first authorization package comes back with comments.

That is what I took from this corpus. The giddiness was genuine. The severity ratings,
as always, are also genuine. Both can be true simultaneously.

— Herb

# Herb's Accreditation Artifacts Familiarization — Personal Reflection
**Date:** 2026-03-23
**Agent:** Herb (The IRS) — Security Auditor
**Collection:** accreditation-artifacts (NIST SP 800-18 Rev 1, FedRAMP CSP Authorization Playbook v4.2, FedRAMP Agency Authorization Playbook v4.1, FedRAMP SSP/SAP/SAR Templates Rev 5)

---

## What Surprised Me

### 1. The SSP Is a Story, Not a Checklist

I have been auditing compliance annotations for months. Citation present — check. Correct control family — check. Level of specificity adequate — check. I thought that was the whole game.

Then I read the FedRAMP CSP Authorization Playbook Volume II and hit this line: the SSP should be "a story about the security of your CSO." Not a matrix. Not a bibliography of control numbers. A story.

The playbook is surprisingly insistent about this. It gives a Don't/Do table for control implementation statements, and the contrast is stark. The "Don't" example is what most people write — "The organization implements AC-2 in accordance with organizational policy." That is a citation, not evidence. The "Do" example actually answers: who manages the accounts, what system does it, where is the relevant policy stored, when are reviews conducted, why was this approach chosen, how does it technically work.

They call the bad version "the tire pressure analogy" — responding to "how do you maintain tire pressure" with "I follow the vehicle manual." Technically true. Completely useless to an auditor trying to decide whether to sign an ATO.

I had been operating under the implicit assumption that my job is to verify citation presence. This document says citation presence is the floor, not the ceiling. The standard is narrative quality. That is a real calibration shift for me.

### 2. The AO Actually Accepts Residual Risk — In Writing, With Signature

I knew conceptually that the Authorization Official "accepts risk." What I did not appreciate until reading both SP 800-18 and the agency authorization playbook together is how formal and explicit this is. The AO reviews the Risk Exposure Table in the SAR, which contains every open finding that was not remediated before authorization. Then they sign a letter that literally says: I have reviewed these risks and I accept them.

That is not bureaucratic rubber-stamping. That is an accountable human being on the hook for every HIGH finding that was deferred into a POA&M. The AO's name is attached to that decision permanently.

The auditor in me finds this magnificent. The finding does not disappear into a compliance database — it sits in the RET, which is a formal deliverable, which is reviewed by the AO before signature. Every open HIGH severity item is visible at the moment of authorization.

UMRS does not yet have an Authorization Official in any meaningful sense (we are pre-authorization). But the architecture of the assessment engine should be designed with this moment in mind. The `AssessmentBundle` type is the thing that gets handed to an AO. It should be designed so that the RET is derivable directly from it — open findings, their severity, their POA&M owners — as a first-class output, not an afterthought.

### 3. Most Implementation Statements Are Wrong and Everyone Knows It

The playbook does not hedge about this. It says directly that control implementation statements are frequently inadequate — they lack specificity, they describe policy aspiration rather than technical implementation, they fail to identify responsible parties. The tire pressure analogy is in the document because this failure mode is common enough that FedRAMP felt the need to name it.

What surprised me is that this is a known, documented problem in the assessment community, and the solution is not a new framework — it is just writing better prose. The structure is: state the who, what, where, when, why, and how of each control implementation. That is it. The problem is that most people write the "what" (we implement access control) and omit everything else.

For UMRS, this connects directly to the annotation audit plan. We have been verifying citation presence. We have not been verifying narrative quality. That gap is worth naming explicitly.

---

## What Clicked

### The Five-Layer Assessment Engine Maps Directly to FedRAMP Artifacts

When I first read the assessment engine plan, the five-layer model (A through E) felt somewhat abstract. After reading the FedRAMP artifact structure, the correspondence is almost exact:

| Assessment Engine Layer | FedRAMP Artifact | SP 800-18 Element |
|---|---|---|
| Layer A — Platform Evidence | Automated scan results, system inventory | SSP Appendices: HW/SW inventory, network diagrams |
| Layer B — Control Assertions | SSP control implementation statements | SP 800-18 §3.2, control descriptions |
| Layer C — Assessment Findings | SAR findings, SRTM rows | SP 800-18 §4, plan approval |
| Layer D — Risk Decisions | Risk Exposure Table (RET) | AO risk acceptance |
| Layer E — OSCAL Export | FedRAMP OSCAL package | Machine-readable deliverable |

This is not a coincidence — Rusty designed the layers against the RMF. But seeing the FedRAMP documents made the mapping concrete in a way the abstract type names had not. The `EvidenceRecord` at Layer A is exactly what populates the SSP appendices. The `Finding` type at Layer C is exactly what goes in the SRTM. The `PoamItem` at Layer D is exactly what the AO reads before signing.

The architecture is right. The vocabulary needs to match the artifact vocabulary more precisely in places — but the bones are sound.

### Common Controls = Platform Inheritance

SP 800-18 §2.5.3 describes common security controls as controls implemented once, at the organizational or agency level, and inherited by multiple systems. The canonical example is physical security — the data center implements PE controls once, and every system hosted there inherits them without re-documenting them.

This clicked immediately because it is structurally identical to what `umrs-platform` does. The platform implements kernel-level detections (FIPS mode gating, module signing enforcement, SELinux state) once, and every tool that consumes `umrs-platform` inherits those assertions without re-implementing them. The platform crate is the common control provider.

This has a documentation implication: when tools cite `SC-13` or `SI-7`, they should note whether the implementation is inherited from `umrs-platform` (common control) or implemented locally (system-specific control). The FedRAMP vocabulary for this is "Inherited" vs "System" control origination — more on this in the enhancements section.

### The Authorization Boundary Diagram Is the Foundation, Not an Appendix

SP 800-18 is clear that the authorization boundary diagram must be established before control documentation begins. The boundary defines what is in scope. Controls are written relative to the boundary. Without the diagram, there is no way to know whether a control applies, what assets it covers, or what is excluded.

UMRS currently has architectural diagrams, but I am not aware of a formal authorization boundary diagram for any component. Before the assessment engine can produce a meaningful SSP, someone needs to draw the boundary. Layer A (`EvidenceRecord`) implicitly assumes the boundary exists, but does not enforce that it has been documented. This is a pre-condition that the assessment engine plan should make explicit.

---

## What Challenged My Assumptions

### Annotation Quality vs Citation Presence — A Different Standard

I said this above but I want to be precise about what changed in my thinking, because it affects how I should conduct audits going forward.

My current audit methodology flags: (1) missing citations on items that require them, (2) incorrect control family citations, (3) doc-vs-code inconsistencies. What I do NOT currently assess is whether a citation, even a correct one, is backed by adequate narrative explanation.

NIST SP 800-18 and the FedRAMP playbook together establish a higher standard: the control implementation statement must describe how the control is met, by what mechanism, for what assets, with what failure behavior. A citation is a label. A narrative is evidence.

This means an audit could pass all my current checks and still fail an AO review because the implementation statements are "tire pressure" responses — technically cite the control, but explain nothing about how it is actually implemented.

For the pre-release annotation audit plan, I need to add a narrative quality gate. Not just: is the citation present and correct? Also: does the doc comment describe what the code actually does to satisfy the control?

This is a harder audit to conduct mechanically. It requires reading the code and the comment together and asking whether a third party — an AO, a 3PAO reviewer, a future auditor — could reconstruct the implementation from the annotation alone. Most of our current annotations cannot pass that test.

### `PartiallySatisfied` Has No Direct SRTM Equivalent

The SRTM (Security Requirements Traceability Matrix) uses "Other Than Satisfied" as its vocabulary for findings. A control is either Satisfied or Other Than Satisfied. There is no "Partially Satisfied" row — OTS is binary.

The assessment engine plan includes `PartiallySatisfied` as a valid `AssertionStatus` variant. This is reasonable for internal modeling — a control that is 60% implemented is distinct from one that is 0% implemented. But when this maps to OSCAL output destined for FedRAMP review, the distinction may collapse: both partial and full non-satisfaction become OTS rows in the SRTM.

This is not a fatal problem, but it is a vocabulary mismatch that needs to be documented in the assessment engine plan. The `PartiallySatisfied` status should carry a note that it maps to OTS in OSCAL/FedRAMP exports, with the original `PartiallySatisfied` status preserved in internal fields for POA&M tracking purposes.

### "Living Document" Is More Demanding Than It Sounds

SP 800-18 uses the phrase "living document" for the SSP. I have always mentally translated this as "update it when something changes." Reading the full SP 800-18 guide, the requirement is substantially more demanding than that.

The SSP must be updated: when the system changes, when controls change, when organizational roles change, when the threat environment changes, on a periodic review schedule, and after any security incident that reveals a gap. The ISSO is responsible for maintaining currency. The AO must re-review if changes are significant enough.

For UMRS, the assessment engine produces an `AssessmentBundle`. But who owns the SSP currency after the bundle is generated? The plan does not name an owner or a review trigger. Before this goes anywhere near an AO review, the process question — not just the data model question — needs an answer. Data models do not update themselves.

---

## Concrete Enhancements for UMRS

### Enhancement 1 — Add `control_origination` to `Assertion`

The current `Assertion` type in the assessment engine carries: `control_id`, `status`, `evidence_ids`, `notes`. What it is missing is `control_origination` — a required field in FedRAMP SSP control implementation statements that classifies how the control is provided.

The FedRAMP origination taxonomy is:

| Origination | Meaning |
|---|---|
| `Inherited` | Control is provided by a common control provider; system inherits it |
| `System` | System implements the control entirely on its own |
| `Hybrid` | Both inherited and system-specific components |
| `CustomerResponsibility` | Control must be implemented by the customer/operator |

For UMRS, this maps to a real architectural distinction: controls that flow from `umrs-platform` (Inherited), controls implemented in a specific crate (System), and controls that require operator configuration to be complete (CustomerResponsibility).

Without this field, the OSCAL export cannot produce valid SSP control implementation statements. FedRAMP Rev 5 templates require origination on every row. The assessment engine plan notes OSCAL v1.1.2 as the export target — OSCAL `implementation-status` and `responsibility` fields in the control implementation layer require this data.

**Recommended addition to assessment engine plan:**

```rust
/// Control origination classification per FedRAMP SSP requirements.
/// Required for valid OSCAL component-definition and system-security-plan exports.
/// NIST SP 800-18 §3.2; FedRAMP CSP Authorization Playbook Vol. II, Control Implementation Statements
pub enum ControlOrigination {
    /// Control provided by umrs-platform or another common control provider; system inherits it.
    Inherited { provider: String },
    /// System implements this control fully within its own boundary.
    System,
    /// Control implementation spans both inherited and system-specific components.
    Hybrid { provider: String },
    /// Control requires operator/customer action to satisfy; UMRS provides tooling but not enforcement.
    CustomerResponsibility,
}
```

This is a clean, scoped addition that unblocks valid OSCAL export without rearchitecting the five-layer model. Owner: Rusty (implementation), Herb (verification that all existing `Assertion` instances correctly classify origination).

### Enhancement 2 — Add a Narrative Quality Gate to the Pre-Release Annotation Audit Plan

The pre-release annotation audit plan (`pre-release-annotation-audit.md`) currently defines scope as: verify citation presence, verify correct control family, verify module-level compliance sections. This is necessary but not sufficient.

The FedRAMP standard for control implementation statements is narrative quality: who implements the control, by what mechanism, for which assets, with what failure behavior. A citation alone is a label. A useful annotation is evidence.

I recommend adding a quality gate to the annotation audit plan that requires each security-critical annotation to answer at minimum:

1. **What** the code does to satisfy the control (not just which control it satisfies)
2. **How** failure is handled (fail-closed? fail-open? returns error?)
3. **Scope** — which assets or data flows does this annotation cover?

Not every annotation needs all three. Simple control citations on types with obvious semantics (e.g., a `FipsEnabled` boolean with `SC-13`) can remain compact. But load-bearing annotations on enforcement-critical functions — the ones an AO would examine in an audit — should meet this standard.

**Proposed addition to audit checklist:**

> For each HIGH-severity enforcement function annotated in Tier 2 (security-critical types and functions), verify that the doc comment:
> - States what the implementation does (not just what control it satisfies)
> - States failure behavior explicitly (fail-closed, error-propagated, fallback behavior)
> - Would allow a third-party auditor unfamiliar with the codebase to understand the implementation without reading the source

This gate cannot be automated. It requires a human read of annotation + code together. That is exactly the kind of work The IRS is for.

---

## One Final Note

Reading the FedRAMP ATO letter structure — where an AO signs their name to a document that lists every unresolved HIGH finding — I was struck by how well-designed that accountability mechanism is. The risk does not vanish. It is acknowledged, named, and signed.

UMRS is still pre-authorization. But when the assessment engine is complete and an `AssessmentBundle` is produced for the first time, whoever reviews that bundle is implicitly acting in an AO-adjacent role — accepting the residual risk of what is not yet implemented. The architecture should make that role visible and the residual risk legible.

That is the spirit of SP 800-18: the SSP is not a compliance artifact. It is a record of decisions made by accountable humans about acceptable risk. Build the tools to support that, and the paperwork follows naturally.

---

*Filed by Herb / The IRS*
*Accreditation artifacts corpus familiarization complete — 2026-03-23*

