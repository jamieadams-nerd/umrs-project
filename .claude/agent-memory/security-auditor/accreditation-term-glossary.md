---
name: accreditation term glossary
description: Canonical definitions of accreditation and authorization terms from SP 800-18 and FedRAMP playbooks
type: reference
---

# Accreditation Term Glossary

## Core Authorization Terms

**ATO (Authority to Operate)**
Formal decision by an AO that a system may operate at an acceptable level of risk.
Each federal agency must issue its own ATO — the initial authorizing agency's ATO is NOT
a government-wide risk acceptance.

**ATU (Authority to Use)**
Alternative to ATO that some agencies issue for leveraging a FedRAMP-authorized CSO without
performing a full independent authorization.

**Authorization Package**
Complete set of security documentation submitted for AO review: SSP + appendices, SAP,
SAR, POA&M, and ATO letter.

**Authorization Boundary**
The defined set of all system services, components, and devices that will be included in
and authorized as part of the system. The foundation of the SSP.

**Authorizing Official (AO)**
Senior federal official with authority to formally assume responsibility for operating
an information system at an acceptable risk level. Issues the ATO letter.

**CSO (Cloud Service Offering)**
The specific cloud service product being authorized. Distinct from the CSP (provider).

**CSP (Cloud Service Provider)**
The organization offering the cloud service seeking FedRAMP authorization.

## Assessment Terms

**3PAO (Third Party Assessment Organization)**
FedRAMP-recognized independent organization that performs security assessments. Must be
independent from the CSP (A2LA R311 two-year advisory/assessment separation rule).

**IAO (Independent Assessment Organization)**
A non-3PAO assessor acceptable for FedRAMP agency authorizations when directed by the agency
AO, who must attest to independence.

**RAR (Readiness Assessment Report)**
Documents a CSP's capability to meet federal security requirements. Produced by a 3PAO
during the optional readiness assessment phase. Basis for "FedRAMP Ready" designation.

**SAP (Security Assessment Plan)**
Developed by the 3PAO. Describes scope, methodology, test plan, and rules of engagement.
Both CSP and 3PAO must sign. Must be approved before testing begins.

**SAR (Security Assessment Report)**
Documents the results of the security assessment. Produced by the 3PAO. Describes risks
remaining at the conclusion of the assessment. Basis for AO's risk-based authorization decision.

**SRTM (Security Requirements Traceability Matrix)**
Maps each security control to its test procedure and result. SAR Appendix B (required
FedRAMP template). The primary evidence record for control-by-control assessment results.

**RET (Risk Exposure Table)**
SAR Appendix A. Enumerates all findings with risk levels, CVSSv3 scores, and remediation
status. Used by AO to make authorization decision.

## Planning Terms

**SSP (System Security Plan)**
The "security blueprint" of a system. Defines the authorization boundary, describes
security controls in place, documents roles and responsibilities. Required by FISMA/OMB A-130.
SP 800-18 is the authoritative guide for SSP development.

**POA&M (Plan of Action and Milestones)**
Tracks security weaknesses found during assessments and the planned remediation actions
with milestones. SSP Appendix O. Updated monthly during ConMon.

**ConMon (Continuous Monitoring)**
Ongoing security oversight after authorization. CSPs submit monthly deliverables to
agencies. Includes vulnerability scans, deviation requests, significant change requests.

**Deviation Request (DR)**
Formal request for agency approval of a risk adjustment (RA), operational requirement (OR),
or false positive. Required when a High risk is proposed for downgrade to Moderate.

**WBS (Work Breakdown Structure)**
Authorization project plan submitted with the In-Process Request (IPR) to FedRAMP.

**IPR (In-Process Request)**
Submitted to FedRAMP to list a CSO as "FedRAMP In Process" on the Marketplace.

## System Classification Terms (SP 800-18)

**Major Application**
Requires special management oversight due to criticality or risk. Expected FIPS 199
impact level of Moderate or High. Defined by OMB Circular A-130.

**General Support System (GSS)**
Interconnected set of information resources under same direct management control sharing
common functionality. Can host major applications. Has its own SSP.

**Minor Application**
Security controls provided by the GSS or major application it runs on. No separate SSP
required unless not connected to a major system. FIPS 199 Low or Moderate.

**Subsystem**
A major subdivision of an information system. Falls under the same SSP as the parent system
but may have its own subsystem boundary.

## Control Terms

**Common Control**
A security control that is implemented at the organizational level and shared by multiple
systems. Incorporated into system SSPs by reference.

**Compensating Control**
An alternative control used when the baseline control cannot be applied. Must be documented
in the SSP with rationale.

**Control Origination**
Field in SSP Appendix A indicating who is responsible for implementing the control:
System Specific, Hybrid, Inherited (from FedRAMP-authorized IaaS/PaaS), or Common.

**ODP (Organization-Defined Parameter)**
A value that the CSP or FedRAMP must specify to complete a control requirement.
FedRAMP-assigned ODPs cannot be changed by the CSP.

**"Other Than Satisfied"**
Assessment conclusion applied when any portion of a control is Planned or Partially
Implemented. Triggers a finding in the SAR and a POA&M entry.

## Roles (SP 800-18 Canonical)

**CIO**: Agency-wide information security program; designates SAISO.
**SAISO**: CIO's primary liaison; coordinates SSP development/review/acceptance.
**Information System Owner**: Develops/maintains SSP; accountable for system security posture.
**Information Owner**: Statutory authority over specific data; establishes rules of behavior.
**ISSO**: Assigned by SAISO/AO; maintains operational security posture; actively updates SSP.
**AO**: Senior official; approves SSP; grants/denies/revokes ATO; independent from system owner.
