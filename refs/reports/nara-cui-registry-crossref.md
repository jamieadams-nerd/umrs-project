# NARA CUI Registry Cross-Reference Report

**Date:** 2026-03-21
**Researcher:** Librarian agent
**Subject:** Cross-reference of `umrs-cui/cui-labels.json` entries against the NARA canonical CUI Registry

---

## Scope

This report addresses two questions:

1. **Standalone-to-group placement** — Six entries in our mapping have no parent group (standalone at the CUI level) despite DoD documentation placing them under parent groups. Does NARA agree with DoD's grouping?

2. **OURS_ONLY entries** — Eighteen entries exist in our mapping but not in the DoD registry. Does NARA list them, and if so, under what group and with what abbreviation?

**Primary sources used:**

- NARA CUI Registry: `https://www.archives.gov/cui/registry/category-list` (fetched 2026-03-21)
- NARA individual category detail pages: `https://www.archives.gov/cui/registry/category-detail/<slug>`
- DoD CUI categories and abbreviations: `https://www.dodcui.mil/CUI-Categories-and-Abbreviations/`
- Project reference: `components/platforms/rhel10/mls/notes/cui-category-abbreviations.txt` (DoD canonical list with NARA groupings, already in repo)

**Note on the project abbreviations file:** `cui-category-abbreviations.txt` is already the authoritative cross-reference. It records the official NARA organizational index groupings (as surfaced through the DoD CUI registry, which mirrors NARA). This report uses that file as the ground truth and verifies its claims against NARA detail pages where search results confirmed individual entries.

---

## Question 1: Standalone → Grouped Placement

These six entries exist in our JSON as `parent_group: "CUI"` (standalone, no parent group), but they have documented parent groups in the official registry.

### 1.1 CTI — Controlled Technical Information

| Field | Our Mapping | DoD/NARA |
|---|---|---|
| Key | `CUI//CTI` | — |
| NARA abbreviation | CTI | CTI |
| NARA group | None (standalone in our JSON) | **Defense** |
| NARA detail page | `category-detail/controlled-technical-info.html` | Confirmed |

**Finding:** Our JSON places CTI as `parent_group: "CUI"` (standalone). NARA and DoD both place CTI under the **Defense** organizational index group. DoD and NARA agree. Our mapping diverges.

**Correction needed:** The JSON entry `CUI//CTI` should be restructured to reflect that CTI is a Defense-group category. In NARA's marking model it would appear as `CUI//SP-CTI` (Specified), not as a standalone group.

---

### 1.2 NNPI — Naval Nuclear Propulsion Information

| Field | Our Mapping | DoD/NARA |
|---|---|---|
| Key | `CUI//NNPI` | — |
| NARA abbreviation | NNPI | NNPI |
| NARA group | None (standalone in our JSON) | **Defense** |
| NARA detail page | `category-detail/naval-nuclear-propulsion-info` | Confirmed |

**Finding:** Our JSON places NNPI as `parent_group: "CUI"` (standalone). NARA and DoD both place NNPI under the **Defense** organizational index group. The task description stated "DoD says Defense group" — this is confirmed. Our mapping diverges.

**Correction needed:** NNPI should be grouped under Defense in our model.

---

### 1.3 OPSEC — Operations Security

| Field | Our Mapping | DoD/NARA |
|---|---|---|
| Key | `CUI//OPSEC` | — |
| NARA abbreviation | OPSEC | OPSEC |
| NARA group | None (standalone in our JSON) | **Intelligence** |
| NARA detail page | `category-detail/opsec` | Confirmed |

**Finding:** Our JSON places OPSEC as `parent_group: "CUI"` (standalone). The name field even says "Operations Security (Intelligence)" — acknowledging the group — but the `parent_group` field is not set. DoD and NARA agree it belongs under **Intelligence**. Our `abbrv_name` is correct; only `parent_group` is wrong.

**Correction needed:** Set `parent_group` to the Intelligence group identifier in our model.

---

### 1.4 PROT — Temporary Protected Status

| Field | Our Mapping | DoD/NARA |
|---|---|---|
| Key | `CUI//PROT` | — |
| NARA abbreviation | PROT | PROT |
| NARA group | None (standalone in our JSON) | **Immigration** |
| NARA detail page | `category-detail/temporary-protected-status` | Confirmed |

**Finding:** Our JSON places PROT as `parent_group: "CUI"` (standalone). The name field says "Temporary Protected Status (Immigration)" — acknowledging the group — but `parent_group` is not set. DoD and NARA agree it belongs under **Immigration**. Our `abbrv_name` is correct.

**Correction needed:** Set `parent_group` to the Immigration group identifier in our model.

---

### 1.5 PSEC — Secrecy Orders

| Field | Our Mapping | DoD/NARA |
|---|---|---|
| Key | `CUI//PSEC` | — |
| NARA abbreviation | PSEC | PSEC |
| NARA group | None (standalone in our JSON) | **Patent** |
| NARA detail page | `category-detail/secrecy-orders` | Confirmed |

**Finding:** Our JSON places PSEC as `parent_group: "CUI"` (standalone). The name field says "Secrecy Orders (Patent)" — acknowledging the group — but `parent_group` is not set. DoD and NARA agree it belongs under the **Patent** organizational index group, alongside Patent Applications (APP) and Inventions (INVENT).

**Correction needed:** Set `parent_group` to the Patent group identifier in our model.

---

### 1.6 RAIL — Railroad Safety Analysis Records

| Field | Our Mapping | DoD/NARA |
|---|---|---|
| Key | `CUI//RAIL` | — |
| NARA abbreviation | RAIL | RAIL |
| NARA group | None (standalone in our JSON) | **Transportation** |
| NARA detail page | `category-detail/railroad-safety-analysis-records` | Confirmed |

**Finding:** Our JSON places RAIL as `parent_group: "CUI"` (standalone). DoD and NARA agree it belongs under the **Transportation** organizational index group, alongside SSI (Sensitive Security Information).

**Correction needed:** Set `parent_group` to the Transportation group identifier in our model. Note: the Transportation group in NARA contains only two categories — RAIL and SSI. It is not a large group.

---

### Summary: Question 1

All six cases have the same verdict: **DoD and NARA agree with each other, and our mapping diverges from both.** The `parent_group` field is either missing or set to `"CUI"` when it should reference the specific group.

| Abbreviation | Name | Our `parent_group` | Correct NARA group | DoD agrees? |
|---|---|---|---|---|
| CTI | Controlled Technical Information | `CUI` (standalone) | Defense | Yes |
| NNPI | Naval Nuclear Propulsion Information | `CUI` (standalone) | Defense | Yes |
| OPSEC | Operations Security | `CUI` (standalone) | Intelligence | Yes |
| PROT | Temporary Protected Status | `CUI` (standalone) | Immigration | Yes |
| PSEC | Secrecy Orders | `CUI` (standalone) | Patent | Yes |
| RAIL | Railroad Safety Analysis Records | `CUI` (standalone) | Transportation | Yes |

---

## Question 2: OURS_ONLY Entries — Do They Exist in NARA?

These 18 entries appear in our JSON but not in the DoD registry. The verdict for each:

### 2.1 AGR — Agriculture (parent group)

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//AGR` | — |
| Our abbreviation | `AGR` | **NARA uses `AG`** |
| NARA name | Agriculture | Agriculture |
| NARA group | Parent group in our model | Standalone NARA category under its own group |

**Finding:** NARA does list an "Agriculture" category — but the **official abbreviation is `AG`**, not `AGR`. This is confirmed by `cui-category-abbreviations.txt` (line: `AG – Agriculture (Intelligence)`). There is a second discrepancy: NARA places Agriculture under the **Intelligence** organizational index group, not as its own top-level group. Our JSON uses `AGR` as a parent group containing AMNT, CHEM, and PCI — this entire structure is non-canonical.

**Status:** The concept exists in NARA, but the abbreviation (`AG` not `AGR`), the group structure, and the subcategory hierarchy we invented are all wrong.

---

### 2.2 AMNT — Ammonium Nitrate

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//AGR/AMNT` | — |
| Our abbreviation | `AMNT` | **NARA uses `CRITAN`** |
| NARA name | Ammonium Nitrate | Ammonium Nitrate |
| NARA group | `AGR` (our invented parent) | **Critical Infrastructure** |

**Finding:** NARA does list Ammonium Nitrate — but the **official abbreviation is `CRITAN`**, not `AMNT`. NARA places it under the **Critical Infrastructure** organizational index group, not under Agriculture. Our entry has the wrong abbreviation, the wrong parent group, and the wrong group classification.

`cui-category-abbreviations.txt` line 25 confirms: `CRITAN – Ammonium Nitrate (Critical Infrastructure)`.

**Status:** Concept exists in NARA. Wrong abbreviation. Wrong parent group. Our model is fabricated.

---

### 2.3 CHEM — Agricultural Chemicals

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//AGR/CHEM` | — |
| Our abbreviation | `CHEM` | **No such entry in NARA** |
| NARA equivalent | — | Not found |

**Finding:** There is no CUI category with abbreviation `CHEM` or named "Agricultural Chemicals" in the NARA registry or in the DoD canonical list. NARA does list `TSCA – Toxic Substances (Critical Infrastructure)` and `PEST – Pesticide Producer Survey (Statistical)`, which are tangentially related but neither matches. This entry appears to be **invented** — not sourced from any official registry.

**Status:** Does NOT exist in NARA or DoD registry. Fabricated.

---

### 2.4 PCI — Pesticide Control Information

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//AGR/PCI` | — |
| Our abbreviation | `PCI` | **No such entry in NARA** |
| NARA equivalent | `PEST – Pesticide Producer Survey (Statistical)` | Different scope |

**Finding:** There is no CUI category with abbreviation `PCI` or named "Pesticide Control Information" in the NARA registry. The closest is `PEST – Pesticide Producer Survey`, which is a statistical category covering pesticide producer surveys, not pesticide control policy. This entry appears to be **invented**.

**Status:** Does NOT exist in NARA or DoD registry. Fabricated.

---

### 2.5 BSA — Bank Secrecy Act Information

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//FNC/BSA` | — |
| Our abbreviation | `BSA` | **NARA uses `FSEC`** |
| NARA name | Bank Secrecy | Bank Secrecy |
| NARA group | `FNC` (Financial, our invented parent) | **Financial** |

**Finding:** NARA does list Bank Secrecy information — but the **official abbreviation is `FSEC`**, not `BSA`. The name in NARA is "Bank Secrecy" (no "Act Information" suffix). Confirmed: `cui-category-abbreviations.txt` line 39: `FSEC – Bank Secrecy (Financial)`. The parent group (Financial) is correct in concept but our invented `FNC` parent does not match — in NARA's model, Financial is the organizational index group, and the category is just `FSEC` directly under it.

**Status:** Concept exists. Wrong abbreviation (`BSA` instead of `FSEC`). Parent group concept correct, but our structural model differs.

---

### 2.6 GOVT — Government (parent group)

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//GOVT` | — |
| Our abbreviation | `GOVT` | **No such group in NARA** |
| NARA equivalent | — | Not found as a group |

**Finding:** There is no "Government" organizational index group in the NARA CUI Registry. The registry has 20 groups, and "Government" as a top-level container does not appear. Our JSON invents `GOVT` as a parent for BUDG, PROC, RECS, LEGL, and LMI — but in NARA, these entries belong to different groups (Financial for BUDG, Procurement and Acquisition for PROCURE, Legal for LMI, etc.). The `GOVT` group is entirely our invention.

**Status:** Does NOT exist in NARA as a group. Fabricated hierarchy.

---

### 2.7 PROC — Government Procurement

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//GOVT/PROC` | — |
| Our abbreviation | `PROC` | **NARA uses `PROCURE`** |
| NARA name | General Procurement and Acquisition | General Procurement and Acquisition |
| NARA group | `GOVT` (our invented parent) | **Procurement and Acquisition** |

**Finding:** NARA lists General Procurement and Acquisition with abbreviation **`PROCURE`**, not `PROC`. It belongs to the **Procurement and Acquisition** organizational index group — not a `GOVT` parent. Confirmed: `cui-category-abbreviations.txt` line 84: `PROCURE – General Procurement and Acquisition (Procurement and Acquisition)`.

**Status:** Concept exists. Wrong abbreviation (`PROC` vs `PROCURE`). Wrong parent group (our `GOVT` vs correct standalone Procurement and Acquisition group).

---

### 2.8 RECS — Federal Records Information

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//GOVT/RECS` | — |
| Our abbreviation | `RECS` | **No such entry in NARA** |
| NARA equivalent | — | Not found |

**Finding:** There is no CUI category with abbreviation `RECS` or named "Federal Records Information" in the NARA registry or DoD canonical list. NARA is the records management authority itself; records information does not appear as a CUI category. This entry appears to be **invented**.

**Status:** Does NOT exist in NARA or DoD registry. Fabricated.

---

### 2.9 LEGL — Legal

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//GOVT/LEGL` | — |
| Our abbreviation | `LEGL` | **No such abbreviation in NARA** |
| NARA equivalent | `PRIVILEGE – Legal Privilege (Legal)` | Different scope |
| NARA group | `GOVT` (our invented parent) | **Legal** organizational index group |

**Finding:** There is no top-level "Legal" CUI category or `LEGL` abbreviation in the NARA registry. NARA has a **Legal** organizational index group containing numerous subcategories: ADPO (Administrative Proceedings), BARG (Collective Bargaining), CHLD (Child Pornography), JURY (Federal Grand Jury), PRIVILEGE (Legal Privilege), LMI (Legislative Materials), PRE (Presentence Report), PRIOR (Prior Arrest), LPROT (Protective Order), LVIC (Victim), WIT (Witness Protection). There is no single `LEGL` umbrella — it is a group, not a category.

**Status:** Does NOT exist in NARA as a category. The "Legal" group exists, but `LEGL` is not an abbreviation for anything. Fabricated.

---

### 2.10 PRIVACY — General Privacy (parent group)

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//PRIVACY` | — |
| Our abbreviation | `PRIVACY` | **NARA uses `PRVCY`** |
| NARA name | General Privacy | General Privacy |
| NARA group | Parent group in our model | Category under the **Privacy** organizational index group |

**Finding:** NARA lists General Privacy with abbreviation **`PRVCY`**, not `PRIVACY`. `cui-category-abbreviations.txt` line 87: `PRVCY – General Privacy (Privacy)`. Our JSON uses `PRIVACY` as a parent group (containing HEALTH, MIL, PERS, etc.) — and these subcategory relationships are correct in structure, but the abbreviation for the parent is wrong.

**Status:** Concept exists. Wrong abbreviation (`PRIVACY` vs `PRVCY`). The child groupings (MIL, PERS, GENETIC, STUD, CONTRACT, DREC, PRIIG) are structurally correct per NARA.

---

### 2.11 HEALTH — Health (subcategory under PRIVACY)

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//PRIVACY/HEALTH` | — |
| Our abbreviation | `HEALTH` | **NARA uses `HLTH`** |
| NARA name | Health Information | Health Information |
| NARA group | `PRIVACY` (our parent) | **Privacy** organizational index group |

**Finding:** NARA lists Health Information with abbreviation **`HLTH`**, not `HEALTH`. Banner marking is `CUI//SP-HLTH`. `cui-category-abbreviations.txt` line 45: `HLTH – Health Information (Privacy)`. The parent group (Privacy) is correct in concept.

**Status:** Concept exists. Wrong abbreviation (`HEALTH` vs `HLTH`).

---

### 2.12 TRANSPORT — Transportation (parent group)

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//TRANSPORT` | — |
| Our abbreviation | `TRANSPORT` | **No such group abbreviation in NARA** |
| NARA equivalent | Transportation is an organizational index group | It is a group, not a marked category |

**Finding:** The NARA Transportation organizational index group contains only two categories: RAIL (Railroad Safety Analysis Records) and SSI (Sensitive Security Information). There is no parent-level Transportation category with its own abbreviation/marking in NARA. Our JSON creates a `TRANSPORT` entry as if it were a markable category — it is not.

**Status:** "Transportation" exists as a NARA group name, but `TRANSPORT` is not a marking abbreviation. Fabricated as a markable category.

---

### 2.13 AVIATION — Aviation Safety

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//AVIATION` | — |
| Our abbreviation | `AVIATION` | **No such entry in NARA** |
| NARA equivalent | — | Not found |

**Finding:** There is no CUI category named "Aviation Safety" or with abbreviation `AVIATION` in the NARA registry or DoD canonical list. Aviation security concerns are generally covered by SSI (Sensitive Security Information) under Transportation. This entry appears to be **invented**.

**Status:** Does NOT exist in NARA or DoD registry. Fabricated.

---

### 2.14 MARITIME — Maritime Security

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//MARITIME` | — |
| Our abbreviation | `MARITIME` | **No such entry in NARA** |
| NARA equivalent | — | Likely covered by SSI |

**Finding:** There is no CUI category named "Maritime Security" or with abbreviation `MARITIME` in the NARA registry. Maritime security information is covered by SSI (Sensitive Security Information) under the Transportation group. This entry is **invented**.

**Status:** Does NOT exist in NARA or DoD registry. Fabricated.

---

### 2.15 PIPELINE — Pipeline Security

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//PIPELINE` | — |
| Our abbreviation | `PIPELINE` | **No such entry in NARA** |
| NARA equivalent | `CEII` (Critical Energy Infrastructure Information) | Partial overlap |

**Finding:** There is no CUI category named "Pipeline Security" or with abbreviation `PIPELINE` in the NARA registry. Pipeline security concerns are covered by `CEII – Critical Energy Infrastructure Information (Critical Infrastructure)`. This entry is **invented**.

**Status:** Does NOT exist in NARA or DoD registry. Fabricated.

---

### 2.16 EMERGENCY — Emergency Services

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//EMERGENCY` | — |
| Our abbreviation | `EMERGENCY` | **NARA uses `EMGT`** |
| NARA name | Emergency Management | Emergency Management |
| NARA group | Standalone (our model) | **Critical Infrastructure** |

**Finding:** NARA lists Emergency Management with abbreviation **`EMGT`**, not `EMERGENCY`. And critically, in NARA it belongs to the **Critical Infrastructure** organizational index group, not as a standalone category. Our JSON also places it as `parent_group: "CUI"` (standalone), whereas our other `CRIT` subcategories (like `EMGT` in the `cui-labels.json`) already correctly use `parent_group: "CRIT"`. We have both a correct `CUI//CRIT/EMGT` entry AND an incorrect `CUI//EMERGENCY` entry — this is a duplicate with wrong abbreviation.

**Status:** Concept exists. Wrong abbreviation (`EMERGENCY` vs `EMGT`). Duplicate of the already-correct `CUI//CRIT/EMGT` entry.

---

### 2.17 EXPORT — Export Control (parent group)

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//EXPORT` | — |
| Our abbreviation | `EXPORT` | **NARA uses `EXPT`** |
| NARA name | Export Controlled | Export Controlled |
| NARA group | Parent group in our model | Category under **Export Control** organizational index group |

**Finding:** NARA lists Export Controlled with abbreviation **`EXPT`** (Specified: `CUI//SP-EXPT`), not `EXPORT`. `cui-category-abbreviations.txt` line 33: `EXPT – Export Controlled (Export Control)`. Our JSON uses `EXPORT` as a standalone parent group — but we also have a correct `CUI//CTI/EXPT` entry, placing Export Controlled under CTI. This is doubly wrong: wrong abbreviation and wrong structural position.

**Status:** Concept exists. Wrong abbreviation (`EXPORT` vs `EXPT`). The Export Control organizational index group exists in NARA, but `EXPORT` is not a valid abbreviation.

---

### 2.18 FEDCON — Federal Contract Information

| Field | Our Mapping | NARA |
|---|---|---|
| Key | `CUI//FEDCON` | — |
| Our abbreviation | `FEDCON` | **`FEDCON` is a Limited Dissemination Control, not a category** |
| NARA equivalent | — | Not a CUI category |

**Finding:** `FEDCON` in the NARA CUI system is a **Limited Dissemination Control (LDC)** marking, meaning "Federal Employees and Contractors Only." It is an access restriction modifier, not a content category. It appears in a banner marking as `CUI//FEDCON` to restrict who can see the document — but it does not classify what type of CUI the content is. Federal Contract Information (FCI) is a concept defined separately in FAR 52.204-21, not in the CUI Registry. There is no `FEDCON` CUI category.

**Status:** Critically misclassified. `FEDCON` is an LDC dissemination control, not a CUI category. Should be removed from the categories model and documented separately if needed.

---

### Summary: Question 2

| Our abbreviation | NARA status | NARA abbreviation | NARA group | Assessment |
|---|---|---|---|---|
| AGR (group) | Exists as category, not a group | `AG` | Intelligence | Wrong abbrev + wrong structure |
| AMNT | Exists | `CRITAN` | Critical Infrastructure | Wrong abbrev + wrong group |
| CHEM | Does NOT exist | — | — | Fabricated |
| PCI | Does NOT exist | — | — | Fabricated |
| BSA | Exists | `FSEC` | Financial | Wrong abbreviation |
| GOVT (group) | Does NOT exist as a group | — | — | Fabricated hierarchy |
| PROC | Exists | `PROCURE` | Procurement and Acquisition | Wrong abbrev + wrong parent |
| RECS | Does NOT exist | — | — | Fabricated |
| LEGL | Does NOT exist as a category | — | Legal (group name only) | Fabricated |
| PRIVACY (group) | Exists as category | `PRVCY` | Privacy | Wrong abbreviation |
| HEALTH | Exists | `HLTH` | Privacy | Wrong abbreviation |
| TRANSPORT (group) | Not a markable category | — | Transportation (group) | Fabricated as marking |
| AVIATION | Does NOT exist | — | — | Fabricated |
| MARITIME | Does NOT exist | — | — | Fabricated |
| PIPELINE | Does NOT exist | — | — | Fabricated |
| EMERGENCY | Exists | `EMGT` | Critical Infrastructure | Wrong abbrev + duplicate of `CUI//CRIT/EMGT` |
| EXPORT (group) | Exists as category | `EXPT` | Export Control | Wrong abbreviation + wrong structure |
| FEDCON | NOT a category — is an LDC | — | — | Fundamentally misclassified |

---

## Priority Remediation Recommendations

These findings are ordered from highest to lowest structural impact:

### P1 — Remove or reclassify FEDCON

`CUI//FEDCON` must be removed from the categories model. It is a dissemination control, not a category. If there is a use for it in the system, it should be represented in a separate dissemination controls structure. Leaving it in categories will produce incorrect markings.

### P2 — Replace fabricated abbreviations

| Wrong | Correct |
|---|---|
| `AMNT` | `CRITAN` |
| `BSA` | `FSEC` |
| `PROC` | `PROCURE` |
| `PRIVACY` (as abbreviation) | `PRVCY` |
| `HEALTH` | `HLTH` |
| `EMERGENCY` (standalone) | Already exists as `EMGT` under CRIT; remove duplicate |
| `EXPORT` (standalone) | `EXPT` under Export Control group |

### P3 — Fix parent_group for Question 1 entries

Set correct parent group for: CTI (Defense), NNPI (Defense), OPSEC (Intelligence), PROT (Immigration), PSEC (Patent), RAIL (Transportation).

### P4 — Remove fully fabricated entries

Remove from the canonical categories model: `CHEM`, `PCI`, `RECS`, `LEGL` (as a category), `TRANSPORT` (as a markable group), `AVIATION`, `MARITIME`, `PIPELINE`, `GOVT` (as a group).

If these concepts are needed for the UMRS model (e.g., aviation security handling), they must be documented as UMRS-local extensions and clearly distinguished from NARA-canonical categories. They must not be mixed into the same flat namespace.

### P5 — Restructure non-canonical group hierarchies

The invented groups `AGR`, `GOVT`, `PRIVACY` (used as parent keys), `CTI` (used as a parent for EXPT/EXPTR), and `TRANSPORT` do not match NARA's organizational index structure. The NARA structure is flat — categories belong directly to organizational index groups; there are no multi-level subcategory trees with parent keys of parent keys.

---

## Notes for Implementers

1. The **DoD CUI registry** at `dodcui.mil` mirrors NARA but applies DoD-specific overlays. For the UMRS model, NARA is the canonical authority; DoD overlays apply where UMRS handles specifically defense-context information.

2. The **canonical abbreviation list** is already in the repo at `components/platforms/rhel10/mls/notes/cui-category-abbreviations.txt`. Any future addition to `cui-labels.json` should be cross-checked against that file first.

3. NARA has 20 organizational index groups and approximately 125 categories. Our JSON currently has more "groups" than NARA because of the invented hierarchies. A correct model would have exactly the NARA groups as top-level organizational buckets, with categories underneath.

4. `CUI//SP-<abbreviation>` denotes CUI Specified (handling requirements specified by law/regulation). `CUI//<abbreviation>` (no SP prefix) denotes CUI Basic. Our JSON does not currently distinguish these — this may be a separate issue to address, but it is not in scope for this report.

---

## Manifest Entry

```
name: NARA CUI Registry Cross-Reference
path: refs/reports/nara-cui-registry-crossref.md
version: 1.0
source_url: https://www.archives.gov/cui/registry/category-list
date_retrieved: 2026-03-21
sha256: (compute after write)
relevance: umrs-cui, CUI marking, CMMC, AC-4
status: research_report
```
