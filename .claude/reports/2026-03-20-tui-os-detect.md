# Guest Admin Review — umrs-tui OS Detection Audit Card

```
Date: 2026-03-20
Scope: umrs-tui OS Detection Audit Card (umrs-os-detect-tui binary)
       Source files reviewed: main.rs, dialog.rs, keymap.rs, status_bar.rs,
       app.rs, data_panel.rs (selected sections), audit-card.md, how-to.md
Binaries evaluated: none pre-built — no binary was available for live execution.
                    All findings are derived from source code review.
```

---

## Framing Note

No binary was available for execution. This review is based entirely on
source code. I am calling that out once, here, and not in every finding —
but the absence of a live run means I cannot report on actual rendering,
terminal resize behavior, or scrollbar feedback. Those gaps are the
responsibility of whoever runs this before field deployment.

---

## Documentation Findings

---

```
Section: No operator-facing documentation exists (umrs-tools module)
Finding: There is no page in docs/modules/umrs-tools/ describing what
         umrs-os-detect-tui does, how to launch it, what it requires, or
         how to interpret its output. The only non-source text is how-to.md
         (a developer guide for adding new binaries) and audit-card.md (a
         design request document, not operator guidance). An operator handed
         this binary has no starting point.
Type: Completeness
Severity: HIGH
Suggestion: Add an operator page to docs/modules/umrs-tools/pages/ covering:
            (1) what the tool does in one paragraph; (2) how to run it;
            (3) minimum privilege required; (4) what the three tabs mean;
            (5) color and symbol legend; (6) what a red finding means and
            what to do about it.
Source consulted: yes — confirmed by searching docs/modules/umrs-tools/ for
                  TUI and os-detect references.
```

---

```
Section: how-to.md — audience mismatch
Finding: how-to.md is titled "How to Add a New TUI Binary." It is addressed
         entirely to developers extending the crate. It is not an operator
         guide. A sysadmin handed this document as reference material would
         gain nothing useful and would correctly conclude that documentation
         for operators does not exist.
Type: Completeness
Severity: HIGH
Suggestion: Either rename the file clearly (e.g., developer-guide.md) to
            prevent it being mistaken for operator guidance, or add a
            separate operator quickstart file. Do not expand this file with
            operator content — the audiences are distinct.
Source consulted: no
```

---

```
Section: Trust tier descriptions (main.rs, trust_level_description)
Finding: The T1 description reads: "procfs verified via PROC_SUPER_MAGIC +
         PID coherence." This appears in the Trust / Evidence tab visible to
         operators. PROC_SUPER_MAGIC is an internal implementation term —
         it is the filesystem magic number used by fstatfs(2) to confirm
         that /proc is the real kernel procfs. An operator reading this
         sees jargon with no context. The T2 description "Mount topology
         cross-checked (mountinfo vs statfs)" has the same problem: it names
         internal data structures without explaining the threat being mitigated.
Type: Assumed Knowledge
Severity: MEDIUM
Suggestion: Rewrite for operator audience. Example for T1: "Kernel filesystem
            (/proc) confirmed genuine — not a crafted substitute." Example for T2:
            "Kernel filesystem layout cross-checked using two independent methods."
            The verification codes PROC_MAGIC and SYS_MAGIC in the evidence
            chain help text are separately addressed below.
Source consulted: yes — confirmed that trust_level_description() values
                  appear directly in operator-visible DataRows (build_trust_summary_rows).
```

---

```
Section: Evidence verification codes in help text (main.rs, help_text_for_tab, tab 2)
Finding: The help text for the Trust / Evidence tab does explain PROC_MAGIC
         and SYS_MAGIC with: "These confirm the source is the real kernel
         filesystem, not a crafted file at the same path." This is good.
         However, it explains the method (fstatfs filesystem magic) without
         first explaining the threat: why does it matter whether /proc is
         "real"? An operator who does not know about procfs spoofing attacks
         will not understand why the check matters, and therefore may not
         escalate when they see a FAIL on a path-opened record vs. an
         fd-opened record. The help explains the mechanism but not the threat.
Type: Missing Rationale
Severity: MEDIUM
Suggestion: Add one sentence before the verification codes block explaining
            the threat: something like "An attacker can create a file at
            /proc/version that mimics kernel output. File-descriptor reads with
            filesystem magic verification confirm the data came from the real
            kernel, not a planted file." This gives operators a reason to care
            about the distinction between (fd, PROC_MAGIC) and (path).
Source consulted: no
```

---

```
Section: Error state from_error() (main.rs, OsDetectApp::from_error)
Finding: When the detection pipeline fails, the OS Information tab shows
         "Detection pipeline failed" in both the Status row and the Reason
         row. The Reason values are:
           - "Hard gate: procfs is not real procfs"
           - "Hard gate: PID coherence broken"
           - "Hard gate: I/O error during kernel anchor"
         The phrase "Hard gate" will be opaque to operators who have never
         read UMRS design documentation. There is no inline explanation of
         what a hard gate is, why it triggered, or what action the operator
         should take. "Hard gate: procfs is not real procfs" is alarming
         but does not say what to do — is this a hardware failure, a
         container environment, a security event?
Type: Clarity
Severity: HIGH
Suggestion: Rewrite error messages to include a brief action hint. Examples:
            "procfs is not genuine kernel procfs — this system may be a
             container or have a tampered /proc. Verify environment before
             proceeding."
            "PID coherence check failed — the process ID in /proc does not
             match the running process. This may indicate procfs tampering."
            "I/O error reading kernel anchor — check permissions on /proc
             and rerun."
            Also replace "Hard gate" with something like "Security check
            failed" in the Status row since operators do not know the term.
Source consulted: yes — confirmed from_error() directly populates displayed
                  DataRows visible to operators.
```

---

```
Section: Contradiction terminology in Trust / Evidence help (main.rs, help_text_for_tab, tab 2)
Finding: The help text for tab 2 distinguishes OS detection contradictions
         (two sources disagree about OS identity) from Kernel Security tab
         contradictions (kernel vs. config disagree). This distinction is
         explained but requires two careful reads. The sentence
         "Contradictions here are OS detection contradictions: two
         independent sources (e.g., /etc/os-release vs package DB) reported
         conflicting values for the same fact. This is distinct from
         kernel/config contradictions on the Kernel Security tab." is correct
         but uses the word "contradictions" five times across four lines with
         two different meanings. An operator scanning quickly will conflate them.
Type: Clarity
Severity: MEDIUM
Suggestion: Use distinct terms in the two tabs rather than explaining the
            distinction inside help text. Possible rename: call the Trust /
            Evidence version "Identity Conflicts" and keep "Contradictions"
            for the Kernel Security tab (which already has the DRIFT /
            NOT PERSISTED / UNVERIFIABLE vocabulary). Alternatively, put a
            one-line anchor at the top of each help text that names only
            the relevant type of contradiction for that tab.
Source consulted: no
```

---

```
Section: Label trust display (main.rs, label_trust_display)
Finding: The Label Trust field on the Trust / Evidence tab shows values like:
           "UntrustedLabelCandidate — do not use for policy"
           "LabelClaim — structurally valid; integrity unconfirmed"
           "TrustedLabel — T4: ownership + digest verified"
           "IntegrityVerifiedButContradictory: <truncated description>"
         The internal type names (UntrustedLabelCandidate, LabelClaim,
         TrustedLabel, IntegrityVerifiedButContradictory) appear verbatim.
         These are Rust enum variant names, not operator-facing vocabulary.
         An operator will read "LabelClaim" and have no intuition about what
         a "claim" means vs. a "label" vs. "trusted." The parenthetical
         descriptions help but do not replace the missing context.
Type: Assumed Knowledge
Severity: MEDIUM
Suggestion: Either remove the internal variant names from the display string
            and show only the description, or map them to plain-English
            display names. Examples:
              "Identity unverified — do not use for policy decisions"
              "Identity asserted but not verified"
              "Identity verified — ownership and file digest confirmed"
              "Identity verified but conflicting evidence found: <desc>"
            If the internal names serve a debugging purpose, move them to
            a parenthetical after the plain label, not before it.
Source consulted: yes — confirmed these strings are the operator-visible
                  display output in build_trust_summary_rows.
```

---

```
Section: "Platform Facts" row (main.rs, build_os_info_rows)
Finding: The OS Information tab includes a row labelled "Platform Facts" with
         a numeric value (e.g., "3"). There is no explanation of what a
         "platform fact" is. The number appears alongside other OS identity
         fields with no description row beneath it. An operator reading "3"
         would not know whether 3 is good, bad, normal, or whether more
         facts mean higher confidence.
Type: Clarity
Severity: LOW
Suggestion: Add a description row beneath "Platform Facts" in the same
            pattern used for Downgrade Reasons — e.g., "Number of independent
            package database records confirming platform identity." Or
            rename the key to something that carries meaning without context:
            "Package ID Records" or "Identity Corroborations."
Source consulted: no
```

---

```
Section: "Probe Used" row (main.rs, build_os_info_rows)
Finding: The OS Information tab shows "Probe Used" with a value like a
         package manager name or probe identifier. An operator does not know
         what a "probe" is in this context. Is it a kernel probe? A package
         query? The term is borrowed from the detection pipeline internals.
Type: Assumed Knowledge
Severity: LOW
Suggestion: Rename to "Detection Method" or "Identity Source" and provide
            a description row: "The package database or detection method
            used to confirm platform identity."
Source consulted: no
```

---

```
Section: Downgrade Reasons: count shown before context (main.rs, build_trust_summary_rows)
Finding: When downgrade reasons exist, the display shows:
           Downgrade Reasons: 2
           (blank key): Each reason below prevented a trust tier upgrade.
           [1]: <reason text>
           [2]: <reason text>
         The count comes before the explanation of what a downgrade reason is.
         An operator encountering "Downgrade Reasons: 2" with a yellow color
         might interpret it as an error count. The explanation one row below
         ("prevented a trust tier upgrade") is the operative context and
         should come first or be combined with the count.
Type: Clarity
Severity: LOW
Suggestion: Combine into a single row: "Downgrade Reasons: 2 (trust tier
            could not be fully elevated — see below)". Or reverse the order:
            show the explanation row before the numbered list.
Source consulted: no
```

---

## CLI / TUI Usability Findings

---

```
Tool: umrs-os-detect-tui
Argument/Command: (status bar key legend)
Finding: The status bar legend reads: "Tab: tabs | ↑↓/jk: scroll | ?: help | q: quit"
         This is accurate and helpful. However, it omits the Refresh action (r key)
         and PageUp/PageDown navigation. An operator on a long Kernel Security tab
         with many indicators would not discover page scrolling from the legend.
         The Refresh action also goes undiscovered, which matters if the operator
         wants to re-collect data after making a system change.
Type: Help Text
Severity: MEDIUM
Suggestion: Add PageUp/PageDown to the legend: "PgUp/PgDn: page" — even
            abbreviated. Consider adding "r: refresh" since it is a
            functionally important action for an assessment workflow where
            the operator may apply a sysctl and want to see the updated
            posture immediately.
Source consulted: yes — confirmed r is bound as Refresh in keymap.rs and
                  PageUp/PageDown are also bound but absent from KEY_LEGEND
                  in status_bar.rs.
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: Kernel Security tab — "No Assessment" row
Finding: When some indicators have no assessment (meets_desired = None),
         the summary pane shows "No Assessment: 3" with a dim style. An
         operator will not immediately know what causes a "No Assessment"
         result — is it a permission failure, an older kernel, a probe
         that is not implemented yet, or a custom/non-standard configuration?
         Each of these has different operator implications.
Type: Output
Severity: MEDIUM
Suggestion: Add a brief parenthetical to the count row, or a description row
            beneath it, explaining what triggers the No Assessment state.
            Example: "No Assessment: 3 (kernel node unreadable or indicator
            not applicable to this kernel version)." This gives the operator
            context to decide whether to investigate or ignore.
Source consulted: yes — confirmed "No Assessment" is app.rs None branch for
                  meets_desired in build_kernel_security_summary_rows. The
                  rendering in indicator_group_rows skips unreadable rows
                  entirely, so the operator cannot scroll to see why.
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: Evidence chain — "✓ ok (path)" vs "✓ ok (fd, PROC_MAGIC)"
Finding: The Verification column shows two passing states: "✓ ok (path)" and
         "✓ ok (fd, PROC_MAGIC)". Both show green and a checkmark. An operator
         seeing two different passing strings for the same column will not
         immediately understand that these represent different trust levels.
         Path-opened reads are weaker — the tool read the file by name, which
         is spoofable. FD-opened reads with magic verification are stronger.
         The column renders both as green checkmarks with no visual difference
         in trust weight.
Type: Output
Severity: MEDIUM
Suggestion: Either use a different color or symbol to distinguish path-opened
            from fd-opened records, or add a sub-column or badge that makes
            the hierarchy visible. If color differentiation is not desired,
            the help text for tab 2 already explains the distinction — ensure
            the in-TUI explanation is easily findable (it currently is, via
            the ? key, which is appropriate).
            Alternatively, add a "Strength" column or suffix like
            "(verified)" vs "(unverified)" to make the distinction
            visually clear without requiring the operator to read help text.
Source consulted: yes — confirmed in evidence_verification_str() and the
                  data_panel rendering path.
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: Kernel Security tab — Contradiction marker labels
Finding: The three contradiction marker labels are:
           "⚠ DRIFT: config says hardened, kernel is not"
           "⚠ NOT PERSISTED: hardened now, lost after reboot"
           "⚠ UNVERIFIABLE: config exists but kernel node unreadable"
         These are clear and actionable — this is the strongest part of the
         operator-facing display. No finding required; noting for the record
         as exemplary.
         HOWEVER: The help text uses a slightly different vocabulary. Help
         text for tab 1 shows "⚠ DRIFT" and "⚠ NOT PERSISTED" but then
         labels the third as "⚠ UNVERIFIABLE" in the help text, which matches.
         The help text uses the full label text in descriptions but the
         actual marker in the display is "UNVERIFIABLE" not "SourceUnavailable."
         The internal name SourceUnavailable does not appear to operators.
         This is correct — I am confirming the good practice.
Type: Output
Severity: LOW
Suggestion: No change needed to the contradiction marker labels. Consider
            using these same concise labels (DRIFT / NOT PERSISTED /
            UNVERIFIABLE) as the primary vocabulary throughout all
            documentation about this feature, since they are the most
            operator-comprehensible form.
Source consulted: yes — confirmed in data_panel.rs marker_text values.
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: Dialog — help popup dismiss instructions
Finding: The help popup ends with "Press Enter, Esc, or q to close this help."
         In the event loop, Quit (Esc or q) while a dialog is open dismisses
         the dialog rather than quitting the application. This is the correct
         behavior. However, an operator who wants to quit entirely while reading
         help must first dismiss the help, then press q again. This two-step
         behavior is not communicated in the help text. An operator who presses
         q while reading help may expect to exit and be surprised when only the
         help closes.
Type: Output
Severity: LOW
Suggestion: Change the dismiss instruction to: "Press Enter, Esc, or q to
            close help (press q again to quit)." This sets the correct
            expectation without disrupting the current two-step behavior.
Source consulted: yes — confirmed in the event loop (main.rs, Action::Quit
                  when help_dialog.is_some() sets help_dialog = None rather
                  than setting state.should_quit).
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: OS Information tab — "boot_id" row label
Finding: The "boot_id" key label is displayed in lowercase with underscores —
         it is the raw kernel field name from /proc/sys/kernel/random/boot_id.
         All other rows on the same tab use title-case human labels: "Platform
         Family", "Platform Distro", "Platform Version". The inconsistency
         signals that boot_id was left as the raw kernel name. It is also not
         explained — an operator will ask "what is a boot ID?" and find no
         answer on screen.
Type: Clarity
Severity: LOW
Suggestion: Rename the key to "Boot ID" for consistency with surrounding
            labels, and add a description row: "Unique identifier for this
            boot session. Changes on every reboot. Used for journald log
            correlation."
Source consulted: no
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: (overall — missing --help or --version CLI flags)
Finding: From source inspection, main() takes no command-line arguments and
         provides no --help or --version output. An operator who runs the
         binary with --help will receive no usage guidance. There is no way
         to discover what the tool does from the command line before launching
         the TUI. Operator documentation (as noted above) does not yet exist,
         so the binary is self-documenting only once it is running.
Type: Missing Capability
Severity: MEDIUM
Suggestion: Add --help output that describes the tool in 3-5 sentences and
            lists key bindings. Add --version output. These are standard
            operator expectations for any CLI/TUI binary.
Source consulted: yes — confirmed main() has no argument parsing.
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: (overall — no --json output mode)
Finding: The tool has no --json output mode. The UMRS project design principle
         (CLAUDE.md) requires: "Provide --json output mode for all commands
         that return structured data." The OS detection result and posture
         snapshot are highly structured data. Without JSON output, the tool
         cannot feed into automated assessment pipelines, monitoring scripts,
         or be called from a higher-level orchestrator. As a standalone TUI
         this is acceptable for now, but it will become a gap as soon as
         operators want to automate assessments.
Type: Missing Capability
Severity: MEDIUM
Suggestion: Add a --json flag. When present, emit the detection result and
            posture snapshot as JSON to stdout and exit without launching the
            TUI. This follows the project's own stated design principle and
            enables scripted use.
Source consulted: yes — confirmed no argument parsing in main().
```

---

```
Tool: umrs-os-detect-tui
Argument/Command: Kernel Security tab — group description for CRYPTOGRAPHIC POSTURE
Finding: The group description reads: "Verifies government-validated
         cryptography and correct entropy sourcing. Failures here mean
         sensitive operations may use unvalidated algorithms."
         The phrase "government-validated cryptography" is correct for a
         DoD/federal audience but not explained. An operator who does not
         know FIPS 140-2/3 may not understand that "government-validated"
         refers to a specific certification regime with legal force for CUI
         systems, not just "algorithms that are generally regarded as secure."
         The description for FIPS mode within the group does explain this:
         "Required for DoD and federal deployments processing CUI." The two
         descriptions are consistent but the group header description misses
         the opportunity to establish the stakes.
Type: Missing Rationale
Severity: LOW
Suggestion: Add a brief CUI reference to the group header description:
            "Verifies FIPS-validated cryptography is active. Required for
            CUI processing. Failures here mean sensitive operations may
            use unvalidated algorithms."
Source consulted: no
```

---

## High-Assurance Communication Assessment

An operator who runs this tool and reads the Kernel Security tab would
understand that something security-significant is being assessed — the color
coding, the checkmarks vs. X marks, and the contradiction markers are clear
signals. The DRIFT and NOT PERSISTED labels are the single best piece of
operator-facing communication in the tool: they name the finding in plain
English, explain the consequence, and appear exactly where the problem is.

What the tool does not communicate well is the stakes. An operator who has
never read UMRS design documentation would not understand from the tool alone
why "Trust Tier T2" is different from "Trust Tier T4" in ways that matter for
production use, why a path-opened evidence record is weaker than an
fd-opened record, or why the term "hard gate" in a failure message should
trigger an escalation rather than a re-run. The tool reads like a security
tool, but it does not yet explain why its findings matter enough to act on.

The inline help for the Kernel Security tab (tab 1) comes closest to
communicating appropriate stakes: "Red rows require remediation before CUI
processing" is direct, actionable, and tells the operator what the finding
means in their workflow. That sentence is the model for how the rest of the
operator-facing text should be written. More of it is needed, particularly
in error states and in the Trust tier descriptions.

---

## Overall Operator Usability Score: 6.5 / 10

The tool is well-built internally and the Kernel Security tab's contradiction
display and indicator descriptions are clearly operator-oriented. The score
is limited by: no operator documentation exists; no --help flag; hard-gate
error messages are not actionable; internal type names (LabelClaim,
UntrustedLabelCandidate) appear in the operator display; and the trust tier
descriptions use internal jargon. None of these are architectural problems —
they are presentation and documentation gaps that can be closed without
changing the underlying system.

---

## Summary

```
Sections reviewed: 3 tabs (OS Information, Kernel Security, Trust / Evidence),
                   status bar, keymap, help popup, dialog system, error path,
                   available operator documentation
Tools evaluated: 1 (umrs-os-detect-tui — source only, no live binary)
Total findings: 18 (4 HIGH, 8 MEDIUM, 6 LOW)
```
