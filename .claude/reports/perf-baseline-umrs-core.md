# Performance Baseline — umrs-core

**Date:** 2026-04-03
**Crate:** `umrs-core` v0.1.0
**Benchmark harness:** Criterion 0.5 (harness = false)
**Bench file:** `libs/umrs-core/benches/core_bench.rs`
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
| CPU BogoMIPS | 48.00 per core |
| CPU features | fp, asimd, aes, pmull, sha1, sha2, crc32, atomics, sha3, sha512, asimddp (subset) |
| Memory (total) | 7.4 GiB (~7,777,856 KiB) |
| Rust toolchain | rustc 1.92.0 (Red Hat 1.92.0-1.el10) |
| Criterion version | 0.5.1 |

**Architecture note:** All Criterion measurements are wall-clock time on a 4-core
ARM64 VM. The formatting functions (metricfmt, sizefmt) resolve to simple floating-point
arithmetic and `format!` calls. The textwrap benchmarks are the heaviest, reflecting
the word-splitting and string allocation cost of the `textwrap` crate. The validate
benchmarks are dominated by Mutex lock acquisition for the regex cache, not by regex
execution itself.

---

## Benchmark Results

### Group 1: `metricfmt` — SI-Prefix Scaling

`auto_format` selects the best SI prefix from a 12-element table via a linear scan,
scales the value, and builds a formatted string. `format_in_prefix` skips the prefix
walk but follows the same format path.

| Benchmark | Mean | Low (95% CI) | High (95% CI) | Outliers |
|---|---|---|---|---|
| `auto_format/nano (842 ns)` | 118.91 ns | 117.03 ns | 121.56 ns | 6 (6%) |
| `auto_format/kilo (12.5 kHz)` | 117.92 ns | 116.60 ns | 119.84 ns | 2 (2%) |
| `auto_format/base (42.7)` | 69.18 ns | 69.14 ns | 69.23 ns | 5 (5%) |
| `format_in_prefix/micro forced` | 64.53 ns | 64.48 ns | 64.57 ns | 2 (2%) |

**Analysis:**

- **Prefix-selection cost:** ~54 ns overhead for `auto_format` relative to `format_in_prefix`
  on the same value scale. The linear prefix walk (up to 12 iterations) accounts for most of
  this. For the `base` case (value 42.7), the walk exits at the 7th entry and costs ~5 ns
  more than `format_in_prefix` — consistent with a short early-exit path.
- **Dominant cost:** String allocation from `format!("{:.*}", decimals, scaled)`. Both
  `auto_format` and `format_in_prefix` produce a `String` on every call; this heap
  allocation is ~60–65 ns on this platform.
- **Conclusion:** These functions are not hot-path bottlenecks for posture reports
  (called O(tens) of times per report). If called in a tight loop (e.g., a live TUI
  update at 60 Hz), consider caching formatted strings across frames.

---

### Group 2: `sizefmt` — Byte-Size Formatting

`auto_format` delegates to the `humansize` crate. `format_in_unit` performs a
manual `u128` division followed by a `format!` call.

| Benchmark | Mean | Low (95% CI) | High (95% CI) | Outliers |
|---|---|---|---|---|
| `auto_format/decimal (1.5 GB)` | 107.53 ns | 106.44 ns | 109.17 ns | 4 (4%) |
| `auto_format/binary (1.5 GiB)` | 64.25 ns | 64.11 ns | 64.38 ns | 1 (1%) |
| `format_in_unit/MiB (512 MB as MiB)` | 59.62 ns | 59.56 ns | 59.67 ns | 6 (6%) |
| `format_in_unit/bytes (4096 B)` | 102.99 ns | 102.33 ns | 104.29 ns | 2 (2%) |

**Analysis:**

- **Decimal vs binary disparity:** `auto_format/decimal` costs ~43 ns more than
  `auto_format/binary`. The `humansize` DECIMAL formatter appears to perform
  additional string work (likely an extra unit-label lookup or a more complex
  formatting path) compared to the BINARY path.
- **`format_in_unit/bytes` anomaly:** The raw-bytes case (divisor = 1, no division)
  costs ~43 ns more than `format_in_unit/MiB`. The MiB case produces a compact
  3–4 digit number while the bytes case produces `"4096 B"` — the extra character
  in the format string and the integer-to-string conversion for a 4-digit number
  explain the difference.
- **Conclusion:** All `sizefmt` operations are well under 150 ns. No optimization
  needed for the current usage pattern.

---

### Group 3: `human::textwrap::text_wrap` — Word Wrapping

`text_wrap` delegates to the `textwrap` crate, which allocates a `Vec<Cow<str>>`
per call and performs a full word-splitting pass. Called for help text rendering and
posture report body paragraphs.

| Benchmark | Mean | Low (95% CI) | High (95% CI) | Outliers |
|---|---|---|---|---|
| `text_wrap/short no-wrap (80 col)` | 44.22 ns | 43.84 ns | 44.65 ns | 12 (12%) |
| `text_wrap/long wrap (80 col, 4-pad)` | 5.46 µs | 5.45 µs | 5.47 µs | 5 (5%) |
| `text_wrap/long wrap (40 col, 4-pad)` | 6.19 µs | 6.15 µs | 6.26 µs | 3 (3%) |
| `text_wrap/long wrap with right-pad (80 col, 4+4)` | 5.92 µs | 5.89 µs | 5.98 µs | 5 (5%) |

**Analysis:**

- **Short-string fast path:** When the input already fits in the target width, `textwrap`
  avoids splitting and returns a single `Cow::Borrowed`. Cost is ~44 ns (dominated by
  a single Vec allocation).
- **Long-string cost:** A 400-character paragraph costs ~5.5 µs at 80 columns. At 40
  columns (more wraps needed), cost rises to ~6.2 µs — a ~13% increase for roughly
  double the output lines. The cost is roughly linear in output line count.
- **Right-padding overhead:** ~460 ns additional cost vs the same wrap without padding
  (5.92 µs vs 5.46 µs). The padding loop iterates over `N` wrapped lines and performs
  a `chars().count()` per line — a scan through the UTF-8 string that allocates no
  additional heap.
- **`text_wrap` is the most expensive function in umrs-core** by a factor of ~100×
  over `metricfmt`. It should not be called in animation-rate loops. Called once at
  render time (e.g., when a help pane opens), the cost is imperceptible.

---

### Group 4: `validate::is_valid` — Regex Cache Lookup

The regex cache uses `OnceLock<Mutex<HashMap<UmrsPattern, Regex>>>`. Benchmarks
below exercise the warm-cache path only (all patterns pre-compiled at harness startup).
The Mutex `lock()` + `HashMap::get()` + `Regex::is_match()` all occur on every call.

| Benchmark | Mean | Low (95% CI) | High (95% CI) | Outliers |
|---|---|---|---|---|
| `is_valid/Email valid` | 2.65 µs | 2.63 µs | 2.68 µs | 2 (2%) |
| `is_valid/Email invalid` | 1.72 µs | 1.71 µs | 1.74 µs | 5 (5%) |
| `is_valid/RgbHex valid` | 2.39 µs | 2.38 µs | 2.41 µs | 4 (4%) |
| `is_valid/RgbHex invalid (no hash)` | 93.61 ns | 93.14 ns | 94.08 ns | 0 |
| `is_valid/SafeString valid` | 1.72 µs | 1.72 µs | 1.72 µs | 4 (4%) |
| `is_valid/SafeString invalid (control char)` | 1.55 µs | 1.54 µs | 1.55 µs | 0 |

**Analysis:**

- **Dominant cost: Mutex contention, not regex execution.** The `RgbHex invalid`
  case (93 ns) reveals this: the pattern `^#([A-Fa-f0-9]{6})$` immediately fails
  on a string with no `#` prefix — the regex engine exits at byte 0. Yet the other
  invalid cases cost 1.5–1.7 µs. The difference (~1.4–1.6 µs) is the Mutex lock
  overhead. The `RgbHex invalid` case likely benefits from an OS-level fast-path
  when the Mutex is uncontended and the regex exits immediately after cache lookup.
- **Email regex cost:** The Email pattern matches the full string when valid (~2.65 µs
  total; subtract ~1.6 µs Mutex cost = ~1.0 µs for the regex itself). This is expected
  for a non-trivial anchored `^...$` pattern on an 18-character string.
- **Architectural note:** The `Mutex` around the regex HashMap is a correctness choice
  (thread-safe lazy compilation) but becomes a contention point under concurrent
  validation. If `is_valid` is called from multiple threads simultaneously (e.g., in a
  parallel posture pipeline), this Mutex is a serialization bottleneck. Consider
  migrating to `OnceLock<Regex>` per pattern (eliminates the HashMap and reduces to
  a single atomic load on the warm path) if this becomes a measured bottleneck.

---

### Group 5: `timed_result::Timed::measure` — Timing Wrapper Overhead

Measures the wall-clock cost of `Instant::now()` + `start.elapsed()` — the overhead
introduced by wrapping any computation in `Timed::measure`.

| Benchmark | Mean | Low (95% CI) | High (95% CI) | Outliers |
|---|---|---|---|---|
| `Timed::measure (trivial add)` | 35.27 ns | 35.19 ns | 35.35 ns | 0 |
| `Timed::measure (format! string)` | 48.75 ns | 48.67 ns | 48.83 ns | 6 (6%) |

**Analysis:**

- **Timing floor: ~35 ns per `Timed::measure` call** on this ARM64 platform. This is
  the cost of a single `clock_gettime(CLOCK_MONOTONIC, ...)` syscall pair (once at
  `Instant::now()`, once at `start.elapsed()`).
- **format! overhead:** The `format! string` case costs ~13.5 ns more than the trivial
  add, matching the `format_in_prefix/micro` baseline (~64 ns minus the trivial inner
  cost ~50 ns). The Timed wrapper itself contributes no additional allocation beyond
  the `Duration` struct (8 bytes, stack-allocated).
- **Accumulation:** 10 pipeline phases × 35 ns/phase = ~350 ns total timing overhead.
  This is negligible compared to the actual work of any detection phase (µs to ms range)
  and does not warrant optimization.

---

## Summary Table

| Function | Mean | Notes |
|---|---|---|
| `timed_result::Timed::measure` | 35 ns | `Instant` overhead floor; negligible |
| `textwrap::text_wrap (short)` | 44 ns | Fast path; no wrapping occurs |
| `metricfmt::format_in_prefix` | 65 ns | `format!` + static label |
| `metricfmt::auto_format (base)` | 69 ns | Short prefix walk |
| `validate::is_valid/RgbHex invalid` | 94 ns | Regex exits at byte 0; Mutex fast path |
| `sizefmt::format_in_unit/MiB` | 60 ns | Manual division + `format!` |
| `sizefmt::auto_format/binary` | 64 ns | `humansize` BINARY path |
| `metricfmt::auto_format/nano` | 119 ns | Full prefix walk |
| `metricfmt::auto_format/kilo` | 118 ns | Full prefix walk |
| `sizefmt::auto_format/decimal` | 108 ns | `humansize` DECIMAL path (~43 ns slower than BINARY) |
| `validate::is_valid/SafeString invalid` | 1.55 µs | Mutex + early-exit regex |
| `validate::is_valid/Email invalid` | 1.72 µs | Mutex + partial-match regex |
| `validate::is_valid/SafeString valid` | 1.72 µs | Mutex + full-string regex |
| `validate::is_valid/RgbHex valid` | 2.39 µs | Mutex + anchored regex |
| `validate::is_valid/Email valid` | 2.65 µs | Mutex + full-string regex (longest) |
| `textwrap::text_wrap (long, 80 col)` | 5.46 µs | Most expensive: word splitting + heap allocs |
| `textwrap::text_wrap (long, 40 col)` | 6.19 µs | More wraps; linear cost increase |

---

## Key Findings

1. **`text_wrap` is the most expensive function** — 5–6 µs for a paragraph. This is
   expected for a general-purpose word-wrapper, and the usage pattern (once at render
   time) does not require optimization.

2. **`validate::is_valid` is bounded by Mutex overhead**, not regex complexity —
   ~1.5–2.6 µs per call. If validation is called in a tight concurrent loop, migrate
   to `OnceLock<Regex>` per pattern to eliminate HashMap lock contention.

3. **`metricfmt` and `sizefmt` are fast** — 60–120 ns. The dominant cost is `format!`
   heap allocation, not the arithmetic. These functions are safe to call frequently.

4. **`Timed::measure` overhead is 35 ns** — negligible relative to any real detection
   phase. No optimization needed.

5. **`sizefmt::auto_format` decimal/binary disparity** — DECIMAL is ~43 ns slower than
   BINARY on this platform. Investigate `humansize` internals if decimal formatting
   becomes a bottleneck (unlikely at current call rates).
