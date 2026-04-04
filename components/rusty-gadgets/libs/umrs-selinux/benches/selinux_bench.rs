// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// Performance baseline benchmarks for umrs-selinux.
//
// Benchmark targets:
//   1. SecurityContext::from_str()  — TPI Path B parser (the FromStr leg)
//   2. CategorySet operations       — union, intersection, dominates, contains
//   3. SensitivityLevel::from_str() — ordinal sensitivity parser
//   4. MlsLevel::from_str()         — composite level parser (sensitivity + categories)
//   5. SecureDirent::from_path()    — fd-anchored, TOCTOU-safe directory entry construction
//
// MlsRange is not benchmarked: the range module currently contains only its
// documentation stub (25 lines, no implementation). The benchmark will be
// added when the implementation lands.
//
// Run:
//   cargo bench -p umrs-selinux --bench selinux_bench
//
// Results are written to target/criterion/ (HTML + JSON).

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::path::Path;
use std::str::FromStr;

use umrs_selinux::category::{Category, CategorySet};
use umrs_selinux::context::SecurityContext;
use umrs_selinux::mls::level::MlsLevel;
use umrs_selinux::secure_dirent::SecureDirent;
use umrs_selinux::sensitivity::SensitivityLevel;

// ---------------------------------------------------------------------------
// Benchmark 1 — SecurityContext::from_str()
//
// Uses a full MCS-style context string that exercises all four fields:
// user, role, type, and a level with both a range separator and categories.
// This is the common form on RHEL 10 targeted-policy systems.
//
// "unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023" is not yet
// supported by the crate's category range parser (.cN notation), so we
// use a comma-separated form that the parser does handle.
// ---------------------------------------------------------------------------

fn bench_security_context_parse(c: &mut Criterion) {
    // A realistic MCS context from a process running on RHEL 10.
    // Exercises user, role, type, and a level with multiple categories.
    let input = "unconfined_u:unconfined_r:unconfined_t:s0:c0,c100,c500,c1023";

    c.bench_function("SecurityContext::from_str (full MCS context)", |b| {
        b.iter(|| {
            let ctx = SecurityContext::from_str(black_box(input));
            black_box(ctx)
        });
    });

    // Also benchmark the minimal 3-field form (no level) to isolate the
    // level-parsing overhead by comparison.
    let bare_input = "system_u:system_r:sshd_t";

    c.bench_function("SecurityContext::from_str (no level)", |b| {
        b.iter(|| {
            let ctx = SecurityContext::from_str(black_box(bare_input));
            black_box(ctx)
        });
    });
}

// ---------------------------------------------------------------------------
// Benchmark 2 — CategorySet operations
//
// Two representative sets:
//   set_a: sparse — c0, c10, c50, c100, c500
//   set_b: dense  — c0 through c99 (first 100 categories)
//
// Operations measured individually so callers can identify which
// is the hot path in their use case.
// ---------------------------------------------------------------------------

fn build_sparse_set() -> CategorySet {
    let mut s = CategorySet::new();
    for &id in &[0u16, 10, 50, 100, 500, 900, 1023] {
        s.insert(Category::new(id).expect("valid category id"));
    }
    s
}

fn build_dense_set() -> CategorySet {
    let mut s = CategorySet::new();
    for id in 0u16..100 {
        s.insert(Category::new(id).expect("valid category id"));
    }
    s
}

fn bench_category_set_operations(c: &mut Criterion) {
    let sparse = build_sparse_set();
    let dense = build_dense_set();
    let target_cat = Category::new(50).expect("valid category");

    c.bench_function("CategorySet::union", |b| {
        b.iter(|| {
            let result = black_box(&sparse).union(black_box(&dense));
            black_box(result)
        });
    });

    c.bench_function("CategorySet::intersection", |b| {
        b.iter(|| {
            let result = black_box(&sparse).intersection(black_box(&dense));
            black_box(result)
        });
    });

    c.bench_function("CategorySet::dominates (sparse dominates dense = false)", |b| {
        b.iter(|| {
            let result = black_box(&sparse).dominates(black_box(&dense));
            black_box(result)
        });
    });

    c.bench_function("CategorySet::dominates (dense dominates sparse = true)", |b| {
        b.iter(|| {
            let result = black_box(&dense).dominates(black_box(&sparse));
            black_box(result)
        });
    });

    c.bench_function("CategorySet::contains", |b| {
        b.iter(|| {
            let result = black_box(&sparse).contains(black_box(target_cat));
            black_box(result)
        });
    });
}

// ---------------------------------------------------------------------------
// Benchmark 3 — SensitivityLevel::from_str()
//
// Exercises the validated parser for the hierarchical sensitivity component.
// Benchmarked at both the low end (s0) and mid-range (s7) to confirm the
// parser cost is input-independent (it should be).
// ---------------------------------------------------------------------------

fn bench_sensitivity_level_parse(c: &mut Criterion) {
    c.bench_function("SensitivityLevel::from_str (s0)", |b| {
        b.iter(|| {
            let lvl = SensitivityLevel::from_str(black_box("s0"));
            black_box(lvl)
        });
    });

    c.bench_function("SensitivityLevel::from_str (s7)", |b| {
        b.iter(|| {
            let lvl = SensitivityLevel::from_str(black_box("s7"));
            black_box(lvl)
        });
    });

    // Ordering comparison — pure ordinal on u16; expected to be sub-nanosecond.
    // Confirms the comparison cost is negligible relative to parse cost.
    let s0 = SensitivityLevel::new(0).expect("valid");
    let s3 = SensitivityLevel::new(3).expect("valid");

    c.bench_function("SensitivityLevel ordinal comparison (s0 < s3)", |b| {
        b.iter(|| {
            let result = black_box(s0) < black_box(s3);
            black_box(result)
        });
    });
}

// ---------------------------------------------------------------------------
// Benchmark 4 — MlsLevel::from_str()
//
// Composite parser: sensitivity + comma-separated categories. This is the
// level field parser used by MlsLevel (in mls/level.rs), not the context-
// embedded level parser in context.rs (which is Path B of the TPI gate).
// ---------------------------------------------------------------------------

fn bench_mls_level_parse(c: &mut Criterion) {
    c.bench_function("MlsLevel::from_str (sensitivity only: s0)", |b| {
        b.iter(|| {
            let lvl = MlsLevel::from_str(black_box("s0"));
            black_box(lvl)
        });
    });

    c.bench_function("MlsLevel::from_str (3 categories: s2:c1,c7,c42)", |b| {
        b.iter(|| {
            let lvl = MlsLevel::from_str(black_box("s2:c1,c7,c42"));
            black_box(lvl)
        });
    });

    // Worst-case realistic: full MCS span with many categories
    c.bench_function("MlsLevel::from_str (10 categories: s0:c0,c1,c2,...,c9)", |b| {
        b.iter(|| {
            let lvl = MlsLevel::from_str(black_box("s0:c0,c1,c2,c3,c4,c5,c6,c7,c8,c9"));
            black_box(lvl)
        });
    });
}

// ---------------------------------------------------------------------------
// Benchmark 5 — SecureDirent::from_path()
//
// Exercises the full TOCTOU-safe construction path:
//   - symlink_metadata() call
//   - AbsolutePath and ValidatedFileName validation
//   - File::open() for the fd-anchored read
//   - ioctl_getflags() for inode security flags
//   - fgetxattr() for the security.selinux xattr (TPI gate)
//
// /etc/hostname is used: it exists on all Linux systems, is world-readable,
// and typically carries a valid SELinux label on SELinux-enabled systems.
// On systems where SELinux is disabled, the xattr will be absent and the
// result will still be Ok (the label state will be SelinuxCtxState::Unlabeled).
//
// /proc/self/status is NOT used here: SecureReader routing rules mandate that
// /proc/ paths go through ProcfsText/SysfsText, not direct File::open. The
// SecureDirent construction path opens the file directly, so it must not be
// pointed at /proc/ or /sys/. /etc/hostname is the correct choice.
// ---------------------------------------------------------------------------

fn bench_secure_dirent_construction(c: &mut Criterion) {
    let path = Path::new("/etc/hostname");

    // Guard: skip if the path is inaccessible so the bench does not panic.
    if !path.exists() {
        eprintln!("[bench] /etc/hostname not found — skipping SecureDirent benchmark");
        return;
    }

    c.bench_function("SecureDirent::from_path (/etc/hostname)", |b| {
        b.iter(|| {
            let entry = SecureDirent::from_path(black_box(path));
            black_box(entry)
        });
    });
}

// ---------------------------------------------------------------------------
// Criterion wiring
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_security_context_parse,
    bench_category_set_operations,
    bench_sensitivity_level_parse,
    bench_mls_level_parse,
    bench_secure_dirent_construction,
);

criterion_main!(benches);
