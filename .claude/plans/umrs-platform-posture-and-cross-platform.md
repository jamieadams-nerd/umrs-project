---
name: umrs-platform Expansion — Cross-Platform & Serialization
path: components/rusty-gadgets/umrs-platform
agent: rust-developer
status: active umbrella — subsidiary plans in progress
split: CPU extension probe extracted to .claude/plans/cpu-extension-probe.md (2026-03-18)
---

## Vision

`umrs-platform` is the low-level OS and kernel layer for UMRS. It currently contains OS
detection and kernel attribute (kattrs) probing. The crate is expanding across three pillars:

1. **OS Detection** — done; `OsDetector` is the public entry point
2. **Kernel Security Posture Probe** — in progress; `IndicatorId` catalog, contradiction engine,
   snapshot pipeline
3. **CPU Extension Detection** — future; **see `.claude/plans/completed/cpu-extension-probe.md`**

This file covers pillars 1–2, cross-platform readiness, and serialization.
CPU extension detection has its own plan.

---

## Display Grouping Reference

When posture signals are presented to operators (TUI, reports, `--json`), organize them
under Jamie's 7-domain Capability Matrix. See `.claude/references/capability-matrix-domains.md`
for the full mapping. Source: `.claude/jamies_brain/kernel-probe-grouping.txt`.

### Dual-Audience API

The public interface must serve two audiences:

- **Novice and intermediate programmers** need a single object they can create and then
  query for answers. They just want to know: is SELinux capable? What OS version is this?
  Is this package installed? The `OsDetector::detect()` pattern is exactly right — simple to
  create, simple to query, hides the evidence chain.

- **Experienced programmers and auditors** need the full evidence chain and trust tier
  classification when they need it. The evidence and confidence model must remain accessible
  without complicating the simple path.

The goal is: easy things are easy, hard things are possible. Do not collapse these two
audiences into a single complicated API. The detailed trust checks and evidence chain should
be available but should not impose on callers who only need a basic answer.

Jamie's note: "Things like the OsDetector:: is **GREAT**! Love public facing detectors like
this. Simple for them and keep the detailed, advanced stuff we have for experienced
programmers."

### Module Refactoring Consideration

As the crate grows, refactoring for readability may be warranted:
- `kattrs/` might benefit from a `kernel/` top-level module that groups both reading/parsing
  and the queryable structures
- The base structures will expand to include storage for evidence and level of trust
- Public functions should present settings with an iterator — easy-to-use interfaces
- Plenty of `log::debug!()` to show what is going on for developers and auditors
- Any refactoring should prioritize ease of reading and management without breaking the
  public API

---

## Subsidiary Plans

These plans are standalone and are NOT absorbed into this document. This section is a
reference index only.

| Plan | Status | File |
|------|--------|------|
| Kernel Security Posture Probe | Phase 2a reviewed, Phase 2b next | `.claude/plans/kernel-security-posture-probe.md` |
| CPU Security Corpus | Phase 0 complete, Phase 0.5 (spec update) next | `.claude/plans/cpu-security-corpus-plan.md` |
| CPU Extension Probe | Future — blocked on posture probe + corpus | `.claude/plans/cpu-extension-probe.md` |

The CPU corpus plan is a research and corpus-building gate only. No Rust implementation of
CPU extension detection begins until the kernel posture probe project is complete AND the
corpus research is validated.

---

## Cross-Platform Readiness (RHEL10 + Ubuntu)

> This section was absorbed from the standalone plan `.claude/plans/umrs-platform-cross-platform.md`
> (created 2026-03-11). The standalone file remains as the authoritative source; this section
> is a consolidated view for the umbrella.

`umrs-platform` compiles cleanly on both RHEL10 and Ubuntu — all dependencies are pure-Rust
and Linux-compatible. `SELINUX_MAGIC` is a compile-time constant; no selinuxfs installation
is required at compile time.

At runtime the detection pipeline runs on Ubuntu but tops out at T2 (EnvAnchored) because
the Biba pre-check is hard-wired to SELinux enforce mode.

### Runtime Behavior by Platform

| Phase                        | RHEL10 (SELinux enforcing)   | Ubuntu (AppArmor)                   |
|------------------------------|------------------------------|-------------------------------------|
| Kernel anchor (procfs)       | Full success → T1            | Full success → T1                   |
| Mount topology               | Works → T2                   | Works → T2                          |
| Release candidate            | Works                        | Works                               |
| Package substrate — RPM      | Succeeds (2 facts)           | Fails gracefully (no /var/lib/rpm)  |
| Package substrate — dpkg     | Skipped (RPM wins)           | Succeeds (2 facts)                  |
| Biba pre-check (SELinux)     | Passes → T3                  | Fails — no selinuxfs → stays T2     |
| Kernel lockdown              | Works (securityfs present)   | Soft failure recorded in evidence   |
| Release parse                | Works                        | Works                               |

### Issue 1 — Biba pre-check is SELinux-only (blocks T3 on Ubuntu)

**File:** `src/detect/pkg_substrate.rs` — `check_selinux_enforce()`

The function reads `SelinuxEnforce` unconditionally. On Ubuntu, selinuxfs is not mounted, so
this always returns `false`, and T3 is never asserted even when the dpkg substrate is
perfectly valid.

**Plan:**
- Make the pre-check substrate-aware:
  - RPM substrate wins → check `SelinuxEnforce` (current behavior, unchanged)
  - dpkg substrate wins → check AppArmor enforcement status, or accept T3 with a downgrade
    note and a recorded reason
- Introduce a `MandatoryAccessControl` abstraction or a phase-level decision table that maps
  `OsFamily` → MAC check strategy
- Requires an architectural decision before implementation (see Architectural Decisions — Resolved)

**Priority:** Medium — Ubuntu reaches a useful T2 without this; T3 on Ubuntu is not a
current deployment requirement.

### Issue 2 — KernelLockdown soft-failure on Ubuntu

**File:** `src/kattrs/security.rs` — `KernelLockdown`

`/sys/kernel/security/lockdown` exists on Ubuntu kernels built with
`CONFIG_SECURITY_LOCKDOWN_LSM=y` (most recent LTS kernels have it), but it is not
guaranteed. The `read_lockdown` call in `kernel_anchor.rs` is already a soft phase —
failure records in evidence and continues.

**Plan:**
- No code change required; behavior is already correct.
- Add a doc note in `kattrs/security.rs` stating that the file may be absent on Ubuntu
  kernels without `CONFIG_SECURITY_LOCKDOWN_LSM` and that this is handled gracefully.

**Priority:** Low (documentation only).

### Issue 3 — No integration tests cover Ubuntu / dpkg-path behavior

**File:** `tests/kattrs_tests.rs` and `detect/` pipeline

There are no CI paths or test fixtures for the dpkg probe path. Regressions on
Ubuntu-specific behavior (dpkg succeeds, RPM fails, Biba check soft-fails) are invisible.

**Plan:**
- Introduce a seam in `pkg_substrate::run_inner` allowing the probe list and the MAC check
  function to be injected in tests.
- Write integration tests that assert:
  - dpkg probe returns `parse_ok=true` when `/var/lib/dpkg/status` is present
  - T3 is not asserted when `check_selinux_enforce` returns false
  - Evidence bundle contains the correct downgrade reason
- Alternatively, run the test suite on a Ubuntu CI runner (GitHub Actions matrix).

**Priority:** Medium — needed before Ubuntu is treated as a supported platform.

### Issue 4 — FIPS mode behavior on Ubuntu

**File:** `src/kattrs/procfs.rs` — `ProcFips`

`/proc/sys/crypto/fips_enabled` exists on Ubuntu but returns `0` unless
`ubuntu-advantage-tools` FIPS mode is explicitly activated. No code change needed — the
value is read correctly either way.

**Plan:**
- Add a doc note in `procfs.rs` that FIPS mode on Ubuntu requires explicit activation via
  `ubuntu-advantage-tools` and that a `0` reading is expected and correct on a standard
  Ubuntu install.

**Priority:** Low (documentation only).

### Cross-Platform Definition of Done

- [ ] Issue 1: Architectural decision recorded; Biba pre-check is substrate-aware
- [ ] Issue 1: Ubuntu with dpkg substrate can assert T3 when MAC check passes
- [ ] Issue 2: Doc note added to `kattrs/security.rs`
- [ ] Issue 3: Test seam introduced; Ubuntu dpkg-path covered by integration tests
- [ ] Issue 4: Doc note added to `kattrs/procfs.rs`
- [ ] `cargo xtask clippy && cargo xtask test` clean on both platforms

---

## DetectionResult Serialization (Future)

The SEC pattern's `decode_cached_result()` currently re-runs the pipeline on cache hit
because `DetectionResult` has no serialization impl. A future iteration should add a
serialization/deserialization layer for `DetectionResult` so that verified cache hits return
the stored result directly — avoiding the pipeline re-run entirely.

This is the key remaining step to realize the full performance benefit of SEC.

**Design considerations:**
- `DetectionResult` contains `OsRelease` (with validated newtypes), `EvidenceBundle`
  (append-only), `ConfidenceModel`, and `SubstrateIdentity` — all of these require
  careful serialization design
- Validated newtypes must re-validate on deserialization, or the deserialization path must
  be treated as a trusted boundary with MAC enforcement
- Options: custom binary format, `serde` with `postcard` (compact binary), CBOR, or JSON
  (human-readable but larger)

**Decided (2026-03-16):** JSON format. Evidence chain must be human-readable for operators
and auditors on CUI/DoD systems. Aligns with the `--json` output standard planned for all
tools, the assessment engine's evidence pipeline, and `umrs-logspace` structured events.

**Sequencing:** Implement after assessment engine types stabilize (decided 2026-03-16).
The type structure is still evolving as the posture probe work proceeds. Serializing an
unstable type creates migration debt.

---

## Architectural Decisions — Resolved

Decided by Jamie on 2026-03-16.

| Decision | Choice | Rationale |
|----------|--------|-----------|
| `CpuIndicatorId` enum design | **(A) Separate `CpuIndicatorId` enum** | Keeps posture catalog and CPU extension catalog from growing into a single unwieldy type. Aligns with rust-developer recommendation. |
| `MandatoryAccessControl` abstraction | **(B) Phase-level decision table** | A trait is over-engineering for two MAC systems. A decision table (`OsFamily` → MAC check) is concrete and testable. Evolve to a trait only if a third MAC system matters. |
| `DetectionResult` serialization format | **(C) JSON** | Evidence chain must be human-readable for operators and auditors on CUI/DoD systems. Aligns with the `--json` output standard planned for all tools, the assessment engine's evidence pipeline, and `umrs-logspace` structured events. Size overhead is acceptable for system state snapshots. |
| Serialization sequencing | **(A) After assessment engine types stabilize** | Type structure is still evolving with posture probe and CPU extension work. Serializing an unstable type creates migration debt. |

---

## Compliance Citations

| Pillar / Section | Controls |
|------------------|----------|
| OS Detection | NIST SP 800-53 CM-8 (component inventory), SA-12 (supply chain risk) |
| Kernel Security Posture Probe | NIST SP 800-53 CM-6 (configuration settings), CA-7 (continuous monitoring) |
| CPU Extension Detection | See `.claude/plans/cpu-extension-probe.md` |
| Cross-Platform Readiness | NIST SP 800-53 CM-6 (configuration settings), SI-7 (software integrity) |
| DetectionResult Serialization | NIST SP 800-53 SI-7 (software integrity), SC-28 (protection of information at rest) |

---

## Umbrella Definition of Done

- [ ] Kernel posture probe complete (all phases — see `kernel-security-posture-probe.md`)
- [ ] Cross-platform issues resolved (all four issues in the Cross-Platform Readiness section)
- [ ] DetectionResult serialization implemented (JSON format, SEC pipeline updated)
- [ ] CPU extension probe complete (see `cpu-extension-probe.md`)
- [ ] All subsidiary plan Definitions of Done satisfied
- [ ] `cargo xtask clippy && cargo xtask test` clean on all supported platforms

---

## Model Assignments

| Work Item | Agent | Model | Rationale |
|---|---|---|---|
| Cross-platform Issue 1 (MAC abstraction) | rust-developer | **opus** | Architectural decision, new trait design, trust tier impact |
| Cross-platform Issue 2 (lockdown doc note) | rust-developer | **haiku** | Simple doc comment addition |
| Cross-platform Issue 3 (Ubuntu test seam) | rust-developer | **sonnet** | Test infrastructure, follows established patterns |
| Cross-platform Issue 4 (FIPS doc note) | rust-developer | **haiku** | Simple doc comment addition |
| DetectionResult Serialization (design) | rust-developer | **opus** | Validated newtype round-trip, format decision |
| DetectionResult Serialization (impl) | rust-developer | **sonnet** | Implementation after design is stable |

---

## DO NOT CHANGE ANY CODE Right Now

This is a planning document. No implementation work begins without an explicit decision from
Jamie. Keep this plan in the queue. Ask questions, record decisions, and update this
document as the work evolves.
