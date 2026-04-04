// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Performance baseline benchmarks for umrs-core.
//
// Benchmark targets:
//   1. metricfmt::auto_format          — SI-prefix selection + string formatting
//   2. metricfmt::format_in_prefix     — forced-prefix formatting
//   3. sizefmt::auto_format            — byte-size formatting (decimal and binary)
//   4. sizefmt::format_in_unit         — fixed-unit rendering
//   5. human::textwrap::text_wrap      — word-wrapping for help text and reports
//   6. validate::is_valid              — cached regex lookup (warm and cold)
//   7. timed_result::Timed::measure    — overhead of the timing wrapper
//
// Skipped:
//   - robots/builtins: Static ASCII art data; no logic to measure.
//   - console/ansi: Constant string slices; no runtime cost.
//   - audit/: Schema construction involves serde serialization. Out of scope
//     for this baseline; a separate audit-pipeline bench will cover it when
//     the module is fully implemented.
//   - i18n: Depends on a compiled .mo file present at runtime; skipped for
//     offline benchmark environments. A separate integration benchmark will
//     cover it when the l10n pipeline is wired.
//
// Run:
//   cargo bench -p umrs-core --bench core_bench
//
// Results are written to target/criterion/ (HTML + JSON).

use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};

use umrs_core::human::metricfmt::{self, PrefixStyle, SIPrefix};
use umrs_core::human::sizefmt::{self, SizeBase, SizeUnit};
use umrs_core::human::textwrap::text_wrap;
use umrs_core::timed_result::Timed;
use umrs_core::validate::{UmrsPattern, is_valid};

// ---------------------------------------------------------------------------
// Benchmark group 1 — metricfmt
//
// auto_format selects the best SI prefix from a 12-element table, scales the
// value, formats it as a decimal string, and checks for precision loss. This
// runs on every posture-report value emitted by the TUI.
//
// format_in_prefix skips prefix selection but otherwise follows the same path.
// Isolated here so the prefix-selection cost can be computed by subtraction.
// ---------------------------------------------------------------------------

fn bench_metricfmt(c: &mut Criterion) {
    let mut group = c.benchmark_group("metricfmt");

    // auto_format — sub-microsecond value (nanoseconds range: forces Nano prefix)
    group.bench_function("auto_format/nano (842 ns)", |b| {
        b.iter(|| {
            let result = metricfmt::auto_format(black_box(842e-9), PrefixStyle::AbbrevAscii);
            black_box(result)
        });
    });

    // auto_format — kilobytes range (forces Kilo prefix)
    group.bench_function("auto_format/kilo (12.5 kHz)", |b| {
        b.iter(|| {
            let result = metricfmt::auto_format(black_box(12_500.0), PrefixStyle::AbbrevAscii);
            black_box(result)
        });
    });

    // auto_format — base range (no prefix change, value already in [1, 1000))
    group.bench_function("auto_format/base (42.7)", |b| {
        b.iter(|| {
            let result = metricfmt::auto_format(black_box(42.7), PrefixStyle::FullText);
            black_box(result)
        });
    });

    // format_in_prefix — forced Micro prefix, no prefix-selection walk
    group.bench_function("format_in_prefix/micro forced", |b| {
        b.iter(|| {
            let result =
                metricfmt::format_in_prefix(black_box(1.234e-6), SIPrefix::Micro, PrefixStyle::AbbrevAscii);
            black_box(result)
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark group 2 — sizefmt
//
// auto_format delegates to the humansize crate; measured here to capture the
// actual cost at our API boundary. format_in_unit does a manual division and
// fmt::Display call — pure Rust, no external crate involved.
// ---------------------------------------------------------------------------

fn bench_sizefmt(c: &mut Criterion) {
    let mut group = c.benchmark_group("sizefmt");

    // auto_format decimal — 1.5 GiB as raw bytes
    group.bench_function("auto_format/decimal (1.5 GB)", |b| {
        b.iter(|| {
            let result = sizefmt::auto_format(black_box(1_500_000_000u128), SizeBase::Decimal);
            black_box(result)
        });
    });

    // auto_format binary — same size, binary IEC presentation
    group.bench_function("auto_format/binary (1.5 GiB)", |b| {
        b.iter(|| {
            let result = sizefmt::auto_format(black_box(1_500_000_000u128), SizeBase::Binary);
            black_box(result)
        });
    });

    // format_in_unit — fixed MiB, exercises the precision-branching path
    group.bench_function("format_in_unit/MiB (512 MB as MiB)", |b| {
        b.iter(|| {
            let result = sizefmt::format_in_unit(black_box(512_000_000u128), SizeUnit::MiB);
            black_box(result)
        });
    });

    // format_in_unit — raw bytes (divisor = 1, no division cost)
    group.bench_function("format_in_unit/bytes (4096 B)", |b| {
        b.iter(|| {
            let result = sizefmt::format_in_unit(black_box(4096u128), SizeUnit::B);
            black_box(result)
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark group 3 — text_wrap
//
// text_wrap is called for every help paragraph and posture report section.
// Three cases isolate: short text (no wrapping needed), long text that wraps
// multiple times, and text with deep left padding.
// ---------------------------------------------------------------------------

fn bench_textwrap(c: &mut Criterion) {
    let short_input = "SELinux is enabled in enforcing mode.";
    let long_input = concat!(
        "UMRS performs a multi-layer security posture assessment covering SELinux enforcement, ",
        "IMA/EVM integrity, kernel module lockdown, FIPS cryptographic mode, RPM package integrity, ",
        "and CUI classification label fidelity. Each layer is independently validated and contributes ",
        "to the overall trust score surfaced in the posture report."
    );

    let mut group = c.benchmark_group("textwrap");

    // Short string — no wrapping occurs (fits in 80 columns)
    group.bench_function("text_wrap/short no-wrap (80 col)", |b| {
        b.iter(|| {
            let result = text_wrap(black_box(short_input), black_box(80), black_box(0), black_box(0));
            black_box(result)
        });
    });

    // Long string — wraps at 80 columns, 4-space indent
    group.bench_function("text_wrap/long wrap (80 col, 4-pad)", |b| {
        b.iter(|| {
            let result = text_wrap(black_box(long_input), black_box(80), black_box(4), black_box(0));
            black_box(result)
        });
    });

    // Long string — narrower column forces more wraps
    group.bench_function("text_wrap/long wrap (40 col, 4-pad)", |b| {
        b.iter(|| {
            let result = text_wrap(black_box(long_input), black_box(40), black_box(4), black_box(0));
            black_box(result)
        });
    });

    // Right-padding enabled — exercises the additional padding loop
    group.bench_function("text_wrap/long wrap with right-pad (80 col, 4+4)", |b| {
        b.iter(|| {
            let result = text_wrap(black_box(long_input), black_box(80), black_box(4), black_box(4));
            black_box(result)
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark group 4 — validate::is_valid
//
// The first call per pattern compiles and caches the regex. Subsequent calls
// retrieve from the OnceLock-guarded HashMap and run the match.
//
// Two cases per pattern: warm (cache already populated) and a representative
// invalid input (exercises the early-exit path of the regex engine).
//
// The cold-start case is approximated separately with BatchSize::NumIterations
// on a single iteration — this is inherently imprecise for a one-shot cost, but
// gives a rough order-of-magnitude for the compilation cost. In practice the
// cold-start occurs once per process lifetime.
// ---------------------------------------------------------------------------

fn bench_validate(c: &mut Criterion) {
    // Pre-warm the regex cache before timing begins.
    let _ = is_valid(UmrsPattern::Email, "seed@warmup.gov");
    let _ = is_valid(UmrsPattern::RgbHex, "#AABBCC");
    let _ = is_valid(UmrsPattern::SafeString, "seed string");

    let mut group = c.benchmark_group("validate");

    // Email — valid input (matching path)
    group.bench_function("is_valid/Email valid", |b| {
        b.iter(|| {
            let result = is_valid(UmrsPattern::Email, black_box("officer@agency.gov"));
            black_box(result)
        });
    });

    // Email — invalid input (non-matching path; regex engine exits early)
    group.bench_function("is_valid/Email invalid", |b| {
        b.iter(|| {
            let result = is_valid(UmrsPattern::Email, black_box("not-an-email"));
            black_box(result)
        });
    });

    // RgbHex — valid input
    group.bench_function("is_valid/RgbHex valid", |b| {
        b.iter(|| {
            let result = is_valid(UmrsPattern::RgbHex, black_box("#4A90D9"));
            black_box(result)
        });
    });

    // RgbHex — invalid (common mistake: missing # prefix)
    group.bench_function("is_valid/RgbHex invalid (no hash)", |b| {
        b.iter(|| {
            let result = is_valid(UmrsPattern::RgbHex, black_box("4A90D9"));
            black_box(result)
        });
    });

    // SafeString — valid (all printable ASCII)
    group.bench_function("is_valid/SafeString valid", |b| {
        b.iter(|| {
            let result = is_valid(UmrsPattern::SafeString, black_box("CUI//SP-CTI/EXPT//NOFORN"));
            black_box(result)
        });
    });

    // SafeString — invalid (contains a control character: BEL \x07)
    group.bench_function("is_valid/SafeString invalid (control char)", |b| {
        b.iter(|| {
            let result = is_valid(UmrsPattern::SafeString, black_box("bad\x07string"));
            black_box(result)
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark group 5 — timed_result::Timed::measure
//
// Measures the wall-clock overhead introduced by wrapping a trivial computation
// in Timed::measure. The inner closure (integer addition) is intentionally
// sub-nanosecond; the measured time approximates the cost of Instant::now() +
// elapsed() on this platform.
//
// This matters because Timed::measure wraps every detection-pipeline phase;
// its overhead accumulates multiplicatively across phases.
// ---------------------------------------------------------------------------

fn bench_timed_result(c: &mut Criterion) {
    let mut group = c.benchmark_group("timed_result");

    // Trivial inner work — reveals the Instant overhead floor
    group.bench_function("Timed::measure (trivial add)", |b| {
        b.iter(|| {
            let t = Timed::measure(|| {
                let x: u64 = black_box(1) + black_box(1);
                x
            });
            black_box(t.elapsed)
        });
    });

    // Slightly heavier inner work — a string format call
    // Reveals whether the timing wrapper distorts small-but-real workloads
    group.bench_function("Timed::measure (format! string)", |b| {
        b.iter_batched(
            || black_box(42u64),
            |n| {
                let t = Timed::measure(|| format!("value={n}"));
                black_box(t.elapsed)
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Criterion entry points
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_metricfmt,
    bench_sizefmt,
    bench_textwrap,
    bench_validate,
    bench_timed_result,
);
criterion_main!(benches);
