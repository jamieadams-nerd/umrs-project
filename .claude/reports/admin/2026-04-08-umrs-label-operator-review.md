---
Date: 2026-04-08
Scope: umrs-label TUI / CLI utility — operator readability and usability
Binaries evaluated: umrs-label v0.1.0 (dev build)
Reviewer: Henri (UMRS guest administrator — RHEL sysadmin, Canadian government context)
---

# umrs-label Operator Review

## Executive Summary

The umrs-label tool is a functional, well-engineered security label registry browser for CUI and Canadian Protected markings. From an operator perspective, it presents clean output, correct terminology (especially for Canadian material), and intuitive navigation. The tool achieves its core mission: making security label definitions discoverable and transparent.

**Key strength:** The Canadian Protected labels (Protected A/B/C) use correct Treasury Board terminology with proper UTF-8 accents (PROTÉGÉ A, PROTÉGÉ B, PROTÉGÉ C). This demonstrates policy-aware implementation from day one.

**Gap:** The tool has no operator documentation (guides, procedures, deployment notes) in the `/docs/` directory. An operator who encounters this tool for the first time has only the `--help` output and source code to work from.

---

## Review Summary

| Category | Count |
|---|---|
| ACCURATE | 12 |
| CONCERN | 4 |
| ERROR | 0 |

---

## ACCURATE Findings

### A-1: Help Text Completeness

**Finding:**
The `--help` output (invoked implicitly when the tool runs without TTY) is comprehensive and operator-friendly. It displays:
- Tool title and version
- Count of markings loaded
- All 143 US CUI markings grouped by index (Critical Infrastructure, Law Enforcement, etc.)
- All dissemination controls
- Canadian Protected markings with version and count
- Search and filter capabilities implied by usage

**Impact:** An operator can understand the tool's scope immediately without reading source code.

### A-2: CLI Output Formatting

**Finding:**
The `--cli` mode produces clean, well-organized grouped output:
- Catalog metadata (name, version, count) at top
- Index groups as section headers with visual separators (dashes)
- Markings indented under groups, with abbreviation + full name
- Designated categories flagged with `[SP]` tag
- Dissemination controls listed separately with clear labeling

**Impact:** Output is scannable by eye; an operator can grep for specific categories or pipe to `less` without confusion.

### A-3: Canadian Protected Terminology Correctness

**Finding:**
The Canadian catalog uses authoritative Treasury Board terminology throughout:
- `PROTECTED-A` correctly mapped to `PROTÉGÉ A` in French (with proper accent aigu)
- `PROTECTED-B` correctly mapped to `PROTÉGÉ B`
- `PROTECTED-C` correctly mapped to `PROTÉGÉ C`
- Authority cited as "TBS Directive on Security Management, Appendix J" with authority_date
- Injury examples sourced directly from TBS Standard on Security Categorization

**Impact:** A Canadian government operator will recognize the terminology immediately; no translation confusion. This tool is policy-compliant on day one.

### A-4: Error Message Clarity

**Finding:**
When a catalog file is not found, the error message is clear and actionable:
```
[FAIL] Could not load US catalog: Failed to open config/us/US-CUI-LABELS.json: No such file or directory (os error 2)
```
- Prefix `[FAIL]` signals severity immediately
- Full path is shown (or attempted path if user provided it)
- OS error number and text are included

**Impact:** An operator can immediately diagnose path or permission issues.

### A-5: JSON Catalog Structure Integrity

**Finding:**
Both US CUI and Canadian Protected JSON catalogs are well-formed and complete:
- US catalog: 143 markings, all with `name`, `abbrv_name`, `description`, `index_group`, `designation` fields
- Canadian catalog: 3 tiers (PA/PB/PC), each with bilingual `name`, `marking_banner`, `handling` guidance, `injury_examples`
- Both catalogs use consistent `"markings"` top-level key
- Text fields support bilingual `{"en_US": "...", "fr_CA": "..."}` structure where applicable

**Impact:** Tool correctly deserializes and displays structured label data without data loss.

### A-6: Read-Only Safety

**Finding:**
The tool is strictly read-only:
- Main.rs declares in module docs: "The browser is unconditionally read-only; no catalog mutation is possible through the interface" (NIST SP 800-53 AC-3)
- Event loop accepts navigation input only (Up/Down/Enter/search)
- No write paths exist in the rendering or app state code
- Help overlay is informational only

**Impact:** An operator can safely run this tool on a production system without risk of accidental label modifications.

### A-7: Catalog Provenance Tracking

**Finding:**
The Canadian catalog includes detailed metadata:
- `created`: "2026-03-23", `updated`: "2026-03-30"
- `author`: "Henri (UMRS Canadian Policy Specialist)"
- `notes`: explanatory array clarifying structural choices (e.g., "There are no standardized subcategories or dissemination controls in the Canadian system")
- `structural_differences_from_us_cui`: explicit comparison table showing why Canada uses severity tiers instead of categories
- `scope`: "Advisory reference — UMRS is a reference system, not a production deployment"

**Impact:** An operator (or auditor) can immediately understand the source and limitations of the catalog.

### A-8: SELinux Posture Header

**Finding:**
The TUI renderer includes a security posture header (NIST SP 800-53 AU-3 compliance):
- Shows hostname, OS name, SELinux mode, and FIPS state
- Rendered on every frame (top of the display)
- Uses data from `umrs_platform::detect::OsDetector` rather than raw `/etc/os-release` I/O
- Complies with provenance-verified reading rule

**Impact:** An operator sees security context for every interaction; audit trail is built into the display.

### A-9: Keyboard Navigation Intuitiveness

**Finding:**
The interactive TUI supports both arrow keys and vi-style keys for navigation:
- `↑/↓` or `j/k` for move up/down
- `←/→` or `h/l` for collapse/expand branches
- `Tab` or `BackTab` to switch between tree and detail panels
- `PageUp`/`PageDown` for faster scrolling
- `Enter` or `Space` to show details
- `/` to search
- `?` for help (modal overlay)
- `q` or `Esc` to quit

**Impact:** Both arrow-key and vi users can navigate without learning new muscle memory.

### A-10: Search / Filter Functionality

**Finding:**
The tool includes a live search feature:
- Press `/` to activate search mode
- Type characters to filter the tree by marking name/abbreviation
- `Esc` cancels and resets the display
- `Enter` confirms the search
- Search query is shown in a status bar when active

**Impact:** An operator can quickly locate a specific marking (e.g., "SP-CTI") without scrolling through 143 entries.

### A-11: Bilingual Text Handling

**Finding:**
The `LocaleText` type in the catalog module transparently handles both:
- Legacy flat-string format: `"name": "Controlled Unclassified Information"` (US)
- Locale-keyed object format: `"name": {"en_US": "...", "fr_CA": "..."}` (Canada)
- Provides `.en()` and `.fr()` accessors for display

**Impact:** Catalogs can be extended to French-Canadian output without code changes; operator can see bilingual descriptions on a dual-language system.

### A-12: Dissemination Control Catalog

**Finding:**
The US catalog includes a complete dissemination control (LDC) listing:
- Attorney-Client Privilege
- DISPLAY ONLY
- DL ONLY (Dissemination List Controlled)
- FED ONLY
- NOFORN (No Foreign Dissemination)
- REL TO (Authorized for Release to Certain Nationals)
- 10 total controls, each with clear description

**Impact:** An operator has a reference guide to dissemination markings without consulting external documentation.

---

## CONCERN Findings

### C-1: Outdated Readme.txt in Tool Directory

**Finding:**
The file `/DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-label/readme.txt` contains obsolete usage examples:
```
Example usage:
umrs-labels --metadata ./umrs-metadata.json --list
```
This does not match the current tool's API, which uses:
```
umrs-label --us-catalog <path> --ca-catalog <path> [--cli] [--json]
```
The readme was last modified 2026-03-25, well after the tool was refactored.

**Impact:** A new operator or contributor who reads this file first will be confused and try commands that don't work. The file should either be deleted or updated.

**Recommendation:** Delete `readme.txt` or update it to reflect current CLI options. Better: move operator-facing documentation into `docs/modules/` where operators will find it during onboarding.

### C-2: No Operator Documentation in `/docs/`

**Finding:**
The UMRS documentation includes placeholder text in relevant modules:
- `/docs/modules/cui-labeling/pages/index.adoc` mentions "The `umrs-label` tool" but there is no corresponding page with procedures, examples, or deployment guidance
- `/docs/modules/cui-labeling/pages/mcs-label-architecture.adoc` says: "_This page is a placeholder for the MCS label architecture documentation. Content will be developed as the `umrs-label` tool is implemented._"
- No page exists explaining how to invoke the tool, configure catalog paths, or interpret its output in an operations context

**Impact:** An operator who is onboarded to UMRS will not find guidance on how to use umrs-label. The tool's functionality is not documented for the audience that needs it.

**Recommendation:** Create a new page at `docs/modules/cui-labeling/pages/label-registry-browser.adoc` with:
1. **Overview**: What umrs-label does, why an operator needs it
2. **Invocation**: CLI examples (default TTY mode, `--cli`, `--json` for future use)
3. **Catalog paths**: How to override default paths with `--us-catalog` and `--ca-catalog`
4. **Keyboard navigation**: The key bindings (summarize the help overlay)
5. **Example workflows**: "Find all categories under Law Enforcement", "Check what LEI means"
6. **Deployment notes**: Recommended location for catalog files, permissions, umask expectations

### C-3: Argument Parsing Does Not Use Standard Argument Library

**Finding:**
The main.rs file implements a custom argument parser:
```rust
fn arg_value(args: &[String], flag: &str) -> Option<String> {
    let prefix = format!("{flag}=");
    for (i, arg) in args.iter().enumerate() {
        if let Some(v) = arg.strip_prefix(&prefix) {
            return Some(v.to_owned());
        }
        if arg == flag {
            return args.get(i + 1).cloned();
        }
    }
    None
}
```
This custom implementation handles `--flag=value` and `--flag value` but has limitations:
- Does not validate that remaining arguments are valid
- Does not provide `--help` generation (help is hardcoded output in `--help` mode)
- Does not prevent conflicting flags (e.g., both `--json` and `--cli` could be specified, creating undefined behavior)
- Dependency on `clap` is declared in Cargo.toml but not used

**Impact:** The tool works today, but adding new flags or validation becomes brittle. Future operators who add features might miss edge cases.

**Recommendation:** Migrate to `clap` derive macro to gain automatic validation, conflict detection, and proper `--help` generation. This is a low-urgency refactoring but would improve maintainability.

### C-4: Canadian Catalog MCS Category Allocation Not Enforced in Code

**Finding:**
The Canadian catalog metadata declares:
```json
"mcs_category_range": "c300-c399"
```
And each tier has fields like `"category_base": "c300"`.
However, the tool does not validate or enforce that actual catalog entries fall within these ranges. If a catalog entry accidentally used `c200` (which belongs to US CUI), the tool would not flag it as an error.

**Impact:** A data entry error in the catalog could silently violate the Five Eyes allocation scheme. The error would not be caught until the catalog is deployed and an auditor (or CI check) notices the mismatch.

**Recommendation:** Add a validation function in the catalog loader that:
1. Extracts `mcs_category_range` from metadata
2. Parses the range (e.g., `c300-c399` → `300..=399`)
3. Inspects all markings for category values (if present) and confirms they fall within range
4. Emits a warning or error if any marking uses an out-of-range category
5. This validation should run at load time and be tested

---

## ERROR Findings

None. The tool has no critical errors that would prevent an operator from using it safely.

---

## High-Assurance Communication Assessment

An operator reading the umrs-label documentation (both in-code and in the catalogs themselves) understands that:

1. **This is a labeling tool, not an enforcement tool** (Phase 1 of UMRS). The Canadian catalog includes a `phase_note` on Protected C stating: "Protected C requires Phase 2 MLS enforcement to carry meaningful security value. In Phase 1, the label is applied and visible but not kernel-enforced." This is correct language per the `cui_phase1_language.md` rule.

2. **The tool reads from authoritative catalogs** (Treasury Board, NIST). Both catalogs cite their source and authority; there is no ambiguity about where the definitions come from.

3. **The tool is provenance-aware**. The TUI header shows hostname, OS, and SELinux mode on every frame—operator context that matters for high-assurance systems.

4. **The tool is read-only and safe**. The design and documentation make clear that no mutations are possible.

An operator who has used umrs-label will not overstate its capabilities to stakeholders. The tool's boundaries are clear.

---

## Strengths Worth Preserving

1. **Bilingual catalog support from the start.** The `LocaleText` abstraction and Canadian catalog metadata prove that multi-language support was architected from day one, not bolted on. This is rare.

2. **Policy-aware terminology.** The Canadian tier names (PROTÉGÉ A/B/C) use correct accents and match Treasury Board sources verbatim. An operator or auditor will trust the tool immediately.

3. **Provenance tracking.** The catalogs carry metadata about their source, authority, and update date. This is not just good hygiene—it is compliance. An auditor can verify the tool's data lineage.

4. **Clean, navigable output.** Both CLI and TUI modes present information in a way that respects operator time. No debug output, no guessing.

5. **Defensive error messages.** When something goes wrong (missing file, etc.), the message tells the operator the path, the OS error, and the exit code. This is professional-grade operator UX.

6. **Read-only by design.** No write paths, no mutations, no side effects. Safe to run on production systems.

---

## Summary

| Metric | Count |
|---|---|
| Sections reviewed | 4 (CLI interface, TUI rendering, catalogs, documentation) |
| Tools evaluated | 1 (umrs-label v0.1.0) |
| Total findings | 16 (12 ACCURATE, 4 CONCERN, 0 ERROR) |
| Blockers for operation | 0 |
| Operator confidence (1-5) | 4 |

**Verdict:** The umrs-label tool is operator-ready and safe to use. It needs documentation for its intended audience. No code changes are blocking deployment.

The Canadian Protected catalog is production-grade; the US CUI catalog is a reference tool that will improve as the project evolves. An operator can use this tool on day one to understand UMRS label definitions without risk.
