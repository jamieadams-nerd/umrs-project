# umrs-uname Full UX Review
**Reviewer:** Sage (Savannah) — The Imodium Operator
**Date:** 2026-03-22
**Scope:** Complete tool review — layout, visual hierarchy, information architecture, operator
experience, text, outreach lens, and conference/screenshot appeal.

**Sources read:**
- `components/rusty-gadgets/umrs-uname/src/main.rs` (full, ~1,850 lines)
- `components/rusty-gadgets/umrs-ui/src/header.rs`
- `components/rusty-gadgets/umrs-ui/src/status_bar.rs`
- `components/rusty-gadgets/umrs-ui/src/theme.rs`
- `components/rusty-gadgets/umrs-ui/src/tabs.rs`
- `components/rusty-gadgets/umrs-ui/src/data_panel.rs`
- `components/rusty-gadgets/umrs-ui/src/indicators.rs`
- `components/rusty-gadgets/umrs-ui/src/dialog.rs`
- Previous review: `.claude/reports/hci-review-pre-training-sage.md` (my pre-training baseline)
- Intern reviews: finn and nora help-text reviews
- Memory: `project_help_text_philosophy.md`, `project_intern_review_insights.md`

**Note on the pre-training baseline:** This review supersedes
`.claude/reports/hci-review-pre-training-sage.md`. That file is the baseline. This is the
considered judgment after reading the full implementation. Where I previously flagged something
and now see the code handles it well, I say so. Where findings harden into concrete fixes, I
provide exact replacement text.

---

## Executive Summary

umrs-uname is doing most things right. The structural decisions — four-zone layout, pinned
summary panes, trust color vocabulary, inline descriptions, evidence chain ordering — are all
sound. The tool's engineering identity is coherent and the security rationale is woven through
the implementation in a way that is rare and worth preserving.

The remaining gaps fall into two clusters:

**Cluster 1 — Terminology opacity.** A handful of display strings are internal vocabulary
(CamelCase type names, raw field names, UMRS-model-specific labels) that will confuse first-run
operators and look rough in conference demos. These are isolated, fixable, and do not touch
architecture.

**Cluster 2 — Missing operator orientation signals.** The tool presents findings well but
leaves the operator to discover how to interpret them. Status bar messages, contradiction labels,
and curated-indicator framing can all do more work without adding complexity.

Fixes in this review are surgical. I am not proposing architectural changes. I am proposing
text changes and two small label changes that will make the tool significantly more legible to
the audiences that matter.

---

## KEEP — Things Working Well (Do Not Improve Away)

### K-1: The four-zone layout is correct
Header → tabs → data panel → status bar. This mirrors the physical audit card mental model.
Every audience — security engineers, program managers, Five Eyes reviewers — will parse it
immediately. The layout is not trying to be clever. It is trying to be clear. It succeeds.

### K-2: The pinned-row design is the best thing in the tool
Tabs 1 and 2 keep the top-level verdict always visible while the operator scrolls through
detail. This is the correct answer to the "where do I look first" problem. Every future
UMRS tool should replicate this pattern. Document it explicitly in the developer guide.

### K-3: The trust color vocabulary is consistent and earned
Green/yellow/red for trust levels, applied consistently across status bar, trust summary,
indicator rows, header indicators, and evidence chain. The dual encoding with ✓/✗/? symbols
in indicator rows is the right accessibility choice. Do not change the color semantics.

### K-4: Inline remediation guidance on red indicators
When an indicator fails, the recommended setting appears immediately below it. The operator
does not leave the tool to know what to fix. This is the onsite security officer model in
practice. Keep it exactly as implemented.

### K-5: Group descriptions that explain WHY before the indicators
Every indicator group has a plain-language sentence explaining what the group is protecting
against. "Controls that hide kernel internals from unprivileged processes. Weak settings let
attackers locate exploitable code and bypass ASLR." This is pedagogically correct. An
operator encountering KERNEL SELF-PROTECTION for the first time understands the stakes
before they start reading the indicators. Keep all of these verbatim.

### K-6: Evidence pipeline ordering follows trust elevation
Kernel runtime (/proc) → kernel attributes (/sys) → filesystem identity → config files →
package database → symlink targets. This ordering is not just organizational preference —
it follows the trust-elevation narrative. An assessor reading the evidence chain reads it
in the same order the pipeline built the trust case. This is good information architecture.

### K-7: The fail-closed behavior throughout
`"unavailable"` is never empty. Every read failure surfaces explicitly. The status bar always
shows a level. Detection errors still populate the Kernel Security tab from the posture snapshot.
These are all correct security-first choices. Do not trade any of them for cleanliness.

### K-8: The configured-value line on contradictions
When a kernel setting contradicts the persisted config, the tool shows `Configured: <value>
(from <source_file>)`. The operator knows where to look. This is exactly right.

### K-9: The help overlay is contextual per tab
Three different help texts, each specific to the current tab. This is right. An operator
pressing `?` on the Trust/Evidence tab gets trust tier definitions, not boot integrity
explanations. The depth of the help text is appropriate for the audience.

### K-10: The status bar key legend is present from frame zero
New operators can discover navigation without pressing `?`. The legend is compact and does
not crowd the status message. This is correct.

---

## FIX — Changes Required, With Exact Replacement Text

### F-1: LabelTrust display strings use CamelCase type names
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `label_trust_display()` (line 1402)
**Severity:** HIGH — these are operator-facing strings. CamelCase enum variant names are
developer vocabulary, not operator vocabulary.

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
```
(Lines 1404–1415, approximately.)

**Replace with:**
```rust
LabelTrust::UntrustedLabelCandidate => (
    "Untrusted — integrity not checked; do not use for policy".to_owned(),
    StyleHint::TrustRed,
),
LabelTrust::LabelClaim => (
    "Claimed — structure valid, integrity not confirmed".to_owned(),
    StyleHint::TrustYellow,
),
LabelTrust::TrustedLabel => (
    "Verified — T4: ownership and digest confirmed".to_owned(),
    StyleHint::TrustGreen,
),
```

**Why:** An operator seeing `UntrustedLabelCandidate` does not know whether "candidate" means
"we have not checked yet" or "something is wrong." "Untrusted" is the verdict. The plain
English string delivers the verdict without requiring knowledge of the UMRS type system.

---

### F-2: `boot_id` field name leaks internal naming convention
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 437–445)
**Severity:** MEDIUM — inconsistent with every other label in the tab; looks like a
copy-paste from code.

**Current (line 438):**
```rust
rows.push(DataRow::normal("boot_id", boot.clone()));
```

**Replace with:**
```rust
rows.push(DataRow::normal("Boot ID", boot.clone()));
```

And the fallback row (line 440):
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

**Why:** Every other label in the OS Information tab is Title Case. `boot_id` is the kernel's
internal variable name. A conference demo screenshot with `boot_id` next to `Platform Family`
looks like a bug.

---

### F-3: "Platform Facts" count is unexplained and confusing
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 418–421)
**Severity:** MEDIUM — operators will not know whether a higher or lower count is better,
or what a "fact" is.

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
    format!("{} verified", sub.facts_count),
));
```

**Why:** "Platform Facts" is an internal model concept. "Package Records" is what the
operator can relate to — the package database entries the pipeline cross-checked.
"5 verified" is more legible than "5."

---

### F-4: "Probe Used" field has no context
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 422–425)
**Severity:** MINOR — operators seeing "rpm" or "dpkg" as a field value will not know
what this means without the help overlay.

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
    i18n::tr("Identity Probe"),
    sub.probe_used.to_owned(),
));
```

**Why:** "Identity Probe" communicates purpose (we used this to verify identity) rather
than just naming a mechanism. The value "rpm" or "dpkg" is still present and informative.
"Probe Used" sounds like debug output; "Identity Probe" sounds like a tool feature.

---

### F-5: "Contradictions" label is overloaded across two different concepts
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `append_kernel_contradiction_rows()` (line 638)
**Severity:** MEDIUM — both Tab 1 and Tab 2 use the label "Contradictions" for different
things: kernel/config disagreements (Tab 1) vs. OS identity source disagreements (Tab 2).
An operator who sees "Contradictions: 0" on one tab and "Contradictions: 2" on another
will not know why they differ.

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

And the explanation row when count > 0 (lines 651–657):
```rust
rows.push(DataRow::key_value(
    "",
    i18n::tr(
        "The running kernel and persisted configuration disagree on one or more \
         settings. Drift means intended hardening is not active; hotfixes mean \
         current hardening will be lost on reboot.",
    ),
    StyleHint::TrustRed,
));
```
This text is correct. Keep it.

And in `build_kernel_security_summary_rows()` — the zero-count explanation row (line 645):
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
    i18n::tr("Running kernel and persisted configuration agree on all settings."),
    StyleHint::Dim,
));
```

**Why:** "Config Drift" is the right security term for kernel-vs-config disagreements.
It distinguishes this finding from the "Contradictions" label in the Trust/Evidence tab,
which refers to OS identity source disagreements. The distinction matters: drift is an
operational state that may need a fix; identity contradictions are a potential integrity
concern. Using the same word for both trains operators to conflate them.

The updated zero-count explanation is slightly more active: it says what IS true rather
than what is NOT happening, which is easier to read.

---

### F-6: The "Curated indicators" note is placed after the verdict, not before
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_kernel_security_summary_rows()` (lines 827–837)
**Severity:** MINOR — the framing note appears at the bottom of the summary pane, after
the operator has already seen the indicator counts. It should frame before scoring.

**Current position:** After the contradiction section, before the final separator.

**Proposed:** Move the note to the top of the summary pane — before the Indicators count
line. Place it immediately after the Catalog Baseline row and the "All values below are
read live..." provenance note.

In the row builder at approximately line 746, after the provenance note row:
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

Add immediately after that separator:
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

And remove the current instance of this text at the bottom (lines 827–837).

**Why:** The operator should understand what they are looking at before they look at it.
Moving the framing note above the scoring data reduces the chance an operator reads the
count ("5 not hardened") without knowing that the catalog is curated and advisory.

---

### F-7: Section headers missing in OS Information tab — three different label styles need visual separation
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_os_info_rows()` (lines 369–447)
**Severity:** MINOR — the tab mixes raw `os-release` field names (`ID`, `NAME`,
`VERSION_ID`, `PRETTY_NAME`, `CPE_NAME`) with interpreted labels (`Platform Family`,
`Platform Distro`, etc.) without signaling the transition.

**Before the os-release fields block** (before line 374's first `DataRow`):
```rust
rows.push(DataRow::group_title(i18n::tr("OS RELEASE")));
```

**Before the platform identity block** (before line 401's substrate check):
```rust
rows.push(DataRow::group_title(i18n::tr("PLATFORM IDENTITY")));
```

The separator already present between these sections (`rows.push(DataRow::separator());`
at line 397) can remain.

**Why:** The group_title rows from the Kernel Security tab are the right pattern here. They
make the label style shift between raw kernel field names and interpreted platform identity
labels feel intentional rather than inconsistent. A conference demo screenshot will look
significantly more organized with two titled sections instead of one undifferentiated list.

---

### F-8: Status bar messages for lower trust tiers are too terse
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_status()` (lines 1428–1457)
**Severity:** MEDIUM — operators unfamiliar with UMRS trust tiers who land on a
`SubstrateAnchored` or `KernelAnchored` result will see a status bar message and not
know whether that is good, concerning, or normal.

**Current for SubstrateAnchored (line 1433):**
```rust
StatusMessage::new(StatusLevel::Info, i18n::tr("Platform Verified"))
```

**Replace with:**
```rust
StatusMessage::new(StatusLevel::Info, i18n::tr("T3: Platform Verified — normal operating posture"))
```

**Current for EnvAnchored (line 1436–1443):**
```rust
StatusMessage::new(
    StatusLevel::Warn,
    format!(
        "{} — {}",
        i18n::tr(trust_level_label(TrustLevel::EnvAnchored)),
        i18n::tr(trust_level_description(TrustLevel::EnvAnchored))
    ),
),
```

The current description is `"Mount topology cross-checked (mountinfo vs statfs)."` — that
is internal implementation detail, not operator orientation.

**Replace the description logic for EnvAnchored and KernelAnchored:**

For `EnvAnchored`, use:
```
"T2: Environment Anchored — partial trust; see Trust / Evidence tab"
```

For `KernelAnchored`, use:
```
"T1: Kernel Anchored — minimal trust; see Trust / Evidence tab"
```

**Implementation:** Update `trust_level_description()` to return operator-oriented strings,
or construct the status message directly in `build_status()` without calling
`trust_level_description()` for the status bar. If `trust_level_description()` is used
elsewhere with the implementation-detail strings, split it: keep the existing function for
the Trust/Evidence tab detail row, and write new status-bar-specific strings in `build_status()`.

**For T4 — IntegrityAnchored (line 1430):**
```rust
StatusMessage::new(StatusLevel::Ok, i18n::tr("Integrity Anchored"))
```
**Replace with:**
```rust
StatusMessage::new(StatusLevel::Ok, i18n::tr("T4: Integrity Anchored — highest confidence"))
```

**Why:** The status bar is the operator's first verdict. Adding `T-N:` prefix and a
one-phrase orientation ("normal operating posture", "partial trust") gives the operator
enough context to know whether to dig deeper without having to navigate to Tab 2 first.
The Tier prefix also teaches the tier system passively — after a few sessions, the
operator knows what T3 means without pressing `?`.

---

### F-9: `build_status()` for Untrusted misses an orientation phrase
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `build_status()` (line 1452)
**Severity:** MINOR

**Current:**
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
    i18n::tr("T0: Untrusted — no kernel anchor; do not rely on any value shown"),
)
```

**Why:** T0 is the most critical state. The operator needs to know immediately that the
entire card's data should be treated with suspicion. "no kernel anchor" is correct but
abstract. Adding "do not rely on any value shown" makes the operational implication
explicit in the status bar without requiring the operator to consult the help overlay.

---

### F-10: Detection pipeline failure error rows are not actionable
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `from_error()` (lines 254–265)
**Severity:** MEDIUM — per the help-text philosophy: "surface the finding, steer toward
resolution, do not dictate site procedure." The error rows currently surface the finding
but give no steering.

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
            "Hard gate: procfs is not real procfs. \
             Verify this tool is running on a Linux host with /proc mounted."
        )
    }
    DetectionError::PidCoherenceFailed { .. } => {
        i18n::tr(
            "Hard gate: PID coherence check failed. \
             This may indicate a container environment or a tampered /proc."
        )
    }
    DetectionError::KernelAnchorIo(_) => {
        i18n::tr(
            "Hard gate: I/O error reading kernel anchor. \
             Check permissions and whether /proc is accessible."
        )
    }
};
```

**Why:** Each error message now tells the operator where to look without prescribing
a site-specific procedure. "Verify this tool is running on a Linux host" and "may
indicate a container environment" are orientation, not instructions. The operator's
site procedure handles the rest.

---

### F-11: The tab name "OS Information" undersells security relevance
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `OsDetectApp::from_result()` and `OsDetectApp::from_error()` (lines 216, 302)
**Severity:** MINOR — outreach lens: "OS Information" sounds like `uname -a`. "System
Identity" communicates that this tab contributes to the trust assessment.

**Current (both from_result and from_error):**
```rust
TabDef::new(i18n::tr("OS Information")),
```

**Replace with:**
```rust
TabDef::new(i18n::tr("System Identity")),
```

Update both occurrences (lines 216 and 302).

Also update the module-level doc comment at line 22:
```
//! - **Tab 0 — OS Information**: `os-release` fields, platform identity, boot ID.
```
**Replace with:**
```
//! - **Tab 0 — System Identity**: `os-release` fields, platform identity, boot ID.
```

And the `help_text_for_tab()` function, tab 0 header (line 1482):
```
r" OS INFORMATION Tab
```
**Replace with:**
```
r" SYSTEM IDENTITY Tab
```

And the sub-header line (line 1483):
```
   ◼ Identity fields extracted from the running system.
```
This can stay verbatim.

**Why:** "System Identity" communicates to a security operator that this tab is a
trust-relevant artifact, not an administrative detail screen. In a conference demo,
a tab labeled "System Identity" next to "Kernel Security" and "Trust / Evidence"
reads as a coherent assessment vocabulary. "OS Information" reads as a sysadmin utility.

---

### F-12: The card title in `AuditCardApp` is not what operators need
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `card_title()` (line 329)
**Severity:** MINOR — the card title appears in the header border frame.

**Current:**
```rust
fn card_title(&self) -> String {
    i18n::tr("OS Detection Audit")
}
```

**Replace with:**
```rust
fn card_title(&self) -> String {
    i18n::tr("Platform Security Audit")
}
```

**Why:** "OS Detection Audit" names the internal pipeline. "Platform Security Audit"
names what the operator is doing. An operator running this on a CUI system wants to
know they are performing a platform security audit, not that the tool ran an "OS
detection" step. The header will read: `[ Platform Security Audit ]`. That is a
screenshot caption that explains itself.

---

### F-13: Help text for Tab 1 Kernel Security references "EVIDENCE CHAIN" — wrong section header
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `help_text_for_tab()`, case `1` (line 1549)
**Severity:** MINOR — "EVIDENCE CHAIN" is the Tab 2 concept. Tab 1 has an "INDICATOR
LIST." Reusing "evidence chain" language here blurs the conceptual distinction.

**Current (line 1549):**
```
 EVIDENCE CHAIN:
   Indicator       ✓ = Hardened (green)
```

**Replace with:**
```
 INDICATOR LEGEND:
   Indicator       ✓ = Hardened (green)
```

**Why:** Tab 2 has the evidence chain. Tab 1 has a posture indicator list. Different
things. The terminology confusion will propagate to operators who use "evidence chain"
to describe everything they see after scrolling, which conflates posture assessment
with trust evidence.

---

### F-14: The "OS Detection" internal name surfaces in `report_name()`
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Functions:** `report_name()` (line 321) and `report_subject()` (line 325)
**Severity:** MINOR — the header renders `"Assessment : OS Detection / Platform Identity
and Integrity"`. "OS Detection" names the internal pipeline step, not the assessment type.

**Current:**
```rust
fn report_name(&self) -> &'static str {
    "OS Detection"
}

fn report_subject(&self) -> &'static str {
    "Platform Identity and Integrity"
}
```

**Replace with:**
```rust
fn report_name(&self) -> &'static str {
    "Platform Assessment"
}

fn report_subject(&self) -> &'static str {
    "System Identity and Security Posture"
}
```

**Why:** The header will read `"Assessment : Platform Assessment / System Identity and
Security Posture"`. "Platform Assessment" is OSCAL-compatible and operator-legible.
"System Identity and Security Posture" accurately names what both Tab 0 and Tab 1 are
examining. A screenshot of this header does not require a caption to explain itself.

---

### F-15: Trust level descriptions used in status bar are implementation-facing
**File:** `components/rusty-gadgets/umrs-uname/src/main.rs`
**Function:** `trust_level_description()` (lines 76–92)
**Severity:** MINOR — these descriptions are used both in the Trust/Evidence tab
(where they are appropriate as technical detail) and — via `build_status()` — in
the status bar (where they are not appropriate).

The descriptions currently read:
- T1: `"procfs verified via PROC_SUPER_MAGIC + PID coherence."`
- T2: `"Mount topology cross-checked (mountinfo vs statfs)."`
- T3: `"Platform identity verified; >= 2 independent package facts confirmed."`
- T4: `"os-release ownership + installed digest verified."`

For the Trust/Evidence tab detail rows, these are correct and valuable. For the
status bar, they are implementation detail.

This fix is addressed in F-8 (build different status bar strings in `build_status()`,
do not call `trust_level_description()` for the status bar). The existing descriptions
stay on the Trust/Evidence detail rows unchanged.

No code change to `trust_level_description()` itself. The fix is in `build_status()`.

---

## CONSIDER — Ideas Needing Jamie's Input Before Implementation

### C-1: Default landing tab
The tool opens on Tab 0 (System Identity after F-11). An operator who has already
confirmed system identity may prefer to land on Tab 1 (Kernel Security) where the
findings are. Consider a `--tab` CLI argument or a persisted last-tab preference via
the XDG state directory. This is not urgent but it will come up once repeat users
accumulate.

**My recommendation:** Add `--tab <0|1|2>` as a CLI argument. No persistence. Clean,
predictable, composable with scripts.

### C-2: "r: refresh" in the status bar legend
The key legend currently reads:
```
Tab: tabs | ↑↓/jk/PgDn: scroll | ?: help | q: quit
```
File: `components/rusty-gadgets/umrs-ui/src/status_bar.rs`, line 39.

Consider:
```
Tab: tabs | ↑↓/jk/PgDn: scroll | r: refresh | ?: help | q: quit
```

This assumes a `r` refresh binding exists or will exist. If refresh is planned
(MEMORY.md notes it), add it to the legend at the same time the binding is wired.
If not planned for Cantrip, leave it out of the legend. Do not advertise a binding
that does not exist.

**My recommendation:** Defer to whenever refresh is implemented. But when it lands,
the legend update is a one-line change at `status_bar.rs:39`.

### C-3: Scroll position indicator
The Kernel Security tab can run to 60-80 visual lines. Operators have no way to know
where they are in the list or whether they have seen everything. A `[12/67]` position
indicator in the panel border or pinned pane would remove the uncertainty.

This requires a change to `data_panel.rs` to thread the total line count and current
offset through to the render function and inject it into the panel border title. It is
not a trivial change — it touches the render architecture — but it is not an
architectural change.

**My recommendation:** Schedule for the post-Cantrip polish cycle. The scrollbar is
present and functional; this is an enhancement, not a defect.

### C-4: Header indicator symbol prefix for NO_COLOR environments
Header indicators (SELinux, FIPS) are styled by color alone. In NO_COLOR mode they lose
visual distinction. Adding `[ON]` / `[OFF]` / `[?]` prefix text (honoring `NO_COLOR`)
would give text-mode fallback matching what the Kernel Security tab already does with
✓/✗/?.

This is a small change to `indicators.rs` / `header.rs` but requires agreeing on the
text format for the prefix. Worth doing before any government or Five Eyes engagement
where constrained terminal environments are common.

**My recommendation:** High priority for the Cantrip release review cycle.

### C-5: Visual separator between pinned and scrollable panes
Elena's review suggested a thin visual separator with labels `── Summary ──` /
`── Details ──` at the split boundary between the pinned pane and the scrollable section.

This is a `data_panel.rs` render change. The structural benefit is real — the split
is currently invisible to the operator. The risk is that it adds a visual element with
no security or data content, which may feel like clutter in a dense audit card.

**My recommendation:** Try it. If it reads as clutter, remove it. The cost is low.

### C-6: OS Information tab — section titles for raw vs. interpreted data
F-7 proposes `group_title` rows to separate "OS RELEASE" from "PLATFORM IDENTITY."
This is low-risk and high-benefit for screenshot legibility. If Jamie approves F-7,
the Kernel Security group-title style is the template.

**My recommendation:** Approve F-7. It is a three-line code change.

---

## Outreach Lens Assessment

### Screenshot quality
A screenshot of the current tool on a healthy system would show:
- Header: `[ OS Detection Audit ]`, `Assessment : OS Detection / Platform Identity and Integrity`
- Status bar: Green — `✓ Integrity Anchored`

With the fixes in this review, the same screenshot would show:
- Header: `[ Platform Security Audit ]`, `Assessment : Platform Assessment / System Identity and Security Posture`
- Status bar: Green — `✓ T4: Integrity Anchored — highest confidence`

The second version explains itself without a caption. A program manager seeing this in a
slide deck does not need to be told what the tool does. The first version requires context.

### Conference demo audience
Tab 1 (Kernel Security) is the demo-worthy tab. The group structure (BOOT INTEGRITY →
CRYPTOGRAPHIC POSTURE → KERNEL SELF-PROTECTION → etc.) reads well as a live demo because
the group names are short enough to read on a projector. The pinned summary pane showing
the score before the operator scrolls into the indicator list is a strong demo moment.

One thing that would make this even stronger: when indicators are all green, the "all
hardened ✓" line in the pinned summary is the headline. Make sure the demo system has at
least one or two red indicators for dramatic effect — a system that is completely hardened
is less compelling than one where the tool finds something actionable.

### Five Eyes / government partner framing
The Trust/Evidence tab is the government-relevant one. The evidence chain ordered by
trust tier, the dual-verified procfs/sysfs reads (fd + PROC_MAGIC / SYS_MAGIC), and
the SHA-256 digest display for T4 records are all credible to a formal assessor. The
weakness for that audience is the LabelTrust CamelCase strings (F-1) and the status
bar messages (F-8). Fix those and the Trust/Evidence tab can hold its own in a formal
review context.

The "Platform Security Audit" title (F-12 and F-14) will read better in any formal
documentation context. "OS Detection Audit" sounds like a QA step in a CI pipeline.

---

## Summary of Fixes by Priority

| Fix | Description | Severity | Effort |
|---|---|---|---|
| F-1 | Replace CamelCase LabelTrust display strings | HIGH | ~15 min |
| F-5 | Rename kernel contradictions to "Config Drift" | MEDIUM | ~15 min |
| F-8 | Status bar messages: add tier prefix and orientation | MEDIUM | ~30 min |
| F-10 | Error messages: add steering text | MEDIUM | ~20 min |
| F-2 | `boot_id` → `Boot ID` | MEDIUM | ~5 min |
| F-3 | "Platform Facts: N" → "Package Records: N verified" | MEDIUM | ~5 min |
| F-12 | Card title: "OS Detection Audit" → "Platform Security Audit" | MINOR | ~5 min |
| F-14 | report_name/report_subject: operator-facing strings | MINOR | ~5 min |
| F-11 | Tab name: "OS Information" → "System Identity" | MINOR | ~10 min |
| F-7 | OS Info tab section headers | MINOR | ~15 min |
| F-4 | "Probe Used" → "Identity Probe" | MINOR | ~5 min |
| F-6 | Move curated-indicators note above the scoring rows | MINOR | ~15 min |
| F-9 | T0 status bar message: add "do not rely on any value" | MINOR | ~5 min |
| F-13 | Help text: "EVIDENCE CHAIN" → "INDICATOR LEGEND" in Tab 1 | MINOR | ~5 min |
| F-15 | Document pattern: build_status() does not call trust_level_description() | NOTE | No code change |

**Total estimated implementation time:** ~2–3 hours for all fixes.

**Recommended batching for Rusty:** F-1 through F-5 as a single pass (the terminology
cluster), then F-7, F-11, F-12, F-14 as a second pass (the naming cluster), then
F-6, F-8, F-9, F-10 as a third pass (the operator orientation cluster).

---

*Review complete. Files do not need to be modified as part of this review — findings
are documented for Rusty's implementation pass.*
