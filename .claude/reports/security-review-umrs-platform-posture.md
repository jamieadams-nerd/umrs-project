# Security Review: umrs-platform — posture module (Phase 1)

```
Audit date: 2026-03-14
Depth: in-depth
Scope: umrs-platform/src/posture/ (all 7 files),
       umrs-platform/tests/posture_tests.rs,
       umrs-platform/examples/posture_demo.rs,
       umrs-platform/src/kattrs/traits.rs (SecureReader engine — integration verification)
```

---

## Executive Summary

The posture module is well-structured and demonstrates a strong security posture overall. The foundational design decisions are sound: all `/proc/` and `/sys/` reads route through the provenance-verified `SecureReader` engine; the static catalog enforces compile-time path binding; fail-closed degradation is consistent; and `#[must_use]` coverage is thorough on security-critical types.

Three findings require attention before deployment. The most significant is a type-safety gap in the `perf_event_paranoid` reader: the kernel legitimately emits `-1` for this sysctl, which causes `parse_sysctl_u32` to return `Err` and silently degrade the signal to `live_value: None` — giving a false assurance picture. The second is a known-but-undocumented key normalization gap in `SysctlConfig` (slash-style keys in sysctl.d files are not matched against dot-style catalog keys). The third is a missing `#[must_use]` annotation on `PostureSnapshot::collect()`, which is the primary entry point.

Test coverage is good for unit-level logic but has significant gaps at the integration boundary: `parse_sysctl_line` is private and exercised only indirectly via temp-file workarounds, the `Sysrq` Custom logic is untested, and the sysctl.d key normalization gap has no regression test.

---

## Findings

### `umrs-platform/src/posture/reader.rs`

---

**Finding 1**

```
File: umrs-platform/src/posture/reader.rs
Location: parse_sysctl_u32 (shared parser, line ~97) and PerfEventParanoid expansion
Finding: kernel.perf_event_paranoid can legitimately be set to -1 ("unrestricted for
  all users"). The parser uses parse::<u32>() which returns Err on a negative value.
  When this node reads "-1\n", collect_one() degrades the signal to live_value: None
  with meets_desired: None. The operator sees "node absent" rather than "value is -1
  (unhardened)". This is a false-assurance failure: a misconfigured system reports
  no finding instead of a FAIL.
Severity: High
Control reference: NIST 800-53 CA-7 (Continuous Monitoring); NIST 800-53 SI-10 (Input
  Validation — rejects valid kernel output)
Remediation owner: coder
Recommended action: Change the parse type for perf_event_paranoid (and any other sysctl
  that can emit negative values) to i32, then widen to i64 or use a signed intermediate
  before comparing against the u32 desired threshold. Alternatively, introduce a
  parse_sysctl_i32 variant and a separate LiveValue::SignedInteger(i32) arm, then add
  an AtLeast comparison path for signed values. The desired value for PerfEventParanoid
  (AtLeast(2)) must compare correctly against -1 → false. At minimum, the parse failure
  on "-1" must map to LiveValue::Integer(u32::MAX) with meets_desired: Some(false) rather
  than degrading to None.
```

---

**Finding 2**

```
File: umrs-platform/src/posture/reader.rs
Location: line 434 — pub fn read_live_sysctl
Finding: read_live_sysctl is annotated #[must_use] but collect_one() calls it via
  read_live_sysctl_signal(), which is a private wrapper — the must_use on the public
  function does not protect internal callers. The annotation is correct but incomplete:
  all callers in snapshot.rs match the Result properly, so this is informational.
  The real gap is that the function returns Ok(None) for signals not handled by the
  dispatch match — which currently includes the non-sysctl signals. If a Sysctl-class
  signal were inadvertently excluded from the match arms, it would silently return
  Ok(None) and degrade instead of producing a compile error. The existing comment
  acknowledges this but does not fail loudly.
Severity: Low
Control reference: NSA RTB: Fail Secure
Remediation owner: coder
Recommended action: Add a debug-mode assertion or compile-time unreachable!() in the
  Ok(None) arm of read_live_sysctl_signal for Sysctl-class signals. The Ok(None) path
  in read_live_sysctl should only be reachable for the explicitly non-sysctl signal IDs
  listed in the last match arm. If a Sysctl-class signal falls through to Ok(None), that
  is a catalog/reader mismatch that should panic in debug builds.
```

---

### `umrs-platform/src/posture/configured.rs`

---

**Finding 3**

```
File: umrs-platform/src/posture/configured.rs
Location: parse_sysctl_line (line 273) and SysctlConfig::get (line 135) / NOTE in doc comment
Finding: sysctl.d files may use slash-style keys (e.g., kernel/kptr_restrict) in addition
  to dot-style keys (kernel.kptr_restrict). The catalog's sysctl_key fields use dot-style
  exclusively. The parser stores keys as-is without normalizing slashes to dots.
  SysctlConfig::get() performs an exact string match, so a sysctl.d file using slash-style
  keys will produce ConfiguredValue: None for every signal, silently suppressing all
  contradiction detection. This is documented in a code comment but is not surfaced to the
  operator and has no test coverage.
Severity: Medium
Control reference: NIST 800-53 CM-6 (Configuration Settings — configured-value lookup
  must be reliable); NIST 800-53 CA-7 (Continuous Monitoring — silent suppression of
  contradictions degrades monitoring)
Remediation owner: coder
Recommended action: Normalize keys at insertion time in load_conf_file: replace '/' with
  '.' in the key string before inserting into the map. Add a test case verifying that a
  sysctl.d file with slash-style keys (e.g., "kernel/kptr_restrict = 2") produces a
  ConfiguredValue for "kernel.kptr_restrict". Update the parse_sysctl_line doc comment to
  state that normalization is performed.
```

---

**Finding 4**

```
File: umrs-platform/src/posture/configured.rs
Location: load_conf_file (line 217), debug log at ~line 238
Finding: The debug-level log at configured.rs:238 emits the raw key=value content of
  every sysctl.d line parsed:
    log::debug!("posture: sysctl.d: {}:{} {}={}", source, line_no, key, value);
  In a DoD/CUI environment, debug-level logging may be enabled during troubleshooting.
  sysctl.d files can contain values that, while not secret, should not be emitted to
  a general-purpose log channel without consideration (e.g., a custom crypto FIPS key
  configuration). More importantly, 'value' is untrusted input from a filesystem path
  and could contain format strings or log-injection characters if a malicious sysctl.d
  file is placed on the system. The log macro uses positional arguments ({}) so format
  string injection is not possible in Rust — but the value is still emitted verbatim to
  the debug stream.
  This is a LOW-severity concern for log hygiene in the audit trail; the format-injection
  risk is mitigated by Rust's log macro semantics.
Severity: Low
Control reference: NIST 800-53 AU-9 (Protection of Audit Information); NIST 800-53
  SI-12 (Information Management and Retention)
Remediation owner: coder
Recommended action: Truncate or sanitize value output in the debug log. Consider logging
  only the key and source file at debug level, and omitting the raw value (or truncating
  to 16 chars). The full key=value pair should only appear in a trace-level log (if a
  trace level is added) or in an auditor-accessible record, not in the default debug
  stream.
```

---

### `umrs-platform/src/posture/snapshot.rs`

---

**Finding 5**

```
File: umrs-platform/src/posture/snapshot.rs
Location: line 107 — pub fn collect()
Finding: PostureSnapshot::collect() returns Self (PostureSnapshot), which carries
  #[must_use] at the type level. However, collect() itself does not carry a method-level
  #[must_use] annotation. The type-level annotation will trigger the lint only when the
  value is unused, but an explicit method-level annotation provides a clearer message at
  the call site and is required by the project's Must-Use Contract Rule ("all public
  functions that return a security-relevant type must carry #[must_use]").
Severity: Low
Control reference: NIST 800-53 SI-10; RTB: Fail Secure; project Must-Use Contract Rule
Remediation owner: coder
Recommended action: Add #[must_use = "posture snapshot contains security findings —
  examine before discarding"] to pub fn collect().
```

---

**Finding 6**

```
File: umrs-platform/src/posture/snapshot.rs
Location: lines 159, 167, 172, 179 — iter(), findings(), contradictions(), by_impact()
Finding: The four iterator methods that return security-relevant data do not carry
  #[must_use] annotations. These are the primary consumption path for posture findings.
  Calling snap.findings() without consuming the iterator is a silent no-op that an
  auditor scanning the codebase could mistake for an intentional discard. The project
  Must-Use Contract Rule requires all public functions returning security-relevant types
  to carry the annotation.
Severity: Low
Control reference: NIST 800-53 SI-10; project Must-Use Contract Rule
Remediation owner: coder
Recommended action: Add #[must_use = "findings iterator carries unhardened signals —
  examine each report"] to findings(), #[must_use = "..."] to contradictions(),
  #[must_use] to iter(), and #[must_use = "..."] to by_impact().
```

---

**Finding 7**

```
File: umrs-platform/src/posture/snapshot.rs
Location: read_live_cmdline_signal (line ~345)
Finding: For every cmdline signal that is not Lockdown, the live_value is set to
  LiveValue::Text(cmdline_str.to_owned()) — the full /proc/cmdline string. This means
  every cmdline signal in the snapshot stores a complete copy of the kernel cmdline.
  For a snapshot with 7 cmdline signals, this is 7 heap-allocated copies of the cmdline
  string (potentially hundreds of bytes each). More importantly, the cmdline may contain
  boot parameters considered sensitive in some deployment configurations (e.g., disk
  encryption key slots, IMA policy tokens). Storing the full cmdline in every cmdline
  signal report increases the exposure surface.
  This is a LOW-severity design concern, not a correctness bug; the cmdline is already
  read in full and the duplicate copy is benign in most environments.
Severity: Low
Control reference: NIST 800-53 SC-28 (Protection of Information at Rest); NIST 800-53
  SI-12 (Information Management)
Remediation owner: coder
Recommended action: Store only the matched token (or "absent"/"present" status) as the
  LiveValue for cmdline signals rather than the full cmdline string. Alternatively,
  document explicitly that SignalReport.live_value for cmdline signals contains the full
  cmdline and that callers must not log it in full at elevated log levels.
```

---

### `umrs-platform/src/posture/catalog.rs`

---

**Finding 8**

```
File: umrs-platform/src/posture/catalog.rs
Location: SignalDescriptor for Lockdown (live_path: "/sys/kernel/security/lockdown"),
  snapshot.rs read_live_lockdown()
Finding: The catalog entry for Lockdown has class: SignalClass::KernelCmdline but its
  live_path is /sys/kernel/security/lockdown (securityfs), not /proc/cmdline.
  read_live_cmdline_signal() special-cases this by branching on desc.id == SignalId::Lockdown
  before doing anything else. This works correctly at runtime but the class assignment
  is semantically misleading: KernelCmdline means "live value from /proc/cmdline,
  configured from bootloader entries." Lockdown's live value comes from securityfs
  (SECURITYFS_MAGIC), not procfs, and its persistence is not via cmdline key=value pairs
  but via a kernel LSM state file.
  The mismatch between the catalog class and the actual read path is a maintenance trap:
  a future developer adding a new KernelCmdline signal may not notice the special-case
  branch and may misclassify their signal. It also means contradiction detection for
  Lockdown (which returns None from configured_cmdline() in Phase 1) is silently
  suppressed without a clear rationale in the catalog entry.
Severity: Low
Control reference: NIST 800-53 CM-6; NSA RTB: Compile-Time Path Binding
Remediation owner: coder
Recommended action: Either (a) add a SignalClass::SecurityFs variant for signals read
  from /sys/kernel/security/, and update the Lockdown entry and dispatch, or (b) add an
  explicit comment to the Lockdown catalog entry and the KernelCmdline dispatch branch
  explaining the exception. Option (a) is preferred for long-term maintainability.
```

---

### `umrs-platform/examples/posture_demo.rs`

---

**Finding 9**

```
File: umrs-platform/examples/posture_demo.rs
Location: line 140
Finding: The example uses .expect("filtered to contradictions only") on
  report.contradiction. This is technically correct (the iterator only yields reports
  where contradiction.is_some()), but .expect() is not permitted by the project's
  #[deny(clippy::unwrap_used)] policy and its spirit extends to expect(). In an example
  that is compiled and run as documentation, this sets a bad precedent for callers.
Severity: Low
Control reference: Project coding rules — no unwrap()/expect() in compiled code
Remediation owner: coder
Recommended action: Replace .expect("...") with an if let Some(kind) = report.contradiction
  block or use .unwrap_or(ContradictionKind::SourceUnavailable) with a comment, or
  restructure the loop to pattern-match on the full SignalReport fields.
```

---

## Suggested Additional Test Cases

The following test functions should be added to `umrs-platform/tests/posture_tests.rs`. Each is described with sufficient specificity for the rust-developer to implement directly.

---

### A. `perf_event_paranoid` negative value handling

```rust
/// Verify parse_sysctl_u32 fails on the kernel-valid value "-1\n".
/// This is a regression test for the signed-integer type gap (Finding 1).
/// The test documents the CURRENT (broken) behavior; it should be updated
/// when Finding 1 is resolved to assert the correct parsed result.
#[test]
fn parse_sysctl_u32_negative_is_error_regression() {
    // kernel.perf_event_paranoid = -1 is a valid kernel state meaning
    // "unrestricted". The u32 parser currently returns Err for this input.
    // This test pins the current behavior. When the signed-integer fix lands,
    // this test should be replaced with one that asserts parse_sysctl_i32(b"-1\n") == Ok(-1).
    assert!(parse_sysctl_u32(b"-1\n").is_err(),
        "regression: negative sysctl values are not currently handled by u32 parser");
}
```

---

### B. sysctl.d slash-key normalization

```rust
/// Verify that a sysctl.d file using slash-style keys (kernel/kptr_restrict)
/// is found when queried with the dot-style key (kernel.kptr_restrict).
/// This is a regression test for the slash normalization gap (Finding 3).
/// The test will FAIL until Finding 3 is resolved — document it as such.
#[test]
fn sysctl_config_slash_key_is_normalized_to_dot() {
    use std::io::Write;
    use umrs_platform::posture::configured::SysctlConfig;

    // Write a temp sysctl.d conf file with a slash-style key.
    let tmp = tempfile::NamedTempFile::new().expect("tempfile");
    writeln!(tmp.as_file(), "kernel/kptr_restrict = 2").expect("write");

    // This is a white-box test that calls the internal parsing logic.
    // Because parse_sysctl_line is private, we test via SysctlConfig indirectly
    // using a fixture directory — or we expose parse_sysctl_line as pub(crate)
    // for testing. Until then, document the gap here.
    //
    // TODO(rust-developer): expose parse_sysctl_line as pub(crate) or add a
    // test-only constructor to SysctlConfig that accepts a BufRead, so slash
    // normalization can be tested without filesystem interaction.
    let _ = tmp;
}
```

*Note to rust-developer: `parse_sysctl_line` is private. Either expose it as `pub(crate)` for integration testing, or add a `SysctlConfig::from_reader` constructor that accepts a `BufRead` — this would also make the parser unit-testable without touching the filesystem.*

---

### C. `parse_sysctl_line` direct parser tests (requires `pub(crate)` exposure)

Once `parse_sysctl_line` is exposed as `pub(crate)`, add:

```rust
#[test]
fn parse_sysctl_line_dotted_key() {
    // Standard dot-style key
    assert_eq!(
        umrs_platform::posture::configured::parse_sysctl_line("kernel.kptr_restrict = 2"),
        Some(("kernel.kptr_restrict", "2"))
    );
}

#[test]
fn parse_sysctl_line_slash_key() {
    // Slash-style key — must produce the same logical key after normalization
    assert_eq!(
        umrs_platform::posture::configured::parse_sysctl_line("kernel/kptr_restrict = 2"),
        Some(("kernel/kptr_restrict", "2"))  // raw — normalization is caller's job
    );
}

#[test]
fn parse_sysctl_line_comment_is_none() {
    assert_eq!(
        umrs_platform::posture::configured::parse_sysctl_line("# kernel.kptr_restrict = 2"),
        None
    );
}

#[test]
fn parse_sysctl_line_empty_key_is_none() {
    assert_eq!(
        umrs_platform::posture::configured::parse_sysctl_line("= 2"),
        None
    );
}

#[test]
fn parse_sysctl_line_no_equals_is_none() {
    assert_eq!(
        umrs_platform::posture::configured::parse_sysctl_line("kernel.kptr_restrict"),
        None
    );
}

#[test]
fn parse_sysctl_line_value_with_spaces() {
    // Some values contain spaces (rare but valid in sysctl.conf syntax for comments)
    // The parser should take everything after the first '='.
    let result = umrs_platform::posture::configured::parse_sysctl_line("kernel.sysrq = 176");
    assert_eq!(result, Some(("kernel.sysrq", "176")));
}

#[test]
fn parse_sysctl_line_no_spaces_around_equals() {
    assert_eq!(
        umrs_platform::posture::configured::parse_sysctl_line("kernel.kptr_restrict=2"),
        Some(("kernel.kptr_restrict", "2"))
    );
}
```

---

### D. Sysrq `Custom` desired value logic

```rust
/// Verify the Sysrq signal uses DesiredValue::Custom in the catalog.
#[test]
fn catalog_sysrq_uses_custom_desired() {
    use umrs_platform::posture::signal::DesiredValue;
    let desc = SIGNALS.iter().find(|d| d.id == SignalId::Sysrq)
        .expect("Sysrq must be in catalog");
    assert_eq!(desc.desired, DesiredValue::Custom,
        "Sysrq must use Custom desired value for bitmask semantics");
}

/// Verify that the snapshot's collect() applies the default hardened check
/// for Sysrq (value == 0) when the custom logic runs.
/// This tests the hardcoded fallback in read_live_sysctl_signal.
#[test]
fn sysrq_custom_default_check_is_value_zero() {
    // DesiredValue::Custom.meets_integer() always returns None.
    let d = DesiredValue::Custom;
    assert_eq!(d.meets_integer(0), None,
        "Custom must return None from meets_integer — custom logic is caller's job");
    // The snapshot applies: Some(v == 0) for Custom signals.
    // We verify this by checking the snapshot report for Sysrq if readable.
    let snap = PostureSnapshot::collect();
    if let Some(report) = snap.get(SignalId::Sysrq) {
        if let Some(LiveValue::Integer(v)) = report.live_value {
            // Whatever the live value, meets_desired should be Some(v == 0).
            assert_eq!(report.meets_desired, Some(v == 0),
                "Sysrq meets_desired must be Some(live == 0)");
        }
    }
}
```

---

### E. `PostureSnapshot` timestamp and boot_id fields

```rust
/// Verify collected_at is a recent SystemTime (not UNIX_EPOCH or future).
#[test]
fn snapshot_collected_at_is_recent() {
    use std::time::{Duration, SystemTime};
    let snap = PostureSnapshot::collect();
    let now = SystemTime::now();
    let age = now.duration_since(snap.collected_at)
        .unwrap_or(Duration::ZERO);
    assert!(age < Duration::from_secs(10),
        "collected_at must be within 10 seconds of now");
    assert!(snap.collected_at <= now,
        "collected_at must not be in the future");
}

/// Verify boot_id, if present, is a non-empty string (UUID format is not
/// validated here — just that it is not blank).
#[test]
fn snapshot_boot_id_non_empty_if_present() {
    let snap = PostureSnapshot::collect();
    if let Some(ref id) = snap.boot_id {
        assert!(!id.is_empty(), "boot_id must not be empty if present");
        assert!(id.len() >= 32,
            "boot_id should be at least 32 chars (UUID without dashes)");
    }
    // boot_id may be None in container environments — that is acceptable.
}
```

---

### F. Contradiction detection with evaluate_configured_meets — overflow boundary

```rust
/// Verify evaluate_configured_meets handles u32::MAX configured value correctly.
#[test]
fn eval_configured_u32_max() {
    assert_eq!(
        evaluate_configured_meets("4294967295", &DesiredValue::AtMost(0)),
        Some(false)
    );
    assert_eq!(
        evaluate_configured_meets("4294967295", &DesiredValue::Exact(4294967295)),
        Some(true)
    );
}

/// Verify evaluate_configured_meets rejects overflow (u32::MAX + 1).
#[test]
fn eval_configured_overflow_is_none() {
    // 4294967296 cannot be parsed as u32 — should return None.
    assert_eq!(
        evaluate_configured_meets("4294967296", &DesiredValue::Exact(0)),
        None
    );
}
```

---

### G. Catalog: desired-value type consistency assertions

```rust
/// Every Sysctl-class signal must have an integer-compatible desired value
/// (Exact, AtLeast, AtMost, or Custom). No Sysctl signal should have a
/// CmdlinePresent or CmdlineAbsent desired value.
#[test]
fn catalog_sysctl_signals_have_integer_desired() {
    use umrs_platform::posture::signal::{DesiredValue, SignalClass};
    for desc in SIGNALS {
        if desc.class == SignalClass::Sysctl {
            match &desc.desired {
                DesiredValue::CmdlinePresent(_) | DesiredValue::CmdlineAbsent(_) => {
                    panic!("Sysctl signal {:?} has cmdline desired value", desc.id);
                }
                _ => {}
            }
        }
    }
}

/// Every KernelCmdline-class signal must have a cmdline or custom desired value.
#[test]
fn catalog_cmdline_signals_have_cmdline_desired() {
    use umrs_platform::posture::signal::{DesiredValue, SignalClass};
    for desc in SIGNALS {
        if desc.class == SignalClass::KernelCmdline {
            match &desc.desired {
                DesiredValue::Exact(_) | DesiredValue::AtLeast(_) | DesiredValue::AtMost(_) => {
                    // Lockdown is an exception — it uses securityfs, not /proc/cmdline.
                    // This assertion should be updated when SignalClass::SecurityFs is added.
                    if desc.id != umrs_platform::posture::signal::SignalId::Lockdown {
                        panic!(
                            "KernelCmdline signal {:?} has integer desired value", desc.id
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
```

---

### H. sysctl.d precedence — last-writer-wins

```rust
/// Verify that when two sysctl.d "files" define the same key, the second
/// (higher precedence) value wins. This tests the merge semantics without
/// filesystem I/O by exercising load_conf_file via a pub(crate) test helper.
///
/// NOTE: This test requires either pub(crate) exposure of load_conf_file,
/// or a SysctlConfig::from_reader test constructor. Until then, document
/// the gap here.
#[test]
#[ignore = "requires pub(crate) test constructor — see Finding 3 note"]
fn sysctl_config_last_writer_wins() {
    // Two sources for the same key — second should win.
    // Source A (lower precedence): kernel.kptr_restrict = 1
    // Source B (higher precedence): kernel.kptr_restrict = 2
    // Expected result: get("kernel.kptr_restrict") == Some(ConfiguredValue { raw: "2", source_file: "B" })
    todo!("implement after pub(crate) test constructor is available")
}
```

---

## Overall Assessment

**The posture module is deployment-ready with the caveat that Finding 1 (perf_event_paranoid type gap) and Finding 3 (slash-key normalization) must be resolved first.** Both affect the reliability of the posture assessment itself — the first causes a specific signal to produce false-assurance output; the second silently disables contradiction detection for systems using slash-style sysctl.d keys.

Findings 5 and 6 (`#[must_use]` gaps on `collect()` and the iterator methods) are straightforward two-line fixes and should be resolved in the same patch.

Finding 8 (Lockdown `SignalClass::KernelCmdline` mismatch) is a code-clarity and maintainability issue. Adding `SignalClass::SecurityFs` is a small change with meaningful long-term benefit for catalog correctness.

Finding 9 (`.expect()` in example) is a policy violation in compiled code and should be fixed to avoid setting the wrong precedent for callers.

The test suite covers the core logic well. The most important gaps are the unsigned integer/negative value regression (tied to Finding 1), the slash-normalization test (tied to Finding 3), and the `parse_sysctl_line` visibility barrier that prevents direct white-box testing of the parser.

**No raw `unwrap()` calls exist in production code.** No inline tests exist in source files. Compliance annotations are present at module, type, and function level throughout. The `SecureReader` provenance chain is correctly used for all kernel reads.

---

## Gap Analysis Summary

```
Files reviewed: 9
  (signal.rs, catalog.rs, reader.rs, configured.rs, contradiction.rs,
   snapshot.rs, mod.rs, posture_tests.rs, posture_demo.rs)
  + kattrs/traits.rs reviewed for SecureReader integration verification

Total findings: 9 (0 Critical, 1 High, 1 Medium, 7 Low)

Policy artifacts written: none (no SELinux policy changes required by this module)
Policy artifacts needed:  none (posture module reads procfs/securityfs only — existing
  allow rules for umrs-platform binary cover these reads)

Documentation gaps:
  - Lockdown SignalClass::KernelCmdline is a misleading class for a securityfs-read
    signal; should be documented or corrected (see Finding 8)
  - parse_sysctl_line slash-vs-dot limitation is documented in a code comment but not
    in the public API docs for SysctlConfig::get (see Finding 3)
  - perf_event_paranoid description in reader.rs mentions -1 as a valid kernel value
    but the parser silently fails on it (see Finding 1)

Code-vs-policy inconsistencies:
  - SignalId::Lockdown catalog entry: class = KernelCmdline, but live_path =
    /sys/kernel/security/lockdown (securityfs) — mismatched class vs. actual source
    filesystem (Finding 8)
  - PerfEventParanoid desired = AtLeast(2): comparing a signed kernel value (-1 possible)
    against an unsigned threshold via u32 parse (Finding 1)
```

---

*Remediation owners for each finding:*
| Finding | Severity | Owner |
|---|---|---|
| 1 — perf_event_paranoid signed integer gap | High | coder |
| 2 — Ok(None) silent degradation for Sysctl signals | Low | coder |
| 3 — sysctl.d slash-key normalization missing | Medium | coder |
| 4 — sysctl.d debug log emits raw values | Low | coder |
| 5 — collect() missing #[must_use] | Low | coder |
| 6 — iterator methods missing #[must_use] | Low | coder |
| 7 — cmdline live_value stores full cmdline per-signal | Low | coder |
| 8 — Lockdown SignalClass::KernelCmdline mismatch | Low | coder |
| 9 — .expect() in posture_demo.rs example | Low | coder |
