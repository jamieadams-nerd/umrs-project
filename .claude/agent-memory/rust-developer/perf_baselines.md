---
name: Performance Baselines
description: Criterion benchmark results for umrs-core, umrs-platform, and umrs-selinux; key cost figures for regression detection
type: project
---

## umrs-core (2026-04-03, aarch64 goldeneye, rustc 1.92.0)

Bench file: `libs/umrs-core/benches/core_bench.rs`
Report: `.claude/reports/perf-baseline-umrs-core.md`

| Operation | Mean |
|---|---|
| `Timed::measure` overhead (Instant floor) | ~35 ns |
| `textwrap::text_wrap` short no-wrap | ~44 ns |
| `metricfmt::format_in_prefix` | ~65 ns |
| `metricfmt::auto_format` (base range) | ~69 ns |
| `sizefmt::format_in_unit/MiB` | ~60 ns |
| `sizefmt::auto_format/binary` | ~64 ns |
| `sizefmt::auto_format/decimal` | ~108 ns |
| `metricfmt::auto_format` (nano/kilo range) | ~118 ns |
| `validate::is_valid/RgbHex invalid` (early exit) | ~94 ns |
| `validate::is_valid/SafeString` | ~1.5–1.7 µs |
| `validate::is_valid/Email` | ~1.7–2.7 µs |
| `textwrap::text_wrap` (long paragraph, 80 col) | ~5.5 µs |
| `textwrap::text_wrap` (long paragraph, 40 col) | ~6.2 µs |

Key: `validate::is_valid` dominated by Mutex lock (~1.5 µs), not regex.
`text_wrap` is the most expensive at ~5–6 µs; safe for one-shot render calls only.

## umrs-selinux (2026-04-03, aarch64 goldeneye, rustc 1.92.0)

Bench file: `libs/umrs-selinux/benches/selinux_bench.rs`
Report: `.claude/reports/perf-baseline-umrs-selinux.md`

| Operation | Mean |
|---|---|
| `SensitivityLevel` ordinal comparison | ~0.70 ns |
| `CategorySet::contains` | ~0.83 ns |
| `CategorySet::dominates` | ~1.00 ns |
| `CategorySet::union` / `intersection` | ~5.43 ns |
| `SensitivityLevel::from_str` | ~1.77 ns |
| `MlsLevel::from_str` (sensitivity only) | ~53.85 ns |
| `SecurityContext::from_str` (no level) | ~65.30 ns |
| `MlsLevel::from_str` (3 cats) | ~87.53 ns |
| `MlsLevel::from_str` (10 cats) | ~162.30 ns |
| `SecurityContext::from_str` (full MCS, 4 cats) | ~190.27 ns |
| `SecureDirent::from_path` | ~7.52 µs |

Per-category cost in `MlsLevel::from_str`: ~10.8 ns/category.
`MlsRange` not benchmarked — implementation is a stub as of this date.

## umrs-platform (2026-04-03, aarch64 goldeneye, rustc 1.92.0)

Bench file: `libs/umrs-platform/benches/platform_bench.rs`
Report: `.claude/reports/perf-baseline-umrs-platform.md`

| Operation | Mean |
|---|---|
| Full `OsDetector::detect()` pipeline | ~174.45 µs |
| Phase 4 — PkgSubstrate (SQLite) | ~187 µs (cold), dominates |

**Why:** Pre-existing unused-variable warnings in `posture/snapshot.rs` appear
during `cargo bench` (bench profile) but not during `cargo xtask clippy`
(which uses `-D warnings` on the clippy pass). These are harmless and pre-existing.

## Step 3 Debug Instrumentation — umrs-core (2026-04-03)

`log` 0.4 (`release_max_level_info`) added to `libs/umrs-core/Cargo.toml`.
`#[cfg(debug_assertions)]` timing added to two instrumentation targets:

| Location | Pattern | What is logged |
|---|---|---|
| `validate.rs` — `is_valid()` | `Instant` t0 before get_regex+is_match, debug log after result | pattern name (enum variant), bool result, elapsed µs |
| `human/textwrap.rs` — `text_wrap()` | `Instant` t0 at entry, debug log after join | input char count, width, elapsed µs |

`Timed::measure`, `sizefmt`, `metricfmt` skipped — Timed would be circular; fmt ops are ~40-170 ns, too fast.

`release_max_level_info` means `log::debug!()` compiles to nothing in release builds.
`#[cfg(debug_assertions)]` additionally removes the `Instant` capture itself in release.

## Step 3 Debug Instrumentation — umrs-selinux (2026-04-03)

`#[cfg(debug_assertions)]` timing added to three hot paths in `umrs-selinux`:

| Location | Pattern |
|---|---|
| `context.rs` — `SecurityContext::from_str` | `Instant` start + debug log at success exit |
| `mls/level.rs` — `MlsLevel::from_str` | `Instant` start + debug log at Ok exit |
| `secure_dirent.rs` — `SecureDirent::from_path` | `Instant` start + debug log with label_state enum (not string) |

`CategorySet` ops skipped — ~1–5 ns operations; instrumentation overhead would dwarf the measurement.
`xattrs.rs::read_context` was already fully instrumented (Instant + #[cfg(debug_assertions)] + log).

**Error Information Discipline fix applied**: `context.rs` line 218 previously emitted the raw
MLS level string in a `log::debug!` call (violation of SI-11). Fixed to log only structural
metadata (part count). The linter renames `_start` → `start` inside `#[cfg(debug_assertions)]`
blocks — this is correct behavior; use `start` not `_start` when both declaration and use
are inside the same cfg block.

## Step 6 Optimization Results (2026-04-03)

Report: `.claude/reports/perf-optimization-delta.md`

6 of 7 approved optimizations implemented. Key actual results:

| Rank | Change | Key Result |
|---|---|---|
| 1 | `OnceLock<Regex>` per variant | `is_valid/Email valid`: 2.65 µs → 15.4 ns (**-99.4%**) |
| 2 | `EvidenceBundle::with_capacity(32)` | `detect_full_pipeline`: 174.45 µs → 168.26 µs (**-3.5%**) |
| 3 | `hex_decode` byte-slice | allocation-free; contributes to Rank 2 result |
| 4 | `read_bounded` pre-allocation | allocation-efficient; contributes to Rank 2 result |
| 5 | `SecurityContext::from_str` splitn | full MCS: 190.27 ns → 162.61 ns (**-14.5%**) |
| 7 | `EvidenceRecord::Default` + spread | code quality only — 0 runtime change confirmed |

Updated hot-path baselines after optimizations:

| Operation | Before | After |
|---|---|---|
| `validate::is_valid/Email valid` | 2.65 µs | 15.4 ns |
| `validate::is_valid/RgbHex invalid` | 93.6 ns | 2.0 ns |
| `SecurityContext::from_str (full MCS)` | 190.27 ns | 162.61 ns |
| `SecurityContext::from_str (no level)` | 65.30 ns | 58.94 ns |
| `MlsLevel::from_str (10 cats)` | 162.30 ns | 150.91 ns |
| `detect_full_pipeline` | 174.45 µs | 168.26 µs |

TPI gate integrity: Path A (nom) untouched. Path B (splitn) produces identical results.
Cross-check continues to run unconditionally. Confirmed by test suite.

## Steps 4+5 Optimization Analysis (2026-04-03)

Report: `.claude/reports/perf-optimization-opportunities.md`

Ranked opportunities:

| Rank | Item | Expected Gain | Risk |
|---|---|---|---|
| 1 | `OnceLock<Regex>` per variant (validate.rs) | 50–70% on is_valid cached path | Low |
| 2 | `EvidenceBundle::new()` → `with_capacity(32)` | ~10–15% pipeline alloc reduction | Very low |
| 3 | `hex_decode()` → byte slice (not Vec<char>) | ~15–20% on Phase 6 | Low |
| 4 | `read_bounded()` → `with_capacity(1024)` | ~5–10% on Phase 6 | Very low |
| 5 | `context::from_str` splitn iterator (no Vec<&str>) | ~10–15% on TPI Path B | Low |
| 6 | Batch Phase 5+6 RPM queries | ~17 µs combined | Medium — needs Jamie |
| 7 | EvidenceRecord builder / Default | Code quality only, no runtime | Very low |

Key investigation finding: SQLite connection IS shared across Phases 4/5/6 via
`Mutex<Option<RpmDb>>` in RpmProbe — connection opened exactly once per pipeline run.
No re-open opportunity (none needed).

Phase 7 TPI (ReleaseParse) confirmed DO-NOT-OPTIMIZE per task brief.
SecureDirent::from_path 7-syscall cost confirmed irreducible (all TOCTOU-required).
Rank 6 requires Jamie approval before implementation (phase architecture change).
