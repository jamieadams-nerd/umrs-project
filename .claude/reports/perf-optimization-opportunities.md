# Performance Optimization Opportunities

```
Analysis date: 2026-04-03
Scope: umrs-platform, umrs-selinux, umrs-core
Analyst: Rusty (rust-developer agent)
Status: RESEARCH ONLY — no production code modified
Baseline source: perf-baseline-umrs-platform.md, perf-baseline-umrs-selinux.md,
                 perf-baseline-umrs-core.md
```

---

## Summary Table

| Rank | Opportunity | Crate | Expected Gain | Risk | Controls Affected |
|---|---|---|---|---|---|
| 1 | Replace `OnceLock<Mutex<HashMap>>` with per-variant `OnceLock<Regex>` | umrs-core | 50–70% reduction on cached paths (removes Mutex acquire + clone per call) | Low | NIST SP 800-53 SI-10 |
| 2 | Pre-allocate `EvidenceBundle` Vec with known capacity | umrs-platform | ~10–15% overall pipeline reduction; ~8 reallocations eliminated | Very low | NIST SP 800-53 AU-10 |
| 3 | Replace `chars().collect::<Vec<char>>()` with byte-slice indexing in `hex_decode` | umrs-platform | ~15–20% reduction in Phase 6 (30 µs → ~24–26 µs) | Low | NIST SP 800-53 SI-7 |
| 4 | Pre-allocate `read_bounded` buffer with `with_capacity` | umrs-platform | ~5–10% reduction in Phase 6 | Very low | NIST SP 800-53 SI-7 |
| 5 | Eliminate `split(':').collect::<Vec<&str>>()` in `SecurityContext::from_str` | umrs-selinux | ~10–15% reduction on full-MCS paths (190 ns → ~160 ns) | Low | NIST SP 800-53 SI-7 |
| 6 | Batch the two Basenames SQLite queries into one | umrs-platform | ~20–30% reduction in Phase 4+5 combined (but requires schema analysis) | Medium | NIST SP 800-53 CM-8, SA-12 |
| 7 | Inline `EvidenceRecord` construction via a builder or macro | umrs-platform | ~3–5% overall; code quality improvement primarily | Very low | NIST SP 800-53 AU-3 |

---

## Opportunity Details

---

### Rank 1 — `OnceLock<Mutex<HashMap>>` → per-variant `OnceLock<Regex>` (umrs-core)

**Location:** `libs/umrs-core/src/validate.rs` — `REGEX_CACHE`, `get_regex()`, `is_valid()`

**What was found:**

The current design uses a single `OnceLock<Mutex<HashMap<UmrsPattern, Regex>>>`. Every call
to `is_valid()` — even after the regex is fully compiled and cached — must:

1. Call `OnceLock::get_or_init()` to obtain the `Mutex` reference (cheap, but a load).
2. Acquire the `Mutex` lock (atomic operation on the lock word; blocks other threads).
3. Perform a `HashMap::get()` lookup.
4. Clone the `Regex` out of the map (non-trivial — `Regex` contains an `Arc<>` internally,
   which increments a reference count).
5. Release the lock.
6. Run `re.is_match()` on the now-cloned `Regex`.

The baseline measured 1.5–2.7 µs per call. On a system with light thread contention the
overhead is primarily from the clone; under any concurrency the `Mutex` itself adds
non-determinism.

**Proposed alternative:**

Three separate `static` cells — one per `UmrsPattern` variant — each a `OnceLock<Regex>`:

```rust
static RE_EMAIL:      OnceLock<Regex> = OnceLock::new();
static RE_RGBHEX:     OnceLock<Regex> = OnceLock::new();
static RE_SAFESTRING: OnceLock<Regex> = OnceLock::new();
```

`get_regex()` selects the correct cell and calls `get_or_init()` once per variant lifetime.
After initialization, subsequent reads are lock-free: `OnceLock::get()` is a single atomic
load and pointer dereference — no `Mutex`, no `HashMap` lookup, no `Regex` clone.
`Regex::is_match()` is called directly on the shared reference.

**Expected improvement:**

Cached-path cost drops from ~1.5–2.7 µs to ~200–400 ns (dominated by `Regex` DFA
execution). This is a 50–70% wall-clock reduction per call. On hot paths that validate many
strings (e.g., CUI marking construction) the aggregate saving is significant.

**Risk assessment:** Low.

- Functional behavior is identical: same regex pattern, same match semantics.
- `OnceLock<Regex>` is the idiomatic Rust pattern for static singleton initialization and
  is explicitly recommended in the `regex` crate documentation for this use case.
- No security-relevant change: the pattern strings are `const` and authorship-controlled.
- Thread safety is preserved: `OnceLock` provides a once-only initialization guarantee
  backed by an `Once` (compare-and-swap).
- `UmrsPattern::regex()` remains a `const fn` returning `&'static str`; the only change
  is the storage mechanism for the compiled form.

**Controls:** NIST SP 800-53 SI-10 (Information Input Validation) — no change to validation
semantics. The improvement reduces lock contention without altering correctness guarantees.
NSA RTB RAIN: the non-bypassability property is preserved because the validation function
boundary is unchanged.

---

### Rank 2 — Pre-allocate `EvidenceBundle` Vec (umrs-platform)

**Location:** `libs/umrs-platform/src/evidence.rs` — `EvidenceBundle::new()`

**What was found:**

`EvidenceBundle::new()` calls `Vec::new()`, which allocates zero capacity. During a normal
detection run the pipeline pushes approximately 18–25 `EvidenceRecord`s:

- Phase 1 (KernelAnchor): ~4 records
- Phase 2 (MountTopology): ~3 records
- Phase 3 (ReleaseCandidate): ~2 records
- Phase 4 (PkgSubstrate): ~4 records (RPM open + namespace check + stub warning + SELinux
  enforce record)
- Phase 5 (FileOwnership): ~2 records
- Phase 6 (IntegrityCheck): ~2 records
- Phase 7 (ReleaseParse): ~3 records

A `Vec` starting at zero capacity doubles on each reallocation. Over 18–25 pushes the
allocator is called approximately 4–5 times (at capacities 1, 2, 4, 8, 16, 32), each time
copying all existing records to the new backing store.

`EvidenceRecord` is not trivially small — it contains two `Option<String>`, a
`Vec<String>` (notes), and several other optional fields. Each reallocation copies the entire
set of accumulated records.

**Proposed alternative:**

```rust
pub fn new() -> Self {
    Self {
        records: Vec::with_capacity(32),
    }
}
```

32 covers the typical run with headroom. It is one allocation at construction instead of
four at push time. The constant is small (32 × `size_of::<EvidenceRecord>()`) and causes no
measurable heap pressure.

**Expected improvement:**

~10–15% reduction in total allocation cost across the pipeline. The benefit is distributed
— each phase that pushes records benefits from the already-sized backing store. The effect
is most visible in profiling allocator overhead. Wall-clock improvement on a quiet system:
estimated 5–20 µs depending on allocator jitter.

**Risk assessment:** Very low.

- The AU-10 append-only invariant is in `push()`, not in `new()`. Pre-allocating capacity
  does not change the enforcement boundary.
- The `records` field is private; no caller observes the internal capacity.
- `is_empty()` and `len()` remain correct: `Vec::with_capacity(n).len()` is `0`.
- `const fn new()` would need to become a regular `fn` since `Vec::with_capacity` is not
  `const`. This is a minor API change — `const fn` on a constructor that allocates is
  already unusual, and the `Default` impl via `#[derive]` calls `Vec::new()` anyway.

**Controls:** NIST SP 800-53 AU-10 (Non-Repudiation) — unaffected. AU-3 (Audit Record
Content) — unaffected. The change is purely in Vec growth strategy.

---

### Rank 3 — `hex_decode`: `chars().collect::<Vec<char>>()` → byte-slice indexing (umrs-platform)

**Location:** `libs/umrs-platform/src/detect/substrate/rpm_db.rs` — `hex_decode()`

**What was found:**

```rust
fn hex_decode(hex: &str) -> Result<Vec<u8>, RpmDbError> {
    // ...
    let chars: Vec<char> = hex.chars().collect();   // ← heap allocation + char expansion
    let mut i = 0usize;
    while i < chars.len() {
        let hi = *chars.get(i).ok_or(RpmDbError::HexDecode)?;
        let lo = *chars.get(i + 1).ok_or(RpmDbError::HexDecode)?;
        // ...
    }
}
```

The `hex.chars().collect::<Vec<char>>()` call:
1. Allocates a heap `Vec<char>`.
2. Expands each byte of the ASCII hex string into a 4-byte `char` (because `char` is
   UTF-32 on all Rust targets). A 64-byte SHA-256 hex string produces a 256-byte `Vec`.
3. The `chars.get(i)` calls then dereference this expanded buffer.

For a valid hex string (ASCII digits and a-f/A-F only), single-byte UTF-8 is guaranteed.
The `char` expansion is unnecessary — we can index the `str` as bytes directly using
`hex.as_bytes()`. This eliminates the allocation and the UTF-32 expansion entirely.

The `hex_nibble()` function already works on `u8` (it takes `char` only due to this
caller); converting to take `u8` is a one-line change.

**Expected improvement:**

~15–20% reduction in `hex_decode()` runtime. This function is called in Phase 6
(IntegrityCheck, 30 µs) every time a reference digest is compared. On a SHA-256 hex digest
(64 characters), the eliminated allocation is 256 bytes + allocator metadata.

**Risk assessment:** Low.

- The validation logic in `hex_nibble()` is unchanged: all non-hex bytes still return
  `None → Err(HexDecode)`.
- The odd-length check (`!hex.len().is_multiple_of(2)`) operates on byte length, which for
  ASCII hex is identical to character count. This is correct and unchanged.
- `hex.as_bytes()` is a zero-cost view into the existing `str` allocation.
- Bounds checking via `.get()` is preserved on the `&[u8]` slice. No panic path.

**Controls:** NIST SP 800-53 SI-7 (Software Integrity) — the digest comparison result is
unaffected. NIST SP 800-218 SSDF PW.4.1 (checked arithmetic) — the `checked_add(2)` on
the index cursor is preserved.

---

### Rank 4 — `read_bounded`: `Vec::new()` → `Vec::with_capacity(hint)` (umrs-platform)

**Location:** `libs/umrs-platform/src/detect/integrity_check.rs` — `read_bounded()`

**What was found:**

```rust
fn read_bounded(file: &mut File, max_bytes: usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();    // ← starts at zero capacity
    let bytes_read = file.take((max_bytes as u64).saturating_add(1))
                         .read_to_end(&mut buf)?;
    // ...
}
```

`read_to_end` will grow `buf` as it reads. For `os-release` files (typically 200–500 bytes
on RHEL 10), the allocator is called 1–3 times (at capacities 8, 64, and potentially 512).

A modest initial capacity hint eliminates these reallocations. The `os-release` file size
is a known bounded value (validated at parse time; well under 2 KiB on any real system).

**Proposed alternative:**

```rust
fn read_bounded(file: &mut File, max_bytes: usize) -> io::Result<Vec<u8>> {
    // 1 KiB covers all real os-release files with headroom.
    // The +1 in take() ensures we detect size violations.
    let mut buf = Vec::with_capacity(1024.min(max_bytes));
    let bytes_read = file.take((max_bytes as u64).saturating_add(1))
                         .read_to_end(&mut buf)?;
    // ...
}
```

**Expected improvement:**

~5–10% reduction in Phase 6 allocation cost. The absolute saving is small (1–2 allocations
per run, ~200 ns each) but contributes to overall Phase 6 determinism.

**Risk assessment:** Very low.

- `read_to_end` does not use the initial capacity as a read limit; the `take()` bound is
  the enforced limit. The capacity hint is purely a growth hint to the allocator.
- The oversize check (`bytes_read > max_bytes`) is unchanged. Security invariant preserved.

**Controls:** NIST SP 800-53 SI-7 — digest read and comparison logic is unchanged.

---

### Rank 5 — `split(':').collect::<Vec<&str>>()` in `SecurityContext::from_str` (umrs-selinux)

**Location:** `libs/umrs-selinux/src/context.rs` — `FromStr for SecurityContext`

**What was found:**

```rust
let parts: Vec<&str> = s.split(':').collect();
```

This allocates a `Vec<&str>` on the hot path of Path B (TPI). For a full MCS context
(`user:role:type:s0:c0,c1`) the split produces 5 elements and the `Vec` is allocated, used
briefly to validate `parts.len()` and index `parts[0..2]`, then immediately dropped.

The fields could be extracted with `splitn` without any allocation:

```rust
let mut iter = s.splitn(5, ':');
let user_s  = iter.next().ok_or(ContextParseError::InvalidFormat)?;
let role_s  = iter.next().ok_or(ContextParseError::InvalidFormat)?;
let type_s  = iter.next().ok_or(ContextParseError::InvalidFormat)?;
// Remaining (optional): the level field may contain further colons
let level_s = iter.next();  // None for bare contexts
```

This avoids the `Vec<&str>` allocation and the bounds check on `parts.len()`. The level
remainder is also handled correctly without the `parts[3..].join(":")` allocation.

**Expected improvement:**

~10–15% reduction in `SecurityContext::from_str` (Path B) on full MCS contexts. The
baseline for a full MCS parse is ~190 ns total; eliminating one `Vec` allocation and one
`join()` allocation saves roughly 20–30 ns per call. On paths where `from_str` is invoked
in tight loops (e.g., bulk xattr reads in `umrs-ls`), this compounds.

**Risk assessment:** Low.

- TPI contract is unchanged: both Path A (nom) and Path B (FromStr) are always called; the
  cross-check gate is unaffected.
- The `MlsLevel` logic for parsing the level remainder must be preserved: the level field
  can itself contain colons (e.g., `s0:c0,c1`), which is why `splitn(5, ':')` is needed
  rather than `splitn(4, ':')`. The last element from `splitn(5, ':')` would be
  `s0:c0,c1` — correct.
- `ContextParseError::InvalidFormat` is returned on `None` from `iter.next()` at the
  third field, matching the current `parts.len() < 3` check.
- Validation via `SelinuxUser::from_str`, `SelinuxRole::from_str`, `SelinuxType::from_str`
  is unchanged.

**Controls:** NIST SP 800-53 AC-4, SI-7 — TPI parsing semantics unchanged. NSA RTB RAIN:
non-bypassability of the cross-check gate is not affected by the allocation strategy.

---

### Rank 6 — Batch Basenames SQLite queries in `query_file_owner` and `query_file_digest` (umrs-platform)

**Location:** `libs/umrs-platform/src/detect/substrate/rpm_db.rs` — `query_file_owner()`,
`query_file_digest()`

**What was found:**

Both `query_file_owner()` and `query_file_digest()` execute an identical first query:

```sql
SELECT hnum FROM Basenames WHERE key = ?1
```

When Phases 5 (FileOwnership) and 6 (IntegrityCheck) both run — which is the normal full
pipeline path — this Basenames query runs twice for the same `path`. The blob fetch and
header parse also repeat: both functions call:

```sql
SELECT blob FROM Packages WHERE hnum = ?1
```

for the same `hnum`, fetching and parsing the same binary blob twice.

**Potential approach:**

The `RpmDb` struct could expose a method that returns both the ownership record and the
digest entry in a single header parse pass:

```rust
pub fn query_file_info(
    &self,
    path: &Path,
) -> Result<Option<FileInfo>, RpmDbError>
```

where `FileInfo` carries both ownership (`name`, `version`) and digest (`algo`, `bytes`).
The Basenames and Packages queries run once; the header parse runs once.

**Expected improvement:**

Phase 5 (42 µs) and Phase 6 (30 µs) combined = 72 µs. The Basenames query and blob fetch
are the dominant costs in Phase 5. A single-query path could reduce the combined phases to
roughly 55–60 µs — a ~17 µs saving (~24% reduction on the combined phases, ~9% overall).

**Risk assessment:** Medium.

This requires an API change to `RpmDb` that is visible to callers in `file_ownership.rs`
and `integrity_check.rs`. The `PackageProbe` trait interface would also need to change (or
a new method added). The change is non-trivial: it alters the separation between phases,
which are currently designed to be independent and composable.

There is also an audit implication: `EvidenceRecord` for Phase 5 (FileOwnership) and
Phase 6 (IntegrityCheck) currently capture independent evidence records, each with its own
`duration_ns`. Batching the query changes what goes into each phase's evidence record.
This must be discussed with Jamie before implementation — the current independent-phase
design is a deliberate architectural decision documented in `detect/mod.rs`.

**Controls:** NIST SP 800-53 CM-8, SA-12 — provenance evidence per phase must be preserved.
NIST SP 800-53 AU-3 — evidence record granularity may change. Any implementation must
ensure both ownership and integrity evidence records remain accurate and distinct.

---

### Rank 7 — `EvidenceRecord` construction verbosity (umrs-platform)

**Location:** All phases — `pkg_substrate.rs`, `mount_topology.rs`, `kernel_anchor.rs`, etc.

**What was found:**

`EvidenceRecord` struct literal construction appears ~30 times across all phases with all
fields spelled out, many of which are `None`. This produces correct but verbose code.
Example from `pkg_substrate.rs` line 156:

```rust
evidence.push(EvidenceRecord {
    source_kind: SourceKind::PackageDb,
    opened_by_fd: false,
    path_requested: "pkg_substrate/stub-warning".to_owned(),
    path_resolved: None,
    stat: None,
    fs_magic: None,
    sha256: None,
    pkg_digest: None,
    parse_ok: true,
    notes: vec![format!("stub probe ...")],
    duration_ns: None,
});
```

A `Default` impl or a builder (`EvidenceRecord::for_source(kind, path, ok)`) would reduce
each call site by ~7 lines and make the non-None fields stand out. This is primarily a code
quality improvement.

**Expected improvement:**

No runtime improvement — struct literal initialization compiles to the same instructions as
a helper call. The benefit is maintainability: adding a field to `EvidenceRecord` currently
requires updating ~30 call sites.

**Risk assessment:** Very low for a `Default` impl; low for a builder (builder adds a type).

**Controls:** NIST SP 800-53 AU-3 — audit record completeness. A builder must enforce that
`parse_ok` and `source_kind` are always set explicitly (no silent defaulting of
security-relevant fields).

---

## Opportunities Investigated and Rejected

### umrs-platform Phase 4: SQLite connection not re-opened per call

Initial hypothesis: `RpmDb::open()` might be called on each phase invocation.

**Finding:** Incorrect. The `RpmDb` connection is held inside `Mutex<Option<RpmDb>>` on
`RpmProbe`. The `try_open_db()` function in `rpm.rs` checks `guard.is_none()` before opening
and returns `(true, true)` immediately if already open. The `probe_box` returned from Phase 4
is passed by reference to Phases 5 and 6, which use `query_ownership_inner` and
`installed_digest_inner` through the same `Mutex<Option<RpmDb>>`. The connection is opened
exactly once per detection run. No optimization is needed here.

### umrs-platform Phase 4: SQLite `prepare_cached`

`query_file_owner()` and `query_file_digest()` both use `self.conn.prepare_cached()` for
the Basenames query. `rusqlite::Connection::prepare_cached` uses an internal `LruCache`
to avoid re-preparing the same SQL statement. This is already the optimal approach for the
single-connection pattern. No further optimization is possible at this layer without
changing the query structure itself (see Rank 6).

### umrs-platform Phase 7: ReleaseParse TPI

Phase 7 (ReleaseParse, 12 µs) uses Two-Path Independence (nom + `split_once`) for os-release
parsing. This phase is marked **DO NOT OPTIMIZE** in the task brief. The TPI overhead is a
correctness requirement, not a target. Confirmed and respected.

### umrs-selinux `SecureDirent::from_path` syscall reduction

`from_path` issues: 1 `lstat` + 1 `open` + 1 `ioctl(FS_IOC_GETFLAGS)` + 4 `fgetxattr`
calls (POSIX ACL size probe, POSIX ACL read, IMA size probe, IMA read, SELinux size probe,
SELinux read). The TOCTOU safety design explicitly requires the single `File` fd anchor.
Removing any of the `fgetxattr` calls would change the security model (removing IMA or POSIX
ACL detection). The 7.5 µs cost is dominated by kernel round-trips — not allocations — and
cannot be reduced without reducing security coverage. No optimization is appropriate.

### umrs-selinux `MlsLevel::from_str` linear category cost

The `parse_categories()` function in `mls/level.rs` iterates comma-separated tokens and
calls `Category::from_str` per token. This is already optimal for the input format — each
category must be individually validated. There is no SIMD or batch-parse opportunity for
a comma-separated category list with 1–100 categories in the typical case.

The `splitn(2, ':')` in `FromStr for MlsLevel` is already allocation-free (returns an
iterator). No change needed.

### umrs-core `text_wrap` heap allocation profile

`text_wrap` delegates to the `textwrap` crate's `wrap()` function. The heap allocations
(for the `lines` `Vec<Cow<str>>` output and the join) are intrinsic to the interface — the
function returns an owned `String`. The 5–6 µs cost is dominated by the `textwrap` library's
word-break logic on the input content. This is a library boundary; no improvement is
available without replacing the library or changing the API (return an iterator instead of
a `String`, which is a caller-visible change).

---

## `read_hw_timestamp` Overhead Notes

The pipeline calls `umrs_hw::read_hw_timestamp()` 14 times (7 phases × 2 boundary reads).
On x86_64 this is `RDTSCP` + `CPUID` serialization — approximately 25–40 cycles per call,
or ~10–15 ns at 3 GHz. At 14 calls the timing overhead is ~140–210 ns total — approximately
0.1% of the 174 µs hot-path cost. This is below the measurement noise floor and is not an
optimization target.

---

## Compliance Notes

No proposed optimization changes any security boundary, audit guarantee, or TPI contract.
Specifically:

- **Rank 1** (regex cache): same validation patterns, same result semantics.
- **Rank 2** (EvidenceBundle pre-allocation): AU-10 append-only invariant enforced in
  `push()`, not in `new()`. Capacity pre-allocation is invisible to callers.
- **Rank 3** (hex_decode): identical byte-level comparison logic.
- **Rank 4** (read_bounded): identical size bound enforcement.
- **Rank 5** (context::from_str): TPI gate unchanged; only allocation strategy differs.
- **Rank 6** (batched RPM query): requires design review before implementation. Must not
  merge phase evidence records or degrade the audit trail.
- **Rank 7** (EvidenceRecord verbosity): no runtime effect.

Ranks 1–5 can be implemented independently. Rank 6 requires Jamie's approval on the
phase architecture change before any code is written.
