# CCE Cross-Reference Audit — Posture Catalog Work Instruction

Audit date: 2026-03-18
Depth: surface
Scope: `components/rusty-gadgets/umrs-platform/src/posture/catalog.rs`,
       `components/rusty-gadgets/umrs-platform/src/posture/indicator.rs`

**Purpose:** This report is the concrete work instruction for the rust-developer (Rusty) to
complete CCE cross-referencing of the posture catalog. It synthesizes prior SCAP/STIG
familiarization findings against the current state of catalog.rs.

---

## Current State — What Has Already Been Done

The `IndicatorDescriptor` struct already has a `cce: Option<&'static str>` field (catalog.rs
lines 77–78), properly documented with a NIST SP 800-53 CA-2 citation. This work is complete.

The following 13 indicators already have CCE values populated and are confirmed correct against
the RHEL 10 STIG SCAP content:

| IndicatorId | CCE in catalog | Confirmed correct |
|---|---|---|
| `KptrRestrict` | `CCE-88686-1` | yes |
| `RandomizeVaSpace` | `CCE-87876-9` | yes |
| `UnprivBpfDisabled` | `CCE-89405-5` | yes |
| `PerfEventParanoid` | `CCE-90142-1` | yes |
| `YamaPtraceScope` | `CCE-88785-1` | yes |
| `DmesgRestrict` | `CCE-89000-4` | yes |
| `KexecLoadDisabled` | `CCE-89232-3` | yes |
| `ProtectedSymlinks` | `CCE-88796-8` | yes |
| `ProtectedHardlinks` | `CCE-86689-7` | yes |
| `BluetoothBlacklisted` | `CCE-87455-2` | yes |
| `UsbStorageBlacklisted` | `CCE-89301-6` | yes |
| `CorePattern` | `CCE-86714-3` | yes |
| `Pti` | `CCE-88971-7` | yes — STIG `grub2_pti_argument` |

The NIST control precision corrections originally identified during familiarization have also
been applied. The current `nist_controls` strings use:
- `KptrRestrict`: `SC-30, SC-30(2), SC-30(5)` — correct
- `DmesgRestrict`: `SI-11(a), SI-11(b)` — correct
- `YamaPtraceScope`: `SC-7(10), AC-6` — correct
- `UnprivBpfDisabled`: `AC-6, SC-7(10)` — correct

---

## Remaining Work — Three Partial-Coverage Indicators

These three indicators have related STIG CCEs that are not direct equivalents but are the
closest authoritative SCAP anchors. The decision of whether to populate `cce` requires a
judgment call. This section documents the options and a recommended decision for each.

---

### 1. `FipsEnabled` — `cce: None`

**Current catalog state:** `cce: None`

**Nearest STIG anchor:** `CCE-89085-5` (`configure_crypto_policy`)

**Analysis:** The STIG check `configure_crypto_policy` verifies that the system crypto policy
is set to `FIPS` via the `update-crypto-policies` tool. UMRS reads the kernel truth directly
from `/proc/sys/crypto/fips_enabled`. These are related but not equivalent:
- The STIG checks the policy name (configuration layer).
- UMRS checks the kernel enforcement state (runtime layer).
A system could have the policy set to `FIPS` but `fips_enabled = 0` if the kernel was not
rebooted after the policy change.

**Recommendation for Rusty:** Add `cce: Some("CCE-89085-5")` with an inline comment noting the
semantic difference. The CA-2 traceability value of having an anchor outweighs the partial
mismatch. The existing `cce` field doc comment already states "direct mapping exists" is the
condition — update the inline comment rather than the struct doc. Add a comment on the same
line noting the partial coverage:

```rust
// CCE-89085-5 covers configure_crypto_policy (policy layer); UMRS checks
// the kernel enforcement state directly. Nearest STIG anchor, not exact equivalent.
// (RHEL 10 STIG, scap-security-guide 2026-03-17)
cce: Some("CCE-89085-5"),
```

---

### 2. `ModuleSigEnforce` — `cce: None`

**Current catalog state:** `cce: None`

**Nearest STIG anchors:** `CCE-89982-3` (`audit_rules_kernel_module_loading_delete`),
`CCE-88638-2` (`audit_rules_kernel_module_loading_finit`),
`CCE-90172-8` (`audit_rules_kernel_module_loading_init`)

**Analysis:** These three CCEs cover audit rules for module loading events — they are reactive
(detect after the fact). UMRS `ModuleSigEnforce` checks the `module.sig_enforce=1` kernel
cmdline flag — this is proactive (blocks loading of unsigned modules entirely). There is no
STIG rule that directly checks the `module.sig_enforce` cmdline parameter.

**Recommendation for Rusty:** Leave `cce: None`. The semantic gap here is too large — the
audit-rules CCEs describe a different control mechanism. Populating this with a CCE would
mislead automated compliance tools into treating UMRS's proactive enforcement check as
equivalent to an audit-rule check. Document this explicitly with a comment:

```rust
// No direct STIG CCE for module.sig_enforce=1 cmdline enforcement.
// UMRS proactive check exceeds STIG baseline (STIG audits reactively via
// audit rules CCE-89982-3, CCE-88638-2, CCE-90172-8 instead).
cce: None,
```

---

### 3. `SuidDumpable` — `cce: None`

**Current catalog state:** `cce: None`

**Nearest STIG anchors:**
- `CCE-88330-6` (`disable_users_coredumps`) — checks `/etc/security/limits.conf` for `* hard core 0`
- `CCE-88732-3` (`coredump_disable_storage`) — checks systemd `Storage=none` in `coredump.conf`

**Analysis:** Both STIG anchors address coredump prevention but via different mechanisms than
`fs.suid_dumpable = 0`. `disable_users_coredumps` applies a PAM/limits restriction per-user;
`coredump_disable_storage` prevents systemd from writing coredumps. UMRS checks the kernel
sysctl directly. These are complementary controls, not the same control.

**Recommendation for Rusty:** Add both CCEs as a comment reference rather than as the primary
`cce` field value. Leave `cce: None` to avoid misleading automated tools, but add a doc
comment identifying the related CCEs for human cross-reference:

```rust
// No direct STIG CCE for fs.suid_dumpable. Related STIG rules that address
// coredump prevention via different mechanisms:
//   CCE-88330-6 (disable_users_coredumps, PAM limits layer)
//   CCE-88732-3 (coredump_disable_storage, systemd layer)
// UMRS checks the kernel sysctl directly.
cce: None,
```

---

## Indicators with No CCE — UMRS-Only Strengths

These indicators have `cce: None` and should remain `None`. No action required. They represent
hardening checks that exceed the RHEL 10 STIG baseline. Document this in compliance evidence
as "UMRS exceeds STIG baseline."

| IndicatorId | Reason no CCE exists |
|---|---|
| `Lockdown` | No STIG rule for lockdown LSM state verification |
| `ModulesDisabled` | No STIG rule for `kernel.modules_disabled` one-way latch |
| `RandomTrustCpu` | No STIG rule for CPU entropy trust flags |
| `RandomTrustBootloader` | No STIG rule for bootloader entropy trust flags |
| `Sysrq` | No STIG CCE for `kernel.sysrq`; bitmask semantics are site-policy-dependent |
| `FirewireCoreBlacklisted` | No STIG rule (hardware-rare in server deployments) |
| `ThunderboltBlacklisted` | No STIG rule |
| `UnprivUsernsClone` | No STIG rule for RHEL 10 (Fedora-specific kernel parameter) |
| `NfConntrackAcct` | No STIG rule (module parameter, not a security hardening check) |
| `ProtectedFifos` | No direct STIG rule |
| `ProtectedRegular` | No direct STIG rule |
| All 8 CPU mitigation sub-indicators | No STIG per-CVE cmdline override checks |
| `Mitigations` (umbrella) | No STIG check for `mitigations=off`; per-CVE checks are used instead |

---

## NIST Control Precision — Confirmed Resolved

The following control precision gaps were identified during familiarization. All have been
corrected in the current catalog and require no further action. Recorded here for audit trail
completeness.

| IndicatorId | Old citation (incorrect) | Corrected citation | Source of truth |
|---|---|---|---|
| `KptrRestrict` | `SI-7, SC-39` | `CM-6(a), SC-30, SC-30(2), SC-30(5)` | RHEL 10 STIG `sysctl_kernel_kptr_restrict` |
| `DmesgRestrict` | `SI-7, SC-28` | `SI-11(a), SI-11(b)` | RHEL 10 STIG `sysctl_kernel_dmesg_restrict` |
| `YamaPtraceScope` | `SC-39, AC-6` | `SC-7(10), AC-6` | RHEL 10 STIG `sysctl_kernel_yama_ptrace_scope` |
| `UnprivBpfDisabled` | `CM-7, SC-39` | `AC-6, SC-7(10)` | RHEL 10 STIG `sysctl_kernel_unprivileged_bpf_disabled` |

---

## `IndicatorDescriptor.cce` Field — Design Confirmation

The `cce` field is already implemented correctly. No changes needed to the struct definition.
The current design is:

```rust
/// CCE identifier from the RHEL 10 STIG, if this indicator has a SCAP
/// equivalent. Format: `CCE-NNNNN-N`. `None` for indicators that exceed
/// STIG coverage (UMRS-only hardening checks with no direct STIG analog).
///
/// NIST SP 800-53 CA-2: assessment evidence traceability — links this
/// indicator to its authoritative SCAP content identifier for use in
/// automated compliance reporting and STIG gap analysis.
pub cce: Option<&'static str>,
```

**Field type:** `Option<&'static str>` — correct. `None` is the semantically meaningful value
for UMRS-only indicators and must not be replaced with an empty string or a dummy value.

**Format enforcement:** The canonical form `CCE-NNNNN-N` is enforced by convention and the
Citation Format Rule. There is no runtime validation of the CCE string format. Rusty should
verify the check digit of any new CCE before adding it — the CCE check digit is computed
using the Luhn algorithm on the numeric portion.

**STIG version annotation:** Per the Citation Format Rule, when adding a CCE identifier, include
the STIG version and date in an inline comment. The existing entries do not all have this.
When adding the `FipsEnabled` CCE annotation, use the pattern:

```rust
// CCE-89085-5 (RHEL 10 STIG, scap-security-guide 2026-03-17)
cce: Some("CCE-89085-5"),
```

Optionally add these version comments to the existing 13 entries in the same pass for
consistency — this is housekeeping, not a blocking defect.

---

## Concrete Work Items for Rusty

In priority order:

### Work Item 1 — Add `FipsEnabled` CCE with partial-coverage comment (MEDIUM priority)

File: `components/rusty-gadgets/umrs-platform/src/posture/catalog.rs`

Change `FipsEnabled` entry from:

```rust
cce: None,
```

To:

```rust
// CCE-89085-5 covers configure_crypto_policy (policy layer); UMRS checks
// the kernel enforcement state directly — nearest STIG anchor, not exact equivalent.
// (RHEL 10 STIG, scap-security-guide 2026-03-17)
cce: Some("CCE-89085-5"),
```

### Work Item 2 — Add explanatory comments to `ModuleSigEnforce` and `SuidDumpable` (LOW priority)

These remain `cce: None` but need inline rationale so future maintainers do not assume
the CCE research was simply forgotten. Add the comments specified in the analysis sections
above.

### Work Item 3 — Add STIG version comments to the 13 existing CCE entries (housekeeping)

Each existing `cce: Some(...)` entry should have the inline comment:

```rust
// (RHEL 10 STIG, scap-security-guide 2026-03-17)
cce: Some("CCE-XXXXX-X"),
```

This is already done for `KexecLoadDisabled` (confirmed in prior commit). Verify the rest
and add the comment where missing. This is pure documentation hygiene, not a functional change.

---

## Findings Summary

```
Files reviewed: 2
Total findings: 3 (0 HIGH, 1 MEDIUM, 2 LOW)
```

### Finding 1 — MEDIUM

```
File: components/rusty-gadgets/umrs-platform/src/posture/catalog.rs
Location: FipsEnabled entry (~line 372)
Finding: FipsEnabled has cce: None but CCE-89085-5 (configure_crypto_policy) is the closest
         authoritative STIG anchor. Omitting it leaves a CA-2 traceability gap in automated
         compliance tools that consume the catalog.
Severity: MEDIUM
Recommended citation: CCE-89085-5 (RHEL 10 STIG) with partial-coverage comment
Remediation owner: coder
```

### Finding 2 — LOW

```
File: components/rusty-gadgets/umrs-platform/src/posture/catalog.rs
Location: ModuleSigEnforce entry (~line 302)
Finding: cce: None has no explanatory comment. Future maintainers cannot distinguish
         "no CCE exists" from "CCE research not done." The three related audit-rule CCEs
         should be documented inline.
Severity: LOW
Remediation owner: coder
```

### Finding 3 — LOW

```
File: components/rusty-gadgets/umrs-platform/src/posture/catalog.rs
Location: SuidDumpable entry (~line 272)
Finding: cce: None has no explanatory comment. Two related CCEs (CCE-88330-6,
         CCE-88732-3) cover coredump prevention at different layers and should be
         documented inline.
Severity: LOW
Remediation owner: coder
```

---

## Reference Documents

- SCAP familiarization notes: `.claude/agent-memory/security-auditor/scap_familiarization.md`
- STIG signal index: `.claude/references/scap-security-guide/stig-signal-index.md`
- CCE cross-ref: `.claude/references/scap-security-guide/cce-nist-crossref.md`
- Coverage report: `.claude/references/reports/stig-signal-coverage.md`
- SCAP/STIG corpus plan: `.claude/plans/scap-stig-corpus-plan.md`
- Citation Format Rule: `.claude/rules/rust_design_rules.md`
