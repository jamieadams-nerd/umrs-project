---
name: accreditation concept index
description: Key concepts from SP 800-18 and FedRAMP playbooks — authorization lifecycle, SSP structure, roles, impact levels, document relationships
type: reference
---

# Accreditation Concept Index

## Authorization Lifecycle (Three Phases)

The FedRAMP agency authorization process has three phases, identical in both playbooks but
described from the CSP and Agency perspectives respectively.

**Phase 1 — Preparation**
- Optional (but highly recommended): Readiness Assessment → Readiness Assessment Report (RAR)
  → FedRAMP review → "FedRAMP Ready" Marketplace designation
- Mandatory: Partnership Establishment, Authorization Planning (select 3PAO, confirm impact level),
  Work Breakdown Structure (WBS), In-Process Request (IPR), Kickoff Meeting
- Kickoff outcome: shared understanding of boundary, data flows, gaps, remediation plan, timeline

**Phase 2 — Authorization**
- Full Security Assessment: 3PAO develops SAP and tests against SSP; system frozen during testing
- Authorization Package: SSP + appendices A-Q, SAP + appendices A-D, SAR + appendices A-F, POA&M
- SAR Debrief: 3PAO presents findings; CSP presents remediation plan
- Agency Authorization Process: Agency review → SAR Debrief → Remediation → Agency Final Review
  → Agency Issues ATO letter → FedRAMP Review → FedRAMP Marketplace Designation: Authorized
- All High risks must be remediated before initial authorization recommendation

**Phase 3 — Continuous Monitoring (ConMon)**
- Monthly ConMon deliverables: updated POA&M, vulnerability scan files and reports, deviation
  requests, significant change requests, incident reporting
- Annual assessment by 3PAO
- Each agency that uses a FedRAMP Authorized CSO must issue its own ATO (not government-wide)

## FedRAMP Designations (Three Only)

Only three recognized designations exist. "FedRAMP Compliant" and "FedRAMP Equivalent" are
NOT recognized designations.

| Designation | Meaning |
|---|---|
| FedRAMP Ready | 3PAO attests to security capabilities; RAR reviewed and accepted by FedRAMP |
| FedRAMP In Process | Actively working toward authorization with a federal agency partner |
| FedRAMP Authorized | Successfully completed FedRAMP authorization; available for government-wide reuse |

## Impact Levels (FIPS 199 Basis)

| Level | Adverse effect | Typical systems |
|---|---|---|
| Low (LI-SaaS) | Limited — minimal PII beyond login data | Low-impact SaaS; smaller control set |
| Low | Limited adverse effects | Basic cloud services |
| Moderate | Serious adverse effects, not loss of life | ~80% of FedRAMP-authorized CSOs |
| High | Severe or catastrophic — could involve life/financial ruin | Law enforcement, emergency services, financial systems |

**Key rule**: The security impact level is ultimately determined by the federal agency AO,
not the CSP. CSPs must coordinate with agency customers to confirm categorization.

**FIPS 199 categorization template** is Appendix K of the SSP template. NIST SP 800-60
volume 2 revision 1 is the companion guide for categorizing data types.

## Security Objectives (CIA Triad — FIPS PUB 199)

| Objective | Definition |
|---|---|
| Confidentiality | Access and disclosure controls protect privacy and proprietary information |
| Integrity | Stored information is sufficiently guarded against modification |
| Availability | Timely and reliable access to information is ensured |

## System Boundary Concepts (SP 800-18)

**Authorization Boundary**: Defines what exactly is being authorized. Must be validated against
inventory during 3PAO assessment. Foundation on which the SSP is built.

**Authorization Boundary Diagram (ABD)**: Visual representation of all system services,
components, and devices. Must also show external systems (even those outside the boundary)
so AOs understand risk-acceptance areas.

**Data Flow Diagram (DFD)**: Depicts how federal data and sensitive system data flows internal
and external to the CSO. Must be developed before documenting security controls.

**System types (SP 800-18)**:
- Major Application: FIPS 199 Moderate or High; requires special management oversight
- General Support System (GSS): Interconnected resources under same management control sharing
  common functionality (LAN, backbone, data center, tactical radio, etc.)
- Minor Application: Security controls typically provided by the GSS; no separate SSP required
  unless not connected to a GSS

**High-watermark rule**: When grouping subsystems with varying FIPS 199 levels into a single
GSS, the highest impact level must be applied to the whole system unless adequate boundary
protection (firewalls, encryption) isolates the high-impact components.

## Roles (SP 800-18, Section 1.7)

| Role | Core responsibility |
|---|---|
| Chief Information Officer (CIO) | Agency-wide information security program; designates SAISO |
| Senior Agency Information Security Officer (SAISO) | CIO's liaison to system owners and ISSOs; coordinates SSP development/review/acceptance |
| Information System Owner | Develops and maintains the SSP; responsible for system security posture |
| Information Owner | Establishes rules of behavior; decides access privileges; has authority over specific data |
| Information System Security Officer (ISSO) | Assigned by SAISO/AO/ISO; maintains operational security posture; actively develops and updates the SSP |
| Authorizing Official (AO) | Senior management; formally authorizes operation; approves SSP; can deny or halt operations |

**Note**: AO is independent from the system owner. The AO approves the SSP prior to certification
and accreditation (C&A), not the system owner.

## SP 800-18 Security Plan Inputs/Outputs

Inputs to the Security Plan:
- FIPS 199, FIPS 200, SP 800-53 (control selection)
- Risk Assessment
- Ongoing Monitoring
- POA&Ms
- Configuration Management

Outputs from the Security Plan:
- C&A Process (now RMF)
- Feeds back into POA&Ms and Ongoing Monitoring
