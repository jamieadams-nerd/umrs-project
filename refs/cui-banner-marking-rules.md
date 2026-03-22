# CUI Banner Marking Rules — Reference Extraction

**Source:** ISOO "Introduction to CUI Marking" video
**URL:** https://youtu.be/UxpF21AsxZE
**Published by:** Information Security Oversight Office (ISOO)
**Transcript retrieved:** 2026-03-21
**Additional ISOO resources:** https://isoo.blogs.archives.gov/
**CUI Registry:** https://www.archives.gov/cui (authoritative source for category markings,
limited dissemination controls, coversheets, and marking handbooks)

---

## 1. Why Standardized CUI Marking

Before CUI, each agency used its own acronyms and labels for sensitive information.
Standardized CUI marking ensures:

- Adequate protection by all agencies.
- Timely sharing with authorized recipients.

---

## 2. CUI Basic vs. CUI Specified

| Attribute | CUI Basic | CUI Specified |
|---|---|---|
| Definition | CUI requiring protection, with no further handling specifics in law/regulation | CUI requiring protection AND specific handling/marking/sharing requirements |
| Category marking in banner | Recommended but NOT mandatory | MANDATORY — must appear with "SP-" prefix |
| Where to find which applies | CUI Registry | CUI Registry |

---

## 3. Mandatory Components of Every CUI Marking

Two components are mandatory on all CUI-marked materials:

1. **Designation Indicator** — identifies who originated the CUI.
2. **Banner Marking** — appears at the top of the document in BOLD CAPITALIZED centered text.

---

## 4. Designation Indicator

The designation indicator must identify the originator. Acceptable forms:

- Agency letterhead
- Signature block
- "Controlled by" line with agency name

Best practice: include the name and contact information of the specific Point of Contact.

---

## 5. Banner Marking — Format and Structure

Banners must be:

- **BOLD**
- **CAPITALIZED**
- **Centered** (when feasible)
- Placed at the **top** of the document

### 5.1 Three Components, Separated by Double Forward Slash

```
<CUI Control Marking> // <Category Marking(s)> // <Limited Dissemination Control(s)>
```

The double forward slash (`//`) separates each major component.
A single forward slash (`/`) separates multiple entries within a component.

### 5.2 CUI Control Marking (Required — leftmost element)

The control marking is always the first element. Two acceptable values:

- `CUI`
- `CONTROLLED`

### 5.3 Minimum Valid Banner for CUI Basic

```
CUI
```

or

```
CONTROLLED
```

These alone are sufficient when a document contains only CUI Basic and the agency does not
choose to include category markings.

### 5.4 Category Markings (Component 2)

- Introduced by `//` after the control marking.
- Multiple categories within the same component are separated by single `/`.
- Alphabetize all categories of the same type.

**CUI Basic example (two categories):**

```
CUI//EMGT/WATER
```

**CUI Specified — mandatory, must include "SP-" prefix:**

```
CUI//SP-PRVCY
```

**Mixed CUI Specified + CUI Basic — Specified always precedes Basic, both alphabetized
within their type:**

```
CUI//SP-HLTH/SP-PRVCY/DETH/STUDREC
```

Rule: All `SP-` entries come before all non-`SP-` entries. Within each group, alphabetize.

### 5.5 Limited Dissemination Control Markings (Component 3)

- Introduced by `//` after the category marking component (or after the control marking if
  no category markings are included).
- Must be the **last element** in the banner.
- Multiple limited dissemination controls are alphabetized and separated by single `/`.

**Examples:**

```
CUI//NOFORN
```
(Only releasable to US citizens)

```
CUI//SP-PRVCY//DL ONLY/NOFORN
```
(CUI Specified, Privacy; dissemination-list-only and no foreign nationals)

The full approved list of limited dissemination controls is on the CUI Registry.

### 5.6 DL ONLY — Special Requirement

When `DL ONLY` is used, the document must include a dissemination list — either on a
separate sheet or somewhere apparent on the document — naming every individual,
organization, or entity authorized to access the information.

---

## 6. Footer Marking

- An identical footer marking is **optional**.
- If used, it MUST be identical to the banner marking. No variation is permitted.

---

## 7. Multi-Page Documents

- The **same banner applies to the entire document**.
- If any page contains CUI Specified or a specific CUI Basic category, that category
  must appear in the banner for all pages — even pages that do not themselves contain
  that category.
- Determine banner content by examining the entire document, not page by page.

---

## 8. Administrative Markings (e.g., DRAFT)

- CUI banner markings and administrative markings must remain **separate**.
- An administrative marking (e.g., "DRAFT", "PROVISIONAL") must NOT be incorporated
  into the CUI banner marking.

**Acceptable:**

```
DRAFT

CUI//EMGT
```

**Not acceptable:**

```
DRAFT CUI//EMGT
```

---

## 9. Portion Marking

- Portion marking is **recommended but not mandatory**.
- When used, it **must be applied throughout the entire document** — not selectively.
- Portion marks appear in parentheses before each paragraph.

| Paragraph content | Portion mark |
|---|---|
| Uncontrolled Unclassified Information | `(U)` |
| CUI Basic (no category specified) | `(CUI)` |
| CUI with category and/or LDC | `(CUI//CTGRY)` or `(CUI//CTGRY//LDC)` |

Portion markings follow the same separator rules as banner markings.

---

## 10. Email Marking

Requirements:

1. **Banner marking** at the top of the email body — mandatory.
2. **Subject line** indicator — `[Contains CUI]` at the end of the subject line (recommended
   but not listed as mandatory in the transcript; check CUI Marking Handbook for current
   requirement status).

**Forwarding rule:** When forwarding a CUI email, all previous markings must be carried
forward, with the banner marking moved back to the top of the email.

**Attachments:** Best practice (not mandatory) is to include `[Contains CUI]` in the
attachment filename.

Portion marking of email body follows the same rules as documents.

---

## 11. Presentation (PowerPoint / Slides) Marking

Same two mandatory components apply:

1. Designation Indicator
2. Banner Marking

If slides contain only CUI Basic, only the CUI control marking is mandatory.
If slides contain CUI Specified, the category marking (with `SP-` prefix) is also mandatory.

---

## 12. Transmittal Documents (Fax Cover Sheets)

Requirements:

1. Message on the transmittal that CUI is enclosed or attached.
2. Clear, conspicuous statement: **"When enclosure is removed, this document becomes
   Uncontrolled Unclassified Information."**

---

## 13. Coversheets

Can substitute for CUI banner headings when space is limited (e.g., forms).

| Optional Form | Use when |
|---|---|
| OF 901 | Document contains only CUI Basic (no category disclosure) |
| OF 902 | Wish to convey specific CUI categories |
| OF 903 | Wish to detail special handling requirements |

Available from the CUI Registry.

---

## 14. Forms Containing CUI

If a form will be filled in with CUI, it must be marked with a CUI banner.
If insufficient space exists at the top of the form, use a coversheet instead.

---

## 15. Electronic Storage Media

Physical media (hard drives, USB drives, CDs) containing CUI must have a physical marking
on the outside:

- Minimum: `CONTROLLED` or `CUI` plus the designating agency name.
- When feasible, include the full category designation.

---

## 16. Audio, Video, and Photographs

- **Digital photographs:** watermark.
- **Physical photographs:** label the back or use a frame label.
- **Audio and video files:** insert a disclaimer at the beginning (before content starts)
  that states CUI is present, and identifies relevant categories or limited dissemination
  controls.

Reference: ISOO "CUI Audio, Photography, and Video Marking Brochure" on the CUI Registry.

---

## 17. Mailing CUI

- CUI can be mailed via interagency mail systems, USPS, or commercial delivery services.
- **The envelope must NOT indicate the presence of CUI.**
- Using tracking is best practice (not mandatory).

---

## 18. Decontrol Marking

CUI is decontrolled when it no longer requires protection under a law, regulation, or
government-wide policy. Upon decontrol:

- A marker or indicator must appear on the first page indicating the information is no
  longer CUI.
- Follow agency policy, which may involve striking through or removing old CUI markings on
  the first page, cover page, or first page of any attachment.
- Purpose: avoid expending protection resources on information that no longer requires them.

---

## 19. Banner Marking Syntax Summary

```
CUI Control  //  [Category Component]  //  [LDC Component]
^required        ^optional (Basic)         ^optional
                 ^mandatory (Specified)
```

Separator rules:

| Separator | Where used |
|---|---|
| `//` (double slash) | Between major components (control → categories; categories → LDCs) |
| `/` (single slash) | Between entries within the same component |

Category ordering rules:

| Priority | Type |
|---|---|
| First | CUI Specified categories (alphabetized, each prefixed `SP-`) |
| Second | CUI Basic categories (alphabetized, no prefix) |

LDC ordering: alphabetized, all within the same `//` component.

---

## 20. UMRS Cross-Reference and Conformance Notes

### 20.1 Current UMRS Format

Memory records UMRS uses `CUI//LEI/INV` format. Cross-referencing against ISOO rules:

| ISOO Rule | UMRS Current Practice | Status |
|---|---|---|
| `CUI` control marking as leftmost element | `CUI` as prefix in `CUI//LEI/INV` | CONFORMS |
| `//` separates control marking from category component | `CUI//LEI/INV` uses `//` after `CUI` | CONFORMS |
| Multiple categories in same component separated by `/` | `LEI/INV` uses single `/` | CONFORMS |
| CUI Specified categories must use `SP-` prefix | LEI and INV are not marked `SP-` — verify CUI Registry | VERIFY |
| LDC appears after second `//` | No LDC present in `CUI//LEI/INV` | N/A |

The `SP-` question is the most important: if LEI or INV are CUI Specified categories
(i.e., the CUI Registry entry specifies particular handling requirements for them), UMRS
must prefix them with `SP-` in all banner markings.

**Action required:** Check the CUI Registry entries for Law Enforcement Information (LEI)
and Investigations (INV) to confirm whether each is CUI Basic or CUI Specified.

### 20.2 Items UMRS Should Implement or Validate

1. **SP- prefix validation** — `umrs-mcs` or the CUI label validator must reject
   CUI Specified categories that appear without the `SP-` prefix. Conversely, it must
   reject non-Specified categories that use the `SP-` prefix.

2. **Category ordering enforcement** — The validator must enforce that all `SP-` entries
   precede all non-`SP-` entries, and that entries within each group are alphabetized.

3. **LDC ordering enforcement** — The validator must enforce alphabetical order within the
   LDC component.

4. **DL ONLY dissemination list link** — If a CUI label includes `DL ONLY`, the label
   metadata should carry a reference to the dissemination list. This is a document-level
   requirement that UMRS tooling cannot enforce alone, but the catalog entry for `DL ONLY`
   should note the requirement.

5. **Decontrol state** — When CUI is decontrolled (Phase 2 enforcement context), UMRS
   must be able to represent that state distinctly from "no label assigned." A decontrolled
   document has a label history; an unlabeled document does not.

6. **Multi-page banner rule** — If UMRS processes multi-page documents, the catalog-level
   label must reflect the union of all categories present in any page of the document, not
   the page-level label of the first page.

7. **Administrative marking separation** — Any UMRS tool that renders or validates CUI
   banners (e.g., in TUI display or report output) must treat document-status strings
   ("DRAFT", "PROVISIONAL") as separate metadata, never embedded in the CUI banner string.

---

## Source and Authoritative References

- **CUI Registry:** https://www.archives.gov/cui — canonical source for category short names,
  LDC codes, coversheet forms, and marking handbooks.
- **CUI Marking Handbook:** Available on the CUI Registry.
- **ISOO blog:** https://isoo.blogs.archives.gov/
- **ISOO audio/photo/video brochure:** Available on the CUI Registry.
- **32 CFR Part 2002:** The federal rule implementing the CUI program.
- **EO 13556 (2010):** Executive Order establishing the CUI program.
