# Plan: umrs-platform Cross-Platform Readiness (RHEL10 + Ubuntu)

Date: 2026-03-11

## Summary

`umrs-platform` compiles cleanly on both RHEL10 and Ubuntu — all dependencies
are pure-Rust and Linux-compatible. `SELINUX_MAGIC` is a compile-time constant;
no selinuxfs installation is required at compile time.

At runtime the detection pipeline runs on Ubuntu but tops out at T2 (EnvAnchored)
because the Biba pre-check is hard-wired to SELinux enforce mode.

---

## Runtime Behavior by Platform

| Phase                        | RHEL10 (SELinux enforcing)   | Ubuntu (AppArmor)                  |
|------------------------------|------------------------------|-------------------------------------|
| Kernel anchor (procfs)       | Full success → T1            | Full success → T1                   |
| Mount topology               | Works → T2                   | Works → T2                          |
| Release candidate            | Works                        | Works                               |
| Package substrate — RPM      | Succeeds (2 facts)           | Fails gracefully (no /var/lib/rpm)  |
| Package substrate — dpkg     | Skipped (RPM wins)           | Succeeds (2 facts)                  |
| Biba pre-check (SELinux)     | Passes → T3                  | Fails — no selinuxfs → stays T2     |
| Kernel lockdown              | Works (securityfs present)   | Soft failure recorded in evidence   |
| Release parse                | Works                        | Works                               |

---

## Issues and Work Items

### Issue 1 — Biba pre-check is SELinux-only (blocks T3 on Ubuntu)

**File:** `src/detect/pkg_substrate.rs` — `check_selinux_enforce()`

The function reads `SelinuxEnforce` unconditionally. On Ubuntu, selinuxfs is not
mounted, so this always returns `false`, and T3 is never asserted even when the
dpkg substrate is perfectly valid.

**Plan:**
- Make the pre-check substrate-aware:
  - RPM substrate wins → check `SelinuxEnforce` (current behavior, unchanged)
  - dpkg substrate wins → check AppArmor enforcement status, or accept T3 with
    a downgrade note and a recorded reason
- Introduce a `MandatoryAccessControl` abstraction or a phase-level decision
  table that maps `OsFamily` → MAC check strategy
- Requires an architectural decision before implementation

**Priority:** Medium — Ubuntu reaches a useful T2 without this; T3 on Ubuntu is
not a current deployment requirement.

---

### Issue 2 — KernelLockdown soft-failure on Ubuntu

**File:** `src/kattrs/security.rs` — `KernelLockdown`

`/sys/kernel/security/lockdown` exists on Ubuntu kernels built with
`CONFIG_SECURITY_LOCKDOWN_LSM=y` (most recent LTS kernels have it), but it is
not guaranteed. The `read_lockdown` call in `kernel_anchor.rs` is already a soft
phase — failure records in evidence and continues.

**Plan:**
- No code change required; behavior is already correct.
- Add a doc note in `kattrs/security.rs` stating that the file may be absent on
  Ubuntu kernels without `CONFIG_SECURITY_LOCKDOWN_LSM` and that this is
  handled gracefully.

**Priority:** Low (documentation only).

---

### Issue 3 — No integration tests cover Ubuntu / dpkg-path behavior

**File:** `tests/kattrs_tests.rs` and `detect/` pipeline

There are no CI paths or test fixtures for the dpkg probe path. Regressions on
Ubuntu-specific behavior (dpkg succeeds, RPM fails, Biba check soft-fails) are
invisible.

**Plan:**
- Introduce a seam in `pkg_substrate::run_inner` allowing the probe list and
  the MAC check function to be injected in tests.
- Write integration tests that assert:
  - dpkg probe returns `parse_ok=true` when `/var/lib/dpkg/status` is present
  - T3 is not asserted when `check_selinux_enforce` returns false
  - Evidence bundle contains the correct downgrade reason
- Alternatively, run the test suite on a Ubuntu CI runner (GitHub Actions matrix).

**Priority:** Medium — needed before Ubuntu is treated as a supported platform.

---

### Issue 4 — FIPS mode behavior on Ubuntu

**File:** `src/kattrs/procfs.rs` — `ProcFips`

`/proc/sys/crypto/fips_enabled` exists on Ubuntu but returns `0` unless
`ubuntu-advantage-tools` FIPS mode is explicitly activated. No code change
needed — the value is read correctly either way.

**Plan:**
- Add a doc note in `procfs.rs` that FIPS mode on Ubuntu requires explicit
  activation via `ubuntu-advantage-tools` and that a `0` reading is expected
  and correct on a standard Ubuntu install.

**Priority:** Low (documentation only).

---

## Definition of Done

- [ ] Issue 1: Architectural decision recorded; Biba pre-check is substrate-aware
- [ ] Issue 1: Ubuntu with dpkg substrate can assert T3 when MAC check passes
- [ ] Issue 2: Doc note added to `kattrs/security.rs`
- [ ] Issue 3: Test seam introduced; Ubuntu dpkg-path covered by integration tests
- [ ] Issue 4: Doc note added to `kattrs/procfs.rs`
- [ ] `cargo xtask clippy && cargo xtask test` clean on both platforms
