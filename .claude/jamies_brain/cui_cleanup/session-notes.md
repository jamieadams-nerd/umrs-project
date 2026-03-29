# CUI Cleanup Session Notes

## Key Learnings

1. **CUI is a flat namespace** — no hierarchy. Categories are not subcategories of index groups.
2. **Organizational Index Grouping** is for search/display only (`parent_group` in our JSON). Has no effect on banner markings.
3. **Banner markings are category-direct** — `CUI//INV`, not `CUI//LEI/INV`.
4. **Multi-category is rare but valid** — `CUI//INV/DNA` means two co-equal categories, not parent/child.
5. **Basic vs Specified is not a category property** — it's a designation property. Same category (e.g., EXPT) can appear as `CUI//EXPT` (basic) or `CUI//SP-EXPT` (specified). Both get their own `c` number in setrans.conf.
6. **Always use the specific banner marking** — never bare `CUI`. Always `CUI//EXPT` etc.
7. **NARA "Category Marking" = our `abbrv`** field in JSON.
8. **`parent_group`** in JSON is really `index_group` — but field name stays as-is.

## JSON Update Decisions

- **One JSON entry per category**, not per basic/specified variant.
- **Add `specified_marking` field** — e.g., `"specified_marking": "SP-EXPT"` so search resolves both `EXPT` and `SP-EXPT` to the same entry.
- **LDCs as a peer section to `markings`** under the US section — not nested inside markings.
- **LDCs get their own `c` numbers** in setrans.conf (e.g., NOFORN, FEDCON each get a `c` number).
- **Distribution Statements (A–F)** — details TBD from Jamie.

## File Inventory

| # | File | Status | Content |
|---|---|---|---|
| 1 | `nara-cui-category-marking-list.tsv` | Done | Raw TSV of marking list table |
| 2 | `nara-cui-categories-by-index.md` | Done | Categories organized by index group |
| 3 | `nara-cui-merged-reference.md` | Done | Merge of 1 and 2 — full detail by index group |
| 4 | `nara-cui-category-details-ldc-dist.md` | Done | Per-category LDC/distribution requirements (124-page scrape) |
| 5 | TBD | **Next** | Merge of 3 and 4 |
| — | `nara-limited-dissemination-controls.md` | Done | All 10 LDCs with markings and descriptions |
| — | `session-notes.md` | This file | Decisions and context for JSON update |

## Scrape Findings (File 4 Summary)

- **1 category with Distribution Statement requirement:** CTI → Dist Stmt B–F (DoD Instruction 5230.24)
- **1 category with explicit LDCs:** Legal Privilege (PRIVILEGE) → AC and AWP (exclusive to this category)
- **7 categories with special dissemination notes** (not formal LDC/DST): Bank Secrecy, Internal Data, International Agreements, US Census, Tax Convention, NATO Restricted, NATO Unclassified
- **1 page 404:** Privileged Safety Information (PSI) — broken link on NARA's site

## Next Steps

1. ~~Scrape all 124 category detail pages for LDC/distribution requirements (file 4).~~ Done.
2. ~~Break after retrieval.~~ Now.
3. Merge file 3 + file 4 into file 5.
4. Update the JSON using all accumulated reference data.
5. Jamie will provide Distribution Statement A–F details separately.
6. Jamie will provide more info on LDC c-number assignments (NOFORN, FEDCON, etc.).
