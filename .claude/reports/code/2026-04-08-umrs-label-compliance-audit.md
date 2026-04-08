# Security and Compliance Audit — `umrs-label` Crate

Audit date: 2026-04-08
Depth: in-depth
Scope: All `.rs` files under `components/rusty-gadgets/umrs-label/src/`, all test files under `tests/`, and catalog JSON files at `config/us/US-CUI-LABELS.json` and `config/ca/CANADIAN-PROTECTED.json`.

---

## Summary Table

| Category | Count |
|---|---|
| ACCURATE | 28 |
| CONCERN | 6 |
| ERROR | 3 |

---

## ERROR Findings

### E-1 — CUI Phase 1 Language Violation: "MAC enforcement" in Operator-Visible JSON

**File:** `config/us/US-CUI-LABELS.json`
**Location:** Line 37, `markings["CUI"].description.en_US`
**Severity:** HIGH — The cui_phase1_language.md rule is an unconditional CONSTRAINT applying to all agents, all output.

The `description` field of the `CUI` umbrella entry contains the following operator-visible string:

> "In UMRS, specific category markings (CUI//ABBREVIATION) are always required for granular MAC enforcement."

The phrase "MAC enforcement" is a violation of the CUI Phase 1 Language Rule. Under targeted policy, UMRS applies and displays MCS labels — it does not enforce mandatory access control. The string is served through the TUI detail panel and will be seen by operators. The French translation on line 38 repeats the violation: "l'application granulaire du contrôle d'accès obligatoire."

**Recommended replacement:**

> "In UMRS, specific category markings (CUI//ABBREVIATION) are always required for granular MCS labeling and auditability. A file marked just CUI would be invisible to category-based label tracking."

(French: "Dans UMRS, des marquages de catégorie spécifiques (CUI//ABRÉVIATION) sont toujours requis pour l'étiquetage MCS granulaire et la traçabilité.")

**Remediation owner:** tech-writer (data file)

---

### E-2 — `CuiMarking` Regex Rejects Syntactically Valid Full-Banner Strings With LDC

**File:** `src/validate.rs`
**Location:** Line 54, `CuiPattern::CuiMarking` regex value
**Severity:** HIGH — `is_valid()` is a security-gating function. The doc comment on line 86–87 calls it "syntax-only", but a valid CUI full-banner string that includes an LDC (`CUI//SP-CTI//NOFORN`) is syntactically valid per the CUI banner rules and will return `false` from `is_valid()`. Any caller checking a complete banner string with an LDC suffix will receive a fail-closed rejection for a legitimately formed marking.

The regex is: `^CUI(//[A-Z][-A-Z]*)(/[A-Z][-A-Z]*)*$`

Under this pattern, `CUI//SP-CTI//NOFORN` fails because the second `//` is not in the pattern — only single `/` separators between additional segments are allowed. But the CUI banner spec uses a second `//` to introduce the LDC block.

**Two corrective options:**

1. Widen the pattern to handle the LDC suffix:
   ```
   ^CUI(//[A-Z][-A-Z]*)(/[A-Z][-A-Z]*)*(//[A-Z][-A-Z /]*)?$
   ```
   (The LDC group is optional and uses a second `//` prefix.)

2. Narrow the scope to "category-only" markings and document explicitly that full-banner strings with LDCs are out of scope. Add a second `CuiFullBanner` variant for complete validation.

The doc comment currently states "syntax-only check" but provides no warning that valid full-banner strings are rejected. The ambiguity is load-bearing: a caller who passes a complete operator-provided banner string will see silent rejection.

**Remediation owner:** coder

---

### E-3 — `handling_group_id: ""` Pattern: Empty String Where `null` Is Required

**File:** `config/us/US-CUI-LABELS.json`
**Location:** 139 entries use `"handling_group_id": ""` (empty string)
**Severity:** MEDIUM (escalated from CONCERN to ERROR because it affects a security-relevant field used by catalog consistency checks)

The `handling_group_id` field is defined as `Option<String>` in `catalog.rs`. Entries that have no handling group should use JSON `null` to produce `None` in Rust — the type's intended absent value. Empty string `""` deserializes to `Some("")`, which is a distinct value from `None`. The predicate `has_handling_group()` correctly handles empty strings (trims and checks), but catalog consumers relying on `handling_group_id.is_none()` to mean "no group defined" will silently receive the wrong answer for 139 of ~150 entries.

This is documented as a known pattern in the 2026-03-30 audit at `code/2026-04-01-umrs-c2pa-security-audit-review.md` but it now appears in the US CUI catalog which was not covered by that audit.

**Recommended fix:** Replace all `"handling_group_id": ""` with `"handling_group_id": null` in `US-CUI-LABELS.json`. This is a JSON-only change; the Rust code already handles `null` correctly as `None`.

**Remediation owner:** tech-writer (data file)

---

## CONCERN Findings

### C-1 — RELIDO Missing `NOFORN` Mutual-Exclusivity Entry

**File:** `config/us/US-CUI-LABELS.json`
**Location:** Line 2616–2630, `dissemination_controls["RELIDO"]`
**Recommendation:** The audit knowledge archive (from the 2026-03-30 CUI catalog audit) identifies that RELIDO is a permissive foreign disclosure marking that is logically mutually exclusive with NOFORN — you cannot authorize a senior foreign disclosure authority to make sharing decisions while simultaneously prohibiting all foreign dissemination. The `mutually_exclusive_with` field on the RELIDO entry is currently `[]`. Add `["NOFORN"]` to match the documented constraint.

Additionally, the `name` field reads "Releasable by Information Disclosure Official" — the authoritative NARA name uses "Disclosure and Release Authority" (SFDRA, Senior Foreign Disclosure **and Release** Authority). The name should be "Releasable by Disclosure and Release Official" or the description text should be updated to be internally consistent with the correct authority acronym SFDRA.

**Remediation owner:** tech-writer (data file)

---

### C-2 — `CUI//SP-PCII`, `CUI//SP-SGI`, `CUI//SP-TAX` Missing `required_warning_statement`

**File:** `config/us/US-CUI-LABELS.json`
**Location:** Lines 1931, 2033, 2152

From the known mandatory warning statement inventory (audit knowledge archive):

- **CVI (SP-CVI):** Present — 6 CFR 27.400. ✓
- **DCNI (SP-DCNI):** Present — 10 U.S.C. 128. ✓
- **EXPT (SP-EXPT):** Present — ITAR/AECA warning. ✓
- **SSI (SP-SSI):** Present — 49 CFR 15/1520. ✓
- **UCNI (SP-UCNI):** Present — 42 U.S.C. 2168. ✓
- **TAX (SP-TAX):** **MISSING** — 26 U.S.C. §§ 6103/7213 require a statutory warning.
- **PCII (SP-PCII):** **MISSING** — 6 CFR 29.8 requires a mandatory warning statement for PCII.
- **SGI (SP-SGI):** **MISSING** — 10 CFR 73.21 (safeguards information) requires a mandatory warning.

Three specified categories required by law to carry warning statements have `null` in that field.

**Recommendation:** Research and populate `required_warning_statement` for SP-TAX, SP-PCII, and SP-SGI from the governing statutes and regulations cited above.

**Remediation owner:** tech-writer (data file)

---

### C-3 — `setrans_tests.rs` AC-4 Citation: Correct Control, Missing AC-16

**File:** `tests/setrans_tests.rs`
**Location:** Module-level comment, lines 14–22

The module comment cites:
- NIST SP 800-53 AC-4 (Information Flow Enforcement) — ✓ correct, setrans entries control label-based information flow
- NIST SP 800-53 AC-16 (Security Attributes) — ✓ correct
- NIST SP 800-53 AU-3 (Audit Record Content) — ✓ correct

However, the test module is not a `//!` block — it is a `//` block comment. It will not appear in generated documentation. Test files are excluded from the Module Documentation Checklist Rule explicitly, so this is not a compliance violation, but the comment in the test file should acknowledge that AC-4 here refers to label-based MCS category flow tracking, not kernel-enforced flow control (Phase 1 targeted policy). The claim "MCS category assignments are the mechanism that controls information flow" is accurate for MLS (Phase 2) but overstated for Phase 1.

**Recommendation:** Add a parenthetical: "(Phase 2 MLS enforcement will realize this control; in Phase 1, labels are applied and visible but not kernel-enforced)".

**Remediation owner:** coder

---

### C-4 — US Catalog Uses `mcs_ranges` Object; `CatalogMetadata` Models `mcs_category_range` as `Option<String>`

**File:** `src/cui/catalog.rs` line 77 vs. `config/us/US-CUI-LABELS.json` lines 8–12

The US catalog stores MCS ranges as a structured JSON object under the key `mcs_ranges`:
```json
"mcs_ranges": {
  "categories": "c0-c249",
  "ldc_and_distribution": "c250-c279",
  "reserved": "c280-c299"
}
```

The `CatalogMetadata` struct has `mcs_category_range: Option<String>` which maps to the key `mcs_category_range`. Since the US catalog uses `mcs_ranges` (different name, object type), `metadata.mcs_category_range` will always be `None` for the US catalog. The `catalog_metadata_rows()` function in `app.rs` will silently skip the MCS range row for US catalogs.

The Canadian catalog correctly uses `"mcs_category_range": "c300-c399"` (string, correct field name).

This is a silent data gap: operators viewing the US catalog root in the TUI will see no MCS category range information. The `mcs_ranges` object is suppressed in `app.rs` line 561 (`if k == "scope" || k == "notes" || k == "mcs_ranges"`).

**Recommendation:** Either: (a) normalize US catalog to also have `"mcs_category_range": "c0-c299"` at the top level (a summarized string), or (b) add logic to `catalog_metadata_rows()` to extract and display the nested `mcs_ranges` object fields. Option (a) is simpler and keeps the two catalogs structurally consistent.

**Remediation owner:** coder + tech-writer (data file alignment)

---

### C-5 — `validate.rs` Regex Cache Uses `OnceLock<Mutex<HashMap>>` Instead of Per-Variant `OnceLock<T>`

**File:** `src/validate.rs`
**Location:** Lines 63–80

The `Performance-Aware Construction Patterns` rule specifies: "When caching compiled resources (regex, parsed configs, lookup tables) with a fixed set of keys known at compile time, declare one `static OnceLock<T>` per key variant rather than a shared `Mutex<HashMap<K, T>>`."

`CuiPattern` has a fixed set of variants enumerable at compile time (currently one: `CuiMarking`). The regex cache should be:

```rust
static CUI_MARKING_REGEX: OnceLock<Regex> = OnceLock::new();
```

with `get_or_init` directly on that static rather than acquiring a mutex on every call. The current `Mutex<HashMap>` design is correct for dynamically keyed caches, but the rules forbid it when keys are statically known. This is a pattern compliance gap, not a security defect.

**Recommendation:** Replace the `Mutex<HashMap>` with one `static OnceLock<Regex>` per `CuiPattern` variant. The comment in `validate.rs` line 10–11 even references the `OnceLock<Mutex<HashMap>>` design from `umrs-core::validate`, but that module presumably has a larger, more dynamic pattern set.

**Remediation owner:** coder

---

### C-6 — `palette.rs` Is a Stub Module With No Exported Types

**File:** `src/cui/palette.rs`
**Location:** Module-level doc block

The `palette.rs` module's `//!` block states it is "currently a placeholder for future palette definitions" with no exported types, constants, or functions. The module doc states palette functions "will be added here as the UMRS label display layer matures."

The Module Documentation Checklist Rule requires modules to list key exported types. A module with zero exports technically satisfies the rule with a note explaining the absence, which this module does. However, the `lib.rs` exposes `pub mod cui` which exposes `pub mod palette`, making this an empty public module in the crate's API. Clients importing `umrs_labels::cui::palette` will find nothing. This creates documentation noise and API confusion.

**Recommendation:** Either (a) add a `#[doc(hidden)]` to the module re-export until the module has content, or (b) add at minimum the palette reference key constants that the JSON catalog references (e.g., `"CUI-BASE"`, `"CTI-GROUP"`) as typed constants.

**Remediation owner:** coder

---

## ACCURATE Findings

The following items were reviewed and found correct.

**A-1** — `lib.rs` has a complete `//!` block with Purpose, Key Types, and Compliance section citing NIST SP 800-53 AC-16, AU-3, and CMMC AC.L2-3.1.3 in canonical form.

**A-2** — `main.rs` has a complete `//!` block with Purpose, Output Modes, Catalog File Paths, and Compliance section (AC-16, AU-3, AC-3). Citations use `NIST SP 800-53` form throughout.

**A-3** — `validate.rs` has a complete `//!` block with Purpose, Key Types, and Compliance (SI-10, AC-16, CMMC AC.L2-3.1.3). The `CuiPattern` type and `is_valid()` function both carry inline control citations. Correct citation form throughout.

**A-4** — `cui/mod.rs` has a complete `//!` block with Purpose, Sub-modules, and Compliance (AC-16, AU-3, SI-10). Correct citation form.

**A-5** — `cui/catalog.rs` has a complete `//!` block with Purpose, Key Types, and Compliance (AC-16, AU-3, SI-10, CMMC). All major types (`CatalogMetadata`, `DisseminationControl`, `Catalog`, `Marking`, `LevelDefinition`, `LevelRegistry`) carry `## Compliance` sections. Citations use canonical form throughout.

**A-6** — `cui/locale_text.rs` has a complete `//!` block with Purpose, Key Types, and Compliance (AC-16, SI-10, AU-3). The `LocaleText` type carries its own compliance annotations.

**A-7** — `cui/palette.rs` has a `//!` block with Compliance (AC-16). The block explicitly acknowledges the placeholder status.

**A-8** — `tui/mod.rs` has a complete `//!` block with Purpose, Sub-modules, Layout diagram, and Compliance (AC-16, AU-3, AC-3, NSA RTB RAIN). The NSA RTB RAIN citation is correctly applied to the non-bypassable read-only contract.

**A-9** — `tui/app.rs` has a complete `//!` block with Purpose, Key Types, Tree Structure documentation, and Compliance (AC-16, AU-3, AC-3, NSA RTB RAIN). The `LabelRegistryApp` struct carries its own `## Compliance` block. NSA RTB RAIN used correctly.

**A-10** — `tui/render.rs` has a complete `//!` block with Purpose, Layout description, and Compliance (AU-3, AC-3, AC-16, NSA RTB RAIN). RTB RAIN applied correctly to the renderer's immutable contract.

**A-11** — No `unwrap()` calls found in production source (`src/`). Clippy deny is honoured. The single use of `expect()` in `validate.rs` line 77 is for a compile-time literal regex that cannot fail at runtime; the comment documents the reasoning inline. No `unwrap()` in tests per project policy.

**A-12** — `#[must_use]` annotations with descriptive messages are present on all public functions returning `Result`, `Option`, or security-relevant types. No bare `#[must_use]` (without message) was found in this crate — this crate is clean on that dimension.

**A-13** — `#![forbid(unsafe_code)]` is present in both `lib.rs` (line 7) and `main.rs` (line 35). The compile-time proof annotation is in canonical form with the NIST SP 800-218 SSDF PW.4 / NSA RTB rationale comment.

**A-14** — `#![deny(clippy::unwrap_used)]` is present in both `lib.rs` (line 13) and `main.rs` (line 38).

**A-15** — OsDetector integration in `main.rs` is correct. The recent fix replaces raw `/etc/os-release` reads with `OsDetector::default().detect()`, flowing through provenance-verified `SecureReader` paths. The comment on lines 190–192 documents the rationale. The `.ok().and_then()` chain fails closed to `"unavailable"` on any error, satisfying the fail-closed default requirement.

**A-16** — `LocaleText` deserialization is fail-closed: the `LocaleTextVisitor` rejects any JSON value that is neither a string nor an object with string values, returning a serde error. Non-string locale values (numbers, arrays, booleans, null) cause deserialization to fail at the trust boundary. This satisfies NSA RTB RAIN and NIST SP 800-53 SI-10.

**A-17** — `load_catalog()` and `load_levels()` both carry `#[must_use]` with descriptive messages and return `Result<T, String>` with human-readable diagnostics. The error messages include the path (`path_ref.display()`), not internal library internals — information discipline is maintained.

**A-18** — Canadian catalog `CANADIAN-PROTECTED.json`: all three entries (`PROTECTED-A`, `PROTECTED-B`, `PROTECTED-C`) have `index_group: null` as required. The Canadian catalog has no dissemination controls section, consistent with the documented structural difference.

**A-19** — MCS category range does not overlap between US (c0-c299) and Canadian (c300-c399). The ranges are disjoint and consistent with the `labeling_mcs.md` rules.

**A-20** — All CUI marking keys use `CUI//CATEGORY` double-slash format. All specified categories use the `SP-` prefix (e.g., `CUI//SP-CTI`, `CUI//SP-CVI`). Abbreviations are uppercase A-Z with the permitted `SP-` prefix. No abbreviation exceeds 15 characters.

**A-21** — `validate_tests.rs`: covers valid single-segment, valid hyphenated (`SP-CTI`), valid two-segment, and invalid forms (empty, lowercase, missing double-slash, trailing slash, numeric segment, spaces, and double-slash-only). Regex cache idempotency is tested. Test coverage is adequate for the current scope of the `CuiMarking` pattern.

**A-22** — `catalog_tests.rs`: comprehensive — covers US catalog loading, metadata, marking count bounds, lookup, field predicates, iteration, level registry, country flag utility, and cross-catalog compatibility. Canadian catalog tests cover all three tiers, bilingual names, and null `handling_group_id`. Test structure follows the project rule (tests in `tests/`, not inline).

**A-23** — `setrans_tests.rs` (partially reviewed): module-level comment present with compliance citations. Test structure is correct (external test directory).

**A-24** — No enforcement language found in any `.rs` source file. The three hits in the enforcement grep are: "enforces umask" (system call, not CUI enforcement), "enforces" in `locale_text.rs` (referring to fidelity at deserialization boundary, not CUI access control), and "no enforcement semantics" on `index_group` (explicitly denying enforcement, which is correct).

**A-25** — `ratatui::init()` / `ratatui::restore()` are correctly paired in `run_tui()`. `ratatui::restore()` always runs at function end; the event loop is extracted into `run_event_loop()` to guarantee `restore()` fires regardless of how the loop terminates.

**A-26** — `--json` flag is handled: the binary exits with an informative message (line 96–98 of `main.rs`). The `--cli` / non-TTY fallback is implemented. Both structured modes comply with the TUI/CLI rules.

**A-27** — `Protected C` `phase_note` in `CANADIAN-PROTECTED.json` (line 200) correctly states that "the label is applied and visible but not kernel-enforced" — this is accurate Phase 1 language that does not overclaim enforcement.

**A-28** — `LevelDefinition.description` comment on `catalog.rs` line 388 mentions "enforcement characteristics" in the context of a sensitivity level definition, not a claim about UMRS Phase 1 enforcement. The field stores authoritative TBS/NARA descriptions of MLS levels. This is accurate — it describes what the levels would mean under MLS enforcement, which is a factual characterization of the levels, not a claim about what UMRS currently does.

---

## Remediation Owner Summary

| Priority | Finding | Owner | Severity |
|---|---|---|---|
| 1 | E-1: MAC enforcement language in CUI description field | tech-writer | HIGH |
| 2 | E-2: CuiMarking regex rejects valid full-banner LDC strings | coder | HIGH |
| 3 | E-3: `handling_group_id: ""` should be `null` (139 entries) | tech-writer | MEDIUM |
| 4 | C-2: Missing required_warning_statement on SP-TAX, SP-PCII, SP-SGI | tech-writer | MEDIUM |
| 5 | C-1: RELIDO missing NOFORN mutual-exclusivity + name accuracy | tech-writer | LOW |
| 6 | C-3: AC-4 setrans_tests comment overstates Phase 1 enforcement | coder | LOW |
| 7 | C-4: US catalog `mcs_ranges` not surfaced in TUI metadata panel | coder + tech-writer | LOW |
| 8 | C-5: OnceLock/Mutex pattern in validate.rs violates construction pattern rule | coder | LOW |
| 9 | C-6: `palette.rs` is an empty public module | coder | LOW |

---

## Strengths Worth Preserving

**Annotation discipline is excellent.** Every module has a `//!` block with a `## Compliance` section. Every public function returning `Result` or `Option` has `#[must_use]` with a descriptive message. The team has eliminated the bare `#[must_use]` pattern that was a workspace-wide debt item in the 2026-04-02 audit. This crate is the cleanest in the workspace on that metric.

**Fail-closed defaults are correct throughout.** `LocaleText` deserialization fails on bad input. `OsDetector` fails to `"unavailable"`. `load_catalog()` returns `Err`, not a partial catalog. No silent data corruption paths were found.

**OsDetector integration is a model fix.** The comment explaining why the OsDetector pipeline is used instead of raw file I/O is precise and auditable. Future reviewers will understand the provenance trust chain from the comment alone.

**The Canadian catalog is structurally exemplary.** Bilingual names, accurate injury descriptions quoted from TBS, `index_group: null`, correct MCS range, accurate mutual exclusivity documentation, and a well-hedged `phase_note` on Protected C that accurately describes Phase 1 limitations.

**Test coverage is strong.** The three test files cover 50+ distinct cases across validation, catalog loading, field predicates, iteration, level registry, country flag computation, setrans consistency, and cross-catalog compatibility. The country flag test suite in particular is thorough edge-case coverage.

**CUI Phase 1 language is clean in source code.** The enforcement language violations are confined to a JSON data field (E-1) and a test comment (C-3). The Rust source code itself never claims enforcement capability.
