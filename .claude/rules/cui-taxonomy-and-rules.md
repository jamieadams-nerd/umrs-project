## CUI Taxonomy and Banner Marking Rules

Applies when working with CUI label files, banner marking construction, category
abbreviations, or the UMRS JSON label catalogs.

---

### Core Terminology

- **CUI Category** — what the data is (e.g., CTI, PRVCY, EXPT). This is the marking.
- **Index Group** — an organizational grouping on the NARA website (e.g., "Critical Infrastructure",
  "Law Enforcement", "Defense"). Index groups exist for human navigation only.
- **Limited Dissemination Control (LDC)** — who can see it (e.g., NOFORN, FED ONLY).
- **Abbreviation** — the short uppercase code that appears in the banner (e.g., CEII, INV, CTI).
- **Designation** — whether a category is "basic" or "specified" (or dual-status).

[AXIOM] Index groups never appear in a banner marking. They are purely organizational.
Only category abbreviations appear in banners.

[AXIOM] A category abbreviation is the only identifier that matters for marking and
enforcement. If the NARA registry renames the long-form category name, the abbreviation
remains the stable key.

---

### Banner Syntax

A CUI banner has up to three parts, always in this order:

```
CUI // CATEGORIES // LDC
```

Anatomy:

| Part | Example | Required |
|---|---|---|
| Control marking | `CUI` | Always |
| Category separator | `//` | When categories follow |
| Categories | `SP-CTI/EXPT` | Depends on designation |
| LDC separator | `//` | When LDC follows |
| LDC | `NOFORN` | Only if dissemination-restricted |

Full example: `CUI//SP-CTI/EXPT//NOFORN`

#### Abbreviation Format

[RULE] Category abbreviations must be:
- Uppercase letters A-Z only
- Under 15 characters
- No digits, no special characters
- Exception: the `SP-` prefix for specified categories (the only non-letter characters allowed)

#### Slash Rules

[RULE] Double slash (`//`) is used in exactly two places:
1. Between `CUI` and the first category: `CUI//CEII`
2. Between the last category and an LDC: `CUI//CEII//NOFORN`

[RULE] Single slash (`/`) separates categories from each other: `CUI//INV/PRVCY`

[ANTI-PATTERN] `CUI//SP-CTI//SP-EXPT` — never use double slash between categories.
Correct: `CUI//SP-CTI/EXPT`

---

### CUI Basic vs Specified

#### CUI Basic

- Follows the baseline safeguards defined by NARA (32 CFR Part 2002, NIST SP 800-171).
- Category marking in the banner is optional (but mandatory in UMRS for granular MAC).
- Banner can be just `CUI` in the physical world, but UMRS always uses `CUI//ABBREVIATION`.

#### CUI Specified

- The governing law or regulation mandates specific controls that differ from or exceed
  the Basic baseline.
- Category marking in the banner is mandatory and must carry the `SP-` prefix.
- Example: `CUI//SP-CTI`

#### Dual-Status Categories

[RULE] Some categories can be Basic or Specified depending on which authority applies
to the specific data. The designation is per-document, not per-category.

Common dual-status categories:

| Category | Basic marking | Specified marking | Why it varies |
|---|---|---|---|
| Export Control | `CUI//EXPT` | `CUI//SP-EXPT` | ITAR vs EAR |
| Nuclear | `CUI//NUC` | `CUI//SP-NUC` | Statute-dependent |
| Privacy | `CUI//PRVCY` | `CUI//SP-PRVCY` | Health/student record laws |
| Financial | `CUI//FNC` | `CUI//SP-FNC` | Banking/tax law specifics |

[RULE] For dual-status categories, UMRS assigns separate MCS category numbers to the
Basic and Specified variants (e.g., `c10` for EXPT, `c11` for SP-EXPT). A user cleared
for SP-EXPT does not automatically get access to EXPT Basic — both require explicit
category membership.

#### Key Comparison

| Feature | CUI Basic | CUI Specified |
|---|---|---|
| Source of controls | Standardized NARA guidelines | Specific law or regulation |
| Banner marking | Category optional (mandatory in UMRS) | Category mandatory with SP- prefix |
| Safeguards | Baseline (NIST SP 800-171) | Specific or enhanced requirements |
| Hierarchy | Standard CUI | Not higher — different |

---

### Combining Categories

[RULE] When a document contains information from multiple CUI categories, all categories
must appear in the banner.

#### Ordering Rules

[RULE] Specified categories (SP-) come first, before any Basic categories.

[RULE] Within each group (Specified and Basic), categories are alphabetized left-to-right.

[RULE] Use a single slash (`/`) to separate categories from each other.

#### Examples

Single Basic category:
```
CUI//PRVCY
```

Single Specified category:
```
CUI//SP-CTI
```

Two Basic categories (alphabetized):
```
CUI//INV/PRVCY
```

Mixed Specified + Basic (Specified first, then Basic, each alphabetized):
```
CUI//SP-CTI/EXPT
```

Multiple Specified + multiple Basic:
```
CUI//SP-CTI/SP-EXPT/INV/PRVCY
```

With LDC:
```
CUI//SP-CTI/EXPT//NOFORN
```

#### Sum of Parts Rule

[RULE] The banner is cumulative across all portions of a document. If page 1 contains
INV data and page 5 contains NUC data, the banner on every page must include both:
`CUI//INV/NUC`.

#### Overlapping Categories

No categories are strictly mutually exclusive. If a document contains data from two
categories, both must be listed. However, some combinations are rare because they
overlap heavily:

- **SP-CTI and EXPT** — frequently co-occur; list both: `CUI//SP-CTI/EXPT`
- **PRVCY and HLTH** — HLTH is a specific type of privacy data; use the most specific
  category applicable

#### Common Mistakes

[ANTI-PATTERN] `CUI//LEI/INV` — LEI (Law Enforcement) is an index group, not a category.
If the data is about an investigation, the marking is `CUI//INV`.

[ANTI-PATTERN] `CUI//SP-CTI//SP-EXPT` — double slash between categories. Use single
slash: `CUI//SP-CTI/EXPT`.

[ANTI-PATTERN] Using an index group name as a category. Index groups (Critical
Infrastructure, Defense, Law Enforcement, etc.) never appear in banners.

---

### Limited Dissemination Controls (LDCs)

LDCs specify who can see the data. They appear at the end of the banner after a
double slash.

#### The Big 6

| LDC | Meaning |
|---|---|
| NOFORN | No foreign dissemination |
| FED ONLY | Federal employees only |
| FEDCON | Federal employees and contractors only |
| DL ONLY | Distribution list only |
| REL TO | Authorized for release to specific country/group |
| DISPLAY ONLY | For viewing only, no copying or distribution |

#### LDC Pairing

Most CUI is shared without an LDC, under the general "Lawful Government Purpose"
boundary. Some categories are frequently or strictly paired with specific LDCs:

- **EXPT** — almost always paired with NOFORN or REL TO
- **INTEL** — frequently paired with NOFORN
- **TAX** — often paired with FED ONLY

#### Mutually Exclusive LDCs

[RULE] A file cannot carry logically contradictory LDCs. The system must flag or
block these combinations:

- NOFORN + REL TO (cannot deny all foreigners and release to a specific country)
- FED ONLY + FEDCON (cannot restrict to federal only and also allow contractors)

#### Additional LDCs

| LDC | Banner Marking | Description |
|---|---|---|
| No contractors | NOCON | No dissemination to contractors; permits dissemination to state/local/tribal employees |
| Releasable by disclosure official | RELIDO | Originator has authorized a Senior Foreign Disclosure Authority to make further sharing decisions |
| Attorney-Client | Attorney-Client | Attorney-client privilege; may only be used with the PRIVILEGE category |
| Attorney Work Product | Attorney-WP | Attorney work product privilege; may only be used with the PRIVILEGE category |

[RULE] Attorney-Client and Attorney-WP LDCs may ONLY be used with the Legal
Privilege (PRIVILEGE) category. No other category may carry these LDCs.

[RULE] REL TO and DISPLAY ONLY require parameterized country/organization lists.
USA must always appear first, followed by trigraph country codes in alphabetical
order, followed by tetragraph codes for international organizations in alphabetical
order. Example: `CUI//EXPT//REL TO USA, AUS, CAN, GBR`

#### DoD Distribution Statements

[RULE] For DoD technical data (e.g., SP-CTI), the dissemination control is a
Distribution Statement (B through F), not a standard LDC. UMRS treats Distribution
Statements as LDC-equivalent tags in the MCS category set.

[RULE] Distribution Statements do NOT appear in the CUI banner. The banner shows
only the SP- category (e.g., `CUI//SP-CTI`). The Distribution Statement appears in
two locations on the document:
1. The CUI Designation Indicator Block (lower-right corner, first page) — letter only
   (e.g., "Distribution Statement: D")
2. Full verbatim text of the statement on the first page

[ANTI-PATTERN] `CUI//SP-CTI//DIST-D` — Distribution Statement letters are not
recognized CUI LDC abbreviations and must never appear in the banner.

---

### Access Control Model

Access to CUI is governed by three gates. All three must be satisfied:

1. **Lawful Government Purpose** — the person needs the information for official duties.
2. **Category Membership** — the person is authorized for the specific CUI category.
3. **LDC Attributes** — the person meets the dissemination control requirements
   (e.g., nationality for NOFORN, employment status for FED ONLY).

[AXIOM] CUI Basic is a handling standard, not an access level. There is no "Basic
clearance" that grants access to all Basic categories. An HR specialist authorized
for PRVCY has no business seeing PROCURE data, even though both are Basic. Each
category requires its own authorization.

[RULE] If a file carries multiple categories (e.g., `CUI//INV/SP-CTI`), the user
must be authorized for ALL categories to gain access. Missing any one category
results in denial.

---

### UMRS JSON Field Mapping

This section defines how CUI program concepts map to fields in the UMRS label
catalog JSON files (e.g., `US-CUI-LABELS.json`).

#### Top-Level Structure

```json
{
  "_metadata": { ... },
  "markings": {
    "CUI//CEII": { ... },
    "CUI//SP-CTI": { ... }
  }
}
```

[RULE] The top-level key for label entries is `"markings"` in all nation catalogs
(US, CA, UK, AU, NZ). This is the shared structural contract.

#### Marking Entry Fields

| JSON field | CUI concept | Example |
|---|---|---|
| key (object name) | Full banner marking | `"CUI//CEII"` |
| `name` | Full category name | `"Critical Energy Infrastructure Information"` |
| `abbrv_name` | Banner abbreviation | `"CEII"` |
| `index_group` | NARA organizational group (display only) | `"Critical Infrastructure"` |
| `designation` | Basic or Specified | `"basic"`, `"specified"`, or `null` |
| `level` | MLS sensitivity level | `"s1"` |

[RULE] The JSON object key is the complete banner marking string. It encodes the
marking relationship directly — no separate parent field is needed.

[RULE] `index_group` is a display and navigation field only. It is nullable. Nations
without index groups (e.g., Canada) set it to `null`.

[RULE] `abbrv_name` is the short code that appears in the banner. It must match
the abbreviation format rules (uppercase A-Z, under 15 characters, SP- prefix for
specified).

[CONSTRAINT] JSON catalog files are protected files. Do not modify without explicit
user instruction.

---

### NARA Index Groups

There are 18 active index groups in the UMRS catalog. Index groups are purely
organizational — they never appear in banner markings.

| # | Index Group | Category count |
|---|---|---|
| 1 | Critical Infrastructure | 11 |
| 2 | Defense | 5 |
| 3 | Export Control | 2 |
| 4 | Financial | 12 |
| 5 | Immigration | 7 |
| 6 | Intelligence | 8 |
| 7 | International Agreements | 1 |
| 8 | Law Enforcement | 18 |
| 9 | Legal | 12 |
| 10 | Natural and Cultural Resources | 3 |
| 11 | Nuclear | 5 |
| 12 | Patent | 3 |
| 13 | Privacy | 9 |
| 14 | Procurement and Acquisition | 3 |
| 15 | Proprietary Business Information | 6 |
| 16 | Statistical | 4 |
| 17 | Tax | 4 |
| 18 | Transportation | 2 |

#### Excluded Groups

**NATO** — "NATO Restricted" and "NATO Unclassified" have no finalized category
abbreviations in the NARA registry as of 2026-03. Excluded until markings are assigned.

**Provisional** — Nine categories (e.g., "Homeland Security Agreement Information",
"Sensitive Personally Identifiable Information") have no finalized abbreviations.
Excluded until markings are assigned.

**Safety** — Some sources list a 20th index group called "Safety" containing
"Accident Investigation." In the NARA registry, AIV (Accident Investigation) is
filed under Law Enforcement. UMRS follows the NARA registry as the authoritative
source.
