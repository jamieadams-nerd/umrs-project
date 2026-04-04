# Performance Baseline — umrs-platform

**Date:** 2026-04-03
**Crate:** `umrs-platform` v0.1.0
**Benchmark harness:** Criterion 0.5.1 (harness = false)
**Bench file:** `libs/umrs-platform/benches/platform_bench.rs`
**Profile:** release (optimized)
**Author:** Rusty (rust-developer agent)

---

## System Context

| Property | Value |
|---|---|
| Hostname | goldeneye |
| Kernel | 6.12.0-211.el10.aarch64 |
| Architecture | aarch64 |
| CPU implementer | ARM (0x41), architecture ARMv8 |
| CPU cores | 4 (BogoMIPS: 48.00 per core) |
| CPU features | fp, asimd, aes, pmull, sha1, sha2, crc32, atomics, sha3, sha512 |
| Memory (total) | 7.4 GiB |
| Memory (available at run) | ~5.6 GiB |
| Rust toolchain | rustc 1.92.0 (Red Hat 1.92.0-1.el10) |
| Criterion version | 0.5.1 |

**Architecture note:** On aarch64, `umrs_hw::read_hw_timestamp()` uses
`CLOCK_MONOTONIC_RAW` (nanoseconds). Phase `duration_ns` values below are
**nanoseconds**. On x86_64, these would be CPU cycles (~1 cycle/ns on modern
processors). The end-to-end Criterion measurements are wall-clock time in all
cases.

---

## Benchmark Results

### `detect_full_pipeline` — End-to-End Pipeline

Measures `OsDetector::default().detect()` from invocation to `Ok(DetectionResult)`.
This is the authoritative measure of pipeline cost for callers.

| Statistic | Value |
|---|---|
| Mean | 174.45 µs |
| Low bound (95% CI) | 174.05 µs |
| High bound (95% CI) | 174.84 µs |
| Samples | 100 |
| Outliers | 1 high severe (1.00%) |
| Estimated total sample time | ~5.3 s (30k iterations) |

**Interpretation:** The full 7-phase pipeline completes in ~174 µs per call on
this system. The tight CI (±0.4 µs) indicates stable, reproducible results.
One high-severe outlier is typical for a pipeline that does real I/O.

---

### `detect_phase_durations` — Per-Phase Internal Timing

These values come from `DetectionResult::phase_durations`, which records
`umrs_hw::read_hw_timestamp()` spans for each phase. They represent a
**single pipeline run** from the setup call, not a repeated benchmark.

On aarch64, `duration_ns` is nanoseconds from `CLOCK_MONOTONIC_RAW`.

| Phase | duration_ns | Evidence records |
|---|---|---|
| Phase 1 — KernelAnchor | 45,958 ns (~46 µs) | 4 |
| Phase 2 — MountTopology | 72,750 ns (~73 µs) | 3 |
| Phase 3 — ReleaseCandidate | 6,708 ns (~7 µs) | 2 |
| Phase 4 — PkgSubstrate | 186,500 ns (~187 µs) | 3 |
| Phase 5 — FileOwnership | 41,709 ns (~42 µs) | 1 |
| Phase 6 — IntegrityCheck | 29,917 ns (~30 µs) | 1 |
| Phase 7 — ReleaseParse | 11,833 ns (~12 µs) | 2 |
| **Total (saturating sum)** | **395,375 ns (~395 µs)** | **16** |

**Note on discrepancy:** The single-run phase sum (~395 µs) is larger than the
Criterion mean (~174 µs). This is expected — the single-run timing includes I/O
cold-path costs (page faults, dentry lookups, RPM DB open) that amortize heavily
across Criterion's 30k iteration warm-up. The Criterion mean reflects the hot-path
cost after the filesystem cache is warm and the RPM DB connection is open for each
iteration. Both numbers are meaningful — they answer different questions:
- **174 µs** → steady-state cost in a long-running service context (cache warm)
- **~395 µs** → cold-call cost for a fresh system-scan context

---

### Pipeline Trust Result (at baseline run)

| Property | Value |
|---|---|
| Trust level | SubstrateAnchored (T3) |
| Label trust | LabelClaim |

`SubstrateAnchored` (T3) means the RPM substrate was successfully probed and
corroborated OS identity from at least two independent facts. `LabelClaim` means
os-release was parsed and is structurally valid, but digest verification did not
reach T4 (`IntegrityAnchored`). This is consistent with a system where the RPM DB
is present but integrity check could not complete to full T4 (possibly due to the
os-release file not being in the RPM digest DB on this particular RHEL build).

---

## Phase Cost Analysis

| Phase | Cost | Dominant activity |
|---|---|---|
| Phase 4 — PkgSubstrate | 187 µs (47% of total) | RPM DB SQLite query |
| Phase 2 — MountTopology | 73 µs (18%) | /proc/self/mountinfo read + statfs |
| Phase 1 — KernelAnchor | 46 µs (12%) | procfs fstatfs + PID coherence + boot_id |
| Phase 5 — FileOwnership | 42 µs (11%) | RPM DB ownership query |
| Phase 6 — IntegrityCheck | 30 µs (8%) | SHA-256 compute + DB digest fetch |
| Phase 7 — ReleaseParse | 12 µs (3%) | TPI nom + split_once + corroboration |
| Phase 3 — ReleaseCandidate | 7 µs (2%) | statx + symlink resolution |

**Key finding:** Phase 4 (RPM database probe) dominates at ~47% of single-run cost.
This is the expected SQLite connection + query cost. In the Criterion hot-path
measurements, this cost is amortized by the OS page cache warming the SQLite DB.

If pipeline performance becomes a constraint, the `SealedCache` (SEC pattern) in
`umrs-platform` already provides HMAC-sealed result caching to avoid repeated
full pipeline runs within a boot session. The cache path bypasses all 7 phases.

---

## Benchmark Infrastructure Notes

- Criterion HTML reports: `target/criterion/platform_bench/report/index.html`
- Gnuplot was not available; Criterion used the `plotters` backend for charts.
- The `detect_phase_durations` benchmark group measures a trivial tuple return
  in its `b.iter()` loop (by design — the pipeline-internal `duration_ns` values
  cannot be re-measured by criterion without re-running detect()). The per-phase
  `eprintln!` output lines are the meaningful baseline numbers; the Criterion
  timing for those sub-benchmarks (~250 ps) reflects only the tuple return cost.

---

## Regression Tracking

To detect regressions against this baseline in future sessions:

```bash
cargo bench -p umrs-platform --bench platform_bench 2>&1 | grep -E "detect_full_pipeline|time:"
```

A regression is flagged by Criterion automatically on the next run if the mean
exceeds the stored baseline by more than the noise threshold. Criterion stores
its baseline data under `target/criterion/` (not committed; re-generate by running
the benchmark on the target system).

---

## Pre-existing Warning Note

During the bench profile build, two `unused_variables` warnings appeared in
`libs/umrs-platform/src/posture/snapshot.rs` (variables `readable` and `hardened`
at lines 158–159). These are pre-existing in the production source and do not
appear in the `xtask clippy` dev-profile run. They are not introduced by this
benchmark work and are tracked separately.
