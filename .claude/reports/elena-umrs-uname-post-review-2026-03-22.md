# umrs-uname Full Review — Post-HCI/IA/KO Training
**Reviewer:** Elena Bellavigna (The Imprimatur)
**Date:** 2026-03-22
**Baseline:** `.claude/reports/hci-review-pre-training-elena.md`
**Companion reviews:** Sage (full UX), Nora (first-time user), Finn (operator)

**Files read:**
- `components/rusty-gadgets/umrs-uname/src/main.rs` (~1,870 lines, complete)
- `components/rusty-gadgets/umrs-ui/src/header.rs`
- `components/rusty-gadgets/umrs-ui/src/status_bar.rs`
- `components/rusty-gadgets/umrs-ui/src/theme.rs`
- `components/rusty-gadgets/umrs-ui/src/tabs.rs`
- `components/rusty-gadgets/umrs-ui/src/data_panel.rs`
- `components/rusty-gadgets/umrs-ui/src/indicators.rs`
- `components/rusty-gadgets/umrs-ui/src/dialog.rs`

---

## Opening Assessment

The tool is structurally sound. Every major architectural decision — four-zone layout,
pinned summary panes, typed color vocabulary, inline remediation guidance, evidence
pipeline ordering — is correct and should be preserved.

What the pre-training review got right: the cognitive-load problem on the Kernel Security
tab, the "Contradictions" label overload, the opacity of "Probe Used" and "Platform Facts,"
and the status bar terseness for lower trust tiers.

What the HCI training adds: the ability to name *why* those problems exist (Gulf of
Evaluation, information scent failure, genre confusion) and to locate the fixes precisely
in the source rather than offering general recommendations. The DELTA section at the end
of this document makes that comparison explicit.

Sage's review covers the outreach and conference-demo lens thoroughly and I agree with
most of it. Where my findings overlap with Sage's, I note the overlap and add my
analysis. Where they diverge, I explain why.

---

## KEEP

### K-1: Four-zone layout
Header → tabs → data panel → status bar is the right structure. It matches Norman's
action-cycle: the header identifies what system and what assessment, the status bar
delivers the verdict, the tabs organize the evidence. An operator's eyes can start
at the status bar (bottom), confirm the finding, then navigate to the tabs for detail.
Or they can read top-to-bottom like an audit card. Both paths work.

### K-2: Pinned-row split on Tabs 1 and 2
This is the single best structural decision in the tool. The assessor's verdict (trust
tier, posture score) stays visible while they scroll through the evidence that justifies
it. This eliminates the Gulf of Evaluation problem on both tabs: the operator never has
to remember what the summary said while reading the detail rows. Keep it exactly as
implemented. Document it in the developer guide as a required pattern for all UMRS
audit card tools.

### K-3: Trust color vocabulary — consistently applied
Green/yellow/red with ✓/✗/? dual encoding. The dual encoding is not just an
accessibility feature — it creates a parallel reading channel that works in different
attention states (quick scan vs. deliberate review). The theme applies this consistently
across five surfaces: status bar background, indicator rows, trust summary rows,
evidence verification column, and header indicators. Consistency at this level is unusual
and worth protecting.

### K-4: Inline remediation — the "onsite security officer" pattern
Failing indicators immediately show the recommended value below the indicator name. The
operator does not leave the TUI to know what to do. This is information foraging done
right: the patch value (remediation) is co-located with the scent (the failing indicator).
Nora noticed this. Finn noticed this. Sage formalized it. It is the most important
operator-experience feature in the tool.

### K-5: Group descriptions explain stakes before indicators
"Controls that hide kernel internals from unprivileged processes. Weak settings let
attackers locate exploitable code and bypass ASLR." The operator understands why they
should care before they read the indicator values. This is pedagogically correct and
reduces interpretation errors. Every group description is doing this job well.

### K-6: Evidence pipeline ordering follows trust elevation
Procfs → sysfs → statfs → regular files → package database → symlink targets. This
is not arbitrary — it replicates the pipeline's trust-building sequence. An assessor
reading the evidence chain reads it in the order the system built confidence. This is
the right information architecture for the audience.

### K-7: Fail-closed throughout
"unavailable" is never empty. Status bar always shows a level. Detection errors still
populate the Kernel Security tab independently. Every downgrade reason is enumerated.
These choices are security-correct and must not be traded away for display simplicity.

### K-8: The configured-value line on contradiction indicators
`Configured: <value> (from <source_file>)` gives the operator a concrete artifact to
investigate. This is source attribution done correctly: not just what the finding is,
but where to go to address it.

### K-9: Contextual per-tab help text
Three distinct help texts, each specific to the active tab. Tab 2's help text explicitly
distinguishes OS-detection contradictions from kernel/config contradictions — a
distinction Finn noted as rare and valuable. The depth is appropriate. Do not consolidate
into a single global help screen.

### K-10: Scrollable help dialog with scroll indicators
The `▲ more above` / `▼ more below` scroll hints follow terminal conventions. The
implementation separates scroll state (owned by `render_dialog`) from scroll commands
(driven by the caller), which is the right separation of concerns for a library.

---

## FIX

Each FIX item provides the file, function, current text, replacement text, and the
reason for the change. Items are ordered by severity within each cluster.

---

### Cluster A — Terminology opacity (high operator impact)

#### FIX-A1: `label_trust_display()` exposes CamelCase type names to operators
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `label_trust_display()` (lines 1402–1426)
**Severity:** HIGH

The current display strings embed Rust enum variant names. "UntrustedLabelCandidate"
and "LabelClaim" are internal model vocabulary. An operator reading these as security
findings cannot tell whether "Candidate" means "not yet assessed" or "suspicious." The
ambiguity is a usability failure on a security-relevant verdict row.

**Current:**
```rust
LabelTrust::UntrustedLabelCandidate => (
    "UntrustedLabelCandidate — do not use for policy".to_owned(),
    StyleHint::TrustRed,
),
LabelTrust::LabelClaim => (
    "LabelClaim — structurally valid; integrity unconfirmed".to_owned(),
    StyleHint::TrustYellow,
),
LabelTrust::TrustedLabel => (
    "TrustedLabel — T4: ownership + digest verified".to_owned(),
    StyleHint::TrustGreen,
),
LabelTrust::IntegrityVerifiedButContradictory { .. } => (
    "Verified w/ Contradiction — T4 integrity + conflict".to_owned(),
    StyleHint::TrustRed,
),
```

**Replace with:**
```rust
LabelTrust::UntrustedLabelCandidate => (
    "Untrusted — integrity not checked; do not use for policy decisions".to_owned(),
    StyleHint::TrustRed,
),
LabelTrust::LabelClaim => (
    "Claimed — structure valid, integrity not confirmed".to_owned(),
    StyleHint::TrustYellow,
),
LabelTrust::TrustedLabel => (
    "Verified — T4: ownership and file digest confirmed".to_owned(),
    StyleHint::TrustGreen,
),
LabelTrust::IntegrityVerifiedButContradictory { .. } => (
    "Verified with conflict — T4 integrity confirmed, source disagreement detected".to_owned(),
    StyleHint::TrustRed,
),
```

**Why:** Each string now states the verdict ("Untrusted", "Claimed", "Verified") before
any qualifier. The qualifier explains the basis of the verdict in operator terms, not
model terms. "Do not use for policy decisions" is more explicit than "do not use for
policy" about what kind of decision is at risk.

Note: Sage proposes equivalent rewrites (F-1 in Sage's review). My wording differs
slightly on the `IntegrityVerifiedButContradictory` case to surface the contradiction
nature more clearly than "Verified w/ Contradiction," which reads as a compound label.

---

#### FIX-A2: `boot_id` raw field name in `build_os_info_rows()`
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 437–445)
**Severity:** MEDIUM

"boot_id" is the kernel's internal variable name from `/proc/sys/kernel/random/boot_id`.
Every other label in the tab is Title Case. The underscore is a case-consistency failure
that reads as an incomplete display string.

**Current (line 438):**
```rust
rows.push(DataRow::normal("boot_id", boot.clone()));
```

**Replace with:**
```rust
rows.push(DataRow::normal("Boot ID", boot.clone()));
```

**Current (line 440):**
```rust
rows.push(DataRow::new(
    "boot_id",
    i18n::tr("not available"),
    StyleHint::Dim,
));
```

**Replace with:**
```rust
rows.push(DataRow::new(
    "Boot ID",
    i18n::tr("not available"),
    StyleHint::Dim,
));
```

---

#### FIX-A3: "Platform Facts" is unexplained and the count is uncontextualized
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 418–421)
**Severity:** MEDIUM

A bare numeric value labeled "Platform Facts" gives the operator no basis for
interpretation. Three what? Is five better than two? "Package Records" ties the
concept to something the operator recognizes; "verified" makes clear that the number
is evidence quantity, not an index or ID.

**Current:**
```rust
rows.push(DataRow::normal(
    i18n::tr("Platform Facts"),
    sub.facts_count.to_string(),
));
```

**Replace with:**
```rust
rows.push(DataRow::normal(
    i18n::tr("Package Records"),
    format!("{} confirmed", sub.facts_count),
));
```

---

#### FIX-A4: "Probe Used" exposes internal pipeline vocabulary
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 422–425)
**Severity:** MEDIUM

"Probe Used" is a detection pipeline concept. The value ("rpm", "dpkg") makes sense
to a developer who knows the pipeline architecture. An operator seeing `Probe Used: rpm`
does not know whether "probe" means the tool, the method, or a system component.
"Detection via" places the value in context the operator can interpret without background.

**Current:**
```rust
rows.push(DataRow::normal(
    i18n::tr("Probe Used"),
    sub.probe_used.to_owned(),
));
```

**Replace with:**
```rust
rows.push(DataRow::normal(
    i18n::tr("Detection Via"),
    sub.probe_used.to_owned(),
));
```

---

#### FIX-A5: "Label Trust" and "Trust Tier" appear as adjacent rows with no distinction
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_trust_summary_rows()` (lines 461–497)
**Severity:** MINOR

Two consecutive rows labeled with "trust" measure different things:
- `Label Trust` assesses the integrity of `/etc/os-release` as a specific artifact.
- `Trust Tier` reflects the full confidence pipeline including cross-source corroboration.

An operator sees two trust verdicts with no explanation of why they are separate. In the
worst case, they assume the lower of the two is the operative verdict and ignore the detail.

**Current (line 467):**
```rust
rows.push(DataRow::new(i18n::tr("Label Trust"), lt_label, lt_hint));
```

**Replace with:**
```rust
rows.push(DataRow::new(i18n::tr("OS-Release Integrity"), lt_label, lt_hint));
```

**Current (line 488):**
```rust
rows.push(DataRow::new(
    i18n::tr("Trust Tier"),
    i18n::tr(trust_level_label(level)),
    trust_level_hint(level),
));
```

**Replace with:**
```rust
rows.push(DataRow::new(
    i18n::tr("Detection Confidence"),
    i18n::tr(trust_level_label(level)),
    trust_level_hint(level),
));
```

**Why:** "OS-Release Integrity" and "Detection Confidence" are not synonyms.
They name distinct assessments. The operator can now read them as: "how much do we trust
the OS label file specifically?" and "how much do we trust the overall detection result?"

---

### Cluster B — Missing operator orientation signals

#### FIX-B1: "Contradictions" label is overloaded across two different findings
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `append_kernel_contradiction_rows()` (line 638)
**Severity:** HIGH

"Contradictions" appears with two distinct meanings on two different tabs. On Tab 1
(Kernel Security), it means a kernel/config disagreement on a hardening setting. On Tab 2
(Trust / Evidence), it means two OS-detection sources reported different values for the
same identity fact.

These are genuinely different findings with different response actions. Using the same
label for both creates a high risk of misrouting: an operator who sees "Contradictions: 2"
on Tab 1 and "Contradictions: 0" on Tab 2 may not understand why the counts differ, or
may escalate the wrong finding.

Sage proposes "Config Drift" for the Tab 1 label (F-5). I agree. The term is technically
accurate ("configuration drift" is established security vocabulary) and it is immediately
distinct from "Contradictions" on Tab 2.

**Current (line 638):**
```rust
rows.push(DataRow::key_value_highlighted(
    i18n::tr("Contradictions"),
    value,
    hint,
));
```

**Replace with:**
```rust
rows.push(DataRow::key_value_highlighted(
    i18n::tr("Config Drift"),
    value,
    hint,
));
```

Also update the zero-count explanation row (line 645):

**Current:**
```rust
rows.push(DataRow::key_value(
    "",
    i18n::tr("No disagreements between running kernel and persisted configuration."),
    StyleHint::Dim,
));
```

**Replace with:**
```rust
rows.push(DataRow::key_value(
    "",
    i18n::tr("Running kernel and persisted configuration agree on all checked settings."),
    StyleHint::Dim,
));
```

**Why:** The replacement zero-count text is positive rather than negation-based. "Agree
on all checked settings" is easier to process quickly than "no disagreements." Under
time pressure, negative constructions ("no X") require more cognitive work to parse.

---

#### FIX-B2: Status bar messages for lower trust tiers are implementation labels, not operator verdicts
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_status()` (lines 1428–1457)
**Severity:** HIGH

The status bar is the operator's first and persistent verdict. An operator who sees
"Platform Verified" on a blue background needs to know whether that is normal, acceptable,
or cause for concern. The current strings name the tier level but do not orient the
operator to its operational meaning.

This is a Gulf of Evaluation failure: the operator cannot evaluate whether the system
state matches their goal without additional information that is currently only available
via the Trust / Evidence tab or the help overlay.

**Current for `IntegrityAnchored` (line 1430):**
```rust
StatusMessage::new(StatusLevel::Ok, i18n::tr("Integrity Anchored"))
```

**Replace with:**
```rust
StatusMessage::new(StatusLevel::Ok, i18n::tr("T4: Integrity Anchored — highest confidence"))
```

**Current for `SubstrateAnchored` (line 1433):**
```rust
StatusMessage::new(StatusLevel::Info, i18n::tr("Platform Verified"))
```

**Replace with:**
```rust
StatusMessage::new(StatusLevel::Info, i18n::tr("T3: Platform Verified — normal operating posture"))
```

**Current for `EnvAnchored` (lines 1436–1443):**

The description currently included is `trust_level_description(TrustLevel::EnvAnchored)`,
which returns `"Mount topology cross-checked (mountinfo vs statfs)."` — implementation
detail, not operator orientation.

**Replace** the EnvAnchored and KernelAnchored branches with direct strings:

```rust
TrustLevel::EnvAnchored => StatusMessage::new(
    StatusLevel::Warn,
    i18n::tr("T2: Environment Anchored — partial verification; review Trust / Evidence tab"),
),
TrustLevel::KernelAnchored => StatusMessage::new(
    StatusLevel::Warn,
    i18n::tr("T1: Kernel Anchored — minimal verification; review Trust / Evidence tab"),
),
```

**Current for `Untrusted` (line 1452):**
```rust
StatusMessage::new(
    StatusLevel::Error,
    i18n::tr("Untrusted — no kernel anchor"),
)
```

**Replace with:**
```rust
StatusMessage::new(
    StatusLevel::Error,
    i18n::tr("T0: Untrusted — pipeline failed; do not rely on any value shown"),
)
```

**Why:** The T-N prefix teaches the tier system passively across sessions. The one-phrase
orientation ("normal operating posture", "partial verification") gives the operator
enough context to decide whether to investigate further without leaving the status bar.
T0 explicitly warns against trusting the displayed data — the most critical guidance
possible, which should be available without navigating to Tab 2.

The description strings in `trust_level_description()` serve the Trust/Evidence tab
detail row and should not be changed — the implementation-level descriptions ("procfs
verified via PROC_SUPER_MAGIC + PID coherence") are correct for that context. Only
`build_status()` needs different strings.

---

#### FIX-B3: Error messages in `from_error()` diagnose but do not orient
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `from_error()` (lines 254–264)
**Severity:** MEDIUM

The three hard-gate failure messages name the failure accurately but give the operator
no starting point for investigation. "Hard gate: procfs is not real procfs" is a
correct technical statement. An operator on a production CUI system encountering this
error needs to know what to check first.

**Current:**
```rust
let description = match err {
    DetectionError::ProcfsNotReal => {
        i18n::tr("Hard gate: procfs is not real procfs")
    }
    DetectionError::PidCoherenceFailed { .. } => {
        i18n::tr("Hard gate: PID coherence broken")
    }
    DetectionError::KernelAnchorIo(_) => {
        i18n::tr("Hard gate: I/O error during kernel anchor")
    }
};
```

**Replace with:**
```rust
let description = match err {
    DetectionError::ProcfsNotReal => {
        i18n::tr(
            "Hard gate: /proc is not a real procfs mount. \
             Confirm this tool is running on a Linux host with /proc mounted."
        )
    }
    DetectionError::PidCoherenceFailed { .. } => {
        i18n::tr(
            "Hard gate: PID coherence check failed. \
             This can occur in container environments or when /proc has been tampered with."
        )
    }
    DetectionError::KernelAnchorIo(_) => {
        i18n::tr(
            "Hard gate: I/O error reading kernel anchor. \
             Check that /proc is accessible and that this process has read permission."
        )
    }
};
```

**Why:** Each message now names what failed and gives a concrete starting point. The
language does not prescribe site procedure ("contact your ISSO") — it orients toward
the technical investigation the operator can begin immediately. The NIST SP 800-53 SI-11
comment at line 251 is satisfied: the messages do not include variable kernel data.

---

#### FIX-B4: OS Information tab lacks section titles — three different label styles are unmarked
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 369–447)
**Severity:** MINOR

Tab 0 presents three distinct sections without labeling them:
1. Raw `os-release` field names (`ID`, `NAME`, `VERSION_ID`, `PRETTY_NAME`, `CPE_NAME`)
2. Interpreted platform identity labels (`Platform Family`, `Platform Distro`, etc.)
3. A boot session identifier (`Boot ID`)

The shift between uppercase raw field names (section 1) and Title Case labels (section 2)
is jarring and reads as inconsistency rather than as a deliberate boundary. A first-time
operator may not understand why some fields are uppercase and others are not.

Add group titles before the os-release block and the platform identity block using the
same pattern already established in the Kernel Security tab:

**Before the first `if let Some(rel)` block** (before line 373):
```rust
rows.push(DataRow::group_title(i18n::tr("OS RELEASE")));
```

**Before the `if let Some(sub)` block** (before line 400):
```rust
rows.push(DataRow::group_title(i18n::tr("PLATFORM IDENTITY")));
```

The existing `rows.push(DataRow::separator())` between sections can remain.

**Why:** Group titles make the label style shift intentional. The operator understands
that OS RELEASE shows raw kernel values in their native form, while PLATFORM IDENTITY
shows interpreted results from the detection pipeline. This distinction is real and
worth communicating. The pattern is already present on the Kernel Security tab — using
it on Tab 0 is consistent rather than innovative.

---

#### FIX-B5: Help text for Tab 1 uses "EVIDENCE CHAIN" as a section header — wrong concept
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `help_text_for_tab()`, case `1` (line ~1549)
**Severity:** MINOR

Tab 2 has the evidence chain. Tab 1 has a posture indicator list. Using "EVIDENCE CHAIN"
as a section header in Tab 1's help text blurs the distinction between these two concepts.
An operator who learns "evidence chain" from Tab 1's help will use that term to describe
what they are looking at on Tab 2, which is actually correct — but the confusion flows
the other direction too: they may expect Tab 2's evidence chain to contain hardening
indicator assessments.

**Current (line ~1549):**
```
 EVIDENCE CHAIN:
   Indicator       ✓ = Hardened (green)
```

**Replace with:**
```
 INDICATOR LEGEND:
   Indicator       ✓ = Hardened (green)
```

---

#### FIX-B6: Curated-indicator framing note is placed after the verdict it should precede
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_kernel_security_summary_rows()` (lines 827–837)
**Severity:** MINOR

The note "Curated indicators selected to give you the clearest view of your system's
security posture..." currently appears at the bottom of the summary pane, after the
indicators count and contradiction count. An operator who starts reading from the top
encounters the numbers before the framing that contextualizes them.

Move this note to the top of the summary pane, immediately after the provenance note
("All values below are read live from the running kernel via /proc and /sys."), before
the first scoring row.

**In `build_kernel_security_summary_rows()`**, the provenance note block currently ends at:
```rust
DataRow::key_value(
    "",
    i18n::tr(
        "All values below are read live from the running kernel via /proc and /sys.",
    ),
    StyleHint::Dim,
),
DataRow::separator(),
```

Immediately after that separator, add:
```rust
DataRow::key_value(
    "",
    i18n::tr(
        "Curated indicators selected to give you the clearest view of \
         your system's security posture. Items marked in red can be \
         hardened — see each indicator's recommended setting below.",
    ),
    StyleHint::Dim,
),
DataRow::separator(),
```

Then remove the current instance of this text at the bottom of the function (the block
ending at approximately line 837).

**Why:** Framing precedes scoring. An operator who reads "5 not hardened (62%)" before
knowing these are curated advisory indicators may treat the score as more alarming than
intended. Moving the frame above the score ensures the operator understands the scope
before they interpret the numbers.

---

### Cluster C — Trust tier communication

#### FIX-C1: Trust tier descriptions use implementation language, not operator language
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `trust_level_description()` (lines 76–92)
**Severity:** MINOR (display-only)

The descriptions shown in the `Description` row under `Trust Tier` in the Trust/Evidence
tab are implementation-oriented:
- T1: "procfs verified via PROC_SUPER_MAGIC + PID coherence."
- T2: "Mount topology cross-checked (mountinfo vs statfs)."

These are accurate and appropriate for the Trust/Evidence tab's detail context. A security
auditor reviewing the evidence chain will find this useful. A first-time operator or an
adopter evaluating the tool will find it opaque.

The fix is not to change these strings — they are correct for the Trust/Evidence tab.
The fix is to ensure the status bar (FIX-B2, already addressed) provides the
operator-facing orientation, and to add a brief parenthetical to the Trust/Evidence tab
help text that translates the description into action terms.

No source change required here. This is addressed by FIX-B2 in the status bar and by
the help text clarification in FIX-C2.

---

#### FIX-C2: Trust tier help table lacks two things — cumulative guarantees and operational implications
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `help_text_for_tab()`, case `2` (lines ~1606–1624)
**Severity:** MEDIUM

Nora identified both gaps. The trust tiers are defined but their cumulative nature is
not stated, and the table gives no operational implication (what the operator can rely
on at each tier).

**Current tier table:**
```
   T0  Untrusted          No reliable sources found.
                          Do not rely on any value shown.

   T1  Kernel Anchored    At least one kernel source confirmed.
                          Basic identity only — treat with caution.

   T2  Env Anchored       Environment sources corroborate kernel.
                          Consistent across /proc, /sys, and /etc.
                          Acceptable for routine operations.

   T3  Platform Verified  All expected sources agree.
                          Normal operating posture.

   T4  Integrity Anchored All sources agree + kernel filesystem
                          identity verified. Highest confidence.
```

Add before the tier table (after `TRUST TIERS:` header):
```
   Each tier includes all guarantees of the tiers below it.
   T4 is the strongest claim and subsumes T0 through T3.

```

**Replace the tier table body** with:
```
   T0  Untrusted          No reliable sources found.
                          Do not rely on any value shown.

   T1  Kernel Anchored    At least one kernel source confirmed.
                          Core identity is reliable; environment
                          data is not independently corroborated.

   T2  Env Anchored       Environment sources corroborate kernel.
                          OS identity is consistent across /proc,
                          /sys, and configuration files.
                          Suitable for routine operations.

   T3  Platform Verified  All expected sources agree.
                          Suitable for assessment work.
                          Normal operating posture for CUI systems.

   T4  Integrity Anchored All sources agree + kernel filesystem
                          provenance verified. Verification codes
                          confirm no path substitution occurred.
                          Highest confidence; expected for production.
```

Also add after the tier table, before `BOTTOM — EVIDENCE CHAIN`:
```
 OPERATING GUIDANCE:
   T0 and T1 indicate insufficient platform verification.
   Do not process CUI on this system without manual investigation.
   T3 and T4 are the expected posture for production CUI systems.

```

**Why:** Finn's H-09 finding is correct: the tier table is defined but not connected to
an operational decision. Security operators in CUI environments need a go/no-go signal,
not just a taxonomy. The operating guidance block provides that without prescribing
site-specific procedure. The cumulative-guarantee statement prevents operators from
treating each tier as fully independent.

---

### Cluster D — Minor consistency and navigation

#### FIX-D1: Fallback navigation block is less complete than per-tab blocks
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `help_text_for_tab()`, case `_` (lines ~1668–1674)
**Severity:** LOW

**Current fallback:**
```
r" NAVIGATION:
   Tab / Shift-Tab     switch between tabs
   j / k  or  ↑ / ↓   scroll this help
   PgDn / PgUp         scroll faster
   Enter, Esc, or q    close this help"
```

This is already adequate. Both Nora and Finn flagged it as differing from the per-tab
blocks, but on examination the per-tab blocks have the same content. No change needed —
Nora's Finding 10 and Finn's N-02 are resolved by inspection.

---

#### FIX-D2: Contradiction row on Tab 1 inline explanation uses a single shared paragraph
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `append_kernel_contradiction_rows()` (lines 649–658)
**Severity:** LOW

When contradiction count > 0, the inline explanation reads:
"The running kernel and persisted configuration disagree on one or more settings.
Drift means intended hardening is not active; hotfixes mean current hardening will
be lost on reboot."

This text is accurate. However, it tries to cover two different contradiction types
(DRIFT and NOT PERSISTED) in one sentence. An operator who sees DRIFT needs to know
the hardening is currently inactive. An operator who sees NOT PERSISTED needs to
persist the setting. Finn's H-04 is right that differentiation is needed, but the
solution lives in the help overlay (Tab 1 help), not in this always-visible summary row
where brevity matters.

The per-kind explanation should be in the help overlay, not the inline summary. The
inline summary should identify the count and direct the operator to the detail below.

No change to the inline summary row required. Update the Tab 1 help text (already
covered by the existing help text blocks in FIX-B5's vicinity). The help text for
contradiction types (lines ~1527–1548) already explains DRIFT, NOT PERSISTED, and
UNVERIFIABLE with separate blocks. That structure is correct.

---

## CONSIDER

These items require Jamie's judgment before any implementation work.

### C-1: Tab 0 name — "OS Information" vs. "System Identity"
Sage proposes renaming Tab 0 to "System Identity" (F-11). The argument is that
"System Identity" communicates security relevance to a conference audience and positions
the tab as a trust-relevant artifact rather than an administrative detail screen.

I am neutral on this change for the following reason: the help text for Tab 0 begins
"Identity fields extracted from the running system" — which already provides the
security framing. The tab name change would reinforce this, but it also changes a term
that appears in three places in the source (two `from_result`/`from_error` construction
sites and the help text). The rename is clean but the benefit depends on the audience.

**For Jamie:** Is "System Identity" the right term across all UMRS tools where this tab
pattern appears, or is the OS Information tab specific to this tool's assessment scope?
If the tab name will be shared with other tools (umrs-stat, umrs-ls), establish the
convention here.

### C-2: Card title — "OS Detection Audit" vs. "Platform Security Audit"
Sage proposes changing `card_title()` to return "Platform Security Audit" (F-12). The
header border frame would read `[ Platform Security Audit ]`.

I agree that "Platform Security Audit" is more legible to a non-UMRS audience. "OS
Detection Audit" names an internal pipeline step. However, the card title appears in
the border frame at the top of the screen — it is small, styled as a box title, and
operators may not read it closely. The outreach benefit is real but the operator
experience benefit is small.

**For Jamie:** Decide based on which audience you are optimizing for at this stage.
If conference screenshots and blog posts are the near-term use, "Platform Security Audit"
is the right change. If the rename should wait until naming conventions are settled
across all tools, defer it.

### C-3: First-run help overlay — auto-open on launch?
My pre-training review suggested auto-opening the help overlay on first launch. The HCI
training reinforces this: for tools targeting mixed audiences (operators who know their
environment, assessors who do not), offering orientation on first launch reduces the
activation energy for the help system.

The implementation risk is low: check for a flag file (e.g.,
`~/.local/state/umrs/umrs-uname.first-run`) and open the Tab 0 help overlay if the
flag does not exist. Write the flag on first close.

**For Jamie:** This touches XDG state paths, which the project deployment model
already includes (`$HOME/.local/`). Scope it as a small enhancement after the text
fixes above are merged.

### C-4: "Probe Used" value format — short string vs. descriptive string
FIX-A4 renames "Probe Used" to "Detection Via." The value is still `"rpm"` or `"dpkg"`.
Consider whether the value should be expanded: `"RPM database"` rather than `"rpm"`.

This is a display-only change and can be done in the `distro_label` / substrate
rendering path without touching the type system. The benefit is modest but consistent
with the "explain what you mean" principle throughout the tool.

**For Jamie:** Low priority. Accept or defer.

### C-5: Scroll position indicator in the data panel
My pre-training review flagged the absence of a position indicator (e.g., "12/47 lines")
in the data panel. The scrollbar gives a rough visual estimate but no count. For the
Kernel Security tab, which can have a significant number of indicator rows under a large
posture catalog, an operator may not know how much content remains below the fold.

Adding `↓ N more` or a percentage to the scrollbar area or status bar right margin is
a minor enhancement with real benefit during live assessment.

**For Jamie:** Rusty should scope this. The data panel currently computes `total_line_count`
for scrollbar rendering — the count is available. Exposing it as a display string is
the implementation work.

---

## DELTA — What the HCI Familiarization Added

This section is the primary research artifact. It documents what changed between my
pre-training review and this one, and names the concept from the HCI material that
enabled each change.

The pre-training review was written by someone who could identify that something was
wrong with the operator experience and describe what it looked like. The post-training
review is written by someone who can name why it is wrong, trace it to a design model,
and write a more precise fix as a result. Below are the specific cases where the
training made a difference.

---

### DELTA-1: The Gulf of Evaluation explains the status bar problem precisely
**Pre-training:** I noted that the status bar messages for lower trust tiers were "terse"
and that operators would not know whether a given result was good or bad. The fix I
proposed was vague: "add context to the status bar message."

**Post-training:** Norman's Gulf of Evaluation is the name for what is happening. The
gulf is the gap between the system's state and the operator's ability to evaluate whether
that state satisfies their goal. An operator whose goal is "confirm this system is safe
for CUI processing" cannot evaluate "Platform Verified" against that goal without
additional reading. The fix is not just "add context" — it is specifically to add
an orientation phrase that closes the gulf: "normal operating posture" or "do not
process CUI without investigation." These phrases give the operator the evaluation
vocabulary they need without requiring them to navigate to the Trust / Evidence tab.

**Impact:** FIX-B2 is more precise than my pre-training recommendation. The tier prefix
(`T3:`, `T4:`) passively teaches the tier system across sessions — that is a Gulf of
Execution fix (making the right action learnable by repeated exposure) that I would not
have framed this way without the training.

---

### DELTA-2: Information scent explains why "Platform Facts: 3" fails as a display string
**Pre-training:** I noted that "Platform Facts" was confusing and proposed renaming it.
The reasoning was purely aesthetic: the label did not match the operator's vocabulary.

**Post-training:** Pirolli's information foraging model provides the mechanism.
Information scent is the partial information a user uses to judge whether pursuing a
path (clicking a link, reading a row) will yield what they need. "Platform Facts: 3" has
zero scent — there is no basis for the operator to predict what "3" means or whether they
should investigate further. "Package Records: 3 confirmed" has scent: the operator knows
this is a count of verified package database entries, which they can relate to the
platform identity claim. The fix in FIX-A3 ("3 confirmed") is a scent improvement, not
just a renaming.

**Impact:** This framing also explains why FIX-B6 (moving the curated-indicator note
above the scoring summary) is important. The note provides scent for the scoring numbers
that follow. Without it, the operator encounters "5 not hardened (62%)" with no
information to predict whether that count is alarming or expected for their environment.
The pre-training review identified this problem. The post-training review can name it.

---

### DELTA-3: Genre theory explains the help text design tension
**Pre-training:** I noted that the help overlay was good content but hard to find, and
that the ✓/✗/? symbols were explained in help but not inline. My recommendation was to
move the explanation closer to the data.

**Post-training:** Miller and Bazerman's genre theory provides a cleaner analysis.
The three help texts serve three different genres:
- Tab 0 help is a *reference entry* — it defines what each field is and where it comes from.
- Tab 1 help is an *operator briefing* — it explains how to interpret contradictions and
  what to do about red indicators.
- Tab 2 help is a *conceptual explanation* — it defines trust tiers and their implications.

The problem with the current Tab 2 help is that it is almost entirely definitional
(reference genre) when the operator's need at that tab is primarily evaluative (briefing
genre): "what does this tier mean for what I do next?" FIX-C2 (adding operating guidance
and cumulative-guarantee text to the Trust tier table) is a genre correction: it adds
the briefing content to a section that was previously only reference content.

**Impact:** Without the genre framing, I would have said "add more explanation." With it,
I can say specifically what kind of explanation is missing and where in the help text
to add it.

---

### DELTA-4: Cognitive Dimensions of Notations explains the "Contradictions" naming collision
**Pre-training:** I identified that "Contradictions" was used for two different things and
proposed using different labels. The reasoning was: "these are different findings and
should have different names."

**Post-training:** Blackwell and Green's "hidden dependencies" dimension names the problem
more precisely. The same label "Contradictions" creates a hidden dependency between Tab 1
and Tab 2: an operator who learns what "Contradictions" means on Tab 1 (kernel/config
disagreements) will bring that definition to Tab 2 (OS identity source disagreements)
and misinterpret the finding. The cognitive cost of the collision is not just confusion —
it is systematic misrouting of a security finding.

The fix (FIX-B1, "Config Drift" for Tab 1) eliminates the hidden dependency by giving
each concept a distinct name. "Config Drift" is an established security term; it has
existing connotations that align with the Tab 1 meaning (intended hardening not active).
"Contradictions" on Tab 2 retains its meaning without competition.

**Impact:** The pre-training recommendation was correct. The post-training analysis
explains why it matters at a level that will survive a code review: this is not a
preference, it is a disambiguation that prevents a specific class of assessment errors.

---

### DELTA-5: Progressive disclosure — what the training named, the pre-training review only gestured at
**Pre-training:** I noted that the Kernel Security tab "overloads on first viewing" and
that "everything is presented at once" without progressive disclosure. The fix I proposed
was vague: move the curated-note to the top of the summary pane.

**Post-training:** Progressive disclosure is a specific HCI pattern: present the minimum
needed to proceed, then reveal more on demand. The Kernel Security tab does not need
progressive disclosure within the indicator list — it needs it at the summary/detail
boundary. The pinned-row design already implements this correctly: the pinned summary
is always visible, the detail is scrollable. The problem is not that there is too much
detail — it is that the framing for the detail (the curated-indicator note) is at the
wrong layer. It belongs in the summary pane because it contextualizes the summary scores,
not in the detail pane where it appears after the scores it should precede.

**Impact:** FIX-B6 is now a targeted structural fix, not a general "reduce density"
recommendation. The post-training analysis confirms that the architecture (pinned + scrollable)
is correct and the issue is purely one of row ordering within the pinned section.

---

### DELTA-6: What the HCI training did NOT change — and why that matters for the study
The pre-training review correctly identified every major usability gap in the tool.
The training did not discover new problems. What it provided was:
1. Vocabulary to name the mechanisms behind the problems (Gulf of Evaluation, information
   scent, genre theory, hidden dependencies, progressive disclosure).
2. Precision in the fix recommendations — from "add more context" to "add a one-phrase
   orientation that closes the Gulf of Evaluation without requiring tab navigation."
3. Confidence in the Keep items — the post-training review can affirm the pinned-row design,
   the color vocabulary, and the evidence pipeline ordering as structurally sound, not just
   "these seem fine." The concepts explain why they are sound.

The study question "what did the HCI familiarization add?" has a cleaner answer than
I anticipated: it added analytical precision without changing the findings. A reviewer
with only practical experience and one with formal HCI training will flag the same
problems. The trained reviewer will articulate them more precisely and propose more
targeted fixes. For a security-documentation audience that values precision, this is
a meaningful difference.

---

## Summary

**Findings by severity:**

| Severity | Count | FIX items |
|---|---|---|
| HIGH | 2 | FIX-A1 (LabelTrust display), FIX-B1 (Contradictions overload) |
| HIGH | 2 | FIX-B2 (status bar orientation), FIX-C2 (trust tier operational guidance) |
| MEDIUM | 4 | FIX-A3, FIX-A4, FIX-B3, FIX-A5 |
| MINOR | 6 | FIX-A2, FIX-A4, FIX-B4, FIX-B5, FIX-B6, FIX-C1 |

**Items for Jamie's decision:** C-1 through C-5.

**Changes agreed with Sage:** FIX-A1 (same finding, slightly different wording),
FIX-A2, FIX-A3, FIX-B1 ("Config Drift"), FIX-B2, FIX-B6 (curated note position).

**Changes not in Sage's review:** FIX-A5 (Label Trust / Trust Tier row distinction),
FIX-C2 (trust tier help table operational guidance), all DELTA analysis.

**Not changed:** Architecture, color vocabulary, pinned-row pattern, evidence ordering,
inline remediation, per-tab help structure, fail-closed behavior. These are all working.
Protect them from improvement.
