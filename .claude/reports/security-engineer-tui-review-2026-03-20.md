# TUI Deployment Security Review — umrs-tui OS Detection Tool

```
Audit date: 2026-03-20
Depth: in-depth
Scope: components/rusty-gadgets/umrs-tui/src/ — all source files
       Files reviewed: main.rs, app.rs, data_panel.rs, dialog.rs, indicators.rs,
       keymap.rs, layout.rs, lib.rs, header.rs, status_bar.rs, tabs.rs, theme.rs
```

---

## Executive Summary

The TUI tool is well-structured from a security posture perspective. Trust
boundaries are clearly documented and correctly enforced at the code level.
The fail-closed contract on `IndicatorValue`, the provenance-gated kernel
reads, the dialog API, and the annotation discipline are all sound. No HIGH
findings. Six MEDIUM and four LOW findings follow, covering gaps in the
contradiction taxonomy, evidence verification string accuracy, Quit-while-
dialog behavior, a missing NO_COLOR path, and indicator grouping placement.

---

## Findings

---

### File: `components/rusty-gadgets/umrs-tui/src/main.rs`

---

**Finding 1**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 1009, function indicator_description (IndicatorId::Lockdown arm)
          and source comment at line 913-915 (append_boot_integrity_group)
Finding: ModulesDisabled is placed in BOOT INTEGRITY with a justification that
         it is a "tamper-resistance control, not a cryptographic primitive."
         The rationale is defensible but may confuse operators. modules_disabled
         is a one-way latch that, once set, prevents any new kernel module from
         loading — it is a post-boot integrity freeze, not a boot-time
         measurement. However, it also directly strengthens the MAC enforcement
         surface by preventing SELinux policy bypass via a rogue module. The
         current placement is not wrong, but the indicator description does not
         mention the SELinux/MAC angle, which is the primary reason DoD/CUI
         deployments care about it. An operator may not understand why it
         appears in BOOT INTEGRITY rather than, say, MODULE RESTRICTIONS.
Severity: LOW
Control reference: NIST SP 800-53 CM-7 (least functionality), AC-3 (MAC
                   bypass via module loading)
Remediation owner: coder
Recommended action: Add to the indicator_description for ModulesDisabled a
                    sentence explaining the SELinux/MAC enforcement angle:
                    "On SELinux systems, this also prevents loading a rogue
                    module that could bypass type enforcement policy." The
                    current placement in BOOT INTEGRITY is acceptable; the
                    description just needs the MAC rationale to be complete.
```

---

**Finding 2**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 1696-1717, function evidence_verification_str
Finding: The verification column displays "PROC_MAGIC" and "SYS_MAGIC" for
         procfs and sysfs records respectively. These labels describe the
         fstatfs(2) filesystem magic check performed by SecureReader. The
         implementation is sound — the label accurately reflects that the
         kernel confirmed PROC_SUPER_MAGIC or SYSFS_MAGIC on the open file
         descriptor.

         However, the label "SYS_MAGIC" is imprecise. The actual kernel
         constant is SYSFS_MAGIC (0x62656572), not "SYS_MAGIC". An assessor
         reading SP 800-53A examination records will look for "SYSFS_MAGIC"
         in kernel documentation and find no match for "SYS_MAGIC". This is
         an accuracy problem for auditors, not a security flaw in the
         implementation. The corresponding help text at line 1880 correctly
         says "sysfs verified via fstatfs() filesystem magic" but then names
         the constant "SYS_MAGIC" in the verification code table at line 1882,
         which is inconsistent with the kernel ABI name.
Severity: MEDIUM
Control reference: NIST SP 800-53 AU-3 (accurate audit record content)
Remediation owner: coder
Recommended action: Change the verification string from "fd, SYS_MAGIC" to
                    "fd, SYSFS_MAGIC" (matching the kernel constant name) in
                    evidence_verification_str (line 1702) and in the help text
                    table at line 1882. The procfs label "PROC_MAGIC" should
                    also be verified — the actual constant is
                    PROC_SUPER_MAGIC (0x9fa0). Change to "fd, PROC_SUPER_MAGIC"
                    for precision, or adopt a project-wide abbreviation
                    convention and document it. Consistency between the
                    display strings and the kernel ABI names matters for
                    auditability.
```

---

**Finding 3**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 1838-1845, help_text_for_tab (tab 1 help text)
Finding: The contradiction type labels in the help overlay text are:
           ⚠ DRIFT         — config says hardened but kernel is not
           ⚠ NOT PERSISTED — kernel is hardened now but config will not keep it
           ⚠ UNVERIFIABLE  — config exists but kernel value could not be read

         The first two labels match the semantics of ContradictionKind::BootDrift
         and ContradictionKind::EphemeralHotfix respectively. However, the
         operator-facing names in the help text ("DRIFT", "NOT PERSISTED") do
         not match the ContradictionKind variant names used in Rust code
         ("BootDrift", "EphemeralHotfix", "SourceUnavailable"). This is a
         documentation/display consistency gap.

         More importantly: "UNVERIFIABLE" covers SourceUnavailable but not a
         fourth case — a configured value exists and a live value exists but
         they agree (no contradiction). This is the no-contradiction path and
         needs no label; that is fine. But there is a gap in the taxonomy for
         the case where the configured value is present, the live kernel value
         is present, and they DISAGREE but neither maps cleanly to BootDrift
         or EphemeralHotfix because the disagreement direction is unclear (e.g.,
         both are unhardened but different unhardened values). The current three
         types correctly cover the most important operational scenarios; the gap
         is theoretical for current indicators but should be noted for future
         indicator expansion where the desired direction is not binary.
Severity: LOW
Control reference: NIST SP 800-53 CM-6, CA-7
Remediation owner: coder
Recommended action: (1) Align operator-facing label names in the help text
         with the names used in indicator_description and in the audit trail:
         consider "CONFIG DRIFT" (BootDrift), "EPHEMERAL HOTFIX" or
         "NOT PERSISTED" (EphemeralHotfix), and "UNVERIFIABLE" (SourceUnavailable).
         The current names are understandable but mixing internal and operator
         terminology creates inconsistency for trained operators who read both
         the help text and log output.
         (2) Add a comment in the ContradictionKind definition (in umrs-platform
         posture module) noting that a fourth variant may be needed for
         "ValueMismatch" (both sides readable, both values present, but neither
         maps to BootDrift/EphemeralHotfix direction). Flag for coder to evaluate.
```

---

**Finding 4**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 1990-1999 (event loop, Quit action while dialog open)
Finding: When a help dialog is open, the Quit key ('q' / Esc) dismisses
         the dialog rather than quitting the application. This is the correct
         behavior for the current use case (help-only dialogs). The comment
         at line 1990 acknowledges this design choice.

         However, this behavior is generalized: the same arm handles
         Quit unconditionally when help_dialog.is_some(). If in a future
         phase a SecurityWarning or Confirm dialog is used (currently not,
         but the dialog API is designed for this), pressing 'q' or Esc
         would dismiss that dialog with no response recorded — equivalent
         to a silent cancel, without triggering the AU-2/AU-3 audit obligation
         that dialog.rs documents. The current code does not set
         help_dialog.response before clearing it; it just sets help_dialog = None.
         For an Info dialog this is correct (no audit requirement). For a
         SecurityWarning or Confirm dialog, this would silently bypass the
         required journald audit record.

         This is a latent risk in the event loop architecture: the Quit guard
         at line 1994 does not differentiate dialog modes. If SecurityWarning
         dialogs are added in the future (Phase 10 per dialog.rs comments),
         the Quit action must check the dialog mode before clearing it,
         and must emit an audit record if the mode requires one.
Severity: MEDIUM
Control reference: NIST SP 800-53 AU-2, AU-3 (auditable events — operator
                   dismissal of a security warning dialog must be logged)
Remediation owner: coder
Recommended action: Add a guard in the Quit/DialogCancel arm that checks
         whether the active dialog is a SecurityWarning or Confirm mode. If so:
         (1) record response = Some(false) on the dialog state before clearing,
         (2) emit the required journald record per the DialogState audit
         obligation documented in dialog.rs.
         Concretely, before `help_dialog = None`, insert:
           if let Some(ref d) = help_dialog {
               if matches!(d.mode, DialogMode::SecurityWarning | DialogMode::Confirm) {
                   // emit journald record: cancelled, boot_id, tool name
               }
           }
         This latent risk is low severity today because only Info dialogs are
         used; it becomes HIGH if SecurityWarning dialogs are added without
         fixing the event loop.
```

---

**Finding 5**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 1316-1323, indicator_group_rows (configured_line construction)
Finding: The configured_line displayed to the operator is:
           "Configured: {raw} (from {source_file})"
         where `raw` is the ConfiguredValue::raw field (the configured value
         string from /etc/sysctl.d/ or equivalent) and `source_file` is the
         path of the config file.

         The raw value of a configured setting is operator-visible here.
         On a CUI system this is appropriate and expected — the operator needs
         to see both what the kernel currently has and what the config file
         says. However, the Debug Log Information Discipline Rule
         (.claude/rules/high_assurance_pattern_rules.md) requires that
         raw values from configuration files must NOT appear in debug log
         output (SI-11). The display path here is correct; the risk is if
         anyone adds a log::debug! call nearby that uses `configured_line`
         or `raw` directly.

         Additionally, `source_file` is an unvalidated path string
         sourced from the posture snapshot. If a crafted sysctl.d filename
         contains unusual characters (e.g., a very long path, embedded
         newlines, ANSI escape sequences), the display string could break
         the TUI layout or, in a terminal that processes escape sequences
         in ratatui Span content, inject terminal escape sequences.
         Ratatui itself is generally safe here, but the path is not sanitized
         before inclusion in the display string.
Severity: MEDIUM
Control reference: NIST SP 800-53 SI-11 (error handling / output discipline),
                   SI-10 (input validation — display-bound strings)
Remediation owner: coder
Recommended action: Before constructing `configured_line`, sanitize
         `source_file` to strip any characters outside the printable ASCII
         range (or Unicode letter/digit/punctuation categories). A simple
         approach: replace any byte < 0x20 or == 0x7f with '?'. This prevents
         terminal injection through crafted filenames and makes the display
         robust against unusual filesystem names.
         Example:
           let safe_path: String = source_file
               .chars()
               .map(|c| if c.is_control() { '?' } else { c })
               .collect();
```

---

**Finding 6**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 800-887, build_kernel_security_rows — indicator group
          assignments
Finding: The "NETWORK AUDITING" group currently contains only one indicator:
         NfConntrackAcct. The group description says it enables "traffic
         accounting for anomaly detection and forensic reconstruction."
         This is accurate as far as it goes.

         However, nf_conntrack_acct is a per-connection byte/packet counter
         toggle, not a control that enables network auditing in the security
         sense (e.g., it does not enable nftables logging, auditd network
         rules, or SELinux network policy enforcement). Placing it in a group
         called "NETWORK AUDITING" overstates what the indicator controls.
         An operator could interpret "NETWORK AUDITING" as meaning the system's
         network-layer audit posture is fully assessed when only conntrack
         accounting is checked.

         The more accurate framing is "NETWORK TELEMETRY" or "NETWORK TRACKING"
         — it is a data collection enabler for forensics, not an audit control.
         This is a display accuracy/trust assertion concern.
Severity: LOW
Control reference: NIST SP 800-53 AU-12 (audit record generation), CA-7
Remediation owner: coder
Recommended action: Rename the group to "NETWORK TELEMETRY" and update the
         description to: "Enables per-connection byte and packet counters for
         netfilter. Required for traffic-volume data in anomaly detection and
         forensic reconstruction. Not a substitute for nftables audit logging
         or SELinux network policy enforcement." Update the corresponding help
         text at line 1832 to match the new group name.
```

---

**Finding 7**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 879-887 (MODULE RESTRICTIONS group), line 862 (group description)
Finding: The MODULE RESTRICTIONS group description correctly identifies
         USB, FireWire, and Thunderbolt as "primary data exfiltration and DMA
         attack vectors." It also includes Bluetooth. However, the group
         description does not mention that the mechanism checked is module
         blacklisting via modprobe.d, not module signature enforcement or
         kernel build-time CONFIG_BT=n exclusion.

         An operator who sees "bluetooth (blacklisted) — ✓" may believe the
         Bluetooth kernel code is absent, when in fact only a modprobe blacklist
         entry exists. The blacklist is a DAC-level defense: it can be bypassed
         by an operator running `modprobe bluetooth` with root. The indicator
         description for BluetoothBlacklisted says only "The Bluetooth protocol
         stack is large, historically vulnerability-prone, and serves no purpose
         on server infrastructure" — it does not explain that the check is a
         blacklist (not a build-time exclusion or MAC enforcement).
Severity: MEDIUM
Control reference: NIST SP 800-53 CM-7 (least functionality), AC-3
Remediation owner: coder
Recommended action: Update the indicator_description for all four blacklisted
         indicators (Bluetooth, UsbStorage, FirewirCore, Thunderbolt) to add
         a sentence clarifying the mechanism:
         "The check confirms a modprobe.d blacklist entry exists, which
         prevents automatic loading. A privileged user can still load the
         module manually. For stronger assurance, remove the module from the
         kernel build (CONFIG_BT=n)."
         This prevents an operator from interpreting "blacklisted" as equivalent
         to "absent from kernel."
```

---

**Finding 8**

```
File: components/rusty-gadgets/umrs-tui/src/main.rs
Location: line 1362-1368, translate_live_value (LiveValue::Text branch)
Finding: The sentinel string "absent" is translated to "Not Present" for
         display. This sentinel originates from the posture snapshot when
         a cmdline token is not found in /proc/cmdline or when no modprobe
         blacklist entry exists. The translation is display-only and correct.

         However, the sentinel is a string comparison against a magic value
         ("absent") sourced from a library type (umrs-platform). If the
         platform library ever changes this sentinel to a different string,
         or if a legitimate text value of "absent" appears in a future
         indicator type, the display path would silently misrepresent it as
         "Not Present." This is a coupling risk between the TUI display layer
         and the platform library's internal sentinel convention.
Severity: LOW
Control reference: NIST SP 800-53 SI-10 (input validation)
Remediation owner: coder
Recommended action: Add a dedicated LiveValue variant to umrs-platform
         (e.g., LiveValue::Absent) instead of the "absent" string sentinel.
         The TUI can then match on the variant without string comparison.
         This is a type-system improvement that eliminates the string
         coupling and makes the invariant mechanically verifiable. If a
         dedicated variant is not feasible in the current platform API,
         at minimum add a module-level comment in indicators.rs documenting
         the dependency on the "absent" sentinel and flag it for the coder
         to address when the platform LiveValue type is next revised.
```

---

### File: `components/rusty-gadgets/umrs-tui/src/indicators.rs`

---

**Finding 9**

```
File: components/rusty-gadgets/umrs-tui/src/indicators.rs
Location: line 192-232, read_selinux_status
Finding: When SELinux is active (EnforceState::Enforcing or Permissive),
         the policy type is read from /etc/selinux/config via selinux_policy().
         This is gated behind the kernel confirmation of enforcement state,
         satisfying the Trust Gate pattern (CM-6). However, the policy label
         read from /etc/selinux/config is used to augment the display string
         (e.g., "Enforcing (Targeted)"). The config file is a RegularFile
         source — lower trust than the kernel attribute.

         The comment at line 196 says this read is "gated behind kernel
         confirmation" which is correct for the decision to read or not read
         the file. But the displayed string combines kernel-verified state
         (enforcing/permissive) with config-file-derived data (policy name)
         without visually distinguishing them. An operator could interpret
         "Enforcing (Targeted)" as a fully kernel-verified assertion when
         "Targeted" comes from /etc/selinux/config, not from the kernel.

         The actual policy type visible to the kernel at runtime is available
         via /sys/fs/selinux/policyvers (policy version, not type) or via
         /sys/kernel/security/lsm. Neither is currently used for the policy
         type string. This is not a trust violation because the display string
         is labeled as display-only in app.rs HeaderContext documentation —
         but the operator-facing label does not make this distinction clear.
Severity: MEDIUM
Control reference: NIST SP 800-53 CM-6 (configuration settings — source
                   attribution for mixed-trust display strings), AU-3
Remediation owner: coder
Recommended action: Append a parenthetical source qualifier to the policy
         name when it comes from /etc/selinux/config:
           "Enforcing (Targeted, from config)"
         or change the display to show only the kernel-confirmed state in the
         header ("Enforcing") and move the policy type to the Kernel Security
         tab's Boot Integrity group where source attribution is always shown.
         The current approach is not wrong, but the mixed-trust display string
         could mislead an assessor into treating the policy type as
         kernel-verified evidence.
```

---

### File: `components/rusty-gadgets/umrs-tui/src/app.rs`

---

**Finding 10**

```
File: components/rusty-gadgets/umrs-tui/src/app.rs
Location: line 276, StatusMessage::new — #[must_use] annotation
Finding: The #[must_use] annotation on StatusMessage::new (line 277) is
         bare — it carries no message string explaining why the return
         value matters. The Must-Use Contract Rule requires that all
         #[must_use] annotations include a descriptive message.
         This is a minor rule compliance gap.
Severity: LOW
Control reference: NIST SP 800-53 SI-10, SA-11 (Must-Use Contract Rule)
Remediation owner: coder
Recommended action: Change:
           #[must_use]
           pub fn new(...) -> Self {
         to:
           #[must_use = "StatusMessage must be stored and passed to the render path; \
                         discarding it leaves the status bar without a level or message"]
           pub fn new(...) -> Self {
```

---

## Assessment of Specific Questions

### 1. Trust Boundaries

The separation between kernel-trusted data and config-file data is correctly
modeled and documented. The TrustLevel hierarchy (T0–T4), the Trust Gate
pattern in read_selinux_status, and the explicit `display-only` annotations
on hostname, kernel_version, and architecture all reflect sound design.
The only concern is Finding 9 — the header displays a mixed-trust string
(kernel-confirmed enforcement state + config-file policy name) without
distinguishing the provenance of the policy name component.

### 2. Information Leakage

No sensitive data leakage was identified. The SI-12 citations throughout the
codebase are appropriate and correctly applied. The error path in
OsDetectApp::from_error suppresses variable kernel data. The status bar text
is bounded and does not include raw kernel values. The one area requiring
attention is Finding 5 — unsanitized source_file paths in configured_line
display strings could pass control characters to the display layer.

### 3. Provenance Verification (PROC_MAGIC / SYS_MAGIC)

The implementation is sound. SecureReader uses fd-anchored fstatfs(2) to
confirm filesystem magic before reading any bytes. The display strings
in evidence_verification_str correctly reflect that this check was performed
and identify which magic constant was confirmed. Finding 2 identifies an
accuracy problem: the label "SYS_MAGIC" does not match the kernel ABI name
"SYSFS_MAGIC," and "PROC_MAGIC" does not match "PROC_SUPER_MAGIC." This is
an audit record accuracy issue, not a security flaw in the implementation.

### 4. Contradiction Model

The three types — BootDrift, EphemeralHotfix, SourceUnavailable — correctly
cover the three operationally significant cases:
  - Intended hardening is configured but not active (BootDrift): HIGH urgency,
    CUI processing blocked.
  - Active hardening will not survive reboot (EphemeralHotfix): MEDIUM urgency,
    regression risk.
  - Cannot verify because kernel value is unreadable (SourceUnavailable): LOW
    urgency, instrumentation gap.

This three-type taxonomy is appropriate and sufficient for current indicators.
The gap identified in Finding 3 (value-mismatch without clear direction) is
theoretical for current sysctl-based indicators and should be noted for future
expansion rather than addressed immediately.

### 5. Dialog Security

The dialog model is well-designed for its current use (Info-only help dialogs):
  - Two-button dialogs default to Cancel (fail-safe, SC-5 compliant).
  - No auto-dismiss (AC-2 compliant).
  - `SecurityWarning` and `Confirm` modes are type-safe and correctly
    differentiated from `Info`/`Error`.
  - The AU-2/AU-3 audit obligation is clearly documented in dialog.rs and
    must be enforced by callers.

The only structural risk is Finding 4 — the Quit key handler in the event loop
does not check dialog mode before clearing the dialog state. This is safe today
(only Info dialogs are used) but becomes a latent audit bypass if SecurityWarning
dialogs are added in Phase 10 without updating the event loop.

### 6. Indicator Groupings

Groups are logically organized and the group descriptions are accurate.
Specific concerns:
  - ModulesDisabled in BOOT INTEGRITY: acceptable placement, description
    needs the MAC/SELinux angle added (Finding 1).
  - MODULE RESTRICTIONS blacklist mechanism: description understates that
    blacklisting is a DAC control, not MAC or build-time exclusion (Finding 7).
  - NETWORK AUDITING name: overstates the scope of the single indicator
    (nf_conntrack_acct). Should be renamed NETWORK TELEMETRY (Finding 6).

The other six groups (CRYPTOGRAPHIC POSTURE, KERNEL SELF-PROTECTION, PROCESS
ISOLATION, FILESYSTEM HARDENING, and their indicator assignments) are correct
and the descriptions accurately represent the threat model for each group.

---

## Gap Analysis Summary

```
Files reviewed: 12
Total findings: 10 (0 HIGH, 5 MEDIUM, 5 LOW)

Policy artifacts written: none — this tool reads kernel state; it does not
   modify system policy. No SELinux policy changes are required from this review.
   Existing policy in components/platforms/rhel10/ covers tool execution context.

Policy artifacts needed: none from this review.

Documentation gaps:
  - Help text uses "SYS_MAGIC" / "PROC_MAGIC" (should be "SYSFS_MAGIC" /
    "PROC_SUPER_MAGIC" to match kernel ABI names) — also affects displayed
    evidence verification strings.
  - MODULE RESTRICTIONS group description does not explain that blacklisting
    is a modprobe.d DAC mechanism, not MAC or build-time exclusion.
  - NETWORK AUDITING group name overstates scope of nf_conntrack_acct.
  - Contradiction type names in help text do not match Rust type names.

Code-vs-policy inconsistencies:
  - read_selinux_status combines kernel-verified enforcement state with
    config-file-derived policy name into a single display string without
    source attribution. Display-only annotation in HeaderContext documentation
    mitigates this but does not surface the distinction to operators.
  - "absent" sentinel in LiveValue::Text creates string-coupling dependency
    between TUI display layer and platform library internals.

Key latent risk (not HIGH today, will be HIGH if not addressed before Phase 10):
  - Quit/DialogCancel key handler does not check dialog mode before clearing
    dialog state. If SecurityWarning dialogs are added in Phase 10, this handler
    must be updated to emit the required AU-2/AU-3 journald record.
```

---

*Security engineer review. Not an approval. All findings require human review
before any action is taken.*
