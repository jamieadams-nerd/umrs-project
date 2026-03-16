---
name: Three-Layer Evaluation Model Advisory
description: Security-auditor analysis of the proposed live/configured/expected three-layer
  kernel security assessment model. Covers methodology alignment, finding taxonomy
  completeness, expected-value governance, integration path with ContradictionKind,
  G4 assessment engine implications, and risks.
type: project
date: 2026-03-15
---

# Three-Layer Kernel Security Evaluation Model — Advisory Analysis

**Prepared by:** security-auditor (The IRS)
**Date:** 2026-03-15
**Status:** Advisory only — no code changes recommended here. Recommendations are
flagged for the rust-developer (implementation) and tech-writer (documentation).

---

## 1. Is the Model Correct from an Assessment Methodology Perspective?

**Short answer: Yes, and it maps cleanly to SP 800-53A.**

SP 800-53A Rev 5 defines three assessment methods:

- **Examine** — review specifications, policies, procedures, plans, and documentation
- **Interview** — conduct discussions with personnel
- **Test** — exercise a mechanism, process, or activity and observe results

The three-layer model maps directly to Examine and Test:

| Layer | 800-53A Analog | What UMRS reads |
|---|---|---|
| Live | **Test** — exercise the mechanism | `/proc/sys/*`, `/sys/kernel/security/`, `/proc/cmdline` |
| Configured | **Examine** — review the specification | `/etc/sysctl.d/`, `/etc/modprobe.d/`, BLS entries |
| Expected | **Examine** — review policy | Catalog `desired` field; future: OSCAL baseline |

SP 800-53A's CM-6 assessment procedure (CM-06-a, CM-06-b) explicitly calls for:

1. Examining configuration management policy and configuration settings
2. Testing the configuration settings to determine whether they are implemented as stated

The contradiction detection (Live vs. Configured) satisfies the "testing" step. The control-gap
check (Live/Configured vs. Expected) satisfies the "examining" step against a stated baseline.

This model is methodologically sound. It is what a human assessor would do manually, mechanized.

---

## 2. Are the Three Finding Types Sufficient?

The three types as proposed (Contradiction, Control Gap, Clean) are correct but **incomplete**.
There are additional states the current taxonomy does not name. Some are already handled
implicitly by the existing `ContradictionKind` enum; others are entirely new.

### 2a. States Already Handled (but unnamed in the proposal)

The current `ContradictionKind` enum captures three sub-types within "Contradiction":

- `EphemeralHotfix` — live hardened, configured not hardened (will regress on reboot)
- `BootDrift` — configured hardened, live not hardened (config failed to apply)
- `SourceUnavailable` — configured value present but live value unreadable

These are not interchangeable. `BootDrift` is a higher-severity finding than `EphemeralHotfix`
because it means the running system is softer than intended, not just that a manual fix will be
lost. The taxonomy in the design notes collapses these into a single "Contradiction" bucket.
That is acceptable for display purposes at the summary level, but the underlying
`ContradictionKind` variant must be preserved and surfaced to operators — the variant is
the actionable detail.

### 2b. States Missing from the Proposal

**Missing state 1: Unverifiable**

Live value could not be read AND no configured value exists. This is distinct from
`SourceUnavailable` (which requires a configured value to be present). An unverifiable signal
is neither Clean nor a Contradiction — it is simply unknown. Example: `kernel.modules_disabled`
on a kernel built without that sysctl. The correct operator message is "cannot assess" not
"clean."

Current `SignalReport` represents this as `live_value: None, configured_value: None,
meets_desired: None, contradiction: None` — the Findings tab will need to distinguish this
from a genuinely Clean result, or it risks silently hiding coverage gaps.

**Recommendation:** Name this state explicitly. Proposed: `Unverifiable`. A signal is
`Unverifiable` when `live_value` is `None` and `configured_value` is `None`.

**Missing state 2: Configured Absent (Implicit Reliance)**

Live value meets the expected baseline, but there is no configuration file that explicitly
sets it. This means the system is relying on kernel defaults for a security-critical parameter.
On RHEL 10, many kernel defaults are already hardened — but an assessor cannot distinguish
"hardened by explicit policy" from "hardened by luck." This is a gap under CM-6, which requires
explicit configuration of security settings, not reliance on defaults.

Example: `fs.protected_symlinks` is 1 by default on RHEL 10. If no sysctl.d entry exists,
the signal is `Clean` in the proposed model, but a real CM-6 assessment would note that there
is no explicit configuration documenting the intended value. An operator removing
`/etc/sysctl.d/` files would not know this default is security-relevant.

**Recommendation:** Consider a `ReliesOnDefault` sub-classification for signals where
`live_meets = Some(true)`, `meets_desired = Some(true)`, and `configured_value = None`.
This is a soft finding — not a contradiction, not a control gap — but worth surfacing for
CM-6 compliance evidence purposes. A DoD assessor will ask "show me your configuration
baseline" and a system with no sysctl.d entries fails that ask even if the defaults are right.

**Missing state 3: Clean But Under-Specified Expected**

The `DesiredValue::Custom` variant in the catalog (used for `kernel.sysrq`) means the
expected-layer check is undefined. A signal evaluated as `Custom` cannot produce a Control Gap
finding because there is no numerical expectation to compare against. This is correct by design,
but it means the tool silently under-assesses any signal using `Custom`. The operator cannot
tell the difference between "no finding" and "we don't know how to check this."

**Recommendation:** Signals with `DesiredValue::Custom` that are not explicitly evaluated
by a custom validator should produce a distinct `PolicyDependent` state rather than silently
passing through. This signals to the operator: "This setting requires site-specific policy
review and we have not automated the check."

### 2c. Summary of Proposed Extended Taxonomy

| Finding Type | Condition | Severity Hint |
|---|---|---|
| Clean | live = configured = expected | None |
| EphemeralHotfix | live hardened, configured not | Medium (will regress) |
| BootDrift | configured hardened, live not | High (currently soft) |
| ControlGap | live = configured, both fail expected | High |
| SourceUnavailable | configured present, live unreadable | Medium |
| Unverifiable | live unreadable, no configured value | Low/Informational |
| ReliesOnDefault | live meets expected, no configured value | Low (CM-6 gap) |
| PolicyDependent | `DesiredValue::Custom`, no validator | Informational |

---

## 3. Where Do the "Expected" Values Come From?

### 3a. Current Answer: The Catalog

Right now, the `desired` field in each `SignalDescriptor` in `catalog.rs` IS the expected
layer. These values are what the security-auditor agent has defined as the hardened baseline.
They are compile-time constants, compiler-verified, and annotated with NIST controls. This is
correct and appropriate for a Phase 1 tool.

However, "expected" is not a single thing in the real assessment world. There are at least
three distinct sources of expectation:

1. **Hardening baseline** (current): "what a well-configured system should have" — e.g.,
   `kernel.kptr_restrict = 2`. This is the catalog's `desired` field.

2. **Organizational baseline** (future): the organization's SSP-documented configuration
   baseline, which may be more conservative or permissive than the generic hardening guide.
   Example: a site may permit `kernel.yama.ptrace_scope = 0` for a development environment
   while the hardening guide says `>= 1`.

3. **Regulatory floor** (future): the minimum required by a specific framework (CMMC Level 2,
   DISA STIG, FedRAMP Moderate). These may differ from both the hardening baseline and the
   org baseline.

The current design correctly implements source #1. Sources #2 and #3 are the OSCAL territory.

### 3b. OSCAL Mapping

OSCAL (Open Security Controls Assessment Language) defines three relevant document types:

- **Profile**: a customized selection of controls from a catalog (e.g., "we implement NIST
  800-53 Rev 5 Moderate baseline with these parameter values")
- **Component Definition**: a machine-readable description of what a software component
  provides (this is what UMRS could emit)
- **Assessment Plan / Assessment Results**: structured records of what was tested and what
  was found

The "expected" layer maps to the OSCAL Profile: specifically, the `set-parameter` elements
within a profile that bind specific values to control parameters. For CM-6, the profile would
specify `cm-6_prm_1` (the list of configuration settings to document) and `cm-6_prm_2`
(the frequency of review).

**Practical implication for UMRS:** The catalog's `desired` field is the internal baseline.
For G4 to produce assessment reports that satisfy a formal OSCAL-based assessment, the tool
will need a way to load an external profile that overrides or supplements the catalog's expected
values. This is a design decision for Phase 4 or later.

**Who defines the values today?** The security-auditor agent (this role). The catalog entries
were written with reference to DISA STIG V1R3 for RHEL 8/9, NSA RHEL Hardening Guide, and
CIS Benchmarks. The `nist_controls` field in each `SignalDescriptor` provides the
traceability back to specific controls.

### 3c. Governance Recommendation

Before G4 ships with automated finding generation, the expected values in the catalog must
be formally reviewed and approved against at least one named baseline (DISA STIG, CIS Level 2,
or NSA RHEL guide). The catalog commit that establishes those values should reference the
baseline version and date. Without that, an automated finding report carries no named
authority — it is the tool's opinion, not a recognized standard.

This is an important distinction: a **deviation finding** (we expected X, we found Y) is only
meaningful if the "expected" value has a named, version-stamped authority behind it.

---

## 4. How Does This Feed into G4 (Assessment Engine)?

The three-layer model is the correct foundation for G4. Here is how the data flows:

```
PostureSnapshot
  └── Vec<SignalReport>
        ├── live_value       ─── Layer 1
        ├── configured_value ─── Layer 2
        ├── meets_desired    ─── Expected layer (Layer 3) comparison result
        └── contradiction    ─── ContradictionKind (Contradiction finding)
```

G4 needs to add:

1. **Control-gap classification**: a `SignalReport` where `meets_desired = Some(false)` AND
   `contradiction = None` is a control gap. There is currently no typed field for this
   distinction — `findings()` returns all `meets_desired = Some(false)` reports including
   those that also have a contradiction. G4 will need to classify each report into exactly
   one of: Clean, Contradiction (with kind), ControlGap, or one of the missing states above.

2. **A `FindingRecord` type**: the `SignalReport` is the raw data; G4 needs a derived type
   that represents a finding suitable for display and export. This type should carry:
   - The `SignalId` and display label
   - The finding classification (enum)
   - The live and configured values (for display)
   - The expected value (from the catalog `desired` field, human-readable)
   - The `nist_controls` citation from the descriptor
   - The `AssuranceImpact` tier
   - A severity (which may differ from the impact tier based on finding type)

3. **Deduplication**: a signal can simultaneously have a `ContradictionKind` and fail
   `meets_desired`. For display purposes, the contradiction is the primary finding (it is more
   actionable). A finding where both are true should not count as two findings.

4. **Aggregation**: G4 needs summary counts by finding type, by impact tier, and by
   control family. These are the numbers an assessor reports.

5. **Stable finding IDs**: for assessment continuity, each finding needs a stable identifier
   (not a row number) so that a finding in snapshot N can be tracked to snapshot N+1.
   The `SignalId` enum variant serves this purpose — it is compile-time stable.

**Assessment engine design note**: G4 should NOT re-implement the probe logic. The correct
architecture is: `PostureSnapshot::collect()` is the probe; G4 is the classifier and reporter
that operates on the snapshot. This maintains the layered separation rule.

---

## 5. NIST Control Alignment

The three-layer model satisfies or supports the following controls:

### CM-6 — Configuration Settings (Primary)

**Layer 2 (Configured)** directly implements CM-6. The control requires that:
- Configuration settings are established, documented, and implemented
- Deviations from established settings are documented with justification

Contradiction detection (Live vs. Configured) is the automated check for "implemented as
documented." Control gap detection (Live/Configured vs. Expected) is the automated check
for "settings reflect security requirements."

CM-6 enhancement CM-6(1) requires automated mechanisms to document, verify, and enforce
configuration settings. The three-layer probe is an implementation of CM-6(1) for the kernel
attack surface.

### CA-7 — Continuous Monitoring

**All three layers together** implement CA-7. The control requires ongoing monitoring of
security controls. The posture probe is the monitoring mechanism; the snapshot is the
monitoring record; the finding types are the alert conditions.

CA-7 enhancement CA-7(4) (risk monitoring) requires correlating security control monitoring
with risk assessment. The `AssuranceImpact` tier in each `SignalDescriptor` is the risk
tie-in.

### SI-7 — Software, Firmware, and Information Integrity

The **Live layer** implements SI-7 for kernel configuration integrity. Reading from provenance-
verified paths (SECURITYFS_MAGIC, SYSFS_MAGIC, PROCFS_MAGIC) with the existing kattrs
infrastructure is SI-7 applied to kernel attribute reads.

### AU-3 — Audit Record Content

The **ContradictionKind enum and the proposed FindingRecord type** implement AU-3. Audit
records must contain enough information to determine what happened, when, where, and who was
responsible. Typed enums enable machine-readable audit classification.

### RA-5 — Vulnerability Monitoring and Scanning

The control-gap finding type (Live = Configured but both fail Expected) is a form of
vulnerability scanning applied to kernel configuration. RA-5 requires identifying vulnerabilities
in organizational systems; a kernel parameter that enables a known attack primitive is a
configuration vulnerability.

### SI-2 — Flaw Remediation

The `EphemeralHotfix` and `BootDrift` contradiction types support SI-2. They identify conditions
where a remediation has not been properly persisted (EphemeralHotfix) or where a configuration
intended to remediate a flaw has not taken effect (BootDrift).

### PM-6 — Measures of Performance (Supporting)

Aggregate metrics (hardened count, contradiction count, control gap count by impact tier) are
the performance measures for the kernel hardening program. PM-6 requires identifying,
collecting, and reporting performance metrics.

---

## 6. Risks and Concerns

### 6a. False Confidence from "Clean" Status

This is the most important risk. A `Clean` result from the three-layer evaluation means:

1. The live value was successfully read
2. It matches the configured value
3. Both match the expected baseline

What "Clean" does NOT mean:

- The kernel parameter was actually enforced for a specific process at a specific moment
- The sysctl.d file was applied in the correct order (last-writer-wins semantics are subtle)
- The configured value is correct for this site's operational profile
- The expected value in the catalog is authoritative for this site's compliance framework
- The system has not been compromised at a layer below the visible kernel interface

**Recommended mitigations:**

1. The tool must never use the word "compliant" or "passing" in its output — only "hardened"
   or "meets baseline." The distinction matters in an assessment context.
2. "Clean" status should display with a footnote: "meets internal hardening baseline, not a
   compliance determination."
3. The Findings tab (Phase 9) should lead with findings, not with a summary "X signals clean"
   count. Green counts breed complacency.

### 6b. Baseline Drift

The catalog's `desired` values are static compile-time constants. They reflect the hardening
baseline at the time the catalog was written. Kernel releases introduce new parameters;
hardening guidance evolves; DISA STIGs update on an 18-month cycle.

**Risk:** The catalog becomes stale. A system is assessed as "Clean" against a 2026 baseline
when the 2027 DISA STIG has added three new required settings.

**Recommended mitigation:** The catalog version must be surfaced in the posture snapshot
metadata and in any exported assessment report. The `SIGNALS` array needs a version
identifier or a date stamp, and that identifier must appear in `PostureSnapshot`. An
assessor must be able to say "this assessment was performed against catalog version X."

### 6c. Scope Confusion: Kernel Parameters vs. System Policy

The three-layer model assesses kernel parameters only. It does not assess:

- SELinux policy correctness (that is the umrs-selinux domain)
- User account configuration (AC-2)
- Network filtering rules
- File permission correctness
- Service configuration

This is correct scope for the kernel security tab, but G4 must clearly label its scope.
A report that says "kernel posture: 24/26 hardened" next to an unrelated access control
finding will confuse operators about what domain is being evaluated.

### 6d. The Configured Layer Is Not the Ground Truth for Intent

Configuration files reflect what an administrator wrote down, not necessarily organizational
intent. A sysctl.d file written to work around a bug, then never removed, may configure a
setting that no one intends. The tool cannot distinguish "intentional configuration" from
"accidental configuration." This limits the reliability of Contradiction findings as
indicators of unauthorized change.

**Practical implication:** `EphemeralHotfix` and `BootDrift` findings require human
investigation. The tool is correct to surface them and correct to state it cannot determine
the cause. The design notes already acknowledge this. Ensure this is prominent in the UI and
any exported report.

### 6e. cmdline Contradiction Detection Gap (Known)

The existing code documents (in `read_configured_boot_cmdline`) that the generic contradiction
engine cannot evaluate `KernelCmdline`-class signals through `evaluate_configured_meets`,
because the raw configured value stored is the full BLS options string, not an integer. The
architectural decision defers a dedicated cmdline contradiction path to future work.

**Risk:** Boot cmdline signals (lockdown, mitigations, pti, module.sig_enforce) never produce
`EphemeralHotfix` or `BootDrift` findings, even when the BLS entry and the running cmdline
genuinely disagree. These are some of the highest-impact signals (all Critical or High).

**Recommendation:** This gap should be documented as a known limitation in the Findings tab
UI for cmdline-class signals, and the issue should be captured as a tracked work item for G4.
Do not ship G4 without resolving cmdline contradiction detection for Critical signals.

---

## 7. Integration of the "Expected" Layer with ContradictionKind

The existing `ContradictionKind` enum operates on the Live vs. Configured axis only. The
Expected layer is a separate axis. These two dimensions are orthogonal:

```
                    Expected
                    met      not met
                 ┌────────┬──────────┐
Live=Configured  │ Clean  │ ControlGap│
                 ├────────┼──────────┤
Live≠Configured  │ Contradiction (EphemeralHotfix / BootDrift / SourceUnavailable)
                 └────────┴──────────┘
```

A signal can simultaneously have a contradiction AND a control gap. Example: live = `integrity`
(meets expected), configured = `confidentiality` (mismatch), expected = `integrity`. The live
value is fine, but the configured value is wrong. This is an `EphemeralHotfix` — if the system
reboots, it will fall to `confidentiality` and THEN have a control gap.

**Recommendation:** The recommended architecture is a two-field classification in the
finding record:

```rust
pub struct FindingRecord {
    pub signal: &'static SignalDescriptor,
    pub live_value: Option<LiveValue>,
    pub configured_value: Option<ConfiguredValue>,
    pub contradiction: Option<ContradictionKind>,  // Live vs. Configured axis
    pub meets_expected: Option<bool>,               // Live vs. Expected axis
    // ... other fields
}
```

Do NOT collapse these into a single finding enum — they answer different questions and require
different operator actions. A contradiction finding says "go fix your configuration." A control
gap finding says "go escalate to your ISSO, your system is softer than it should be."

The display logic in the TUI can then produce the two-row format shown in the design notes:

```
KERNEL LOCKDOWN
 live (kernel)    : integrity          meets-expected ✓
 configured       : confidentiality    contradiction  ✗ MISMATCH
 expected (CM-6)  : integrity
```

When `meets_expected = Some(true)` AND `contradiction = None`, show single-row clean format.
When either is a finding, show expanded two-row or three-row format.

---

## 8. Summary Recommendations

Ordered by priority:

1. **Retain the two-axis model.** Contradiction (Live vs. Configured) and Control Gap (Live
   vs. Expected) are independent. Do not collapse them into a single finding type. Both are
   needed and both must appear in the `FindingRecord`.

2. **Name the missing states.** `Unverifiable`, `ReliesOnDefault`, and `PolicyDependent` are
   real states that will occur in practice. Implement them before G4 or the Findings tab will
   produce misleading clean counts.

3. **Resolve cmdline contradiction detection before G4.** The current gap means zero
   contradiction findings are possible for Critical-impact cmdline signals. This is the
   most important technical gap.

4. **Add catalog version metadata to `PostureSnapshot`.** Every assessment report must carry
   the baseline version it was assessed against. Without this, findings have no named
   authority.

5. **Do not use the word "compliant" in UI output.** The tool assesses against an internal
   hardening baseline. Compliance determination requires a human assessor. The UI should
   use "hardened / not hardened" and "meets baseline / does not meet baseline."

6. **Plan for OSCAL output in G4.** The `nist_controls` field in each `SignalDescriptor` is
   the seed data. G4 should be able to emit an OSCAL Assessment Results document. This is
   the output format that feeds into accreditation workflows.

7. **Document the ReliesOnDefault finding class in the hardening guide.** Sites that have no
   explicit sysctl.d hardening configuration but happen to run on RHEL 10 defaults will see
   many `ReliesOnDefault` findings. The documentation must explain why this matters under CM-6
   even when the live value is correct.

---

*Advisory complete. No code changes were made. Findings are recommendations for the
rust-developer (items 1, 2, 3, 4) and tech-writer (items 5, 6, 7).*
