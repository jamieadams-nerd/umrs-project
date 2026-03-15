# TUI Enhancement Plan — Security Auditor Review
**Report date:** 2026-03-15
**Reviewer:** security-auditor agent
**Subject:** `.claude/plans/tui-enhancement-plan.md`
**Depth:** In-depth (operational review as primary consumer)

---

## Purpose of This Review

This is an operational review, not a code-level annotation audit. The security-auditor
agent is a named primary consumer of these interfaces. This review evaluates whether the
plan produces interfaces that:

1. Let an operator/auditor immediately assess system security posture
2. Generate sufficient, traceable evidence for SP 800-53A assessment
3. Serve the acknowledgement workflow implied by the Dialog API
4. Correctly cite applicable controls for all new types and functions
5. Identify gaps the plan does not address that an assessor would require

---

## Overall Assessment

The plan is technically sound and well-structured. The trust boundary note, the enum-over-builder
decision for `DataRow`, and the `Unavailable` fail-closed design for `IndicatorValue` are all
correct and demonstrate good design discipline. The compliance annotation assignments in the
Cross-Cutting Constraints section are appropriate.

Several operational and evidence-sufficiency gaps were identified. They are enumerated below.
None block implementation but several should be resolved before the Kernel Security tab content
is populated in future phases.

---

## Phase-by-Phase Review

---

### Phase 1 — Security Indicators in Header

**Positive findings:**

- `IndicatorValue::Unavailable` as the fail-closed default is correct. The plan explicitly
  states "do not guess or default to active" — this is the right behavioral requirement and
  it aligns with NIST SP 800-53 CM-6 (configuration management settings must reflect actual
  state, not assumed state).
- The mandate that all reads go through `SecureReader` (no `File::open` on `/sys/` or
  `/proc/`) is correct and matches the project's RAIN principle.
- The indicator row format is scannable. An operator glancing at the header will immediately
  see enforcement state without navigating to a tab.

**Operational gap — indicator granularity for SELinux:**

The plan maps `selinux_status` to `/sys/fs/selinux/enforce`, which yields a binary
`0`/`1` value. This is insufficient for rapid operator assessment. An operator needs to
distinguish at minimum three SELinux states: disabled, permissive, and enforcing. A system
running in permissive mode is a security concern that must be immediately visible. The current
design would render `Active("0")` for permissive (SELinux enabled but not enforcing), which
an operator will misread as "inactive." The `IndicatorValue` design conflates the enabled/disabled
axis with the enforcing/permissive axis.

**Recommendation:** The `selinux_status` field should produce a three-way value:
`Active("enforcing")`, `Inactive("permissive")`, or `Unavailable`. Reading only
`/sys/fs/selinux/enforce` (enforcing node) is correct. Reading `1` → enforcing (Active),
reading `0` → permissive (Inactive, with the string "permissive" in the display), and a
read failure → `Unavailable`. The existing `IndicatorValue` variants already support this
mapping without API change; the population function in `indicators.rs` must implement it
correctly.

**Operational gap — secure boot indicator reliability:**

The plan acknowledges uncertainty about the `secure_boot` source path
(`/sys/firmware/efi/efivars/SecureBoot-…` or `/proc/sys/kernel/secure_boot`). On RHEL 10
the correct path is the EFI variable — the procfs path does not exist in standard kernels.
The efivars path has a variable GUID suffix that must be determined at runtime, which
`SecureReader` cannot do with a compile-time path constant. The stub/`Unavailable` fallback
is correct for now, but this field should carry a note in the implementation that it cannot
be reliably populated via the standard kattr mechanism and requires a different probe strategy
(directory enumeration under `/sys/firmware/efi/efivars/` looking for `SecureBoot-*`).

**Citation check — Phase 1 compliance annotations:**

- `SecurityIndicators` → `NIST SP 800-53 SI-7, CM-6`: correct. SI-7 covers software and
  information integrity (kernel-sourced posture flags); CM-6 covers configuration settings.
- `IndicatorValue` → `NIST SP 800-53 SI-3, CM-6`: The plan cites SI-3 (malicious code
  protection). This is incorrect for an indicator that represents kernel configuration state.
  The cross-cutting constraints table at the bottom of the plan correctly reassigns
  `IndicatorValue` to `SI-7, CM-6`. The Phase 1 `IndicatorValue` doc comment draft should
  use `SI-7, CM-6`, not `SI-3, CM-6`. SI-3 is a malware detection control, not a kernel
  posture display control.
- `indicators.rs` module → `NIST SP 800-53 SI-7, CM-6, CM-7`: correct. CM-7 (least
  functionality — kernel should restrict unnecessary services) is relevant because this module
  queries whether kernel lockdown mode is active.

---

### Phase 2 — Header Extensibility

**Positive findings:**

- The default `&[]` return is the correct design. Backward-compatible default methods on
  traits are preferable to additional required methods.
- The `HeaderField` type comment correctly prohibits security labels and credentials in
  the value field. This is the right trust-boundary enforcement at the display layer.

**Citation check:**

- `HeaderField` → `NIST SP 800-53 AU-3`: correct. Every audit card header field is labelled,
  satisfying the AU-3 requirement that audit records identify the information source.

**Operational observation — no validation of field count:**

The plan specifies truncation with `…` when supplemental fields overflow the header height.
This is the correct graceful-degradation approach. However, the plan does not specify what
happens to `AuditCardState::scroll_offset` when a header truncation event occurs (i.e., the
operator's scroll position in the data panel is unrelated to header overflow). This is a
minor implementation note, not a plan-level issue.

---

### Phase 3 — Two-Column Layout

**Positive findings:**

- The `DataRow` enum decision is correct and well-reasoned. A builder-style `add_row(Column::Left, ...)`
  API would require stateful rendering; the `TwoColumn` variant keeps the render function
  stateless and auditable.
- All six `TwoColumn` fields being individually visible means an assessor can programmatically
  inspect the left and right content independently — consistent with the "security findings as
  data" pattern.

**Operational gap — column pairing semantics for security data:**

The feedback document shows an intended pairing:

```
LEFT COLUMN        RIGHT COLUMN
OS identity        Kernel flags
SELinux state      Boot security
```

The `TwoColumn` variant correctly models this. However, the plan does not specify a
convention for when `TwoColumn` vs. `KeyValue` should be used in the OS detection card
specifically. If the implementation mixes `TwoColumn` and `KeyValue` rows ad-hoc, the
resulting display may have uneven columns and mixed visual rhythm that reduces scan speed.

**Recommendation:** Add a convention note (in code documentation, not the plan) that
`TwoColumn` is appropriate when two logically independent but equally-weighted facts belong
on the same conceptual row, and `KeyValue` is used for single facts with subordinate detail.
The OS Detection tab should use `TwoColumn` for peer facts (distro name vs. kernel version,
SELinux enforcing status vs. FIPS status) and `KeyValue` for facts with detail rows below.

**Citation check — Phase 3:**

- `DataRow` enum → `NIST SP 800-53 AU-3`: correct. The cross-cutting constraints table
  confirms this.

---

### Phase 4 — Group Titles

**Positive findings:**

- The caller-convention approach to indentation (prepend `" "` to key strings) is the right
  choice. Stateless rendering is more auditable.
- The explicit documentation requirement for the indentation convention in `GroupTitle`'s
  doc comment is correct.

**Operational gap — group title as assessment boundary marker:**

For an SP 800-53A assessor, group titles serve a more important function than visual
organization: they mark the boundary between assessment objects. For example, a
`SELINUX` group title implicitly claims "all rows below this title are SELinux-sourced
posture data." If a caller accidentally places a non-SELinux row under a SELINUX group title,
the visual presentation misleads the assessor.

The plan correctly leaves grouping policy to the caller (the library is policy-free). However,
the doc comment on `GroupTitle` should explicitly state that group titles carry no semantic
enforcement — they are visual organizers only, and the caller is responsible for placing the
correct rows under the correct group.

---

### Phase 5 — Theme Styling

**Positive findings:**

- Centralizing all style definitions in `theme.rs` with no inline color values elsewhere is
  correct.
- The `indicator_style()` helper with a `#[must_use]` annotation is correct.

**Operational gap — color accessibility for DoD operators:**

The plan specifies `Color::Green` for `indicator_active` and `Color::DarkGray` for both
`indicator_inactive` and `indicator_unavailable`. On most DoD terminal emulators (typically
running over SSH to a restricted console), `Color::DarkGray` and `Color::Green` on a dark
background remain distinguishable. However, `Inactive` (dim gray) and `Unavailable` (dim gray
+ DIM modifier) will be nearly identical to color-blind operators. The distinction between
"inactive" (explicitly disabled) and "unavailable" (could not be read — a potential security
concern) is operationally significant.

**Recommendation:** `indicator_unavailable` should use a distinct color from
`indicator_inactive`. Suggested: `indicator_inactive` uses `Color::DarkGray` (explicitly
off), `indicator_unavailable` uses `Color::Yellow` or `Color::Magenta` (something is wrong —
the kernel node could not be read, which itself warrants investigation). An operator seeing
a yellow `[FIPS:??]` or `[SB:??]` knows to investigate; gray-on-gray suggests "not relevant."
This distinction directly serves the CA-7 (continuous monitoring) goal.

---

### Phase 6 — Evidence Tab Grouping

This is the phase with the most operational impact for assessors.

**Positive findings:**

- Grouping by `SourceKind` is correct. A procfs evidence record and a package-db evidence
  record require different evaluation procedures; keeping them visually separate maps to
  how an SP 800-53A assessor would actually evaluate evidence chains.
- The `BTreeMap<SourceKind, Vec<&EvidenceRecord>>` deterministic ordering requirement is
  correct — sorted, reproducible output is required for assessment comparability across
  runs.
- Path truncation and the `"verified"` / `"FAIL"` mapping for `parse_ok` are the correct
  SI-12 discipline approach.

**Evidence sufficiency gap — verification column is too sparse:**

The proposed three-column table format is:

```
Evidence Type        Source            Verification
os-release           /etc/os-release   digest verified
package ownership    rpm database      validated
filesystem metadata  statx             confirmed
```

For SP 800-53A assessment, an Examine object (evidence record) must be sufficient for an
assessor to independently verify the control implementation without additional context. The
"Verification" column strings "digest verified", "validated", "confirmed" are conclusions
without a checkable basis. An assessor reviewing this display cannot determine:

1. What digest algorithm was used
2. Whether the digest was checked against a known-good value or self-referentially
3. Whether the RPM database query succeeded against the actual installed package or a
   stale cache entry

**Recommendation for the Verification column:** The column should display the verification
outcome as a structured code rather than a narrative string. Proposed format:
`OK(sha256)` for a digest-verified source, `OK(rpm-db)` for package database, `FAIL` for
a parse failure, `SKIP` for a source that was probed but skipped for this record.
This gives the assessor a checkable claim (SHA-256 digest was verified) not just a conclusion
(digest verified).

This is an enhancement to `build_trust_rows()` in `main.rs`, not a library change. It does
not require plan modification, but should be captured as an implementation note for the Phase 6
developer.

**Evidence sufficiency gap — no evidence timestamp:**

The current `EvidenceRecord` type (from `umrs-platform`) does not include a timestamp. The
evidence tab will display records without any indication of when they were collected. For CA-7
(continuous monitoring), each evidence record should carry a collection time so an assessor
can determine whether the evidence is current. This is a gap in `umrs-platform`
`EvidenceRecord`, not in the TUI plan. The plan cannot fix it, but the TUI plan should note
it explicitly as a known evidence-sufficiency limitation.

**Evidence sufficiency gap — no evidence integrity indicator:**

There is no display indicator showing whether the evidence bundle as a whole has been
sealed (HMAC-verified from the `SealedCache`). A sealed bundle should carry a visual
indicator (e.g., `[sealed]`) in the evidence tab header to distinguish cached-and-verified
evidence from freshly-collected evidence. Freshly-collected and sealed-cache evidence have
different trust properties for an assessor.

**TwoColumnTable variant naming inconsistency:**

The plan names the variant `TwoColumnTable` but it is described as "a fixed three-column
table row." This is a naming error. A three-column row should be named `ThreeColumnTable`
or `TableRow` to avoid the obvious naming contradiction. The `TwoColumn` variant in Phase 3
is genuinely two-column; the Phase 6 variant is three-column. Having `TwoColumn` and
`TwoColumnTable` in the same enum where the latter is actually three columns creates
confusion for every future reader of this code. The cross-cutting constraints section
(compliance annotations) also refers to `TwoColumnTable` — a rename must be applied
consistently.

---

### Phase 7 — Kernel Security Placeholder Tab

**Positive findings:**

- Placeholder content with `"(not yet probed)"` styled `Dim` is the correct approach.
  Absent data is visually distinct from positive data without suggesting a false state.
- Connecting placeholder content to `SecurityIndicators` values from Phase 1 (using actual
  data where available) is the right approach — partial population is better than entirely
  empty content.

**Operational gap — evidence linkage requirement from feedback:**

The feedback document explicitly states: "Each kernel item must have a corresponding evidence
chain entry." The plan describes placeholder rows but does not plan how future kernel security
items will be linked to evidence records. When this tab is populated, each kernel posture item
(lockdown mode, FIPS state, secure boot) must reference the evidence record (source path,
verification method) that supports it. This is the difference between a display that reads
"FIPS: active" and one that reads "FIPS: active [evidence: /proc/sys/crypto/fips_enabled,
verified]."

**Recommendation:** Before this tab is populated with real data, a design note should be
written specifying how the kernel security tab will reference evidence records. This does not
need to be resolved in Phase 7 (placeholder phase), but the design space needs to be reserved.
One approach: the Kernel Security tab's `GroupTitle` rows carry an optional evidence record
index that links to the corresponding entry in the Trust/Evidence tab.

---

### Dialog API Design

**Positive findings:**

- Caller-owned `Option<DialogState>` (not embedded in `AuditCardState`) is the correct
  ownership model. The library providing the rendering function while the caller manages
  dialog lifetime avoids coupling.
- `DialogMode::SecurityWarning` as a distinct variant from `Error` is operationally
  necessary. A FIPS violation and a "file not found" error require different operator
  responses and visual urgency.
- The explicit `AC-2` citation (session lifecycle — dialog must be dismissed explicitly,
  no auto-dismiss) is correct.

**Operational gap — acknowledgement workflow is undefined:**

The plan describes the dialog mechanics (render, focus, confirm/cancel) but does not describe
the acknowledgement workflow. From the event logging architecture referenced in the task brief,
events require operator acknowledgement. The dialog API must produce a durable record of the
acknowledgement (operator confirmed, timestamp, what was acknowledged). The current `DialogState`
design produces only a transient `Option<bool>` response — once the dialog is dismissed, the
response is gone unless the caller explicitly records it.

**This is a HIGH-severity operational gap for the acknowledgement use case.** An acknowledgement
that leaves no audit trail violates NIST SP 800-53 AU-10 (non-repudiation) and AU-9 (protection
of audit information). The `SecurityWarning` mode specifically implies a security-significant
acknowledgement that must be logged.

**Recommendation:** The dialog API design should require that when `DialogMode::SecurityWarning`
is used, the caller is responsible for emitting an audit log entry when `response == Some(true)`.
The library cannot enforce this (it has no access to the logging subsystem), but the doc comment
on `DialogState` and on `DialogMode::SecurityWarning` must state this requirement explicitly.
A convenience type (`AcknowledgementRecord { message: String, confirmed_at: SystemTime }`) should
be produced by a helper function that the caller invokes when processing a `SecurityWarning`
response. This helper documents the audit trail requirement at the API surface.

**Operational gap — `SecurityWarning` styling ambiguity:**

The plan calls for a future review of whether `SecurityWarning` styling is "visually
unambiguous enough for a DoD operator under time pressure." This review should happen before
implementation, not after. The minimum requirement is: `SecurityWarning` dialogs must be
visually distinct from `Error` dialogs, and both must be distinguishable from `Info` dialogs
even on monochrome terminals (i.e., shape/structure difference, not just color difference).
A border made of `#` characters, or a title bar text like `[!!! SECURITY WARNING !!!]`, gives
structural distinctness beyond color.

**Citation for `DialogState` → `NIST SP 800-53 AC-2`:** The plan correctly cites this.
However, `SecurityWarning` mode should additionally cite `AU-10` (non-repudiation) in its
variant doc comment to make the audit trail requirement discoverable at the API surface.

---

## RMF Methodology Assessment (SP 800-53A)

The plan is reviewed here against SP 800-53A assessment methods to determine whether the
completed interface would support an external assessor.

### Controls Supported by This Interface

| Control | How the interface supports it |
|---|---|
| AU-3 | Header identification (report name, host, subject) on every frame |
| AU-3 (1) | Evidence tab groups records by source kind — structured, not free-form |
| CA-7 | Security indicators in header provide continuous-monitoring visual signal |
| CM-6 | Kernel posture flags reflect actual kernel state (fail-closed on read failure) |
| CM-7 | Lockdown mode indicator shows kernel feature restriction state |
| SI-7 | FIPS and secure boot indicators; trust level display on OS detect card |
| AC-2 | Dialog explicit-dismiss requirement; `AuditCardState.should_quit` clean exit |
| SC-13 | FIPS indicator in header gives immediate visibility of crypto posture |

### Examine Object Sufficiency

SP 800-53A defines three assessment methods: Examine, Interview, and Test. The TUI interface
contributes to the **Examine** method. An external assessor would look at screenshots or
live terminal output from these tools as examination evidence.

For the completed 7-phase implementation to serve as a credible Examine object:

1. The evidence tab must produce structured, verifiable records (Phase 6 gaps above apply).
2. The kernel security tab must be populated with real data, not placeholders, before being
   submitted as assessment evidence (Phase 7 is correctly deferred).
3. The security indicator row must display actual enforcement state, not inferred state
   (Phase 1 fail-closed requirement is correctly specified).
4. An assessor must be able to see what software version collected the evidence. The header
   currently has no tool version field. A supplemental `HeaderField` (Phase 2) carrying
   the crate version would satisfy this.

**Recommended HeaderField for assessors:** `HeaderField::normal("Tool version", env!("CARGO_PKG_VERSION"))` in each binary's `header_fields()` implementation. This is not
a library requirement — it is a binary-level implementation note. But it is a requirement
for the output to serve as an SP 800-53A Examine object with sufficient identification.

---

## Missing Capabilities — Assessor Perspective

The following capabilities are absent from the 7-phase plan and are needed for full
assessment utility. These are not defects in the plan — the plan correctly scopes its
deliverables. They are recorded here as future work.

### F-1: Finding Severity Indicators

Current evidence and observation displays do not distinguish severity levels on individual
evidence items. The `file_stat` binary's Observations tab has `ObservationKind::Risk` vs.
`ObservationKind::Warning` — this is the right pattern. The OS Detection evidence tab has no
equivalent severity indicator on individual evidence records. An assessor scanning the
evidence tab cannot quickly identify which records indicate failure vs. advisory conditions.

**Proposed addition:** `StyleHint::TrustRed` on `DataRow::table_row(...)` rows where
`rec.parse_ok == false` — already supported by the `style_hint` field on `TwoColumnTable`.
This is an implementation note for Phase 6, not a new library feature.

### F-2: Trend / Comparison Across Runs

The plan produces a snapshot view. For CA-7 (continuous monitoring), an assessor needs to
know whether posture has changed between observation periods. The current design has no
mechanism for displaying deltas (e.g., "FIPS: active [was: unavailable at last run]").
This requires persistent state storage — outside the scope of the TUI enhancement plan but
worth capturing as a future goal under G4 (Assessment Engine).

### F-3: Export / Report-to-File

An assessor viewing a TUI cannot extract evidence programmatically. A future capability to
export the current audit card state as structured data (JSON, OSCAL fragment) would allow
UMRS output to serve as a direct input to a OSCAL-based SAR. This aligns with G4. It is out
of scope for this plan.

### F-4: Cross-Card Posture Summary

When an operator is running multiple audit cards (OS detect, file stat, kernel security),
there is no aggregate posture view. A future "summary card" showing combined trust level
across all assessed components would serve the CA-7 ongoing assessment goal. Out of scope.

### F-5: Timestamp Display

No audit card currently displays the time at which detection was run. For assessment
purposes, every Examine object must be timestamped. The main binary uses a `DetectionResult`
that does not carry a timestamp (this is an `umrs-platform` gap, noted in the evidence
timestamp observation above). Until `EvidenceRecord` carries timestamps, the header should
at minimum display the card's render start time (when the binary was invoked). This is a
two-line change in each binary: capture `SystemTime::now()` before detection runs, display
it as a `HeaderField`.

---

## Specific Plan Corrections Required

The following items are direct corrections to the plan document, not implementation guidance.

**Correction 1:** Phase 1, `IndicatorValue` doc comment — cite `NIST SP 800-53 SI-7, CM-6`
not `SI-3, CM-6`. SI-3 is malicious code protection. It does not apply to kernel configuration
indicator display.

**Correction 2:** Phase 6, `TwoColumnTable` variant name — rename to `ThreeColumnTable` or
`TableRow`. The variant is three columns. Naming it `TwoColumnTable` creates permanent
confusion in the codebase.

**Correction 3:** Phase 6, `TwoColumnTable` variant has a `style_hint` field but
`TableHeader` does not. This is asymmetric. A table header row with an error style
(e.g., when the evidence group itself failed to load) is a valid use case. Add `style_hint`
to `TableHeader` or document why header rows are always unstyled.

**Correction 4:** Dialog API, `SecurityWarning` variant doc comment — add `NIST SP 800-53
AU-10` citation and a note requiring the caller to emit an audit log entry when the operator
confirms a `SecurityWarning` dialog.

---

## Summary of Operational Issues (by severity)

| ID | Severity | Phase | Issue |
|---|---|---|---|
| O-1 | HIGH | Dialog | `SecurityWarning` acknowledgements produce no audit trail. AU-10 violation if no durable record is created by the caller. |
| O-2 | HIGH | Phase 1 | `IndicatorValue` doc comment cites SI-3 (malware); correct citation is SI-7, CM-6. |
| O-3 | HIGH | Phase 6 | `TwoColumnTable` variant is named for two columns but is three columns. Permanent naming confusion. |
| O-4 | MEDIUM | Phase 1 | SELinux indicator population: `0` must display as `Inactive("permissive")`, not `Active("0")`. Enforcing vs. permissive is operationally critical. |
| O-5 | MEDIUM | Phase 5 | `indicator_inactive` and `indicator_unavailable` share `DarkGray` styling. Unavailable is a distinct security concern from inactive; it needs a distinct color. |
| O-6 | MEDIUM | Phase 6 | Evidence verification column uses narrative strings ("digest verified"). Assessors need structured codes (e.g., `OK(sha256)`) for independent verification. |
| O-7 | MEDIUM | Phase 6 | No evidence bundle seal indicator. Cached-and-sealed vs. freshly-collected evidence have different assessor trust properties. |
| O-8 | LOW | Phase 7 | Evidence linkage from kernel security tab items to evidence records is undefined. Required before real data is added to this tab. |
| O-9 | LOW | All phases | No tool version field in header. Required for audit card to serve as a credible SP 800-53A Examine object. |
| O-10 | LOW | All phases | No detection run timestamp on any card. Every Examine object must be timestamped. |
| O-11 | LOW | Phase 1 | Secure boot indicator cannot be reliably populated via compile-time kattr path (GUID suffix in efivars path). Implementation note needed. |

---

## Conclusion

The plan is well-designed and the architectural decisions (enum-based `DataRow`, fail-closed
`IndicatorValue`, stateless rendering, caller-owned dialog state) are all correct. The primary
operational concerns are:

1. The `SecurityWarning` dialog must document the AU-10 audit trail requirement at the API
   surface — the library cannot enforce it but must make the requirement undeniable to every
   future caller.
2. The `TwoColumnTable` naming error must be corrected before implementation to prevent
   permanent technical debt.
3. The SI-3 citation on `IndicatorValue` must be corrected to SI-7.
4. The SELinux indicator must distinguish permissive from enforcing — this is operationally
   critical and the distinction is supported by the existing `IndicatorValue` design.

All four of these are resolvable before Phase 1 implementation begins. The remaining
medium/low findings are implementation guidance for the developer and future-phase considerations
for the Assessment Engine (G4).
