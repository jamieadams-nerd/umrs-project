# Appendix J Familiarization Report

**Agent:** Henri (Canadian Policy Specialist)
**Date:** 2026-03-31
**Source:** TBS Directive on Security Management -- Appendix J: Standard on Security Categorization (EN)
**Source file:** `.claude/references/five-eyes-classification/tbs-dosm-appendix-j-security-categorization-en.html`
**Authority date:** 2019-07-01

---

## 1. Document Structure and Scope

Appendix J is a standard appended to the TBS Directive on Security Management. It replaced
parts of the Security Organization and Administration Standard (in effect 1995-06-01 to
2019-06-30). It took effect 2019-07-01.

The standard has three procedural sections:

| Section | Title | Content |
|---------|-------|---------|
| J.2.2 | The security categorization process | How to assess and assign categories |
| J.2.3 | General security categories | Impact levels (Low / Medium / High / Very High) |
| J.2.4 | Information confidentiality categories | Classified (C/S/TS) and Protected (A/B/C) |

### Key structural observations

1. **Two separate categorization tracks.** Appendix J defines *general* impact levels
   (Low through Very High, applicable to confidentiality + integrity + availability) and
   *confidentiality-specific* categories (Classified and Protected). These are complementary,
   not alternative. A security categorization exercise may produce both.

2. **The C-I-A triad is examined independently.** Section J.2.2.1 requires examining
   confidentiality, integrity, and availability injury potential *separately*. The Protected
   tiers (A/B/C) pertain specifically to confidentiality loss. Integrity and availability
   injuries are addressed through the general impact levels.

3. **Aggregation is a consideration.** Section J.2.2.4 states that the category for a
   repository must reflect the aggregation effect -- a collection of PA documents may
   collectively warrant PB if aggregate compromise causes greater injury.

4. **ATIP exemption/exclusion criteria.** Section J.2.2.6 requires the categorization to
   consider Access to Information Act and Privacy Act exemption/exclusion criteria to avoid
   over-protecting information that could be made public.

---

## 2. General Security Categories (J.2.3)

These are the four impact levels applicable across the C-I-A triad:

| Impact Level | Injury Threshold (verbatim) | Ref |
|--------------|----------------------------|-----|
| **Very High** | "severe to exceptionally grave injury" | J.2.3.1.1 |
| **High** | "serious to severe injury" | J.2.3.1.2 |
| **Medium** | "moderate to serious injury" | J.2.3.1.3 |
| **Low** | "limited to moderate injury" | J.2.3.1.4 |

Non-sensitive information (no injury from compromise) may be assigned "Low" for categorization
purposes (J.2.3.2).

---

## 3. Information Confidentiality Categories (J.2.4)

### 3.1 The National Interest Dividing Line

The defining structural feature of the Canadian system is the **national interest boundary**:

- **Classified** (Confidential / Secret / Top Secret): injury to **the national interest**
- **Protected** (A / B / C): injury **outside the national interest**

This is the single most important policy distinction. It determines which track applies.

### 3.2 Classified Tiers (for reference)

| Tier | Injury Threshold (verbatim) | Ref |
|------|----------------------------|-----|
| **Top Secret** | "exceptionally grave injury to the national interest" | J.2.4.1.1 |
| **Secret** | "serious injury to the national interest" | J.2.4.1.2 |
| **Confidential** | "limited or moderate injury to the national interest" | J.2.4.1.3 |

Note: Appendix J lists Top Secret as applying to "the very limited amount of information."

### 3.3 Protected Tiers -- Exact Definitions

**IMPORTANT:** Appendix J lists Protected tiers in *descending* severity order (C, B, A).

#### Protected C (J.2.4.2.1)

> "Applies to the very limited amount of information when unauthorized disclosure could
> reasonably be expected to cause extremely grave injury outside the national interest,
> for example, loss of life."

Key elements:
- Injury level: **extremely grave**
- Scope: **outside the national interest** (not classified)
- Volume qualifier: "the very limited amount of information" (parallels Top Secret language)
- Canonical example: **loss of life**

#### Protected B (J.2.4.2.2)

> "Applies to information when unauthorized disclosure could reasonably be expected to cause
> serious injury outside the national interest, for example, loss of reputation or competitive
> advantage."

Key elements:
- Injury level: **serious**
- Scope: **outside the national interest**
- Canonical examples: **loss of reputation or competitive advantage**

#### Protected A (J.2.4.2.3)

> "Applies to information when unauthorized disclosure could reasonably be expected to cause
> limited or moderate injury outside the national interest, for example, disclosure of an
> exact salary figure."

Key elements:
- Injury level: **limited or moderate**
- Scope: **outside the national interest**
- Canonical example: **disclosure of an exact salary figure**

---

## 4. Comparison: Appendix J vs CANADIAN-PROTECTED.json

Two catalog files exist. The current/active catalog is at:
`components/rusty-gadgets/umrs-label/config/ca/CANADIAN-PROTECTED.json` (v0.2.0)

An older prototype exists at:
`components/rust-prototypes/umrs-cui/data/ca/CANADIAN-PROTECTED.json` (v0.1.0)

The analysis below focuses on the current v0.2.0 catalog.

### 4.1 Definition Accuracy

| Field | Appendix J (verbatim) | v0.2.0 Catalog | Status |
|-------|----------------------|-----------------|--------|
| **PA description** | "limited or moderate injury outside the national interest" | Matches verbatim | ACCURATE |
| **PA example** | "disclosure of an exact salary figure" | Matches | ACCURATE |
| **PA authority_section** | J.2.4.2.3 | Matches | ACCURATE |
| **PB description** | "serious injury outside the national interest" | Matches verbatim | ACCURATE |
| **PB example** | "loss of reputation or competitive advantage" | Matches | ACCURATE |
| **PB authority_section** | J.2.4.2.2 | Matches | ACCURATE |
| **PC description** | "extremely grave injury outside the national interest, for example, loss of life" | Matches verbatim | ACCURATE |
| **PC example** | "Loss of life" | Matches | ACCURATE |
| **PC authority_section** | J.2.4.2.1 | Matches | ACCURATE |
| **PC volume qualifier** | "the very limited amount of information" | **Not captured in catalog** | GAP |
| **Mutual exclusivity** | Implied by tier structure (one tier per asset) | Explicitly documented | ACCURATE (UMRS is more explicit, which is fine) |
| **Aggregation rule** | J.2.2.4 -- repository categorization reflects aggregation | **Not captured** | GAP |
| **ATIP consideration** | J.2.2.6 -- consider ATIP exemption/exclusion criteria | Mentioned in mutual_exclusivity.atip_note | PARTIAL |

### 4.2 MCS Category Assignment

| Tier | v0.2.0 category_base | Constraint rule (c300-c399) | Status |
|------|---------------------|----------------------------|--------|
| PA | c300 | Within c300-c399 range | COMPLIANT |
| PB | c301 | Within c300-c399 range | COMPLIANT |
| PC | c302 | Within c300-c399 range | COMPLIANT |
| Reserved | c303-c399 | Documented for departmental subcategories | COMPLIANT |

Note: The older v0.1.0 prototype uses c200-c299. The current v0.2.0 correctly uses c300-c399
per the constraint in `labeling_mcs.md`.

### 4.3 Sensitivity Level Mapping

| Tier | v0.2.0 level | Appendix J basis | Status |
|------|-------------|-----------------|--------|
| PA | s1 | No direct mapping in Appendix J | DESIGN DECISION |
| PB | s2 | No direct mapping in Appendix J | DESIGN DECISION |
| PC | s3 | No direct mapping in Appendix J | DESIGN DECISION |

Appendix J does not define MLS sensitivity levels. The s1/s2/s3 mapping is a UMRS design
decision for future MLS enforcement. This is documented in the catalog's `_metadata.notes`
and is appropriate. Under Phase 1 targeted policy, all categories exist at s0 per the
SELinux axiom.

### 4.4 French Terminology

| Tier | v0.2.0 name_fr | v0.2.0 marking_banner_fr | Notes |
|------|---------------|------------------------|-------|
| PA | Protege A | PROTEGE A | Correct with accents in file |
| PB | Protege B | PROTEGE B | Correct with accents in file |
| PC | Protege C | PROTEGE C | Correct with accents in file |

French terms use proper accent aigu on both E characters. This aligns with TBS authoritative
French terminology. Validation of French descriptions against the FR version of Appendix J
is a separate task (assigned to Simone for linguistic accuracy, Henri for policy accuracy).

---

## 5. setrans.conf Analysis

The setrans.conf at `components/rusty-gadgets/libs/umrs-selinux/config/setrans.conf`
contains **no Canadian Protected entries**. All entries are US CUI categories in the
c0-c249 range.

### 5.1 Missing Canadian Entries

Per the labeling rules, Canadian entries should appear in the c300-c399 range.
The setrans.conf needs entries such as:

```
s0:c300 = PROTECTED A
s0:c301 = PROTECTED B
s0:c302 = PROTECTED C
```

Or in French-capable deployments:

```
s0:c300 = PROTEGE A
s0:c301 = PROTEGE B
s0:c302 = PROTEGE C
```

This is a gap. The JSON catalog defines the categories but the setrans.conf does not
translate them for `chcat -L` or audit log display.

### 5.2 setrans.conf Language Question

Appendix J does not address this, but the setrans.conf language is a policy decision:

- On an `en_CA` system, the marking should be `PROTECTED A` / `PROTECTED B` / `PROTECTED C`
- On a `fr_CA` system, the marking should be `PROTEGE A` / `PROTEGE B` / `PROTEGE C`
- setrans.conf is a single static file -- it does not support locale switching

This creates a Five Eyes interoperability question. If a Canadian system displays
`PROTEGE B` in audit logs and those logs are shared with a US system, the US analyst
must know that `PROTEGE B` = `PROTECTED B`. Route to Jamie for decision on whether
setrans.conf should use English markings (interoperability) or French markings
(Official Languages Act compliance on fr_CA systems).

---

## 6. Findings

### FINDING: Protected C volume qualifier not captured in catalog
SEVERITY: Low
DOMAIN: Canadian Policy
SOURCE: TBS Appendix J, section J.2.4.2.1
DETAIL: Appendix J states Protected C "applies to the very limited amount of information."
This volume qualifier parallels the Top Secret language and signals that PC should be rare.
The CANADIAN-PROTECTED.json catalog does not capture this policy signal. While it does not
change handling requirements, it is relevant for categorization guidance -- operators should
understand that PC is expected to be uncommon.
REMEDIATION: Add a `volume_qualifier` or `usage_note` field to the PC entry noting that
Appendix J characterizes this tier as applicable to "the very limited amount of information."

### FINDING: Aggregation rule not captured in catalog
SEVERITY: Low
DOMAIN: Canadian Policy
SOURCE: TBS Appendix J, section J.2.2.4
DETAIL: Appendix J requires that repository categorization reflect the aggregation effect --
a collection of lower-tier information may warrant a higher tier if aggregate compromise
causes greater injury. The catalog does not reference this rule. This is relevant for any
UMRS feature that manages collections of protected files (vaults, directories).
REMEDIATION: Add an `aggregation_rule` field or a note in `_metadata.notes` documenting
the J.2.2.4 aggregation consideration.

### FINDING: No Canadian entries in setrans.conf
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: labeling_mcs.md constraint; CANADIAN-PROTECTED.json catalog
DETAIL: The setrans.conf file contains US CUI entries (c0-c249) but no Canadian Protected
entries (c300-c399). Any file labeled with Canadian MCS categories will display as raw
`s0:cNNN` in `chcat -L` output and audit logs rather than human-readable markings.
REMEDIATION: Add Canadian Protected entries to setrans.conf in the c300-c302 range. Decide
on EN vs FR marking language (route to Jamie -- Five Eyes interoperability vs OLA compliance).

### FINDING: setrans.conf language for Canadian markings is an unresolved policy question
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: Official Languages Act; Five Eyes information sharing conventions
DETAIL: setrans.conf is a static file that cannot switch locale. Canadian Protected markings
must appear in either English or French. English maximizes Five Eyes interoperability. French
is required by the Official Languages Act on fr_CA systems. This is a policy decision that
affects audit log readability across national boundaries.
REMEDIATION: Route to Jamie. Options: (a) English-only setrans.conf for interoperability,
(b) two setrans.conf variants shipped per locale, (c) English markings with French
cross-reference in a separate mechanism.

### FINDING: General impact levels (Low/Medium/High/Very High) not represented in catalog
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: TBS Appendix J, section J.2.3
DETAIL: Appendix J defines four general impact levels (Low, Medium, High, Very High) that
apply across the C-I-A triad, separate from the confidentiality-specific Protected tiers.
The CANADIAN-PROTECTED.json catalog only addresses confidentiality categories (Protected
A/B/C). The general impact levels are not represented. This is acceptable for a
confidentiality-focused catalog, but should be documented as a scope limitation.
REMEDIATION: No immediate action required. Add a note to `_metadata.notes` clarifying that
the catalog covers information confidentiality categories only (J.2.4), not general security
categories (J.2.3).

### FINDING: Prototype catalog (v0.1.0) uses outdated c200-c299 range
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: labeling_mcs.md constraint on Canadian range c300-c399
DETAIL: The older prototype at `components/rust-prototypes/umrs-cui/data/ca/CANADIAN-PROTECTED.json`
(v0.1.0) still uses `c200` as category_base and `c201-c299` as reserved range. This conflicts
with the established constraint that Canadian Protected uses c300-c399. If any code references
this prototype file, it will produce incorrect MCS category assignments.
REMEDIATION: Either delete the prototype file or update it to match v0.2.0's c300-c399 range.
Flag to Jamie for decision on prototype file lifecycle.

---

## 7. Terminology Notes for Label Catalog

### Injury language precision

Appendix J uses a specific vocabulary for injury levels. The exact terms matter for
policy fidelity:

| Tier | Appendix J injury term | Notes |
|------|----------------------|-------|
| PA | "limited or moderate" | Two-word range, not just "limited" |
| PB | "serious" | Single term |
| PC | "extremely grave" | Not just "grave" -- the adverb is load-bearing |

The catalog correctly captures these. Do not paraphrase.

### "Outside the national interest" vs "to the national interest"

This phrase is the definitional boundary between Protected and Classified. Every Protected
tier definition includes "outside the national interest." Every Classified tier includes
"to the national interest." The catalog descriptions correctly include this phrase. It must
never be dropped in any summary, UI display, or translation.

### "Could reasonably be expected to cause"

This is the Canadian injury test standard -- it appears in every tier definition. It is a
legal standard of proof (reasonable expectation), not certainty. The catalog correctly uses
this language.

### Banner marking form

Appendix J uses "Protected A" / "Protected B" / "Protected C" with a non-breaking space
(rendered as `&nbsp;` in the HTML source). The catalog uses both full form (`name` field)
and uppercase banner form (`marking_banner_en` / `marking_banner_fr` fields). Both are
correct representations.

---

## 8. Cross-Reference: General Impact Levels to Protected Tiers

For completeness, here is the approximate correspondence between the two Appendix J
categorization tracks:

| Protected Tier | Approximate General Impact Level | Basis |
|---------------|--------------------------------|-------|
| Protected A | Low to Medium | "limited or moderate" maps to Low-Medium range |
| Protected B | Medium to High | "serious" maps to Medium-High overlap |
| Protected C | Very High | "extremely grave" maps to Very High |

This is an approximation. The general impact levels and Protected tiers serve different
purposes (C-I-A breadth vs confidentiality-specific) and the injury language does not
align perfectly. Do not treat this as a formal mapping.

---

## 9. Summary Assessment

The CANADIAN-PROTECTED.json v0.2.0 catalog is **substantially accurate** relative to
Appendix J. The core definitions, injury thresholds, examples, and authority section
references are correct. The structural metadata (mutual exclusivity, extension points,
US comparison notes) is well-documented.

The gaps identified are:
1. **Protected C volume qualifier** -- minor omission, easy to add
2. **Aggregation rule** -- minor omission, relevant for vault/collection features
3. **Missing setrans.conf entries** -- functional gap requiring action
4. **setrans.conf language policy** -- unresolved policy question for Jamie
5. **General impact levels not in scope** -- by design, but should be documented
6. **Prototype file with outdated category range** -- cleanup item

No findings cross into security control territory (Herb/Knox domain). All findings are
Canadian policy scope and route to Jamie.
