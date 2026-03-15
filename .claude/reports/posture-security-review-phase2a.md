# Security Review — Kernel Security Posture Probe Phase 2a (Comprehensive)

```
Audit date: 2026-03-14
Depth: in-depth
Scope: components/rusty-gadgets/umrs-platform/src/posture/ (all 9 files:
         mod.rs, signal.rs, catalog.rs, reader.rs, configured.rs,
         snapshot.rs, contradiction.rs, modprobe.rs, fips_cross.rs)
       components/rusty-gadgets/umrs-platform/tests/posture_tests.rs
       components/rusty-gadgets/umrs-platform/tests/posture_modprobe_tests.rs
       components/rusty-gadgets/umrs-platform/tests/posture_fips_tests.rs
       components/rusty-gadgets/umrs-platform/examples/posture_demo.rs
Reviewer: security-engineer
```

---

## Summary Assessment

**Pass with findings.**

The posture module is architecturally sound. Every `/proc/` and `/sys/` read
routes through the provenance-verified `SecureReader` engine. The static
catalog enforces compile-time path binding. Fail-closed degradation is
consistent throughout. The blacklist sentinel fix (`"blacklisted"` handling
in `evaluate_configured_meets`) previously identified as HIGH is confirmed
implemented and correct. FIPS trust gate is correctly applied.

Seven findings are recorded below. One is **HIGH** (debug log leaks raw
configured values from the entire catalog), four are **MEDIUM**, and two are
**LOW**. None prevent correct operation in normal use, but the HIGH finding
violates Error Information Discipline on DoD/CUI systems and must be
resolved before deployment.

---

## Findings by File

---

### `snapshot.rs`

---

#### F-01 — HIGH: `collect_one` debug log emits raw configured value for every signal

**File:** `components/rusty-gadgets/umrs-platform/src/posture/snapshot.rs`
**Location:** line 263–270 (`collect_one`)
**Severity:** HIGH
**Control reference:** NIST SP 800-53 SI-11; NSA RTB Error Discipline
**Remediation owner:** coder

**Description:**

`collect_one` unconditionally emits the following at `log::debug!` for every
signal in the catalog:

```rust
log::debug!(
    "posture: {:?} live={:?} meets={:?} configured={:?} contradiction={:?}",
    desc.id,
    live_value,
    meets_desired,
    configured_value.as_ref().map(|c| &c.raw),  // <-- raw configured value
    contradiction
);
```

This log line is not gated on `#[cfg(debug_assertions)]`. It runs in release
builds whenever the host logging framework has `debug` enabled. On a DoD/CUI
system where an operator enables debug logging for troubleshooting (a routine
activity), this line emits the raw configured value for every one of the 27
catalog signals, including:

- `FipsEnabled` configured value (a summary of `/etc/system-fips` presence,
  cmdline tokens, and crypto-policy name — discloses the system's
  cryptographic posture).
- `NfConntrackAcct` configured value (a numeric parameter from modprobe.d).
- All sysctl.d values from the sysctl merge tree.

The `configured.rs` and `modprobe.rs` loaders explicitly suppress raw values
from their debug output, citing SI-11/SC-28 Error Information Discipline
(e.g., `configured.rs:248–256`). `collect_one` bypasses this discipline by
logging the fully assembled `ConfiguredValue::raw` field after the fact.

Note: `read_configured_modprobe` in the same file (`snapshot.rs:665–671`)
also logs `nf_conntrack` configured values with their raw string. That log
is also outside `#[cfg(debug_assertions)]`.

**Recommended action:**

Option A (preferred): Gate the `collect_one` summary log behind
`#[cfg(debug_assertions)]` to match every other timing/value log in the
module. This is consistent with the pattern established throughout
`reader.rs`, `modprobe.rs`, `configured.rs`, and `fips_cross.rs`.

Option B: Log only the signal ID, meets_desired status, and contradiction
kind — not the raw configured value — in the non-gated path.

The `read_configured_modprobe` NfConntrackAcct raw-value log at line 665
should receive the same treatment.

---

#### F-02 — MEDIUM: `catalog_covers_all_signal_ids` in `posture_tests.rs` does not cover Phase 2a signals

**File:** `components/rusty-gadgets/umrs-platform/tests/posture_tests.rs`
**Location:** line 31–62 (`catalog_covers_all_signal_ids`)
**Severity:** MEDIUM
**Control reference:** NIST SP 800-53 CA-7 (monitoring scope); NIST SP 800-218 SSDF PW.4
**Remediation owner:** coder

**Description:**

The `catalog_covers_all_signal_ids` test in `posture_tests.rs` enumerates
only the Phase 1 22-signal list. The five Phase 2a variants
(`NfConntrackAcct`, `BluetoothBlacklisted`, `UsbStorageBlacklisted`,
`FirewireCoreBlacklisted`, `ThunderboltBlacklisted`) are absent from its
`all_ids` array.

The associated `catalog_length_matches_signal_id_count` test at line 79
correctly asserts `SIGNALS.len() == 27` (Phase 1 + Phase 2a), so the
catalog count is verified. However, `catalog_covers_all_signal_ids` is the
exhaustive-match guard that forces this test to be updated when `SignalId`
gains a new variant. With the Phase 2a variants absent from the array, a
future developer could add a new `SignalId` variant, add it to the catalog,
and not notice that `catalog_covers_all_signal_ids` fails to exercise it.

The Phase 2a signals are covered by `posture_modprobe_tests.rs::catalog_covers_phase_2a_modprobe_signals`,
but only in that separate file. The primary catalog completeness test in
`posture_tests.rs` is the first-look test for most reviewers and should be
authoritative.

**Recommended action:**

Add all five Phase 2a variants to the `all_ids` array in
`catalog_covers_all_signal_ids`. Update the comment from
`"// Phase 1: 22 signals"` to reflect the full set. This also ensures the
test array length stays in sync with `catalog_length_matches_signal_id_count`.

---

### `snapshot.rs` / `reader.rs` / `configured.rs` / `modprobe.rs`

---

#### F-03 — MEDIUM: Three bare `#[must_use]` annotations without message strings

**Files:**
- `components/rusty-gadgets/umrs-platform/src/posture/reader.rs` — lines 440, 446
- `components/rusty-gadgets/umrs-platform/src/posture/configured.rs` — line 146
- `components/rusty-gadgets/umrs-platform/src/posture/snapshot.rs` — lines 204, 210
**Severity:** MEDIUM
**Control reference:** NIST SP 800-53 SI-10, SA-11; project Must-Use Contract Rule
**Remediation owner:** coder

**Description:**

The project Must-Use Contract Rule requires: "The `#[must_use]` annotation
must include a message string explaining why the return value matters. Bare
`#[must_use]` without a message is non-compliant."

The following annotations are bare:

- `reader.rs:440` — `CmdlineReader::as_str()`
- `reader.rs:446` — `CmdlineReader::contains_token()`
- `configured.rs:146` — `SysctlConfig::key_count()`
- `snapshot.rs:204` — `PostureSnapshot::readable_count()`
- `snapshot.rs:210` — `PostureSnapshot::hardened_count()`

These are not security-critical functions individually, but
`readable_count()` and `hardened_count()` feed into the summary metrics
displayed to operators and auditors. A caller that inadvertently discards
these return values gets no compile-time warning about the lapse.

**Recommended action:**

Add descriptive message strings to each annotation. Examples:

```rust
// reader.rs
#[must_use = "cmdline content is used for token lookup — discarding loses boot parameter context"]
pub fn as_str(&self) -> &str { ... }

#[must_use = "token presence check drives cmdline signal evaluation — discarding silently drops the result"]
pub fn contains_token(&self, token: &str) -> bool { ... }

// configured.rs
#[must_use = "key count is used to verify sysctl.d load success"]
pub fn key_count(&self) -> usize { ... }

// snapshot.rs
#[must_use = "readable_count feeds operator summary and audit metrics"]
pub fn readable_count(&self) -> usize { ... }

#[must_use = "hardened_count feeds operator summary and audit metrics"]
pub fn hardened_count(&self) -> usize { ... }
```

---

#### F-04 — MEDIUM: `Path::exists()` used for sysctl.d and modprobe.d directory guards

**Files:**
- `components/rusty-gadgets/umrs-platform/src/posture/configured.rs` — lines 94, 104
- `components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs` — line 158
- `components/rusty-gadgets/umrs-platform/src/posture/fips_cross.rs` — line 266
**Severity:** MEDIUM
**Control reference:** NIST SP 800-53 SI-10; project recurring gap pattern (TOCTOU)
**Remediation owner:** coder

**Description:**

`Path::exists()` is a check-then-use pattern vulnerable to TOCTOU races.
Between the `.exists()` check and the subsequent `std::fs::read_dir()` or
`std::fs::read_to_string()` call, a directory or file can be replaced,
removed, or symlinked to a different target by another process.

Affected sites:

1. `configured.rs:94` — `if dir_path.exists()` before `load_conf_dir`
2. `configured.rs:104` — `if legacy.exists()` before `load_conf_file`
3. `modprobe.rs:158` — `if dir_path.exists()` before `load_conf_dir`
4. `fips_cross.rs:266` — `Path::new(SYSTEM_FIPS_MARKER).exists()` for the
   FIPS marker check

**Risk assessment:**

These are reads from `/etc/` and `/usr/lib/` — regular files and directories
owned by root on a hardened system. The practical exploitability on a
correctly deployed, SELinux-enforcing RHEL 10 system is low. However, this
is a configuration-advisory read path that determines what the posture probe
reports to operators. An adversary with write access to `/run/sysctl.d/`
(possible in some container configurations) could race the `exists()` check.

More critically, this pattern violates the project's recurring gap rule: "A
file selected via `statx` in one phase and re-opened by path in a later phase
is a TOCTOU window." The same principle applies to existence checks.

**Recommended action (priority: medium, no live security data at risk):**

For `/etc/sysctl.conf` and the FIPS marker: replace `exists()` with a
`std::fs::metadata()` call that produces a `std::io::Error(NotFound)` as the
non-existence signal. The file read itself is still by path, but this
collapses the two-step check-then-open to a single `read_to_string` call with
`NotFound` handled gracefully:

```rust
// Before (two-step):
if legacy.exists() {
    let _ = load_conf_file(legacy, &mut map);
}

// After (single-step, fail-closed):
match load_conf_file(legacy, &mut map) {
    Ok(_) | Err(ref e) if e.kind() == io::ErrorKind::NotFound => {}
    Err(e) => log::debug!("posture: sysctl.conf: {e}"),
}
```

For the directory guards: `std::fs::read_dir()` already returns `NotFound`
if the directory does not exist. The `exists()` guard is therefore
redundant — remove it and handle `NotFound` in the `read_dir` error match.

This pattern is already used correctly in `reader.rs::BootIdReader::read()`
(line 484: `Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None)`).

---

#### F-05 — MEDIUM: `evaluate_configured_meets` silently produces `None` for negative sysctl.d configured values (e.g., `perf_event_paranoid = -1`)

**File:** `components/rusty-gadgets/umrs-platform/src/posture/contradiction.rs`
**Location:** line 183 (`evaluate_configured_meets`)
**Severity:** MEDIUM
**Control reference:** NIST SP 800-53 CA-7, CM-6
**Remediation owner:** coder

**Description:**

`evaluate_configured_meets` parses `raw` as `u32`. For sysctl.d files that
configure `kernel.perf_event_paranoid = -1`, the parse returns `Err` and the
function returns `None`. This means a system where:

- `sysctl.d` has `kernel.perf_event_paranoid = -1` (configured unhardened)
- live kernel reads `-1` (live unhardened)

…produces `configured_meets = None`, suppressing the `(Some(false), None)`
path in `classify()` which returns `None` (no contradiction). The correct
result would be `configured_meets = Some(false)`, yielding
`classify(Some(false), Some(false)) = None` (no contradiction, both agree),
which is technically correct but only by accident.

The more important failure mode is when:

- `sysctl.d` has `kernel.perf_event_paranoid = -1` (configured unhardened)
- live kernel has `perf_event_paranoid = 2` (live hardened, e.g., a
  runtime `sysctl` hotfix was applied)

This should produce `EphemeralHotfix` (`live_meets = Some(true)`,
`configured_meets = Some(false)`). Instead, `configured_meets = None` and
no contradiction is reported. An operator sees the system as hardened with
no indication that the configuration does not persist across reboots.

The comment in `contradiction.rs:185–189` acknowledges the conservative
behaviour but frames it only as "no contradiction triggered" — it does not
flag the EphemeralHotfix suppression.

**Recommended action:**

Extend `evaluate_configured_meets` to attempt `i32` parsing when `u32`
parsing fails. If the `i32` value is negative, pass it to a new
`DesiredValue::meets_signed_integer()` path (this method already exists on
`DesiredValue`):

```rust
pub fn evaluate_configured_meets(raw: &str, desired: &DesiredValue) -> Option<bool> {
    if raw.trim() == "blacklisted" {
        return desired.meets_integer(1);
    }
    match raw.trim().parse::<u32>() {
        Ok(v) => desired.meets_integer(v),
        Err(_) => {
            // Attempt signed parse for sysctl nodes like perf_event_paranoid
            // that legitimately store negative configured values.
            raw.trim().parse::<i32>()
                .ok()
                .and_then(|v| desired.meets_signed_integer(v))
        }
    }
}
```

Add a regression test: `sysctl.d` value `-1` with `DesiredValue::AtLeast(2)`
must produce `Some(false)`, and the full `classify(Some(true), Some(false))`
path must produce `EphemeralHotfix`.

---

### `catalog.rs`

---

#### F-06 — LOW: Citation format violations — `NIST 800-53` instead of `NIST SP 800-53` in doc comments

**File:** `components/rusty-gadgets/umrs-platform/src/posture/catalog.rs`
**Location:** module doc comment (lines 24–28), `SignalDescriptor` doc (lines 43–45),
  `SIGNALS` doc (lines 77–78)
**Severity:** LOW
**Control reference:** project Citation Format Rule
**Remediation owner:** coder

**Description:**

The Citation Format Rule requires: "All NIST citations in Rust doc comments
must use the canonical form: `NIST SP 800-53` (not `NIST 800-53`)."

The following doc-comment citations in `catalog.rs` use the abbreviated form:

```
//! NIST 800-53 CM-6: ...
//! NIST 800-53 CA-7: ...
//! NIST 800-218 SSDF PW.4: ...
/// NIST 800-53 CM-6: ...
/// NIST 800-53 AU-3: ...
/// NIST 800-53 CA-7: ...
```

Six additional violations exist across `signal.rs`, `snapshot.rs`, and
`fips_cross.rs`. The `nist_controls` string fields in the `SIGNALS` array
are runtime display strings and may use abbreviated forms per the Citation
Format Rule exception ("runtime output strings may use abbreviated forms for
display compactness"). Those are not flagged here.

Note that doc comments in `reader.rs`, `contradiction.rs`, and
`modprobe.rs` use the correct `NIST SP 800-53` form consistently — `catalog.rs`
is the main offender.

**Recommended action:**

Replace `NIST 800-53` with `NIST SP 800-53` and `NIST 800-218 SSDF` with
`NIST SP 800-218 SSDF` in all doc-comment (`///` and `//!`) occurrences
within the posture module. Approximately 6 occurrences in `catalog.rs` and
6 spread across other files.

---

#### F-07 — LOW: `is_module_loaded` uses `Path::is_dir()` rather than a provenance-verified sysfs check for the trust gate

**File:** `components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs`
**Location:** line 479 (`is_module_loaded`)
**Severity:** LOW
**Control reference:** NIST SP 800-53 SI-7; NSA RTB RAIN
**Remediation owner:** security-engineer (design clarification) / coder (implementation)

**Description:**

`is_module_loaded` uses `Path::new(SYS_MODULE_BASE).join(module_name).is_dir()`
to determine whether a module is loaded. This is a metadata operation on
`/sys/module/<name>/`, not a content read, so it does not pass through
`SysfsText` + `SYSFS_MAGIC` verification.

The doc comment at line 463–465 acknowledges this explicitly:

> "The directory existence check is a regular filesystem call, not a
> provenance-verified read. This is intentional: we are not reading content
> from the kernel — we are testing for the presence of a sysfs directory,
> which is a metadata operation."

The rationale is sound for a trust gate. The concern is:

1. A symlink at `/sys/module/<name>` that points outside sysfs could make
   `is_dir()` return `true` while pointing to a non-sysfs path. On a
   correctly configured SELinux-enforcing system this is blocked by the
   policy, but the code does not verify it.

2. The doc comment does not cite the SELinux/MAC enforcement dependency
   that makes this safe. Without that documentation, a future reviewer
   may flag this as a gap without context.

**Risk assessment:** LOW. On RHEL 10 with SELinux enforcing and the targeted
or MLS policy, the sysfs filesystem is protected from symlink substitution.
This is a documentation gap more than a code defect.

**Recommended action:**

Add a comment to `is_module_loaded` noting that symlink substitution in
`/sys/module/` is prevented by SELinux type enforcement policy (sysfs_t
labeling, `sysfs` object class constraints). This explicitly documents the
MAC dependency that makes the unverified `is_dir()` safe to use here.
Example addition to the function doc:

```rust
/// Security note: `/sys/module/` is labeled `sysfs_t` under SELinux. On an
/// enforcing system with the RHEL 10 targeted or MLS policy, symlink
/// substitution attacks against this directory are blocked by type
/// enforcement. This metadata check is therefore safe as a trust gate
/// without full SYSFS_MAGIC provenance verification.
```

---

## Items Verified Correct (Prior Findings — Resolved)

The following items were flagged in earlier review sessions and are confirmed
implemented and correct in the current code:

1. **`evaluate_configured_meets` blacklist sentinel** (`contradiction.rs:179`):
   The `"blacklisted"` sentinel is handled explicitly before integer parsing.
   `evaluate_configured_meets("blacklisted", &DesiredValue::Exact(1))` returns
   `Some(true)`, enabling `BootDrift` detection for loaded blacklisted modules.
   Regression test in `posture_modprobe_tests.rs` confirms coverage. RESOLVED.

2. **`PerfEventParanoid` signed reader** (`reader.rs:259`):
   A dedicated `PerfEventParanoid` struct using `parse_sysctl_i32` is
   implemented. `LiveValue::SignedInteger(-1)` is represented correctly.
   `meets_signed_integer` is called in `read_live_sysctl_signal`. RESOLVED.

3. **FIPS trust gate** (`fips_cross.rs:128`):
   `FipsCrossCheck::evaluate` correctly gates all config reads on
   `live_fips_readable`. Trust gate test in `posture_fips_tests.rs`
   confirms both blocked and unblocked paths. RESOLVED.

4. **modprobe.d debug log value suppression** (`modprobe.rs:312–322`):
   `load_conf_file` logs `param=<value>` (not the actual value) with
   explicit Error Information Discipline comment citing SI-11/SC-28.
   RESOLVED at source. (Note: F-01 above identifies that this suppression
   is bypassed by `collect_one`'s summary log.)

5. **Slash→dot key normalization** (`configured.rs:246`):
   Sysctl.d slash-style keys are normalized to dot-style at insertion time.
   The normalization comment correctly explains the consequence of omission
   (silent contradiction detection failure). RESOLVED.

---

## Compliance Mapping

The posture module satisfies or partially satisfies the following controls:

| Control | Status | Evidence |
|---|---|---|
| NIST SP 800-53 CA-7 (Continuous Monitoring) | Satisfied | `PostureSnapshot::collect()` provides typed, structured posture assessment across all monitored signals |
| NIST SP 800-53 CM-6 (Configuration Settings) | Satisfied | Live vs. configured comparison via sysctl.d and modprobe.d merge trees |
| NIST SP 800-53 AU-3 (Audit Record Content) | Satisfied | `SignalReport` carries typed findings; `ContradictionKind` enum enables programmatic audit classification |
| NIST SP 800-53 SI-7 (Software and Information Integrity) | Satisfied | All `/proc/` and `/sys/` reads use `SecureReader` with `fstatfs` provenance verification |
| NIST SP 800-53 SI-10 (Input Validation) | Satisfied | Parsers fail closed; malformed lines are logged and skipped; non-numeric input returns `None` |
| NIST SP 800-53 SI-11 (Error Handling) | Partial | Debug discipline is applied in `configured.rs` and `modprobe.rs` but bypassed in `snapshot.rs::collect_one` (F-01) |
| NIST SP 800-53 SC-13 (Cryptographic Protection) | Satisfied | FIPS configured-value cross-check covers kernel live, cmdline, marker, and crypto-policy indicators |
| NIST SP 800-53 CM-7 (Least Functionality) | Satisfied | Blacklist signals detect DMA-capable module load; contradiction detection (when configured) flags `BootDrift` |
| NSA RTB RAIN (Non-Bypassability) | Satisfied | `StaticSource::read()` is the only path to sysctl/sysfs data; no raw `File::open` on pseudo-fs paths |
| NSA RTB Compile-Time Path Binding | Satisfied | All signal paths, expected magic values, and desired values are `const`-bound in the catalog |
| NIST SP 800-218 SSDF PW.4 | Satisfied | Debug-mode pattern execution timing present on all major read paths |
| FIPS 140-2/140-3 | Partial | FIPS configured-value cross-check present; FIPS gate before crypto primitive use not applicable here (module does not perform cryptographic operations) |

---

## Phase 2b Recommendations

Based on this review, the following items are recommended for Phase 2b:

1. **Bootloader cmdline configured values**: `configured_cmdline()` always
   returns `None` (deferred per plan). Phase 2b should implement grubenv /
   UEFI loader entry parsing to enable `EphemeralHotfix`/`BootDrift`
   detection for cmdline signals (`ModuleSigEnforce`, `Mitigations`, `Pti`,
   `Lockdown`, `RandomTrustCpu`, `RandomTrustBootloader`).

2. **Hard blacklist via `install` directives**: `modprobe.d` `install <mod> /bin/true`
   is a hard blacklist (prevents loading even with explicit `modprobe`).
   The current Phase 2a implementation only covers soft `blacklist` entries.
   Phase 2b should parse `install` directives and emit a stronger evidence
   grade when a module is hard-blacklisted.

3. **Negative sysctl.d configured value handling** (F-05 above): The fix for
   `evaluate_configured_meets` to handle `i32` configured values is a
   regression-risk item. It should include a test that drives the full
   `EphemeralHotfix` path for `PerfEventParanoid` with `-1` in a sysctl.d
   file.

4. **SELinux policy for posture probe binary**: If the posture probe is
   packaged as a standalone binary (not only as a library function), a
   dedicated SELinux type (`umrs_posture_exec_t`) and file context should
   be defined in `components/platforms/rhel10/`. The binary reads
   `/proc/sys/`, `/sys/module/`, `/sys/kernel/security/lockdown`,
   `/etc/sysctl.d/`, `/etc/modprobe.d/`, `/etc/system-fips`, and
   `/etc/crypto-policies/state/current`. All of these paths have
   implications for the SELinux policy allow rules required.

---

## Gap Analysis Summary

```
Files reviewed: 14
Total findings: 7 (0 Critical, 1 HIGH, 4 MEDIUM, 2 LOW)
Policy artifacts written: none (source-only review)
Policy artifacts needed: umrs_posture_exec_t SELinux type when binary is packaged (Phase 2b)
Documentation gaps: None identified in this review
Code-vs-policy inconsistencies: None (no deployed policy for this module yet)
```

**Findings requiring pre-deployment resolution:**
- F-01 (HIGH): `collect_one` debug log leaks configured values in release builds

**Findings recommended before deployment:**
- F-02 (MEDIUM): Phase 2a signals absent from primary catalog completeness test
- F-03 (MEDIUM): Bare `#[must_use]` annotations on 5 functions
- F-04 (MEDIUM): `Path::exists()` TOCTOU pattern in config directory guards
- F-05 (MEDIUM): Negative sysctl.d configured values silently suppress EphemeralHotfix

**Low-priority cleanup:**
- F-06 (LOW): Citation format violations in doc comments
- F-07 (LOW): `is_module_loaded` missing MAC enforcement documentation note
