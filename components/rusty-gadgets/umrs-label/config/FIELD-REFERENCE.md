# UMRS Label Catalog — Field Reference

Documents every field in the `markings` and `dissemination_controls` sections
of the nation-specific label catalog JSON files.

**Source files:**
- `us/US-CUI-LABELS.json` — United States CUI catalog (143 markings, 10 dissemination controls)
- `ca/CANADIAN-PROTECTED.json` — Canadian Protected catalog (3 markings, no dissemination controls)

**Last updated:** 2026-03-30

---

## Markings Fields

### Shared Fields (present in all nation catalogs)

These fields form the common contract. Every marking entry in every catalog
must include them, even if the value is `null` or empty string.

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | string | Yes | Full human-readable category name in English. Example: `"Critical Energy Infrastructure Information"` |
| `abbrv_name` | string | Yes | Short uppercase abbreviation used in banner markings. Uppercase A-Z only, under 15 characters. `SP-` prefix for specified categories. Examples: `"CEII"`, `"SP-CTI"`, `"PA"` |
| `description` | string | Yes | Brief explanation of what information this category covers. May be empty string if not yet populated. |
| `index_group` | string or null | Yes | NARA organizational grouping for display and navigation. Never appears in banner markings. Set to `null` for nations without index groups (e.g., Canada). Examples: `"Critical Infrastructure"`, `"Law Enforcement"`, `null` |
| `designation` | string or null | Yes | Whether this category is CUI Basic or CUI Specified. Values: `"basic"`, `"specified"`, or `null` (for nations where the concept does not apply, e.g., Canada). |
| `level` | string | Yes | MLS sensitivity level identifier. Values: `"s1"` (US CUI, Canadian PA), `"s2"` (Canadian PB), `"s3"` (Canadian PC). In targeted policy, all US CUI operates at `s0`; the `s1` value reflects the MLS canonical mapping. |
| `handling_group_id` | string or null | Yes | Links this marking to a handling requirements group defined in the companion `US-CUI-HANDLING.json`. Set to `null` when no handling group is defined. Examples: `"CUI-BASE"`, `"CTI-GROUP"`, `null` |
| `required_warning_statement` | string or null | Yes | Verbatim warning text that must appear on documents carrying this marking. Set to `null` for categories with no required warning. Example: `"WARNING: This record contains Chemical-Terrorism Vulnerability Information controlled by 6 CFR 27.400..."` |
| `required_dissemination_control` | string or null | Yes | Describes any mandatory dissemination control or distribution statement requirement. Set to `null` for categories with no required dissemination control. Example: `"Distribution Statements B through F per DoD Instruction 5230.24"` |

### Canada-Only Fields

These fields are present in the Canadian catalog but not in the US catalog.
The Rust `Marking` struct carries them as `Option<T>` — they deserialize as
`None` when absent from a JSON entry.

| Field | Type | Description |
|---|---|---|
| `name_fr` | string | Category name in Canadian French. Authoritative source: TBS French text. Example: `"Protégé A"` |
| `description_fr` | string | Category description in Canadian French. Example: `"Renseignements dont on pourrait vraisemblablement s'attendre à ce que la divulgation non autorisée cause un préjudice limité..."` |
| `category_base` | string | MCS category number assigned to this tier. Example: `"c300"` (Protected A). Used for SELinux MCS labeling. |
| `category_range_reserved` | string | MCS category range reserved for future departmental subcategories within this tier. Example: `"c303-c399"` |
| `handling` | object | Structured handling requirements with bilingual fields. Contains sub-fields: `personnel_screening`, `personnel_screening_fr`, `storage`, `storage_fr`, `transmission`, `transmission_fr`, `destruction`, `destruction_fr`, `need_to_know` (boolean). US entries may use this field as a plain string or leave it absent. |
| `injury_examples` | string | Example of the type of injury that would result from unauthorized disclosure at this tier. Example: `"Disclosure of an exact salary figure."` |
| `injury_examples_fr` | string | Injury examples in Canadian French. Example: `"La divulgation d'un salaire exact."` |
| `authority_section` | string | Reference to the specific section of the governing authority document. Example: `"J.2.4.2.3"` (TBS Appendix J section for Protected A). |
| `marking_banner_en` | string | Display banner text in English. Used when the banner format differs from the JSON key. Example: `"PROTECTED A"` |
| `marking_banner_fr` | string | Display banner text in Canadian French. Example: `"PROTEGE A"` (with accents: `"PROTÉGÉ A"`). |
| `us_cui_approximate_correspondence` | string | Advisory note describing approximate equivalence to US CUI categories. Not a formal mapping. Example: `"Some CUI Basic categories address similar injury levels, but the systems are structurally different."` |
| `dissemination_controls` | any or null | Nation-specific dissemination control information. Currently `null` for all Canadian entries — Canada does not have standardized dissemination controls in the TBS framework. |
| `phase_note` | string | UMRS implementation phase notes. Indicates when a marking requires a future phase to carry enforcement weight. Example: `"Protected C requires Phase 2 MLS enforcement to carry meaningful security value."` |

### Future Fields (anticipated but not yet present)

These fields are expected as additional Five Eyes catalogs (UK, AU, NZ) are added:

| Field | Anticipated use |
|---|---|
| `name_<cc>` | Country-specific name variants (e.g., `name_nz`, `name_au`) for nations with different terminology for equivalent concepts. |
| `marking_banner_<cc>` | Country-specific banner display text. |
| `palette_ref` | Link to color palette entry in `UMRS-PALETTE.json` for TUI rendering. Previously present in some US entries; will be re-added when the palette system is finalized. |
| `risk_domains` | Array of risk domain identifiers linking to `US-CUI-RISK-DOMAINS.json`. |

---

## Dissemination Controls Fields

The `dissemination_controls` section is a sibling of `markings` at the top level
of the JSON file. Currently only the US catalog has dissemination controls.
Canada may share some (e.g., NOFORN, REL TO) in Five Eyes contexts — pending
review with Henri.

Each dissemination control is keyed by its canonical identifier (e.g., `"NOFORN"`,
`"REL TO"`).

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | string | Yes | Full human-readable name of the dissemination control. Example: `"No Foreign Dissemination"` |
| `banner_marking` | string | Yes | Text that appears in the CUI banner for this control. Example: `"NOFORN"`. For parameterized controls, this is the base text before the parameter list: `"REL TO"`. |
| `portion_marking` | string | Yes | Abbreviated marking used in portion markings (paragraph-level markings within a document). May differ from the banner marking. Examples: `"NF"` (for NOFORN), `"AC"` (for Attorney-Client). |
| `description` | string | Yes | Explanation of what this dissemination control restricts and how. |
| `parameterized` | boolean | Yes | Whether this control requires a parameter list (country codes or organization codes). `true` for REL TO and DISPLAY ONLY; `false` for all others. |
| `parameter_format` | string | Conditional | Format specification for the parameter list. Required when `parameterized` is `true`. Describes ordering rules (USA first, then trigraph codes alphabetically, then tetragraph codes). Absent when `parameterized` is `false`. |
| `parameter_example` | string | Conditional | Example of the control with parameters filled in. Required when `parameterized` is `true`. Example: `"REL TO USA, AUS, CAN, GBR, NZL"`. Absent when `parameterized` is `false`. |
| `category_restriction` | string or null | Yes | If this control may only be used with a specific CUI category, the abbreviation of that category. Set to `null` for general-purpose controls. Example: `"PRIVILEGE"` (for Attorney-Client and Attorney-WP, which may only be used with the Legal Privilege category). |
| `mutually_exclusive_with` | array of strings | Yes | List of dissemination control keys that are logically contradictory with this one. The system must flag or block any marking that combines mutually exclusive controls. Relationships are symmetric — if A excludes B, B must also exclude A. Example: `["REL TO", "DISPLAY ONLY"]` (for NOFORN). Empty array `[]` if no exclusions. |

### Mutual Exclusivity Map

| Control | Excludes |
|---|---|
| NOFORN | REL TO, DISPLAY ONLY |
| REL TO | NOFORN |
| DISPLAY ONLY | NOFORN |
| FED ONLY | FEDCON |
| FEDCON | FED ONLY, NOCON |
| NOCON | FEDCON |
| DL ONLY | (none) |
| RELIDO | (none) |
| Attorney-Client | (none) |
| Attorney-WP | (none) |

---

## JSON Key Conventions

| Convention | Rule |
|---|---|
| Marking keys | The complete banner marking string, alphabetically sorted. Examples: `"CUI//CEII"`, `"CUI//SP-CTI"`, `"PROTECTED-A"`. |
| Dissemination control keys | The canonical control identifier. Examples: `"NOFORN"`, `"REL TO"`, `"Attorney-Client"`. |
| Null vs empty string | Use `null` when the concept does not apply (e.g., `designation` for Canada). Use `""` (empty string) when the concept applies but the value is not yet populated. |
| Top-level key | `"markings"` in all nation catalogs. This is the shared structural contract. |
