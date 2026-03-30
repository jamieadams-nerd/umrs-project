## MCS Labeling and CUI Catalog Rules

Applies when working with CUI label files, `setrans.conf`, MCS categories,
`umrs-labels`, or Five Eyes classification mappings.

### Architecture

UMRS uses three coordinated artifacts for CUI/MCS labeling:

1. **LEVELS.json** — shared sensitivity level definitions (`s0`–`s3`) across all Five Eyes nations.
2. **Nation-specific catalog JSON** (e.g., `US-CUI-LABELS.json`, `CANADIAN-PROTECTED.json`) —
   label definitions, markings, handling instructions, and MCS category assignments.
3. **setrans.conf** — SELinux MCS translation configuration that maps `s0:cN` tuples to
   human-readable labels (e.g., `s0:c0 = CUI`, `s0:c90 = CUI//LEI`).

### Axioms

[AXIOM] In targeted policy, all MCS labels exist at `s0`. The sensitivity level
in label JSON files (`"level": "s1"`) reflects the MLS canonical mapping for
future enforcement — it has no effect under targeted policy.

[AXIOM] A file's controlled status is determined by category membership, not
sensitivity level alone. A file at `s1` with no CUI category is unclassified.

[AXIOM] `setrans.conf` is a translation table only — it does not create or enforce
policy. It maps MCS tuples to human-readable strings for `chcat -L` and audit logs.

[AXIOM] Canadian Protected A/B/C tiers are mutually exclusive on the same
information asset. US CUI categories are additive (a file can be `CUI//LEI` and
`CUI//PRIVACY` simultaneously via multiple MCS categories).

### Constraints

[CONSTRAINT] JSON catalog files (`*.json` under `umrs-labels/data/`) are protected
files. Do not modify without explicit user instruction.

[CONSTRAINT] `setrans.conf` is a protected file. Do not modify without explicit
user instruction.

[CONSTRAINT] Nation-specific MCS category ranges must not overlap:
- US CUI categories: `c0`–`c249`
- US LDCs and Distribution Statements: `c250`–`c279`
- US reserved (NATO, Provisional, growth): `c280`–`c299`
- Canadian Protected: `c300`–`c399`
- UK/AU/NZ: ranges TBD (not yet assigned)

[CONSTRAINT] CUI palette colors must not use red or orange. In Five Eyes
classified systems, red signifies SECRET and orange signifies TOP SECRET.
Using these colors for unclassified CUI markings would create dangerous
visual confusion for operators who work across classification levels.

### Rules

[RULE] Every entry in `setrans.conf` must have a corresponding entry in the
appropriate nation-specific catalog JSON. The two must stay synchronized.

[RULE] `setrans.conf` uses the format `s0:cN = LABEL` (with spaces around `=`).
Category tuples use commas without spaces: `s0:c90,c91 = CUI//LEI/AIV`.

[RULE] CUI marking format follows NARA registry: `CUI//CATEGORY/SUBCATEGORY`.
Double-slash separates CUI from the first category. Single-slash separates
category from subcategory.

[RULE] The `_metadata` block in catalog JSON files must include `country_code`
and `mcs_category_range` fields.

### Patterns

[PATTERN] Category numbering uses decade grouping: `c0` = CUI umbrella,
`c1`–`c9` = Privacy, `c10`–`c19` = Procurement, `c20`–`c29` = Proprietary,
etc. Subcategories use base + offset within the decade.

[PATTERN] When adding a new CUI category, update three artifacts in order:
1. Add the label definition to the nation-specific catalog JSON.
2. Add the `setrans.conf` translation entry.
3. Update any palette or handling JSON files if applicable.
