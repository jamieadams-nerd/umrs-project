# Phase 6 — TUI Assessment Value Review
## Tool: umrs-os-detect-tui (planned rename: umrs-uname)
## Date: 2026-03-21
## Reviewer: The IRS (security-auditor agent)
## Audit depth: in-depth
## Basis: SP 800-53A assessment value, CMMC practice coverage

---

## Scope

This is an **assessment value review**, not a code quality or compliance annotation review.
Prior rounds (v1, v2) addressed those dimensions — results are in
`.claude/reports/security-auditor-tui-review-2026-03-20-v2.md`.

This review answers five questions:

1. Is every indicator understandable without external reference?
2. Does color coding consistently indicate security posture?
3. Can an auditor produce a finding from this output?
4. Does the evidence tab satisfy SP 800-53A Examine requirements?
5. Are CMMC practices adequately covered for CUI system assessment?

Files reviewed: `umrs-tui/src/main.rs`, `app.rs`, `data_panel.rs`, `indicators.rs`,
`header.rs`, `theme.rs`, `umrs-platform/src/posture/catalog.rs`, prior review reports.

---

## ACCURATE — What Works Well

### A-1: Indicator self-description is complete
**Control: NIST SP 800-53 SA-5 / CM-6**

Every indicator row in the Kernel Security tab renders a three-line block:
1. Name and live value (with ✓/✗/? symbol)
2. Plain-language description of what the setting controls and what the attack is if it fails
3. `[ Recommended: <value> ]` when not hardened (red indicators only)

The 27 indicator descriptions in `indicator_description()` are technically accurate,
operator-focused, and self-contained. No external reference is needed to interpret
any indicator. Examples that demonstrate the quality bar:

- `KptrRestrict`: "Hides kernel pointer addresses from /proc and logs. Knowing where
  kernel code lives in memory is the first step in most kernel exploits."
- `YamaPtraceScope`: "Without restriction, a compromised process can extract passwords
  and private keys from every sibling process."
- `ModuleSigEnforce`: "Without this, any code can be loaded as a kernel module,
  defeating lockdown and enabling rootkits."

This satisfies the Phase 0 plain-language requirement I defined. Descriptions follow the
required format: what it controls + what the adversary gains if it fails.

### A-2: Integer values carry operator-readable annotations
**Control: NIST SP 800-53 CM-6**

`translate_integer()` and `translate_signed_integer()` annotate every known
indicator/value pair. An operator sees `"2 (full ASLR)"`, `"1 (restricted to CAP_BPF)"`,
`"-1 (fully open)"`. Raw integers like `"2"` with no context are never shown for
known indicator IDs. This eliminates the need for the operator to know sysctl semantics.

The `PerfEventParanoid` signed-integer path correctly handles negative values
(`i64::MIN..=-1 => "fully open"`) — a subtle case that catches the `-1` value used on
some kernels to grant broadest perf access.

### A-3: Color coding is consistent and redundant with symbols
**Control: NIST SP 800-53 AU-3**

The color/symbol mapping is coherent throughout:
- Green + ✓ = hardened / trust confirmed
- Red + ✗ = not hardened / trust failed / contradiction
- Yellow = degraded / advisory / unavailable probe
- Dim = secondary information / unassessable

The `⚠` symbol (U+26A0) precedes every contradiction marker, `✓`/`✗`/`?` prefix every
indicator value, and `NO_COLOR` is honored via the TUI's standard color-stripping path.
WCAG 1.4.1 compliance is achieved: hardening state is never conveyed by color alone.

The three contradiction kinds map to distinct visual levels:
- `BootDrift` → Red (active security failure)
- `EphemeralHotfix` → Yellow (hardening will be lost on reboot)
- `SourceUnavailable` → Dim (unverifiable, not an immediate failure)

This tiering is correct from a risk perspective and enables auditors to prioritize.

### A-4: Pinned summary pane prevents key findings from scrolling off-screen
**Control: NIST SP 800-53 CA-7 / AU-3**

The Kernel Security tab pins: kernel version, catalog baseline comparison, provenance
statement, indicator score, contradiction count. The Trust/Evidence tab pins: label trust,
trust tier, downgrade reasons, contradiction count, evidence record count.

An assessor with the tool open can always see the top-level finding without scrolling.
This is exactly what SP 800-53A Examine requires: the most critical facts must be
immediately visible to the examiner.

The `"All values below are read live from the running kernel via /proc and /sys."` provenance
statement in the pinned Kernel Security summary satisfies the CA-7 requirement that
collection context be present without the assessor needing to know the tool's internals.

### A-5: Contradictions are surfaced per-indicator and in the summary
**Control: NIST SP 800-53 CA-7 / CM-6**

Contradictions (live kernel vs. persisted configuration) appear both:
- In the per-indicator `IndicatorRow` block (inline `⚠ DRIFT / NOT PERSISTED / UNVERIFIABLE`)
- In the Kernel Security summary pane as a total count with a one-line explanation

The per-indicator placement is superior to a separate contradiction list because it
preserves context: the operator sees which specific setting is drifting, its current live
value, and what the config file says — all in one location. An assessor can record a
specific finding without scrolling to a separate section.

### A-6: Catalog baseline comparison addresses the staleness risk
**Control: NIST SP 800-53 CA-7 / CM-6**

`catalog_baseline_row()` compares the running kernel against `CATALOG_KERNEL_BASELINE`
("6.12.0") and surfaces one of three clearly worded outcomes:
- Running newer than baseline → advisory (yellow)
- Running matches baseline → confirmed (green)
- Running older than baseline → warning (red)

An assessor on a RHEL 10 kernel newer than 6.12.0 will immediately see that some
indicators may not reflect current kernel behavior, preventing false assurance. This is
a meaningful improvement over tools that ship a fixed indicator set with no currency signal.

The fail-open behavior on parse failure (dim, no false alarm) is correct: a kernel version
string that doesn't parse as `MAJOR.MINOR.PATCH` doesn't trigger a false red alert.

### A-7: Evidence chain satisfies SP 800-53A Examine structure
**Control: NIST SP 800-53 AU-3 / CA-7 / SA-11**

The Trust/Evidence tab provides all elements required for an SP 800-53A Examine object:
- `HeaderContext.assessed_at` — ISO-8601 timestamp of collection (CA-7)
- `HeaderContext.tool_name` + `tool_version` — tool identity and version (SA-11)
- `HeaderContext.hostname` — system under assessment (AU-3)
- `HeaderContext.boot_id` — boot instance correlation (CA-7)
- Evidence records with source type, path, and verification outcome including magic codes
  (`"✓ ok (fd, PROC_MAGIC)"`, `"✓ ok (fd, SYS_MAGIC)"`, `"✓ ok (fd, statfs)"`)
- Trust tier with plain-language description and numeric level
- Downgrade reasons enumerated individually
- Contradictions enumerated with source pair labels

An assessor can record all of this in an assessment finding without leaving the TUI.
The sticky evidence table header ensures column labels are always visible at any
scroll position.

### A-8: CMMC practice coverage is substantive for assessed controls
**CMMC practices covered:**

| Practice | Coverage |
|---|---|
| CM.L2-3.4.1 (baseline config) | Kernel Security tab shows live kernel config vs. baseline |
| CM.L2-3.4.2 (security config settings) | 27 indicators cover the primary RHEL 10 kernel hardening baseline |
| CM.L2-3.4.6 (least functionality) | MODULE RESTRICTIONS group: Bluetooth, USB storage, FireWire, Thunderbolt |
| SC.L2-3.13.10 (FIPS cryptography) | CRYPTOGRAPHIC POSTURE group; FIPS indicator calls CMMC SC.L2-3.13.10 explicitly in catalog |
| AC.L2-3.1.3 (CUI flow control) | FIPS + SELinux indicators visible in header on every frame |
| CA.L2-3.12.1 (periodic assessment) | Tool provides evidence records with timestamps; supports recurring CA-7 assessment cycle |
| AU.L2-3.3.1 (audit logging) | NETWORK AUDITING group (nf_conntrack); evidence records document evidence chain |
| SI.L2-3.14.2 (malicious code protection) | Boot integrity group (lockdown, kexec, module sig) directly supports this |
| MP.L2-3.8.7 (removable media) | USB storage blacklist indicator |

No single tool can satisfy all CMMC practices — this is a kernel posture assessment tool,
not a full CMMC assessment platform. The coverage above represents what this tool can
legitimately contribute to a CMMC assessment package.

### A-9: Group descriptions eliminate the need for an external reference
**Control: NIST SP 800-53 SA-5 / CM-6**

Every indicator group has a one-line plain-language description immediately below the
group title. Examples:
- BOOT INTEGRITY: "Verifies the kernel loaded in a tamper-resistant state and cannot be
  silently replaced at runtime."
- KERNEL SELF-PROTECTION: "Controls that hide kernel internals from unprivileged processes.
  Weak settings let attackers locate exploitable code and bypass ASLR."
- MODULE RESTRICTIONS: "Verifies high-risk kernel modules are blocked from loading. USB,
  FireWire, and Thunderbolt are primary data exfiltration and DMA attack vectors."

An operator who has never used this tool can read these descriptions and immediately
understand what each group is assessing.

### A-10: Recommended values are actionable for 27 of 27 displayed indicators
**Control: NIST SP 800-53 CM-6**

Every red (not hardened) indicator shows `[ Recommended: <value> ]`. Values are
operator-readable: `"2 (full ASLR)"`, `"blacklist bluetooth in modprobe.d"`,
`"remove mitigations=off from cmdline"`. No raw numeric-only recommendations exist.

`DesiredValue::Custom` indicators (`Sysrq`) correctly show `"0 (fully disabled)"` as the
recommended starting point while the catalog comment notes that bitmask semantics are
site-policy-dependent. This is appropriate — the recommendation is directional, not
prescriptive.

---

## CONCERN — Issues That Weaken Assessment Value

### C-1 (MEDIUM): Help text "Red rows require remediation before CUI processing" is too absolute
**Control: NIST SP 800-53 CM-6**
**File:** `umrs-tui/src/main.rs`
**Location:** Line 1891

This finding was first raised in v2 as C-7v2 and was not resolved before this review.
The statement causes two problems from an assessment perspective:

1. Some red indicators are configuration policy choices, not universal security failures.
   `Sysrq = DesiredValue::Custom` — the catalog itself acknowledges bitmask values are
   site-policy-dependent. An operator using a restricted `sysrq = 176` has a defensible
   policy. The help text would cause them to believe CUI processing is blocked.

2. Module blacklist indicators that are red because the module does not exist on the
   system (e.g., a RHEL server with no Bluetooth hardware) are informational absences.
   The display correctly shows "Not Present" and marks it red because the blacklist entry
   itself is not confirmed. The help text implies this is a blocking condition.

The text should guide priority review, not declare a gate. A finding written from this
help text could incorrectly block a legitimate CUI system.

**Recommended fix:** Replace line 1891 with:
`"Red rows do not meet the hardened baseline — review each indicator's description\n"`
`"             for risk context and remediation priority before CUI processing.\n"`
**Remediation owner:** coder
**SP 800-53 control:** CM-6 — configuration settings must be assessable in context

### C-2 (MEDIUM): IntegrityVerifiedButContradictory truncation silently discards finding details
**Control: NIST SP 800-53 SI-7 / AU-3**
**File:** `umrs-tui/src/main.rs`
**Location:** Line 1805 (`label_trust_display`)

This finding was raised in v2 as C-15v2. Still unresolved.

The string `.take(48)` is applied to the contradiction description. The formatted output
is `"IntegrityVerifiedButContradictory: <desc>"`. The prefix alone is 35 characters,
leaving at most 13 characters of the actual contradiction description visible. For a
T4-downgraded finding — the most security-sensitive trust state transition — an assessor
sees an incomplete contradiction without any indication that text was truncated. No
secondary row with the full text is emitted.

This is the highest-value trust finding the tool can surface (a system that reached T4
integrity verification but then contradicted itself) and its primary evidence string is
silently clipped.

**Recommended fix:** Emit a secondary `DataRow::key_value("", full_contradiction_text, StyleHint::TrustRed)`
row immediately following the Label Trust row when the variant is
`IntegrityVerifiedButContradictory`. This is consistent with how downgrade reasons and
contradiction entries are handled elsewhere in `build_trust_summary_rows`.
**Remediation owner:** coder
**SP 800-53 control:** AU-3 — audit records must be complete; SI-7 — integrity finding must be fully documented

### C-3 (LOW): T3 status bar color is blue (Info) while data panel renders T3 as green
**Control: NIST SP 800-53 AU-3**
**File:** `umrs-tui/src/main.rs`
**Location:** Line 1820 (`build_status`, `SubstrateAnchored` arm)

This finding was raised in v2 as C-T3-STATUS. Still unresolved.

`SubstrateAnchored` (T3) uses `StatusLevel::Info` (blue background) in the status bar,
but `trust_level_hint()` correctly returns `TrustGreen` for T3. The data panel renders
T3 trust rows in green. An assessor looking at the status bar and the data panel
simultaneously sees conflicting signals: green in the data area, blue in the status bar.

T3 is a positive trust outcome — the platform identity was verified and >= 2 independent
package facts were confirmed. It should not display as a neutral informational state.

**Recommended fix:** Line 1820: change `StatusLevel::Info` to `StatusLevel::Ok`.
**Remediation owner:** coder
**SP 800-53 control:** AU-3 — status display must be unambiguous

### C-4 (LOW): Recommendations tell operators WHAT to set but not HOW to set it
**Control: NIST SP 800-53 CM-6 / SA-5**
**File:** `umrs-tui/src/main.rs`
**Location:** `indicator_recommended()` function (~line 1235)

Guest-admin review (H2) raised this. Recommendations like `"1 (FIPS mode active)"` or
`"blacklist bluetooth in modprobe.d"` are directional but not immediately actionable
for a junior operator. An operator seeing `"2 (full ASLR)"` must still know that the
persistent fix is `echo "kernel.randomize_va_space = 2" >> /etc/sysctl.d/99-hardening.conf`
and not just `sysctl -w kernel.randomize_va_space=2` (which is ephemeral).

The current recommendations will not be misread or cause harm — they are technically
accurate. But they leave a gap between "what to set" and "how to make it survive reboot"
that is exactly the gap the `EphemeralHotfix` contradiction is designed to catch.

This is a LOW because the information needed to act is present — it just requires an
additional lookup. It becomes a MEDIUM if the tool is deployed without accompanying
operator documentation. The planned how-to guide expansion (see project notes) would
resolve this; this concern is a signal to prioritize that guide.

**Remediation owner:** tech-writer (operator how-to guide) / coder (optionally expand recommendation strings)
**SP 800-53 control:** SA-5 — inline documentation; CM-6 — remediation guidance

### C-5 (LOW): `nist_controls` field in catalog uses abbreviated `NIST 800-53` form
**Control: Citation Format Rule (project CLAUDE.md)**
**File:** `umrs-platform/src/posture/catalog.rs`
**Location:** All `nist_controls` fields (lines 135, 149, 163, 176, 189, 202, 216, 233,
247, 261, 275, 288, 301, 314, 327, 350, 363, 380, 394, 411, 424, 438, 455, 468, 483,
498, 512, 533, 547, 561, 574, 588, 602, 616, 630, 650, and more)

Every `nist_controls` field in the `INDICATORS` array uses `"NIST 800-53 ..."` (without
the `"SP"` qualifier). The Citation Format Rule requires `"NIST SP 800-53"` in doc
comments and source annotations.

The `IndicatorDescriptor` struct doc comment includes the relevant exemption language:
"Runtime output strings (e.g., `nist_controls` fields in catalog entries) may use
abbreviated forms for display compactness." This exemption applies here because
`nist_controls` is a runtime display string (it appears in the evidence chain and can
be used in JSON output). The abbreviation is therefore rule-compliant.

**However**, the `IndicatorDescriptor` struct-level doc comment cites `NIST SP 800-53`
in canonical form while the `INDICATORS` array content uses the abbreviated form. An
assessor reading the struct docs and then the array content will encounter inconsistent
citation formats within the same file. The exemption should be stated more explicitly
in the catalog module's `//!` doc block to make it clear to reviewers that the
abbreviation in data strings is intentional.

**Recommended fix:** Add to the `catalog.rs` `//!` module doc block's `## Compliance`
section: "Note: `nist_controls` field values use the abbreviated `NIST 800-53` form
(permitted for runtime display strings per project Citation Format Rule)."
**Remediation owner:** coder
**SP 800-53 control:** AU-3 — citation consistency supports audit traceability

---

## ERROR — Must Fix Before Release

None identified. The three items carried from v2 (C-15v2, C-7v2, C-T3-STATUS) are
tracked above as C-1, C-2, C-3 and remain CONCERN/LOW/MEDIUM — none rise to ERROR
because they do not cause incorrect findings, they degrade finding completeness or
introduce inconsistent visual signals.

No new ERROR-level issues were found in this review.

---

## Assessment Value Summary

### SP 800-53A Examine requirements

An SP 800-53A assessment requires the assessor to Examine objects (artifacts, mechanisms,
activities) and produce a finding for each control being assessed.

| Examine requirement | Status |
|---|---|
| Object identification (what system, when collected) | SATISFIED — header carries host, tool, version, timestamp, boot ID on every frame |
| Method documentation (how was the evidence collected) | SATISFIED — verification column shows magic codes; provenance statement in pinned summary |
| Evidence completeness | SATISFIIED with caveat — T4 contradiction truncation (C-2) reduces completeness for the highest-trust state only |
| Finding actionability (can an assessor record a finding from this output?) | SATISFIED — indicator + live value + description + contradiction kind + configured line provides all elements |
| Control traceability | PARTIAL — `nist_controls` visible in catalog but not surfaced in TUI display per indicator; assessor must consult catalog source |

The single gap in "control traceability" is not a blocker — the tool is an awareness
and assessment aid, not a compliance report generator. An assessor using this tool will
produce their own control mapping. Future enhancement: surfacing the applicable NIST
control(s) in the indicator row or detail view would elevate this from advisory to
direct evidence-to-control linkage.

### CMMC assessment contribution

The tool provides substantive evidence for CM.L2-3.4.2, CM.L2-3.4.6, SC.L2-3.13.10,
MP.L2-3.8.7, and SI.L2-3.14.2 — the kernel-hardening subset of CMMC Level 2.
It cannot substitute for a full CMMC assessment but provides documented, timestamped,
tool-versioned evidence for the assessed indicators. The evidence chain is sufficient to
attach to an assessment package as supporting artifacts.

---

## Gap Analysis Summary

**Files reviewed:** 8 (`main.rs`, `app.rs`, `data_panel.rs`, `indicators.rs`, `header.rs`,
`theme.rs`, `catalog.rs`, prior review reports v1 + v2 + guest-admin)

**Total findings:** 5 (0 ERROR, 2 MEDIUM, 3 LOW)

**ACCURATE items:** 10 major assessment-value strengths identified

**Uncited security claims:** None. All load-bearing security claims carry citations.

**Inconsistencies (code vs. docs):** None.

**Items carried from prior reviews (not yet resolved):**

| ID | From | Severity | Description | Owner |
|---|---|---|---|---|
| C-1 (was C-7v2) | v2 review | MEDIUM | Help text declares all red rows block CUI processing | coder |
| C-2 (was C-15v2) | v2 review | MEDIUM | IntegrityVerifiedButContradictory truncated to 13 usable chars | coder |
| C-3 (was C-T3-STATUS) | v2 review | LOW | T3 status bar blue while data panel renders T3 green | coder |

**New findings this review:**

| ID | Severity | Description | Owner |
|---|---|---|---|
| C-4 | LOW | Recommendations tell WHAT but not HOW (sysctl.d persistence gap) | tech-writer / coder |
| C-5 | LOW | `nist_controls` abbreviated form not documented as intentional in module doc | coder |

**Release readiness assessment:**

The tool is ready for release once C-1 and C-2 are resolved. C-1 risks causing operators
to block legitimate CUI work; C-2 silently discards the most security-sensitive trust
finding detail. C-3 through C-5 are post-release polish items.
