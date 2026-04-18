# Performance Baseline Plan

**Status:** completed — criterion harnesses + 6 approved optimizations landed 2026-04-03
**Created**: 2026-03-16
**ROADMAP**: G3 (High-Assurance Platform), G6 (Tooling)
**Scope**: umrs-platform, umrs-selinux, umrs-core

## Objective

Establish criterion benchmarks, record baselines, add debug-only instrumentation,
then identify and rank optimization opportunities — one crate at a time.

## Execution Order

1. **umrs-platform** — `OsDetector::detect()` and its 7-phase pipeline
2. **umrs-selinux** — TPI parsing, `CategorySet` operations, `SecureDirent`
3. **umrs-core** — formatting, i18n, timing utilities

Each crate follows the same 6-step process below.

---

## The 6-Step Process (per crate)

### Step 1: Criterion Benchmark Harness

**New dev-dependency** in the crate's `Cargo.toml`:
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "<crate>_bench"
harness = false
```

**Security impact**: Zero. `criterion` is `[dev-dependencies]` — never compiled into
release binaries. Benchmark files live in `benches/`, outside `src/`.

**Supply chain**: criterion 0.5 is the de facto Rust benchmarking standard. Transitive
deps: plotters, clap, serde, csv, tinytemplate, ciborium. All well-established.

#### umrs-platform Benchmark Design

**File**: `umrs-platform/benches/detect_bench.rs`

Approach: **Option B** — benchmark the full `detect()` call, extract per-phase
breakdowns from `DetectionResult::phase_durations`. No visibility changes to private
phase modules.

| Benchmark | What it measures |
|---|---|
| `detect_full_pipeline` | `OsDetector::default().detect()` end-to-end |
| Per-phase extraction | From `phase_durations` in the result (custom metric reporting) |

Note: Benchmarks touch real `/proc/`, `/sys/`, and RPM DB — must run on a live
RHEL/Fedora system. CI environments without these will need graceful skip logic.

#### umrs-selinux Benchmark Design (planned)

Target functions TBD during Step 4 analysis. Likely candidates:
- `SecurityContext::from_str()` — TPI dual-path parsing
- `CategorySet` bit operations (union, intersection, dominates)
- `SecureDirent::from_fd()` — fd-anchored directory entry construction
- `SensitivityLevel` comparisons
- `MlsRange` dominance checks

#### umrs-core Benchmark Design (planned)

Target functions TBD. Likely candidates:
- Formatting utilities (report layout, column alignment)
- Any i18n string lookup paths

### Step 2: Record Baseline Results

```bash
cargo bench -p <crate> --bench <crate>_bench -- --save-baseline initial
```

Record summary in `.claude/reports/perf-baseline-<crate>.md`:
- Mean, median, std dev for each benchmark
- System specs (CPU, kernel version, RPM DB size where applicable)
- Per-phase breakdown for umrs-platform (from `PhaseDuration` records)

### Step 3: Debug Instrumentation — `log::debug!()`

Add `log::debug!()` calls at key measurement points in the crate's hot paths.

**Production code safety**:
- The `log` crate with `release_max_level_info` compiles `debug!()` to a no-op in
  release builds. LLVM eliminates the entire call body at compile time — verified by
  checking `log::STATIC_MAX_LEVEL` against `Level::Debug`.
- No new dependencies added to `[dependencies]`.
- No feature gates or visibility changes needed.
- Verify with `cargo-asm` that release builds contain zero instructions for debug calls.

**Error Information Discipline (SI-11)**: Debug output must contain only phase/function
names and timing values — never configuration file contents, kernel attribute values,
or security label data.

#### umrs-platform Instrumentation

Location: end of `OsDetector::detect()`, after all phases complete.

```rust
// Emit per-phase timing at debug level.
for pd in &phase_durations {
    log::debug!(
        "[{phase}] duration_ns={dur} records={rec}",
        phase = pd.phase.name(),
        dur = pd.duration_ns,
        rec = pd.record_count,
    );
}
let total_ns: u64 = phase_durations
    .iter()
    .map(|pd| pd.duration_ns)
    .fold(0u64, u64::saturating_add);
log::debug!(
    "[detect_pipeline] total_duration_ns={total} phases={count}",
    total = total_ns,
    count = phase_durations.len(),
);
```

Requires: `DetectionPhase::name(&self) -> &'static str` method (new, needs compliance
annotation — NIST SP 800-53 AU-8).

#### umrs-selinux / umrs-core Instrumentation (planned)

**Timing source**: These crates do not depend on `umrs-hw` directly (dependency
rules prohibit it). For debug instrumentation, route through umrs-platform's
re-exported timestamp API (`umrs_platform::timestamp::BootSessionTimestamp`).
umrs-selinux already depends on umrs-platform; umrs-core depends on both.

**Future consideration**: If profiling reveals that RDTSCP-level granularity is
needed in these crates (sub-microsecond measurement of parsing or bit operations),
evaluate adding `umrs-hw` as a direct dependency at that time. For now, criterion
handles benchmark measurement and `BootSessionTimestamp` (backed by
`CLOCK_MONOTONIC_RAW`) is sufficient for debug instrumentation.

TBD — will follow the same pattern. Instrument TPI validation timing,
category set operations, and any function identified as a hot path in Step 4.

### Step 4: Optimization Opportunity Analysis

Review implementation against the rust-performance corpus
(`.claude/references/performance-corpus/rust-performance/`).

#### umrs-platform Areas to Investigate

| Area | What to look for |
|---|---|
| Phase 4 (PkgSubstrate) | SQLite query cost — connection open/close per call? |
| Phase 6 (IntegrityCheck) | SHA-256 buffer allocation — optimal for small files? |
| Phase 2 (MountTopology) | mountinfo parsing — actual vs. allocated buffer size |
| Phase 7 (ReleaseParse) | TPI dual parsing — NO optimization (correctness requirement) |
| Cross-phase | EvidenceBundle Vec growth — pre-allocation feasibility |
| Cross-phase | read_hw_timestamp() overhead — 14 calls, cost of timing itself |

#### umrs-selinux / umrs-core Areas (planned)

TBD — identified after baseline measurement.

For each opportunity, document:
- What the opportunity is
- Which function/phase it affects
- Expected improvement (memory, latency, throughput)
- Risk to correctness or audit guarantees
- NIST control implications if any

### Step 5: Ranked Opportunity List

Present ranked list sorted by impact/risk ratio:

```
Rank | Opportunity | Location | Expected Gain | Risk | Controls
1    | ...         | ...      | ...           | ...  | ...
```

**Await Jamie's approval before proceeding.**

### Step 6: Sequential Implementation

For each approved optimization:
1. Implement the single change
2. `cargo bench -p <crate> --bench <crate>_bench -- --baseline initial`
3. Compare against baseline — confirm improvement materialized
4. Verify `log::debug!()` output still correct (`RUST_LOG=debug`)
5. `cargo xtask test` — full test suite
6. Report results, move to next

**No batching. One at a time.**

---

## Security Posture Assessment

| Change | Production impact | Assessment |
|---|---|---|
| `criterion` in `[dev-dependencies]` | None — not in release binary | Safe |
| `benches/*.rs` files | Not compiled in release | Safe |
| `log::debug!()` in `src/` | Compiled out by `release_max_level_info` | Safe — verify with cargo-asm |
| `DetectionPhase::name()` method | Trivial match, `&'static str` return | Safe — add compliance annotation |
| No `pub` visibility changes | N/A | No encapsulation changes |
| No new `[dependencies]` | N/A | No supply chain expansion |

**Conclusion**: Zero impact on release binary security posture. All benchmark
infrastructure is dev-only. The only production code change (debug logging) is
eliminated at compile time by LLVM.

---

## Deliverables

Per crate:
- `<crate>/benches/<crate>_bench.rs` — criterion benchmark harness
- `.claude/reports/perf-baseline-<crate>.md` — recorded baseline
- Debug instrumentation in source (compile-time eliminated in release)
- Ranked optimization opportunity report
- Implemented optimizations (after approval), each with before/after measurements
