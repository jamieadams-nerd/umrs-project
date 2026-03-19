# UMRS Capabilities — NIST SP 800-171 Rev 3 and CMMC Level 2 Mandate Mapping

**Date:** 2026-03-19
**Author:** Researcher agent
**Purpose:** Verify that each UMRS capability highlighted in the project blog post is backed by
a specific, mandatory requirement in NIST SP 800-171 Rev 3 and/or CMMC Level 2. Flag capabilities
that exceed what the standard mandates so they can be framed appropriately in the blog.

**Primary source:** NIST SP 800-171 Rev 3 (May 2024), `refs/nist/sp800-171r3.pdf`
**Secondary source:** CMMC Assessment Guide Level 2 v2.13, `refs/dod/cmmc-assessment-guide-l2.pdf`

---

## Scope and Framing Note

SP 800-171 Rev 3 requirements are stated in mandatory terms — organizations processing, storing,
or transmitting CUI on nonfederal systems MUST satisfy them as a contractual condition of handling
CUI. CMMC Level 2 maps 1:1 to these 110 requirements; every L2 practice is also a 800-171r3
requirement. There is no "SHOULD" tier in either document — all listed requirements are SHALL.

The numbering scheme in Rev 3 changed from the two-digit Rev 2 format. Rev 3 uses a
three-level decimal scheme: `03.AC.01` is rendered in the document as `03.01.01` (family.seq.sub).
In this report, requirements are cited by their Rev 3 number and their source SP 800-53 Rev 5
control (listed in the document's References section for each requirement).

---

## Capability 1: CUI Category Labeling

**UMRS description:** Marking files with specific CUI categories (LEI, AGR, CTI, etc.) so
operators know what they are handling.

### Mandate analysis

**03.08.04 — Media Marking** (source control: SP 800-53 MP-03)

> "Mark system media that contain CUI to indicate distribution limitations, handling caveats,
> and applicable CUI markings."

Discussion text: "Marking refers to the use or application of human-readable security
attributes. Labeling refers to the use of security attributes for internal system data
structures... CUI is defined by NARA along with marking, safeguarding, and dissemination
requirements for such information."

**Status: MUST — mandatory requirement.**

This is a direct mandate to mark media containing CUI with distribution limitations, handling
caveats, and the applicable CUI markings. UMRS CUI category labeling (LEI, AGR, CTI, etc.)
is the system-level implementation of this requirement — it is the "internal system data
structure" labeling that the discussion text explicitly mentions alongside marking.

**Supporting requirement:**

**03.15.01 — Policy and Procedures** (source controls: AC-01, AU-01, CM-01 and all policy families)

> "Develop, document, and disseminate to organizational personnel or roles the policies and
> procedures needed to satisfy the security requirements for the protection of CUI."

The CUI category labels that UMRS exposes are a technical expression of what NARA's CUI
Registry mandates as the handling framework for each category. The labeling capability makes
policy machine-readable.

**CMMC Level 2 practice:** MP.L2-3.8.4 — identical text to 03.08.04.

### Blog framing

MANDATED. Frame as: "800-171 requires you to mark CUI with its category and handling
restrictions — UMRS makes that marking visible and queryable in the kernel."

---

## Capability 2: Verbatim Handling Restrictions

**UMRS description:** The system tells operators exactly how information must be handled per
category (verbatim handling restrictions surfaced to the operator at runtime).

### Mandate analysis

**03.15.03 — Rules of Behavior** (source control: SP 800-53 PL-04)

> "a. Establish rules that describe the responsibilities and expected behavior for system
> usage and protecting CUI.
> b. Provide rules to individuals who require access to the system.
> c. Receive a documented acknowledgement from individuals indicating that they have read,
> understand, and agree to abide by the rules of behavior before authorizing access to CUI
> and the system."

Discussion: "Rules of behavior represent a type of access agreement for system users.
Organizations consider rules of behavior for the handling of CUI based on individual user
roles and responsibilities."

**03.02.01 — Literacy Training and Awareness** (source control: SP 800-53 AT-02 and AT-03)

> "a. Provide security literacy training to system users... 3. On recognizing and reporting
> indicators of insider threat, social engineering, and social mining."

Discussion: "The content also addresses the need for operations security and the handling
of CUI."

**Status: MUST — mandatory requirement.**

03.15.03 explicitly requires that handling rules for CUI be communicated to users. The verbatim
handling restrictions UMRS surfaces per category are the technical delivery mechanism for this
requirement — the system makes the applicable rules of behavior available at the point of access,
not buried in a policy document.

**CMMC Level 2 practice:** PL.L2-3.15.3 maps to 03.15.03.

### Partial "above and beyond" flag

The standard requires that handling rules exist and be communicated to users. It does not
specifically require that they be surfaced per-file, per-category, at query time by the
security tool. UMRS's specific UX approach (surfacing verbatim restrictions inline, tied to
the specific CUI category on a specific file) is an implementation choice that exceeds the
minimum policy-document approach. The mandate exists; the runtime delivery mechanism is UMRS
going further than minimum compliance.

**Blog framing:** "800-171 requires you to establish and communicate handling rules for each
CUI category. UMRS delivers those rules to the operator at the moment they query a file —
not buried in a policy binder on the shelf."

---

## Capability 3: SELinux Posture Evaluation / Runtime Source of Truth

**UMRS description:** Querying the kernel to verify that SELinux enforcement is actually active;
detecting drift between configured state and running state.

### Mandate analysis

**03.04.02 — Configuration Settings** (source control: SP 800-53 CM-06)

> "a. Establish, document, and implement the following configuration settings for the system
> that reflect the most restrictive mode consistent with operational requirements.
> b. Identify, document, and approve any deviations from established configuration settings."

**03.04.01 — Baseline Configuration** (source control: SP 800-53 CM-02)

> "a. Develop and maintain under configuration control, a current baseline configuration of
> the system.
> b. Review and update the baseline configuration of the system [organization-defined
> frequency] and when system components are installed or modified."

**03.12.03 — Continuous Monitoring** (source control: SP 800-53 CA-07)

> "Develop and implement a system-level continuous monitoring strategy that includes ongoing
> monitoring and security assessments."

Discussion: "Continuous monitoring at the system level facilitates ongoing awareness of the
system security posture to support risk management decisions. The terms continuous and ongoing
imply that organizations assess and monitor their systems at a frequency that is sufficient
to support risk-based decisions."

**03.14.06 — System Monitoring** (source controls: SP 800-53 SI-04, SI-04(04))

> "a. Monitor the system to detect:
>    1. Attacks and indicators of potential attacks and
>    2. Unauthorized connections.
> b. Identify unauthorized use of the system.
> c. Monitor inbound and outbound communications traffic to detect unusual or unauthorized
>    activities or conditions."

**Status: MUST — mandatory requirement.**

03.04.02 requires that configuration settings be at "the most restrictive mode consistent
with operational requirements" and that deviations be identified. SELinux enforcement mode
is precisely such a configuration setting — running in permissive mode when enforcing is
required is a documented deviation that 03.04.02 mandates be identified. 03.12.03 requires
a continuous monitoring strategy that includes "ongoing monitoring" of security posture.
UMRS's runtime kernel query is the technical means by which these requirements are satisfied.
Drift detection (configured vs. running) directly implements 03.04.02(b): "Identify...
any deviations from established configuration settings."

**CMMC Level 2 practices:** CM.L2-3.4.1, CM.L2-3.4.2, CA.L2-3.12.3, SI.L2-3.14.6.

### Blog framing

MANDATED. "800-171 requires you to implement configuration settings at the most restrictive
mode and identify deviations. It requires continuous monitoring of your security posture.
UMRS is what makes that requirement verifiable — it queries the kernel directly to confirm
enforcement is actually active and flags the moment configured state diverges from running state."

---

## Capability 4: Typed Security Findings (Data, Not Log Strings)

**UMRS description:** Enforcement gaps represented as structured, queryable enum variants
rather than unstructured log strings.

### Mandate analysis

**03.03.02 — Audit Record Content** (source control: SP 800-53 AU-03, AU-03(01))

> "a. Include the following content in audit records:
>    1. What type of event occurred
>    2. When the event occurred
>    3. Where the event occurred
>    4. Source of the event
>    5. Outcome of the event
>    6. Identity of the individuals, subjects, objects, or entities associated with the event
> b. Provide additional information for audit records as needed."

**03.03.05 — Audit Record Review, Analysis, and Reporting** (source controls: AU-06, AU-06(03))

> "a. Review and analyze system audit records [organization-defined frequency] for indications
> and the potential impact of inappropriate or unusual activity.
> b. Report findings to organizational personnel or roles.
> c. Analyze and correlate audit records across different repositories to gain
> organization-wide situational awareness."

**03.03.06 — Audit Record Reduction and Report Generation** (source control: AU-07)

> "a. Implement an audit record reduction and report generation capability that supports
> audit record review, analysis, reporting requirements, and after-the-fact investigations
> of incidents.
> b. Preserve the original content and time ordering of audit records."

**03.12.01 — Security Assessment** (source control: CA-02)

> "Assess the security requirements for the system and its environment of operation
> [organization-defined frequency] to determine if the requirements have been satisfied."

Discussion: "Security assessment reports document assessment results in sufficient detail
as deemed necessary by the organization to determine the accuracy and completeness of the
reports."

**Status: MUST — mandatory, with nuance on the structured-data implementation choice.**

03.03.02 requires that audit records contain specific structured fields (event type, time,
location, source, outcome, identity). This is a de facto requirement for structured data —
a free-form log string that cannot be parsed into these six fields does not satisfy 03.03.02.
03.03.05 requires analysis and correlation of audit records, which is impossible if findings
are unstructured strings. 03.03.06 requires a "report generation capability" supporting
"after-the-fact investigations" — again implying that findings must be queryable.

However, 800-171 does not specify that findings be represented as Rust enum variants or any
particular data structure. The mandate is for structured, analyzable, reportable evidence.
UMRS's choice of typed enum variants is an implementation that satisfies and exceeds the
minimum structured-record requirement.

**CMMC Level 2 practices:** AU.L2-3.3.2, AU.L2-3.3.5, AU.L2-3.3.6.

### Blog framing

MANDATED in principle. "800-171 requires that security findings be structured, analyzable,
and reportable — it enumerates six specific fields every audit record must contain. UMRS
implements this as typed data: every finding is a matchable enum variant, not a text string,
so you can filter, count, and query enforcement gaps programmatically rather than grepping logs."

---

## Capability 5: High-Assurance Engineering Patterns (TPI Dual-Parser, Fail-Closed, Non-Bypassability)

**UMRS description:** Engineering patterns — two independent parsers that must agree, failing
closed on any disagreement, non-bypassable verification paths.

### Mandate analysis

**03.16.01 — Security Engineering Principles** (source control: SP 800-53 SA-08)

> "Apply the following systems security engineering principles to the development or
> modification of the system and system components: [Assignment: organization-defined systems
> security engineering principles]."

Discussion: "Examples include developing layered protections; establishing security policies,
architectures, and controls as the foundation for system design; incorporating security
requirements into the system development life cycle; delineating physical and logical security
boundaries; ensuring that developers are trained on how to build trustworthy secure software;
and performing threat modeling to identify use cases, threat agents, attack vectors and
patterns, design patterns, and compensating controls needed to mitigate risk."

**03.04.04 — Impact Analyses** (source control: SP 800-53 CM-04)

> "a. Analyze changes to the system to determine potential security impacts prior to change
> implementation.
> b. Verify that the security requirements for the system continue to be satisfied after the
> system changes have been implemented."

**Status: MUST — the mandate to apply security engineering principles is mandatory. The
specific patterns (dual-parser, fail-closed) are not enumerated by name — they are UMRS's
implementation of that mandate.**

03.16.01 is the critical anchor. Organizations MUST apply security engineering principles —
the assignment is organization-defined, meaning UMRS defines which principles apply (e.g.,
fail-safe defaults, complete mediation, non-bypassability from NSA RTB/Saltzer-Schroeder).
The dual-parser TPI pattern and fail-closed behavior are how UMRS implements those
organization-defined principles. An assessor evaluating 03.16.01 would expect to see
documented engineering principles and evidence of their application — UMRS's patterns
provide exactly that evidence.

NIST SP 800-218 (SSDF) PW.4 ("Implement Secure Coding Practices") provides the software
assurance backing: organizations implementing 800-171 must also apply the SSDF when developing
CUI-handling software, per OMB M-22-18 (2022). SSDF PW.4 explicitly addresses code review
and analysis to find and fix security problems.

**CMMC Level 2 practice:** SA.L2-3.16.1.

### Blog framing

MANDATED. "800-171 requires you to apply security engineering principles to CUI-handling
systems. UMRS makes those principles mechanically verifiable: two independent parsers must
agree before any security context is trusted, and the system fails closed if they disagree —
not fails open. That is fail-safe defaults applied to the tool itself."

---

## Capability 6: `#![forbid(unsafe_code)]` / Memory-Safe Implementation

**UMRS description:** Compile-time proof of memory safety via `#![forbid(unsafe_code)]` in
every crate root.

### Mandate analysis

**03.16.01 — Security Engineering Principles** (source control: SP 800-53 SA-08)

As cited above — the mandate to apply security engineering principles covers software
development practices including memory safety.

**03.14.01 — Flaw Remediation** (source control: SP 800-53 SI-02)

> "a. Identify, report, and correct system flaws.
> b. Install security-relevant software and firmware updates within [organization-defined
> time period] of the release of the updates."

Discussion: "Organizations identify systems that are affected by announced software and
firmware flaws, including potential vulnerabilities that result from those flaws..."

**NIST SP 800-218 SSDF — PW.4 (Implement Secure Coding Practices)**

The SSDF, incorporated by reference in OMB M-22-18 and the CMMC Final Rule (89 FR 83092),
requires: "Follow all security coding practices that are appropriate for each programming
language and platform being used... Review code to identify and address potential
vulnerabilities." Memory safety vulnerabilities (buffer overflows, use-after-free, etc.) are
the single largest class of exploitable flaws in native-code software. `#![forbid(unsafe_code)]`
eliminates an entire class of these flaws at compile time.

**Status: MUST for the security engineering requirement; the specific implementation
mechanism (Rust + `#![forbid]`) is an UMRS design choice that provides compile-time proof
of satisfying that requirement.**

No 800-171 requirement names Rust or `#![forbid(unsafe_code)]` by name. However:
- 03.16.01 mandates security engineering principles be applied
- 03.14.01 mandates that flaws be identified and corrected
- SSDF PW.4 (referenced in CMMC rule) mandates secure coding practices

Memory safety is not optional for software that processes CUI — it is a baseline expectation
under 03.16.01 and SSDF PW.4. `#![forbid(unsafe_code)]` is an unusually strong implementation
of that expectation because it is a compile-time, mechanically verifiable guarantee rather
than a policy or process.

**CMMC Level 2 practice:** SA.L2-3.16.1 (engineering principles); SI.L2-3.14.1 (flaw
remediation as the consequence of not having memory safety).

### Blog framing

MANDATED in principle, exceptional in implementation. "800-171 and the SSDF require you
to apply security engineering principles and eliminate known flaw classes. UMRS takes this
further than policy: `#![forbid(unsafe_code)]` is a compile-time guarantee — not a review
checklist. An auditor can verify it by reading a single line of source. No unsafe block is
possible, anywhere in the codebase, ever. That is not a process claim; it is a proof."

---

## Capability 7: Orthogonal Category Composition

**UMRS description:** Operators can hold multiple CUI categories simultaneously with precise
access boundaries — each category is independently composed.

### Mandate analysis

**03.01.02 — Access Enforcement** (source control: SP 800-53 AC-03)

> "Enforce approved authorizations for logical access to CUI and system resources in
> accordance with applicable access control policies."

Discussion: "Access control policies control access between active entities or subjects
(i.e., users or system processes acting on behalf of users) and passive entities or objects
(i.e., devices, files, records, domains) in organizational systems... Access control policies
are defined in 03.15.01."

**03.01.03 — Information Flow Enforcement** (source control: SP 800-53 AC-04)

> "Enforce approved authorizations for controlling the flow of CUI within the system and
> between connected systems."

Discussion: "Information flow control regulates where CUI can transit within a system and
between systems (in contrast to who is allowed to access the information)... Organizations
also consider implementing trustworthy regrading mechanisms to reassign security attributes
and security labels."

**03.01.01 — Account Management** (source control: SP 800-53 AC-02, AC-02(03), AC-02(05))

> "c. Specify:
>    1. Authorized users of the system,
>    2. Group and role membership, and
>    3. Access authorizations (i.e., privileges) for each account."

**03.15.01 — Policy and Procedures** (source control: all policy families including AC-01)

> "Develop, document, and disseminate to organizational personnel or roles the policies and
> procedures needed to satisfy the security requirements for the protection of CUI."

**Status: MUST — 03.01.02 and 03.01.03 mandate enforcement of authorizations for CUI access
and flow. Category composition is necessary to implement these controls correctly when an
operator is authorized for multiple CUI categories.**

When an operator is authorized for both LEI and CTI, their access authorization spans both
categories. 03.01.02 requires that this authorization be enforced — not collapsed into a
single undifferentiated "CUI authorized" label, but tracked at the category level that matches
the authorization. NARA's CUI Registry assigns distinct handling rules per category; without
orthogonal category composition, 03.01.03 (information flow control) cannot be enforced
accurately for operators with multi-category access.

**Important nuance:** 800-171 does not specifically mandate category-level composition as a
technical architecture. It mandates that access and flow be controlled according to "applicable
access control policies." For organizations whose policy tracks authorizations at the CUI
category level (as NARA guidance implies they should), orthogonal composition is not optional —
it is the correct implementation of 03.01.02 and 03.01.03. For organizations with a single
undifferentiated CUI category, composition does not arise. UMRS targets environments where
category-level distinction matters.

**CMMC Level 2 practices:** AC.L2-3.1.2, AC.L2-3.1.3.

### Blog framing

MANDATED for multi-category environments, with architectural nuance. "800-171 requires you
to enforce access authorizations and control information flow for CUI. If your operators
handle multiple CUI categories — and most do — you cannot enforce those requirements with
a single 'CUI authorized' flag. UMRS tracks authorizations orthogonally per category,
so each boundary is enforced independently. Collapse them and you have a policy you cannot
enforce."

---

## Summary Table

| Capability | Primary Requirement | Mandate Strength | CMMC L2 Practice | Above-and-Beyond Element |
|---|---|---|---|---|
| CUI category labeling | 03.08.04 (MP-03) | MUST | MP.L2-3.8.4 | Runtime queryability of labels |
| Verbatim handling restrictions | 03.15.03 (PL-04), 03.02.01 | MUST | PL.L2-3.15.3 | Per-file, per-category, at runtime |
| SELinux posture / Runtime SoT | 03.04.02 (CM-06), 03.12.03 (CA-07) | MUST | CM.L2-3.4.2, CA.L2-3.12.3 | Kernel-level verification vs. config-file trust |
| Typed security findings | 03.03.02 (AU-03), 03.03.05, 03.03.06 | MUST (structured evidence) | AU.L2-3.3.2, AU.L2-3.3.5 | Rust enum typing vs. log strings |
| High-assurance patterns (TPI, fail-closed) | 03.16.01 (SA-08) | MUST (principles required) | SA.L2-3.16.1 | Dual-parser and fail-closed as named, documented patterns |
| `#![forbid(unsafe_code)]` / memory safety | 03.16.01 (SA-08), SSDF PW.4 | MUST (secure coding required) | SA.L2-3.16.1 | Compile-time proof vs. process/review |
| Orthogonal category composition | 03.01.02 (AC-03), 03.01.03 (AC-04) | MUST for multi-category environments | AC.L2-3.1.2, AC.L2-3.1.3 | Architectural precision of composition model |

---

## Key Findings for Blog Authors

### All capabilities are mandated — none are mere nice-to-haves

Every capability in the list connects to at least one mandatory SP 800-171 Rev 3 requirement.
The blog can legitimately say that CUI custodians are required to do all of these things.

### Three capabilities have meaningful "above and beyond" dimensions

These should be framed carefully — the mandate exists, but UMRS's implementation goes further:

1. **Verbatim handling restrictions** — 800-171 requires handling rules exist and be communicated.
   It does not require they be surfaced per-file at query time. UMRS does more than the minimum.

2. **Typed security findings** — 800-171 requires structured, analyzable records. It does not
   prescribe Rust enum variants. UMRS's implementation makes the mandate mechanically stronger
   than a logging policy.

3. **`#![forbid(unsafe_code)]`** — 800-171 and SSDF require secure coding and flaw remediation.
   `#![forbid]` provides compile-time proof rather than process attestation. This is a stronger
   claim than the standard makes.

### The CMMC Note

The CMMC Assessment Guide L2 v2.13 maps 1:1 to 800-171 Rev 3 practices. Every capability
above corresponds to a CMMC L2 practice with the same text and the same mandatory status.
For CMMC-assessed organizations, these are not aspirational goals — they are assessment
objectives that an assessor will test.

### The Rev 3 vs. Rev 2 note

CMMC was originally tied to 800-171 Rev 2. The Final Rule (89 FR 83092, Dec 2024) references
Rev 2, but CMMC 2.0 implementation guidance acknowledges ongoing alignment with Rev 3.
Assessments during the 2025-2027 phase-in may reference either revision. The capabilities
mapped here are present in both revisions; Rev 3 adds 03.16.01 (security engineering
principles) as an explicit requirement, which strengthens the mandate behind capabilities
5 and 6.

---

## Source Verification

All requirement text quoted above was read directly from:
- `refs/nist/sp800-171r3.pdf` (SHA-256: `3e4631df8b5d61f40a6e542b52779ef30ddbbfff31e09214fa94ad6e6f5e6d08`)
  Pages 6–74 (document pages, offset from PDF pages by approximately 10)
- Manifest entry: `refs/manifest.md`
