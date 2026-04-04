// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// Performance baseline benchmarks for umrs-platform.
//
// Benchmarks the OsDetector::detect() end-to-end pipeline on a live system
// and reports per-phase timing extracted from DetectionResult::phase_durations.
//
// Run with:
//   cargo bench -p umrs-platform --bench platform_bench
//
// Results are written to target/criterion/platform_bench/.
// HTML reports are available at target/criterion/platform_bench/report/index.html.
//
// Note: this benchmark requires a real RHEL/Fedora system with /proc, /sys,
// and an RPM database. It reads live kernel interfaces — no mocking is needed
// or desirable for a pipeline that is itself a provenance verification system.

use criterion::{criterion_group, criterion_main, Criterion};
use umrs_platform::detect::{DetectionPhase, OsDetector};

/// Benchmark the full `OsDetector::detect()` pipeline end-to-end.
///
/// This measures the wall-clock cost of all seven phases running in sequence
/// on a live system. The result includes:
/// - Phase 1: Kernel Anchor (procfs magic, PID coherence, boot_id, lockdown)
/// - Phase 2: Mount Topology (namespace IDs, mountinfo, statfs)
/// - Phase 3: Release Candidate (os-release path probe, statx, symlink)
/// - Phase 4: Package Substrate (RPM/dpkg probe, SELinux enforce pre-check)
/// - Phase 5: File Ownership (package ownership query)
/// - Phase 6: Integrity Check (SHA-256 vs package DB digest)
/// - Phase 7: Release Parse (TPI nom + split_once, substrate corroboration)
fn bench_detect_full_pipeline(c: &mut Criterion) {
    let detector = OsDetector::default();

    c.bench_function("detect_full_pipeline", |b| {
        b.iter(|| {
            // Errors from hard-gate failures (ProcfsNotReal, PidCoherenceFailed)
            // would indicate a broken test environment — we surface them as panics
            // rather than silently swallowing them.
            detector
                .detect()
                .expect("detect() failed — is this a real Linux system with /proc?")
        });
    });
}

/// Benchmark per-phase timing extracted from `DetectionResult::phase_durations`.
///
/// Runs detect() once to obtain a result, then extracts each phase's
/// `duration_ns` (CPU cycles on x86_64, nanoseconds on other arches) and
/// reports them as custom Criterion measurements via `bench_with_input`.
///
/// This provides a detailed breakdown of where pipeline time is spent
/// without needing to instrument the pipeline internals from the bench side.
fn bench_detect_phase_durations(c: &mut Criterion) {
    let detector = OsDetector::default();

    // Run detect once to obtain the phase duration breakdown. Fail loudly
    // on hard-gate errors — those indicate a broken test environment, not
    // a benchmark concern.
    let result = detector
        .detect()
        .expect("detect() failed — is this a real Linux system with /proc?");

    // Map DetectionPhase to a human-readable label for Criterion's output.
    let phase_label = |phase: DetectionPhase| -> &'static str {
        match phase {
            DetectionPhase::KernelAnchor => "phase1_kernel_anchor",
            DetectionPhase::MountTopology => "phase2_mount_topology",
            DetectionPhase::ReleaseCandidate => "phase3_release_candidate",
            DetectionPhase::PkgSubstrate => "phase4_pkg_substrate",
            DetectionPhase::FileOwnership => "phase5_file_ownership",
            DetectionPhase::IntegrityCheck => "phase6_integrity_check",
            DetectionPhase::ReleaseParse => "phase7_release_parse",
        }
    };

    // Report each phase duration as a named custom metric.
    // These are single-sample observations, not repeated benchmarks — their
    // purpose is to record the breakdown for the baseline report, not to
    // produce statistically stable distributions for each individual phase.
    let mut group = c.benchmark_group("detect_phase_durations");

    for pd in &result.phase_durations {
        let label = phase_label(pd.phase);
        let duration_ns = pd.duration_ns;
        let record_count = pd.record_count;

        // Use bench_function with a trivial closure so Criterion records the
        // benchmark name and timing infrastructure. We print the actual observed
        // phase duration as a custom metric using eprintln — Criterion's custom
        // measurement API requires a measurement type impl that is heavyweight
        // for a one-shot reporting use case like this. The observed values from
        // the initial detect() call are the meaningful baseline numbers.
        group.bench_function(label, |b| {
            b.iter(|| {
                // Return a tuple so the compiler doesn't optimize away the values.
                (duration_ns, record_count)
            });
        });

        // Print the actual observed phase duration from the live pipeline run.
        // This is the baseline-meaningful number — the criterion iter() above
        // measures the trivial tuple return, not the pipeline itself.
        eprintln!(
            "[phase-duration] {label}: {duration_ns} cycles/ns, {record_count} evidence records"
        );
    }

    group.finish();

    // Print pipeline-level summary for the baseline report.
    let total_ns: u64 = result
        .phase_durations
        .iter()
        .map(|pd| pd.duration_ns)
        .fold(0u64, |acc, v| acc.saturating_add(v));

    let total_records: usize = result
        .phase_durations
        .iter()
        .map(|pd| pd.record_count)
        .fold(0usize, |acc, v| acc.saturating_add(v));

    eprintln!(
        "[phase-duration] total pipeline: {total_ns} cycles/ns across {total_records} evidence records"
    );
    eprintln!(
        "[detect] trust_level: {:?}  label_trust: {:?}",
        result.confidence.level(),
        result.label_trust,
    );
}

criterion_group!(benches, bench_detect_full_pipeline, bench_detect_phase_durations);
criterion_main!(benches);
