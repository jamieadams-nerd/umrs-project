# UMRS US CUI Labels Catalog — Correctness and Consistency Audit

**Date:** 2026-03-30
**Reviewer:** Knox (security-auditor)
**File audited:** `components/rusty-gadgets/umrs-label/config/us/US-CUI-LABELS.json`
**Reference:** `.claude/jamies_brain/cui_cleanup/nara-cui-merged-reference.md`
**Reference:** `.claude/jamies_brain/cui_cleanup/nara-limited-dissemination-controls.md`
**Reference:** `.claude/jamies_brain/cui_cleanup/nara-cui-category-details-ldc-dist.md`
**Catalog version audited:** 0.3.0 (updated 2026-03-30)

---

## Summary Table

| Category | ACCURATE | CONCERN | ERROR |
|---|---|---|---|
| Structural consistency | — | 1 | 0 |
| Designation correctness | — | 2 | 1 |
| Abbreviation format | 143 | 0 | 0 |
| Banner key correctness | — | 0 | 0 |
| Index group consistency | — | 0 | 1 |
| Warning statements | — | 3 | 1 |
| Dissemination controls | — | 3 | 1 |
| Missing categories | — | 0 | 0 |
| MCS range metadata | — | 0 | 1 |
| **TOTALS** | — | **9** | **5** |

---

## Detailed Findings

### Errors

---

**E-1: `mcs_ranges` metadata contradicts `labeling_mcs.md` constraint**

The `_metadata.mcs_ranges.categories` field declares `"c0-c249"` for US CUI categories. The project constraint in `.claude/rules/labeling_mcs.md` explicitly assigns:

- US CUI: `c0–c199`
- Canadian Protected: `c200–c299`

The metadata claims US CUI consumes `c0–c249`, which overlaps 50 categories into the Canadian Protected range (`c200–c249`). If this metadata is consumed by any code that allocates MCS categories, it will produce category assignments that collide with the Canadian catalog.

**Severity:** ERROR — this is a hard numerical conflict with the stated constraint.
**Location:** `_metadata.mcs_ranges.categories` (line 9)
**Recommended fix:** Change to `"c0-c199"` to match the constraint in `labeling_mcs.md`.
**Owner:** Jamie (configuration decision — this may be an intentional range expansion, but if so the constraint must also be updated)

---

**E-2: `CUI//SP-EXPT` has an incorrect `required_dissemination_control` value**

`CUI//SP-EXPT` sets `required_dissemination_control` to `"Distribution Statements B through F"`. Distribution Statements B–F are a DoD mechanism defined in DoDI 5230.24 and apply specifically to **Controlled Technical Information (CTI)**. The NARA category detail scrape at `nara-cui-category-details-ldc-dist.md` confirms that Export Controlled (EXPT) has no Distribution Statement requirement — only CTI does.

Export Controlled documents may carry NOFORN or REL TO dissemination controls when applicable, but there is no mandatory distribution statement requirement for EXPT. The value present is a factual error that could cause UMRS to render incorrect markings on export-controlled documents.

**Severity:** ERROR — incorrect mandatory field value affecting marking correctness.
**Location:** `CUI//SP-EXPT` → `required_dissemination_control` (line 995)
**Recommended fix:** Set to `null`. The warning statement is correct and sufficient.
**Owner:** Jamie

---

**E-3: `CUI//ADPO` designation conflict with dual-status NARA entry**

The NARA merged reference shows Administrative Proceedings (ADPO) has both a specified authority (`CUI//SP-ADPO`) and a basic authority (`CUI`). The JSON correctly creates two entries — `CUI//ADPO` (basic) and `CUI//SP-ADPO` (specified). However, the NARA basic banner for ADPO is `CUI` (the umbrella), not `CUI//ADPO`. Using `CUI//ADPO` as a banner marking for the basic tier is a UMRS convention that the metadata notes acknowledge, but the `designation` field on `CUI//ADPO` reads `"basic"` while the actual NARA marking for basic-tier ADPO is the unadorned `CUI`.

This is a narrow issue: if downstream code uses the JSON key as the authoritative banner marking, operators marking basic ADPO documents would mark them `CUI//ADPO` where NARA's guidance produces just `CUI`. For basic categories, NARA permits (but does not require) the category marking in the banner. The existing approach is defensible as a UMRS policy decision to always use specific markings, but this should be explicitly stated in the `description` field to prevent operator confusion.

**Severity:** ERROR (marking accuracy) — but remediation is documentation, not data deletion.
**Location:** `CUI//ADPO` → `description` (line 54–60); same issue applies to all basic entries whose NARA basic-tier marking is `CUI` (umbrella), not `CUI//ABBREV`.
**Recommended fix:** Add a sentence to the `CUI` umbrella entry and/or `_metadata.notes` clarifying that for dual-status categories, the basic `CUI//ABBREV` key represents the UMRS convention of always using specific markings — NARA permits just `CUI` for the basic tier. This context is necessary for any operator reading the catalog.
**Owner:** Jamie

---

**E-4: `CUI//ADPO` — `designation: "basic"` without a basic authority in NARA for ADPO alone**

Expanding on E-3: the NARA merged reference for Administrative Proceedings shows the basic authority column as `CUI` — meaning the basic banner is the umbrella CUI, not `CUI//ADPO`. The catalog entry `CUI//ADPO` with `designation: "basic"` implies that `CUI//ADPO` is a valid NARA banner marking for basic-tier ADPO. This is not what NARA specifies.

The same pattern applies to all entries where NARA's "Basic Authority" column shows only `CUI` (not `CUI//ABBREV`): ADJ, ADPO, AG, APP, ARCHR, ASYL, BARG, BATT, CMPRS, COMPT, CONREG, CONV, CRIT, CVIC, DCNI (basic), DCRIT, DREC, EMGT, EXPT (basic), EXPTR, FHFANPI, FINT, FNC (basic), FSI, GENETIC (basic), HLTH (basic), ID (basic), INF (basic), INTEL (basic), INV (basic), INVENT, ISVI, IVIC, JURY (basic), JUV, LCOMM, LEI, LMI, LNSL, LPROT (basic), LSCRN, LVIC, MERG, MIL, NUC (basic), OCCMTO, OPSEC, PERS (basic), PEST, PHYS, POST, PRE, PRIIG (basic), PRIOR, PRIVILEGE, PROPIN (basic), PROT, PRVCY (basic), PSEC, PSI, RAIL, RECCOM, RESD, RTR, RWRD, SAFE, SBIZ, SCV, SERV, SRI (basic), SSEL (basic), STAT (basic), STUD (basic), SURV, TAI, TRACE, UCNI (basic), VISA, WATER, WHSTL (basic), WIT (basic), XFER.

This is a systemic design decision that needs explicit documentation. This finding consolidates to E-3 — the `_metadata.notes` must explicitly address this convention.

**Note:** This is being called out as a single ERROR rather than ~80 individual errors because it is a catalog-wide design decision, not 80 independent mistakes. One metadata note fix addresses all instances.

---

**E-5: `CUI//SP-AIV` index group is incorrect**

`CUI//SP-AIV` (Accident Investigation) is assigned `"index_group": "Law Enforcement"`. The NARA merged reference places Accident Investigation (AIV) under the **Law Enforcement** index group. This appears correct at first glance, but the NARA category list actually places AIV under **Law Enforcement**. Checking the merged reference: `| Accident Investigation | AIV | CUI//SP-AIV | |` is listed under `## Law Enforcement`. The JSON entry at line 859 also says `Law Enforcement`.

Retracting — this is correct. Removing E-5 from ERROR count.

---

### Concerns

---

**C-1: `handling_group_id` uses empty string `""` inconsistently with null pattern**

The catalog uses `null` for absent `required_warning_statement` and `required_dissemination_control` fields throughout. However, `handling_group_id` uses `""` (empty string) for entries without a handling group, while using named identifiers (`"CUI-BASE"`, `"CTI-GROUP"`, `"NNPI-GROUP"`) for a handful of entries. Empty string and null carry different semantics in most JSON consumers. If the companion `US-CUI-HANDLING.json` is loaded by a parser that checks for null/empty, the inconsistency may cause silent failures.

**Recommendation:** Standardize to `null` for absent `handling_group_id` values, matching the null pattern used by other optional fields.
**Location:** All entries with `"handling_group_id": ""` (majority of the catalog)
**Owner:** Jamie

---

**C-2: `CUI//SP-PCII` missing required warning statement**

Protected Critical Infrastructure Information (PCII) is governed by 6 CFR Part 29. Section 29.8 specifies that documents containing PCII must display a marking that includes the text that the information is protected from disclosure under the Critical Infrastructure Information Act of 2002 (6 U.S.C. 131-134). The JSON entry for `CUI//SP-PCII` has `required_warning_statement: null`.

This is similar in structure to CVI (which has a correct warning statement) and SSI (which also has a correct warning). PCII is at least as sensitive as CVI and has a statutory disclosure prohibition.

**Recommendation:** Research the exact PCII statutory warning text from 6 CFR 29.8 and populate `required_warning_statement`.
**Location:** `CUI//SP-PCII` → `required_warning_statement` (line 1259)
**Owner:** Jamie

---

**C-3: `CUI//SP-SGI` missing warning statement**

Safeguards Information (SGI) is governed by 10 CFR 73.21 and 10 CFR 2.390. NRC Regulatory Guide 5.29 specifies the exact marking text that must appear on SGI documents. The JSON entry has `required_warning_statement: null`. Given that SGI is Specified (not Basic) and has a statutory basis with specific marking requirements, this gap is likely to cause incorrect labeling in a production system.

**Recommendation:** Populate with NRC-specified SGI marking text from 10 CFR 73.21 or RG 5.29.
**Location:** `CUI//SP-SGI` → `required_warning_statement` (line 1325)
**Owner:** Jamie

---

**C-4: `CUI//SP-NNPI` missing warning statement**

Naval Nuclear Propulsion Information (NNPI) carries statutory handling requirements under the Naval Nuclear Propulsion Program. NNPI documents are expected to carry a specific marking statement. The basic entry `CUI//NNPI` having no warning is defensible, but the Specified entry `CUI//SP-NNPI` with `required_warning_statement: null` may be incomplete.

**Recommendation:** Confirm whether the NNPI governing authority specifies a mandatory warning statement; populate if so.
**Location:** `CUI//SP-NNPI` → `required_warning_statement` (line 1226)
**Owner:** Jamie

---

**C-5: `CUI//SP-EXPT` warning statement conflates EAR and ITAR without differentiating**

The `required_warning_statement` for `CUI//SP-EXPT` reads:

> "WARNING: This document contains technical data whose export is restricted by the Arms Export Control Act (Title 22, U.S.C., Sec 2751, et seq.) or the Export Control Reform Act of 2018. Violations of these export laws are subject to severe criminal penalties."

This statement presents ITAR (Arms Export Control Act) and EAR (Export Control Reform Act / 15 CFR 730-774) as alternatives ("or"). In practice:

- ITAR-controlled items use ITAR-specific warning language citing 22 CFR 120-130.
- EAR-controlled items use EAR-specific warning language citing 15 CFR 730-774.

The correct warning depends on the governing authority for the specific item. A single combined warning using "or" is ambiguous and may not satisfy the specific statutory requirements of either regime for a given document. This is a particularly consequential gap because EXPT is a common category that operators will see frequently.

**Recommendation:** Either (a) create two entries `CUI//SP-EXPT-ITAR` and `CUI//SP-EXPT-EAR` with regime-specific warnings, or (b) document clearly that the combined warning is a conservative placeholder and that operators must substitute the regime-appropriate statement when marking actual documents.
**Location:** `CUI//SP-EXPT` → `required_warning_statement` (line 994)
**Owner:** Jamie

---

**C-6: `CUI//SP-CTI` has no `required_warning_statement`**

Controlled Technical Information (CTI) under `CUI//SP-CTI` has `required_warning_statement: null` despite having a `required_dissemination_control` value. DoD documents marked with CTI typically carry a Distribution Statement block (e.g., "Distribution authorized to DoD components only") as the human-readable notice. This notice is functionally a warning statement. Its absence from the field means any system that generates document headers from this catalog will produce CTI-marked documents without the required DoD distribution notice.

**Recommendation:** Populate with the standard Distribution Statement C (or the appropriate statement) text per DoDI 5230.24.
**Location:** `CUI//SP-CTI` → `required_warning_statement` (line 961)
**Owner:** Jamie

---

**C-7: `DISPLAY ONLY` dissemination control — missing mutual exclusivity with `NOFORN`**

Wait — checking the JSON: `DISPLAY ONLY` already lists `"mutually_exclusive_with": ["NOFORN"]` at line 1631, and `NOFORN` lists `"mutually_exclusive_with": ["REL TO", "DISPLAY ONLY"]` at line 1676. This is correct and symmetric.

Retracting — not a finding.

---

**C-7: `RELIDO` dissemination control — missing mutual exclusivity with `NOFORN`**

`RELIDO` is a permissive foreign disclosure marking — it explicitly authorizes a disclosure official to release the information to foreign parties. `NOFORN` explicitly prohibits foreign disclosure. These two controls are logically incompatible on the same document. The JSON entry for `RELIDO` has `"mutually_exclusive_with": []`, meaning no conflicts are declared.

The NARA LDC reference does not explicitly list this mutual exclusivity, but it is a logical policy consequence. If UMRS generates or validates markings, a document with both NOFORN and RELIDO would pass validation but carry contradictory handling instructions.

**Recommendation:** Flag `NOFORN` as mutually exclusive with `RELIDO`, symmetric with the existing `NOFORN` → `["REL TO", "DISPLAY ONLY"]` pattern.
**Location:** `RELIDO` → `mutually_exclusive_with` (line 1696)
**Owner:** Jamie

---

**C-8: `DISPLAY ONLY` and `REL TO` — `mutually_exclusive_with` asymmetry**

`DISPLAY ONLY` lists `"mutually_exclusive_with": ["NOFORN"]` but does not list `"REL TO"`. `REL TO` lists `"mutually_exclusive_with": ["NOFORN"]` but does not list `"DISPLAY ONLY"`. DISPLAY ONLY and REL TO are not mutually exclusive (a document can be display-only to a specific set of nations), so this is correct — no finding here.

Retracting.

---

**C-8: `CUI//PRIVILEGE` `required_dissemination_control` field uses narrative text instead of LDC key references**

The `required_dissemination_control` field for `CUI//PRIVILEGE` contains:

> "LDC restricted to Attorney-Client (AC) and/or Attorney Work Product (AWP) only. These LDCs may only be used with this category."

This is narrative documentation text, not a structured value. Other entries that use this field store the value as a formatted string (e.g., `"Distribution Statements B through F per DoD Instruction 5230.24"`). The field has no defined schema, but the narrative style is inconsistent with the more terse values elsewhere.

More importantly, this field is being used to express a bidirectional constraint (these LDCs can ONLY be used with PRIVILEGE, and PRIVILEGE should ONLY use these LDCs). The `category_restriction` field in the `dissemination_controls` section already expresses the AC/AWP → PRIVILEGE binding from the LDC side. The PRIVILEGE entry is adding the reverse direction narratively, which may cause double-processing or confusion.

**Recommendation:** Clarify whether `required_dissemination_control` on a marking entry means "required to appear" or "restricted to". If it means "this LDC must appear on this category", that changes the semantics from the LDC side's `category_restriction`. If it means something else, document the intended semantics in `_metadata.notes`.
**Location:** `CUI//PRIVILEGE` → `required_dissemination_control` (line 687)
**Owner:** Jamie

---

**C-9: `CUI//SP-TAX` — no warning statement despite 26 U.S.C. § 6103 statutory prohibition**

Federal taxpayer information under IRC § 6103 carries one of the most stringent statutory disclosure prohibitions in U.S. law — unauthorized disclosure is a federal crime (26 U.S.C. § 7213). IRS documents containing FTI are expected to carry a specific statutory warning. The JSON entry has `required_warning_statement: null`.

The description correctly cites 26 U.S.C. 6103, which makes the absent warning more notable.

**Recommendation:** Populate with the standard IRS/FTI warning statement citing 26 U.S.C. §§ 6103 and 7213.
**Location:** `CUI//SP-TAX` → `required_warning_statement` (lines 1393–1402)
**Owner:** Jamie

---

## Strengths Worth Preserving

**Accurate designation coverage.** Every entry's `designation` field correctly reflects the NARA registry's basic/specified classification. The dual-status pattern (separate entries for basic and specified tiers) is correctly applied to all dual-status categories: ADPO, CVI, DCNI, EXPT, FNC, FSEC, GENETIC, HLTH, ID, INF, INTEL, INV, JURY, LPROT, NNPI, NUC, PERS, PRIIG, PROCURE (specified-only), PROPIN, PRVCY, SGI (specified-only), SRI, SSEL, STAT, STUD, UCNI, WHSTL, WIT. No dual-status category is missing its counterpart.

**NATO and Provisional correctly excluded.** The metadata notes document the intentional exclusion of NATO categories (no finalized abbreviations) and Provisional categories. The exclusion is clean — no partial or placeholder entries.

**Abbreviation format compliance.** All 143 `abbrv_name` values are uppercase, under 15 characters, and specified entries correctly carry the `SP-` prefix with no other non-letter characters.

**Banner key correctness.** Every JSON key matches `CUI//` + the `abbrv_name` value. The key-to-field consistency is perfect across the entire catalog. This is a non-trivial property for a 143-entry file.

**Accurate index group assignments.** Every entry's `index_group` matches the NARA merged reference section headings. No category is misassigned.

**Correct LDC mutual exclusivity for the primary set.** NOFORN ↔ REL TO and NOFORN ↔ DISPLAY ONLY are symmetric. FED ONLY ↔ FEDCON and FEDCON ↔ NOCON are symmetric. The core mutual exclusivity graph is correct.

**Parameterized LDC metadata.** REL TO and DISPLAY ONLY both have `parameter_format` and `parameter_example` fields populated with correct guidance (USA first, trigraph country codes alphabetical, tetragraph org codes alphabetical). This is operationally important for correct banner generation.

**Warning statements that exist are substantively correct.** The three warning statements present (CVI, DCNI, SSI, UCNI, EXPT) contain accurate statutory citations. CVI cites 6 CFR 27.400 correctly. DCNI cites 10 U.S.C. 128 and 5 U.S.C. 552(b)(3) correctly. SSI cites 49 CFR parts 15 and 1520 correctly. UCNI cites 42 U.S.C. 2168 (section 148 of the Atomic Energy Act) correctly. The EXPT warning statement has an accuracy concern (C-5) but the statutory citations present are valid law.

**NETW entry correctly modeled.** Net Worth (NETW) is an unusual case where NARA explicitly shows the basic banner as `CUI//NETW` (not just `CUI`). The JSON correctly creates only a basic entry with the specific marking, matching the NARA registry's non-standard basic banner treatment for this category.

---

## Gap Analysis Summary

**File reviewed:** 1 (`US-CUI-LABELS.json`)
**Total entries reviewed:** 153 (143 markings + 10 dissemination controls)
**Total findings:** 13 (4 ERROR, 9 CONCERN)

**Errors:**
- E-1: `mcs_ranges.categories` declares `c0–c249`, conflicting with `labeling_mcs.md` constraint of `c0–c199`
- E-2: `CUI//SP-EXPT` has incorrect `required_dissemination_control` (Distribution Statements B–F belong to CTI, not EXPT)
- E-3/E-4: Systemic — 80+ basic entries use `CUI//ABBREV` format as the banner key where NARA's basic-tier marking is just `CUI`; this is a defensible UMRS convention but must be documented in `_metadata.notes` to prevent operator misinterpretation
- (E-5 retracted after re-verification)

**Concerns:**
- C-1: `handling_group_id: ""` vs. `null` inconsistency across the catalog
- C-2: `CUI//SP-PCII` — missing warning statement (6 CFR 29.8)
- C-3: `CUI//SP-SGI` — missing warning statement (10 CFR 73.21)
- C-4: `CUI//SP-NNPI` — possible missing warning statement (needs verification)
- C-5: `CUI//SP-EXPT` warning conflates ITAR/EAR without differentiation
- C-6: `CUI//SP-CTI` — `required_warning_statement` null despite mandatory distribution statement requirement
- C-7: `RELIDO` — missing `NOFORN` in `mutually_exclusive_with`
- C-8: `CUI//PRIVILEGE` — `required_dissemination_control` field uses narrative text; bidirectional constraint semantics unclear
- C-9: `CUI//SP-TAX` — missing warning statement despite 26 U.S.C. § 6103/7213 statutory prohibition

**Inconsistencies (catalog vs. rules):**
- `mcs_ranges.categories: "c0-c249"` vs. `labeling_mcs.md` constraint `c0–c199` (E-1)
- UMRS basic-marking convention (`CUI//ABBREV` keys) vs. NARA's bare `CUI` basic banner (E-3/E-4, requires documentation)

**Priority remediation order:**
1. E-1 (MCS range conflict — could corrupt Canadian catalog allocation)
2. E-2 (incorrect distribution statement on EXPT — marking accuracy)
3. C-9 (FTI warning — statutory criminal liability)
4. C-2 (PCII warning — statutory disclosure prohibition)
5. C-3 (SGI warning — NRC regulatory requirement)
6. C-6 (CTI distribution notice — DoD marking requirement)
7. E-3/E-4 (documentation of basic marking convention — operator safety)
8. C-5 (EXPT ITAR/EAR differentiation — operator clarity)
9. C-1 (handling_group_id null normalization — parser safety)
10. C-7 (RELIDO mutual exclusivity — marking validation logic)
11. C-8 (PRIVILEGE LDC semantics — schema clarity)
12. C-4 (NNPI warning — needs research before action)

---

*Report written by Knox (security-auditor). All findings are recommendations to the catalog owner (Jamie). No source files were modified.*

---

## Rules File Review: cui-taxonomy-and-rules.md

**Date:** 2026-03-30
**Reviewer:** Knox (security-auditor)
**File reviewed:** `.claude/rules/cui-taxonomy-and-rules.md`
**References cross-checked:**
- `.claude/jamies_brain/cui_cleanup/nara-cui-merged-reference.md`
- `.claude/jamies_brain/cui_cleanup/nara-cui-category-details-ldc-dist.md`
- `.claude/jamies_brain/cui_cleanup/nara-limited-dissemination-controls.md`
- `.claude/jamies_brain/cui_cleanup/nara-cui-category-marking-list.tsv`
- `components/rusty-gadgets/umrs-label/config/us/US-CUI-LABELS.json`

---

### Summary Table

| Section | ACCURATE | CONCERN | ERROR |
|---|---|---|---|
| Banner syntax rules | 5 | 0 | 0 |
| Basic vs Specified | 3 | 1 | 0 |
| Dual-status table | 1 | 1 | 0 |
| Combining categories anti-patterns | 1 | 0 | 1 |
| LDC section — Big 6 | 1 | 1 | 0 |
| LDC section — Additional LDCs | 3 | 1 | 0 |
| LDC mutual exclusivity rules | 2 | 0 | 0 |
| Distribution Statements | 3 | 0 | 0 |
| Access control model | 3 | 0 | 0 |
| JSON field mapping | 2 | 1 | 0 |
| Index group table — counts | 18 | 0 | 0 |
| Index group exclusions | 2 | 0 | 0 |
| **TOTALS** | **44** | **5** | **1** |

---

### Errors

---

**E-1: LEI is described as an index group — it is a category abbreviation**

Location: line 175–176, Combining Categories section

The rules file contains this anti-pattern:

```
[ANTI-PATTERN] `CUI//LEI/INV` — LEI (Law Enforcement) is an index group, not a category.
If the data is about an investigation, the marking is `CUI//INV`.
```

This is factually wrong. LEI is the category abbreviation for **General Law Enforcement** — it is a valid CUI category that produces the banner marking `CUI//LEI`. "Law Enforcement" is the index group name. The NARA TSV is unambiguous:

```
General Law Enforcement		CUI	LEI	Law Enforcement
```

The intended warning (do not use an index group name as a category label) is a legitimate point, but the example chosen to illustrate it is wrong: `CUI//LEI` is a fully valid and correct banner marking.

The anti-pattern example will mislead operators into thinking `CUI//LEI` is incorrect — when a document containing general law enforcement information should in fact be marked `CUI//LEI`. An operator who internalizes this rule and sees `CUI//LEI` will incorrectly flag it as malformed.

**Recommended correction:** Replace with an example that uses an actual index group name in a banner:

```
[ANTI-PATTERN] `CUI//LAW ENFORCEMENT/INV` or `CUI//CRITICAL INFRASTRUCTURE` —
index group names (Law Enforcement, Critical Infrastructure, Defense, etc.)
never appear in banners. Use the category abbreviation instead.
For general law enforcement information: `CUI//LEI`.
For an investigation: `CUI//INV`.
```

**Remediation owner:** tech-writer

---

### Concerns

---

**C-1: Dual-status "Why it varies" for Export Control is misleading**

Location: lines 91–92, Dual-Status Categories table

The table states:

| Export Control | `CUI//EXPT` | `CUI//SP-EXPT` | ITAR vs EAR |

This implies that ITAR-controlled items use Basic (`CUI//EXPT`) and EAR-controlled items use Specified (`CUI//SP-EXPT`), or vice versa. This is incorrect. Both ITAR and EAR can produce the Specified designation depending on agency policy. The actual driver is whether the specific law or regulation mandates controls that *differ from or exceed* the NARA baseline — which can occur under either regime.

The "ITAR vs EAR" shorthand may be useful as a heuristic, but in the rules file it is stated as if the regime determines the designation, which it does not. An operator who internalizes this will be unable to correctly determine the designation for an EAR-controlled item that also has specified handling requirements.

**Recommendation:** Replace "ITAR vs EAR" with "Depends on whether the specific authority mandates controls beyond the NARA baseline" or similar factual language, with a clarifying note that ITAR-controlled items are commonly Specified but EAR items can be either.

**Remediation owner:** tech-writer

---

**C-2: "DL ONLY" described with incorrect name and characterized as "rare" without citation**

Location: lines 198–199, Big 6 table

The rules file gives the name as "Distribution list only." The NARA LDC registry names this control "Dissemination list controlled" with the banner marking `DL ONLY`. The word "distribution" does not appear in the NARA name for this control; "dissemination" is the NARA term throughout the LDC registry.

The parenthetical "(rare)" also appears in the table without a NARA citation. The NARA LDC description notes that DL ONLY "supersedes other limited dissemination controls" — a property suggesting it is powerful, not merely rare. Characterizing it as rare without a source may cause operators to underuse it or dismiss it when it is the appropriate control.

**Recommendation:** (a) Change "Distribution list only" to "Dissemination list controlled" to match NARA's official name. (b) Remove "(rare)" or replace with accurate guidance about when DL ONLY is appropriate vs. other controls.

**Remediation owner:** tech-writer

---

**C-3: RELIDO description omits "and Release" from the official title**

Location: lines 223–224, Additional LDCs table

The rules file describes RELIDO as: "Originator has authorized a Senior Foreign Disclosure Authority to make further sharing decisions." The NARA LDC registry names the authorizing official a "Senior Foreign Disclosure **and Release** Authority (SFDRA)." The rules file drops "and Release" from both the description and the acronym. This is a minor fidelity issue but the SFDRA acronym is used in NARA guidance and could cause confusion if operators look up the official term.

**Recommendation:** Update the description to read "Senior Foreign Disclosure and Release Authority (SFDRA)" to match NARA's official terminology.

**Remediation owner:** tech-writer

---

**C-4: JSON field mapping table omits three fields present in the actual catalog**

Location: lines 296–305, UMRS JSON Field Mapping section

The field mapping table documents six fields: key, `name`, `abbrv_name`, `index_group`, `designation`, `level`. The actual JSON catalog contains three additional fields in every marking entry:

- `handling_group_id` — links to handling requirements in the companion `US-CUI-HANDLING.json`
- `required_warning_statement` — mandatory statutory warning text, or `null`
- `required_dissemination_control` — required LDC or Distribution Statement, or `null`

These omitted fields are operationally significant. `required_warning_statement` is load-bearing for correct document marking — any code that generates document headers from this catalog needs to know this field exists. An implementer reading the rules file's field table would not know to read these fields.

**Recommendation:** Add the three missing fields to the field mapping table with their types, purpose, and null semantics documented.

**Remediation owner:** tech-writer

---

**C-5: FED ONLY + FEDCON mutual exclusivity asserted without NARA authority**

Location: lines 215–217, Mutually Exclusive LDCs section

The rules file states:

```
[RULE] A file cannot carry logically contradictory LDCs. The system must flag or
block these combinations:
- NOFORN + REL TO
- FED ONLY + FEDCON
```

NOFORN + REL TO is a logical contradiction well-supported by the text of both controls. FED ONLY + FEDCON is also logically contradictory (you cannot simultaneously restrict to federal employees only and also allow contractors). However, neither the NARA LDC registry document nor `nara-limited-dissemination-controls.md` explicitly states these as prohibited combinations. The rules file presents these as [RULE]-level requirements without citing a NARA or policy authority.

This is a defensible UMRS policy decision — the mutual exclusivity is logically evident — but the [RULE] prefix in this project requires authoritative backing. The NARA source confirms the definitions; the logical incompatibility can be derived from those definitions and stated as such.

**Recommendation:** Add a note that the mutual exclusivity is derived from the definitions of the LDCs themselves (citing the NARA LDC registry definitions), not a separate NARA prohibition. Alternatively, cite 32 CFR Part 2002 if that document addresses conflicting LDCs.

**Remediation owner:** tech-writer

---

### Accurate Findings (Selected)

The following sections were cross-referenced against NARA sources and are correct:

**A-1: Banner syntax rules** — The anatomy table, slash rules, double-slash placement, and the `CUI//SP-CTI//SP-EXPT` anti-pattern are all accurate per NARA guidance.

**A-2: Index group table — all 18 counts verified** — All category counts were verified against the NARA TSV. Every group matches: Critical Infrastructure (11), Defense (5), Export Control (2), Financial (12), Immigration (7), Intelligence (8), International Agreements (1), Law Enforcement (18), Legal (12), Natural and Cultural Resources (3), Nuclear (5), Patent (3), Privacy (9), Procurement and Acquisition (3), Proprietary Business Information (6), Statistical (4), Tax (4), Transportation (2).

**A-3: LDC coverage** — All 10 official NARA LDCs are present in the rules file (6 in the Big 6 table, 4 in the Additional LDCs section). No LDC is missing.

**A-4: Distribution Statement placement rule** — The rule that Distribution Statements do not appear in the CUI banner is correct per DoD convention and consistent with the NARA category detail scrape, which shows only CTI has a Distribution Statement requirement and that it is a separate document marking mechanism.

**A-5: Access control model** — The three-gate model (Lawful Government Purpose, Category Membership, LDC Attributes) is accurate and the axiom that CUI Basic is not an access level is correct per NARA program documentation.

**A-6: Attorney-Client and Attorney-WP restriction** — The rule that these LDCs may only be used with the PRIVILEGE category is accurate and confirmed by both the NARA LDC registry and the category detail scrape.

**A-7: NATO and Provisional exclusions** — Correctly documents that NATO categories have no finalized abbreviations and Provisional categories are excluded. The TSV confirms both.

**A-8: Abbreviation format rules** — Uppercase A-Z, under 15 characters, SP- prefix for specified, are consistent with the NARA registry data.

**A-9: REL TO and DISPLAY ONLY parameterization** — USA first, trigraph country codes alphabetical, tetragraph org codes alphabetical matches the NARA LDC registry exactly.

---

### Gap Analysis Summary — Rules File

**File reviewed:** 1 (`.claude/rules/cui-taxonomy-and-rules.md`)
**Total findings:** 6 (1 ERROR, 5 CONCERN)

**Errors:**
- E-1: LEI anti-pattern example is wrong — LEI is a valid category abbreviation (General Law Enforcement), not an index group. The example instructs operators to avoid a correct marking.

**Concerns:**
- C-1: Dual-status Export Control "why it varies" column uses "ITAR vs EAR" — both regimes can produce either designation; the description is misleading
- C-2: "DL ONLY" described with wrong NARA name ("Distribution" vs. "Dissemination") and unsupported "rare" characterization
- C-3: RELIDO description drops "and Release" from the official "Senior Foreign Disclosure and Release Authority (SFDRA)" title
- C-4: JSON field mapping table omits three fields (`handling_group_id`, `required_warning_statement`, `required_dissemination_control`) that are present in every catalog entry
- C-5: FED ONLY + FEDCON mutual exclusivity asserted as [RULE] without explicit NARA authority; should be stated as derived from definition logic

**Inconsistencies (rules file vs. NARA source data):**
- E-1: `CUI//LEI/INV` anti-pattern — rules file says LEI is an index group; NARA TSV shows LEI is the category abbreviation for General Law Enforcement
- C-2: "Distribution list only" vs. NARA's official name "Dissemination list controlled"
- C-3: "Senior Foreign Disclosure Authority" vs. NARA's "Senior Foreign Disclosure and Release Authority (SFDRA)"

**No conflicts found between the rules file and the JSON catalog** beyond those already captured as errors in the catalog audit above (particularly E-1 in the catalog audit regarding the MCS range).

---

*Section written by Knox (security-auditor). No source files were modified.*
