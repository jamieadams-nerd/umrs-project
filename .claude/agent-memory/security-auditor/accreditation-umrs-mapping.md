---
name: accreditation UMRS mapping
description: How FedRAMP/SP 800-18 accreditation concepts map to UMRS audit work — UMRS as examination object, control implementation narratives, finding documentation
type: reference
---

# Accreditation Corpus — UMRS Audit Work Mapping

## How UMRS Audit Reports Map to the Accreditation Framework

The UMRS security-auditor produces audit reports analogous to specific sections of a SAR.
Understanding this mapping improves how findings are framed and prioritized.

| UMRS audit activity | Accreditation analog |
|---|---|
| Source code annotation review | SP 800-53A "Examine" assessment — code is a system design document object |
| Finding (HIGH severity) | SAR Risk Exposure Table entry; requires remediation before authorization |
| Finding (MEDIUM severity) | SAR finding; goes into POA&M with risk-adjusted timeline |
| Finding (LOW severity) | POA&M entry; lower priority remediation |
| Remediation recommendation | POA&M corrective action entry |
| Audit report | SAR component for code-level review |

## UMRS Code as an Examine Object

Per SP 800-53A, "Examine" is one of three assessment methods (Examine / Interview / Test).
Code review is a valid Examine activity. For UMRS, the Examine objects include:

- Source code module-level doc comments (evidence of control selection and implementation intent)
- Public type and function annotations (evidence of security property claims)
- Inline security claims (evidence of implementation decisions)
- Architecture documentation (evidence of system boundary and data flows)

A missing annotation on a security-critical function is an "Other Than Satisfied" condition
for SA-11 (Developer Security Testing and Evaluation) because the developer has not documented
the security-relevant behavior.

## Control Implementation Statements in Code

FedRAMP requires control implementation statements to answer: what, how, who.
UMRS doc comments serve the same purpose at the code level:

**Good annotation pattern** (analogous to FedRAMP "good response"):
```
/// Reads the SELinux security context from the kernel's xattr interface.
///
/// Access is fd-anchored to prevent TOCTOU races between path resolution
/// and attribute reading.
///
/// NIST SP 800-53 SI-7 (Information and System Integrity — Software and
/// Information Integrity); NSA RTB TOCTOU (Time-of-Check, Time-of-Use).
```

**Bad annotation pattern** (analogous to FedRAMP "bad response"):
- Simply names the control without explaining what the code does
- References another function "see foo() for security rationale"
- Uses marketing language ("high-assurance, enterprise-grade xattr reader")

## SSP Section 10 Parallel: Cryptographic Modules

SSP Section 10 requires documentation of all cryptographic modules for Data At Rest (DAR)
and Data In Transit (DIT). UMRS Appendix Q (Cryptographic Modules Table) is the corresponding
artifact.

**Audit implication**: When reviewing UMRS code, SC-13 citations without FIPS validation
evidence are analogous to an SSP claiming "FIPS 140 validated" without naming the validated
module. This is a HIGH finding when the code runs in FIPS mode.

## POA&M Lifecycle for UMRS Findings

UMRS audit findings feed a remediation cycle analogous to a POA&M:

1. Audit finding (this report) = POA&M entry creation
2. Remediation owner assigned (coder or tech-writer)
3. Coder/tech-writer implements fix
4. Follow-up audit confirms closure
5. Finding marked resolved in cross-team notes and CHANGELOG

**Timeline expectations** (adapted from FedRAMP standards):
- HIGH findings: 30 days for critical items; no authorization recommendation until resolved
- MEDIUM findings: 90 days typical
- LOW findings: 180 days or next scheduled audit cycle

## Authorization Boundary Concept Applied to UMRS

SP 800-18 defines a system boundary as the set of resources under the same direct management
control. For UMRS audit purposes, the "authorization boundary" maps to:

- In-scope crates: `umrs-selinux`, `umrs-platform`, `umrs-core`, `umrs-ls`, `umrs-logspace`
- Out of scope (prototype parking lot): `components/rust-prototypes/` — do not audit unless
  explicitly requested

When a finding involves a crate dependency, the boundary matters: if the dependency is a
workspace crate within the boundary, it is auditable. If it is an external crate (crates.io),
it is an inherited control that must be verified via supply chain review (CM-8, SA-12).

## Common Controls Concept Applied to UMRS

SP 800-18 identifies "common security controls" that apply at the agency level (not system-
specific). For UMRS, the workspace-level common controls include:

- `#![forbid(unsafe_code)]` in every crate root — common control for SI-3/PW.4
- Workspace-level Clippy configuration — common control for SA-11
- `cargo xtask test` — common control for SA-11 (developer testing)

These common controls can be cited by reference in individual crate SSP-equivalent
documentation rather than repeated in every module.

## SSP Section 8 Parallel: Architecture and Data Flows

SSP Section 8 (Illustrated Architecture and Narratives) requires ABD and DFD.
For UMRS, the analogous artifacts are:

- Architecture docs in `docs/modules/architecture/` — authorization boundary
- Data flow: how CUI/MLS-labeled data flows through the SELinux label reading pipeline
- Security context parsing pipeline (context.rs) is a trust boundary — data enters as
  untrusted kernel string, exits as validated `SecurityContext` type

**Audit implication**: Module-level doc comments on trust boundary modules (context.rs,
xattrs.rs, kattrs/) must explicitly document the trust boundary and cite the relevant
controls. These are analogous to the DFD narrative in the SSP.

## Key FedRAMP Principle: Do Not Modify Template Content

FedRAMP prohibits CSPs from altering control requirement text in SSP Appendix A.
The analogous UMRS rule from Citation Format Rule:

> NIST SP 800-53 citations must use canonical form. Abbreviated or modified citations
> are non-compliant.

UMRS audit finding analog: A citation that uses "NIST 800-53 AC-4" instead of
"NIST SP 800-53 AC-4" is a MEDIUM finding (incorrect citation format).

## FedRAMP 20x Transition — Audit Relevance

The FedRAMP Rev5 process is expected to cease at end of FY27. FedRAMP 20x is the next
generation. Key implication for UMRS:

- Current audit work should be grounded in Rev5 / NIST SP 800-53 Rev 5 baseline
- Watch for 20x developments that may change control assessment procedures
- Control citations in UMRS code should use the most current control identifiers
  (SP 800-53 Rev 5, not Rev 4)
