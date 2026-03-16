---
name: Platform API Enrichment — Narrative Control at the Library Level
path: components/rusty-gadgets/umrs-platform
agent: rust-developer
status: draft — awaiting Jamie review
depends-on: umrs-platform-expansion.md
---

# Platform API Enrichment Plan

## Problem

`umrs-platform` provides typed, provenance-verified answers about the system —
but it does not explain what those answers *mean*. Human-readable descriptions,
display labels, and semantic categorisation live in the TUI binary (`umrs-tui`),
not in the library. This creates three problems:

1. **Narrative drift** — every consumer (CLI, TUI, GUI, `--json`, assessment
   reports) must independently write explanations. Misinterpretation is inevitable.
2. **Duplication** — `trust_level_label()`, `distro_label()`, `family_label()`,
   `source_kind_label()`, `format_live_value()` etc. are pure data transforms
   with no rendering dependency, yet they live in the TUI binary.
3. **Missing "what"** — the posture catalog has `rationale` (why the hardened
   value matters) but no `description` (what the signal IS in plain English).
   An operator sees `kernel.unprivileged_bpf_disabled` and has no library-level
   explanation of what BPF is or why it matters.

**Jamie's principle:** Control the narrative at the library level. Definitions
belong where the data is defined. A downstream developer writing a TUI tool
should not have to interpret what `SpectreV2Off` means — the library should
tell them.

## Goals

- Every typed value in `umrs-platform` that has a human-facing name carries
  its own `label()` and `description()` methods.
- Signal grouping is a first-class concept in the posture module, not a
  display-layer hardcoding.
- The TUI becomes a thin rendering layer that calls platform API methods
  for all human-readable text.

**ROADMAP alignment:** G1 (Platform Awareness), G5 (Security Tools),
G8 (High-Assurance Patterns)

---

## Audit: TUI Functions That Belong in umrs-platform

### Category 1 — Pure label/description helpers (move to platform)

These functions match on platform types and return `&'static str`. They have
zero rendering dependencies. They belong as methods on the types they describe.

| TUI function | File | Platform type | Proposed API |
|---|---|---|---|
| `trust_level_label(TrustLevel)` | main.rs:60 | `TrustLevel` | `TrustLevel::label(&self) -> &'static str` |
| `trust_level_description(TrustLevel)` | main.rs:70 | `TrustLevel` | `TrustLevel::description(&self) -> &'static str` |
| `source_kind_label(&SourceKind)` | main.rs:100 | `SourceKind` | `SourceKind::label(&self) -> &'static str` |
| `family_label(&OsFamily)` | main.rs:146 | `OsFamily` | `OsFamily::label(&self) -> &'static str` |
| `distro_label(&Distro)` | main.rs:132 | `Distro` | `Distro::label(&self) -> &'static str` |
| `os_name_from_release(Option<&OsRelease>)` | main.rs:122 | `OsRelease` | `OsRelease::display_name(&self) -> String` |
| `format_live_value(&LiveValue)` | main.rs:780 | `LiveValue` | Already has `Display` impl — verify `Bool` maps to "enabled"/"disabled" there |
| `indicator_to_display(&IndicatorValue)` | main.rs:813 | `IndicatorValue` | `IndicatorValue::label(&self) -> &str` (text only; style stays in TUI) |
| `label_trust_display(&LabelTrust)` | main.rs:915 | `LabelTrust` | `LabelTrust::label(&self) -> &'static str` + `LabelTrust::description(&self) -> &'static str` |

### Category 2 — Style/hint mappers (stay in TUI)

These map platform values to `StyleHint` — a TUI-specific type. The *mapping
logic* is presentation policy. These stay in the TUI.

| TUI function | Reason to keep in TUI |
|---|---|
| `trust_level_hint(TrustLevel) -> StyleHint` | StyleHint is a rendering concept |
| `meets_desired_hint(Option<bool>) -> StyleHint` | StyleHint is a rendering concept |
| `evidence_style_hint(&EvidenceRecord) -> StyleHint` | StyleHint is a rendering concept |

### Category 3 — Data assembly (stay in TUI, consume platform API)

These build display structures (`Vec<DataRow>`) from platform data. They stay
in the TUI but should become thinner once platform types carry their own labels.

| TUI function | Notes |
|---|---|
| `build_os_info_rows()` | Calls label helpers — will simplify when labels move to platform |
| `build_trust_rows()` | Same |
| `build_kernel_security_rows()` | Heavy — signal grouping logic should move to platform (see below) |
| `append_signal_group()` | Iterates signals by hardcoded group — should use platform grouping |
| `signal_group_rows()` | Same |
| `append_grouped_evidence()` | Groups evidence by source kind — `SourceKind::label()` simplifies this |

### Category 4 — Data collection (move to platform)

These read system state but live in the TUI binary. They duplicate or
bypass platform-layer reads.

| TUI function | File | Issue | Proposed fix |
|---|---|---|---|
| `read_security_indicators()` | indicators.rs:82 | Reads SELinux, FIPS, lockdown directly — duplicates kattrs | Should consume `PostureSnapshot` or kattrs API |
| `read_selinux_status()` | indicators.rs:195 | Direct kattr read | Use `SelinuxEnforce` from kattrs |
| `read_fips_mode()` | indicators.rs:214 | Direct kattr read | Use `ProcFips` from kattrs |
| `read_lockdown_mode()` | indicators.rs:229 | Direct kattr read | Use `KernelLockdown` from kattrs |
| `read_boot_id()` | indicators.rs:313 | Direct procfs read | Use `BootId` from kattrs |
| `read_system_uuid()` | indicators.rs:344 | Direct sysfs read | Needs platform-level type |

---

## Gaps: What umrs-platform Should Provide But Doesn't

### Gap 1 — Signal descriptions (the "what")

`SignalDescriptor` has `rationale` but no `description`. Every signal needs:

```rust
pub struct SignalDescriptor {
    // ... existing fields ...
    /// Plain-English explanation of what this signal controls.
    /// Suitable for display in TUI help overlays, --json output,
    /// assessment reports, and operator documentation.
    pub description: &'static str,
    // ... existing fields ...
}
```

Examples:

| Signal | description (what it is) | rationale (why the value matters) |
|---|---|---|
| `KptrRestrict` | "Controls whether kernel memory addresses are visible in /proc and other interfaces. Kernel pointers help attackers locate exploit targets." | "Level 2 blocks kernel pointer exposure..." |
| `SpectreV2Off` | "Spectre v2 uses branch predictor injection to leak data across process and kernel boundaries. The kernel mitigates this with retpoline or IBRS." | "Explicitly disabling Spectre v2 mitigation exposes..." |
| `NoSmtOff` | "Simultaneous Multithreading (SMT/Hyper-Threading) runs two threads per physical core. SMT enables cross-thread data leakage attacks (MDS, L1TF)." | "Re-enabling SMT weakens..." |
| `CorePattern` | "Determines where the kernel writes process memory dumps on crash. A piped handler (|) routes dumps through a controlled program; a raw path writes directly to disk." | "A core_pattern beginning with | routes dumps..." |

### Gap 2 — Signal grouping as a typed concept

The TUI hardcodes 6 groups as string labels with manual signal arrays. This should
be a `SignalGroup` enum in `umrs-platform`:

```rust
pub enum SignalGroup {
    BootIntegrity,
    CryptographicPosture,
    KernelSelfProtection,
    ProcessIsolation,
    FilesystemHardening,
    ModuleRestrictions,
}
```

With:
- `SignalGroup::label(&self) -> &'static str` — display name
- `SignalGroup::description(&self) -> &'static str` — group-level explanation
- `SignalDescriptor::group: SignalGroup` — each signal belongs to a group
- `PostureSnapshot::by_group() -> BTreeMap<SignalGroup, Vec<&SignalReport>>` — iteration by group

### Gap 3 — TrustLevel narrative

`TrustLevel` has no methods — `T1 — KernelAnchored` label and description
live only in TUI. These are platform-level definitions that any consumer needs.

### Gap 4 — EvidenceRecord display helpers

`evidence_verification_str()` produces "✓ ok (fd)" / "✗ FAIL (path)" —
this is the canonical display format for evidence verification and should be
a method on `EvidenceRecord`, not a TUI function.

### Gap 5 — IndicatorValue lacks a label method

`IndicatorValue::Active("enforcing")` carries the value but no standard
label or display method. The TUI manually matches and clones strings.

### Gap 6 — Distro and OsFamily lack Display or label methods

`Distro::Rhel` → "RHEL" mapping is TUI-only. This is platform knowledge.

---

## Implementation Phases

### Phase 1 — Signal enrichment (high priority)

- Add `description: &'static str` to `SignalDescriptor`
- Add `SignalGroup` enum with `label()` and `description()`
- Add `group: SignalGroup` to `SignalDescriptor`
- Populate descriptions for all 36 signals
- Add `PostureSnapshot::by_group()` iterator
- IRS reviews descriptions for accuracy

### Phase 2 — Platform type labels (medium priority)

- Add `label()` and `description()` methods to:
  - `TrustLevel`
  - `SourceKind`
  - `OsFamily`
  - `Distro`
  - `LabelTrust`
- Add `OsRelease::display_name()` method
- Add `EvidenceRecord::verification_display()` method
- Add `IndicatorValue::label()` method

### Phase 3 — TUI simplification (after Phase 1-2)

- Replace TUI label helpers with platform method calls
- Replace hardcoded signal groups with `PostureSnapshot::by_group()`
- Remove duplicated data-collection functions from indicators.rs
  (consume kattrs/posture API instead)
- TUI becomes a thin rendering layer

### Phase 4 — JSON output benefit

- `--json` output automatically gets human-readable descriptions
  because the platform types carry them
- Assessment engine gets signal descriptions for evidence reports
- No separate "explanation layer" needed

### Phase 5 — i18n readiness

The `description` and `rationale` strings are prime i18n candidates — they are
the user-facing text that a francophone Five Eyes operator would need translated.

**How this fits the existing i18n architecture:**

- Libraries return typed values with English source strings (unchanged)
- Tool binaries (umrs-ls, umrs-state, umrs-tui) wrap those strings with `gettext!()`
  at the rendering boundary
- The English source strings in `SignalDescriptor.description` and `.rationale`
  become the `.pot` extraction source — one canonical definition, N translations
- Same applies to `TrustLevel::description()`, `SignalGroup::description()`,
  `LabelTrust::description()`, etc.

**What this means for string design:**

- Keep descriptions as complete, self-contained sentences — translators need
  full context, not sentence fragments
- Avoid interpolation in descriptions where possible — `"Controls whether
  kernel memory addresses are visible"` translates cleanly; `format!("Controls
  {thing}")` does not
- The `label()` methods (short names like "RHEL", "T1 — KernelAnchored") may
  or may not need translation — technical identifiers often stay in English
  even in French Five Eyes contexts. The `description()` methods always need it.

**Coordination:** Simone (umrs-translator) extracts strings after Phase 2 lands.
No i18n wrapping happens until the English strings are reviewed and stable.

---

## Principles

1. **The library defines the narrative.** Consumers render it.
2. **Every typed value answers "what is this?" and "why does it matter?"**
3. **Group membership is compile-time.** No runtime string matching.
4. **Descriptions are authoritative.** Reviewed by IRS for accuracy.
5. **TUI stays thin.** Map platform labels → rendering styles. Nothing else.
6. **Strings are i18n-ready.** Complete sentences, no fragments, no embedded
   interpolation. The library provides English source strings; tool binaries
   own the translation boundary.

---

## Compliance

- NIST SP 800-53 AU-3: Audit record content — human-readable descriptions
  in the library make audit output self-documenting
- NIST SP 800-53A CA-2: Assessment clarity — signal descriptions support
  operator and auditor comprehension without external reference
- NIST SP 800-53 SI-11: Error Handling — standardised labels prevent
  inconsistent error descriptions across consumers

---

## Developer Guide Documentation

This pattern — "what/why at the library level" — must be documented in
`docs/modules/devel/` so future contributors understand the design:

1. **Why platform types carry descriptions** — narrative control, single source
   of truth, prevents consumer misinterpretation of security concepts
2. **The what/why split** — `description` answers "what is this thing?" for
   operators; `rationale` answers "why does the hardened value matter?" for
   auditors. Both are needed. Both are authoritative.
3. **Where labels live** — if it's a fact about the data, it's a platform method.
   If it's a colour or a layout decision, it's a TUI concern.
4. **i18n boundary** — libraries provide English source strings; tool binaries
   own the `gettext!()` wrapping. Translators work from `.pot` files extracted
   from the binary crate, but the canonical English lives in the library.
5. **Signal grouping rationale** — groups are security domains, not display
   categories. They carry their own descriptions because an assessment report
   needs to explain what "Kernel Self-Protection" means without a TUI.

Tech-writer picks this up after Phase 2 implementation stabilises.

---

## Definition of Done

- [ ] Phase 1: `SignalDescriptor.description` and `SignalGroup` implemented
- [ ] Phase 1: All 36 signals have descriptions reviewed by IRS
- [ ] Phase 2: Platform types carry `label()` / `description()` methods
- [ ] Phase 3: TUI label helpers removed, replaced with platform API calls
- [ ] Phase 3: TUI indicators.rs data collection replaced with platform API
- [ ] `cargo xtask clippy && cargo xtask test` clean
- [ ] `--json` output includes descriptions automatically

---

## DO NOT CHANGE ANY CODE RIGHT NOW

This is a planning document. Implementation begins after Jamie approves.
