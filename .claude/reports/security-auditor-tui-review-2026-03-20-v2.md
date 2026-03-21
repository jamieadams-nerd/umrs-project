# Security-Auditor TUI Review v2 — Kernel Security & Trust/Evidence Tabs
## Date: 2026-03-20
## Score: 26 ACCURATE / 3 CONCERN / 0 ERROR
## Prior score (v1): 14 ACCURATE / 17 CONCERN / 3 ERROR

---

## FINDING RESOLUTION FROM v1

### Errors — all resolved

**E-1 RESOLVED:** `kptr_restrict = 2` now annotated `"hidden from all users"` in
`translate_integer()` (line 1392) and the recommendation string (line 1182) uses
`"2 (hidden from all users)"`. The two strings now agree.

**E-2 RESOLVED:** `NfConntrackAcct` moved out of MODULE RESTRICTIONS into a dedicated
NETWORK AUDITING group (line 879–887). The group description accurately names
traffic-accounting and anomaly detection. No longer adjacent to exfiltration vector
language.

**E-3 RESOLVED:** `UnprivUsernsClone` remains in PROCESS ISOLATION with its own
`indicator_description` entry explaining unprivileged user namespaces. The
"Unavailable" display for absent RHEL kernels now passes through the standard
`IndicatorRow` path; the indicator is silently omitted when unreadable (the group is
only rendered when at least one indicator has a readable value). E-3's core ask was
that the display not look like a probe failure — silent omission is the correct
policy and is implemented correctly.

### Key Concerns — resolution status

**C-4 RESOLVED:** `ModulesDisabled` is now in BOOT INTEGRITY group (line 920–921).
A comment in `append_boot_integrity_group` (line 913–916) explains the threat-model
rationale: tamper-resistance, not a cryptographic primitive.

**C-7 PARTIALLY RESOLVED:** Help text for the Kernel Security tab (line 1836) still
reads `"Red rows require remediation before CUI processing."` This is slightly less
absolute than v1 because contradiction explanations were added below it, but the
sentence itself is unchanged. See new finding C-7v2 below.

**O-2 RESOLVED:** Kernel tab indicators now render with ✓ / ✗ / ? symbols prefixed to
every value (lines 1279–1283) so hardening state is readable in NO_COLOR mode
(WCAG 1.4.1). The `⚠` symbol is used for contradiction markers. This fully addresses
the NO_COLOR gap.

**C-12 RESOLVED:** Verification column now shows `"✓ ok (fd, PROC_MAGIC)"`,
`"✓ ok (fd, SYS_MAGIC)"`, and `"✓ ok (fd, statfs)"` per source kind
(`evidence_verification_str`, lines 1700–1712). The help text for Tab 2 (lines
1877–1883) explains each code. An SP 800-53A assessor can now identify the exact
examination method inline.

**C-15 PARTIALLY RESOLVED:** The Trust/Evidence tab now uses `.take(64)` (line 547)
for contradictions in `build_trust_summary_rows`, up from the original 48. However,
`label_trust_display` (line 1750) still applies `.take(48)` to the
`IntegrityVerifiedButContradictory` contradiction field before rendering it as the
Label Trust value. The 48-char limit is applied to an already-formatted string that
includes the prefix `"IntegrityVerifiedButContradictory: "` (35 chars), leaving only
13 chars for the actual contradiction description. A T4-then-contradictory finding
will have its full description silently discarded. No secondary row is emitted with
the full text. See new finding C-15v2 below.

**C-13 RESOLVED:** T3 (`SubstrateAnchored`) now maps to `StyleHint::TrustGreen`
(line 99). Trust tier badge renders green for T3 and T4 as expected.

**C-16 RESOLVED:** Evidence groups follow `PIPELINE_ORDER` (lines 1547–1553):
Procfs → SysfsNode → StatfsResult → RegularFile → PackageDb → SymlinkTarget.
This is the trust-elevation narrative order. A comment explains the rationale.

**O-3 RESOLVED:** T4 SHA-256 digest and package-database reference digest are both
surfaced inline beneath the evidence row (lines 1639–1671). The algorithm label
includes a `"md5 (weak)"` warning for the `Md5` variant.

---

## WHAT WORKS WELL — ADDITIONS SINCE v1

**Status bar text color:** `status_text` in `Theme::default()` now uses
`Color::White` with `Modifier::BOLD` (line 201–203). Previously it was black on
colored backgrounds, making it unreadable on the blue Info bar.

**Sticky header alignment:** `render_scrollable_pane` now uses `theme.header_field`
(bright cyan) instead of `theme.data_key` (dim cyan) for the reversed sticky header,
making it visually distinct and readable at all scroll positions.

**Live kernel provenance statement:** Summary pane for Kernel Security tab now opens
with `"All values below are read live from the running kernel via /proc and /sys."`
This satisfies the NIST SP 800-53 CA-7 requirement that collection context be present
in the display without relying on the operator knowing the tool's internals.

**Indicators summary format:** The improved `"N readable — all hardened ✓"` /
`"N readable — N hardened, N not hardened (N%)"` format directly answers the
assessor's scope question. The old `"X of Y total"` framing was misleading because
"total" included unreadable indicators. The new format is unambiguous.

**Contradiction explanations:** Both tabs now include explanations in the help text
and inline. The Kernel Security tab help text (lines 1838–1845) names all three
contradiction kinds with their meaning. The Trust/Evidence help text (lines 1856–1862)
correctly distinguishes OS detection contradictions from kernel/config contradictions —
an important distinction that prevents operator confusion between the two different
contradiction concepts.

**Evidence ordering rationale:** `PIPELINE_ORDER` has a comment explaining
trust-elevation narrative order (lines 1558–1561). Assessors following the chain top
to bottom follow increasing trust tiers naturally.

**Internal reference prohibition:** No `/// Finding N` or `/// RAG Finding N` style
references appear in any doc comment. All rationale is self-contained.

**`⚠ UNVERIFIABLE` text is accurate:** The contradiction text reads
`"UNVERIFIABLE: config exists but kernel node unreadable"` (line 656–657). This
is correct — `SourceUnavailable` means the kernel node could not be read, not that
the configuration itself is suspect.

---

## REMAINING CONCERNS

### C-7v2 (LOW)

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
**Location:** line 1836
**Finding:** Help text for Kernel Security tab still says `"Red rows require remediation
before CUI processing."` This statement is too absolute. Some red indicators
(e.g., `bluetooth (blacklisted)` showing Not Present on a RHEL server where Bluetooth
was never installed) are informational absences rather than active risks. `NfConntrackAcct`
disabled degrades logging but does not block CUI. The sentence should be
`"Red rows indicate settings that do not meet the hardened baseline — review each
indicator's description to determine remediation priority."` The current text may
cause operators to block legitimate CUI work on false urgency.
**Severity:** LOW
**Recommended fix:** Replace line 1836 with: `"Red rows do not meet the hardened
baseline — review each indicator's description for remediation priority."`
**Owner:** coder

### C-15v2 (MEDIUM)

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
**Location:** line 1750 (`label_trust_display`)
**Finding:** `IntegrityVerifiedButContradictory` contradiction string is truncated to
48 characters before formatting. The rendered string is:
`"IntegrityVerifiedButContradictory: <desc[:13]>"`. The prefix `"IntegrityVerifiedButContradictory: "` alone
is 35 characters, so the visible description is at most 13 characters. A T4 finding
that becomes contradictory — the most security-sensitive trust state transition —
silently clips the contradiction description to 13 characters in the Label Trust row.
No secondary row with the full text is emitted. An assessor recording this as a
T4 integrity finding cannot read the full contradiction without consulting raw data
outside the TUI.

The contradictions in `confidence.contradictions` (line 545–553) correctly use
`.take(64)` and also emit a `source_a vs source_b` label row. The `label_trust_display`
path has no equivalent.

**Severity:** MEDIUM
**Recommended fix:** Either (a) increase the limit to 96 chars and use a `KeyValue`
row with an empty key for the overflow if it exceeds the column, or (b) emit a
secondary description row (empty-key `KeyValue`) with the full text immediately below
the Label Trust row when the variant is `IntegrityVerifiedButContradictory`. Option
(b) is consistent with how downgrade reasons and contradictions are handled elsewhere.
**Owner:** coder

### C-T3-STATUS (LOW)

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
**Location:** line 1764–1766 (`build_status`)
**Finding:** `SubstrateAnchored` (T3) maps to `StatusLevel::Info` in the status bar,
rendering a blue background. The `trust_level_hint` function correctly returns
`TrustGreen` for T3 — T3 is a positive trust outcome. The status bar for T3 should
use `StatusLevel::Ok` (green background) to be consistent with the data panel styling.
Currently a T3 result shows a blue `Info` bar while all the data panel rows are green,
creating an inconsistent signal to the operator.
**Severity:** LOW
**Recommended fix:** Change line 1765 to `StatusMessage::new(StatusLevel::Ok, ...)`.
**Owner:** coder

---

## COMPLIANCE ANNOTATION REVIEW

All modules under `umrs-tui/src/` have `//!` doc blocks with `## Compliance` sections
and canonical `NIST SP 800-53` citations (not the abbreviated `NIST 800-53` form).
NSA RTB citations use the correct form. No missing module-level annotations found.

Security-critical public types (`IndicatorValue`, `SecurityIndicators`, `DataRow`,
`HeaderContext`, `DialogState`, `StatusLevel`, `StyleHint`) all carry explicit control
citations at the type level. The tiered annotation requirement is satisfied.

`#[must_use]` annotations with descriptive messages are present on all security-relevant
return types and constructors. No bare `#[must_use]` without a message was found.

`#![forbid(unsafe_code)]` is present in both `lib.rs` and `main.rs` with the required
SSDF PW.4 / NSA RTB attribution comment.

The `Citation Format Rule` is satisfied throughout: `NIST SP 800-53` (not abbreviated),
`NSA RTB RAIN`, `NIST SP 800-218 SSDF PW.4`, `CMMC SC.L2-3.13.10` — all in canonical
form. No CCE citations are present; none are required for TUI display code.

---

## ACCURACY ASSESSMENTS (previously in WHAT WORKS WELL)

These were previously ACCURATE items in v1 and remain accurate in v2:

- Indicator descriptions are technically accurate, threat-model-focused, and self-contained
- Every description follows: what the setting controls + what the attack is if it fails
- Recommended values correct for all 27+ indicators
- `[ Recommended: ... ]` only on red indicators is correct
- Pinned summary pane keeps trust finding always visible (SP 800-53A)
- Contradiction framing "may indicate tampering" is correctly hedged
- `⚠` symbols ensure NO_COLOR compliance for contradiction markers
- Evidence type labels are plain English; operators need no prior knowledge of internal names
- SHA-256 and pkg_digest display is technically accurate and complete
- `PIPELINE_ORDER` trust-elevation narrative is correct (kernel → config → package)
- Both tabs' help text correctly distinguish their respective contradiction types
- T4 `sha256` digest is displayed as hex using `bytes_to_hex` (allocation-efficient)
- `md5 (weak)` label on MD5 package digests is appropriate and informative
- `"Not Present"` display for `"absent"` sentinel (line 1364–1365) is operator-appropriate
- Contradiction marker ordering (before recommendation) is correct per urgency model
- `BootDrift = TrustRed`, `EphemeralHotfix = TrustYellow`, `SourceUnavailable = Dim` is correct
- `UNVERIFIABLE` text accurately describes `SourceUnavailable` (config exists, node unreadable)
- Contradiction explanation in Kernel Security help text names all three kinds correctly
- Trust/Evidence help text correctly separates OS detection contradictions from kernel/config contradictions

---

## GAP ANALYSIS SUMMARY

**Files reviewed:** 10 (`main.rs`, `app.rs`, `data_panel.rs`, `dialog.rs`, `keymap.rs`,
`theme.rs`, `header.rs`, `status_bar.rs`, `lib.rs`, `indicators.rs`)

**Total findings:** 3 (0 HIGH, 1 MEDIUM, 2 LOW)

**Resolved from v1:** 3 ERROR (all resolved), 14 of 17 CONCERN resolved

**Uncited security claims:** None identified. All load-bearing security claims carry
control citations.

**Inconsistencies (code vs. docs):** None. All documentation accurately describes
the implementation. The `⚠ DRIFT / NOT PERSISTED / UNVERIFIABLE` marker texts in
the help dialog exactly match the strings rendered by `expand_indicator_row`.

**Remaining items requiring action:**

| ID | Severity | Description | Owner |
|---|---|---|---|
| C-7v2 | LOW | Help text "Red rows require remediation before CUI processing" is too absolute | coder |
| C-15v2 | MEDIUM | `IntegrityVerifiedButContradictory` truncated to 13 usable chars in Label Trust row | coder |
| C-T3-STATUS | LOW | T3 shows `StatusLevel::Info` (blue) but data panel renders T3 green — inconsistent signal | coder |
