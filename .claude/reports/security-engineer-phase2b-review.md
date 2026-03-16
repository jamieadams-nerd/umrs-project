# Security Engineer Phase 2b Review — Kernel Security Posture Probe

```
Audit date: 2026-03-16
Depth: in-depth
Scope: Phase 2b additions to umrs-platform posture module
       - components/rusty-gadgets/umrs-platform/src/posture/bootcmdline.rs (new)
       - components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs (install directive additions)
       - components/rusty-gadgets/umrs-platform/src/posture/reader.rs (cmdline contradiction dispatch)
       - components/rusty-gadgets/umrs-platform/src/posture/catalog.rs (reviewed — no Phase 2b changes)
       - components/rusty-gadgets/umrs-platform/src/posture/snapshot.rs (KernelCmdline configured-value integration)
       - components/rusty-gadgets/umrs-platform/src/posture/configured.rs (configured_cmdline delegation)
       - components/rusty-gadgets/umrs-platform/src/posture/contradiction.rs (evaluate_configured_meets — cmdline path)
       - components/rusty-gadgets/umrs-platform/tests/posture_bootcmdline_tests.rs
       - components/rusty-gadgets/umrs-platform/tests/posture_modprobe_tests.rs (install directive tests)
```

---

## Executive Summary

Phase 2b is well-structured. The trust boundary reasoning is explicit and largely correct.
The modprobe `install` directive parsing is sound: the hard-blacklist sentinel detection is a
conservative fixed-set string comparison against known no-op command paths; the command string
is recorded but never executed. The `ModprobeConfig` type properly separates soft and hard
blacklists. Error discipline in both `bootcmdline.rs` and `modprobe.rs` is consistent with the
patterns established in Phase 1 and 2a.

Two findings require coder attention before deployment. The remaining findings are documentation
or low-severity posture issues.

---

## Findings

---

### File: `components/rusty-gadgets/umrs-platform/src/posture/bootcmdline.rs`

---

**Finding B-01**

```
File: components/rusty-gadgets/umrs-platform/src/posture/bootcmdline.rs
Location: line 204 — read_kernel_osrelease()
Finding: Raw std::fs::read_to_string on /proc/sys/kernel/osrelease without
         provenance verification (PROC_SUPER_MAGIC). The module comment at line
         195-202 explicitly justifies this as "NOT a provenance-verified read"
         because it is used for "heuristic entry matching — not for security
         decisions." This justification is partially correct but incomplete.

         The function is used by select_entry() to choose which BLS entry's
         options line is fed to the contradiction engine as the "configured
         cmdline". On a multi-kernel RHEL 10 system, a spoofed /proc/sys/kernel/
         osrelease would cause select_entry() to select a different (potentially
         attacker-crafted) BLS entry than the one that was actually booted.
         An attacker able to substitute a false osrelease value could therefore
         influence which boot options are compared against /proc/cmdline, creating
         a false-negative in EphemeralHotfix/BootDrift detection.

         The threat model must be stated explicitly: on an enforcing SELinux
         system, /proc/sys/kernel/osrelease is labeled proc_t and only readable
         by processes with the appropriate allow rule; it is not writable by
         unprivileged processes. The procfs VFS layer prevents userspace from
         injecting false content. So the unverified read is actually safe under
         the project's target deployment (SELinux enforcing + integrity lockdown),
         but this dependency is not documented.

         The missing documentation creates an audit gap: a reviewer cannot
         confirm whether the authors understood the risk or missed it. Under NIST
         SP 800-218 SSDF PW.4 / RAIN Non-Bypassability, security-relevant
         assumptions about the deployment environment must be explicit.
Severity: MEDIUM
Control reference: NIST SP 800-53 SI-7; NIST SP 800-218 SSDF PW.4; NSA RTB RAIN
Remediation owner: coder
Recommended action: Add an explicit security note to read_kernel_osrelease()
  documenting the trust dependency:
  1. The read is not provenance-verified because the osrelease value is used
     for heuristic BLS entry selection, not for a security assertion.
  2. State explicitly: the unverified read is safe because (a) procfs prevents
     userspace injection of false osrelease content, and (b) on a SELinux
     enforcing system with integrity lockdown, the kernel is the sole writer of
     /proc/sys/kernel/osrelease. The posture probe's correctness for entry
     selection is bounded by this deployment assumption.
  3. If posture probe is ever deployed without SELinux enforcing, this read
     should be upgraded to ProcfsText + PROC_SUPER_MAGIC to remove the
     assumption.
  This is a documentation fix, not a code change for the standard deployment
  target, but the note should match the pattern used for the is_module_loaded()
  SELinux dependency note in modprobe.rs (lines 627-639).
```

---

**Finding B-02**

```
File: components/rusty-gadgets/umrs-platform/src/posture/bootcmdline.rs
Location: line 239 — parse_bls_field(), std::fs::read_to_string(entry_path)
Finding: BLS entry file reads use std::fs::read_to_string rather than the
         SecureReader / ProcfsText pattern. The module doc comment at lines
         40-45 explicitly states this is intentional and correct: /boot/loader/
         entries/ is a regular filesystem path; BLS entries represent the
         *intended* configuration (advisory), not the *effective* kernel state;
         the live /proc/cmdline is always authoritative.

         This reasoning is sound and consistent with the trust boundary model
         established in Phase 1 for /etc/sysctl.d/ and /etc/modprobe.d/ reads.
         Advisory configured-value reads from regular filesystems are not
         required to use SecureReader.

         However, there is one structural gap: unlike the sysctl.d and modprobe.d
         readers, bootcmdline.rs does not document the file size bound it relies
         on. std::fs::read_to_string will read an entire file into memory.
         A crafted or corrupted /boot/loader/entries/*.conf that is extremely
         large (e.g., many megabytes) would cause a proportional allocation.
         The posture probe runs as part of the normal tool lifecycle; on a
         resource-constrained system this is a minor availability concern.

         This is distinct from the existing sysctl.d / modprobe.d readers, which
         read .conf files that in practice are small. BLS entry files are also
         small in practice, but no bound is stated.
Severity: LOW
Control reference: NIST SP 800-53 SI-10 (input validation — no stated size bound)
Remediation owner: coder
Recommended action: Add a comment to parse_bls_field() noting the implicit size
  assumption: BLS entry files are expected to be small (<64 KiB). The read will
  succeed on any realistic BLS entry; extreme size would indicate a corrupted or
  adversarially crafted entry but would not cause a security failure (the worst
  outcome is a transient memory allocation, after which the function returns None
  via the error path). This is informational documentation, not a code change.
  If size enforcement is desired, read with a capped buffer instead of
  read_to_string.
```

---

**Finding B-03**

```
File: components/rusty-gadgets/umrs-platform/src/posture/bootcmdline.rs
Location: lines 9-12 — module doc comment (## Scope section)
Finding: The module doc comment states "Phase 2b covers `options` and `blacklist`
         directives" in its Scope section. This text was carried forward from
         the modprobe.rs scope description and is incorrect for bootcmdline.rs —
         bootcmdline.rs has no concept of blacklist or options directives; it only
         reads BLS options lines.
Severity: LOW
Control reference: (none — documentation consistency)
Remediation owner: coder
Recommended action: Remove the stale "Phase 2a covers options and blacklist
  directives" scope text from bootcmdline.rs. The scope section should describe
  BLS entry parsing, the entry selection heuristic, and the advisory nature of
  the read.
```

Wait — re-reading bootcmdline.rs line 9-12 more carefully:

The text at lines 9-12 says:
```
//! Reads the **configured** kernel command line from bootloader entries on
//! disk, providing the "intended" cmdline for contradiction detection against
//! the live `/proc/cmdline`.
```

That is correct for bootcmdline.rs. The stale text in the Scope section is in `modprobe.rs` line 9-12:

```
//! ## Scope
//!
//! Phase 2a covers `options` and `blacklist` directives — both have
//! well-defined, deterministic formats. `install` and `softdep` directives
//! are logged at debug and excluded from the parameter map (Phase 2b).
```

That text is now stale — Phase 2b has shipped install directive support. The finding belongs to modprobe.rs.

---

### File: `components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs`

---

**Finding M-01**

```
File: components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs
Location: lines 9-12 — module doc comment Scope section
Finding: The Scope section states:
  "Phase 2a covers `options` and `blacklist` directives [...] `install` and
  `softdep` directives are logged at debug and excluded from the parameter map
  (Phase 2b)."

  This is now stale. Phase 2b has shipped install directive support. The module
  doc still describes install directives as excluded future work. A reviewer
  reading this text would incorrectly conclude that install directives are not
  handled.
Severity: LOW
Control reference: (none — documentation accuracy)
Remediation owner: coder
Recommended action: Update the Scope section to reflect Phase 2b completion:
  "Phase 2b adds `install` directive parsing. An `install <module> /bin/true`
  (or /bin/false, /usr/bin/true) directive is detected as a hard blacklist and
  recorded in `hard_blacklisted`. `softdep`, `alias`, and `remove` directives
  are logged at debug and excluded."
```

---

**Finding M-02**

```
File: components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs
Location: line 593 — parse_install_directive(), hard-blacklist sentinel set
Finding: The hard-blacklist sentinel set is:
  /bin/true | /usr/bin/true | /bin/false | /usr/bin/false

  The implementation correctly describes this as matching the "first token of
  the command" (line 589: cmd_executable = command.split_whitespace().next()).
  This avoids false positives from complex command strings like:
  `install pcspkr /sbin/modprobe --ignore-install pcspkr && /bin/true`

  However, the sentinel set does not include `/usr/bin/false`. On modern Linux
  (FHS 3.0+ with usr-merge, which is the standard on RHEL 9+/RHEL 10), both
  /bin and /usr/bin coexist as aliases (or /bin is a symlink to /usr/bin). An
  operator writing `/usr/bin/false` as the install command would receive
  is_hard_blacklist = false, the directive would be classified as a "complex
  command — not a hard blacklist", and the module would NOT appear in
  hard_blacklisted.

  The test at line 152 (`parse_install_usr_bin_true_is_hard_blacklist`) covers
  /usr/bin/true but there is no corresponding test for /usr/bin/false.

  On RHEL 10 (usr-merge active), /usr/bin/false is as valid a hard-blacklist
  sentinel as /bin/false. The omission creates an asymmetry: true has usr/bin
  coverage; false does not. Security-relevant: if an operator writes
  `install thunderbolt /usr/bin/false` and the probe doesn't recognise it,
  ThunderboltBlacklisted hard_blacklisted evidence is silently absent.
Severity: MEDIUM
Control reference: NIST SP 800-53 CM-7; NIST SP 800-53 SI-10
Remediation owner: coder
Recommended action:
  1. Add "/usr/bin/false" to the hard-blacklist sentinel set in
     parse_install_directive() (line 590-593):
       let is_hard_blacklist = matches!(
           cmd_executable,
           "/bin/true" | "/usr/bin/true" | "/bin/false" | "/usr/bin/false"
       );
  2. Add a test: parse_install_usr_bin_false_is_hard_blacklist.
  3. Update the doc comment on parse_install_directive() to list all four
     recognised sentinels explicitly.
```

---

**Finding M-03**

```
File: components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs
Location: lines 626-646 — is_module_loaded()
Finding: The function comment at lines 627-639 documents the SELinux MAC
  enforcement dependency: the metadata-only is_dir() check is safe as a trust
  gate because SELinux enforcing mode prevents symlink substitution attacks in
  sysfs-labeled directories. The comment is well-written and explicit.

  There is a secondary trust boundary issue not addressed by the comment: the
  /sys/module/<name>/ path is constructed by joining SYS_MODULE_BASE with
  module_name where module_name is a string from the parsed modprobe.d files
  (or from the catalog, for the blacklist signal dispatch path).

  For catalog-provided module names (bluetooth, usb_storage, firewire_core,
  thunderbolt), the strings are compile-time constants — no runtime injection
  risk. For the general-purpose public API (is_module_loaded and
  read_module_param accept arbitrary &str), a crafted module_name containing
  path traversal characters (e.g., "../net/..") could potentially probe
  unexpected sysfs paths.

  The existing empty-string guard at line 642 protects one edge case. However,
  the function does not reject module names containing '/' or other path
  traversal characters. On SELinux enforcing with SYSFS_MAGIC provenance
  verification in read_module_param, the actual parameter read would be bounded
  to the sysfs hierarchy. But is_module_loaded does not use provenance
  verification — the is_dir() call could resolve a traversal to outside
  /sys/module/ if SELinux happens not to cover the target directory.

  In the current usage, all callers are internal, and the catalog-origin module
  names are compile-time constants. But is_module_loaded and read_module_param
  are pub — they are part of the public API surface. A caller passing an
  adversary-controlled module name could cause unexpected filesystem probing.
Severity: MEDIUM
Control reference: NIST SP 800-53 SI-10 (input validation); NIST SP 800-53 CM-7
Remediation owner: coder
Recommended action: Add input validation to is_module_loaded() and
  read_module_param() rejecting module_name values containing '/', '\0', or
  '..' components. A minimal guard:
    if module_name.contains('/') || module_name.contains('\0') {
        return false; // or Err(io::ErrorKind::InvalidInput)
    }
  Document in the function comment that module names are validated against
  path-traversal characters before use. Alternatively, accept a typed
  ModuleName newtype that enforces validity at construction (Validate at
  Construction pattern).

  NOTE: The compile-time catalog callers are not affected by this gap in
  practice. The finding is against the public API surface, not the current
  internal callers.
```

---

### File: `components/rusty-gadgets/umrs-platform/src/posture/snapshot.rs`

---

**Finding S-01 (HIGH)**

```
File: components/rusty-gadgets/umrs-platform/src/posture/snapshot.rs
Location: lines 696-720 — read_configured_boot_cmdline()
Finding: The configured value for KernelCmdline signals is stored as the raw
  full BLS options string (line 718: raw: boot_opts.to_owned()). The function
  comment at lines 700-716 explicitly acknowledges that evaluate_configured_meets()
  will return None for this raw value (it is not an integer), and states:
  "contradiction detection will not fire on it" and "a dedicated contradiction
  path for cmdline signals is out of scope for Phase 2b."

  This means: for all six KernelCmdline signals (ModuleSigEnforce, Mitigations,
  Pti, RandomTrustCpu, RandomTrustBootloader, and the SecurityFs Lockdown signal),
  BootDrift and EphemeralHotfix contradictions are NEVER produced even when
  configured_value is populated from BLS. The contradiction engine sees
  configured_meets = None (because evaluate_configured_meets returns None for
  any non-integer, non-"blacklisted" raw value) and classify() short-circuits
  to None when configured_meets is None.

  The effect: read_configured_cmdline() was implemented (Phase 2b goal achieved
  for data collection), but the contradiction detection that justifies its
  existence (CA-7, CM-6) is silently non-functional for cmdline signals.

  An operator reviewing the snapshot output will see configured_value populated
  for ModuleSigEnforce, Mitigations, etc., but will NEVER see a BootDrift
  contradiction for these signals, even in the obvious case where:
    - /boot/loader/entries/rhel.conf options line has module.sig_enforce=1
    - /proc/cmdline does NOT have module.sig_enforce=1 (boot drift)

  This is a silent correctness gap, not a crash or error. But it means the
  CA-7 monitoring objective for cmdline signals is unmet: the probe can detect
  that module.sig_enforce=1 is absent from the live cmdline but cannot detect
  that it IS present in the configured cmdline (i.e., the BootDrift case).

  The HIGH severity rating is warranted because:
  1. The configured_value field is populated — operators will see it and assume
     contradiction detection is active.
  2. BootDrift for module.sig_enforce, mitigations, or pti=off are high-impact
     security findings that must not be silently suppressed.
  3. The gap was self-identified in the code comment but was deferred without a
     tracking mechanism visible outside the source code.
Severity: HIGH
Control reference: NIST SP 800-53 CA-7; NIST SP 800-53 CM-6; NIST SP 800-53 AU-3
Remediation owner: coder
Recommended action: Implement a dedicated configured-meets evaluation path for
  KernelCmdline signals. The required logic is:
    1. In evaluate_configured_meets() or in a new evaluate_configured_meets_cmdline():
       given the raw boot options string and a DesiredValue::CmdlinePresent(token)
       or CmdlineAbsent(token), test whether the token is present in the raw
       string using the same whitespace-delimited token check used by
       CmdlineReader::contains_token().
    2. Return Some(true) if the raw string satisfies the desired value; Some(false)
       if it does not.
    3. In collect_one() (snapshot.rs), for SignalClass::KernelCmdline signals,
       compute configured_meets using this dedicated path rather than the generic
       evaluate_configured_meets().
  This enables classify() to produce BootDrift when:
    - /boot/loader/entries/ options contains "module.sig_enforce=1" (configured
      hardened) but /proc/cmdline does not (live unhardened).
  And EphemeralHotfix when:
    - /proc/cmdline has "module.sig_enforce=1" (live hardened) but the BLS
      options do not (configured not hardened — ephemeral runtime injection).
  Add regression tests for both the BootDrift and EphemeralHotfix cases for at
  least ModuleSigEnforce and Mitigations.
```

---

**Finding S-02**

```
File: components/rusty-gadgets/umrs-platform/src/posture/snapshot.rs
Location: lines 289-294 — collect_one() release-mode debug log
Finding: The release-mode debug log at lines 287-294 correctly omits configured
  values (c.raw). This is consistent with Error Information Discipline from
  Phase 2a. No issue.

  However, the live_value is included in the release-mode log at line 290:
    log::debug!(
        "posture: {:?} live={:?} meets={:?} contradiction={:?}",
        desc.id,
        live_value,      ← this includes LiveValue::Text content
        ...
    )
  For KernelCmdline signals, live_value is LiveValue::Text(token_value) where
  token_value is either the matched cmdline token (e.g., "module.sig_enforce=1")
  or the literal string "absent".

  On cmdline signals, the token values are compile-time constants from the
  catalog desiredvalue (e.g., "fips=1", "mitigations=off"), not operator-supplied
  data. These are not sensitive values. So this is not a violation of Error
  Information Discipline for the current signal set.

  The concern is forward-looking: if a future signal stores a LiveValue::Text
  derived from actual kernel output (e.g., core_pattern), the release-mode log
  would emit it. The existing discipline in read_live_cmdline_signal() that
  stores only the matched token (not the full cmdline) already mitigates this
  for cmdline signals. Document this as a known boundary to enforce for future
  Text-valued signals.
Severity: LOW
Control reference: NIST SP 800-53 SI-11; NSA RTB Error Discipline
Remediation owner: coder
Recommended action: Add a comment in collect_one() near the release-mode debug
  log noting:
  "live_value is intentionally logged in release builds because current
  Text-valued signals store only compile-time catalog tokens, not raw kernel
  output. If a future signal stores kernel-supplied text in LiveValue::Text,
  this log line must be gated under #[cfg(debug_assertions)] for that signal."
  No code change required now; the note guards the forward-looking boundary.
```

---

### File: `components/rusty-gadgets/umrs-platform/src/posture/contradiction.rs`

---

**Finding C-01**

```
File: components/rusty-gadgets/umrs-platform/src/posture/contradiction.rs
Location: lines 143-203 — evaluate_configured_meets()
Finding: The function correctly handles the "blacklisted" sentinel (lines 179-181)
  and signed integer fallback (lines 186-203). Both were findings from the Phase 2a
  review and are confirmed resolved.

  For Phase 2b: the function returns None for any non-integer, non-"blacklisted"
  raw value. The module doc comment at lines 30-36 explicitly documents this for
  the FIPS cross-check string. The Phase 2b BLS options string also returns None
  here, as confirmed above (Finding S-01).

  The doc comment at lines 144-168 does NOT document the KernelCmdline / BLS
  options string case — only the FIPS string and blacklist sentinel are mentioned.
  This leaves a gap: a reviewer cannot tell whether the BLS options string
  returning None is intentional behavior or an overlooked case.
Severity: LOW
Control reference: NIST SP 800-53 AU-3 (Security Findings as Data — documentation
  completeness)
Remediation owner: coder
Recommended action: Extend the doc comment on evaluate_configured_meets() to
  document the KernelCmdline / BLS options string case:
  "## KernelCmdline (BLS options string)
  For KernelCmdline signals, the configured raw value is the full BLS options
  string (e.g., 'root=UUID=abc fips=1 module.sig_enforce=1'). This function
  returns None for such values — token-based evaluation for these signals is
  handled via a dedicated path in collect_one(). See read_configured_boot_cmdline()
  and Finding S-01 in the Phase 2b security review."
  This aligns the comment with the FIPS pattern already documented.
  (Note: the dedicated path referenced in this doc comment must be implemented
  per Finding S-01 before this comment is accurate.)
```

---

### Test Coverage Assessment

---

**Finding T-01**

```
File: components/rusty-gadgets/umrs-platform/tests/posture_bootcmdline_tests.rs
Location: tests 1-5 (graceful degrade tests)
Finding: The bootcmdline tests are structurally correct but are environment-
  conditional: most assertions only fire when /boot/loader/entries/ is present.
  In containers and CI, the tests are effectively no-ops.

  The core parser logic (parse_bls_field, comment skipping, blank line handling)
  is tested by reading temp files manually in tests 2-5 but through the test's
  own re-implementation of the parsing logic, not through the actual private
  parse_bls_field function. The tests verify the temp file content, not that
  bootcmdline.rs's parse_bls_field produces the same result.

  The public API (read_configured_cmdline) operates on a fixed path
  (/boot/loader/entries/) and cannot be redirected in tests. This means the
  parser logic in parse_bls_field is not directly tested in CI environments
  where BLS is absent.

  This is a structural constraint of the current design (fixed path constant,
  no injection point). It is noted as an ongoing coverage gap rather than a
  defect, since the graceful-degrade behavior is correctly tested and the manual
  temp-file tests verify the format understanding is correct.
Severity: LOW
Control reference: NIST SP 800-218 SSDF PW.4 (test coverage completeness)
Remediation owner: coder
Recommended action: Consider introducing a testable entry point — either a
  pub(crate) function that accepts a path argument for the BLS directory
  (allowing tests to point at a tempdir), or a pub fn parse_bls_options_from_str
  that exercises the parser logic directly. This would allow full parser
  coverage in any environment without depending on /boot/loader/entries/
  existing. Alternatively, accept the current coverage level as a known gap
  documented here.
```

---

## Contradiction Detection — Trust Boundary Analysis Summary

### bootcmdline read paths

| Path | Method | Provenance Verified? | Justification |
|---|---|---|---|
| `/boot/loader/entries/` dir listing | `std::fs::read_dir` | No | Advisory configured-value; consistent with sysctl.d/modprobe.d pattern |
| `/boot/loader/entries/*.conf` file content | `std::fs::read_to_string` | No | Same as above — regular filesystem, not pseudo-fs |
| `/proc/sys/kernel/osrelease` | `std::fs::read_to_string` | No | Heuristic entry selection only; not a security assertion; SELinux dependency undocumented (Finding B-01) |

The advisory configured-value pattern (no provenance verification for /etc/ and /boot/ paths) is consistent and well-justified in the codebase. The gap is documentation of the SELinux enforcement dependency for osrelease (Finding B-01).

### modprobe install directive path

| Path | Method | Provenance Verified? | Justification |
|---|---|---|---|
| `/etc/modprobe.d/*.conf` reads | `std::fs::read_to_string` | No | Advisory configured-value; same pattern as sysctl.d |
| `install <module> <command>` classification | String comparison (fixed sentinel set) | N/A | Command string is never executed — only compared |
| `/sys/module/<name>/` dir presence | `Path::is_dir()` | No (SELinux-bounded) | Trust gate only; documented SELinux dependency |
| `/sys/module/<mod>/parameters/<param>` | SysfsText + SYSFS_MAGIC | Yes | Provenance-verified through SecureReader |

### TOCTOU assessment

- No second path-based re-open pattern observed in Phase 2b code.
- `is_module_loaded` uses a single `is_dir()` call (trust gate); `read_module_param` does a fresh open via SysfsText; no file descriptor is shared between the two, which creates a theoretical TOCTOU window. However: (a) the module directory cannot disappear from sysfs once the module is loaded (kernel reference counting prevents this); (b) SELinux prevents sysfs entry manipulation; (c) this window was accepted in Phase 2a. No new TOCTOU exposure in Phase 2b.

### Attack surface — modprobe.d install directive parsing

An adversary with write access to `/etc/modprobe.d/` (root-equivalent privilege) could:
1. Write an `install <module> <complex command>` directive — this will NOT be classified as a hard blacklist (is_hard_blacklist = false) and will NOT affect the blacklist maps. The module remains not-hard-blacklisted from the probe's perspective.
2. Write `install <module> /bin/true` — this WILL be classified as a hard blacklist. The probe would record the module as hard-blacklisted.
3. Write `install <module> /usr/bin/false` (note the gap in Finding M-02) — this will NOT be classified as hard blacklist. The probe would miss the hard-blacklist evidence.

In all cases: the attacker already has root. The probe is a monitor, not an enforcement mechanism. These findings affect monitoring fidelity, not security enforcement.

---

## Gap Analysis Summary

```
Files reviewed: 10
Total findings: 8 (0 CRITICAL, 1 HIGH, 2 MEDIUM, 5 LOW)

HIGH findings:
  S-01: KernelCmdline configured-value contradiction detection is non-functional
        (configured_meets always None for BLS options strings; BootDrift/
        EphemeralHotfix never produced for cmdline signals despite configured_value
        being populated).

MEDIUM findings:
  B-01: read_kernel_osrelease() uses raw std::fs::read_to_string on /proc/sys/kernel/
        osrelease without provenance verification; SELinux enforcement dependency
        is undocumented (heuristic entry selection, not a security assertion, but
        the trust assumption must be stated).
  M-03: is_module_loaded() and read_module_param() public API accepts arbitrary
        module name strings without path-traversal validation; current callers are
        safe (catalog constants), but the public surface is unguarded.

LOW findings:
  B-02: No stated file size bound in parse_bls_field(); implicit assumption that
        BLS entry files are small.
  M-01: modprobe.rs module Scope doc comment is stale — install directives are
        now handled in Phase 2b but the comment still describes them as excluded
        future work.
  M-02: /usr/bin/false missing from hard-blacklist sentinel set; asymmetry with
        /usr/bin/true which is present; on usr-merge systems this creates a false
        negative for operators using /usr/bin/false.
  S-02: LiveValue::Text in release-mode debug log is safe for current signals but
        must be gated for future signals that store kernel-supplied text content.
  C-01: evaluate_configured_meets() doc comment does not document the BLS options
        string case, leaving the None return for KernelCmdline signals undocumented.
  T-01: bootcmdline parser logic (parse_bls_field) not directly testable in
        environments without /boot/loader/entries/; CI coverage is effectively
        no-op for most tests.

Policy artifacts written: none (Phase 2b adds no new installed file types requiring
  SELinux type assignments)
Policy artifacts needed: none — posture module runs as part of the tool binary;
  the existing tool binary SELinux type assignments cover it.

Documentation gaps:
  - modprobe.rs Scope section stale (Finding M-01)
  - read_kernel_osrelease() missing SELinux trust assumption note (Finding B-01)
  - evaluate_configured_meets() doc missing KernelCmdline case (Finding C-01)
  - collect_one() release-mode log forward-looking boundary note (Finding S-02)

Code-vs-policy inconsistencies:
  - None. The trust boundary reasoning in the code is consistent with the
    deployed SELinux policy posture described in the module comments.

Remediation priority order:
  1. S-01 (HIGH) — implement KernelCmdline contradiction detection path; add
     regression tests. This is the primary Phase 2b functional gap.
  2. M-02 (MEDIUM/LOW boundary) — add /usr/bin/false to sentinel set; add test.
     Simple one-line fix with low risk.
  3. M-03 (MEDIUM) — add path-traversal validation to public module name API.
     Low complexity; guards against future callers outside the catalog path.
  4. B-01 (MEDIUM) — document SELinux enforcement dependency in
     read_kernel_osrelease(). Documentation only; no code change required.
  5. Remaining LOW findings — documentation and comment updates.
```

---

## Post-Review Notes for the Security Engineer

The Phase 2b implementation is architecturally sound. The trust boundary commentary is one of
the best-documented in the entire posture module: the explicit SELinux enforcement dependency
note in `is_module_loaded()` (lines 627-639 of modprobe.rs) is the standard the rest of the
codebase should follow.

Finding S-01 is the only blocker. The data collection plumbing is there; the contradiction
engine evaluation path for `KernelCmdline` class signals was consciously deferred in the code
comment but without a task to track it. With S-01 resolved, Phase 2b delivers the full
BootDrift/EphemeralHotfix monitoring capability for cmdline-class signals that CA-7 requires.

Finding M-02 (`/usr/bin/false` missing) is a one-line fix but raises monitoring fidelity for
DMA-surface blacklists on RHEL 10 (usr-merge active). Recommend pairing it with S-01 in the
same coder pass.
