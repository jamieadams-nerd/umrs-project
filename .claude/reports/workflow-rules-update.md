# Workflow Rules Update Report

```
Date: 2026-03-14
Author: security-engineer
Source reports reviewed:
  - .claude/reports/security-engineer-posture-2a-review.md
  - .claude/reports/2026-03-14-security-auditor-umrs-platform-audit.md
  - .claude/reports/tech-writer-module-comments-review.md
```

---

## Purpose

Three review reports from 2026-03-14 exposed recurring issue classes in `umrs-platform` that
are not prevented by any existing workflow rule. This report records what rules were added or
updated, which report findings motivated each change, and what future violations each rule is
designed to prevent.

---

## Files Modified

| File | Change type |
|---|---|
| `.claude/rules/high_assurance_pattern_rules.md` | Updated 1 existing rule; added 2 new rules |
| `.claude/rules/rust_design_rules.md` | Added 2 new rules |

---

## Changes to `.claude/rules/high_assurance_pattern_rules.md`

### 1. Must-Use Contract Rule — enforcement note added

**What changed**: Added a bullet point to the existing rule:

> **Enforcement note**: `clippy::must_use_candidate` does not enforce message strings. The
> project relies on code review and the security-auditor agent to catch bare `#[must_use]`
> annotations. When reviewing new code, verify every `#[must_use]` has a descriptive message.

**Motivated by**: Security-auditor report findings F-06, F-07, F-08, F-09. The audit found
five public items in `umrs-platform` — including `SecureReader::read()` (a HIGH finding) —
with either missing `#[must_use]` entirely or bare `#[must_use]` without a message string.
These slipped through because Clippy's `must_use_candidate` lint only flags missing annotations;
it does not enforce that the annotation carries a message. The rule previously stated the
requirement but did not explain the enforcement gap.

**What it prevents**: Future new public functions or types being merged with bare `#[must_use]`
or no annotation, relying incorrectly on Clippy to catch the gap.

---

### 2. Signal Evaluation Path Rule — new rule

**What changed**: New rule added at the end of the file.

**Motivated by**: Security-engineer review Finding 1 (HIGH). The `evaluate_configured_meets()`
function in `contradiction.rs` unconditionally parses `ConfiguredValue.raw` as a `u32`. When
four blacklist signals were added with `raw = "blacklisted"` (a sentinel string set by
`blacklist_configured_value()`), no dedicated evaluation path was implemented and no test
covered the full contradiction detection path for these signal types. The result: contradiction
detection was silently disabled for all four DMA-capable module blacklist signals. The signals
appeared to function correctly (they were read, stored in SignalReport) but `BootDrift` could
never be emitted regardless of system state.

**What it prevents**: Future `SignalId` variants whose `ConfiguredValue.raw` is a non-integer
string sentinel bypassing contradiction detection with no compile-time or test-time signal.

---

### 3. Debug Log Information Discipline Rule — new rule

**What changed**: New rule added at the end of the file.

**Motivated by**: Security-engineer review Finding 2 (MEDIUM). `modprobe.rs` emitted full
module parameter values (`{module} {param}={value}`) in a `log::debug!()` line during the
modprobe.d merge pass. The same discipline — log the key but suppress the value — was already
correctly applied in `configured.rs` with an explicit comment explaining why. The discipline
was not applied consistently to the new file. On DoD/CUI deployment systems where debug logging
is enabled during troubleshooting, this log line exposes the full parameter-value mapping of the
system's modprobe configuration.

**What it prevents**: New source files that handle configuration data emitting raw config
values into debug logs without a deliberate decision to suppress them.

---

## Changes to `.claude/rules/rust_design_rules.md`

### 4. Citation Format Rule — new rule

**What changed**: New rule added at the end of the file.

**Motivated by**: Tech-writer report findings across approximately 30 citation occurrences.
The review found two citation forms in use simultaneously across `umrs-platform`: `NIST SP 800-53`
(canonical, with `SP`) and `NIST 800-53` (abbreviated, without `SP`). The inconsistency was
identified in `lib.rs`, `kattrs/*.rs`, and most `posture/*.rs` modules. The `SP` (Special
Publication) designator is the correct abbreviated form. The inconsistency is a documentation
quality issue that would be flagged by a DoD auditor reviewing source-embedded control citations.

The rule also codifies forms for NSA RTB, NIST SP 800-218 SSDF, FIPS, and CMMC citations.
It explicitly scopes the requirement to doc comments only, allowing runtime output strings
(e.g., `nist_controls` fields in catalog entries) to use abbreviated forms for display
compactness — this preserves the existing catalog data format.

**What it prevents**: New source files adding citation annotations in inconsistent forms,
requiring a sweep correction at the next audit cycle.

---

### 5. Internal Reference Prohibition Rule — new rule

**What changed**: New rule added at the end of the file.

**Motivated by**: Tech-writer report findings on `posture/configured.rs` (comment referencing
"Finding 3" from a security review) and a parallel finding on `os_identity.rs` (referencing
"ANSSI Finding 1"). Both inline comments referenced internal review artifacts that have no
meaning to a reader without access to those documents. The tech-writer report specifically
identified that the `configured.rs` reference should be replaced with a self-contained
explanation of the normalization logic.

This is a security posture concern, not a purely stylistic one: doc comments are the audit
trail embedded in the codebase. A comment that says "per Finding 3" is not auditable without
the finding. A comment that explains the normalization rationale directly is auditable by
any qualified reviewer.

**What it prevents**: Design rationale from review sessions being written into doc comments
as opaque references rather than as self-contained technical justifications.

---

## Existing Rule Inconsistencies Noted (Not Modified)

Two existing rules in `high_assurance_pattern_rules.md` use citation forms that now conflict
with the new Citation Format Rule in `rust_design_rules.md`:

| Rule | Current citation | Correct form |
|---|---|---|
| Validate at Construction Rule | `NIST 800-218 SSDF PW.4.1` | `NIST SP 800-218 SSDF PW.4.1` |
| Trust Gate Rule | `NIST 800-53 CM-6` | `NIST SP 800-53 CM-6` |
| Security Findings as Data Rule | `NIST 800-53 AU-3` | `NIST SP 800-53 AU-3` |

These citations are in workflow rule prose, not in Rust doc comments. The Citation Format Rule
applies to `///` and `//!` doc comments. The rule files themselves are not subject to this
constraint. No change was made to these lines.

---

## Traceability

| Rule change | Source report | Finding |
|---|---|---|
| Must-Use Contract Rule — enforcement note | security-auditor-umrs-platform-audit | F-06, F-07, F-08, F-09 |
| Signal Evaluation Path Rule (new) | security-engineer-posture-2a-review | Finding 1 (HIGH) |
| Debug Log Information Discipline Rule (new) | security-engineer-posture-2a-review | Finding 2 (MEDIUM) |
| Citation Format Rule (new) | tech-writer-module-comments-review | Global finding: ~30 citation occurrences |
| Internal Reference Prohibition Rule (new) | tech-writer-module-comments-review | configured.rs "Finding 3" reference; os_identity.rs "ANSSI Finding 1" reference |
