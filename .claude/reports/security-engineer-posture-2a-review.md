# Security Engineer Review — Kernel Security Posture Probe Phase 2a

```
Audit date:  2026-03-14
Depth:       in-depth
Scope:       components/rusty-gadgets/umrs-platform/src/posture/
             (all files: modprobe.rs, fips_cross.rs, signal.rs, catalog.rs,
              snapshot.rs, reader.rs, contradiction.rs, configured.rs, mod.rs)
```

---

## Executive Summary

Phase 2a is well-structured and largely correct. Trust boundaries are drawn
accurately, the provenance-verification engine is used correctly for every
sysfs read, and the fail-closed posture is consistently applied. The module
doc comments and compliance annotations meet or exceed the project standard.

However, one **High** defect causes contradiction detection to be silently
disabled for all four blacklist signals, and one **Medium** defect causes
module parameter values to be emitted into debug log output in their entirety.
Both are tractable fixes. Three **Low** findings address minor gaps. The
overall verdict is **approve-with-conditions**: the two higher-severity
findings must be resolved before deployment.

---

## Findings by File

### `components/rusty-gadgets/umrs-platform/src/posture/contradiction.rs`

---

#### Finding 1 — HIGH: `evaluate_configured_meets` silently disables contradiction detection for all blacklist signals

**File:** `contradiction.rs`
**Location:** line 139 (`evaluate_configured_meets`)
**Control reference:** NIST 800-53 CM-6, CA-7, AU-3

**What:**
`evaluate_configured_meets` unconditionally tries to parse `raw` as a `u32`.
For blacklist signals, `ConfiguredValue::raw` is the sentinel string
`"blacklisted"` (set by `blacklist_configured_value()` in `modprobe.rs:545`).
`"blacklisted".parse::<u32>()` returns `Err`, so `evaluate_configured_meets`
returns `None`.

In `collect_one` (snapshot.rs:255-261), `configured_meets` becomes `None`,
and `contradiction::classify(live_meets, None)` always returns `None` per
the match arm `(_, None) => None`. The contradiction field in every
`SignalReport` for `BluetoothBlacklisted`, `UsbStorageBlacklisted`,
`FirewireCoreBlacklisted`, and `ThunderboltBlacklisted` is therefore always
`None`, even in the security-critical case where:

- The modprobe.d blacklist is present (`configured_value = Some("blacklisted")`)
- The module is actively loaded (`live_value = Bool(false)`, `meets_desired = Some(false)`)

This is the worst possible result for a blacklist signal: a DMA-capable module
(thunderbolt, firewire_core) is running on a system whose configuration says it
should be blacklisted, and the posture probe reports no contradiction.
`BootDrift` should have been emitted.

**Why it matters:**
The entire purpose of the cross-check is to detect when policy says "this
module must not be loaded" and the kernel disagrees. Silent `None` here is
indistinguishable from "no configured value available" — the contradiction is
erased, not reported. An operator reading the snapshot has no way to know that
the system is out of policy.

This is a direct violation of the Security Findings as Data rule (NIST 800-53
AU-3): the finding must be representable as a typed enum variant, not silently
suppressed.

**Recommended fix (owner: coder):**
`evaluate_configured_meets` must be extended to handle the `"blacklisted"`
sentinel, or a separate evaluation path must be used for `ModprobeConfig`
blacklist signals in `collect_one`.

Option A — extend `evaluate_configured_meets` with a sentinel check:

```rust
pub fn evaluate_configured_meets(
    raw: &str,
    desired: &DesiredValue,
) -> Option<bool> {
    // Blacklist sentinel: "blacklisted" means the module is explicitly
    // blacklisted in modprobe.d. The desired value for blacklist signals
    // is Exact(1) = "blacklist effective". A configured blacklist entry
    // meets the desired value; absence does not.
    if raw.trim() == "blacklisted" {
        return desired.meets_integer(1);
    }
    match raw.trim().parse::<u32>() {
        Ok(v) => desired.meets_integer(v),
        Err(_) => None,
    }
}
```

Option B (preferred for clarity) — in `collect_one`, route blacklist signals
through a dedicated `evaluate_configured_meets_modprobe` that is aware of the
sentinel semantics, keeping `evaluate_configured_meets` unmodified for integer
signals. This preserves the single-responsibility principle and makes the
non-integer path explicit rather than adding a special case to the generic
integer parser.

The fix must be accompanied by an integration test that exercises the
`configured=Some("blacklisted"), live=loaded` path and asserts
`contradiction = Some(ContradictionKind::BootDrift)`.

---

### `components/rusty-gadgets/umrs-platform/src/posture/modprobe.rs`

---

#### Finding 2 — MEDIUM: Module parameter values emitted to debug log unconditionally during merge

**File:** `modprobe.rs`
**Location:** line 308-311 (`load_conf_file`, the `Options` match arm)
**Control reference:** NIST 800-53 SI-11, SC-28

**What:**
The debug log emitted during modprobe.d merge includes the full parameter
value verbatim:

```rust
log::debug!(
    "posture: modprobe.d merge: {source}:{human_no} options \
     {module} {param}={value}"
);
```

This is structurally identical to the error information discipline violation
that was correctly identified and fixed in `configured.rs` (lines 248-253,
where a Finding 4 annotation acknowledges that value content is omitted from
sysctl.d logging). The same discipline was not applied here.

While the current Phase 2a parameter set is limited to `nf_conntrack acct=1`,
`/etc/modprobe.d/` in production environments routinely contains entries like:

```
options cryptd max_cpu_qlen=100
options iwlwifi 11n_disable=8 wd_disable=1 bt_coex_active=0
options drm_kms_helper poll=N
```

More relevantly, in environments where modprobe.d is used for crypto or driver
hardening, parameter values can reflect security policy choices that should not
be broadcast in logs. The `options` parser currently has no scope restriction —
it merges all `.conf` files from all three directories. Future Phase 2b
expansion will add more signals, each reading more parameter values through
this same log path.

At DoD/CUI deployment sites where debug logging may be enabled during
troubleshooting, this log line exposes the full parameter-value mapping of the
system's modprobe configuration.

**Recommended fix (owner: coder):**
Emit the key (param name) but not the value in the merge-time debug line,
matching the discipline applied in `configured.rs`:

```rust
log::debug!(
    "posture: modprobe.d merge: {source}:{human_no} options \
     {module} {param}=<value>"
);
```

The live sysfs read log at lines 510-514 is already correctly guarded by
`#[cfg(debug_assertions)]` and emits the value via `result.as_ref().map(|r| r.as_deref())`.
That guard is acceptable for build-time separation but does not protect
against debug builds deployed during troubleshooting. Apply the same value
omission pattern consistently.

---

#### Finding 3 — LOW: `is_module_loaded` directory existence check is a TOCTOU window (accepted by design, requires documentation)

**File:** `modprobe.rs`
**Location:** line 460 (`is_module_loaded`)
**Control reference:** NIST 800-53 SI-7

**What:**
`is_module_loaded` tests whether `/sys/module/<name>/` exists using
`Path::is_dir()`, and the Trust Gate in `param_configured_and_live` and
`read_live_modprobe` gates the subsequent `read_module_param` call on this
result. There is a TOCTOU window: the module could be unloaded between the
`is_dir()` check and the `SysfsText::new()` + `SecureReader::read_generic_text()`
call that reads the parameter value.

The practical impact of this specific window is low for two reasons:
1. If the module unloads between the check and the read, `read_module_param`
   will return `Ok(None)` (file not found) or an I/O error — both are handled
   gracefully and degrade to `(None, None)`, which is fail-closed for the
   live value.
2. Kernel module unloading is a privileged operation; an attacker who can
   unload modules already has sufficient privilege to manipulate posture
   signals by other means.

The concern is asymmetric: the TOCTOU window cannot produce a false-positive
"hardened" result, only a false-negative "unavailable" result. The current
code is therefore correct by construction for the security claim (cannot be
tricked into reporting a dangerous module as absent when it is present).

**This finding is informational, not actionable as a code change.** The existing
doc comment on `is_module_loaded` (lines 444-453) correctly documents the
design rationale. The recommendation is to add a brief note explicitly stating
that the TOCTOU window fails open (to `unavailable`) rather than to a false
`hardened` result.

**Recommended action (owner: coder):**
Add one sentence to the `is_module_loaded` doc comment:

> The TOCTOU window between this check and a subsequent `read_module_param`
> call can only produce a false-negative (live value unavailable), never a
> false-positive (module incorrectly reported as absent). The security claim
> — that a present module cannot be reported as absent — holds.

---

#### Finding 4 — LOW: `options <module>` with no parameters silently accepted as `Options { params: [] }`

**File:** `modprobe.rs`
**Location:** lines 412-417 (`parse_options_directive`)
**Control reference:** NIST 800-53 SI-10

**What:**
`parse_options_directive` returns `ParsedDirective::Options { module, params: [] }`
when the `params_str` is empty (i.e., `options mymodule` with no parameters).
The check at line 412 only rejects the case where `params_str` is non-empty
but contains no parseable `key=val` pairs. A bare `options mymodule` line
produces a valid `Options` result with an empty `params` vec, which then
iterates zero times in `load_conf_file` — effectively a no-op.

This is not a safety defect: the outcome is correct (no entry is added to the
map). The modprobe.d(5) manual page does not define `options module` without
parameters as valid syntax. However, accepting it silently as `Options`
rather than as `Malformed` or `Unrecognised` means the debug log does not flag
it for an operator who may have made a typo in a configuration file.

**Recommended fix (owner: coder):**
Return `Malformed` when `params_str` is empty (no params after the module
name), matching the intent of the `params.is_empty() && !params_str.is_empty()`
check. A bare `options mymodule` line is not a valid modprobe.d directive.
A test covering this case should be added.

---

### `components/rusty-gadgets/umrs-platform/src/posture/fips_cross.rs`

---

#### Finding 5 — LOW: Trust Gate semantics do not match the stated contract for `cmdline_fips` when live FIPS is unreadable

**File:** `fips_cross.rs`
**Location:** lines 128-139 (`FipsCrossCheck::evaluate`, Trust Gate block)
**Control reference:** NIST 800-53 CM-6

**What:**
The Trust Gate doc comment (module header, line 24-27) states:

> Only reads `/etc/system-fips` and crypto-policy state if
> `/proc/sys/crypto/fips_enabled` was accessible (live state available).
> If the kernel cannot confirm the crypto subsystem state, config reads
> are advisory-only and return `None`.

However, the Trust Gate early-return at lines 133-138 passes through the
`cmdline_has_fips1` value received from the caller — it is NOT gated:

```rust
return Self {
    marker_present: None,
    cmdline_fips: cmdline_has_fips1,   // ← not blocked by the Trust Gate
    crypto_policy: None,
    configured_meets_desired: None,
};
```

The cmdline value is then visible to callers of `as_configured_value()`, and
because `cmdline_fips.is_some()` is checked in `as_configured_value()` (line
210), a `ConfiguredValue` can be returned even when the Trust Gate was
supposed to block all configuration reads.

The inconsistency does not create a false assurance result because
`configured_meets_desired` is correctly set to `None` in the Trust Gate path,
and the contradiction classification in `collect_one` uses
`evaluate_configured_meets` on the raw field — which for the FIPS cross-check
result will produce a non-integer string (e.g., `"cmdline=fips=1"`), returning
`None` from `evaluate_configured_meets` and thus `configured_meets = None`,
and `classify(live, None) = None`. So no incorrect contradiction is generated.

Nevertheless, the behaviour violates the stated contract: a caller inspecting
`FipsCrossCheck.cmdline_fips` and `FipsCrossCheck.configured_meets_desired`
independently could draw incorrect inferences. The contract should either be
corrected in the documentation, or the implementation should null out
`cmdline_fips` in the Trust Gate path.

**Recommended fix (owner: coder):**
Either:

A. Set `cmdline_fips: None` in the Trust Gate early-return and update the
   doc comment to say that ALL fields are `None` when the gate blocks.

B. Update the doc comment to explicitly state that `cmdline_fips` is passed
   through from the caller even in the Trust Gate path, and explain why this
   is safe (the `configured_meets_desired: None` prevents incorrect
   contradiction detection downstream).

Option A is preferable for contract clarity.

---

## Pattern Compliance Checklist

| Pattern | Status | Notes |
|---|---|---|
| Must-Use Contract | PASS | All public `Result`/`Option` returns and security types carry `#[must_use]` with message strings. |
| Validate at Construction | PASS | `ModprobeConfig::load()` and `SysctlConfig::load()` build fully validated values; callers never receive partial state. |
| Security Findings as Data | FAIL (Finding 1) | Blacklist contradiction is silently suppressed — `BootDrift` is never produced for the critical case. |
| Compile-Time Path Binding | PASS | `MODPROBE_SEARCH_DIRS`, `SYS_MODULE_BASE`, `SYSTEM_FIPS_MARKER`, `CRYPTO_POLICY_STATE` are `const`; sysfs reads use `SYSFS_MAGIC`. |
| Pattern Execution Measurement | PASS | All new public entry points log timing under `#[cfg(debug_assertions)]`. |
| Trust Gate | PASS | sysfs parameter reads gated behind `is_module_loaded()`. FIPS config reads gated behind `live_fips_readable`. Minor contract inconsistency in FIPS (Finding 5). |
| Fail-Closed Parsing | PASS | Malformed modprobe.d lines are logged at debug and rejected. Unreadable directories and files degrade gracefully. |
| Non-Bypassability | PASS | All sysfs reads route through `SysfsText` + `SecureReader`. No raw `File::open` on pseudo-filesystem paths. |

---

## Trust Boundary Assessment

**Regular filesystem reads (`/etc/modprobe.d/`, `/etc/system-fips`, `/etc/crypto-policies/`):**
Correctly treated as advisory configured-value sources. No `SecureReader` or
`fstatfs` provenance check applied — this is the correct posture. The live
kernel value is always authoritative.

**Sysfs reads (`/sys/module/<mod>/parameters/<param>`):**
Correctly routed through `SysfsText` + `SYSFS_MAGIC` provenance verification.
The Trust Gate (`is_module_loaded`) is correctly applied before any sysfs
parameter read.

**Module-directory existence check (`/sys/module/<name>/`):**
`Path::is_dir()` is used, not a provenance-verified read. This is correct for
a metadata check (the directory's existence is what we are checking, not its
content). See Finding 3 for the TOCTOU note.

The overall trust boundary architecture is correct. The advisory vs.
authoritative separation is clearly documented and consistently implemented.

---

## Contradiction Detection Correctness

For integer-valued signals (sysctl, NfConntrackAcct): correct. Both
`evaluate_configured_meets` and `classify` work as intended for all
integer-valued `DesiredValue` variants.

For blacklist signals (BluetoothBlacklisted, UsbStorageBlacklisted,
FirewireCoreBlacklisted, ThunderboltBlacklisted): broken (Finding 1).
The `"blacklisted"` sentinel string is not handled by `evaluate_configured_meets`,
so `configured_meets` is always `None` and `classify` always returns `None`.
The contradiction is never reported.

For FIPS cross-check: `evaluate_configured_meets` is called on the
`raw` field of the `ConfiguredValue` returned by `FipsCrossCheck::as_configured_value()`,
which is a human-readable audit summary string (e.g., `"marker=present cmdline=fips=1"`).
This will always parse as non-integer and return `None`. The FIPS configured-value
field therefore cannot participate in `classify()`. This appears intentional
given that `FipsEnabled` uses `DesiredValue::Exact(1)` with a boolean live
value — but the `configured_meets` path is dead for FIPS by construction.
No security defect results, but the contradiction table comment in
`contradiction.rs` should acknowledge this.

---

## Attack Surface Assessment

**New file paths read:** `/etc/modprobe.d/`, `/usr/lib/modprobe.d/`,
`/run/modprobe.d/` (conf files), `/sys/module/*/parameters/*`,
`/etc/system-fips`, `/etc/crypto-policies/state/current`.

**Symlink following:** `std::fs::read_to_string` on modprobe.d and sysctl.d
files will follow symlinks. This is consistent with how `modprobe(8)` itself
operates. On systems with SELinux enforcing (targeted policy), symlink
traversal from the daemon context is constrained by the file type label.
No new attack surface is introduced beyond what modprobe itself already exposes.

**Path injection:** `is_module_loaded` and `read_module_param` construct paths
from `module_name` and `param_name` strings. For the five Phase 2a signals,
these strings are compile-time constants (`"nf_conntrack"`, `"bluetooth"`,
etc.) embedded in `module_name_for_blacklist_signal` and the `collect_one`
match arms. No user-controlled input reaches these path construction calls.
For future Phase 2b expansion, any new signal that derives a module name from
parsed content must validate the name against a strict allowlist before
constructing a sysfs path.

**Directory traversal:** `load_conf_dir` and its sysctl.d counterpart iterate
`read_dir` output filtered to `.conf` files and `is_file()`. The `is_file()`
check resolves symlinks — a symlink to a directory that has a `.conf`-like
name will return `false` and be skipped. This is correct.

---

## Gap Analysis Summary

```
Files reviewed:       9
Total findings:       5  (0 Critical, 1 High, 1 Medium, 3 Low)

Policy artifacts:     None required — this is a Rust library review.
Documentation gaps:   None critical. Finding 3 recommends a one-sentence
                      addition to the is_module_loaded doc comment.
                      Finding 5 recommends a doc comment correction in
                      FipsCrossCheck::evaluate.

Code-vs-policy inconsistencies:
  - Finding 1: evaluate_configured_meets does not handle the "blacklisted"
    sentinel, causing BootDrift contradiction to be silently suppressed for
    all four DMA-attack-surface blacklist signals.
  - Finding 5: FipsCrossCheck Trust Gate passes through cmdline_fips even
    when the gate is supposed to block all config reads.
```

---

## Overall Assessment

**Approve with conditions.**

The architecture is sound. Trust boundaries are drawn correctly. Provenance
verification is applied consistently. The fail-closed posture is maintained
throughout. Pattern compliance is high. Phase 2a is a well-engineered addition.

**Conditions before deployment:**

1. Finding 1 (High) must be resolved. The `evaluate_configured_meets`
   function must be extended or a dedicated path must be added so that
   blacklist signals produce `BootDrift` when the module is loaded despite a
   modprobe.d blacklist entry. An integration test must cover this path.

2. Finding 2 (Medium) must be resolved. Module parameter values must be
   omitted from the debug merge log, matching the discipline already applied
   in `configured.rs`.

Findings 3, 4, and 5 are recommended for resolution in the same PR but are
not blocking deployment.

---

## Remediation Owner Summary

| Finding | Severity | Owner |
|---|---|---|
| 1 — blacklist contradiction detection silently suppressed | High | coder |
| 2 — module parameter values in debug log | Medium | coder |
| 3 — TOCTOU window documentation | Low | coder |
| 4 — bare `options module` silently accepted | Low | coder |
| 5 — FipsCrossCheck Trust Gate contract inconsistency | Low | coder |
