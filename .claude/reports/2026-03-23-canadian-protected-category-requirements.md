# Canadian Protected Category Requirements Report

**Author:** Henri (Canadian Government Information Management & Bilingual Policy Specialist)
**Date:** 2026-03-23
**Audience:** Jamie, Knox, Herb
**Status:** Updated -- Jamie's decisions incorporated (2026-03-23)
**Scope:** Advisory (UMRS is a reference system, not a production deployment)

---

## Executive Summary

Canada's Protected information designation system is structurally different from US CUI.
Where US CUI is a category-based taxonomy (125+ named categories organized under parent
groups), Canada's Protected system is a three-tier impact ladder with no standardized
subcategories or dissemination controls. This report defines the Canadian categories,
their mutual exclusivity rules, handling requirements, and what the CANADIAN-PROTECTED.json
catalog must contain.

The core finding: the Canadian catalog will be structurally simpler than the US CUI catalog.
Three tiers. No subcategories. No dissemination control suffixes. The complexity lives in
the injury threshold definitions and their mapping caveats, not in a category taxonomy.

---

## 1. What Categories Exist

### Source Authority

TBS Directive on Security Management, Appendix J (Standard on Security Categorization),
sections J.2.4.2.1 through J.2.4.2.3. Effective 2019-07-01.

### The Three Protected Tiers

| Tier | EN Designation | FR Designation | Injury Threshold (EN) | Injury Threshold (FR) | MCS Level |
|---|---|---|---|---|---|
| Protected A | Protected A | Protégé A | Limited or moderate injury outside the national interest | Préjudice limité ou modéré à des intérêts autres que l'intérêt national | s1:c200-c299 |
| Protected B | Protected B | Protégé B | Serious injury outside the national interest | Préjudice sérieux à des intérêts autres que l'intérêt national | s2:c200-c299 |
| Protected C | Protected C | Protégé C | Extremely grave injury outside the national interest | Préjudice extrêmement grave à des intérêts autres que l'intérêt national | s3:c200-c299 |

### Critical Distinction: "Outside the National Interest"

The TBS Standard draws a hard line between Classified and Protected designations:

- **Classified** (Confidential / Secret / Top Secret): injury to **the national interest**
- **Protected** (A / B / C): injury to interests **outside the national interest**

This is not a difference in degree. It is a difference in kind. Protected C, despite being
"extremely grave injury," is NOT equivalent to Confidential because the injury domain is
different. Protected C covers loss of life to individuals. Confidential covers injury to
the state's national interest.

UMRS must never conflate these two tracks in documentation or tool output.

### What Protected A/B/C Are NOT

1. **Not a CUI category taxonomy.** There are no named subcategories like CUI//PRIVACY/HEALTH
   or CUI//LEI/INV. Canada categorizes by injury severity, not by information type.

2. **Not a marking system with dissemination suffixes.** There is no Canadian equivalent of
   CUI//SP-EXPT or NOFORN. Canadian handling is governed by the injury tier plus
   departmental need-to-know procedures, not by marking-encoded distribution rules.

3. **Not department-specific.** The three tiers are government-wide. Individual departments
   (DND, RCMP, CSIS, etc.) may have internal handling procedures, but the designation
   ladder itself is standardized by TBS.

---

## 2. Mutual Exclusivity Rules

### Tier Exclusivity

Protected A, Protected B, and Protected C are **mutually exclusive on the same information
asset.** A document is categorized at exactly one Protected tier based on the injury
assessment. It cannot be simultaneously Protected A and Protected B.

This is inherent in the injury-threshold model: if the injury from disclosure would be
"serious" (Protected B), it is by definition not "limited or moderate" (Protected A).
The categorization process in J.2.2 requires a single determination.

### Protected vs Classified Exclusivity

Protected and Classified are mutually exclusive tracks. Information that could cause
injury to the national interest is Classified, not Protected. Information that could cause
injury outside the national interest is Protected, not Classified.

However, a single document or system may contain both Classified and Protected information.
In that case, the Classified designation governs the handling. UMRS does not model Classified
information (that requires a separate policy framework), so this edge case is out of scope.

### MCS Enforcement of Exclusivity

The MCS sensitivity level architecture naturally enforces tier exclusivity:
- A file at s1:c200 is Protected A. It cannot simultaneously be at s2 or s3.
- BLP dominance means a process at s2 can read s1 (PA can read PA) but not s3.
- Upgrading from PA to PB means changing the sensitivity level from s1 to s2.
  This is a reclassification event, not a dual designation.

### Category Coexistence Within a Tier

Within the c200+ namespace of a given sensitivity level, multiple categories CAN coexist.
For example, a file at s2:c201,c205 would carry two distinct Protected B category tags.
However, because Canada does not define standardized subcategories (see Section 3 below),
the initial catalog will define only the tier markers, not information-type subcategories.

Future work could define department-specific categories within the c200+ range (e.g.,
c201 = personnel, c202 = financial, c203 = medical) but this requires departmental
input that UMRS does not currently have. The catalog should document this extension point
without populating it.

---

## 3. Handling Requirements

### What TBS Prescribes

The TBS framework prescribes handling requirements through two mechanisms:

1. **The injury tier itself** -- the tier determines the baseline safeguard level.
   Higher tiers require stronger safeguards (encryption, physical security, personnel
   screening levels).

2. **Need-to-know** -- access is restricted to individuals whose duties require it.
   This is procedural (enforced through access control lists, security briefings,
   and organizational accountability), not marking-based.

### No Standardized Dissemination Controls

FINDING: STRUCTURAL-DIVERGENCE-HANDLING
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: TBS Standard on Security Categorization (Appendix J), full text review
DETAIL: The TBS Standard mentions "applicable caveats" once in passing but never
defines a standardized vocabulary of dissemination controls. Canada does not have
equivalents to NOFORN, FEDONLY, REL TO, or any of the US CUI dissemination control
markings. Canadian handling operates on a two-axis model: (injury tier) + (need-to-know).
The US operates on a three-axis model: (CUI designation) + (category) + (dissemination control).
This structural difference means the CANADIAN-PROTECTED.json catalog will not include
a "dissemination_controls" field or equivalent. Attempting to force-fit US-style LDCs
onto the Canadian system would misrepresent how Canada handles Protected information.
REMEDIATION: Document this as a structural difference in the catalog and in any
Five Eyes interoperability documentation. Do not create placeholder LDC fields.

### Handling by Tier

| Tier | Personnel Screening | Storage | Transmission | Destruction |
|---|---|---|---|---|
| Protected A | Reliability Status | Locked container or access-controlled area | Encrypted or approved courier | Shred or approved destruction |
| Protected B | Reliability Status (Enhanced) | Locked container, alarm-monitored area | Encrypted, double envelope for physical | Cross-cut shred, approved electronic media destruction |
| Protected C | Enhanced screening (approaches Secret-level) | Approved security container, alarm-monitored, access logging | Encrypted with approved cryptography, approved courier with chain of custody | NSA/CSE-approved destruction methods |

Note: These handling requirements come from TBS guidance and CCCS publications
(ITSP.10.222, ITSG-33 PBMM profile). The exact safeguard requirements are departmental
decisions within the TBS framework. The table above represents the general practice, not
a prescriptive standard for UMRS to enforce.

### Need-to-Know Implementation in MCS

Need-to-know maps naturally to MCS category membership:
- A user with category c200 in their clearance range can access files tagged c200.
- A user without c200 cannot, regardless of their sensitivity level.
- This is the MCS two-factor model: sensitivity level (how serious) x category (which compartment).

For the Canadian Protected namespace, the sensitivity level does the heavy lifting (how
serious is the injury?), while categories would identify which compartment of Protected
information the user needs access to. Since Canada does not standardize these compartments,
the initial implementation will use the tier markers only.

---

## 4. Approximate Correspondence to US CUI

This section is the most policy-sensitive part of the report. These correspondences are
approximate and require caveats at every level.

### Why Direct Mapping Fails

| Dimension | US CUI | Canadian Protected |
|---|---|---|
| Organizing principle | Category taxonomy (what type of information) | Injury severity ladder (how bad is disclosure) |
| Number of designations | 125+ named categories | 3 tiers |
| Subcategories | Yes (e.g., PRIVACY/HEALTH, LEI/INV) | No standardized subcategories |
| Dissemination controls | Yes (NOFORN, FEDONLY, REL TO, etc.) | No standardized equivalents |
| Governing authority | NARA CUI Registry, 32 CFR Part 2002 | TBS Standard on Security Categorization |
| Injury test | Varies by authorizing law/regulation | Standardized three-tier injury threshold |

### Approximate Correspondences

| Canadian Tier | Approximate US Correspondence | Caveat |
|---|---|---|
| Protected A | CUI Basic (some categories) | PA covers "limited or moderate injury" which maps to some but not all CUI Basic categories. Many CUI Basic categories have no injury-to-individuals dimension. |
| Protected B | CUI Specified (some categories) | PB covers "serious injury" which roughly corresponds to the enhanced handling required by CUI Specified. However, CUI Specified is category-specific (each has its own authorizing law), while PB is a uniform tier. |
| Protected C | No direct CUI equivalent | PC covers "extremely grave injury outside the national interest." US CUI does not have a tier this severe within unclassified. PC sits at the boundary between unclassified and classified. The closest US analog would be information just below the Confidential classification threshold, but this is not a CUI designation. |

FINDING: MAPPING-BREAKS-AT-PC
SEVERITY: High (advisory)
DOMAIN: Canadian Policy
SOURCE: TBS Standard on Security Categorization J.2.4.2.1; 32 CFR Part 2002
DETAIL: Protected C has no structural equivalent in the US CUI system. US CUI tops
out below the classified threshold. Canadian Protected C sits at the classified
boundary ("extremely grave injury ... for example, loss of life"). Any UMRS
documentation that implies a CUI-to-PC mapping exists would be factually incorrect
and could mislead operators handling sensitive material.
REMEDIATION: Document Protected C as a tier with no US CUI equivalent. In tool
output, when a file is marked at s3:c200+, do not display any US CUI correspondence.
Display the Canadian designation only with its injury threshold description.

FINDING: ROADMAP-M3-LANGUAGE-CORRECTION
SEVERITY: Medium (advisory)
DOMAIN: Canadian Policy
SOURCE: ROADMAP.md M3 milestone description
DETAIL: The ROADMAP describes the Canadian deliverable as "Canadian CUI equivalent
labels (3 basic labels -- Five Eyes interop)." This phrasing implies Canada has a CUI
equivalent, which it does not. Canada has Protected A/B/C, which is a structurally
different system. The phrase "Canadian CUI equivalent" could be cited in a security
assessment as evidence that UMRS conflates the two systems.
REMEDIATION: Revise M3 description to: "Canadian Protected designation labels
(Protected A/B/C -- Five Eyes reference mapping)." This accurately describes what
UMRS provides without implying structural equivalence.

---

## 5. Information Type Categories (The c200-c299 Range)

Jamie confirmed: each Protected tier gets the full c200-c299 range (100 categories per tier) for room to breathe.

### Current Answer: Tier Markers Only

For the initial CANADIAN-PROTECTED.json catalog, I recommend defining **tier-level
markers only**, not information-type subcategories. Rationale:

1. **TBS does not define standardized information-type categories.** The Standard on
   Security Categorization defines injury thresholds, not information types. Departments
   apply these thresholds to their own information assets. There is no Canadian equivalent
   of the NARA CUI Registry's category taxonomy.

2. **Creating subcategories without departmental input would be speculative.** We could
   invent categories like "Personnel" (c201), "Financial" (c202), "Medical" (c203) by
   analogy to US CUI, but these would be UMRS conventions without Canadian policy backing.
   The catalog should not contain unsourced categories.

3. **The extension point should be documented.** The catalog should explicitly note that
   categories c201+ within each Protected tier are reserved for future departmental
   subcategory definitions. This makes the architecture extensible without overpromising.

### Confirmed Category Allocation (Jamie, 2026-03-23)

| MCS Label | Meaning | Catalog Entry |
|---|---|---|
| s1:c200 | Protected A (tier marker) | Yes -- primary PA designation |
| s2:c200 | Protected B (tier marker) | Yes -- primary PB designation |
| s3:c200 | Protected C (tier marker) | Yes -- primary PC designation |
| s1:c201-c299 | Reserved: PA subcategories | Documented as extension point, not populated |
| s2:c201-c299 | Reserved: PB subcategories | Documented as extension point, not populated |
| s3:c201-c299 | Reserved: PC subcategories | Documented as extension point, not populated |

Jamie confirmed: c200-c299 per tier. Room to breathe. This gives each tier a 100-category
namespace for future subcategorization, which is generous for a system that currently defines
zero subcategories.

---

## 6. Bilingual Requirements

### Official Languages Act Implications

For a reference system with no Canadian government deployment, OLA compliance is
advisory. However, UMRS already has French Canadian translation infrastructure (Simone's
work), so the following should be applied for credibility:

1. **Designation names must be available in both languages.** The catalog must include
   both "Protected A" and "Protégé A" with proper UTF-8 accents.

2. **Injury threshold descriptions should be available in both languages.** The TBS
   provides authoritative French text for all injury thresholds. Use the TBS text
   verbatim -- do not paraphrase or retranslate.

3. **Tool output language follows locale.** When `LANG=fr_CA`, display "Protégé B".
   When `LANG=en_CA` or `LANG=en_US`, display "Protected B". This is already how
   UMRS handles CUI strings via the i18n pipeline.

4. **All JSON files must use UTF-8 encoding.** Confirmed by Jamie (2026-03-23). French
   accented characters (é, è, ê, etc.) are stored directly in UTF-8, not escaped.

### Terminology Validation

FINDING: TBS-AUTHORITATIVE-FRENCH-TERMS
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: TBS Standard on Security Categorization (French version), J.2.4.2
DETAIL: The TBS uses "Protege A" / "Protege B" / "Protege C" as the authoritative
French designations. These are the terms that must appear in the catalog and in any
French-language tool output. Simone's translations should use these exact terms for
the designation names. Termium Plus may offer alternative translations for the word
"protected" in other contexts, but for the formal designation, the TBS text governs.
This is a case where the policy instrument overrides the terminology database --
a documented exception to the Termium Plus hierarchy.
REMEDIATION: Document this exception in the catalog. Verify Simone's .po files
use "Protege" (with accent) for the Protected designation, not any Termium Plus
alternative.

---

## 7. Recommendations for CANADIAN-PROTECTED.json

Based on this analysis, the catalog should contain:

### Per-Entry Fields

Adapting the US CUI catalog structure (`cui-labels.json`):

```
{
  "designation": "Protected B",
  "designation_fr": "Protégé B",
  "abbreviation": "PB",
  "sensitivity_level": "s2",
  "category_base": "c200",
  "category_range_reserved": "c201-c299",
  "injury_threshold_en": "Serious injury outside the national interest",
  "injury_threshold_fr": "Préjudice sérieux à des intérêts autres que l'intérêt national",
  "injury_examples_en": "Loss of reputation or competitive advantage",
  "injury_examples_fr": "Perte de réputation ou d'avantage concurrentiel",
  "phase1_labeling_only": true,
  "authority": "TBS Standard on Security Categorization, Appendix J, J.2.4.2.2",
  "authority_date": "2019-07-01",
  "handling_summary_en": "Need-to-know basis. Encrypted storage and transmission. Enhanced reliability screening.",
  "handling_summary_fr": "Principe du besoin de connaître. Stockage et transmission chiffrés. Vérification de fiabilité approfondie.",
  "dissemination_controls": null,
  "us_cui_correspondence": "Approximate: CUI Specified (some categories). Not a direct mapping.",
  "notes": "Canada does not define information-type subcategories. Categories c201-c299 reserved for future departmental definitions."
}
```

### What the Catalog Does NOT Include

- No dissemination control entries (Canada does not standardize them)
- No information-type subcategories (TBS does not define them)
- No Classified designations (out of UMRS scope)
- No department-specific handling procedures (UMRS is department-neutral)

---

## 8. Questions Resolved by Jamie (2026-03-23)

1. **Category range:** c200-c299 per tier. Jamie wants room to breathe. c200 is the tier
   marker; c201-c299 reserved for future subcategories.

2. **Protected C in Phase 1:** Yes, include it. Both catalog and documentation will need
   updating later with correct sX definitions, but Phase 1 only reads the "markings" section.
   Include PC with caveat about Phase 2 enforcement.

3. **UTF-8 encoding:** Confirmed. Everything must be UTF-8. French accented characters
   stored directly, not escaped.

---

## Findings Summary

| Finding | Severity | Route To |
|---|---|---|
| STRUCTURAL-DIVERGENCE-HANDLING | Informational | Jamie |
| MAPPING-BREAKS-AT-PC | High (advisory) | Jamie |
| ROADMAP-M3-LANGUAGE-CORRECTION | Medium (advisory) | Jamie |
| TBS-AUTHORITATIVE-FRENCH-TERMS | Informational | Simone, Jamie |

All findings are advisory per Jamie's directive that UMRS is a reference system.
No findings are blockers.
