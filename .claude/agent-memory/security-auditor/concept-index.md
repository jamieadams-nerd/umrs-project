# Concept Index — rmf-methodology
Generated: 2026-03-15

---

## SP800-37r2

**Full title:** Risk Management Framework for Information Systems and Organizations: A System Life Cycle Approach for Security and Privacy (NIST SP 800-37 Rev. 2)
**Source:** refs/nist/sp800-37r2.pdf
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the full seven-step Risk Management Framework lifecycle that federal agencies use to authorize information systems. Covers organizational roles and responsibilities, the Prepare step (org-level and system-level tasks), and all six primary steps (Categorize, Select, Implement, Assess, Authorize, Monitor) with specific task inputs, outputs, responsible roles, and SDLC phase mappings. Rev. 2 added explicit privacy risk management alongside security, and integrated SCRM throughout.

### Key concepts introduced
- **RMF** — seven-step lifecycle: Prepare, Categorize, Select, Implement, Assess, Authorize, Monitor
- **Authorization Boundary** — what defines a system for RMF purposes; determines scope of all subsequent tasks
- **Authorization to Operate (ATO)** — a formal decision by an Authorizing Official that the risk from operating a system is acceptable
- **Ongoing Authorization** — replaces point-in-time ATOs with continuous monitoring feeding real-time risk determinations
- **Authorization Package** — the set of artifacts (SSP, SAR, POA&M, privacy plan) submitted to the Authorizing Official
- **Plan of Action and Milestones (POA&M)** — tracks control deficiencies and remediation schedules from assessments
- **Common Control** — a security control inherited by multiple systems; assessed once by the common control provider
- **Authorizing Official (AO)** — the senior official who formally accepts residual risk and signs the ATO; this authority cannot be delegated
- **Risk Executive Function** — an org-level role/group that ensures risk-informed decisions are made consistently across Tier 1–3
- **Control Tailoring** — adjusting baseline controls via scoping guidance, compensating controls, and ODP values
- **Supply Chain Risk Management (SCRM)** — integrated from Prepare step P-2; identifies key suppliers and ICT supply chain risks
- **System Security Plan (SSP)** — the primary document describing the system, its boundary, and implementation of security controls
- **Security Categorization** — using FIPS 199 + SP 800-60 to assign impact levels (Low/Moderate/High) per information type
- **Continuous Monitoring Strategy** — org-defined frequency and method for ongoing control effectiveness surveillance
- **Overlays** — extensions or modifications to SP 800-53 baselines for specific communities or technologies
- **ODP (Organization-Defined Parameter)** — a placeholder in a control statement that the organization fills in during tailoring

### Governs these writing tasks
- Determining which RMF task a UMRS audit finding maps to (e.g., a missing annotation on a public type maps to Assess step A-3 control assessment deficiency)
- Understanding what artifacts an Authorizing Official expects to see (SSP, SAR, POA&M)
- Knowing the correct responsible role for each RMF task (relevant when citing AO accountability in reports)
- Understanding how continuous monitoring feeds ongoing authorization decisions
- Determining which step to cite when a finding affects system categorization, control selection, or monitoring posture

### Related documents in corpus
- SP800-53A — governs the Assess step task A-3 (control assessment procedures)
- SP800-30 — governs risk assessment tasks P-3, P-14, R-2 (risk analysis and determination)
- SP800-39 — provides the enterprise risk management context that frames org-level Prepare tasks

---

## SP800-53Ar5

**Full title:** Assessing Security and Privacy Controls in Information Systems and Organizations (NIST SP 800-53A Rev. 5)
**Source:** refs/nist/sp800-53Ar5.pdf
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Provides the authoritative catalog of assessment procedures for every security and privacy control in SP 800-53 Rev. 5. Defines the Examine/Interview/Test assessment methods, specifies assessment objects for each method, and introduces depth (basic/focused/comprehensive) and coverage attributes that scale rigor to system assurance requirements. Includes a four-phase assessment lifecycle: Prepare → Develop Plan → Conduct Assessments → Analyze Results.

### Key concepts introduced
- **Examine** — review and analysis of specifications, mechanisms, activities (documents, records, plans, policies)
- **Interview** — discussions with individuals or groups to elicit information about controls
- **Test** — exercise of mechanisms and activities under defined conditions to verify behavior
- **Assessment Objective** — a determination statement that specifies what must be verified for a control to be "satisfied"
- **Satisfied (S) / Other Than Satisfied (O)** — the two possible findings from an assessment determination statement
- **ODP (Organization-Defined Parameter)** — must be defined and implemented for a control to be assessable; undefined ODP produces an "other than satisfied" finding
- **Depth attribute** — basic / focused / comprehensive; scales the rigor of each assessment method
- **Coverage attribute** — representative sample / specific subset / all; scales the breadth of each assessment
- **Assessment Plan** — formally approved document specifying what will be assessed, how, by whom, and at what depth/coverage
- **Security Assessment Report (SAR)** — documents findings (satisfied / other than satisfied) and recommendations
- **Capability-based assessment** — evaluating groups of mutually reinforcing controls that together achieve a security capability (e.g., IA-02(01)+IA-02(02)+SC-08(01) for secure remote authentication)
- **OSCAL** — Open Security Controls Assessment Language; machine-readable format for SP 800-53A assessment procedures
- **Determination statement** — the atomic unit of an assessment procedure; produces exactly one S or O finding
- **Common control assessment reuse** — assessment results from common control providers may be reused by inheriting systems

### Governs these writing tasks
- Translating a UMRS code audit finding into the assessment method and object that would detect it (e.g., missing NIST citation → Examine: system design documentation → other than satisfied on SA-11)
- Selecting the appropriate assessment depth for a given UMRS module (basic for low-impact, comprehensive for high-impact systems)
- Knowing what "other than satisfied" means in context: not that a control is broken, but that the determination could not be made with certainty or a deficiency exists
- Writing audit finding severity levels that map to SP 800-53A subcategory severity definitions
- Understanding that assessment procedures from the catalog are a starting point — tailoring for RHEL/SELinux environments is expected

### Related documents in corpus
- SP800-37 — RMF Assess step A-1 through A-6 consumes SP 800-53A assessment procedures
- SP800-30 — risk determination post-assessment uses SP 800-30 likelihood/impact tables
- SP800-39 — assessment results feed back into risk response (Respond component) at all three tiers

---

## SP800-30r1

**Full title:** Guide for Conducting Risk Assessments (NIST SP 800-30 Rev. 1)
**Source:** refs/nist/sp800-30r1.pdf
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Defines the methodology for conducting information security risk assessments at any of the three organizational tiers. Provides taxonomic tables for threat sources (adversarial and non-adversarial), threat events, vulnerabilities and predisposing conditions, likelihood determination (initiation and success), impact determination, and risk determination. Establishes a semi-quantitative risk scale (Very High / High / Moderate / Low / Very Low) and a 5x5 risk matrix.

### Key concepts introduced
- **Threat source** — the intent and method targeted at exploitation; adversarial (nation-state, criminal, terrorist, insider) or non-adversarial (errors, accidents, natural disasters)
- **Threat event** — the action taken by a threat source; characterized by relevance and likelihood of initiation
- **Vulnerability** — a weakness that can be exploited; only deficiencies exploitable by threat agents are vulnerabilities in this sense
- **Predisposing condition** — an organizational or system condition that increases susceptibility to a threat event
- **Likelihood of initiation** — probability that a threat source will attempt an attack (adversarial only)
- **Likelihood of success** — probability that the attempt will succeed given existing controls
- **Overall likelihood** — for adversarial threats: f(initiation likelihood, success likelihood); for non-adversarial: direct single estimate
- **Impact** — harm resulting from a threat event materializing; assessed against operations, assets, individuals, other organizations, and the Nation
- **Risk determination** — combination of overall likelihood and impact on the 5x5 matrix producing a risk level
- **Risk response** — accept, avoid, mitigate, share, or transfer risk; mitigate actions feed the POA&M
- **Semi-quantitative scale** — Very High (96–100 or qualitative descriptor) through Very Low; allows consistent cross-system comparison

### Governs these writing tasks
- Assigning severity levels in UMRS audit reports (HIGH/MEDIUM/LOW) — these map to SP 800-30 impact levels
- Determining when a missing annotation is HIGH severity (load-bearing claim about authorization/classification) vs. LOW (indirect impact): HIGH corresponds to Very High / High impact on operations or assets
- Understanding the two-factor likelihood model when reasoning about exploitability of an annotation gap
- Justifying why a doc-vs-code inconsistency on a security claim is HIGH (it misleads the Authorizing Official's risk determination)

### Related documents in corpus
- SP800-37 — risk assessments feed into Prepare tasks P-3 and P-14, and Authorize task R-2
- SP800-39 — SP 800-30 is the operational tool for SP 800-39's risk assessment component
- SP800-53A — assessment findings feed into SP 800-30 risk determination tables

---

## SP800-39

**Full title:** Managing Information Security Risk: Organization, Mission, and Information System View (NIST SP 800-39)
**Source:** refs/nist/sp800-39.pdf
**Type:** regulatory standard
**Normative weight:** normative

### Coverage
Provides the enterprise-level risk management framework that sits above and governs the RMF lifecycle. Establishes the three-tier model (Organization / Mission-Business Process / Information System) and four risk management components: Frame, Assess, Respond, Monitor. Defines risk tolerance, risk management strategy, and the Risk Executive Function role. Explains how risk decisions at each tier constrain and inform decisions at other tiers.

### Key concepts introduced
- **Three-tier model** — Tier 1 (Organization governance), Tier 2 (Mission/Business Process), Tier 3 (Information System); risk decisions flow down and risk information flows up
- **Risk framing** — establishes the risk assumptions, constraints, tolerance, and priorities that govern all subsequent risk management decisions; produces the risk management strategy
- **Risk management strategy** — org-level document that defines how risk is assessed, responded to, and monitored; constrains what AOs can accept
- **Risk tolerance** — the level of risk the organization is willing to accept; established at Tier 1; constrains all lower-tier authorization decisions
- **Risk Executive Function** — an org-level role or group (not an individual) that ensures risk-informed decisions are consistent across the organization
- **Risk response options** — accept, avoid, mitigate, share, transfer; choice constrained by risk tolerance and organizational priorities
- **Risk monitoring** — ongoing surveillance of risk factors, effectiveness of risk responses, and compliance with risk management strategy
- **Enterprise architecture integration** — risk management integrated into EA processes (reference architectures, segment/solution architectures)
- **Trust relationships** — explicit modeling of trust between systems, organizations, and external providers; drives control selection
- **Organizational risk posture** — aggregate risk across all tiers; maintained by the Risk Executive Function

### Governs these writing tasks
- Understanding why org-level risk framing and tolerance must be established before system-level authorization decisions are meaningful
- Recognizing that a UMRS audit finding at the system level (Tier 3) has implications at Tier 2 (mission impact) and Tier 1 (organizational risk posture)
- Knowing when to escalate: findings that affect risk posture beyond a single system belong in Tier 1/2 communications, not just the system SAR
- Framing the "why this matters" section of audit reports in terms of mission/business impact, not just technical deficiency

### Related documents in corpus
- SP800-37 — SP 800-39's three-tier model provides the organizational context for every RMF task
- SP800-30 — SP 800-30 is the operational tool for SP 800-39's risk assessment component
- SP800-53A — assessment results feed SP 800-39's Assess and Respond components at all tiers
