---
name: umrs-platform Expansion Roadmap
path: components/rusty-gadgets/umrs-platform
agent: rust-developer
status: active umbrella — subsidiary plans in progress
---

## Display Grouping Reference

When posture signals are presented to operators (TUI, reports, `--json`), organize them
under Jamie's 7-domain Capability Matrix. See `.claude/references/capability-matrix-domains.md`
for the full mapping. Source: `.claude/jamies_brain/kernel-probe-grouping.txt`.

## Vision

`umrs-platform` is the low-level OS and kernel layer for UMRS. It currently contains OS
detection and kernel attribute (kattrs) probing. The crate is expanding across three pillars:

1. **OS Detection** — done; `OsDetector` is the public entry point
2. **Kernel Security Posture Probe** — in progress; `SignalId` catalog, contradiction engine,
   snapshot pipeline
3. **CPU Extension Detection** — future; three-layer hardware/OS/software activation model

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
- Requires an architectural decision before implementation (see Open Architectural Decisions)

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

## CPU Extension Detection (Future)

CPU extension detection is future work. It will not begin until:
1. The kernel security posture probe project is complete (all phases)
2. The CPU security corpus research is validated (see `cpu-security-corpus-plan.md`)

The research corpus plan is the gate. The corpus sits ready; implementation follows.

The goal is tools to query whether security, high-assurance, or cryptographic extensions
are available — and whether they are actually being used. These can also be used to audit
ELF binaries to see if things are linked using extensions.

### The Three-Layer Activation Model

Having a CPU extension does not mean it is being used. Utilization depends on several
layers of the software stack. In most cases the extension must be explicitly enabled by
the compiler, runtime library, or application code. Only a small subset are transparently
used by the system.

To assess whether a platform actually benefits from an extension, think in terms of three
layers:

**Layer 1 — Hardware Availability (CPU Capability)**

At the lowest layer the processor advertises support through CPUID flags. These appear in
`/proc/cpuinfo`, `cpuid`, and `lscpu`. This only means the silicon supports the
instruction. Nothing is using it yet.

Security-relevant examples: AES-NI, AVX, AVX2, AVX-512, SHA, RDRAND, RDSEED, SGX, BMI1,
BMI2, ADX, VAES, VPCLMULQDQ, SMEP, SMAP, CET-SS, CET-IBT, NX/XD.

Detection path: `/proc/cpuinfo` flags line (safe Rust path, no unsafe required).

**Layer 2 — OS Enablement (Kernel Support)**

Some extensions require the operating system to enable state management. The kernel enables
these through mechanisms like XSAVE, the XCR0 register, and CR4 flags. If the OS does not
enable the feature, software cannot use it even if the CPU supports it.

| Extension  | OS Involvement | Reason                                    |
|------------|----------------|-------------------------------------------|
| AVX / AVX2 | YES            | Kernel must save/restore vector registers |
| AVX-512    | YES            | Large register context                    |
| SGX        | YES            | Enclave management                        |
| PKU        | YES            | Protection key management                 |
| AMX        | YES            | Tile state management                     |

Detection path: `/proc/self/status` (xsave flags), kernel-managed sysfs nodes.

**Layer 3 — Software Utilization (Compiler / Library / Application)**

This is the most important layer. The majority of extensions are only used if software is
compiled to target them. Three common patterns:

- **Compile-time targeting**: compiler generates instructions directly
  (`-C target-cpu=native`, `-mavx2`, `-maes`)
- **Runtime CPU dispatch**: libraries detect CPU features dynamically and select the
  fastest implementation path (OpenSSL, libcrypto, zlib-ng, Rust std crypto backends)
- **Intrinsics / assembly**: code directly calls CPU instruction wrappers
  (`_mm_aesenc_si128()`)

Detection path: `/proc/crypto` (kernel crypto driver registration); ELF binary inspection
(`objdump -d binary | grep aesenc`); `OPENSSL_ia32cap` environment variable.

**The High-Assurance Insight**

For a security evaluation platform like UMRS, the real questions are:
1. Does the CPU support the instruction?
2. Did the kernel enable it?
3. Do the cryptographic libraries actually use it?
4. Was the software compiled to take advantage of it?

Only when all four are true does the platform get the full benefit. A CPU with AES-NI that
runs a binary compiled with `-march=x86-64` gets no AES-NI acceleration.

**Practical mental model:**

```
CPU capability
    ↓
OS enablement
    ↓
compiler support
    ↓
library implementation
    ↓
application usage
```

Failure at any level means the extension is effectively unused.

### Extensions That Are Automatically Used

A small subset of extensions are automatically picked up by standard crypto libraries:
- **AES-NI** — OpenSSL, BoringSSL, libsodium, Rust `ring` all detect and use it
- **SHA extensions** — used automatically if compiled into crypto libraries
- **RDRAND / RDSEED** — used by kernel entropy pools and crypto libraries
- **PCLMULQDQ** — used automatically by AES-GCM implementations

### Extensions That Require Explicit Enablement

These require intentional optimization at the compiler or library level:
AVX-512, AMX, BMI1/BMI2, ADX, SHA512 extensions, VAES. If software was compiled
generically (`-march=x86-64`), none of these will be used.

### Proposed `CpuSignalId` Design

Open question (see Open Architectural Decisions): separate `CpuSignalId` enum vs extending
the existing `SignalId` enum. The rust-developer recommendation is a separate enum to keep
the posture catalog and the CPU extension catalog from growing into a single unwieldy type.

CPU extension detection will likely produce a result type parallel to `SignalReport` but
scoped to hardware capability assertions. Design details to be determined after the corpus
research is complete.

### Reference

The full feature inventory (60 features across 15 categories, 9 detection interfaces,
23-column matrix) is in `.claude/plans/cpu-security-corpus-plan.md`.

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

**Open question:** which format — see Open Architectural Decisions.

**Sequencing:** rust-developer recommends implementing serialization after the assessment
engine type design is stable, not before. The type structure is still evolving as the posture
probe and CPU extension work proceeds. Serializing an unstable type creates migration debt.

---

## Architectural Decisions — Resolved

Decided by Jamie on 2026-03-16.

| Decision | Choice | Rationale |
|----------|--------|-----------|
| `CpuSignalId` enum design | **(A) Separate `CpuSignalId` enum** | Keeps posture catalog and CPU extension catalog from growing into a single unwieldy type. Aligns with rust-developer recommendation. |
| `MandatoryAccessControl` abstraction | **(B) Phase-level decision table** | A trait is over-engineering for two MAC systems. A decision table (`OsFamily` → MAC check) is concrete and testable. Evolve to a trait only if a third MAC system matters. |
| `DetectionResult` serialization format | **(C) JSON** | Evidence chain must be human-readable for operators and auditors on CUI/DoD systems. Aligns with the `--json` output standard planned for all tools, the assessment engine's evidence pipeline, and `umrs-logspace` structured events. Size overhead is acceptable for system state snapshots. |
| Serialization sequencing | **(A) After assessment engine types stabilize** | Type structure is still evolving with posture probe and CPU extension work. Serializing an unstable type creates migration debt. |

---

## Compliance Citations

| Pillar / Section | Controls |
|------------------|----------|
| OS Detection | NIST SP 800-53 CM-8 (component inventory), SA-12 (supply chain risk) |
| Kernel Security Posture Probe | NIST SP 800-53 CM-6 (configuration settings), CA-7 (continuous monitoring) |
| CPU Extension Detection | NIST SP 800-53 SC-13 (cryptographic protection), SI-7 (software integrity) |
| Cross-Platform Readiness | NIST SP 800-53 CM-6 (configuration settings), SI-7 (software integrity) |
| DetectionResult Serialization | NIST SP 800-53 SI-7 (software integrity), SC-28 (protection of information at rest) |

---

## Umbrella Definition of Done

- [ ] Kernel posture probe complete (all phases — see `kernel-security-posture-probe.md`)
- [ ] CPU corpus research complete and validated (Phase 0.5 through corpus completion — see `cpu-security-corpus-plan.md`)
- [ ] CPU extension detection implemented (`CpuSignalId` design decided, three-layer probe built)
- [ ] Cross-platform issues resolved (all four issues in the Cross-Platform Readiness section)
- [ ] DetectionResult serialization implemented (format decided, SEC pipeline updated)
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
| CPU Extension Detection (Phase 3 design) | rust-developer | **opus** | New signal catalog, three-layer model, CpuSignalId design |
| DetectionResult Serialization (design) | rust-developer | **opus** | Validated newtype round-trip, format decision |
| DetectionResult Serialization (impl) | rust-developer | **sonnet** | Implementation after design is stable |

---

## DO NOT CHANGE ANY CODE Right Now

This is a planning document. No implementation work begins without an explicit decision from
Jamie. Keep this plan in the queue. Ask questions, record decisions, and update this
document as the work evolves.
