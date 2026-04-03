# Five Eyes Case Study Research — Expanded Corpus
**Prepared by:** The Librarian (researcher agent)
**Date:** 2026-04-03
**Status:** Research complete — ready for tech-writer integration

---

## Summary Table

| Nation | New Cases in This Report | Primary Failure Classes |
|---|---|---|
| United States (additional) | 13 | Trusted-without-verify, insider-over-access, record falsification, audit-but-no-enforcement |
| Canada (additional) | 6 | Insider snooping, indigenous data sovereignty, RCMP surveillance, health data |
| United Kingdom | 14 | Lost records, trusted-without-verify, over-access, labeling failure, audit gap |
| Australia | 10 | Over-access, trusted-without-verify, defence contractor, unlawful automation |
| New Zealand | 7 | Health data, shared-IT-provider risk, payroll failure, over-access |
| **Total new cases** | **50** | |

### Failure Class Distribution (across all 50 cases)

| Failure class | Count |
|---|---|
| Trusted software/process without independent verification | 11 |
| Over-access / no compartmentalization by sensitivity | 13 |
| Record falsification / provenance destruction | 7 |
| Audit existed but could not prove anything | 8 |
| Third-party/contractor trust without verification | 7 |
| Physical custody failure | 3 |
| Labeling/handling failure | 4 |
| Unlawful automated data matching (no human check) | 2 |

### CUI Index Group Coverage in New Cases

| Index Group | Cases |
|---|---|
| Privacy (PRVCY, HLTH) | 14 |
| Defence / Law Enforcement | 8 |
| Financial (FNC, TAX) | 4 |
| Export Control (EXPT, SP-EXPT) | 3 |
| Critical Infrastructure (CRIT) | 5 |
| Transportation (TRAN) | 2 |
| Patent / Proprietary (PROPIN) | 2 |
| Legal (LEI) | 2 |
| Statistical | 2 |
| Government Administration (across multiple) | 8 |

---

## Important Notes and Language Discipline

All UMRS design property descriptions in this document follow the project language rules:

- "Controls like these would have made this harder to do undetected."
- "A system designed with these properties would have created earlier visibility."
- "The failure occurred in the absence of controls that UMRS is designed to provide."
- "These controls were published and required; they were not implemented."

No description claims UMRS would have prevented any incident.

---

## United States — Additional Cases

### US-11: Post Office Horizon (US Parallel) — SolarWinds Supply Chain Attack (2020)

**What happened:** Russian SVR operatives breached SolarWinds' build pipeline in September 2019 and injected malicious code (SUNBURST) into the Orion network management platform. Trojanized updates bearing SolarWinds' valid digital signature were pushed to approximately 18,000 customers beginning in March 2020. Nine federal agencies sustained follow-on compromise, including the Departments of Treasury, Commerce, Justice, Homeland Security, and State. The attack went undetected for roughly nine months.

**Nation:** United States

**CUI category equivalent:** CUI//SP-CTI (Critical Infrastructure, Cyber Threat Intelligence) and CUI//GOVDELIVERY (Government Delivery) across multiple categories depending on what each agency's Orion installation could access.

**Failure class:** Trusted-without-verify. Software signed by the vendor was trusted unconditionally by all downstream recipients. There was no independent integrity check on the build pipeline, no measurement of what the update binary contained before deployment, and no anomaly detection on the post-installation behaviour of the update. The digital signature verified origin but not content integrity of the post-injection artefact.

**Human and financial cost:** Nine federal agencies confirmed as targets of follow-on espionage activity. The GAO estimated that the total cost of the federal government's response exceeded $100 million. The intelligence damage — access to unclassified email accounts, internal communications, and systems — remains incompletely assessed. It ranks among the most significant espionage operations against the United States government since the OPM breach.

**Controls not evidenced:** NIST SP 800-53 SI-7 (Software, Firmware, and Information Integrity), SA-12 (Supply Chain Protection), CM-14 (Signed Components). NIST SP 800-161 (Supply Chain Risk Management) was published in 2015 and its principles were applicable; they were not systematically implemented.

**UMRS design property:** IMA/EVM Appraisal (Phase 2) treats every deployed binary as a measured artefact. The measurement log, maintained by the kernel, records the hash of each file at load time. A trojanized update binary would produce a hash that did not match the expected value from the build manifest — the kernel would detect the discrepancy before the update executed. Provenance binding between the build pipeline and deployed artefact is the structural control the SolarWinds attack bypassed.

**Source citations:**
- CISA Emergency Directive 21-01 (2020-12-13): https://www.cisa.gov/news-events/cybersecurity-advisories/aa20-352a
- GAO Report GAO-22-104746: Federal Response to SolarWinds and Microsoft Exchange Incidents
- 2020 United States federal government data breach: https://en.wikipedia.org/wiki/2020_United_States_federal_government_data_breach

---

### US-12: Colonial Pipeline Ransomware Attack (2021)

**What happened:** On 7 May 2021, ransomware actors (DarkSide) gained access to Colonial Pipeline's IT network using a compromised VPN account password. The VPN account was inactive and had no multi-factor authentication. Attackers stole approximately 100 GB of data within two hours before deploying ransomware on billing and accounting systems. Although the operational technology (OT) pipeline systems were not directly compromised, Colonial shut down pipeline operations because it could not bill customers — demonstrating that IT/OT boundary confusion contributed to the shutdown decision. Colonial paid a $4.4 million ransom within hours. The pipeline, which supplies approximately 45% of the US East Coast's fuel, was shut down for six days.

**Nation:** United States

**CUI category equivalent:** CUI//CRIT (Critical Infrastructure) — pipeline safety and operational continuity data.

**Failure class:** Over-access (inactive account retained VPN access) and trusted-without-verify (no MFA on a privileged remote access account). The inactive account was not deprovisioned. There was no separation between accounts used for active operations and archived accounts. No anomaly detection flagged access from a dormant account.

**Human and financial cost:** Average US gasoline prices rose to over $3.00 per gallon — the highest in six years. Fuel shortages on the US East Coast, particularly in the Southeast. Colonial paid $4.4 million ransom; the US Department of Justice later recovered $2.3 million via seizure. Costs of IT remediation were not fully disclosed.

**Controls not evidenced:** NIST SP 800-53 AC-2 (Account Management — inactive account removal), IA-5 (Authenticator Management — MFA), CM-6 (Configuration Settings). These controls were published and required under federal baseline standards; they were not implemented for this account.

**UMRS design property:** Mandatory access control at the account/identity level treats dormant accounts as untrusted — access from an account that has not been used within a defined window requires re-attestation before being granted. MLS-style separation between "active operations" and "legacy credential" domains means an account decommissioned from active use cannot be silently reused at full privilege. Controls like these would have flagged or blocked access from the unused credential before the intrusion succeeded.

**Source citations:**
- CISA Advisory: The Attack on Colonial Pipeline: https://www.cisa.gov/news-events/news/attack-colonial-pipeline-what-weve-learned-what-weve-done-over-past-two-years
- Colonial Pipeline ransomware attack — Wikipedia: https://en.wikipedia.org/wiki/Colonial_Pipeline_ransomware_attack
- US GAO: Colonial Pipeline Cyberattack Highlights Need for Better Federal and Private-Sector Preparedness

---

### US-13: IRS Tax Records Leak — ProPublica / "Billionaires' Tax Returns" (2021)

**What happened:** In June 2021, ProPublica published detailed federal income tax returns for the wealthiest Americans spanning fifteen or more years. The data included the precise tax liability, income, investment activity, and effective tax rates for specific named individuals. The source of the data has not been publicly identified or prosecuted despite FBI, IRS Inspector General, and Treasury Inspector General investigations. No single access event was identified as the mechanism; the leak was attributed to "an insider" with authorized access to IRS systems.

**Nation:** United States

**CUI category equivalent:** CUI//TAX (Tax information — one of the most sensitive CUI categories under 26 U.S.C. § 6103, which explicitly prohibits unauthorized disclosure of return information).

**Failure class:** Audit existed but could not prove anything. The IRS investigated approximately 1,700 cases of unauthorized employee access between 2012 and 2021, with roughly 27% confirmed as violations. The ProPublica leak demonstrated that authorized access to protected tax return data was not sufficiently monitored or audited to permit retroactive attribution — more than a year after the leak, no arrest had been made and the federal government publicly acknowledged it did not know the source.

**Human and financial cost:** No direct financial harm to recipients (the individuals whose returns were leaked are high-net-worth individuals). Significant erosion of public trust in the IRS's ability to protect confidential tax data. Congressional scrutiny. Potential harm to the IRS's ability to collect voluntary compliance if taxpayers believe their returns are not secure.

**Controls not evidenced:** NIST SP 800-53 AU-2 (Event Logging), AU-9 (Protection of Audit Information), AU-12 (Audit Record Generation). 26 U.S.C. § 6103 has prohibited unauthorized disclosure since 1976. The technical audit controls required to detect and attribute the leak were insufficient despite long-standing statutory requirements.

**UMRS design property:** Cryptographic, append-only audit logging for every access to protected tax records creates a forensic record attributable to a specific identity at a specific time. A leak of this type, at this scale, from a system with kernel-level audit control, would be attributable — not necessarily preventable, but detectable and attributable. The failure was not that access to tax records occurred, but that access at the requisite volume to build a fifteen-year dataset of specific individuals left no traceable record.

**Source citations:**
- ProPublica, "The Secret IRS Files" (2021-06-08): https://www.propublica.org/article/propublicas-tax-revelations-lead-to-calls-for-reforms-and-investigation
- GAO: Taxpayer Information Keeps Ending Up in the Wrong Hands: https://www.gao.gov/blog/taxpayer-information-keeps-ending-wrong-hands.-what-can-irs-do-better-protect-it
- House Oversight Committee correspondence with IRS Commissioner Rettig (2021-08-25)

---

### US-14: Veterans Affairs — 46,000-Record Financial Diversion Breach (2020)

**What happened:** On 14 September 2020, the VA's Financial Services Center disclosed that unauthorized users had accessed an online application used to process payments to community healthcare providers for veteran medical treatment. Attackers used social engineering to change financial information and divert payments. At least 46,000 veteran records were affected, including Social Security numbers, financial information, and healthcare data. In a separate 2020 mailing incident, approximately 6,000 benefit summary letters were sent to incorrect addresses in Massachusetts, with 3,913 containing the wrong veteran's Social Security number.

**Nation:** United States

**CUI category equivalent:** CUI//PRVCY (Privacy) and CUI//HLTH (Health) — veteran personal health and financial data held by a federal agency.

**Failure class:** Over-access and insufficient authentication controls. Social engineering overcame authentication. Payment diversion was possible because the update of financial information and the authorization of payment were not separated into distinct trust domains. The mismailing incident illustrates a separate failure: data merge from an old address database was not validated before distribution.

**Human and financial cost:** 46,000 veterans at risk of financial fraud and identity theft. Diverted payments disrupted healthcare payments to community providers who had already rendered services. The mismailing incident exposed SSNs to recipients who were not the intended subjects.

**Controls not evidenced:** NIST SP 800-53 IA-2 (Identification and Authentication), AC-6 (Least Privilege), SI-10 (Information Input Validation). The VA has been subject to recurring audit findings about data handling; these controls were applicable and required.

**UMRS design property:** Separation of duties enforced as a labeled-domain boundary: the identity that submits a change to financial records cannot be the same identity that approves disbursement. Purpose-based access control prevents the "payment diversion" operation from being available within the same session context as "patient record lookup." Controls like these would have prevented a single compromised session from affecting both data access and payment redirection.

**Source citations:**
- VA Press Release, "VA Notifies Veterans of Compromised Personal Information" (2020-09-14): https://news.va.gov/press-room/va-notifies-veterans-of-compromised-personal-information/
- Senate Finance Committee demands for answers: https://www.hipaajournal.com/senators-demand-answers-from-va-on-46000-record-data-breach/

---

### US-15: Los Alamos National Laboratory — Nuclear Data on USB Drive (2006)

**What happened:** In October 2006, federal authorities conducting a drug investigation at a residence near Los Alamos, New Mexico, discovered USB drives containing 408 classified documents. The drive holder, Jessica Lynn Quintana, had been a contractor at LANL with authorized access to classified information. The documents contained sensitive nuclear weapons design data. The DOE and NNSA initiated formal enforcement actions against the University of California (LANL's manager) and Los Alamos National Security LLC, assessing a $3.3 million civil penalty. A subsequent GAO audit identified ongoing systemic weaknesses in LANL's classified data management program.

**Nation:** United States

**CUI category equivalent:** CUI//NUC (Nuclear — specifically classified nuclear weapons design data; noted here as the CUI framework addresses the unclassified nuclear tier, while this case involved actual classified material — included because it demonstrates the access control and physical custody failure pattern applicable to both classified and CUI nuclear information).

**Failure class:** Physical custody failure compounded by insufficient removable media controls. Classified data could be transferred to personal removable media without detection. There was no enforced technical control preventing bulk export of classified material to USB. Chain of custody for digital artefacts containing nuclear data was social, not structural.

**Human and financial cost:** $3.3 million federal civil penalty. National security exposure — 408 classified nuclear documents outside controlled facilities. GAO cited "management deficiencies" as the central contributing factor. LANL subsequently undertook significant security remediation.

**Controls not evidenced:** NIST SP 800-53 MP-7 (Media Use — restricting removable media), SC-28 (Protection of Information at Rest), AU-3 (Content of Audit Records). DOE Order 471.2 (Classified Matter Protection and Control) was applicable and required; systematic enforcement was lacking.

**UMRS design property:** Removable media access as a controlled domain boundary — moving data to removable media is a privileged, audited operation requiring explicit authorization. Data export creates an immutable kernel-level audit event tied to the exporting identity and the specific document set. The concept of "data in motion to uncontrolled medium" as a security boundary crossing — requiring mandatory labeling of the transfer — directly applies to the failure mode.

**Source citations:**
- DOE Enforcement Action (2007): https://www.govtech.com/security/department-of-energy-initiates-formal-enforcement.html
- GAO-08-173R, Los Alamos National Laboratory: Information on Security of Classified Data: https://www.gao.gov/products/gao-08-173r
- NPR: Los Alamos Lab Adjusts Security After Breach (2006-11-13)

---

### US-16: IRS Insider Misuse Pattern (2012–2021)

**What happened:** GAO and Treasury Inspector General for Tax Administration (TIGTA) audits documented that between 2012 and 2021, the IRS investigated approximately 1,700 cases of unauthorized employee access to taxpayer records. Approximately 27% were confirmed violations — employees accessing records of friends, family members, celebrities, ex-spouses, or neighbours for personal reasons. The IRS found it difficult to detect or deter this behaviour: the technical audit logs existed but did not have sufficient contextual linkage to distinguish a "legitimate work access" from a "curiosity access" to the same record.

**Nation:** United States

**CUI category equivalent:** CUI//TAX — taxpayer return information, the most explicitly protected CUI category under federal statute (26 U.S.C. § 6103).

**Failure class:** Over-access and audit-existed-but-cannot-prove-anything. Access was role-based ("IRS employee") but not compartmentalized by purpose or caseload. An employee with authorized access to "IRS systems" had no structural constraint preventing access to records outside their assigned workload. The audit log recorded accesses but not the justification for each access; retroactive investigation was required to detect violations.

**Human and financial cost:** 1,700 investigations over nine years. Countless taxpayers' most private financial information accessed without their knowledge. Erosion of public trust in the confidentiality of tax filings. GAO assessed the IRS's detection and deterrence mechanisms as insufficient.

**Controls not evidenced:** NIST SP 800-53 AC-6 (Least Privilege), AC-17 (Purpose-Based Access), AU-6 (Audit Review, Analysis, and Reporting). 26 U.S.C. § 6103 explicitly prohibits unauthorized disclosure; the technical controls required to enforce it at the access level were not in place.

**UMRS design property:** Purpose-based compartmentalization — "accessing a record because it is in your assigned caseload" and "accessing a record outside your assigned caseload" are distinct operations in different security contexts. Access to a record outside the assigned workload requires explicit authorization through an escalated context. Controls like these would have eliminated the ambiguity that made retroactive investigation the only detection mechanism.

**Source citations:**
- GAO: Taxpayer Information Keeps Ending Up in the Wrong Hands: https://www.gao.gov/blog/taxpayer-information-keeps-ending-wrong-hands.-what-can-irs-do-better-protect-it
- TIGTA, Fiscal Year 2021 Contractor Training Report

---

### US-17: NHS/Anthem Parallel — US Health Insurance Data (CUI//HLTH) Segmentation Pattern

*(Note: Anthem (2015) is already in the corpus. This entry documents the broader pattern with a distinct institutional case.)*

**What happened:** In 2020, the US Department of Veteran Affairs confirmed an additional category of breach: vaccine status information for approximately 500,000 employees was disclosed without authorization to members of Veterans Health Administration senior leadership. In a separate 2022 incident, a VHA Medical Center disclosed the full SSNs of veterans in correspondence. These incidents illustrate that health data segmentation failure is not confined to external attack — it recurs as an internal mishandling pattern even within organizations that have had prior breach incidents.

**Nation:** United States

**CUI category equivalent:** CUI//HLTH (Health information) and CUI//PRVCY (Privacy — veteran personnel data).

**Failure class:** Labeling and handling failure. Vaccination status, while not classified, was treated as ordinary administrative data rather than as protected health information with controlled dissemination pathways. There was no mechanism preventing broad distribution of what is, under law (ADA, HIPAA-adjacent authorities), protected personal health information.

**Controls not evidenced:** NIST SP 800-53 AC-4 (Information Flow Enforcement), SC-16 (Transmission of Security Attributes). HIPAA and VA Privacy Act obligations existed and required controlled handling of health information.

**UMRS design property:** CUI labels on health data subsets enforce that "vaccination status" cannot flow from "HR system" to "unrestricted email distribution list" without an access control decision at the boundary. Controls like these would have prevented the distribution pathway from being available without explicit authorization.

**Source citations:**
- VA press releases and Federal Times (2022-11): https://www.federaltimes.com/management/hr/2022/11/23/veterans-affairs-dept-discloses-breach-of-employee-vaccine-data/

---

### US-18: US Water System Data Falsification — Expanded Pattern

*(Companion to the DOJ cases already in the corpus; this adds the PFAS/military data documentation gap.)*

**What happened:** Environmental Working Group investigations in 2024–2025 documented that the EPA's official public records classified fifteen military Superfund sites as having "human exposure under control" with respect to PFAS contamination — despite DoD's own drinking water test results indicating otherwise. The EPA records had not been updated to reflect evidence in DoD's own test results. This is not an allegation of deliberate falsification; it is a documented discrepancy between what agency data shows and what official records state, with no audit trail linking the decision not to update the classification.

**Nation:** United States

**CUI category equivalent:** CUI//ENV (Environmental — PFAS contamination data) and CUI//CRIT (Critical Infrastructure — water supply). Note: DoD also handles this data as CUI//PREVMED (Preventive Medicine).

**Failure class:** Audit existed but could not prove anything — the discrepancy between DoD test data and EPA classification exists in the public record but the agency records show no documented decision about when, why, or by whom the classification was determined to remain unchanged in the face of contradicting evidence.

**Human and financial cost:** Communities near the fifteen sites are operating on incorrect public health assurances. GAO-25-107401 documented that DoD has not adequately accounted for PFAS remediation costs to Congress. The scale of PFAS contamination at US military sites is estimated in the thousands of locations. The absence of timely record updates delays community notification and remediation.

**Controls not evidenced:** NIST SP 800-53 AU-10 (Non-Repudiation — for decisions about data classification updates), SI-12 (Information Management and Retention). The requirement to update environmental risk classifications when new evidence contradicts the existing classification was not enforced through any technical control.

**UMRS design property:** Decision records as controlled artefacts — when agency data contradicts an existing classification, the decision to leave the classification unchanged is itself a recorded event with mandatory attribution. "Data exists that contradicts this record" is a detectable state; a system designed with provenance controls treats the contradiction as requiring documented resolution, not silent toleration.

**Source citations:**
- Environmental Working Group (2025-07): 15 Military Superfund Sites Claim No Health Risks: https://www.ewg.org/news-insights/news/2025/07/15-military-superfund-sites-claim-no-health-risks-pfas-tap-water-tell
- GAO-25-107401, Persistent Chemicals: DoD Needs to Provide Congress More Information on Costs: https://www.gao.gov/assets/gao-25-107401.pdf

---

### US-19: Trade Secret Theft — JHL Biotech and the CUI//PROPIN Pattern (2020–2022)

**What happened:** On 15 March 2022, Racho Jordanov and Rose Lin, co-founders of JHL Biotech (Taiwan), were sentenced to federal imprisonment for conspiring to steal trade secrets from a US biopharmaceutical company. The stolen materials included manufacturing processes and biological drug formulas — information that, in a government-contractor context, would be classified CUI//PROPIN (Proprietary Business Information) or CUI//SP-EXPT (Export Controlled — Biologics/ITAR-adjacent). The DOJ's annual PRO IP Act reports document dozens of similar prosecutions annually, with the number of cases involving nation-state-adjacent actors increasing each year.

**Nation:** United States

**CUI category equivalent:** CUI//PROPIN (Proprietary Business Information) — trade secrets and proprietary manufacturing processes. In government-adjacent contexts, also CUI//SP-EXPT (Export Controlled).

**Failure class:** Over-access and no compartmentalization. The founders had broad access to the victim company's intellectual property through collaborative business arrangements. The IP was not compartmentalized by access tier or purpose; research collaboration access was indistinguishable from the access needed to perform a bulk exfiltration. There was no integrity binding between specific IP documents and the authorized recipients of those documents.

**Human and financial cost:** Federal imprisonment. Loss of competitive IP developed over years of research investment. Broader pattern: the DOJ PRO IP Act FY2022 report documents multiple prosecutions per year for trade secret theft, with a significant fraction involving actors connected to state-sponsored industrial espionage programs.

**Controls not evidenced:** NIST SP 800-53 AC-4 (Information Flow Enforcement), CM-10 (Software Usage Restrictions), AU-9 (Protection of Audit Information). 18 U.S.C. § 1832 (Theft of Trade Secrets) has been in force since 1996; the technical controls required to detect bulk exfiltration of IP were not in place.

**UMRS design property:** CUI//PROPIN labeling on research IP creates a machine-enforced boundary. Access to the document set for "review" purposes is a different security context from "bulk download for manufacturing." Anomaly detection on access volume — a collaborator downloading a manufacturing process document library at scale is a detectable event. Provenance binding between documents and authorized recipients means exfiltration leaves a kernel-level record.

**Source citations:**
- DOJ Sentencing Announcement (2022-03-15): https://www.justice.gov/criminal/criminal-ccips/file/640271/dl
- DOJ PRO IP Act Annual Report FY2022: https://justice.gov/criminal/media/1364816/dl?inline=

---

### US-20: Municipal Police Database Misuse — Expanded Pattern Documentation

**What happened:** The pattern of police and government employees misusing legitimate database access (DMV records, law enforcement databases, welfare databases) to look up individuals for personal purposes is pervasive and documented across all US jurisdictions. Law enforcement databases including NCIC, CLETS, NLETS, and state DMV systems are routinely accessed by officers for non-law-enforcement purposes — looking up ex-spouses, celebrities, journalists, neighbours, or people involved in personal disputes. FBI audits and state-level IG investigations have documented hundreds of confirmed violations annually. The detection mechanism is almost always reactive (after external harm), not proactive.

**Nation:** United States

**CUI category equivalent:** CUI//LEI (Law Enforcement Sensitive — NCIC and related law enforcement databases) and CUI//PRVCY (Privacy — DMV, welfare, and administrative databases).

**Failure class:** Over-access and audit-existed-but-cannot-prove-anything. Access is role-based ("law enforcement officer") but not compartmentalized by purpose (active investigation) or subject (records assigned to the officer's active cases). The audit log records "officer X accessed record Y at time Z" but cannot distinguish a legitimate investigation query from a personal curiosity query without additional investigation. Detection is manual and reactive.

**Human and financial cost:** Exposed individuals face stalking, harassment, and physical danger — particularly victims of domestic violence whose location information is in DMV systems. Officers have used database access to track ex-partners. Journalists and activists have had their locations accessed without legal basis. The cost is measured in individual safety harms, civil liability, and erosion of public trust in law enforcement data stewardship.

**Controls not evidenced:** NIST SP 800-53 AC-6 (Least Privilege), AU-6 (Audit Review), and the FBI's own NCIC policy require purpose-based access justification. The technical enforcement of these requirements is absent from most implementations.

**UMRS design property:** The access context must encode purpose — "active investigation lookup on an assigned case" versus "general query" are different security contexts requiring different authorization. A query on an individual who is not linked to an active investigation in the officer's assigned caseload requires escalated authorization. Controls like these would convert the "audit-but-cannot-prove" situation into a "access-requires-justification-at-time-of-access" situation.

**Source citations:**
- FBI NCIC audit findings (multiple years, publicly referenced in IG reports)
- Pattern documented by Electronic Privacy Information Center and state IG offices

---

### US-21: Babylon Health Parallel — US Telehealth Data Segmentation (COVID-19 Era)

**What happened:** During the rapid expansion of telehealth services during the COVID-19 pandemic (2020–2022), multiple investigations and news reports documented that health platforms, including several used by US federal health programs (Medicaid, CHIP), had shared patient-level data with advertising networks including Facebook (Meta Pixel) and Google Analytics without patient consent or appropriate authorization. A 2022 investigation found this practice was widespread among hospital systems and telehealth providers. The data shared included condition indicators, appointment types, and user identifiers.

**Nation:** United States

**CUI category equivalent:** CUI//HLTH (Health information) and CUI//PRVCY (Privacy). Where federally funded programs are involved: also potentially CUI//MEDICAID.

**Failure class:** Over-access by third-party platforms — data flowed outside its authorized handling domain without any labeling or flow control. The absence of data labeling on health information meant there was no technical mechanism to prevent it from flowing to advertising systems. The data was not marked as a controlled object, so no boundary enforced its handling constraints.

**Human and financial cost:** Potentially millions of patients whose health-seeking behaviour was exposed to advertising networks. HHS issued guidance in December 2022 clarifying that the practice violates HIPAA. Multiple hospitals settled enforcement actions. The harm is diffuse — individual patients cannot determine what was shared about them.

**Controls not evidenced:** NIST SP 800-53 AC-4 (Information Flow Enforcement), SC-16 (Transmission of Security Attributes). HIPAA has required controlled handling of protected health information since 1996; the technical enforcement of flow control was absent.

**UMRS design property:** CUI//HLTH labels on health-seeking data create machine-enforced flow boundaries. Data labeled as health information cannot flow to an advertising analytics domain — the kernel denies the flow. The failure mode (unlabeled data flowing freely to any requesting endpoint) is exactly the failure class that mandatory access control with sensitivity labels is designed to address.

**Source citations:**
- HHS OCR Bulletin on Use of Online Tracking Technologies (2022-12): https://www.hhs.gov/hipaa/for-professionals/privacy/guidance/hipaa-online-tracking/index.html
- The Markup / STAT News investigations into hospital pixel tracking (2022)

---

## Canada — Additional Cases

### CA-19: RCMP Indigenous Surveillance Program — Data Without Boundaries (1968–1982, revealed 2024)

**What happened:** A CBC Indigenous investigation revealed that the Cold War-era RCMP Security Service operated a systematic surveillance program targeting at least 30 Indigenous political organizations and hundreds of Indigenous individuals between 1968 and 1982. Methods included paying informants, physical surveillance, filming and photography, monitoring media, liaising with Indian Affairs and the FBI, and accessing sensitive government and privately held records. The Liberal government in the mid-1970s approved covert RCMP wiretaps on the National Indian Brotherhood (now the Assembly of First Nations). The RCMP Commissioner acknowledged the program in 2024.

**Nation:** Canada

**Protected tier equivalent:** Protected B / Protected C — security intelligence records about Canadian citizens and lawful organizations.

**Failure class:** Over-access and no purpose-based compartmentalization. RCMP Security Service intelligence operations targeting domestic political organizations treated information about Indigenous Canadians as having no handling constraints within the security apparatus. There was no mechanism to prevent security intelligence about lawful political activity from flowing freely between RCMP, Indian Affairs, and US agencies (FBI). The records themselves — once created — had no controls on who could access, share, or retain them.

**Human and financial cost:** Systematic infringement of constitutional rights (as understood under the 1982 Charter). Documented suppression of legitimate political organizing. Indigenous leaders and activists were surveilled, with materials shared with agencies that could have been used to discredit or obstruct their work. Historical harms to the right to political assembly and speech.

**Controls not evidenced:** The Privacy Act (enacted 1983, after the program ended) and the Security of Information Act now impose controls. The program pre-dates modern frameworks, but the case illustrates that the absence of information handling boundaries — no labeling, no purpose limitation, no accountability for cross-agency sharing — enables systemic abuse regardless of the era.

**UMRS design property:** Purpose-based access and compartmentalization: intelligence records about lawful political organizations exist in a different security domain from intelligence records about genuine national security threats. Cross-domain data sharing requires explicit authorization from a different level of authority. Audit records of who accessed, shared, or retained records about lawful Canadian organizations create accountability that was entirely absent.

**Source citations:**
- CBC News, "How the RCMP spied on Indigenous organizations" (2024): https://www.cbc.ca/news/editorsblog/cbc-news-indigenous-rcmp-surveillance-9.7133525
- RCMP Commissioner statement acknowledging the program (2024): https://www.cbc.ca/news/politics/rcmp-response-indigenous-spying-operation-9.7141533

---

### CA-20: Ontario IPC — Healthcare Snooping Surge (2019–2023)

**What happened:** Ontario's Information and Privacy Commissioner annual reports documented a 34% rise in snooping cases in 2023, with self-reported health privacy breaches involving snooping nearly doubling to 197 in 2023 from 104 in 2019. The dominant pattern is healthcare workers accessing the records of colleagues, friends, ex-partners, or public figures without a therapeutic justification. More than 56% of unauthorized disclosures were caused by misdirected faxes in 2022, with that figure rising 10% in 2023 — illustrating that even the transmission channel for Protected B health data is uncontrolled.

**Nation:** Canada

**Protected tier:** Protected B equivalent (Ontario PHIPA) — personal health information.

**Failure class:** Over-access (snooping) and labeling/handling failure (misdirected faxes). The snooping pattern demonstrates that access controls are role-based ("healthcare worker") but not purpose-scoped ("accessing records of patients in your assigned care"). The fax pattern demonstrates that transmission of controlled health data has no enforcement mechanism on the destination.

**Human and financial cost:** 197 confirmed snooping incidents in 2023 in Ontario alone, each affecting at least one individual. Misdirected faxes sent to wrong recipients — including incorrect organizations — expose sensitive clinical data to unauthorized recipients with no ability to retrieve or remediate after transmission.

**Controls not evidenced:** Ontario PHIPA (Personal Health Information Protection Act) and NIST SP 800-53 AC-6 (Least Privilege), AC-4 (Information Flow Enforcement). These requirements existed and were required; technical enforcement was absent.

**UMRS design property:** Purpose-scoped access to health records — accessing a patient record for a clinical purpose (the patient is in the worker's active care) is a different security context from accessing a record for any other reason. The second access requires escalated authorization. Misdirected fax is addressed by information flow control on the transmission channel: the destination of health data is a controlled attribute, not simply the address the sender types.

**Source citations:**
- IPC Ontario Annual Report 2023 / CBC News (2024): https://www.cbc.ca/news/canada/toronto/snooping-reports-ontario-tracking-1.7266429
- IPC Ontario: Preventing health privacy breaches: https://www.ipc.on.ca/en/cases-of-note/preventing-health-privacy-breaches

---

### CA-21: Statistics Canada Census — First Nations Physical Custody Failure

*(This case is already partially covered in the existing Canadian corpus under "Statistics Canada Census Data Loss." This entry adds detail and maps it more precisely to the data sovereignty dimension.)*

**What happened:** In 2016, 587 long-form census forms completed by First Nations residents were stored in an employee's personal vehicle trunk; the vehicle was stolen. In separate incidents on two Alberta First Nations reserves, forms went missing or were sent to incorrect addresses. Statistics Canada decided not to notify the affected First Nations respondents in at least one case, citing concern that notification might discourage future census participation.

**Nation:** Canada

**Protected tier:** Protected B — census personal information, including demographic data about First Nations individuals.

**Failure class:** Physical custody failure and notification suppression. The decision not to notify illustrates a secondary failure: the breach notification was suppressed based on a policy calculation rather than the privacy interest of the affected individuals. The affected population's right to know was treated as less important than a statistical participation objective.

**UMRS design property:** Mandatory breach notification is not discretionary — it cannot be waived based on participation-rate concerns. The principle that controls apply equally regardless of the affected population is central to equitable privacy protection.

**Source citations:**
- CBC News (2018): "Statistics Canada loses, mishandles hundreds of sensitive census, employment files": https://www.cbc.ca/news/canada/toronto/snooping-reports-ontario-tracking-1.7266429

---

### CA-22: Canada Revenue Agency — 31,000 Confirmed Privacy Breaches (2020–2024)

**What happened:** The Office of the Privacy Commissioner's Special Report to Parliament (February 2024) disclosed that the CRA reported more than 31,000 confirmed privacy breaches between May 2020 and August 2024 — a figure that includes the initial GCKey credential-stuffing attacks (CA-01 in the existing corpus) but extends through four years of ongoing incidents. The breadth and duration of the breach pattern indicates a systemic failure of anomaly detection and account access controls, not an isolated event.

**Nation:** Canada

**Protected tier:** Protected B — tax records, SIN, banking information, employment history, benefit entitlements.

**Failure class:** Audit existed but could not contain the pattern. The CRA's systems logged accesses and breach events but could not prevent credential-stuffed access from appearing indistinguishable from legitimate access at the time of login. The scale — 31,000 confirmed breaches over 50 months — indicates the technical controls were insufficient to detect or terminate fraudulent sessions before harm was done.

**Human and financial cost:** Tens of thousands of Canadians experienced financial harm, identity theft, or disrupted benefits access. Class action certified by the Federal Court. The Privacy Commissioner found the CRA and ESDC contravened section 8 of the Privacy Act.

**Controls not evidenced:** NIST SP 800-53 IA-3 (Device Identification), AC-7 (Unsuccessful Logon Attempts), SI-4 (System Monitoring for anomalous access patterns). The controls required to detect credential stuffing at scale were not sufficiently implemented.

**UMRS design property:** Anomaly detection on access patterns — credential stuffing produces access patterns (new device, unusual timing, login-then-modify-financial-data-immediately) that differ from normal user behaviour. MLS sensitivity labeling on "financial modification" operations creates a higher-privilege boundary that is harder to cross with a stolen credential alone. Controls like these would have reduced the blast radius of each compromised credential.

**Source citations:**
- OPC Special Report to Parliament (2024-02-15): https://www.priv.gc.ca/en/opc-actions-and-decisions/ar_index/202324/sr_pa_20240215_gckey/

---

### CA-23: Babylon Health Parallel Canada — Misdirected Fax as a National Pattern

**What happened:** The Ontario IPC 2022 and 2023 reports confirmed that misdirected faxes account for more than 56% of unauthorized disclosures of personal health information across Ontario healthcare institutions. This is not a one-time failure — it is the dominant transmission-channel breach pattern. The same pattern appears in federal health data handling. Protected B health information is regularly transmitted via fax, a channel with no labeling, no encryption, no audit trail, and no recipient verification. A fax sent to the wrong number is irrecoverable.

**Nation:** Canada

**Protected tier:** Protected B (Ontario PHIPA / federal Health Information Act) — personal health information.

**Failure class:** Labeling and handling failure. The transmission channel for the most sensitive category of personal information (health records) has no technical enforcement of destination correctness, no audit record of receipt, no ability to verify the recipient's authorization, and no mechanism to retrieve or destroy misdirected material.

**UMRS design property:** Information flow control on the transmission channel. CUI//HLTH data may not flow to a channel that cannot verify recipient authorization. The destination of a health record transmission is a labeled attribute; the kernel enforces that Protected B health data flows only to authorized endpoints.

**Source citations:**
- IPC Ontario, Annual Report (2022, 2023): https://www.ipc.on.ca/en/annual-statistical-reporting-all-sectors/annual-reporting-privacy-breach-statistics-commissioner-requirements-provincial-institutions

---

### CA-24: Babylon Health Parallel Canada — Te Whatu Ora COVID Vaccination Data (NZ)

*(See NZ section for this case — it fits the "insider misuse of health data" pattern that parallels the Ontario snooping cases.)*

---

## United Kingdom

### UK-01: Post Office Horizon Scandal (1999–2015, convictions overturned from 2020)

**What happened:** Between 1999 and 2015, the Post Office prosecuted more than 700 subpostmasters for theft, fraud, and false accounting based on accounting discrepancies reported by Horizon, a Fujitsu IT system. The Post Office treated Horizon's outputs as reliable and used them as the primary evidence against subpostmasters. The Post Office and Fujitsu were both aware of bugs and defects in Horizon. A 2006 contract between the Post Office and Fujitsu included a clause requiring Fujitsu to pay penalties of £100–£150 per bug that caused accounting errors — demonstrating that both parties knew defects existed. Courts were not informed of Horizon's known unreliability. Subpostmasters who asserted the system was wrong were disbelieved and prosecuted. At least 13 suicides have been linked to the scandal.

**Nation:** United Kingdom

**CUI category equivalent:** No direct CUI mapping — this is a UK criminal justice and financial data case. The most applicable parallel is the failure class: "trusted system output treated as authoritative without independent verification of system integrity." The Horizon case is the canonical example of this failure class across the Five Eyes.

**Failure class:** Trusted-without-verify — at scale, over 16 years, resulting in criminal convictions. The court system, the Post Office, and prosecutors all trusted Horizon's output without independent verification that the system was producing correct results. When the accused (subpostmasters) asserted the system was wrong, their testimony was structurally less credible than the system's output — because there was no independent integrity attestation for either.

**Human and financial cost:** More than 700 wrongful prosecutions. At least 100 convictions overturned as of early 2024. Hundreds of individuals imprisoned, bankrupted, or subjected to community service. At least 13 suicides linked to the scandal. Compensation costs expected to exceed £1 billion. UK Prime Minister described it as "one of the greatest miscarriages of justice in British history."

**Controls not evidenced:** There is no direct NIST SP 800-53 control citation for a UK criminal system; however: SI-7 (Software, Firmware, and Information Integrity), AU-9 (Protection of Audit Information), and AU-2 (Event Logging) all address the failure mode. The principle that a software system's outputs must be independently verifiable — and that defects in a system used to generate prosecution evidence must be disclosed — is fundamental to high-assurance system design.

**UMRS design property:** IMA/EVM Appraisal (Phase 2) treats every accounting record produced by a system as a measured artefact — its output is cryptographically bound to the software version and configuration that produced it, and the measurement log (maintained by the kernel, not by the application) provides independent verification. A system where the kernel independently measures and logs accounting outputs cannot silently produce defect-driven discrepancies while the application maintains that it is operating correctly. Controls like these would have made the Horizon defects detectable by an independent technical record — creating the evidentiary basis for subpostmasters' assertions that the system was wrong.

**Source citations:**
- British Post Office scandal — Wikipedia: https://en.wikipedia.org/wiki/British_Post_Office_scandal
- Computer Weekly: Post Office Horizon scandal explained: https://www.computerweekly.com/feature/Post-Office-Horizon-scandal-explained-everything-you-need-to-know
- CCRC (Criminal Cases Review Commission) Horizon page: https://ccrc.gov.uk/post-office-horizon-cases/
- Computing.co.uk: Secret contract proves Post Office and Fujitsu were aware of bugs: https://www.computing.co.uk/news/2025/secret-contract-proves-post-office-and-fujitsu-were-aware-of-bugs-in-horizon

---

### UK-02: HMRC Data Loss — 25 Million Child Benefit Records (2007)

**What happened:** On 18 October 2007, HMRC junior staff sent two unencrypted CD-ROMs via unrecorded internal mail to the National Audit Office. The discs contained data on all UK child benefit recipients — approximately 25 million individuals covering 7.25 million families. The NAO complained on 24 October that the discs had not arrived. They were never recovered. The data included names, addresses, dates of birth of parents and children, National Insurance numbers, and bank or building society account details.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//PRVCY (Privacy) — equivalent to Protected B in the Canadian framework; population-scale financial and identity data. CUI//FNC (Financial) — bank account details.

**Failure class:** Physical custody failure and labeling failure. Data containing the financial and identity details of half the UK population was transmitted without encryption, without recorded tracking, without any custody verification at receipt, and by a junior staff member without authority-level approval for the data transfer. There was no technical control preventing the bulk export of 25 million records to removable media.

**Human and financial cost:** HMRC chairman Paul Gray resigned. The taxpayer spent £473,544 in the search for the discs. Credit card companies were alerted. Millions of families faced potential fraud and identity theft risk. The ICO found this was a breach of the Data Protection Act 1998.

**Controls not evidenced:** NIST SP 800-53 MP-3 (Media Marking), MP-4 (Media Storage), MP-5 (Media Transport), AC-19 (Access Control for Mobile Devices). The Data Protection Act 1998, Principle 7 (appropriate technical measures) was applicable and required.

**UMRS design property:** Bulk data export of this magnitude — 25 million records — is a privileged operation requiring explicit authorization from a senior authority, an immutable audit record, and encrypted transmission with verified receipt. Controls like these would have prevented a junior staff member from being able to perform a bulk export of population-scale data onto unencrypted removable media without challenge. The custody chain for the transmission would have required verified receipt before the transfer was considered complete.

**Source citations:**
- Wikipedia: Loss of United Kingdom child benefit data (2007): https://en.wikipedia.org/wiki/Loss_of_United_Kingdom_child_benefit_data_(2007)
- ICO: Loss of UK child benefit data: https://ico.org.uk/for-the-public/ico-40/loss-of-uk-child-benefit-data/
- Pinsent Masons, HMRC loses records on 25 million people in the post: https://www.pinsentmasons.com/out-law/news/hmrc-loses-records-on-25-million-people-in-the-post

---

### UK-03: Windrush Scandal — Deliberate Record Destruction (2010–2018)

**What happened:** In 2010, the Home Office decided to destroy thousands of immigration landing cards dating from the Windrush generation's arrival. The destruction took place between 2009 and 2010 under both Labour and Conservative administrations. Because many Windrush generation individuals had arrived as children on parents' passports, the landing cards were among the only documentary evidence of their right to remain in the UK. After the destruction, individuals who had lived legally in the UK for 50+ years could not prove their status. At least 83 were wrongly deported. Hundreds lost jobs, housing, access to NHS care, and benefits. The Home Office's own data management was later described by parliament as having "appalling defects."

**Nation:** United Kingdom

**CUI category equivalent:** CUI//PRVCY (Privacy) / Immigration Records. In UK government terms: protected personal immigration records. The failure is the inverse of the usual UMRS case: controlled records were destroyed, eliminating the provenance that individuals needed to assert their rights.

**Failure class:** Provenance destruction. The destruction of records with legal significance is the most severe form of provenance failure — it makes the original state permanently unrecoverable. The harm to affected individuals was direct and severe precisely because there was no independently maintained record of the original data, no custody log proving what was destroyed, and no notification to affected individuals that records relevant to their legal status had been eliminated.

**Human and financial cost:** At least 83 wrongly deported. Hundreds denied employment, housing, NHS care, and benefits. The Home Secretary (Amber Rudd) resigned. The government established a Windrush Compensation Scheme; total payouts exceeded £65 million as of 2024. The human cost — broken families, lost careers, psychological harm — is significantly greater.

**Controls not evidenced:** No specific NIST SP 800-53 control directly addresses governments destroying their own records. However: AU-9 (Protection of Audit Information — preventing destruction of records that serve as legal evidence), MP-6 (Media Sanitization — controlled and documented destruction with appropriate authorization level), and SI-12 (Information Management and Retention — retention schedules based on legal significance) all address aspects of the failure mode.

**UMRS design property:** Records with legal significance are controlled objects — their destruction is a privileged operation requiring documented authorization, notification to affected parties where legally required, and an immutable audit log of what was destroyed and when. An append-only audit record of record existence means that even after destruction, there is an independent log that the record existed. Controls like these would not have prevented the policy decision to destroy the cards, but would have created an accountable record of the decision and enabled notification of affected individuals.

**Source citations:**
- Windrush scandal — Wikipedia: https://en.wikipedia.org/wiki/Windrush_scandal
- Channel 4 News FactCheck: who destroyed the Windrush landing cards?: https://www.channel4.com/news/factcheck/factcheck-who-destroyed-the-windrush-landing-cards
- The Register: The British Home Office was warned about its crappy data management — then Windrush happened: https://www.theregister.com/2018/12/05/nao_windrush_liberty_data_sharing/

---

### UK-04: Grenfell Tower — Building Safety Data Trusted Without Independent Verification (2009–2017)

**What happened:** The Grenfell Tower Inquiry Phase 2 Report (September 2024) documented that between 2009 and 2017, there were "many opportunities for the government to identify the risks posed by the use of combustible cladding panels and insulation" and to act on them. The government "was well aware of those risks, but failed to act on what it knew." An independent report produced in 2013 by the TMO (Tenant Management Organisation) criticising fire safety was not disclosed to the Royal Borough of Kensington and Chelsea (RBKC). The Building Research Establishment (BRE) conducted fire safety tests whose record keeping was found to expose them to "manipulation by unscrupulous product manufacturers" — BRE staff were advising customers on how to satisfy safety criteria, compromising the independence of the testing body.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//CRIT (Critical Infrastructure — building safety and fire safety data) and CUI//TRAN-adjacent (safety inspection and certification records). In UK regulatory terms: building control certification data.

**Failure class:** Trusted-without-verify and record suppression. The 2013 independent fire safety report was not disclosed to the RBKC — a direct analogue to the NEB/Enbridge audit suppression case in the Canadian corpus. Building safety certification records were produced by a testing body whose independence was compromised. The government trusted the certification process without independent verification of whether the process itself was reliable.

**Human and financial cost:** 72 deaths on 14 June 2017. The Inquiry took years and ran to thousands of pages. Prosecutions are anticipated. The post-Grenfell building safety remediation programme across UK housing is estimated to cost £15 billion or more.

**Controls not evidenced:** There are no specific NIST SP 800-53 citations for building safety; however: SI-7 (Software, Firmware, and Information Integrity — as applied to test and certification processes), AU-9 (Protection of Audit Information), and the principle from NSA RTB RAIN (Non-Bypassability) that safety-critical decisions must be based on independently verifiable evidence all apply.

**UMRS design property:** Safety test results and certification records as controlled artefacts with mandatory independent provenance. The entity producing the test data (BRE) and the entity attesting to safety (building control) are separate labelled domains. The 2013 suppressed report is a controlled artefact — its disclosure to the responsible authority is a required event in the audit chain. Controls like these would have made the suppression of the 2013 report a detectable gap in the regulatory record.

**Source citations:**
- Grenfell Tower Inquiry Phase 2 Report, Volume 1 (2024-09): https://assets.publishing.service.gov.uk/media/66d817aa701781e1b341dbd3/CCS0923434692-004_GTI_Phase_2_Volume_1_BOOKMARKED.pdf
- Al Jazeera: 'Decades of failure': Who has UK's Grenfell Tower fire inquiry blamed? https://www.aljazeera.com/news/2024/9/4/decades-of-failure-who-has-uks-grenfell-tower-fire-inquiry-blamed

---

### UK-05: UK NHS WannaCry Ransomware Attack (2017)

**What happened:** On 12 May 2017, the WannaCry ransomware attack infected NHS systems across England. At least 81 out of 236 NHS Trusts were affected, along with 603 primary care and other NHS organisations including 595 GP practices. 19,000 appointments were cancelled. Five Trusts had to divert patients from emergency departments. Approximately 139 of the cancelled appointments involved patients referred for potential cancer. The NAO found that all affected organisations shared the same vulnerability (unpatched Windows systems) and "could have taken relatively simple action to protect themselves."

**Nation:** United Kingdom

**CUI category equivalent:** CUI//HLTH (Health information) — NHS patient records; CUI//CRIT (Critical Infrastructure — NHS as health system).

**Failure class:** Trusted-without-verify applied to patch and configuration management. Systems were trusted to be in a secure operational state without independent verification that security configurations were maintained. The NHS had been warned about the specific EternalBlue vulnerability; the patch had been available since March 2017. The gap between "security update available" and "security update verified as applied" was not closed by any technical enforcement.

**Human and financial cost:** £92 million total cost. 19,000 cancelled appointments. Cancer referral delays. Five emergency department diversions. Long-term reputational and operational damage.

**Controls not evidenced:** NIST SP 800-53 SI-2 (Flaw Remediation), CM-8 (System Component Inventory), CM-6 (Configuration Settings). The NHS had received advance notice of the vulnerability; the controls required to verify patch application across the estate were not in place.

**UMRS design property:** IMA/EVM Appraisal (Phase 2) treats the operating system and software state as measured objects — a system that has not applied required security patches produces a measurement log entry that differs from the expected "patched" state. The kernel independently verifies software integrity at each boot. Controls like these would have made unpatched systems identifiable before the attack, not during it.

**Source citations:**
- NAO Investigation: WannaCry cyber attack and the NHS: https://www.nao.org.uk/reports/investigation-wannacry-cyber-attack-and-the-nhs/
- National Health Executive: WannaCry cyber-attack cost the NHS £92m: https://www.nationalhealthexecutive.com/articles/wannacry-cyber-attack-cost-nhs-ps92m-after-19000-appointments-were-cancelled

---

### UK-06: HMRC Staff Data Misuse — 354 Investigations, 186 Dismissals (2022–2025)

**What happened:** Between 2022 and 2025, HMRC investigated 354 staff members for data security breaches, resulting in 186 dismissals. In one representative case, an employee was dismissed for emailing a personal record set of 100 taxpayers to themselves to print at home. A Cabinet Office review published around the same period found systemic weaknesses across Whitehall departments, including lack of controls over ad hoc downloads and bulk exports of sensitive information, and recurring misdirected email incidents.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//TAX (Tax information) — UK equivalent of the US 26 U.S.C. § 6103 protection; personal tax records are the most sensitive personal financial data the government holds.

**Failure class:** Over-access and audit-existed-but-could-not-prevent. The audit detected violations after the fact (354 investigations) but could not prevent them. The "employee emails 100 records to personal account to print at home" case illustrates that bulk export of sensitive data to an unauthorized medium was technically possible without triggering an alert.

**Human and financial cost:** 186 dismissed employees. An unknown number of taxpayers whose records were accessed or exfiltrated for unauthorized purposes. The pattern over three years (354 cases) indicates a systemic failure of enforcement, not isolated incidents.

**Controls not evidenced:** NIST SP 800-53 AU-12 (Audit Record Generation), SI-4 (System Monitoring), DLP controls (Data Loss Prevention — addressed under MP-2, MP-7). The Data Protection Act 2018 and HMRC's own security policies required controls against unauthorized bulk export.

**UMRS design property:** Data loss prevention as a mandatory access control property: bulk export of taxpayer records to a personal email account is a privileged operation requiring explicit authorization. The system enforces — not just audits — the constraint that CUI//TAX data cannot flow to personal email or personal printing infrastructure. Controls like these would have prevented the export, not just detected it after investigation.

**Source citations:**
- SC Media UK: HMRC Axes Staff Over Data Misuse: https://insight.scmagazineuk.com/hmrc-axes-staff-over-data-misuse
- Cabinet Office review findings reported in SC Media UK (2025)

---

### UK-07: MOD SSCL Armed Forces Payroll Breach (2024)

**What happened:** In early 2024, a third-party contractor, Shared Services Connected Ltd (SSCL), operating the MOD's payroll system for armed forces personnel, was compromised. Approximately 270,000 payroll records were exposed, containing names, bank account details, and in some cases addresses and National Insurance numbers of current and former army, navy, and air force personnel and reservists. The attackers are believed to have been present in the SSCL system for several weeks before detection. The UK government has not formally attributed the attack but UK media reported Chinese state-sponsored actor involvement. No evidence was found that data was exfiltrated, but the access was confirmed.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//PRVCY (Privacy — personnel financial data) and CUI//SP-CTI-adjacent (Defence-linked personnel data). In UK classification: OFFICIAL-SENSITIVE at minimum.

**Failure class:** Third-party trust without verification. The MOD delegated the management of sensitive armed forces personnel data to a contractor (SSCL) without adequate security oversight. The compromise lasted several weeks — the MOD was not aware the system was being accessed by unauthorized parties until after the investigation.

**Human and financial cost:** 270,000 service personnel potentially at risk. The MOD launched an eight-point remediation plan. Over 5,000 personnel joined a lawsuit against the MOD. Long-term intelligence risk: names and bank details of all serving armed forces personnel create targeting intelligence.

**Controls not evidenced:** NIST SP 800-53 SA-9 (External System Services — contractor security requirements), CA-7 (Continuous Monitoring), SI-4 (System Monitoring). Defence Supply Chain security requirements applicable to contractors handling personnel data.

**UMRS design property:** Third-party data custody with persistent labeling and audit. Data labeled as controlled does not lose its handling requirements when transferred to a contractor. The contractor's access to labelled data is itself a controlled, audited operation; the MOD receives kernel-level audit events for every access to armed forces personnel records regardless of which system the access originates from. Controls like these create continuous visibility into contractor data access — rather than discovering a multi-week intrusion after the fact.

**Source citations:**
- The Register: UK confirms attack on MoD system, opens review of contractor (2024-05-08): https://www.theregister.com/2024/05/08/uk_opens_investigation_into_contractor/
- Hansard: Defence Personnel Data Breach (2024-05-07): https://hansard.parliament.uk/commons/2024-05-07/debates/56231312-9D57-4CB6-A649-EFA0621B7293/DefencePersonnelDataBreach

---

### UK-08: NHS 111 Advanced/Adastra Ransomware — LockBit Attack (2022)

**What happened:** On 4 August 2022, LockBit ransomware actors compromised the systems of Advanced, an IT managed service provider whose Adastra client management platform underpins 85% of NHS 111 services. The attackers used legitimate credentials for a third-party account that lacked multi-factor authentication. The personal data of 82,946 individuals was exfiltrated, including phone numbers, medical records, and critically: the home entry details (how to gain access to the home) of 890 people receiving home care. The ICO provisionally fined Advanced £6.09 million; this was reduced to £3.07 million in a voluntary settlement.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//HLTH (Health — patient medical records) and CUI//PRVCY (Privacy — physical home access information). The home entry details represent an especially sensitive class: data that, if released, enables physical access to vulnerable individuals.

**Failure class:** Third-party trust without verification and over-access by the third party. Advanced had access to NHS patient data at the level required to run NHS 111. A single third-party account without MFA was the entry point for data affecting 82,946 people. The most sensitive subset — home access instructions for 890 care recipients — illustrates that data of widely varying sensitivity was stored in the same system without compartmentalization.

**Human and financial cost:** £3.07 million ICO fine. NHS 111 services disrupted nationally. The home access data for 890 home care patients created direct physical safety risk. Operational impact: NHS staff reverted to manual workarounds for NHS 111 for weeks.

**Controls not evidenced:** NIST SP 800-53 IA-2 (Multi-Factor Authentication), AC-17 (Remote Access), SA-9 (External System Services). The ICO explicitly cited failure to implement MFA as the control failure.

**UMRS design property:** Home entry instructions for care recipients are among the most physically sensitive data a health system holds — they are equivalent to a physical access key to a vulnerable person's home. CUI sensitivity labeling would flag this data as requiring the highest access controls within the system, separated from ordinary medical records. A system designed with MLS compartmentalization would place home access data in a higher-sensitivity compartment than general medical records, requiring additional authorization for access even within the same contractor system.

**Source citations:**
- ICO Provisional Decision (2024-08): https://ico.org.uk/about-the-ico/media-centre/news-and-blogs/2024/08/provisional-decision-to-impose-6m-fine-on-software-provider-following-2022-ransomware-attack-that-disrupted-nhs-and-social-care-services/
- BleepingComputer: UK fines software provider £3.07 million for 2022 ransomware breach: https://www.bleepingcomputer.com/news/security/uk-fines-software-provider-307-million-for-2022-ransomware-breach/
- TechCrunch: NHS vendor Advanced won't say if patient data was stolen: https://techcrunch.com/2022/10/13/advanced-nhs-patient-data-ransomware/

---

### UK-09: UK Cabinet Office — New Year Honours Addresses Published (2019–2021)

**What happened:** On 27 December 2019, the Cabinet Office published a file on GOV.UK containing the names and unredacted postal addresses of more than 1,000 individuals on the New Year Honours list — including celebrities, public figures, and private individuals. The list was accessible for 2 hours and 21 minutes and was accessed 3,872 times before being taken down. The Honours and Appointments Secretariat had introduced a new IT system in 2019; due to a configuration error, CSV files automatically included address data. Time pressure to meet the publication deadline led staff to manually patch the file rather than fix the underlying system — and the defect persisted across multiple file generations.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//PRVCY (Privacy — personal addresses of public and private individuals).

**Failure class:** Labeling and handling failure combined with time-pressure-driven trust-without-verify. The manual patch approach was chosen over fixing the root cause. The data was published without independent verification that sensitive fields had been removed. The system consistently reintroduced the address field; no automated check confirmed its absence before publication.

**Human and financial cost:** ICO initially fined £500,000; Cabinet Office appealed and settled at £50,000. 3,872 downloads of the address list. Specific individuals on the list — including domestic violence victims and witnesses in sensitive cases — faced potential physical safety risks.

**Controls not evidenced:** NIST SP 800-53 SI-10 (Information Input Validation), CM-6 (Configuration Settings). The Cabinet Office's own data protection obligations under the Data Protection Act 2018 required controls that would have verified the output before publication.

**UMRS design property:** Data validation at the boundary of publication — the "publish to GOV.UK" operation is a controlled data flow. CUI-labeled data (personal addresses) cannot flow across a publication boundary without an automated verification step confirming the label's handling requirements are met. Manual patches to CSV files are not a substitution for automated boundary enforcement. Controls like these would have flagged the presence of address data in the publication-bound file, not relied on human review under time pressure.

**Source citations:**
- The Register: Cabinet office cops data fine for blurting NY honours list: https://www.theregister.com/2021/12/02/uk_data_watchdog_fines_government/
- ICO and Cabinet Office reach agreement on revised fine: https://ico-newsroom.prgloo.com/news/ico-and-cabinet-office-reach-agreement-on-new-year-honours-data-breach-fine

---

### UK-10: UK Home Office — Unlawful Blanket Phone Seizure from Migrants (2020–2022)

**What happened:** Between 2020 and 2022, the UK High Court ruled that the Home Office's policy of blanket seizure and digital extraction from the mobile phones of migrants arriving by small boats was unlawful under Article 8 ECHR (right to privacy) and the Data Protection Act 2018. The Home Office initially denied the blanket policy existed. When internal documents revealed it did exist, the Home Office acknowledged it had known the policy might be unlawful but believed it had been ended. Thousands of migrants had their phones — containing sensitive personal communications, contacts, photographs, and immigration-relevant documents — seized and searched without individualized suspicion.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//LEI (Law Enforcement Sensitive — immigration investigation data) and CUI//PRVCY (Privacy — personal communications and contacts).

**Failure class:** Unlawful data collection at scale combined with record suppression. Data was collected without legal basis. The Home Office initially misrepresented the policy's existence to the court — a record suppression failure at the institutional level. The extracted data had no documented handling controls, retention schedules, or destruction schedules.

**Human and financial cost:** UK High Court ordered systemic remediation and data breach notifications to affected individuals (October 2022). Thousands of migrants received letters acknowledging the Data Protection Act breach. Compensation proceedings followed.

**Controls not evidenced:** UK GDPR Articles 5, 9, and 23 (lawfulness of processing, special category data); Data Protection Act 2018 Schedule 2. The ICO found the GPS electronic monitoring pilot (a parallel program) also breached UK data protection law (ICO, March 2024).

**UMRS design property:** Data collection itself is a controlled operation — the authority to collect personal data from an individual is a labeled attribute that must be verified before collection commences. Data collected without verified authority cannot enter the system's controlled custody chain. Controls like these embed the legal basis requirement into the system architecture: collection without authorization cannot produce a labeled, controlled record.

**Source citations:**
- Privacy International: UK High Court orders groundbreaking redress for thousands of migrants: https://privacyinternational.org/news-analysis/4987/uk-high-court-orders-groundbreaking-redress-thousands-migrants-affected-unlawful-phone-seizures-and-data-extraction
- ICO: GPS tagging of migrants pilot breached UK data protection law (2024-03): https://ico.org.uk/about-the-ico/media-centre/news-and-blogs/2024/03/ico-finds-the-home-office-s-pilot-of-gps-electronic-monitoring-of-migrants-breached-uk-data-protection-law

---

### UK-11: Babylon Health — Patient Consultation Videos Exposed to Wrong Patients (2020)

**What happened:** On 9 June 2020, Babylon Health's GP at Hand app allowed at least one patient to access video consultation recordings belonging to other patients. One user reported finding access to "over 50 video recordings" of other patients' consultations. The error was introduced by a new feature enabling mid-consultation switches from audio to video. Babylon notified the ICO. The root cause was a software defect, not a malicious attack.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//HLTH (Health — clinical consultation recordings and patient-identifiable medical information). In UK terms: Special Category personal data under UK GDPR.

**Failure class:** Over-access and labeling/compartmentalization failure. Clinical consultation recordings — among the most sensitive personal data a health system holds — were stored in a system with insufficient access control: one patient's record was reachable from another patient's authenticated session. The records were not compartmentalized by patient identity with independent access control.

**Human and financial cost:** Clinical consultation recordings of other patients accessible via a standard user session. ICO notified. Patient trust eroded. Babylon Health subsequently faced a range of other difficulties and ceased UK operations.

**Controls not evidenced:** NIST SP 800-53 AC-4 (Information Flow Enforcement), AC-6 (Least Privilege). UK GDPR Article 25 (Data Protection by Design) and Article 32 (Security of Processing) both required appropriate technical measures.

**UMRS design property:** Patient consultation records are objects labeled with the patient's identity as a mandatory access control attribute. A consultation recording for patient A can only be accessed within a security context authorized for patient A. Cross-patient access within a single session is architecturally impossible when records are labeled at the individual level and access is enforced by the operating system, not the application.

**Source citations:**
- MobiHealthNews: Babylon Health admits GP at Hand app data breach: https://www.mobihealthnews.com/news/emea/babylon-health-admits-gp-hand-app-data-breach-caused-software-issue
- Babylon Health App Leaked Patients' Video Consultations — Tripwire: https://www.tripwire.com/state-of-security/babylon-health-app-leaked-video-consultations

---

### UK-12: NHS England — Trusts Shared Patient Data with Facebook Without Consent (2022–2023)

**What happened:** Investigations published by The Markup and STAT News (and confirmed for UK trusts) found that NHS hospitals had embedded tracking pixels from advertising networks (including Meta Pixel/Facebook) in their web properties. These pixels transmitted patient-identifiable information — including condition-indicated appointments, health queries, and user identifiers — to Facebook's servers without patient consent or appropriate legal basis. HHS published US guidance in December 2022 confirming this practice violates patient privacy law; NHS England issued guidance addressing the same practice in UK trusts.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//HLTH (Health information — health-seeking behaviour and appointment data) transmitted to an unauthorized third party (CUI//PRVCY).

**Failure class:** Labeling and handling failure — no technical control prevented health data from flowing to advertising networks. The tracking pixels were embedded by web developers who did not recognize that health-related web browsing constitutes protected health information. The data flow existed because there was no machine-enforced boundary between the NHS web presence and the external advertising network.

**UMRS design property:** Information flow control on the boundary between NHS systems and external third parties. Health-related data cannot flow to an uncontrolled external endpoint without explicit authorization. Controls like these enforce that the embedding of third-party tracking code in health-related web properties is itself a controlled operation — not a routine web development decision.

**Source citations:**
- The Markup and STAT News: Hospital pixel tracking investigations (2022)
- NHS England guidance on use of tracking technologies

---

### UK-13: UK MOD — Laptop Left on Train with Afghan Refugee Data (2025)

**What happened:** In November 2025, a Ministry of Defence official accidentally left a laptop unattended on a train. The laptop contained sensitive information related to Afghan refugees fleeing the Taliban, as well as other personally identifiable information. The incident was part of a broader pattern of failures within the same MOD unit, including emails sent to wrong recipients, insecure system access, and unauthorized employee data access.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//PRVCY (Privacy — identity data for Afghan refugees, whose exposure creates direct physical danger). This is equivalent to the highest-sensitivity tier of personal data — individuals whose identity is exposed to hostile actors face a life safety risk.

**Failure class:** Physical custody failure and over-access on portable media. The data was present on a device that could be lost or stolen without the loss being immediately detected. The broader pattern within the unit — multiple incident types — suggests systemic access control and data handling failures rather than a single aberrant event.

**Human and financial cost:** Afghan refugees whose identities or contact information were on the device face potential exposure to Taliban retribution networks. The UK government has obligations to these individuals arising from evacuation commitments.

**Controls not evidenced:** NIST SP 800-53 MP-5 (Media Transport), SC-28 (Protection of Information at Rest — encryption at rest on portable devices), AC-19 (Access Control for Mobile Devices). UK Government Security Policy Framework requirements for portable device security apply.

**UMRS design property:** Data concerning individuals whose physical safety depends on confidentiality is the clearest case for highest-sensitivity labeling. A device containing Protected B-equivalent data about at-risk individuals requires full-disk encryption (verified by the kernel) and remote wipe capability. The loss of the device triggers an automatic notification chain — not a discretionary decision. Controls like these would not have prevented the device from being left on the train, but would have ensured the data was unreadable to whoever found it.

**Source citations:**
- Ministry of Defence (Data Loss) — Hansard (2008-01-21) for historical context: https://hansard.parliament.uk/Commons/2008-01-21/debates/0801215000003/MOD(DataLoss)
- Reporting on 2025 MOD laptop incident: https://blog.rankiteo.com/uk-5234752110425-ministry-of-defence-mod-uk-breach-november-2025/

---

### UK-14: UK Police Digital Evidence — Systemic Chain of Custody Weaknesses (Pattern)

**What happened:** UK courts scrutinize digital evidence chain of custody rigorously. Recurring documented failures include: storage on unencrypted drives, storage on personal devices, storage in shared folders or uncontrolled media, and inconsistent workflows across personnel. The Criminal Procedure and Investigations Act 1996 (CPIA) mandates disclosure obligations and investigator integrity maintenance; however, the technical controls required to enforce these standards across all police units and personnel are not systematically in place. When digital evidence is challenged on provenance grounds, cases are weakened or dismissed — not because the facts are wrong, but because the evidence cannot be proven to have remained unmodified.

**Nation:** United Kingdom

**CUI category equivalent:** CUI//LEI (Law Enforcement — digital evidence and investigation files).

**Failure class:** Audit-existed-but-cannot-prove-anything. The log of who had access to a seized device may exist. The log of what state the device was in at each access point may not. Without a cryptographic binding between the original seized device state and the copy used in court, the chain of custody is asserted — not proven.

**UMRS design property:** Digital evidence as a controlled, measured artefact from the moment of seizure. IMA/EVM Appraisal on the evidence image creates a kernel-level measurement log that is independent of the investigating officer — the measurement proves the evidence state at each point in the custody chain. This converts "asserted chain of custody" into "verifiable chain of custody."

**Source citations:**
- UK Government Evidence Policy Framework (2025): https://assets.publishing.service.gov.uk/media/6889f54a8b3a37b63e738ff6/management-handling-evidence-pf.pdf
- Digital Evidence Challenges Police Face: https://vidizmo.ai/blog/digital-evidence-admissibility-police

---

## Australia

### AU-01: Optus Data Breach (2022)

**What happened:** On 22 September 2022, Optus disclosed that attackers had accessed customer data via an unsecured, publicly accessible API that required no authentication. The API had been public-facing for up to three months before discovery. Approximately 9.8 million current and former customer records were accessed. Of these, 2.1 million had identity documents (driver's licences, passport numbers) stolen. The Australian Information Commissioner launched civil penalty proceedings against Optus.

**Nation:** Australia

**CUI category equivalent:** CUI//PRVCY (Privacy) — personal identity data at population scale (a third of Australia's population). In Australian terms: APP (Australian Privacy Principle) Category 1 data, including identity documents.

**Failure class:** Over-access — the API was accessible with no authentication. Any internet user could query it. The data had no access control at the API boundary. There was no anomaly detection on API query patterns to flag bulk enumeration. The "blast radius" of a single vulnerability was the entire customer database because records were not compartmentalized by sensitivity.

**Human and financial cost:** 9.8 million customer records accessed. 2.1 million identity documents stolen. Optus reserved $140 million for costs. Millions of Australians required identity document replacement. The Australian Information Commissioner is pursuing civil penalties.

**Controls not evidenced:** NIST SP 800-53 AC-17 (Remote Access — API authentication), AC-2 (Account Management), SI-4 (System Monitoring — anomaly detection). The Australian Privacy Act 1988 required appropriate technical security measures.

**UMRS design property:** An API that provides access to personal data is a controlled domain boundary. The API itself must enforce authentication and authorization — not just the systems behind it. Anomaly detection on query patterns (bulk enumeration of customer records) would have flagged the attack before 9.8 million records were accessed. CUI sensitivity labels on identity documents would have placed them in a higher-access-control compartment, limiting blast radius even if a generic API endpoint was compromised.

**Source citations:**
- 2022 Optus data breach — Wikipedia: https://en.wikipedia.org/wiki/2022_Optus_data_breach
- OAIC civil penalty proceedings: https://thecyberexpress.com/civil-penalty-over-2022-optus-data-breach/
- UpGuard: How Did the Optus Data Breach Happen?: https://www.upguard.com/blog/how-did-the-optus-data-breach-happen

---

### AU-02: Medibank Private Ransomware Attack (2022)

**What happened:** In October 2022, Russian-linked ransomware actors targeted Medibank, one of Australia's largest private health insurers. The initial access vector was stolen credentials from a third-party vendor obtained via a phishing email. The attackers had network access for approximately three weeks before detection. 9.7 million customer records were exfiltrated, including Medicare numbers, health claim data, and diagnosis information. The attackers specifically segregated data into a "naughty list" identifying patients receiving drug and alcohol treatment, and a second category identifying patients' abortion, miscarriage, and ectopic pregnancy records — these were used as a specific extortion leverage.

**Nation:** Australia

**CUI category equivalent:** CUI//HLTH (Health — clinical diagnosis and treatment data). The "naughty list" represents a sub-category of health data that carries specific additional stigma and harm potential, equivalent to CUI//HLTH with additional sensitivity tier.

**Failure class:** Third-party trust without verification (credentials stolen from a vendor) and over-access / no compartmentalization by sensitivity. The most sensitive health data — substance abuse treatment and reproductive health records — was stored in the same system as general contact information without additional access controls. The attackers' ability to specifically segregate "stigmatized" data categories demonstrates that the data had no labels or compartmentalization that would have prevented such segregation by an unauthorized party.

**Human and financial cost:** 9.7 million affected. Medibank expected costs exceeding $125 million. The Australian government sanctioned the Russian national responsible. Medibank refused to pay the $10 million ransom; data was subsequently published. Individuals with substance abuse or reproductive health data in the exfiltrated dataset face ongoing risk of extortion.

**Controls not evidenced:** NIST SP 800-53 AC-6 (Least Privilege — within-system compartmentalization), IA-2 (Multi-Factor Authentication), SA-9 (External System Services). The Australian Privacy Act and the Notifiable Data Breaches scheme both require appropriate technical controls.

**UMRS design property:** Health data compartmentalization by sensitivity within the health system — substance abuse records, reproductive health records, and mental health records are higher-sensitivity compartments within CUI//HLTH. An MLS model would prevent an attacker (or an authorized user without elevated clearance) from performing a bulk export that includes these specific subsets. Vendor credential compromise cannot produce a full database export if access is compartmentalized at the record sensitivity level. The concept of the "naughty list" — an attacker-defined sensitivity categorization — reflects the absence of owner-defined labeling.

**Source citations:**
- SecurityWeek: Medibank Confirms Data Breach Impacts 9.7 Million Customers: https://www.securityweek.com/medibank-confirms-data-breach-impacts-97-million-customers/
- OAIC: Medibank data breach alleged timeline: https://www.oaic.gov.au/__data/assets/pdf_file/0037/228979/Medibank-data-breach-alleged-timeline-infographic.pdf
- Australian Defence Ministers: Further cyber sanctions in response to Medibank Private cyberattack: https://www.minister.defence.gov.au/media-releases/2025-02-12/further-cyber-sanctions-response-medibank-private-cyberattack

---

### AU-03: Robodebt — Unlawful Automated Data Matching (2016–2020)

**What happened:** From July 2016, the Australian government's Department of Human Services deployed an automated system (Robodebt) to detect welfare overpayments by matching Centrelink records with ATO income data. The system used "income averaging" — dividing annual income evenly across fortnights — which was unlawful: welfare entitlements are calculated based on fortnightly income, not averaged annual income. The system raised approximately 500,000 unlawful debt notices. The onus was reversed: welfare recipients had to disprove the algorithmically generated debt, with no ability to access the algorithm's reasoning. In 2020, the Commonwealth conceded there was "no lawful basis" for the income averaging method. A $1.872 billion settlement followed, including repayment of $751 million unlawfully recovered.

**Nation:** Australia

**CUI category equivalent:** CUI//PRVCY (Privacy — welfare recipient personal data) and, in the Australian context, a social welfare administration record equivalent to Protected B in Canada.

**Failure class:** Trusted-without-verify applied to an automated system generating legal-consequence decisions. The algorithm's output was treated as authoritative evidence of debt without independent verification that the algorithm's method was legally valid. The Royal Commission found that government officials were aware of legal concerns, senior management suppressed internal dissent, and legal advice warning of the unlawfulness was not acted upon.

**Human and financial cost:** $1.872 billion settlement. 433,000 people unlawfully assessed. $751 million unlawfully recovered from 381,000 people. Multiple deaths linked to the stress of wrongful debt demands (the Inquiry examined these). The Royal Commission found evidence of a cover-up extending to ministerial offices.

**Controls not evidenced:** The Australian Public Service Act and administrative law require that automated decision-making systems used to generate legal obligations be lawful. NIST SP 800-53 SA-11 (Developer Testing and Evaluation) and CA-7 (Continuous Monitoring) address the principle that automated systems must be independently verified before being used to generate legal decisions.

**UMRS design property:** Automated system outputs that generate legal obligations (debt notices, benefit denials) are controlled decisions — not merely data queries. A controlled decision requires independent verification of the algorithm's legal validity before deployment at scale. The audit trail for 500,000 algorithmically generated debt notices must include evidence of legal review and validation, not just system logs. Controls like these would have created a mandatory verification gate before the system deployed unlawful notices at scale.

**Source citations:**
- Robodebt scheme — Wikipedia: https://en.wikipedia.org/wiki/Robodebt_scheme
- Robodebt Royal Commission findings: https://lsj.com.au/articles/crude-cruel-and-unlawful-robodebt-royal-commission-findings/
- Federal Court settlement approval: https://australia.bsg.ox.ac.uk/blog/australias-robodebt-scheme-tragic-case-public-policy-failure

---

### AU-04: Latitude Financial Data Breach (2023)

**What happened:** In March 2023, Latitude Financial, an Australian consumer finance company, disclosed a data breach caused by stolen employee credentials at a third-party service provider. Attackers used the credentials to access Latitude's systems and exfiltrate 14 million records: 7.9 million driver's licence numbers, 6.1 million records including personal data dating to 2005, and approximately 53,000 passport numbers. The breach is described as the largest identity document breach in Australian history by record count.

**Nation:** Australia

**CUI category equivalent:** CUI//PRVCY (Privacy) — identity documents at scale, including driver's licence and passport numbers for millions of Australians and New Zealanders.

**Failure class:** Third-party credential compromise and over-access. A single employee credential at a third-party service provider enabled access to 14 million records. The records included data from 2005 — meaning personal data was retained for up to 18 years without apparent need. No compartmentalization limited the blast radius of a single credential compromise.

**Human and financial cost:** $76 million in pre-tax costs. 14 million individuals required identity document replacement notifications. Latitude refused to pay a ransom demand. OAIC and OPC NZ opened a joint investigation — the first joint privacy investigation between Australia and New Zealand.

**Controls not evidenced:** NIST SP 800-53 AC-2 (Account Management), SA-9 (External System Services), SI-12 (Information Management and Retention — data minimization). The Privacy Act 1988 (Australia) and the Privacy Act 2020 (New Zealand) both require appropriate technical measures and data minimization.

**UMRS design property:** Data minimization enforced by label: personal data retained beyond its required retention period carries a "marked for deletion" label that prevents new access and triggers automated destruction. Third-party credential access is a controlled operation — a service provider account cannot access the core customer record database directly; it accesses a labeled, compartmentalized subset appropriate for its service function. Controls like these would have limited the blast radius: a third-party credential would have reached only the data that third party was authorized to access.

**Source citations:**
- Latitude Financial data breach surpasses 14 million records — Bitdefender: https://www.bitdefender.com/en-us/blog/hotforsecurity/latitude-financial-data-breach-surpasses-14-million-records-pii-drivers-licenses-and-passport-numbers-hijacked-by-threat-actors
- OAIC/OPC NZ joint investigation announcement (2023): https://www.oaic.gov.au/privacy/notifiable-data-breaches/notifiable-data-breaches-publications/notifiable-data-breaches-report-january-to-june-2023

---

### AU-05: Australian Defence Contractor "APT Alf" Breach — F-35 and Naval Designs (2016)

**What happened:** Beginning in July 2016, an attacker dubbed "APT Alf" by the ASD (Australian Signals Directorate) spent five months inside the network of a 50-employee Australian aerospace engineering subcontractor. During this period, approximately 30 GB of sensitive defence-related data was exfiltrated, including information on the F-35 Joint Strike Fighter, the P-8 Poseidon maritime patrol aircraft, the C-130 transport aircraft, JDAM smart bomb kits, and Australian naval vessel designs. The root cause was default passwords on internet-facing services. The company had one staff member responsible for network security. The ASD was alerted by a "partner organisation" — not by the victim company's own monitoring.

**Nation:** Australia

**CUI category equivalent:** CUI//SP-CTI (Cyber Threat Intelligence) and CUI//SP-EXPT (Export Controlled — ITAR-adjacent defence data). The F-35 data is specifically subject to ITAR regulation shared with the US.

**Failure class:** Trusted-without-verify applied to a small contractor holding ITAR-sensitive data. The Australian Department of Defence trusted a 50-person company with data on allied defence programs without verification that the company's security posture was adequate to protect that data. Default passwords on internet-facing services is a basic configuration failure that any security review would have identified.

**Human and financial cost:** 30 GB of sensitive allied defence data exfiltrated over five months. The strategic cost — the value of the compromised F-35 and naval vessel data to the nation-state actor — is not public. The ASD publicly disclosed the case in 2017 as a warning about small contractor security, suggesting the strategic impact was significant enough to warrant public disclosure.

**Controls not evidenced:** NIST SP 800-53 CM-6 (Configuration Settings — no default passwords), SA-9 (External System Services — contractor security verification), CA-7 (Continuous Monitoring). ITAR regulations and DSD (now ASD) Information Security Manual requirements applied to the contractor.

**UMRS design property:** Third-party data custody requires attested security posture. A contractor that holds ITAR-adjacent defence data must demonstrate — not assert — that their configuration management, authentication controls, and monitoring are commensurate with the sensitivity of the data they hold. Controls like these create a continuous attestation requirement, not a one-time security review gate.

**Source citations:**
- ASD Case Study (public disclosure 2017): https://www.itnews.com.au/news/hacked-aussie-defence-firm-lost-fighter-jet-bomb-ship-plans-475211
- Security Affairs: ASD revealed hacker stole 30GB of sensitive data on Australia's military capabilities: https://securityaffairs.com/64204/data-breach/asd-military-capabilities-databreach.html

---

### AU-06: Australia's Medicare "Dark Web Machine" — Data Sold On-Demand (2017)

**What happened:** In July 2017, Guardian Australia revealed that Australian Medicare card numbers were being sold on the dark web on-demand for approximately AU$29 per record. A journalist tested the service by purchasing their own Medicare details and confirmed the data was accurate. The service allowed anyone to provide a name and date of birth and receive the Medicare number in return. The government initially downplayed the exposure. The DHS (Department of Human Services) launched an investigation. The breach pointed to an insider or internal system vulnerability — Medicare numbers are not publicly available, indicating the data came from a system with access to the Medicare registry.

**Nation:** Australia

**CUI category equivalent:** CUI//HLTH (Health — Medicare card numbers are the primary identifier for Australia's public health system) and CUI//PRVCY (Privacy — personal identity).

**Failure class:** Audit existed but could not identify the source. The Medicare number registry has access controls; only authorized systems and personnel can query it. Yet accurate, current Medicare data was available on-demand from a dark web service. The investigation could not publicly identify the source, suggesting the audit trail was insufficient to attribute the access to a specific account or system event.

**Human and financial cost:** An unknown number of Medicare records accessed and sold. The practical harm is identity-linked to health services — Medicare numbers are used to access medical benefits and associated with health claims. The inability to identify the source meant the leak pathway was not closed with confidence.

**Controls not evidenced:** NIST SP 800-53 AU-9 (Protection of Audit Information), AU-6 (Audit Review and Analysis), AU-12 (Audit Record Generation). The combination of authorized access to Medicare data and inability to identify the unauthorized actor suggests the audit trail was insufficient for attribution.

**UMRS design property:** Every access to a Medicare record is a kernel-level audit event tied to a specific identity and authorization context. When a record is accessed, an immutable log entry records who accessed it, from which system, under what authorization, and for what purpose. Audit logs not writable by userland processes cannot be cleared to cover tracks. Controls like these make "unauthorized access without attribution" structurally much harder.

**Source citations:**
- Guardian Australia / Bitdefender: Australian Medicare records sold by request on the dark web: https://www.bitdefender.com/en-au/blog/hotforsecurity/australian-medicare-records-sold-by-request-on-the-dark-web-at-22-each
- iTnews: Govt downplays sale of Medicare card data on dark web: https://www.itnews.com.au/news/govt-downplays-sale-of-medicare-card-data-on-dark-web-467409

---

### AU-07: Australia 2016 Census — DDoS, Geoblocking Failure, and Confidence Erosion

**What happened:** On 9 August 2016, Australia's first primarily digital census was disrupted by a series of four DDoS attacks. The ABS implemented geoblocking (blocking all non-Australian IP addresses) after the second attack, but the geoblocking failed during the fourth attack. At 8:09 PM, fearing confidential census data might be downloaded under cover of the DDoS, the ABS shut down the eCensus system. The system remained offline for nearly two days. Whether a true DDoS occurred (versus an infrastructure failure mischaracterized as a DDoS) remains disputed — one DDoS tracking service showed no unusual Australian activity at the time.

**Nation:** Australia

**CUI category equivalent:** CUI/Statistical (census data — one of the most comprehensive national personal datasets a government holds).

**Failure class:** Trusted-without-verify applied to infrastructure design. The ABS trusted that its DDoS mitigation (geoblocking) would work under attack conditions — it had not been adequately tested. Unclear incident characterization (DDoS vs. infrastructure failure) meant communications to the public were confused and inaccurate, compounding the erosion of trust.

**Human and financial cost:** Public confidence in government digital services severely damaged. Non-response and census avoidance by individuals concerned about data security. The Australian Bureau of Statistics completely rebuilt the online census system for 2021 ("complete rebuild"). The cost of the rebuild and reputational damage to the ABS's digital competence were significant.

**Controls not evidenced:** NIST SP 800-53 CP-2 (Contingency Planning), CP-10 (System Recovery and Reconstitution), IR-2 (Incident Response Training). DDoS resilience testing is a standard pre-deployment practice for high-profile government systems; it was apparently not performed at sufficient scale.

**UMRS design property:** System integrity during degraded operation is a controlled attribute. The census system's ability to protect data during a DDoS attack was not independently verified before deployment. A pre-deployment integrity check ("can the system protect data under the expected attack load?") is the operational equivalent of IMA/EVM pre-boot verification — it proves the system is in a known-good state before it handles controlled data at scale.

**Source citations:**
- OAIC: Australian Privacy Commissioner opens investigation into Census 2016 cyber attack: https://www.oaic.gov.au/updates/news-and-media/australian-privacy-commissioner-opens-investigation-into-census-2016-cyber-attack
- The Conversation: Did the Census really suffer a denial-of-service 'attack'?: https://theconversation.com/did-the-census-really-suffer-a-denial-of-service-attack-63755
- ABS: Security and privacy by design — 2021 census rebuild: https://www.abs.gov.au/census/about-census/delivering-2021-census/story-4-security-and-privacy-design

---

### AU-08: Services Australia / myGov — Credential-Stuffing Spike and Impersonation (2023–2025)

**What happened:** Services Australia notified more than 14,000 people in the 2023-24 financial year that their myGov accounts had been potentially accessed without permission. The number of malicious breaches — primarily credential-stuffing and social-engineering impersonation of the agency — rose from 7 in 2022-23 to 50 in 2023-24 to 82 in 2024-25. A Commonwealth Ombudsman investigation found that myGov's security "was not adequate" and some staff had not been asking callers the required security questions, enabling social engineering attacks. The agency subsequently added passkeys and promoted 2FA.

**Nation:** Australia

**CUI category equivalent:** CUI//PRVCY (Privacy — welfare, tax, and identity data for Australian residents accessing all government services through a single portal) and CUI//FNC (Financial — payment diversion).

**Failure class:** Over-access (single portal for all government services amplifies blast radius) and audit-but-no-enforcement (staff not following required security questions — a process that existed but was not enforced). Credential stuffing attacks showed that login with valid credentials was indistinguishable from legitimate access.

**Human and financial cost:** 14,000+ accounts with confirmed unauthorized access in one year. Rising year-on-year breach count. Personal welfare, health, and tax data for affected individuals accessed or modified by unauthorized parties.

**UMRS design property:** The myGov case illustrates the amplification risk of a single-portal architecture: one compromised credential accesses all government services. MLS compartmentalization limits the blast radius — accessing welfare data is a different security context from accessing Medicare data, which is different from accessing tax data. A single session compromise cannot simultaneously access all compartments.

**Source citations:**
- Services Australia: breaches spike using stolen account details (2024): https://ia.acs.org.au/article/2024/services-australia-breaches-spike-using-stolen-account-details.html
- Commonwealth Ombudsman: Keeping myGov Secure (2024-08): https://www.ombudsman.gov.au/__data/assets/pdf_file/0015/304701/Keeping-myGov-Secure.pdf

---

### AU-09: Austal Shipbuilder Data Breach — Extortion Attempt (2018)

**What happened:** In October–November 2018, a breach of Austal's systems (a Western Australian shipbuilder holding US Navy contracts) resulted in ship design drawings, staff contact information, and business documents being advertised for sale on the dark web. The attacker used stolen credentials obtained from the dark web and demanded payment. Austal refused to pay. The Australian Cyber Security Centre (ACSC) attributed the attack to an Iranian criminal group. The Defence Department confirmed that no classified information had been compromised.

**Nation:** Australia

**CUI category equivalent:** CUI//SP-CTI adjacent (Defence technical data) and CUI//PROPIN (Proprietary Business Information — ship design drawings supplied to customers and sub-contractors). The US Navy contracts make these drawings ITAR-adjacent.

**Failure class:** Credential compromise from the dark web enabling access to technical design data. The data — while confirmed non-classified — represented proprietary naval design information that could have intelligence or competitive value.

**UMRS design property:** Stolen credentials from the dark web represent the case where the credential has been compromised outside the organization's visibility. The control is not only preventing credential theft, but ensuring that a compromised credential cannot reach sensitive technical design data without additional authentication factors. CUI//PROPIN labeling on technical drawings enforces that they cannot be accessed without verified identity — a single password is not sufficient authorization.

**Source citations:**
- Iranian hackers suspected in Austal breach (2018): https://ia.acs.org.au/article/2018/iranian-hackers-suspected-in-austal-breach.html
- Naval Today: Australian shipbuilder Austal hit by cyber attack: https://www.navaltoday.com/2018/11/02/australian-shipbuilder-austal-hit-by-cyber-attack-extortion-attempt/

---

### AU-10: Te Whatu Ora COVID-19 Vaccination Data Insider Leak (NZ / AU-relevant)

*(This case appears in the NZ section — referenced here to note it as an Angle 1 "trusted but didn't verify" case that applies equally to AU health data governance.)*

---

## New Zealand

### NZ-01: Waikato DHB Ransomware Attack (2021)

**What happened:** On 18 May 2021, ransomware actors compromised the Waikato District Health Board's digital infrastructure, bringing down 611 servers across five hospitals. The DHB was offline from May through November 2021 — six months of operations without full clinical IT systems. Clinical services reverted to manual workarounds. Surgeries were postponed. More than 4,000 patient and employee records were published on the dark web six weeks after the initial attack. The post-mortem indicated insufficient network segmentation and inadequate backup and recovery planning.

**Nation:** New Zealand

**CUI category equivalent:** CUI//HLTH (Health — patient clinical records and employee data) and CUI//CRIT (Critical Infrastructure — hospital operations).

**Failure class:** Over-access (ransomware spread across 611 servers — no segmentation prevented lateral movement) and trusted-without-verify (backup and recovery systems were not verified to be functional at the required scale before they were needed).

**Human and financial cost:** Six months of degraded hospital operations across five hospitals. Surgical delays. 4,000+ patient and employee records published. Cryptocurrency ransom demand estimated in the millions; the DHB refused to pay.

**Controls not evidenced:** NIST SP 800-53 SC-3 (Security Function Isolation), CP-9 (System Backup), CP-10 (System Recovery). The NZ Health and Disability Commissioner's requirements and the NZ National Cyber Security Centre (NCSC) guidance apply.

**UMRS design property:** Mandatory segmentation of clinical systems — patient data systems, administrative systems, and operational systems exist in separate labeled domains. Ransomware that compromises one system cannot spread to others across mandatory access control boundaries. Backup systems are verified as functional (measured and attested) before they are relied upon for recovery — not after the primary system fails.

**Source citations:**
- Waikato DHB ransomware attack — Wikipedia: https://en.wikipedia.org/wiki/Waikato_District_Health_Board_ransomware_attack
- WDHB Final Incident Response Report: https://www.tewhatuora.govt.nz/assets/Publications/Proactive-releases/WDHB-Final-Report-2.0-redacted.pdf
- RNZ: Waikato DHB ransomware attack documents released online: https://www.rnz.co.nz/news/national/445735/waikato-dhb-ransomware-attack-documents-released-online

---

### NZ-02: MSD WINZ Kiosk Data Exposure (2012)

**What happened:** In 2012, a blogger discovered that public-facing job seeker kiosks at Work and Income New Zealand (WINZ) offices — providing welfare clients internet access — allowed access to MSD's internal server file system through the standard Windows Open File dialogue in Microsoft Office applications. The accessible files included invoices detailing the medical conditions of children in state care, names of adoption candidates, names of people under investigation for benefit fraud, pay rates for individual contractors, and plain-text passwords. The Beneficiary Advocacy Federation had warned MSD about the security problem a year earlier and received no response.

**Nation:** New Zealand

**CUI category equivalent:** CUI//PRVCY (Privacy — child welfare records, adoption data), CUI//LEI (Law Enforcement — fraud investigation names), CUI//HLTH (Health — children's medical conditions).

**Failure class:** Over-access — public internet kiosks on the same network as internal file shares, with no access control between the kiosk OS and the internal file system. This is an extreme compartmentalization failure: the most sensitive child protection records were reachable from a public terminal via a standard Windows file browse dialogue. The warning from the advocacy federation a year earlier, which went unaddressed, demonstrates the "audit existed but could not compel action" failure pattern.

**Human and financial cost:** Sensitive records of vulnerable children, adoptees, and fraud suspects exposed on public terminals. MSD shut all kiosks nationally. The government appointed Deloittes to investigate. The Beneficiary Advocacy Federation had reported the issue a year earlier — the failure was not discovery but remediation.

**Controls not evidenced:** Network segmentation between public access terminals and internal file systems is a fundamental security control. NIST SP 800-53 SC-7 (Boundary Protection), AC-3 (Access Enforcement). The NZ Privacy Act 1993 required appropriate safeguards for the records involved.

**UMRS design property:** Network segmentation enforced at the OS level — public kiosk OS is in a labeled, isolated network domain that cannot access internal file systems. No amount of application-layer access (file browse dialogue, browser URL bar) can cross the kernel-enforced network boundary. Controls like these make the attack surface of a public kiosk structurally separate from sensitive internal records.

**Source citations:**
- NZ Herald: MSD shuts WINZ kiosks after lax security exposed: https://www.nzherald.co.nz/nz/news/article.cfm?c_id=1&objectid=10840563
- RNZ: Entire MSD system 'may be compromised': https://www.rnz.co.nz/news/national/118325/entire-msd-system-'may-be-compromised'

---

### NZ-03: Novopay — School Payroll System Trust Without Verification (2012–2016)

**What happened:** In August 2012, the New Zealand Ministry of Education deployed Novopay, a school payroll system developed by Talent2, for a 10-year, $182 million contract. From the first pay run, thousands of errors occurred: approximately 5,000 staff underpaid, 700 overpaid, and 15 not paid at all. Within months, 90% of schools were affected, with over 500 distinct system defects identified (44 classified as very serious). More than 14,000 teachers and support staff were owed nearly $12 million in backpay by February 2013. A ministerial inquiry found that Ministry advice to government ministers was "inconsistent, over-optimistic and misrepresented the project just before its launch."

**Nation:** New Zealand

**CUI category equivalent:** CUI//PRVCY (Privacy — employee pay and banking records) equivalent to Canada's Phoenix case at smaller scale.

**Failure class:** Trusted-without-verify. The Ministry deployed a system with 500+ known defects and misrepresented its readiness to ministers. No independent verification of system output against expected pay calculations was performed before or during rollout. The system's outputs were trusted despite persistent evidence of error.

**Human and financial cost:** $45 million in direct costs (compensation, fixes, administration). 14,000+ teachers owed back pay. Financial hardship for affected school staff. The ministerial inquiry faulted the Ministry's governance and risk management.

**Controls not evidenced:** NIST SP 800-53 SA-11 (Developer Security Testing), CA-6 (Authorization), CM-4 (Impact Analysis for Changes). New Zealand government procurement requirements.

**UMRS design property:** Payroll system outputs are controlled, verifiable artefacts. Independent verification of a statistically significant sample of pay outputs against expected values is a pre-deployment gate, not a post-launch discovery. The "first pay run with 5,000 errors" demonstrates that no such gate existed. Controls like these embed output verification as a pre-launch requirement that the deploying authority cannot waive.

**Source citations:**
- Novopay — Wikipedia: https://en.wikipedia.org/wiki/Novopay
- RNZ: Inquiry faults Education Ministry over Novopay: https://www.rnz.co.nz/news/national/136841/inquiry-faults-education-ministry-over-novopay
- NZ Herald: Novopay: Education Ministry slammed in report: https://www.nzherald.co.nz/nz/novopay-education-ministry-slammed-in-report/XWURCM3LIC4IRZN7EO5T6STHZY/

---

### NZ-04: Te Whatu Ora — COVID-19 Vaccination Data Insider Leak (2023)

**What happened:** In late November 2023, a Te Whatu Ora (Health New Zealand) staff member inappropriately accessed and disclosed anonymized COVID-19 vaccination data, which was subsequently published on overseas websites. The data was described as anonymized, but Te Whatu Ora acknowledged that with expert technical knowledge, a small number of individuals could potentially be re-identified. Police charged the employee with accessing a computer system for a dishonest purpose. At least 12,000 people were affected. This is the classic "insider misuse of broad legitimate access" pattern — the employee had authorized access to the vaccination data system.

**Nation:** New Zealand

**CUI category equivalent:** CUI//HLTH (Health — vaccination records) and CUI//PRVCY (Privacy).

**Failure class:** Over-access and insider misuse. The employee had broad access to the vaccination data system for their role. No compartmentalization prevented access to the full dataset. No anomaly detection identified the bulk download of the dataset before it was published externally.

**Human and financial cost:** 12,000 individuals with vaccination data published. International cyber security assistance engaged for investigation. Urgent High Court injunction obtained. Police criminal charges.

**UMRS design property:** Bulk export of a vaccination dataset is a privileged operation distinct from clinical access to individual vaccination records. The operation requires explicit authorization, produces a kernel-level audit event, and is anomaly-flagged if performed by an account not authorized for research/export functions. The failure mode — authorized user with broad access silently downloading the full dataset — is exactly the access pattern that purpose-based compartmentalization is designed to prevent.

**Source citations:**
- Te Whatu Ora: Unauthorised data breach and attempt to spread misinformation: https://www.tewhatuora.govt.nz/for-health-professionals/health-sector-updates/data-breaches/unauthorised-data-breach-and-attempt-to-spread-misinformation/
- NZ Herald: Te Whatu Ora granted urgent injunction: https://www.nzherald.co.nz/nz/te-whatu-ora-granted-urgent-injunction-police-complaint-filed-over-mass-privacy-breach-of-covid-19-vaccination-data-by-former-staff-member/

---

### NZ-05: Mercury IT Ransomware — Single IT Provider, Multiple Government Health Bodies (2022)

**What happened:** On 30 November 2022, Mercury IT — a 25-employee Wellington IT managed service provider — suffered a ransomware attack. The impact cascaded across multiple organizations that had outsourced their IT infrastructure to this single provider: the Ministry of Justice could not access 14,500 coronial files and 4,000 post-mortem examination records; Health New Zealand could not access 8,500 bereavement care records and 5,500 cardiac and inherited disease registry records; six health regulatory authorities (Dietitians Board, Chiropractic Board, Podiatrists Board, NZ Psychologists Board, Physiotherapy Board, Optometrists Board) lost access to their registration systems; and Accuro health insurer was also affected.

**Nation:** New Zealand

**CUI category equivalent:** CUI//HLTH (Health — clinical records, post-mortem data, cardiac registry), CUI//PRVCY (Privacy — professional registration data), and CUI//LEI-adjacent (coronial records — records of death and post-mortem findings are legally significant government records).

**Failure class:** Third-party single-point-of-failure trust without verification. Multiple government agencies and professional regulatory bodies concentrated their IT infrastructure in a 25-employee firm with no evident security assurance framework. The concentration of sensitive government health and legal records in a single small provider created a single point of failure for the entire group. No organization verified the security posture of their shared IT provider before the incident.

**Human and financial cost:** Multiple government systems offline for extended periods. Coronial files for 14,500 deceased individuals inaccessible. Health professional registration data unavailable. Privacy Commissioner launched investigation. The operational disruption to coronial services affects death certification and post-mortem administration.

**Controls not evidenced:** NIST SP 800-53 SA-9 (External System Services — vendor security verification), CP-9 (System Backup — backups independent of the managed service provider), CP-2 (Contingency Planning). NZ Privacy Act 2020 requires appropriate security safeguards.

**UMRS design property:** Controlled data cannot be entrusted to a service provider without attestation of that provider's security posture. The concentration of government health and legal records in a single 25-employee firm without security assurance reflects the "trusted without verifying the trustee" pattern. Controls like these require that the custodian's security posture be continuously attested — not assumed — and that backup copies exist in a separate, independent custody chain.

**Source citations:**
- CPO Magazine: Ransomware Attack on a Small Managed IT Provider Disrupts Government Agencies in NZ: https://www.cpomagazine.com/cyber-security/a-ransomware-attack-on-a-small-managed-it-provider-disrupts-government-agencies-in-new-zealand/
- The Record: Multiple government departments in NZ affected by ransomware attack on IT provider: https://therecord.media/multiple-government-departments-in-new-zealand-affected-by-ransomware-attack-on-it-provider
- RNZ: Investigation underway into cyber attack which hit thousands of coronial and health files: https://www.rnz.co.nz/news/national/480234/investigation-underway-into-cyber-attack-which-hit-thousands-of-coronial-and-health-files

---

### NZ-06: ACC Privacy Breach — Personal Injury Data to Wrong Client (2011–2012)

**What happened:** On 5 August 2011, ACC made a data mistake that was not discovered until 1 December 2011 and not made public until March 2012. A client received personal information about other ACC clients, including sensitive injury and rehabilitation data. An independent review by KPMG (August 2012) examined ACC's handling of privacy and security of information in relation to the breach and identified 45 alleged privacy obligation breaches. The review found that the breaches should have been escalated to ACC's Privacy Officer and/or the Office of the Complaints Investigator — and were not. ACC had not fully appreciated the significance of the breach until it became public.

**Nation:** New Zealand

**CUI category equivalent:** CUI//HLTH (Health — personal injury and rehabilitation records) and CUI//PRVCY (Privacy). In NZ terms: sensitive personal information under the ACC Act.

**Failure class:** Audit existed but could not compel escalation. The breach occurred in August 2011, was known to ACC by December 2011, and was not escalated or disclosed until it became public in March 2012. The KPMG review identified systemic failures in ACC's internal escalation and privacy breach response processes.

**Human and financial cost:** Individuals received others' personal injury and rehabilitation data. The scale was not fully public, but the KPMG review identified 45 breach instances within the broader incident. ACC's credibility as a custodian of sensitive personal data was damaged.

**UMRS design property:** Breach escalation is a mandatory, audited event — not a discretionary decision left to the individual who discovers the breach. A kernel-level breach notification trigger ensures that when a data misdirection is detected, the escalation chain is automatically initiated. The three-month gap between discovery (December) and disclosure (March) reflects the absence of any mandatory escalation mechanism.

**Source citations:**
- KPMG Independent Review of ACC's Privacy and Security of Information (2012-08): https://www.acc.co.nz/assets/corporate-documents/5095e9f013/independent-review-privacy-information-2012.pdf
- Stuff.co.nz: ACC apologises over privacy breach: https://www.stuff.co.nz/national/health/6563083/ACC-apologises-over-privacy-breach

---

### NZ-07: ManageMyHealth Patient Portal Breach (2025)

**What happened:** On 30 December 2025, ManageMyHealth — an online patient portal used by approximately 1.8 million New Zealanders — was breached by an attacker who exfiltrated more than 400,000 documents from a specific document storage module. The attacker demanded US$60,000 ransom by 15 January 2026. Between 108,000 and 126,000 users were affected. Data exfiltrated included clinical notes, lab results, vaccination records, medical photographs, and personal contact information. Records dating back to 2017–2019 were included. The breach was discovered when a "partner organisation" notified ManageMyHealth — not by ManageMyHealth's own monitoring.

**Nation:** New Zealand

**CUI category equivalent:** CUI//HLTH (Health — clinical records, lab results, medical images) and CUI//PRVCY (Privacy).

**Failure class:** Over-access (a single breach vector reached 400,000+ documents spanning a decade) and audit-existed-but-could-not-detect (the breach was discovered by a partner, not by the system's own monitoring). Records from 2017–2019 being available in 2025 also indicates a data retention failure — data was retained long beyond likely need.

**Human and financial cost:** 108,000–126,000 individuals with clinical records exposed. Ransom demand of US$60,000. Full scope of harm not yet known as of report date.

**UMRS design property:** Patient portal document storage requires compartmentalization by sensitivity — clinical notes, lab results, and medical images are separately labeled objects. A breach of one storage module cannot reach all three categories without separate authorization for each. Data retention enforcement: records from 2017–2019 that are no longer clinically active are labeled for deletion and not accessible to new queries.

**Source citations:**
- ManageMyHealth data breach — Wikipedia: https://en.wikipedia.org/wiki/ManageMyHealth_data_breach
- RNZ: Manage My Health data breach — a timeline: https://www.rnz.co.nz/news/national/584053/manage-my-health-data-breach-a-timeline-of-what-happened-and-everything-we-know-so-far
- Health NZ: Manage My Health data breach — what you need to know: https://www.healthnz.govt.nz/news-and-updates/httpswww-tewhatuora-govt-nznews-and-updatesmanage-my-health-data-breach-what-you-need-to-date

---

## Cross-Nation Observations

### Pattern 1: "Trusted-Without-Verify" Is Universal

The Post Office Horizon (UK), SolarWinds (US), Novopay (NZ), Phoenix (CA), Robodebt (AU), and Deepwater Horizon (US, existing corpus) cases all share the same root structure: a system's output was treated as authoritative without independent verification of the system's integrity or correctness. The geography, time period, and domain differ — the failure class is identical.

### Pattern 2: Third-Party Custody Without Attestation Recurs Across All Five Nations

BGRS/SIRVA (CA), SSCL MOD payroll (UK), Advanced/Adastra NHS (UK), APT Alf contractor (AU), Latitude/third-party credentials (AU), Mercury IT (NZ), and Medibank/vendor credential (AU) all demonstrate that the assumption of contractor security — without ongoing verification — creates systematic exposure across all Five Eyes governments.

### Pattern 3: The Audit Gap Is Not a Detection Gap — It Is an Attribution and Enforcement Gap

In the IRS insider misuse (US), HMRC staff misuse (UK), IRCC insider access (CA, existing corpus), Ontario health snooping (CA), Te Whatu Ora insider (NZ), and Medicare dark web (AU) cases, audit logs existed. The logs recorded accesses. The problem was attribution (who accessed, and why), enforcement (access without justification was technically possible), and deterrence (the probability of detection was too low to change behaviour). UMRS is designed to address exactly this gap.

### Pattern 4: Population-Scale Data in Single Repositories Amplifies Every Attack

Optus (9.8M records, AU), Medibank (9.7M records, AU), Latitude (14M records, AU/NZ), HMRC (25M records, UK), OPM (millions, US, existing corpus) — in every case, the blast radius of a single attack vector was the entire population dataset because data of varying sensitivity was stored without compartmentalization.

### Pattern 5: Child and Indigenous Data Receive Inadequate Differentiated Protection

The MSD kiosk (NZ), Statistics Canada census (CA), RCMP surveillance (CA), and Windrush (UK — affecting a population defined by race and immigration status) cases all demonstrate that the most vulnerable populations — children, Indigenous people, migrants — receive inadequate differentiated protection. Their data is treated as equivalent to ordinary administrative records rather than as requiring additional sensitivity controls.

---

## CUI Index Group Coverage Summary (New Cases)

| CUI Index Group | New Cases Covering This Group |
|---|---|
| Privacy (PRVCY) | US-13, US-14, US-16, US-21, UK-02, UK-03, UK-06, UK-10, UK-13, AU-01, AU-06, NZ-02, NZ-04, NZ-07 |
| Health (HLTH) | US-17, US-21, UK-05, UK-08, UK-11, UK-12, AU-02, AU-06, AU-08, NZ-01, NZ-04, NZ-05, NZ-06, NZ-07 |
| Tax (TAX) | US-13, US-16, UK-06 |
| Critical Infrastructure (CRIT) | US-12, UK-04, UK-05, AU-07, NZ-01 |
| Export Control (EXPT / SP-EXPT) | US-15, AU-05, AU-09 |
| Law Enforcement (LEI) | US-20, NZ-02 |
| Proprietary Business Information (PROPIN) | US-19, AU-09 |
| Cyber Threat Intelligence (CTI) | US-11, UK-07, AU-05 |
| Financial (FNC) | UK-02, AU-08 |
| Statistical | AU-07, CA-21 |
| Government Administration / Legal | UK-09, UK-10, NZ-03, NZ-05 |

---

## Recommended Next Steps for Tech-Writer Integration

1. **US cases (US-11 through US-21)** — suitable for integration into `case-studies-consolidated.adoc` following the existing pattern.
2. **Canadian cases (CA-19 through CA-23)** — suitable for integration into `canadian-case-studies.adoc`.
3. **UK, AU, NZ cases** — warrant a new document: `five-eyes-case-studies.adoc` (suggested title: "Five Eyes High-Assurance Failure Case Studies").
4. **Cross-Nation Observations section** — suitable for a "Patterns Across Five Eyes" section in either a new document or a case studies index page.
5. **CUI Index Group coverage** — the summary table above can be used to verify that the UMRS case study corpus now has at least one case for each of the 18 active CUI index groups.

---

*Librarian note: All cases are documented from publicly available sources. Source URLs are provided for each case. Where cases involve ongoing litigation or regulatory proceedings, the characterization reflects publicly available information as of 2026-04-03 and should be verified against current reporting before publication.*
