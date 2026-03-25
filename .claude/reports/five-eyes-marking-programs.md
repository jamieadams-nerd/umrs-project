# Five Eyes Unclassified Information Marking Programs: UK, Australia, New Zealand

**Author:** Henri (Canadian Government Information Management & Bilingual Policy Specialist)
**Date:** 2026-03-25
**Audience:** Jamie, Knox, Herb
**Status:** Research complete -- pending Jamie decisions
**Scope:** Advisory reference (UMRS is a reference system, not a production deployment)
**Prerequisite reading:** `2026-03-23-canadian-protected-category-requirements.md` (Canadian catalog rationale)

---

## Purpose

This document catalogs the unclassified-but-controlled information marking programs for
the United Kingdom, Australia, and New Zealand. Together with the existing US CUI and
Canadian Protected catalogs, these three programs complete the Five Eyes picture at the
unclassified tier.

The goal is to inform JSON catalog design decisions. This document does NOT create catalogs.
It identifies structural patterns, divergences, and open questions that Jamie and Knox must
resolve before catalog construction begins.

---

## Current MCS Category Allocation

Before examining each country, the existing allocation:

| Country | Sensitivity Levels | Category Range | Status |
|---|---|---|---|
| US (CUI) | s0-s3 | c0-c199 | Catalog exists |
| Canada (Protected) | s1-s3 | c200-c299 | Catalog exists |
| UK | TBD | TBD (proposed: c300-c399) | This document |
| Australia | TBD | TBD (proposed: c400-c499) | This document |
| New Zealand | TBD | TBD (proposed: c500-c599) | This document |

---

# 1. United Kingdom: Government Security Classifications

## 1.1 Official Program Name and Governing Authority

**Program name:** Government Security Classifications (GSC)
**Governing authority:** Cabinet Office
**Policy owner:** Government Security Group (GSG), Cabinet Office
**Current version:** Government Security Classifications, May 2018 (replacing the
earlier Government Protective Marking System, GPMS, which was retired April 2014)

The UK overhauled its classification system in 2014, collapsing six protective markings
(UNCLASSIFIED, PROTECT, RESTRICTED, CONFIDENTIAL, SECRET, TOP SECRET) into three
(OFFICIAL, SECRET, TOP SECRET). The unclassified-but-controlled space is now entirely
within OFFICIAL and its handling caveat OFFICIAL-SENSITIVE.

## 1.2 Legal/Policy Framework

- **Security Policy Framework (SPF):** Cabinet Office document setting out governance
  requirements for security across HMG (Her Majesty's Government). Updated periodically.
  The GSC is an annex to or flows from the SPF.
- **Official Secrets Acts 1911-1989:** Criminal law underpinning for unauthorized disclosure.
  Applies across all classification levels.
- **Freedom of Information Act 2000 (FOIA):** UK FOIA applies to OFFICIAL material.
  OFFICIAL-SENSITIVE is not exempt from FOIA by virtue of its marking alone -- exemptions
  must be claimed on a case-by-case basis under specific FOIA sections.
- **Data Protection Act 2018 / UK GDPR:** Personal data within OFFICIAL material must
  comply with data protection requirements regardless of the classification marking.
- **National Cyber Security Centre (NCSC) guidance:** Provides technical handling guidance
  for each classification tier, including cloud security principles mapped to GSC levels.

**Source confidence:** High. The GSC policy document is publicly available from
gov.uk. The 2018 version is the current authoritative text as of this writing.

## 1.3 Marking Tiers (Unclassified Scope Only)

The UK does not use the term "unclassified." All routine government business is OFFICIAL.

| Tier | Marking | Description |
|---|---|---|
| OFFICIAL | `OFFICIAL` | The majority of information created or processed by the public sector. Includes routine business operations, policy development, and public services. Broadly equivalent to the old PROTECT and below. |
| OFFICIAL-SENSITIVE | `OFFICIAL-SENSITIVE` | A handling caveat (not a separate classification) applied to OFFICIAL material that requires additional controls due to the nature or context of the information. Applied by the information owner based on risk assessment. |

**Critical structural point:** OFFICIAL-SENSITIVE is NOT a classification level. It is
a handling caveat applied within the OFFICIAL tier. The UK explicitly states this. There
are only three classification levels in the GSC: OFFICIAL, SECRET, TOP SECRET.
OFFICIAL-SENSITIVE is a "need to know" marker within OFFICIAL, not a fourth level.

This has MCS implications. See Section 1.7.

## 1.4 Handling Requirements per Tier

### OFFICIAL

- **Personnel:** Baseline Personnel Security Standard (BPSS) -- the minimum check for
  all government employees and contractors.
- **Storage:** Commercial-quality premises with standard access controls. No specific
  container requirements.
- **Transmission:** Standard government IT systems. Encryption in transit over public
  networks (TLS minimum). Email within government networks (GSI/PSN) acceptable.
- **Destruction:** Standard commercial shredding. Cross-cut not mandated.
- **Cloud:** OFFICIAL workloads can run on commercial cloud services that meet NCSC
  Cloud Security Principles at OFFICIAL level.

### OFFICIAL-SENSITIVE

- **Personnel:** BPSS minimum; additional "need to know" controls applied by information owner.
- **Storage:** Same as OFFICIAL but with additional access restrictions. Typically
  restricted to named individuals or specific teams.
- **Transmission:** Encrypted. Not to be sent to personal email accounts. Additional
  care with external sharing -- the information owner must authorize.
- **Destruction:** As OFFICIAL but with assurance of completeness -- verified destruction
  for bulk material.
- **Cloud:** OFFICIAL-SENSITIVE workloads can run on the same commercial cloud platforms
  as OFFICIAL, with additional access controls. NCSC does not mandate a separate cloud
  tier for OFFICIAL-SENSITIVE.

## 1.5 Subcategories and Caveats

OFFICIAL-SENSITIVE has three optional descriptor suffixes:

| Descriptor | Marking | Purpose |
|---|---|---|
| Commercial | `OFFICIAL-SENSITIVE: COMMERCIAL` | Commercially sensitive information (tenders, negotiations, market-sensitive) |
| Personal | `OFFICIAL-SENSITIVE: PERSONAL` | Personal data requiring particular care (aligns with DPA/UK GDPR obligations) |
| LOCSEN | `OFFICIAL-SENSITIVE: LOCSEN` | Locally sensitive information (e.g., location-specific security details) |

These descriptors are NOT mandatory. The information owner applies them at their discretion.
They are guidance to the recipient, not enforcement categories. There is no registry of
descriptors equivalent to the NARA CUI Registry.

**Important:** HMG departments may define additional local descriptors, but these have no
cross-government standing. Only COMMERCIAL, PERSONAL, and LOCSEN are recognized across
all of HMG.

## 1.6 Structural Comparison to US CUI and Canadian Protected

| Dimension | UK GSC (OFFICIAL) | US CUI | Canadian Protected |
|---|---|---|---|
| Organizing principle | Risk-based handling caveat | Category taxonomy | Injury severity ladder |
| Number of tiers | 1 level + 1 handling caveat | 2 (Basic/Specified) + 125+ categories | 3 tiers |
| Subcategory model | 3 optional descriptors | Registry of named categories | None standardized |
| Dissemination controls | None formal; "need to know" is procedural | NOFORN, FEDONLY, REL TO, etc. | None standardized |
| FOIA interaction | OFFICIAL-SENSITIVE not FOIA-exempt per se | CUI status does not override FOIA | Protected designation interacts with ATIP |
| Governing authority | Cabinet Office | NARA/ISOO | TBS |

FINDING: UK-OFFICIAL-SENSITIVE-NOT-A-LEVEL
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: UK Government Security Classifications, Cabinet Office, May 2018, Section 3
DETAIL: OFFICIAL-SENSITIVE is explicitly defined as a handling caveat, not a
classification level. The UK policy document states: "OFFICIAL-SENSITIVE is not a
separate security classification." If the UMRS catalog models OFFICIAL-SENSITIVE as
a distinct MCS sensitivity level (e.g., s2 vs s1 for OFFICIAL), it would
misrepresent the UK policy. However, if the catalog models it only as a category
within the same sensitivity level, MCS enforcement semantics may not capture the
additional handling requirements. This is a structural design tension.
REMEDIATION: Route to Jamie and Knox. The MCS mapping decision (separate sX level
vs same level with different categories) requires architectural input. Both approaches
have tradeoffs. Document whichever choice is made and the rationale.

## 1.7 MCS/MLS Mapping Considerations

**Proposed category range:** c300-c399 (100 categories, matching Canadian allocation size)

**Option A -- Two sensitivity levels:**

| MCS Label | Meaning |
|---|---|
| s1:c300 | OFFICIAL |
| s2:c300 | OFFICIAL-SENSITIVE |
| s2:c301 | OFFICIAL-SENSITIVE: COMMERCIAL |
| s2:c302 | OFFICIAL-SENSITIVE: PERSONAL |
| s2:c303 | OFFICIAL-SENSITIVE: LOCSEN |
| c304-c399 | Reserved for future descriptors |

Pro: Captures the handling escalation between OFFICIAL and OFFICIAL-SENSITIVE via BLP
dominance. A process cleared for OFFICIAL-SENSITIVE can read OFFICIAL but not vice versa.

Con: Misrepresents UK policy by treating OFFICIAL-SENSITIVE as a separate level when the
UK explicitly says it is not.

**Option B -- Single sensitivity level with categories:**

| MCS Label | Meaning |
|---|---|
| s1:c300 | OFFICIAL |
| s1:c300,c310 | OFFICIAL-SENSITIVE (c310 = sensitive caveat flag) |
| s1:c300,c310,c301 | OFFICIAL-SENSITIVE: COMMERCIAL |
| s1:c300,c310,c302 | OFFICIAL-SENSITIVE: PERSONAL |
| s1:c300,c310,c303 | OFFICIAL-SENSITIVE: LOCSEN |

Pro: Accurately reflects UK policy -- both are OFFICIAL level. Category membership
controls access to the sensitive subset.

Con: Does not leverage BLP dominance. Access control depends entirely on category
membership, which is correct for MCS but less intuitive for operators expecting a
"higher = more restricted" model.

**Recommendation:** Option A for practical enforcement, with clear documentation that
OFFICIAL-SENSITIVE is a handling caveat within OFFICIAL and the sX separation is a
UMRS enforcement convenience, not a reflection of UK policy structure. This parallels
how the Canadian catalog uses three sX levels for three tiers that are not classification
levels either.

**[UNCERTAINTY]** I am not certain whether the NCSC has published updated technical
guidance post-2018 that changes any handling requirements. The 2018 GSC is the most
recent policy document I have confirmed.

## 1.8 Bilingual Considerations

The UK has no equivalent of the Official Languages Act. Government material is published
in English. Welsh Language Act 1993 and Welsh Language (Wales) Measure 2011 require
Welsh-language versions of material aimed at the Welsh public, but classification markings
are English-only across HMG.

For UMRS purposes: UK catalog entries need English only. No `_fr`, `_cy` (Welsh), or
other language fields required.

## 1.9 Key Differences Affecting Schema Design

1. **Two-state model:** Only OFFICIAL and OFFICIAL-SENSITIVE at the unclassified tier.
   Far simpler than US CUI or even Canadian Protected.
2. **Caveat vs level:** OFFICIAL-SENSITIVE is structurally a caveat, not a level.
   Schema must document this.
3. **Descriptors are optional and owner-applied:** No registry. No enforcement requirement
   for descriptors. The schema should list recognized descriptors but note they are
   guidance, not mandatory markings.
4. **No injury threshold ladder:** Unlike Canada, the UK does not define graduated injury
   tests for OFFICIAL vs OFFICIAL-SENSITIVE. The assessment is risk-based and left to
   the information owner.

---

# 2. Australia: Protective Security Policy Framework (PSPF)

## 2.1 Official Program Name and Governing Authority

**Program name:** Protective Security Policy Framework (PSPF)
**Governing authority:** Attorney-General's Department (AGD)
**Policy custodian:** Protective Security Policy Section, AGD
**Current version:** PSPF (2018 revision, with ongoing updates). The 2018 revision
aligned Australia's system with the UK's 2014 GSC reform.

Prior to 2018, Australia used a six-tier protective marking system similar to the
pre-2014 UK GPMS: UNCLASSIFIED, IN-CONFIDENCE, PROTECTED, CONFIDENTIAL, SECRET,
TOP SECRET. The 2018 reform collapsed the unclassified-to-PROTECTED range into a
single OFFICIAL tier, mirroring the UK approach.

## 2.2 Legal/Policy Framework

- **Protective Security Policy Framework (PSPF):** Non-legislative policy framework
  issued by the Attorney-General's Department. Mandatory for Australian Government
  (Commonwealth) entities covered by the Public Governance, Performance and
  Accountability Act 2013 (PGPA Act).
- **Australian Government Information Security Manual (ISM):** Published by the
  Australian Signals Directorate (ASD). Provides technical controls mapped to
  PSPF classification levels.
- **Archives Act 1983:** Governs records management. Interacts with classification
  markings for retention and disposal.
- **Freedom of Information Act 1982 (Cth):** Australian FOI Act. As with the UK,
  OFFICIAL-Sensitive marking does not create an automatic FOI exemption.
- **Privacy Act 1988 (Cth):** Australian Privacy Principles apply to personal
  information within OFFICIAL material.
- **Criminal Code Act 1995 (Cth), Division 122:** Offences for unauthorized
  disclosure of Commonwealth information.

**Source confidence:** High for the PSPF structure. The PSPF is publicly available
from the AGD's protectivesecurity.gov.au website. Some handling details in the ISM
are updated frequently; specific version dates should be confirmed before catalog
construction.

## 2.3 Marking Tiers (Unclassified Scope Only)

| Tier | Marking | Description |
|---|---|---|
| OFFICIAL | `OFFICIAL` | Default for all Australian Government information. Covers routine business, policy, and administration. |
| OFFICIAL: Sensitive | `OFFICIAL: Sensitive` | A dissemination limiting marker (DLM) applied within OFFICIAL. Indicates information requiring limited access on a need-to-know basis. |
| PROTECTED | `PROTECTED` | A full classification level (not a caveat). Information whose compromise could cause damage to the national interest, organisations, or individuals. |

**Critical structural point:** Unlike the UK, Australia retained PROTECTED as a distinct
classification level in the 2018 reform. PROTECTED sits ABOVE OFFICIAL in the Australian
hierarchy. This means Australia has TWO classification levels in the space UMRS considers
"unclassified-but-controlled": OFFICIAL (with its Sensitive caveat) and PROTECTED.

This is a significant structural divergence from the UK and from Canada.

FINDING: AU-PROTECTED-IS-A-CLASSIFICATION-LEVEL
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: PSPF, Attorney-General's Department, PSPF Policy 8: Sensitive and
classified information
DETAIL: Australian PROTECTED is a formal classification level, not a handling caveat.
It requires security clearance at the Negative Vetting 1 (NV1) level. This places
Australian PROTECTED structurally ABOVE the unclassified tier in a way that US CUI
and Canadian Protected A/B are not. Whether UMRS should model Australian PROTECTED
alongside US CUI and Canadian Protected (both formally unclassified) or treat it as
out of scope (formally classified by Australian standards) is an architectural decision.
Including it provides Five Eyes completeness but conflates different national
definitions of "classified."
REMEDIATION: Jamie must decide: does the UMRS unclassified-but-controlled scope
include Australian PROTECTED? If yes, document the definitional conflict. If no,
the Australian catalog covers only OFFICIAL and OFFICIAL: Sensitive.

## 2.4 Handling Requirements per Tier

### OFFICIAL

- **Personnel:** Baseline vetting (pre-employment checks, identity verification).
- **Storage:** Standard government office environment. No specific container requirements.
- **Transmission:** Government networks or encrypted over public networks. Standard
  email acceptable within government networks.
- **Destruction:** Standard methods; no specific destruction standard mandated beyond
  ensuring information is irrecoverable.

### OFFICIAL: Sensitive

- **Personnel:** Baseline vetting minimum. Access restricted to need-to-know.
- **Storage:** As OFFICIAL with access restrictions. Locked storage recommended for
  physical material.
- **Transmission:** Encrypted over public networks. Care with external sharing.
  Not to be published on public-facing systems.
- **Destruction:** As OFFICIAL with assurance of completeness.

### PROTECTED

- **Personnel:** Negative Vetting 1 (NV1) security clearance.
- **Storage:** Approved security container or secure room that meets ASIO T4 standards.
- **Transmission:** Encrypted with Australian Signals Directorate (ASD) approved
  cryptographic methods. Physical transmission via approved courier with receipting.
- **Destruction:** ASD-approved destruction methods. Media sanitization per ISM guidelines.
- **Access control:** Formal access control with logging. Two-person integrity for
  bulk access where applicable.

## 2.5 Subcategories and Caveats

Australia defines a richer set of dissemination limiting markers (DLMs) and caveats than the UK:

### DLMs for OFFICIAL: Sensitive

| Marker | Full Marking | Purpose |
|---|---|---|
| (no qualifier) | `OFFICIAL: Sensitive` | General sensitive -- needs care in handling |
| Personal | `OFFICIAL: Sensitive//Personal` | Sensitive personal information |
| Legal | `OFFICIAL: Sensitive//Legal` | Legally privileged information |
| Legislative secrecy | `OFFICIAL: Sensitive//Legislative-Secrecy` | Information subject to legislative secrecy provisions |

### Caveats (applicable to PROTECTED and above, but listed for completeness)

| Caveat | Purpose |
|---|---|
| AUSTEO | Australian Eyes Only -- not releasable to foreign nationals |
| AGAO | Australian Government Access Only |
| CABINET | Cabinet-in-Confidence |
| NATIONAL CABINET | National Cabinet (federal-state sensitive) |
| REL [country] | Releasable to named countries |

**Note:** AUSTEO, AGAO, and CABINET are caveats, not classifications. They can be
combined with any classification level.

FINDING: AU-CAVEATS-RESEMBLE-US-DISSEMINATION-CONTROLS
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: PSPF Policy 8; ISM Chapter on Caveats
DETAIL: Australian caveats (AUSTEO, AGAO, REL) are structurally similar to US
dissemination controls (NOFORN, FEDONLY, REL TO). Neither the UK nor Canada has
equivalent formal caveat vocabularies at the unclassified tier. If the UMRS schema
includes a `dissemination_controls` or `caveats` field, Australia will use it
but UK and Canada will not. This is a schema design asymmetry.
REMEDIATION: Consider a nullable `caveats` array field in the schema. Australia
populates it. UK, Canada, and NZ set it to null. US uses its existing
dissemination_controls field. Alternatively, merge caveats and dissemination_controls
into a single polymorphic field. Route to Jamie for structural decision.

## 2.6 Structural Comparison to US CUI and Canadian Protected

| Dimension | Australia PSPF | US CUI | Canadian Protected |
|---|---|---|---|
| Organizing principle | Tiered classification + caveats | Category taxonomy | Injury severity ladder |
| Unclassified-tier levels | 2 (OFFICIAL + PROTECTED) | 2 (Basic + Specified) | 3 (PA/PB/PC) |
| OFFICIAL-Sensitive | DLM (handling caveat) | N/A | N/A |
| Subcategories | DLM qualifiers + caveats | 125+ named categories | None |
| Formal caveats | AUSTEO, AGAO, CABINET, REL | NOFORN, FEDONLY, REL TO | None |
| FOIA interaction | Not exempt by marking | Not exempt by marking | ATIP interaction |
| Governing authority | AGD/ASD | NARA/ISOO | TBS |

## 2.7 MCS/MLS Mapping Considerations

**Proposed category range:** c400-c499 (100 categories)

The mapping depends on Jamie's decision about Australian PROTECTED scope (see Finding
AU-PROTECTED-IS-A-CLASSIFICATION-LEVEL).

**If Australian PROTECTED is in scope:**

| MCS Label | Meaning |
|---|---|
| s1:c400 | OFFICIAL |
| s2:c400 | OFFICIAL: Sensitive |
| s2:c401 | OFFICIAL: Sensitive//Personal |
| s2:c402 | OFFICIAL: Sensitive//Legal |
| s2:c403 | OFFICIAL: Sensitive//Legislative-Secrecy |
| s3:c400 | PROTECTED |
| c410-c419 | Reserved for caveats (c410=AUSTEO, c411=AGAO, c412=CABINET, etc.) |
| c420-c499 | Reserved for future use |

**If Australian PROTECTED is out of scope:**

| MCS Label | Meaning |
|---|---|
| s1:c400 | OFFICIAL |
| s2:c400 | OFFICIAL: Sensitive |
| s2:c401 | OFFICIAL: Sensitive//Personal |
| s2:c402 | OFFICIAL: Sensitive//Legal |
| s2:c403 | OFFICIAL: Sensitive//Legislative-Secrecy |
| c410-c419 | Reserved for caveats |
| c420-c499 | Reserved |

**Note on caveats in MCS:** Caveats like AUSTEO function as compartments -- they restrict
who can see the material. This maps naturally to MCS categories. A user without c410 in
their clearance range cannot access AUSTEO material regardless of their sensitivity level.
This is exactly how MCS categories are designed to work.

## 2.8 Bilingual Considerations

Australia has no official languages act at the Commonwealth level. All government
marking and policy material is in English only. Indigenous language considerations
do not extend to classification markings.

For UMRS purposes: Australian catalog entries need English only.

## 2.9 Key Differences Affecting Schema Design

1. **Three-state model:** OFFICIAL, OFFICIAL: Sensitive, PROTECTED -- more states than
   the UK (two) but fewer than the US (125+) or Canada (three explicit tiers).
2. **PROTECTED is a real classification level:** Requires NV1 clearance. This is
   qualitatively different from US CUI or Canadian Protected A/B.
3. **Formal caveats exist:** AUSTEO, AGAO, etc. The schema needs a caveats field or
   the Australian catalog will be structurally incomplete.
4. **DLM qualifiers are enumerated:** Personal, Legal, Legislative-Secrecy. These are
   closer to US CUI subcategories than to UK descriptors (which are vaguer).
5. **Colon-space syntax:** Australia uses `OFFICIAL: Sensitive` with a colon and space.
   This is the official marking syntax and should be preserved in the catalog, not
   normalized to a hyphenated form.

---

# 3. New Zealand: Protective Security Requirements (PSR)

## 3.1 Official Program Name and Governing Authority

**Program name:** Protective Security Requirements (PSR)
**Governing authority:** New Zealand Security Intelligence Service (NZSIS) for
information security policy; Government Communications Security Bureau (GCSB)
for technical standards.
**Policy custodian:** Protective Security team, NZSIS
**Current version:** PSR (refreshed 2017 and subsequently updated). New Zealand
adopted the OFFICIAL/SECRET/TOP SECRET model in alignment with the UK and
Australia reforms.

Prior to the 2017 refresh, New Zealand used: IN CONFIDENCE, SENSITIVE, RESTRICTED,
CONFIDENTIAL, SECRET, TOP SECRET. The refresh collapsed the lower tiers into
OFFICIAL (with a Sensitive caveat), aligning with Five Eyes partner reforms.

## 3.2 Legal/Policy Framework

- **Protective Security Requirements (PSR):** Cabinet-mandated policy framework.
  Mandatory for all New Zealand government agencies (departments and Crown entities
  listed in Schedule 1 of the State Sector Act 1988, now the Public Service Act 2020).
- **Official Information Act 1982 (OIA):** New Zealand's equivalent of FOIA.
  Classification markings do not automatically exempt information from OIA requests.
  Withholding must be justified under specific OIA sections.
- **Privacy Act 2020:** Information Privacy Principles apply to personal information
  regardless of classification.
- **Crimes Act 1961, Part 6:** Espionage and wrongful communication offences.
- **GCSB Information Security Manual (NZISM):** Technical security controls mapped
  to classification levels. Published by the GCSB. Updated periodically.

**Source confidence:** Medium-High. The PSR is publicly available from
protectivesecurity.govt.nz. The NZISM is published by GCSB. However, New Zealand's
documentation is less detailed in publicly available form than the UK GSC or Australian
PSPF. Some handling details are inferred from NZISM rather than directly stated in the
PSR policy text.

**[UNCERTAINTY]** I have not been able to confirm whether New Zealand has published
a post-2022 PSR update that materially changes the classification structure. The
2017 framework with subsequent updates is the most recent version I can confirm.

## 3.3 Marking Tiers (Unclassified Scope Only)

| Tier | Marking | Description |
|---|---|---|
| UNCLASSIFIED | No marking required | Routine government information with no damage expectation from compromise. |
| IN CONFIDENCE | `IN CONFIDENCE` | Information whose compromise could cause damage to an individual, or disadvantage to the government. Lowest classified tier. |
| SENSITIVE | `SENSITIVE` | Information whose compromise could cause damage, but below the threshold for IN CONFIDENCE. An administrative marking, not a formal classification. |
| OFFICIAL | `OFFICIAL` | Per the PSR refresh -- the default for government business. Equivalent to the UK/AU OFFICIAL tier. |

**Critical structural point:** New Zealand's post-2017 alignment is less clean-cut than
the UK or Australian reforms. Historical context matters:

- **Pre-2017:** NZ used IN CONFIDENCE and SENSITIVE as distinct tiers below RESTRICTED.
- **Post-2017:** NZ adopted OFFICIAL as the baseline, but transitional usage of the
  older terms may persist in legacy systems and documentation.
- **NZISM references:** The GCSB NZISM provides technical controls for UNCLASSIFIED,
  IN CONFIDENCE, SENSITIVE, RESTRICTED, CONFIDENTIAL, SECRET, and TOP SECRET.

FINDING: NZ-TRANSITIONAL-MARKING-AMBIGUITY
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: PSR (protectivesecurity.govt.nz); NZISM
DETAIL: New Zealand's transition from the pre-2017 marking scheme to the aligned
OFFICIAL model is not fully documented in publicly available sources. The NZISM
continues to reference the older tier names (IN CONFIDENCE, SENSITIVE, RESTRICTED)
while the PSR policy text uses the new OFFICIAL model. This creates ambiguity about
which marking vocabulary is authoritative for current NZ government practice. UMRS
must pick one or document both.
REMEDIATION: Route to Jamie. Recommend modeling the current PSR vocabulary
(OFFICIAL / OFFICIAL-Sensitive) for the catalog, with a note documenting the legacy
tier names for systems that may still use them. Do not model both simultaneously --
that doubles the category consumption for no enforcement benefit.

## 3.4 Handling Requirements per Tier

### OFFICIAL (per PSR post-2017 alignment)

- **Personnel:** Pre-employment vetting (identity, qualifications, references).
- **Storage:** Standard government premises with access control.
- **Transmission:** Government networks or encrypted over external networks.
- **Destruction:** Standard methods ensuring irrecoverability.

### OFFICIAL-Sensitive (NZ equivalent of the UK/AU Sensitive caveat)

- **Personnel:** Pre-employment vetting minimum; need-to-know restrictions applied
  by information owner.
- **Storage:** Restricted access within government premises. Locked storage for
  physical material recommended.
- **Transmission:** Encrypted. External sharing requires information owner authorization.
- **Destruction:** Verified destruction for sensitive material.

**[UNCERTAINTY]** New Zealand's publicly available handling guidance is less granular
than the UK or Australian equivalents. The requirements above are synthesized from PSR
and NZISM guidance. Specific container standards and destruction methods are referenced
in the NZISM but may require GCSB authorization to cite in detail.

## 3.5 Subcategories and Caveats

New Zealand defines a smaller set of caveats than Australia:

| Caveat | Purpose |
|---|---|
| NZ EYES ONLY (NZEO) | Not releasable to foreign nationals |
| RELEASABLE TO [country] | Releasable to named partner nations |
| CABINET | Cabinet material requiring additional handling |

New Zealand does not define DLM qualifiers (Personal, Legal, etc.) equivalent to
Australia's OFFICIAL: Sensitive subcategories or the UK's descriptors. The Sensitive
caveat is applied without qualification.

## 3.6 Structural Comparison to US CUI and Canadian Protected

| Dimension | NZ PSR | US CUI | Canadian Protected |
|---|---|---|---|
| Organizing principle | Tiered classification (aligned with UK/AU) | Category taxonomy | Injury severity ladder |
| Unclassified-tier levels | 1 (OFFICIAL) + Sensitive caveat | 2 (Basic/Specified) + 125+ categories | 3 (PA/PB/PC) |
| Subcategories | None | 125+ | None |
| Formal caveats | NZEO, REL TO, CABINET | NOFORN, FEDONLY, REL TO | None |
| FOIA interaction | OIA applies; marking is not exemption | Not exempt by marking | ATIP interaction |
| Governing authority | NZSIS/GCSB | NARA/ISOO | TBS |

## 3.7 MCS/MLS Mapping Considerations

**Proposed category range:** c500-c599 (100 categories)

| MCS Label | Meaning |
|---|---|
| s1:c500 | OFFICIAL |
| s2:c500 | OFFICIAL-Sensitive |
| c510 | NZEO caveat |
| c511 | CABINET caveat |
| c512-c519 | Reserved for future caveats |
| c520-c599 | Reserved |

The NZ model is the simplest of the five -- two states (OFFICIAL and OFFICIAL-Sensitive)
plus a small caveat vocabulary. This maps cleanly to MCS with minimal category consumption.

## 3.8 Bilingual Considerations

New Zealand has two de jure official languages: English and Te Reo Maori (Maori Language
Act 1987, superseded by Te Ture mo Te Reo Maori 2016 / Maori Language Act 2016). A third
language, New Zealand Sign Language (NZSL), has official status under the New Zealand Sign
Language Act 2006.

**However:** Government classification markings are in English only. Te Reo Maori
translations of classification terms are not standardized in the PSR or NZISM.

FINDING: NZ-TE-REO-MARKING-GAP
SEVERITY: Low
DOMAIN: Canadian Policy
SOURCE: Te Ture mo Te Reo Maori 2016; PSR
DETAIL: New Zealand has official language obligations for Te Reo Maori, but
classification markings are not published in Te Reo Maori. If UMRS were to provide
Te Reo Maori marking terms, there is no authoritative source to draw from. This is
an observation, not an action item -- UMRS has no obligation to create translations
that the source government has not defined.
REMEDIATION: None required. Document the observation. If NZ ever publishes Te Reo
Maori marking terms, they could be added to the catalog at that time.

## 3.9 Key Differences Affecting Schema Design

1. **Simplest model:** Two states plus a small caveat set. Lowest category consumption.
2. **Transitional vocabulary:** Legacy terms (IN CONFIDENCE, SENSITIVE, RESTRICTED) may
   appear in NZ systems. Schema should note these but not model them.
3. **Limited public documentation:** Less handling detail is publicly available compared
   to UK or Australia. Schema notes should flag this.
4. **Caveats overlap with Australia:** NZEO parallels AUSTEO. REL TO is shared across
   multiple Five Eyes partners. Consider whether caveat category numbers should be
   harmonized across country catalogs (e.g., c_10 offset = "eyes only" caveat in
   every country's range).

---

# 4. Schema Implications

## 4.1 Common Fields Across All Five Catalogs

The following fields appear in every country's catalog based on structural analysis:

| Field | US | CA | UK | AU | NZ | Notes |
|---|---|---|---|---|---|---|
| `name` | Yes | Yes | Yes | Yes | Yes | English display name |
| `abbreviation` | Yes | Yes | Yes | Yes | Yes | Short code for MCS labels |
| `level` (sX) | Yes | Yes | Yes | Yes | Yes | MCS sensitivity level |
| `category_base` (cXXX) | Yes | Yes | Yes | Yes | Yes | Base MCS category |
| `description` | Yes | Yes | Yes | Yes | Yes | What this tier covers |
| `handling` | Yes | Yes | Yes | Yes | Yes | Object with handling requirements |
| `authority` | Implicit | Yes | Yes | Yes | Yes | Governing policy instrument |

## 4.2 Country-Specific Fields

| Field | Countries | Rationale |
|---|---|---|
| `name_fr` / `description_fr` / all `_fr` fields | CA only | Official Languages Act. No other FVEY country requires French. |
| `injury_threshold` / `injury_examples` | CA only | Canada's system is organized by injury severity. Others are not. |
| `dissemination_controls` | US only | US CUI has a formal dissemination control vocabulary. |
| `caveats` | AU, NZ | AUSTEO, NZEO, AGAO, CABINET, REL TO. UK has no formal caveats at OFFICIAL. |
| `descriptors` | UK, AU | COMMERCIAL, PERSONAL, LOCSEN (UK); Personal, Legal, Legislative-Secrecy (AU). |
| `parent_group` | US only | US CUI's hierarchical category tree. Others are flat or two-level. |
| `handling_group_id` | US only | CUI handling group references. |
| `phase_note` | CA (PC only) | Protected C Phase 2 enforcement caveat. |
| `marking_banner_en` / `marking_banner_fr` | CA | Bilingual marking banner text. |
| `is_classification_level` | AU (PROTECTED) | Boolean flag: is this a formal classification level in its home jurisdiction? |

## 4.3 Does the Canadian `_metadata` Pattern Extend to All Five?

**Yes.** The `_metadata` block in CANADIAN-PROTECTED.json is a sound pattern that should
be replicated in every country catalog. Recommended `_metadata` fields for all catalogs:

| Field | Purpose |
|---|---|
| `catalog_name` | Human-readable catalog title |
| `version` | Semantic version for the catalog |
| `authority` | Governing policy instrument name |
| `authority_date` | Date of the cited policy version |
| `author` | UMRS team member who created the catalog |
| `created` | Creation date |
| `scope` | "Advisory reference" caveat |
| `notes` | Structural notes specific to this country's system |
| `structural_differences_from_us_cui` | How this country diverges from the US CUI baseline |

The Canadian catalog adds `catalog_name_fr` -- this field exists only in the Canadian
catalog due to OLA requirements. The UK, AU, and NZ catalogs do not need it.

## 4.4 Recommended setrans.conf Category Block Allocations

| Block | Country | Range | Size | Rationale |
|---|---|---|---|---|
| c0-c199 | US (CUI) | c0-c199 | 200 | Existing allocation. US has the largest category taxonomy (125+ categories). |
| c200-c299 | Canada (Protected) | c200-c299 | 100 | Existing allocation. Three tiers with room for departmental subcategories. |
| c300-c399 | UK (GSC) | c300-c399 | 100 | Two-state model + 3 descriptors. 100 is generous. |
| c400-c499 | Australia (PSPF) | c400-c499 | 100 | Three states + caveats + DLM qualifiers. May need more than UK/NZ. |
| c500-c599 | NZ (PSR) | c500-c599 | 100 | Simplest model. 100 is very generous. |
| c600-c999 | Reserved | c600-c999 | 400 | Future Five Eyes partner extensions, bilateral agreements, or NATO markings. |

**Total category consumption:** c0-c599 (600 categories out of the 1024 available in
a 10-bit MCS category space, or out of a larger space depending on kernel configuration).

FINDING: MCS-CATEGORY-SPACE-CONSUMPTION
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: SELinux MCS implementation; UMRS MCS allocation design
DETAIL: The proposed Five Eyes allocation consumes c0-c599 (600 categories). The
default SELinux MCS category space is c0-c1023 (1024 categories). This leaves 424
categories (c600-c1023) for non-Five Eyes use, organizational compartments, or
future expansion. This is a comfortable margin. However, if the MCS category space
is configured smaller than 1024 (some deployments use 256 or 512), the Five Eyes
allocation alone would consume the entire space or exceed it.
REMEDIATION: Document the minimum MCS category space requirement for Five Eyes
interoperability. Recommend c1023 minimum (default SELinux). Flag that deployments
with restricted category spaces cannot support the full Five Eyes catalog.

## 4.5 Caveat Harmonization Opportunity

Several caveats are conceptually equivalent across partner nations:

| Concept | US | AU | NZ | UK | CA |
|---|---|---|---|---|---|
| National eyes only | NOFORN | AUSTEO | NZEO | (no formal equiv.) | (no formal equiv.) |
| Government only | FEDONLY | AGAO | (none) | (none) | (none) |
| Releasable to partners | REL TO | REL | REL TO | (none) | (none) |
| Cabinet material | (none) | CABINET / NATIONAL CABINET | CABINET | (none) | Cabinet Confidence* |

*Canadian Cabinet confidence is a distinct legal concept (Access to Information Act, s.69)
rather than a marking caveat.

**Design question:** Should conceptually equivalent caveats share a category offset
within each country's block? For example, if "eyes only" is always at offset +10
(c10, c210, c310, c410, c510), operators and code can reason about the concept
without knowing the country-specific term. This is an elegance-vs-accuracy tradeoff.

---

## 5. Decisions Required

The following decisions require Jamie and/or Knox input before catalog construction:

### 5.1 For Jamie

| # | Decision | Options | Recommendation |
|---|---|---|---|
| D1 | Is Australian PROTECTED in scope? | (a) Yes -- include it as the highest AU tier in the catalog, with documentation noting it is formally classified in AU; (b) No -- AU catalog covers OFFICIAL and OFFICIAL: Sensitive only | (a) Include it. Five Eyes completeness matters more than definitional purity for a reference system. Document the conflict. |
| D2 | How to model OFFICIAL-SENSITIVE (UK) and OFFICIAL: Sensitive (AU/NZ)? | (a) Separate sX level (enforcement convenience); (b) Same sX level with category flag (policy accuracy) | (a) Separate sX level, with explicit documentation that this is an enforcement convenience. Parallels the Canadian sX-per-tier approach. |
| D3 | Should caveat category offsets be harmonized across countries? | (a) Yes -- consistent offset for "eyes only," "government only," etc.; (b) No -- each country allocates caveats independently within its block | (a) Harmonize. The conceptual alignment is real and reduces operator cognitive load. |
| D4 | Should the schema include a `caveats` field? | (a) Nullable array in all catalogs (AU/NZ populate, others null); (b) AU/NZ-specific field not present in other catalogs; (c) Merged with US `dissemination_controls` into a unified field | (a) Nullable array. Keeps the schema uniform without forcing empty structures. |
| D5 | NZ legacy marking terms (IN CONFIDENCE, SENSITIVE, RESTRICTED) -- model or document only? | (a) Model as deprecated aliases in the catalog; (b) Document in `_metadata.notes` only | (b) Document only. Do not consume MCS categories for deprecated terms. |
| D6 | UK catalog language -- English only? | (a) English only; (b) Include Welsh translations for completeness | (a) English only. Welsh language obligations do not extend to classification marking terms, and no authoritative Welsh translations exist. |

### 5.2 For Knox (Security Architecture)

| # | Decision | Context |
|---|---|---|
| K1 | MCS category space minimum requirement | The Five Eyes allocation requires c0-c599 minimum. Knox should confirm the minimum `mcs_num_cats` kernel parameter for UMRS deployments. |
| K2 | BLP dominance across country catalogs | If US s3 and AU s3 both exist, does a process at s3 with US categories read AU s3 material? Cross-country dominance semantics need architectural definition. |
| K3 | Caveat enforcement model | Caveats (AUSTEO, NZEO, NOFORN) are access restrictions. Should they be modeled as MCS categories (preventing access without the category) or as MLS compartments (formal compartmentalization)? MCS categories are simpler; MLS compartments are more correct but require Phase 2. |

### 5.3 For Henri (Self-Assigned Follow-Up)

| # | Task | Dependency |
|---|---|---|
| H1 | Validate UK GSC against any post-2018 Cabinet Office updates | None -- can proceed |
| H2 | Validate NZ PSR against any post-2022 NZSIS updates | None -- can proceed |
| H3 | Confirm AU PSPF current version date from protectivesecurity.gov.au | None -- can proceed |
| H4 | Draft the UK JSON catalog | Blocked on D1, D2, D3, D4, D6 |
| H5 | Draft the AU JSON catalog | Blocked on D1, D2, D3, D4 |
| H6 | Draft the NZ JSON catalog | Blocked on D2, D3, D4, D5 |

---

## Findings Summary

| Finding | Severity | Domain | Route To |
|---|---|---|---|
| UK-OFFICIAL-SENSITIVE-NOT-A-LEVEL | High | Canadian Policy | Jamie, Knox |
| AU-PROTECTED-IS-A-CLASSIFICATION-LEVEL | High | Canadian Policy | Jamie, Knox |
| AU-CAVEATS-RESEMBLE-US-DISSEMINATION-CONTROLS | Medium | Canadian Policy | Jamie |
| NZ-TRANSITIONAL-MARKING-AMBIGUITY | Medium | Canadian Policy | Jamie |
| NZ-TE-REO-MARKING-GAP | Low | Canadian Policy | Jamie |
| MCS-CATEGORY-SPACE-CONSUMPTION | Informational | Canadian Policy | Knox |

All findings are advisory per the standing directive that UMRS is a reference system.
No findings are blockers for research. D1, D2, D3, and D4 block catalog construction.

---

## Source Notes

**Confirmed sources (publicly available policy documents):**
- UK: Government Security Classifications, Cabinet Office, May 2018
- UK: Security Policy Framework, Cabinet Office
- AU: Protective Security Policy Framework, Attorney-General's Department (protectivesecurity.gov.au)
- AU: Australian Government Information Security Manual, ASD
- NZ: Protective Security Requirements, NZSIS (protectivesecurity.govt.nz)
- NZ: NZ Information Security Manual, GCSB

**Knowledge cutoff caveat:** My training data has a cutoff. Policy documents may have been
updated after my last confirmed version. Tasks H1-H3 above address this gap. No catalog
should be finalized without confirming the current version of each source document.

**What this document does NOT cover:**
- Classified tiers (SECRET, TOP SECRET) for any country
- NATO marking system (a separate interoperability question)
- Bilateral marking agreements between specific Five Eyes pairs
- Technical implementation of setrans.conf entries (that is Knox's domain)
