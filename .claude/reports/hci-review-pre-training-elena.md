# HCI / Usability Review — umrs-uname TUI

**Reviewer:** Elena (The Imprimatur)
**Date:** 2026-03-22
**Baseline:** PRE-TRAINING — written before exposure to formal HCI/IA/KO academic material.
This document is one half of a controlled comparison. A post-training review will be written
after knowledge acquisition and compared against this baseline to measure the impact of
formal training on review quality.

**Sources reviewed:**
- `components/rusty-gadgets/umrs-uname/src/main.rs`
- `components/rusty-gadgets/umrs-ui/src/lib.rs`
- `components/rusty-gadgets/umrs-ui/src/layout.rs`
- `components/rusty-gadgets/umrs-ui/src/header.rs`
- `components/rusty-gadgets/umrs-ui/src/keymap.rs`
- `components/rusty-gadgets/umrs-ui/src/theme.rs`
- `components/rusty-gadgets/umrs-ui/src/app.rs`
- `components/rusty-gadgets/umrs-ui/src/status_bar.rs`
- `components/rusty-gadgets/umrs-ui/src/data_panel.rs` (partial — module doc)

---

## Summary Judgment

This is a well-architected tool with a clear mental model: header identifies the system,
tabs organize the data, the status bar gives the overall verdict, and the Trust / Evidence tab
is always the last tab. The design choices are coherent and the security rationale is embedded
in the code. The primary HCI gap is that the tool was designed outward from its data model
and security compliance requirements — which produces a thorough, accurate interface — but
leaves several real operator experience issues unaddressed, particularly around first-run
clarity, label opacity for non-specialist operators, and the steepness of the mental load
on the Kernel Security tab.

Severity ratings: CRITICAL / MAJOR / MINOR / SUGGESTION

---

## 1. Layout and Visual Hierarchy

### 1.1 Header layout is sound — MINOR concern on information density

The header provides a compact, consistent two-column identification block
(Assessment / Host / OS / SELinux+FIPS) with a wizard logo occupying the top-right corner.
This structure is logical: identity first, security posture indicators visible without
scrolling.

**MINOR:** The Assessment line reads `OS Detection / Platform Identity and Integrity`.
This is technically accurate but the slash-delimited format reads awkwardly. An operator
unfamiliar with OSCAL terminology may not know what "Assessment" means in this context or
why the report subject is separate from the report name. The distinction is internally
meaningful (NIST AU-3 framing) but creates a two-part label where one clear label would do.

### 1.2 Status bar placement and content — well executed, one concern

The status bar correctly provides the overall trust verdict (colored background, icon, text)
plus an inline key legend. This satisfies the "what is the verdict and what can I do" question
at a glance.

**MINOR:** The key legend reads `Tab: tabs | ↑↓/jk: scroll | ?: help | q: quit`. Listing
both `↑↓` and `jk` in the same slot implies they are alternatives. This is correct but
slightly compressed. An operator who has never used vim-style navigation will not know why
`jk` is meaningful here. The legend is functional, not self-explanatory.

### 1.3 Pinned-row / scrollable-row split — sound but not visible to the operator

The Kernel Security and Trust / Evidence tabs use a split-panel model: pinned rows above,
scrollable rows below. This is a good pattern for keeping summary state always visible. The
problem is that there is no visual cue that this split exists. An operator who scrolls the
data panel down will see the summary stay put, but without a visible separator or label, they
may not know whether the content above is pinned or simply part of the panel they cannot
scroll past. The scrollbar position will eventually reveal the truth, but the split itself
is invisible.

**MINOR:** Add a thin horizontal separator with a brief label (e.g., `── Summary ──` and
`── Details ──`) at the split boundary. This makes the panel model explicit.

---

## 2. Cognitive Load

### 2.1 Kernel Security tab overloads on first viewing — MAJOR

The Kernel Security tab presents: a pinned summary pane (kernel version, catalog baseline,
indicators summary count, contradictions count, and a curated-indicators note) followed by
a scrollable pane with six indicator groups (BOOT INTEGRITY, CRYPTOGRAPHIC POSTURE, KERNEL
SELF-PROTECTION, PROCESS ISOLATION, FILESYSTEM HARDENING, MODULE RESTRICTIONS, NETWORK
AUDITING). Each group has a title, a description sentence, and then one or more indicator
rows. Each indicator row has: a name, a ✓/✗/? prefixed value, a dim description, and
optionally a recommended setting and a contradiction warning.

That is a substantial amount to process. For an experienced assessor running against a
known-good baseline, this is efficient — they scan for red and move on. For a first-time
operator or a developer verifying their build environment, the density is overwhelming.
There is no progressive disclosure — everything is presented at once.

**MAJOR:** The curated-note at the bottom of the summary pane (`Curated indicators selected
to give you the clearest view of your system's security posture...`) is well-intentioned but
is placed after the scoring data, where operators may not read it on first exposure. Moving
this note to the very top of the summary pane, before the indicators count, would help frame
what the operator is looking at before they encounter the numbers.

### 2.2 Contradiction terminology is used in two different senses — MAJOR

The word "contradiction" appears with two distinct meanings:

- On the **Trust / Evidence tab**: a contradiction means two independent OS-detection sources
  (e.g., `/etc/os-release` vs. the package database) reported conflicting values for the same
  fact about the OS identity.
- On the **Kernel Security tab**: a contradiction means the running kernel and the persisted
  configuration disagree on a hardening setting (DRIFT, NOT PERSISTED, UNVERIFIABLE).

These are genuinely different things. The help text for the Trust / Evidence tab explains this
distinction explicitly (lines 1528-1533), but the distinction is not visible in the UI itself.
An operator looking at both tabs will see "Contradictions: None" on both and reasonably assume
they mean the same thing. If one tab shows contradictions and the other does not, the operator
must read the help text to understand why they are different.

**MAJOR:** Use distinct labels. Consider "OS Identity Conflicts" or "Detection Conflicts" on
Tab 2 and "Configuration Drift" or "Kernel/Config Disagreements" on Tab 1. The current
shared label `Contradictions` obscures the difference and could lead an operator to
under-report or misroute a finding.

### 2.3 The Trust Tier labels are dense without context — MINOR

The trust tiers are labeled `T0 — Untrusted` through `T4 — Integrity Anchored`. The label
includes a short description (e.g., "procfs verified via PROC_SUPER_MAGIC + PID coherence").
This is meaningful to a developer who knows the pipeline architecture. To a security auditor
who has not read the internals, "PROC_SUPER_MAGIC + PID coherence" is opaque. The label tells
them nothing about what passed or failed at that level.

**MINOR:** The description is better served as a "what was verified" statement than an
"how it works" statement. "procfs filesystem identity confirmed; process IDs are coherent"
is more action-legible than the current technical form.

---

## 3. Learnability

### 3.1 Help system is present but requires the operator to know to ask — MINOR

The help overlay (`?` or `F1`) provides contextual, per-tab help text that is genuinely good.
Tab 1's help text explains the symbols, the groups, and what contradictions mean. Tab 2's help
text distinguishes the two contradiction types. This is the right content.

The problem is discoverability. The key legend in the status bar shows `?: help`, which is a
functional hint. However, an operator who opens the tool for the first time and stares at the
Kernel Security tab will see a dense panel and may not think to press `?`. The help is
opt-in, not offered.

**SUGGESTION:** On first launch, open the help overlay automatically for Tab 0 (OS
Information) with a brief orientation note: "This is your first run. Press ? on any tab for
context. Press q or Esc to quit." This is a common pattern for terminal tools aimed at
operators rather than developers. It does not have to be permanent — it can respect a
~/.config or environment flag to suppress it after first use.

### 3.2 Symbol vocabulary is explained in help but not inline — MINOR

The ✓/✗/? symbols are used on the Kernel Security tab to mark hardening outcomes. These are
intuitive — check and cross are nearly universal. However, the `?` for "no assessment" could
be confused with "I don't know what this means" by a new operator. The `?` symbol also
doubles as the help key. These two uses of `?` are in different contexts (one is in the data
panel, one is in the key legend), so they do not collide, but they produce a mild cognitive
friction.

**MINOR:** Consider replacing `?` for "no assessment" with `~` or `–` (en dash). Both convey
"not applicable / not determinable" without ambiguity and without overlap with the help key.

### 3.3 Tab ordering is correct by convention but the convention is not explained — MINOR

The code comments explicitly state that Trust / Evidence is always the rightmost (last) tab —
a UMRS convention. This convention is not communicated to the operator. If this convention
is meant to hold across all UMRS tools (which the codebase suggests it is), operators will
learn it through repeated use. But it is not stated in the help text or UI.

**SUGGESTION:** Add one line to the help overlay for each tab: "This is Tab [N] of 3. Trust
/ Evidence is always the last tab in UMRS tools." Repeated exposure to this line across tools
will teach the convention without requiring documentation.

---

## 4. Error Visibility

### 4.1 Hard-gate failure mode is visually clear — positive finding

When OS detection fails (hard gate), the tool shows:
- Status bar: red background, "Detection pipeline failed"
- Tab 0 (OS Information): two red rows — Status / Reason
- Tab 2 (Trust / Evidence): "T0 — Untrusted" / "Hard gate failure aborted pipeline"

This is explicit, red, and unambiguous. The failure reason uses plain English
("Hard gate: procfs is not real procfs") rather than error codes. Good.

### 4.2 Contradiction detection and display is good but the ⚠ symbol is not explained inline — MINOR

Contradiction rows on the Kernel Security tab use the ⚠ symbol (U+26A0) prefixed to the
contradiction kind (DRIFT, NOT PERSISTED, UNVERIFIABLE). The ⚠ symbol is appropriate for
this usage. However, the three kinds are only explained in the help overlay for Tab 1. An
operator who sees `⚠ DRIFT` inline will not immediately know what DRIFT means without
pressing `?`.

**MINOR:** Add a one-line tooltip or dim note immediately beneath each contradiction row
explaining what that specific kind means. The existing code already appends description rows
for downgrade reasons — the same pattern could apply here. The inline explanation (lines
654-657) in the `append_kernel_contradiction_rows` function explains the *presence* of
contradictions, but not the kind. Per-kind explanation requires going to the help overlay.

### 4.3 "Unavailable" indicator vs. "Disabled" indicator — subtle but meaningful — MINOR

The theme distinguishes:
- `Enabled` → green bold
- `Disabled` → dark gray
- `Unavailable` → yellow

The distinction between "disabled" and "unavailable" is important: disabled means the kernel
confirmed it is off; unavailable means the read failed. The yellow / dark-gray distinction
conveys this, but only if the operator understands the semantic difference. The theme module
doc explains it clearly. The operator-facing help text explains it less clearly. The Tab 1
help text says `dim = unavailable` but does not explain what "unavailable" means operationally
(the read failed, not the feature is off).

**MINOR:** In the Tab 1 help text, clarify: `dim/yellow = unavailable (could not read from
kernel — different from disabled)`.

---

## 5. Accessibility

### 5.1 NO_COLOR support is present and verified — positive finding

The code explicitly uses ✓/✗/? symbols as non-color indicators (lines 1074-1084) so that
hardening status is readable without color. The comment at line 1076 explicitly cites WCAG
1.4.1. This is correct and commendable — it means the tool remains functional in NO_COLOR
environments and on terminals with limited palette support.

### 5.2 NO_COLOR behavior is implied but not documented for the operator — SUGGESTION

The tool honors NO_COLOR (per the CLAUDE.md project rules), but there is no mention of this
in the help overlay or the README. An operator running in a constrained environment (audit
terminal, restricted shell) who sets NO_COLOR will see the interface change but will not
know whether this is intended or broken.

**SUGGESTION:** Add one line to the status bar area or help overlay: "NO_COLOR mode is
supported; symbols indicate status without color."

### 5.3 Dark-only theme is a limitation — SUGGESTION

The project memory notes that a `--light-theme` CLI option is planned. Until it exists, the
current tool is dark-terminal-only. This limits adoption in environments where operators
use light terminal themes (common in browser-based terminals, Windows Terminal defaults,
some corporate SSH setups).

This is a known gap — recording it here for completeness.

### 5.4 Unicode symbol coverage — minor concern

The tool uses: ✓ (U+2713), ✗ (U+2717), ⚠ (U+26A0), — (U+2014 em dash), … (U+2026
ellipsis). These are all in the Basic Multilingual Plane and should render correctly in most
modern terminal fonts. However, on legacy terminals or in environments without full Unicode
support (some DoD terminal configurations), these may render as replacement characters. No
fallback path exists for non-Unicode terminals.

**MINOR:** Consider defining a `NO_UNICODE` path alongside `NO_COLOR`. This would replace
✓/✗ with `+`/`-` or `Y`/`N`, ⚠ with `!`, and `—` with `-`. This is low priority given the
target environment (RHEL 10, modern SSH), but worth noting.

---

## 6. Navigation

### 6.1 Key bindings are appropriate and dual-mapped — positive finding

The keymap provides: `Tab`/`Right` for next tab, `Shift-Tab`/`Left` for previous tab,
`j`/`k`/`↑`/`↓` for scroll, `PageUp`/`PageDown` for fast scroll, `q`/`Esc` for quit,
`?`/`F1` for help. This covers three navigation conventions simultaneously: mouse-trained
users (arrow keys), vim users (jk), and keyboard-shortcut users (Tab). The coverage is good.

**MINOR concern:** `Esc` is bound to Quit. This is conventional for modal TUIs, but `Esc`
is also the most common "go back" or "cancel" action in dialog-driven applications. When
a dialog is open, `Esc` correctly dismisses the dialog (the code checks `help_dialog.is_some()`
before routing to Quit). However, if an operator presses `Esc` while reading the main card
expecting to "go back" (to a terminal prompt, to a previous screen), they will immediately
quit. This could lose their place during a live assessment. The behavior is correct — there
is nothing to go back to — but it may surprise operators from GUI or web-based tool backgrounds.

**SUGGESTION:** Consider whether a "are you sure?" prompt is appropriate here. Given the
security environment, an accidental quit during evidence review is a recoverable annoyance
(re-run the tool), not a data-loss event. A confirmation dialog is probably not worth the
friction. Worth keeping as a known behavior to document.

### 6.2 Scroll position is not displayed — MINOR

The data panel renders a scrollbar when content overflows, which allows the operator to
estimate their position in the content. However, there is no line counter or position
indicator (e.g., `Line 12/47`). On the Kernel Security tab, which can have a significant
number of indicator rows, an operator may not know how much content remains below the
visible area.

**MINOR:** Add a position indicator to the right of the key legend in the status bar, or
above the scrollbar: `↓ more` or `12/47`. This is especially useful during assessment when
the operator needs to confirm they have reviewed all indicators.

### 6.3 There is no way to jump directly to a specific tab by number — SUGGESTION

Tab navigation wraps around via Tab/Right. With only three tabs this is fast. If the tab
count grows in future tools, numeric shortcuts (1, 2, 3) would be useful. For the current
three-tab layout this is a non-issue.

---

## 7. Terminology

### 7.1 "Label Trust" vs. "Trust Tier" — MINOR naming inconsistency

The Trust / Evidence tab shows two rows with similar but distinct labels:

- `Label Trust` — the LabelTrust enum value (UntrustedLabelCandidate, LabelClaim,
  TrustedLabel, IntegrityVerifiedButContradictory)
- `Trust Tier` — the numeric confidence level (T0–T4)

These are different measurements of the same concept (how much to trust the OS detection
result), surfaced as adjacent rows. The distinction is technically accurate — Label Trust
assesses the integrity of the `/etc/os-release` label specifically; Trust Tier reflects the
full confidence pipeline — but to an operator, two consecutive "trust" rows may cause
confusion about which one is the operative verdict.

**MINOR:** Add a brief parenthetical distinguishing them:
- `Label Trust (os-release)`
- `Trust Tier (full pipeline)`

Or consider grouping them under a sub-header `OS-Release Integrity` and `Detection Confidence`.

### 7.2 Evidence type labels are good — positive finding

The `source_kind_label` function maps internal type names to operator-facing labels:
`Procfs` → "Kernel runtime (/proc)", `SysfsNode` → "Kernel attributes (/sys)". This is
exactly right. The labels name the source in terms an operator recognizes, with the actual
path in parentheses for confirmability.

### 7.3 "Catalog Baseline" needs more context at first exposure — MINOR

The `Catalog Baseline` row shows a comparison of the running kernel version against the
version the indicator catalog targets. The row text (e.g., `5.15.1 is newer than catalog
baseline (5.14.0) — some indicators may have changed`) is clear in context. However,
"catalog baseline" is UMRS-specific terminology. A first-time operator will not know what
a "catalog" is or why it matters.

**MINOR:** Rename to `Indicator Catalog` and change the description to: "The indicator
definitions were written for kernel X. Running a newer kernel may mean some indicators
have changed." This does not require a TUI change — the help text update is sufficient.
The current help text (Tab 1 help) does not mention the catalog baseline comparison at all.

### 7.4 "Platform Facts" — unclear label — MINOR

In Tab 0 (OS Information), the `Platform Facts` row shows a count (e.g., `3`). This is the
`substrate_identity.facts_count`. To an operator, "Platform Facts" with a numeric value is
opaque. Three what? What is a "platform fact"?

**MINOR:** Rename to `Evidence Facts` or `Substrate Facts Confirmed` and add a note in the
help text explaining what this count represents (the number of independent package-database
facts that confirmed the platform identity).

### 7.5 "Probe Used" — developer terminology exposed to operators — MAJOR

Tab 0 shows a row `Probe Used` with a value like `rpm` or `dpkg`. This is the
`substrate_identity.probe_used` field. The label "Probe Used" is internal vocabulary —
it makes sense in the context of the detection pipeline architecture but not in an operator
context. An operator will reasonably wonder: what probe? probing for what?

**MAJOR:** Rename to `Package System` or `Detection Method` and use a more descriptive
value if needed (e.g., `RPM database` rather than `rpm`). This row provides useful
information (how the platform was identified) but the current label obscures it.

---

## 8. Information Architecture

### 8.1 Tab organization is logical — positive finding

The three-tab structure (OS Information → Kernel Security → Trust / Evidence) follows a
natural evaluation flow: identify the system, assess its security posture, then verify the
evidence chain. Placing Trust / Evidence last is a deliberate convention and makes sense
for the "deepest dive" use case. Experienced operators will likely spend most time on Kernel
Security; auditors will want Trust / Evidence.

### 8.2 OS Information tab mixes concerns — MINOR

Tab 0 presents three sections:
1. `os-release` fields (ID, NAME, VERSION_ID, PRETTY_NAME, CPE_NAME)
2. Platform identity (Family, Distro, Version, Facts, Probe)
3. Boot ID

Sections 1 and 2 are closely related (both identify the OS). Section 3 (Boot ID) is a
session correlation field — it ties this detection run to a specific boot. It is not
really an "OS Information" item in the way the other fields are; it is more of an
audit/correlation field. Placing it in Tab 0 is reasonable (it has to go somewhere),
but it feels slightly out of place.

**SUGGESTION:** Move the Boot ID to the header context or to a supplemental header field
if space permits, or to the Trust / Evidence tab where it serves as session correlation
for the evidence chain. As-is it is a minor structural misfit, not a usability problem.

### 8.3 The header shows SELinux and FIPS but the Kernel Security tab does not repeat them at the top — potential confusion

The header always shows `SELinux: enforcing` and `FIPS: active`. The Kernel Security tab
shows the same values again (in BOOT INTEGRITY for lockdown, in CRYPTOGRAPHIC POSTURE for
FIPS). The repetition is not exact — the header shows current enforcement mode, the tab
shows a posture assessment — but an operator may wonder why they see similar information
in two places with different labels.

This is not a bug. The header is a "persistent identity snapshot" (not security claims);
the tab is a "hardening assessment". But the distinction is not communicated to the operator.
The header module doc correctly notes these are "display-only" and "not trust-relevant
assertions", but that distinction lives in the code, not the UI.

**MINOR:** A single sentence in the Tab 1 help text would resolve this: "Header indicators
(SELinux, FIPS) show live enforcement mode. The Kernel Security tab evaluates these and
additional indicators against the hardened baseline."

---

## 9. Consistency

### 9.1 Visual consistency is strong — positive finding

The tool uses a consistent visual vocabulary throughout:
- Green = hardened / trusted
- Red = not hardened / untrusted / failed
- Yellow = warning / unavailable
- Gray/dim = informational / secondary
- Cyan = labels and borders

This mapping is applied consistently across all tabs, the header, the status bar, and
dialog borders. An operator who learns the color vocabulary on one tab will correctly
interpret it on all others.

### 9.2 Group title casing is inconsistent — MINOR

Indicator group titles use ALL CAPS (BOOT INTEGRITY, CRYPTOGRAPHIC POSTURE, KERNEL
SELF-PROTECTION). OS Information uses Title Case labels (Platform Family, Platform Distro).
Trust / Evidence uses Title Case (Label Trust, Trust Tier, Downgrade Reasons). The
ALL CAPS group titles on the Kernel Security tab stand out differently from all other labels.

This is a deliberate visual choice (ALL CAPS group titles act as visual dividers), but it
is worth noting as a style inconsistency. If the target audience includes users who associate
ALL CAPS with urgency or warnings, this choice may create mild unintended emphasis. The
group titles are informational, not warning indicators.

**SUGGESTION:** Consider sentence-case group titles with a bold style instead of ALL CAPS.
The `group_title` style is already `bold white`, which provides visual distinction without
the urgency connotation of ALL CAPS.

### 9.3 Separator rows are used inconsistently — MINOR

The code uses `DataRow::separator()` as visual spacing. Within indicator groups, separators
appear between the description and the first indicator, and after the last indicator. In
the Trust / Evidence tab, separators appear before and after the contradiction section. In
the OS Information tab, separators separate the three sections. The separator logic is
correct but the visual rhythm is inconsistent across tabs — some sections feel more cramped
than others. This is probably a consequence of iterative development rather than a design
decision.

**SUGGESTION:** Establish a consistent separator rule: one blank row between sections within
a tab, two blank rows between major visual blocks (pinned summary / scrollable detail). Apply
uniformly across all tabs.

---

## 10. Additional Observations

### 10.1 The wizard mascot is a positive differentiator — positive finding

The `WIZARD_SMALL` ASCII art in the top-right corner gives the tool character without
reducing clarity. It occupies a fixed 17-column panel that would otherwise be empty space.
This is a good use of available real estate — it signals "this is a tool with personality"
without interfering with any functional content.

### 10.2 Evidence verification codes require background knowledge — MINOR

The evidence chain in Tab 2 shows verification codes like `✓ ok (fd, PROC_MAGIC)` and
`✓ ok (fd, SYS_MAGIC)`. These tell a UMRS developer exactly what happened. They tell a
security auditor who has read the documentation what happened. They tell a first-time
operator nothing useful. The help text (lines 1549-1553) explains these codes, but requires
the operator to have read it.

**MINOR:** Add a one-line legend at the top of the evidence section (above the column
headers) explaining the verification codes: `fd = file descriptor (safer); *_MAGIC =
filesystem type confirmed`. This is similar to how indicator groups already include a
description row below the title.

### 10.3 SHA-256 digests in the evidence chain are not contextualised — MINOR

When a T4 integrity record includes a SHA-256 digest, the evidence chain shows:
`sha256: <64-character hex string>`

The operator sees a long hex string but has no immediate way to verify it against anything.
The help text does not mention what to do with the digest. It is a finding that requires
external action (compare against a known-good reference), but the UI does not say this.

**MINOR:** Add a note below the digest row: `Compare against release checksum to verify
integrity.` This is a one-line action prompt that turns a data display into an actionable
instruction.

### 10.4 No way to copy text from the TUI — SUGGESTION

Terminal TUIs traditionally rely on the operator's terminal emulator for text selection and
copy. This tool does not provide any explicit "copy to clipboard" action. For a security
assessment workflow, an operator reviewing evidence may want to copy a path, a boot ID, or
a SHA-256 digest into a report or a ticket. This is a workflow gap.

A TUI copy-to-clipboard action is non-trivial to implement correctly across terminal
environments. This is a suggestion for consideration, not a finding against the current
implementation.

### 10.5 No indication of data freshness — SUGGESTION

The header shows an `Assessed` timestamp, which is the collection time. This is good. But
there is no indication of how stale the data might be — the tool collects once at startup
and does not refresh unless the operator presses `r`. The `r` key binding exists (`Refresh`)
but the action object in `keymap.rs` is defined and the key is bound, yet the event loop
in `main.rs` does not handle `Action::Refresh` — it falls through to
`state.handle_action(&action)` which will silently ignore it since `AuditCardState` does
not implement refresh. This means the `r` key is bound but does nothing.

**CRITICAL:** The `Action::Refresh` key is bound (`r`) and exposed in developer documentation,
but the event loop in `umrs-uname/src/main.rs` does not handle it — the match arm falls to
`state.handle_action(&action)` which does not implement refresh. The operator has no way to
re-collect data without quitting and re-running the tool. More critically, the bound key
`r` silently does nothing when pressed, which may cause an operator to believe the data was
refreshed when it was not. This is both a functional gap and a silent failure.

---

## Finding Summary

| # | Area | Finding | Severity |
|---|---|---|---|
| 2.1 | Cognitive load | Kernel Security tab: curated-note placement obscures framing | MAJOR |
| 2.2 | Cognitive load | "Contradictions" means two different things across tabs | MAJOR |
| 7.5 | Terminology | "Probe Used" is internal vocabulary — operators cannot interpret it | MAJOR |
| 10.5 | Data freshness | `Action::Refresh` is bound but silently does nothing in the event loop | CRITICAL |
| 1.1 | Visual hierarchy | "Assessment" label / slash format is OSCAL-internal terminology | MINOR |
| 1.2 | Visual hierarchy | Key legend lists `↑↓/jk` without explaining why both exist | MINOR |
| 1.3 | Visual hierarchy | Pinned-row / scrollable-row split has no visible separator | MINOR |
| 2.3 | Cognitive load | Trust Tier descriptions use internal implementation vocabulary | MINOR |
| 3.2 | Learnability | `?` symbol used for both "no assessment" and "help key" | MINOR |
| 3.3 | Learnability | "Trust / Evidence is always last" convention not communicated in UI | MINOR |
| 4.2 | Error visibility | ⚠ DRIFT / NOT PERSISTED / UNVERIFIABLE not explained inline | MINOR |
| 4.3 | Error visibility | Help text does not distinguish "unavailable" from "disabled" operationally | MINOR |
| 5.4 | Accessibility | No fallback for non-Unicode terminals | MINOR |
| 6.2 | Navigation | No scroll position indicator | MINOR |
| 7.1 | Terminology | "Label Trust" and "Trust Tier" appear to overlap without explanation | MINOR |
| 7.3 | Terminology | "Catalog Baseline" is UMRS-specific jargon not explained in UI | MINOR |
| 7.4 | Terminology | "Platform Facts" count is opaque | MINOR |
| 8.2 | Information architecture | Boot ID out of place in OS Information tab | SUGGESTION |
| 8.3 | Information architecture | Header SELinux/FIPS vs. Kernel Security tab repetition unexplained | MINOR |
| 9.2 | Consistency | ALL CAPS group titles may connote urgency | SUGGESTION |
| 9.3 | Consistency | Separator row rhythm is uneven across tabs | SUGGESTION |
| 10.2 | Evidence display | Verification codes (fd, PROC_MAGIC) require background knowledge | MINOR |
| 10.3 | Evidence display | SHA-256 digests are not contextualised with an action | MINOR |
| 3.1 | Learnability | First-run orientation is absent | SUGGESTION |
| 5.2 | Accessibility | NO_COLOR behavior not documented for the operator | SUGGESTION |
| 5.3 | Accessibility | Dark-only theme limits adoption (known gap) | SUGGESTION |
| 6.3 | Navigation | No numeric tab shortcuts (non-issue at 3 tabs) | SUGGESTION |
| 10.4 | Workflow | No copy-to-clipboard action | SUGGESTION |

---

## Closing Note

The most significant finding is item 10.5 (the Refresh key silently does nothing). This should
be verified against the current code, as I am reading from source rather than running the tool.
If confirmed, the operator receives false feedback from a bound key — that is a trust issue in
a trust-reporting tool.

The second most important finding (2.2, contradiction terminology) is a terminology design
problem that will scale poorly as the tool gains more tabs or is adopted by operators who
use multiple UMRS tools simultaneously. Resolving it before the Cantrip release would prevent
a terminology debt that grows with the user base.

The positive findings are real: the visual vocabulary is consistent and well-chosen, the
color/symbol dual-encoding for NO_COLOR is correct, the evidence source labels are
operator-legible, the wizard mascot adds character, and the overall information architecture
is sound. This is a better-than-average first TUI for a high-assurance tool.
