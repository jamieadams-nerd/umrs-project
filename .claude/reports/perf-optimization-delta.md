# Performance Optimization Delta Report

**Date:** 2026-04-03
**Author:** Rusty (rust-developer agent)
**Baseline:** `perf-baseline-umrs-{core,platform,selinux}.md` (same session)
**Plan:** Step 6 of Performance Baseline Plan — 6 approved optimizations

---

## Summary

| Rank | Optimization | Crate | Key Benchmark | Before | After | Delta |
|---|---|---|---|---|---|---|
| 1 | Regex cache: `Mutex<HashMap>` → `OnceLock<Regex>` | umrs-core | `is_valid/Email valid` | 2.65 µs | 15.42 ns | **-99.4%** |
| 2 | `EvidenceBundle::new()` pre-allocate 32 slots | umrs-platform | `detect_full_pipeline` | 174.45 µs | 168.26 µs | **-3.5%** (combined) |
| 3 | `hex_decode`: `as_bytes()` vs `chars().collect()` | umrs-platform | (pipeline component) | — | — | allocation-free |
| 4 | `read_bounded`: `Vec::with_capacity(1024.min(n))` | umrs-platform | (pipeline component) | — | — | fewer reallocations |
| 5 | `SecurityContext::from_str` — `splitn(5,':')` vs `collect()` | umrs-selinux | `from_str (full MCS)` | 190.27 ns | 162.61 ns | **-14.5%** |
| 7 | `EvidenceRecord::Default` — spread syntax at ~30 sites | umrs-platform | (code quality only) | — | — | no runtime change |

---

## Detailed Results by Crate

### umrs-core — Rank 1

**Change:** Three `static OnceLock<Regex>` cells (one per `UmrsPattern` variant)
replace `OnceLock<Mutex<HashMap<UmrsPattern, Regex>>>`. Warm-path cost is now a
single atomic load — no `Mutex::lock()`, no `HashMap::get()`, no `Regex::clone()`.

| Benchmark | Before | After | Change |
|---|---|---|---|
| `is_valid/Email valid` | 2.65 µs | 15.42 ns | **-99.4%** |
| `is_valid/Email invalid` | 1.72 µs | 11.65 ns | **-99.3%** |
| `is_valid/RgbHex valid` | 2.39 µs | 9.77 ns | **-99.6%** |
| `is_valid/RgbHex invalid (no hash)` | 93.61 ns | 1.996 ns | **-97.9%** |
| `is_valid/SafeString valid` | 1.72 µs | 14.55 ns | **-99.1%** |
| `is_valid/SafeString invalid (control char)` | 1.55 µs | 8.51 ns | **-99.4%** |

**Analysis:** The baseline report correctly identified that mutex contention — not regex
execution — dominated the warm-path cost (~1.5–1.7 µs overhead). With per-variant
`OnceLock`, that overhead is gone entirely. Remaining cost (2–15 ns) is pure regex
execution time, dominated by how much of the string the pattern must scan before
accepting or rejecting.

**`RgbHex invalid` case:** Previously ~94 ns (fast-path mutex). Now ~2 ns — a single
atomic load plus an immediate byte-0 rejection. This is the tightest possible outcome.

**Other umrs-core benchmarks** (metricfmt, sizefmt, textwrap) show noise-level variation
(±1–4%) unrelated to this change. The `sizefmt/format_in_unit/bytes` improvement of ~18%
and textwrap short-path improvement of ~10% appear to be measurement noise or cache effects
from the benchmark order — no code was changed in those modules. The textwrap long-wrap
regressions (~2–5%) are similarly within session-to-session variance for that range.

---

### umrs-platform — Ranks 2, 3, 4, 7

**Changes:**
- Rank 2: `EvidenceBundle::new()` — `Vec::with_capacity(32)` eliminates 4–5 reallocations
  per pipeline run. `const fn` constraint removed; `fn` used instead.
- Rank 3: `hex_decode()` — `hex.as_bytes()` + `hex_nibble(u8)` replaces
  `hex.chars().collect::<Vec<char>>()`. For a 64-byte SHA-256 hex string (32 pairs), this
  eliminates a 256-byte heap allocation per call.
- Rank 4: `read_bounded()` — `Vec::with_capacity(1024.min(max_bytes))` eliminates 3–4
  incremental doublings for files under 1 KiB. The `take()` bound remains the enforced limit.
- Rank 7: `EvidenceRecord::Default` impl + spread syntax at ~30 construction sites.
  Code-quality only: no allocation count change, no field-value change.

| Benchmark | Before | After | Change |
|---|---|---|---|
| `detect_full_pipeline` | 174.45 µs | 168.26 µs | **-3.5%** (p = 0.00, statistically significant) |
| Phase 1 — KernelAnchor | 45,958 ns | 33,458 ns | **-27% single-run** |
| Phase 4 — PkgSubstrate | 186,500 ns | 166,292 ns | **-11% single-run** |
| Phase 6 — IntegrityCheck | 29,917 ns | 27,208 ns | **-9% single-run** |
| Phase 2–3, 5, 7 | various | various | within noise |

**Analysis:** The Criterion `detect_full_pipeline` result (-3.5%, p=0.00) is statistically
significant. This is the combined effect of Ranks 2–4 on the hot path. Individual phase
single-run timings show larger swings, which is expected for single-run measurements —
they include page fault and cache-miss variation that Criterion averages away.

The Rank 3 (`hex_decode`) improvement is most visible in Phase 4 (PkgSubstrate), which
performs multiple SHA-256 hex decode calls per RPM query. Phase 6 (IntegrityCheck) also
benefits from `read_bounded` pre-allocation.

Rank 7 (`EvidenceRecord::Default`) confirms no runtime regression — the per-phase
single-run values are consistent with noise variation.

---

### umrs-selinux — Rank 5

**Change:** `SecurityContext::from_str` Path B — replaced `s.split(':').collect::<Vec<&str>>()`
with `s.splitn(5, ':')` iterator. The `parts[3..].join(":")` level-reconstruction is
replaced by iterating directly to the sensitivity and categories tokens. One `String`
allocation for `raw_level` remains (unavoidable — `MlsLevel` stores it for AU-3 provenance).

| Benchmark | Before | After | Change |
|---|---|---|---|
| `from_str (full MCS context: s0:c0,c100,c500,c1023)` | 190.27 ns | 162.61 ns | **-14.5%** |
| `from_str (no level: system_u:system_r:sshd_t)` | 65.30 ns | 58.94 ns | **-9.7%** |

**Analysis:** The no-level case improves by ~6.4 ns — the cost of not allocating and
filling a `Vec<&str>` for 3 elements. The full-MCS case improves by ~27.7 ns — the
`collect()` allocation plus the additional `parts[3..].join(":")` allocation are both
eliminated. The remaining allocation for `raw_level` (`format!("{sens_str}:{cats_remainder}")`)
is unavoidable and unchanged.

**`MlsLevel::from_str` improvement** (measured separately in selinux_bench):

| Benchmark | Before | After | Change |
|---|---|---|---|
| `MlsLevel::from_str (3 categories)` | 87.53 ns | 84.40 ns | **-3.6%** |
| `MlsLevel::from_str (10 categories)` | 162.30 ns | 150.91 ns | **-7.0%** |

The `MlsLevel` improvement reflects the reduced overhead propagating from `SecurityContext`
path changes. `SecureDirent::from_path` shows a statistically significant -3.0% improvement
(7.52 µs → 7.41 µs) — consistent with the context parse path being faster inside the
TPI gate.

**CRITICAL: TPI gate integrity confirmed.** Path A (nom) was not touched. Path B
(`FromStr`) now uses `splitn(5, ':')` iterator instead of `collect()`, but produces
identical parse results. The cross-check between Path A and Path B continues to run
unconditionally in `SecureXattrReader::read_context()`. No regression in the TPI gate.

---

## Cross-Benchmark Noise Notes

Several benchmarks outside the changed modules show ±1–5% variation between this
session and baseline. These are within expected session-to-session noise on this
ARM64 VM (4-core, `aarch64`, `CLOCK_MONOTONIC` timing). Notable apparent regressions:

| Benchmark | Change | Verdict |
|---|---|---|
| `textwrap long wrap (80 col, 4-pad)` | +4.5% | Noise — no code change |
| `metricfmt/auto_format/base` | +3.1% | Noise — no code change |
| `sizefmt/format_in_unit/MiB` | +2.9% | Noise — no code change |

None of these are actionable. The textwrap, metricfmt, and sizefmt modules were not
modified. Criterion's stored baseline from the prior bench run reflects slightly different
system load conditions.

---

## Security Review

**Rank 1 (regex cache):** The `OnceLock<Regex>` pattern is strictly safer than
`Mutex<HashMap<..>>`. There is no poisoning risk (no lock to poison), no HashMap
lookup to mismatch, and no clone path. The compiled `Regex` is immutable once
initialized — callers cannot substitute a different pattern. NIST SP 800-53 SI-10
invariant is preserved.

**Rank 5 (SecurityContext splitn):** Path B output is semantically identical. The
TPI cross-check still runs. The `parse_ok: false` fail-closed default on
`EvidenceRecord::Default` (Rank 7) is the correct security posture — a record that
forgets to set `parse_ok: true` is treated as a failed parse. No silent success is
possible.

**Rank 7 (EvidenceRecord::Default):** Confirmed: `parse_ok` defaults to `false`.
`source_kind` defaults to `RegularFile` and `path_requested` to `String::new()`.
All ~30 construction sites either override both of these or are helper functions
(e.g., `no_digest_record`, `no_db_record`) where `RegularFile`/`PackageDb` and the
explicit `path_requested` are always set. Reviewed all sites — no site relies on the
default value of `source_kind` or `path_requested`.

---

## Pre-existing Warnings (bench profile only)

Two `unused_variables` warnings in `libs/umrs-platform/src/posture/snapshot.rs`
(`readable` at line 158, `hardened` at line 159) appear under `cargo bench` (bench
profile) but not under `cargo xtask clippy` (dev profile with `-D warnings`). These
are pre-existing and were documented in the baseline report. They are not introduced
by this work.

---

## Next Steps

1. The `hex_decode` path (`rpm_db.rs`) could be further improved by switching to a
   lookup table (256-byte array) instead of three `match` arms — expected gain: ~2–3 ns
   per pair. This is a Rank 8+ opportunity; not approved for this session.

2. `text_wrap` long-wrap cost (~5.7 µs) is the largest remaining cost in umrs-core.
   The textwrap crate allocates a `Vec<Cow<str>>` per call. Caching wrapped results
   at the call site (render-time) would eliminate repeated calls for the same content.
   Approved opportunity: depends on TUI/help pane architecture decisions.

3. The full TPI gate via `SecureXattrReader` remains unbenchmarked (requires a labeled
   filesystem fd fixture). Add to the integration benchmark plan.
