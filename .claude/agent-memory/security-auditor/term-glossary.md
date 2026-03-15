# Term Glossary — rmf-methodology
Generated: 2026-03-15

Terms listed alphabetically. Source priority: SP800-53/SP800-37 > SP800-39 > SP800-30.

---

## Access Control Family (AC)

**Definition:** The family of security controls governing the access rights and privileges of users and processes to system resources, including policy, account management, enforcement, and information flow.
**Source:** SP800-53A Rev. 5, Section 4.1
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** "Access management" (too vague)
**NIST control reference:** AC (family)

---

## Assessment Method

**Definition:** One of three techniques used to conduct a security or privacy control assessment: Examine (review/analysis of documents and mechanisms), Interview (discussion with individuals), or Test (exercise of mechanisms under defined conditions).
**Source:** SP800-53A Rev. 5, Section 2.1
**Normative:** yes
**Synonyms / variants:** Assessment technique
**Deprecated forms:** "audit method" (misleading in this context)
**NIST control reference:** CA-2, CA-7

---

## Assessment Object

**Definition:** The specific item on which an assessment method is applied; for Examine: specifications, mechanisms, or activities; for Interview: individuals or groups; for Test: mechanisms or activities.
**Source:** SP800-53A Rev. 5, Appendix C
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**NIST control reference:** CA-2

---

## Assessment Plan

**Definition:** A formal document developed by control assessors specifying the controls to be assessed, assessment procedures to be used, depth and coverage attributes, schedule, and key milestones; reviewed and approved by the authorizing official before execution.
**Source:** SP800-53A Rev. 5, Section 3.2; SP800-37 Rev. 2, Task A-2
**Normative:** yes
**Synonyms / variants:** Security assessment plan, privacy assessment plan
**Deprecated forms:** none
**NIST control reference:** CA-2

---

## Authorization Boundary

**Definition:** All components of an information system to be authorized for operation by an authorizing official and excludes separately authorized systems, to which the information system is connected.
**Source:** SP800-37 Rev. 2, Chapter 2
**Normative:** yes
**Synonyms / variants:** System boundary
**Deprecated forms:** "Security perimeter" (too physical)
**Usage notes:** Drives scope of all RMF tasks; must be documented in the SSP. May be physical, logical, or virtual.
**NIST control reference:** CA-3, PL-2

---

## Authorization Package

**Definition:** The set of documentation submitted to the authorizing official for an authorization decision; includes the SSP, security assessment report, privacy assessment report, and plan of action and milestones.
**Source:** SP800-37 Rev. 2, Task R-1
**Normative:** yes
**Synonyms / variants:** ATO package
**Deprecated forms:** none
**NIST control reference:** CA-6, CA-7

---

## Authorization to Operate (ATO)

**Definition:** The official management decision given by a senior organizational official to authorize operation of an information system and to explicitly accept the risk to organizational operations and assets, individuals, other organizations, and the Nation based on the implementation of an agreed-upon set of security and privacy controls.
**Source:** SP800-37 Rev. 2, Task R-4
**Normative:** yes
**Synonyms / variants:** Authority to Operate
**Deprecated forms:** "Authorized" (adjective form, acceptable but imprecise)
**NIST control reference:** CA-6

---

## Authorizing Official (AO)

**Definition:** A senior official or executive with the authority to formally assume responsibility for operating an information system at an acceptable level of risk to organizational operations and assets, individuals, other organizations, and the Nation.
**Source:** SP800-37 Rev. 2, Appendix D
**Normative:** yes
**Synonyms / variants:** Designated Approving Authority (DAA) — deprecated DoD term
**Deprecated forms:** DAA
**Usage notes:** The AO cannot delegate the authority to accept risk; only the AO signs the authorization decision.
**NIST control reference:** CA-6

---

## Common Control

**Definition:** A security or privacy control that is inherited by one or more organizational systems and provided by a common control provider rather than implemented by individual system owners.
**Source:** SP800-37 Rev. 2, Chapter 2
**Normative:** yes
**Synonyms / variants:** Inherited control
**Deprecated forms:** none
**NIST control reference:** PL-2, CA-7

---

## Continuous Monitoring

**Definition:** Maintaining ongoing awareness of information security, vulnerabilities, and threats to support organizational risk management decisions; operationalized in the RMF Monitor step and enabling ongoing authorization.
**Source:** SP800-37 Rev. 2, Section 3.7
**Normative:** yes
**Synonyms / variants:** Ongoing monitoring, information security continuous monitoring (ISCM)
**Deprecated forms:** none
**NIST control reference:** CA-7, SI-4

---

## Controlled Unclassified Information (CUI)

**Definition:** Information the Government creates or possesses, or that an entity creates or possesses for or on behalf of the Government, that a law, regulation, or government-wide policy requires or permits an agency to handle using safeguarding or dissemination controls.
**Source:** SP800-171 / Executive Order 13556 (referenced throughout SP800-53)
**Normative:** yes
**Synonyms / variants:** Sensitive but Unclassified (SBU) — deprecated
**Deprecated forms:** SBU, For Official Use Only (FOUO) — both deprecated by EO 13556
**NIST control reference:** MP-3, SC-28, AC-3

---

## Depth (Assessment Attribute)

**Definition:** The rigor and level of detail associated with the application of an assessment method; values are basic, focused, or comprehensive; higher depth requires more thorough evidence gathering.
**Source:** SP800-53A Rev. 5, Appendix C
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**NIST control reference:** CA-2

---

## Determination Statement

**Definition:** The atomic unit within an assessment procedure that specifies exactly what condition must be true for the corresponding part of the control to be "satisfied"; each determination statement produces exactly one finding (S or O).
**Source:** SP800-53A Rev. 5, Section 3.3 and Appendix E
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**NIST control reference:** CA-2

---

## Impact Level

**Definition:** The magnitude of harm that can be expected to result from a threat event; determined against three values (Low, Moderate, High) for each of three security objectives (confidentiality, integrity, availability) per FIPS 199.
**Source:** SP800-37 Rev. 2 (Task C-1), SP800-30 Rev. 1 (Table I-3)
**Normative:** yes
**Synonyms / variants:** Security categorization level
**Deprecated forms:** none
**NIST control reference:** RA-2

---

## Ongoing Authorization

**Definition:** An authorization approach in which the authorizing official continuously reviews risk information produced by the continuous monitoring program rather than conducting periodic point-in-time reauthorizations.
**Source:** SP800-37 Rev. 2, Task M-6 and Task R-4
**Normative:** yes
**Synonyms / variants:** Continuous authorization
**Deprecated forms:** none
**NIST control reference:** CA-6, CA-7

---

## Organization-Defined Parameter (ODP)

**Definition:** A placeholder in a control statement that an organization must assign a specific value to during the control tailoring process; the assigned value appears in the SSP and makes the control deterministic and assessable.
**Source:** SP800-53A Rev. 5, Chapter 4 (throughout); SP800-37 Rev. 2, Task S-2
**Normative:** yes
**Synonyms / variants:** Assignment value, organization-defined value
**Deprecated forms:** none
**Usage notes:** An undefined ODP produces an "other than satisfied" finding during assessment. ODP values are set in the SSP, not in code annotations. See SDR-005.
**NIST control reference:** (applicable to all controls with ODP notation)

---

## Other Than Satisfied (O)

**Definition:** An assessment finding indicating that the assessment objective for a determination statement has not been met, or that the assessor was unable to obtain sufficient information to make the determination; does not automatically mean the system is insecure, but requires organizational review.
**Source:** SP800-53A Rev. 5, Section 3.3
**Normative:** yes
**Synonyms / variants:** "Not satisfied" (informal)
**Deprecated forms:** "Failed" (implies absolute failure; SP 800-53A distinguishes between deficiency and insufficient evidence)
**NIST control reference:** CA-2, CA-5

---

## Plan of Action and Milestones (POA&M)

**Definition:** A document that identifies tasks requiring completion to remediate weaknesses or deficiencies in implemented security and privacy controls; includes resources required, milestones, and completion dates; reviewed by the AO as part of the authorization package.
**Source:** SP800-37 Rev. 2, Task A-6
**Normative:** yes
**Synonyms / variants:** POAM, POA&M
**Deprecated forms:** none
**NIST control reference:** CA-5

---

## Risk Executive Function

**Definition:** An individual or group in the organization that helps ensure risk-related considerations for individual information systems are viewed from an organization-wide perspective; provides risk-based guidance to authorizing officials and mission/business owners; bridges Tier 1 and Tier 2/3.
**Source:** SP800-39, Chapter 3; SP800-37 Rev. 2 (referenced throughout Authorize step)
**Normative:** yes
**Synonyms / variants:** Senior Accountable Official for Risk Management (senior DoD usage)
**Deprecated forms:** none
**NIST control reference:** PM-2, RA-2

---

## Risk Framing

**Definition:** The first component of the risk management process; establishes the context and risk assumptions, constraints, risk tolerance, and priorities and trade-offs under which risk decisions are made; produces the risk management strategy.
**Source:** SP800-39, Section 3.1
**Normative:** yes
**Synonyms / variants:** Risk context
**Deprecated forms:** none
**NIST control reference:** PM-9, RA-3

---

## Risk Management Strategy

**Definition:** An org-level document and set of decisions that establishes how risk is assessed, how the organization will respond to risk, what risk tolerance levels apply, and the priorities and trade-offs among mission, security, and cost; constrains all lower-tier authorization decisions.
**Source:** SP800-39, Section 3.1
**Normative:** yes
**Synonyms / variants:** none
**Deprecated forms:** none
**NIST control reference:** PM-9

---

## Risk Tolerance

**Definition:** The level of risk or degree of uncertainty that is acceptable to the organization; established at Tier 1 and constrains all AO authorization decisions at Tier 3.
**Source:** SP800-39, Section 3.1; SP800-37 Rev. 2 (Authorize step)
**Normative:** yes
**Synonyms / variants:** Risk appetite (less precise)
**Deprecated forms:** none
**NIST control reference:** PM-9, RA-3

---

## Satisfied (S)

**Definition:** An assessment finding indicating that for the portion of the control addressed by a determination statement, the assessment objective has been met and the result is fully acceptable.
**Source:** SP800-53A Rev. 5, Section 3.3
**Normative:** yes
**Synonyms / variants:** "Passed" (informal, acceptable in non-normative context)
**Deprecated forms:** none
**NIST control reference:** CA-2

---

## Security Assessment Report (SAR)

**Definition:** A document produced by control assessors that details the findings and recommendations from control assessments; includes all satisfied and other-than-satisfied findings, recommended remediation actions, and is a required component of the authorization package.
**Source:** SP800-37 Rev. 2, Task A-4; SP800-53A Rev. 5, Section 3.3
**Normative:** yes
**Synonyms / variants:** Assessment report
**Deprecated forms:** none
**NIST control reference:** CA-2, CA-6

---

## Security Categorization

**Definition:** The process of determining the security category (Low, Moderate, High) for information or an information system based on the potential impact of a security breach on the three security objectives (confidentiality, integrity, availability); performed per FIPS 199 and SP 800-60.
**Source:** SP800-37 Rev. 2, Task C-1
**Normative:** yes
**Synonyms / variants:** System categorization
**Deprecated forms:** none
**NIST control reference:** RA-2

---

## Security Control

**Definition:** A safeguard or countermeasure prescribed for an information system or an organization designed to protect the confidentiality, integrity, and availability of its information and to meet a set of defined security requirements.
**Source:** SP800-53 Rev. 5 (referenced by all four documents in this collection)
**Normative:** yes
**Synonyms / variants:** Security safeguard, security measure
**Deprecated forms:** none
**NIST control reference:** (all families)

---

## Security Control Baseline

**Definition:** The set of minimum security controls established for an information system based on its security categorization; selected from SP 800-53 and tailored during the RMF Select step.
**Source:** SP800-37 Rev. 2, Task S-1
**Normative:** yes
**Synonyms / variants:** Control baseline
**Deprecated forms:** none
**NIST control reference:** RA-2, PL-2

---

## System Security Plan (SSP)

**Definition:** A formal document that provides an overview of the security requirements for an information system and describes the security controls in place or planned for meeting those requirements; the primary artifact documenting how a system implements SP 800-53 controls.
**Source:** SP800-37 Rev. 2, Tasks S-5, I-1; SP800-53A Rev. 5 (referenced as an assessment object)
**Normative:** yes
**Synonyms / variants:** Information system security plan
**Deprecated forms:** none
**Usage notes:** The SSP is an assessment object for nearly every SP 800-53A procedure's Examine method.
**NIST control reference:** PL-2

---

## Threat Source

**Definition:** The intent and method targeted at the intentional exploitation of a vulnerability or a situation and method that may accidentally trigger a vulnerability; categorized as adversarial (nation-state, criminal, terrorist, insider) or non-adversarial (errors, accidents, natural disasters, equipment failures).
**Source:** SP800-30 Rev. 1, Appendix D
**Normative:** yes
**Synonyms / variants:** Threat agent (informal, less precise)
**Deprecated forms:** none
**NIST control reference:** RA-3

---

## Three-Tier Model

**Definition:** The organizational model from SP 800-39 that structures risk management at three levels: Tier 1 (Organization — governance, risk tolerance), Tier 2 (Mission/Business Process — business owners, EA), and Tier 3 (Information System — system owners, RMF); risk decisions flow down, risk information flows up.
**Source:** SP800-39, Chapter 2
**Normative:** yes
**Synonyms / variants:** Risk management tiers
**Deprecated forms:** none
**NIST control reference:** PM-9, RA-2, CA-7
