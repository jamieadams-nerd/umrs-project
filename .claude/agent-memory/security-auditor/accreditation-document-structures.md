---
name: accreditation document structures
description: Required sections and appendices for SSP, SAP, SAR, and POA&M; document quality criteria; control implementation statement guidance
type: reference
---

# Accreditation Document Structures

## Authorization Package Inventory

A complete FedRAMP authorization package contains:

1. System Security Plan (SSP) and appendices A-Q (as necessary)
2. Security Assessment Plan (SAP) and appendices A-D (as necessary)
3. Security Assessment Report (SAR) and appendices A-F (as necessary)
4. Plan of Action and Milestones (POA&M) — SSP Appendix O
5. Signed federal agency Authority to Operate (ATO) letter — agency authorization path

CSPs and 3PAOs are **prohibited from altering or removing content** in the SSP, SAP, and SAR
templates. Only the italicized instructional text must be removed before submission.

## SSP Required Sections (Rev 5 Template)

Sections 1-12 are required for ALL baselines (LI-SaaS, Low, Moderate, High):

| Section | Content |
|---|---|
| 1 | Introduction — SSP is the "security blueprint" |
| 2 | Purpose |
| 3 | System Information (FIPS 199 level, service model, deployment model) |
| 4 | System Owner |
| 5 | Assignment of Security Responsibility |
| 6 | Leveraged FedRAMP-Authorized Services |
| 7 | External Systems and Services Not Having FedRAMP Authorization |
| 8 | Illustrated Architecture and Narratives (includes ABD and DFD) |
| 9 | Services, Ports, and Protocols |
| 10 | Cryptographic Modules for DAR and DIT |
| 11 | Separation of Duties |
| 12 | SSP Appendices List |

**Section 8 is critical**: Authorization boundary diagram (ABD), data flow diagrams (DFD),
and network diagrams must provide enough detail for AO to trace federal data flows.

## SSP Required Appendices

| Appendix | Content | FedRAMP template required? |
|---|---|---|
| A | FedRAMP Security Controls (baseline-specific) | Yes |
| B | Related Acronyms | No |
| C | Security Policies and Procedures | No |
| D | User Guide | No |
| E | Digital Identity Worksheet | No |
| F | Rules of Behavior (RoB) | Yes |
| G | Information System Contingency Plan (ISCP) | Yes |
| H | Configuration Management Plan (CMP) | No |
| I | Incident Response Plan (IRP) | No |
| J | CIS and CRM Workbook | Yes |
| K | FIPS 199 Categorization Worksheet | No |
| L | CSO-Specific Laws and Regulations | No |
| M | Integrated Inventory Workbook (IIW) | Yes |
| N | Continuous Monitoring Plan | No |
| O | POA&M | Yes |
| P | Supply Chain Risk Management Plan (SCRMP) | No |
| Q | Cryptographic Modules Table | Yes |

## SAP Required Sections

| Section | Content |
|---|---|
| 1 | Introduction (About This Document, Who Should Use) |
| 2 | Background (Purpose, Applicable Laws/Regulations/Standards) |
| 3 | Scope (assessment boundary, IP addresses, role testing exclusions) |
| 4 | Assumptions |
| 5 | Methodology (Control Testing, Data Gathering, Sampling, Penetration Test) |
| 6 | Test Plan (Security Assessment Team, POCs, Automated Tools, Manual Methods, Schedule) |
| 7 | Rules of Engagement (Disclosures, End of Testing, Communication, Limitation of Liability) |
| 8 | Signatures |

SAP Appendices: A=Security Controls Selection Worksheet (required template), B=Sampling
Methodology, C=Penetration Testing Plan, D=Significant Change Request Documentation

**SAP timing rule**: If using "Just-In-Time" approach, agency should approve SAP before
3PAO initiates testing. System must be frozen (no changes) during testing.

## SAR Required Sections

| Section | Content |
|---|---|
| 1 | Introduction |
| 2 | Executive Summary (risk summary, strengths/weaknesses, boundary alignment with NIST SP 800-145) |
| 2.1 | Purpose |
| 2.2 | Applicable Laws, Regulations, Standards |
| 2.3 | Scope (Controls Assessed) |
| 3 | System Overview / System Description |
| 4 | Assessment Methodology (Deviations from SAP, SRTM, Consideration of Threats, Document Results) |
| 5 | Risks Known for Interconnected Systems and External Services |

SAR Appendices: A=Risk Exposure Table (RET), B=SRTM Workbook (required template),
C=Vulnerability Scan Results, D=Documentation Review Findings, E=Auxiliary Documents,
F=Penetration Test Report

**SAR key rule**: For initial assessments and uplifts from Moderate to High, all High risks
must be remediated prior to IA recommending authorization. High risks must be remediated
within 30 days in general.

**Risk downgrade rule**: A High risk may be downgraded to Moderate only if IA submits a
Deviation Request (DR) with Risk Adjustment (RA) in the Risk Exposure Table. High
Operational Requirements (ORs) must also be risk-adjusted.

## Control Documentation Structure in SSP Appendix A

Each control in SSP Appendix A has three required sections:

**1. Control Requirement** (from NIST SP 800-53; not editable)
- "The information system..." = technical capability
- "The organization..." = process/procedure
- Parameters: FedRAMP-defined (cannot be changed) or organization-defined (CSP fills in)

**2. Control Summary Information** (table)
- Responsible Role (job title, NOT individual names)
- Parameter values
- Implementation Status: Implemented / Partially Implemented / Planned / Alternative Implementation / Not Applicable
- Control Origination: System Specific / Hybrid / Inherited / Common (if inherited, name the IaaS/PaaS and its FedRAMP ID)

**3. Control Implementation Statement** (written narrative)
The narrative must answer: What is the solution? How is it implemented? Who is responsible?
- Address EVERY sub-part of the control separately
- Be specific: who, what, where, when, why, how — with technical detail
- State explicitly whether the requirement is satisfied
- Do NOT simply restate the control requirement
- Do NOT reference other controls in place of a narrative
- Do NOT copy-paste narratives across controls
- Do NOT include marketing language

**"Other Than Satisfied"** is triggered whenever any portion of a control is "Planned" or
"Partially Implemented."

## Document Quality Criteria (FedRAMP Acceptance Standards)

A submission-ready document must meet four criteria:

| Criterion | Key elements |
|---|---|
| Clarity | Logical presentation; current dates; no ambiguous statements; correct grammar |
| Completeness | All FedRAMP template sections present; all appendices included; no missing content fields |
| Conciseness | Content relevant to audience; no superfluous words |
| Consistency | Same terms throughout; material does not contradict predecessor documents; figure content agrees with text |

## Common Authorization Barriers (FedRAMP-Identified)

- Inaccurate authorization boundary definition or data flow diagrams
- Gaps in vulnerability scanning
- End-of-life software
- Missing FIPS 140 validated encryption modules
- External services holding federal data that are not FedRAMP Authorized at the appropriate level
- MFA not implemented correctly
- Poor documentation with insufficient technical detail
- Immature security practices / not building security in early

## CSP and 3PAO Roles Summary

| Document | CSP responsibility | 3PAO responsibility |
|---|---|---|
| SSP | Develop using FedRAMP templates; validate if using 3PAO advisor | Advisor: help develop; Assessor: validate completeness/accuracy |
| SAP | Review and approve; sign | Define scope/methodology; deliver using FedRAMP templates; sign |
| SAR | Provide evidence artifacts; work with 3PAO to identify risks | Perform assessment; draft SAR; deliver to CSP |
| POA&M | Create, maintain, implement monthly ConMon; track risks | Validate POA&M detail annually |

**Independence rule**: 3PAOs contracted as advisors cannot provide assessment services for
the same CSO for two years (A2LA R311).
