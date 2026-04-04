# Performance Baseline — umrs-selinux

**Date:** 2026-04-03
**Crate:** `umrs-selinux` v0.1.0
**Benchmark harness:** Criterion 0.5 (harness = false)
**Bench file:** `libs/umrs-selinux/benches/selinux_bench.rs`
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
| CPU features | fp, asimd, aes, pmull, sha1, sha2, crc32, atomics, sha3, sha512, asimddp (subset) |
| Memory (total) | 7.4 GiB (~7,777,856 KiB) |
| Rust toolchain | rustc 1.92.0 (Red Hat 1.92.0-1.el10) |
| Criterion version | 0.5 |

**Architecture note:** All Criterion measurements are wall-clock time. The
`CategorySet` bit operations and `SensitivityLevel` comparisons resolve into
single or small sequences of ARM64 NEON/SIMD instructions; times below ~1 ns
represent the measurement overhead floor of Criterion itself.

---

## Benchmark Results

### 1. `SecurityContext::from_str()` — TPI Path B Parser

Exercises the `FromStr` implementation: splits on `:`, validates each field
via the respective newtype constructors (`SelinuxUser`, `SelinuxRole`,
`SelinuxType`), then parses the level field into a `MlsLevel`.

| Variant | Mean | Low (95% CI) | High (95% CI) | Outliers |
|---|---|---|---|---|
| Full MCS context (`s0:c0,c100,c500,c1023`) | 190.27 ns | 189.82 ns | 190.73 ns | 1 high mild (1%) |
| No level (`system_u:system_r:sshd_t`) | 65.30 ns | 65.07 ns | 65.55 ns | none |

**Level-parsing overhead:** ~125 ns additional cost for the MCS level field
(~190 ns minus ~65 ns). This covers sensitivity parsing, 4× category parsing,
4× `CategorySet::insert()`, and the `String` allocation for `raw_level` in
`context::MlsLevel`.

---

### 2. `CategorySet` Bit Operations

The 1024-bit bitmask is stored as `[u64; 16]`. Operations iterate or fold
over 16 words; the compiler vectorizes these loops on aarch64.

Test sets:
- **sparse**: {c0, c10, c50, c100, c500, c900, c1023} — 7 bits set across 10 words
- **dense**: {c0..c99} — 100 bits set, all in the first 2 words

| Operation | Mean | Notes |
|---|---|---|
| `union` (sparse + dense) | 5.43 ns | 16-word bitwise OR |
| `intersection` (sparse & dense) | 5.43 ns | 16-word bitwise AND |
| `dominates` (sparse dom dense = false) | ~0.996 ns | Exits early on word 0 |
| `dominates` (dense dom sparse = true) | ~0.996 ns | Full 16-word scan |
| `contains` (c50 in sparse) | ~0.828 ns | Single word access |

**Key observation:** `dominates()` shows no asymmetry between the false and
true cases (~0.996 ns each). On this input pair, word 0 of the sparse set
does not dominate word 0 of the dense set, so the false case exits on the
first iteration. Yet the timings are identical — suggesting the compiler
unrolled or vectorized the full 16-word scan regardless. This is consistent
with ARM64 NEON autovectorization, where predicated early exit does not always
produce shorter code.

**`contains` outliers:** 19% of measurements are high-severe. This is expected
for a sub-nanosecond benchmark where Criterion's measurement resolution is at
its floor. The reported mean (~0.83 ns) is reliable; individual samples have
poor signal-to-noise at this scale.

---

### 3. `SensitivityLevel::from_str()` — Ordinal Sensitivity Parser

A short parser: checks for `s` prefix, slices the numeric tail, parses `u16`,
validates against `MAX_SENSITIVITY = 15`.

| Variant | Mean | Notes |
|---|---|---|
| `s0` | 1.77 ns | Minimum numeric string |
| `s7` | 1.77 ns | Same cost — confirms input-independence |
| Ordinal comparison (`s0 < s3`) | ~0.699 ns | Direct `u16` comparison |

**Input-independence confirmed:** `s0` and `s7` cost identically at 1.77 ns.
The parse cost is dominated by the `str::parse::<u16>()` call, not the digit
count, because single-digit decimal strings are parsed by the same code path.

---

### 4. `MlsLevel::from_str()` — Composite Level Parser

Parses sensitivity + comma-separated category list. Each category token is
validated through `Category::from_str()` and inserted into a `CategorySet`.
Allocation: `CategorySet` is stack-only (`[u64; 16]`); no heap allocation.

| Variant | Mean | Notes |
|---|---|---|
| Sensitivity only (`s0`) | 53.85 ns | Includes `splitn`, `Option::None` branch |
| 3 categories (`s2:c1,c7,c42`) | 87.53 ns | 3× `Category::from_str` + 3× `insert` |
| 10 categories (`s0:c0..c9`) | 162.30 ns | 10× parse + 10× insert |

**Linear scaling confirmed:** the per-category cost is approximately 10.8 ns
(`(162.30 - 87.53) / (10 - 3) ≈ 10.68 ns`), which is consistent with one
`&str` parse + a single bit-set in a `u64`. The ~54 ns base cost for the
sensitivity-only case reflects `splitn`, `Option` handling, and the cold path
through `CategorySet::default()`.

**Comparison with `SecurityContext::from_str` (no level):** ~65 ns for the
3-field bare context vs. ~54 ns for `MlsLevel` sensitivity-only. The ~11 ns
difference is the cost of parsing three newtype fields (`SelinuxUser`,
`SelinuxRole`, `SelinuxType`) on top of string splitting.

---

### 5. `SecureDirent::from_path()` — TOCTOU-Safe Construction

Exercises the full security-enriched directory entry path:
`symlink_metadata()` → path validation → `File::open()` → `ioctl_getflags()`
→ `fgetxattr()` for `security.selinux` xattr → TPI parse gate
(nom + `FromStr`, both paths, cross-check).

| Variant | Mean | Low (95% CI) | High (95% CI) | Outliers |
|---|---|---|---|---|
| `/etc/hostname` | 7.52 µs | 7.50 µs | 7.55 µs | 8 high (2 mild, 6 severe) |

**Interpretation:** At ~7.5 µs, this is the most expensive operation in the
benchmark suite by two orders of magnitude — expected given that it performs
real I/O (3 syscalls minimum: `fstat`, `open`, `fgetxattr`). The 6 high-severe
outliers (8%) reflect occasional cache misses on the VFS dentry and inode
cache; these are normal for filesystem I/O benchmarks. The tight CI (±25 ns)
confirms the hot-path cost is stable once the cache is warm.

On a system with SELinux enabled, this includes the full TPI gate (two parsers
running on the xattr bytes, cross-check). On a system where the xattr is absent
(SELinux disabled or file unlabeled), the `fgetxattr` syscall returns `ENODATA`
quickly and the label state is `Unlabeled` — the TPI gate is not reached in
that case.

---

## Cost Summary and Analysis

| Benchmark | Mean | Relative cost |
|---|---|---|
| `SensitivityLevel` ordinal comparison | ~0.70 ns | 1× (baseline) |
| `CategorySet::contains` | ~0.83 ns | 1.2× |
| `CategorySet::dominates` | ~1.00 ns | 1.4× |
| `CategorySet::union` / `intersection` | ~5.43 ns | 7.8× |
| `SensitivityLevel::from_str` | ~1.77 ns | 2.5× |
| `MlsLevel::from_str` (s0) | ~53.85 ns | 77× |
| `SecurityContext::from_str` (no level) | ~65.30 ns | 93× |
| `MlsLevel::from_str` (3 cats) | ~87.53 ns | 125× |
| `MlsLevel::from_str` (10 cats) | ~162.30 ns | 232× |
| `SecurityContext::from_str` (full MCS) | ~190.27 ns | 272× |
| `SecureDirent::from_path` | ~7,522 ns | 10,746× |

**Most expensive operation:** `SecureDirent::from_path()` at ~7.5 µs. This is
correct for a path that performs real syscalls; it cannot be meaningfully
optimized without changing the syscall budget. For bulk directory scans, the
caller should parallelize across inodes, not try to reduce per-entry cost.

**Hot-path operations are fast:** `dominates()` at ~1 ns and `contains()` at
~0.83 ns are well within the budget for access control decisions made on every
file access. A hypothetical system making 10,000 label comparisons per second
spends ~10 µs on dominance checks — negligible.

**Parser cost is linear in categories:** ~10.8 ns per category in
`MlsLevel::from_str`. For the worst-case RHEL 10 MCS context (1024 categories
fully set), parsing would cost approximately `54 + 1024 × 10.8 ≈ 11.1 µs` —
still comfortably sub-millisecond, but worth noting for bulk context parsing.

**`SecurityContext::from_str` vs. TPI gate:** This benchmark measures Path B
only. The full TPI gate in `SecureXattrReader::read_context()` runs both Path A
(nom) and Path B and cross-checks them — roughly 2× the parse cost plus the
cross-check overhead. A future benchmark should measure the full TPI gate via
the xattr reader directly.

---

## Notes on Missing Benchmarks

**`MlsRange` dominance:** `mls/range.rs` currently contains only its
module-level documentation (25 lines, no implementation). The benchmark
entry will be added when the implementation lands.

**Full TPI gate via `SecureXattrReader`:** The xattr reader requires a live
file descriptor with a `security.selinux` xattr. This is only meaningful on a
system with SELinux enabled and a labeled filesystem. A dedicated integration
benchmark should be added once the test infrastructure supports fd fixtures.

---

## Pre-existing Workspace Warning

Two `unused_variables` warnings in `libs/umrs-platform/src/posture/snapshot.rs`
(`readable`, `hardened`) were surfaced by `cargo bench` (which uses the bench
profile). These are pre-existing and were already present before this work —
`cargo xtask clippy` (which uses `-D warnings`) was clean before and after.
These are tracked as separate findings and are not introduced by this benchmark
work.
