# Performance Optimization Security Posture Review

```
Audit date: 2026-04-03
Depth: in-depth
Scope: 6 performance optimizations across umrs-core, umrs-platform, umrs-selinux;
       debug instrumentation additions in 6 files; new rust_design_rules.md section;
       EvidenceRecord Default impl and ~30 construction sites
```

## Summary Table

| Category | Count |
|---|---|
| ACCURATE | 18 |
| CONCERN | 4 |
| ERROR | 0 |

---

## Findings by File

### `libs/umrs-core/src/validate.rs` — OnceLock Regex Cache

**A-1: Non-bypassability preserved**

The three `OnceLock<Regex>` statics (`RE_EMAIL`, `RE_RGB_HEX`, `RE_SAFE_STRING`) are
module-private. The only public entry point is `is_valid()`, which routes through
`get_regex()`. There is no way for a caller to bypass validation or substitute a
different pattern. The `UmrsPattern::regex()` method returns compile-time literal
strings that cannot be modified after initialization.

The `OnceLock::get_or_init()` closure compiles the pattern exactly once per process
lifetime; subsequent calls perform a single atomic load. Thread safety is provided
by `OnceLock`'s internal `Once` primitive. No `Mutex` is involved on the warm path.

NIST SP 800-53 SI-10 compliance is maintained.

**A-2: Regex patterns unchanged**

The three pattern strings in `UmrsPattern::regex()` are identical to their
pre-optimization values. The `match` in `get_regex()` covers all three enum variants
exhaustively. No pattern can be omitted without a compile error.

**A-3: Debug instrumentation safe**

`is_valid()` logs `kind:?` (enum variant name) and `result` (bool). No input string
content is logged. The `#[cfg(debug_assertions)]` gate ensures this code is compiled
out in release builds entirely, independent of the `log` crate's
`release_max_level_info` feature flag. Double-gated safety.

---

### `libs/umrs-platform/src/evidence.rs` — EvidenceBundle and EvidenceRecord Default

**A-4: AU-10 append-only invariant preserved**

`EvidenceBundle::new()` changed from `Vec::new()` to `Vec::with_capacity(32)`.
The `records` field remains private. The only mutation path is `push()`. There is no
`pop()`, `clear()`, `sort()`, `remove()`, or `&mut Vec` accessor. The append-only
contract (NIST SP 800-53 AU-10) is enforced at the type-system level regardless of
the initial capacity.

**A-5: `const fn` removal is correct**

`EvidenceBundle::new()` can no longer be `const fn` because `Vec::with_capacity()`
is not const-stable. This has no security impact — the function was never called in
a const context (it cannot be: `EvidenceBundle` contains a `Vec` which requires heap
allocation).

**A-6: `EvidenceRecord::Default` is fail-closed**

`parse_ok` defaults to `false` (line 259). This is the correct fail-closed posture:
a forgotten field produces a conservatively negative result. `source_kind` defaults
to `SourceKind::RegularFile` and `path_requested` to `String::new()` — both are
documented as placeholder values that callers must override. The doc comment
explicitly states this contract.

**A-7: Construction sites correctly set `parse_ok`**

Across ~35 `..Default::default()` construction sites in the `detect/` tree:
- Success records explicitly set `parse_ok: true` (~20 sites verified).
- Failure/error records omit `parse_ok`, inheriting `false` from Default — correct
  fail-closed behavior.
- No site was found where a success record omits `parse_ok: true`.

**C-1: `source_kind` and `path_requested` rely on convention, not enforcement**

`source_kind` defaults to `SourceKind::RegularFile` and `path_requested` to
`String::new()`. The doc comment warns callers to override these, but there is no
compile-time enforcement. A construction site that forgets to set `source_kind` will
silently produce a record classified as `RegularFile` regardless of the actual source.

While no current construction site exhibits this bug, the risk is non-zero for future
code. The `rust_design_rules.md` pattern entry (line 228) correctly identifies this
class of field as one that "must not appear in Default at all" — but the current
implementation does include them.

**Severity:** This is a design tension, not a current bug. The `Default` impl exists
to eliminate boilerplate for ~10 `Option` fields; removing `source_kind` and
`path_requested` from it would require every site to spell out all non-Option fields
explicitly. The current approach with doc-level warnings is a pragmatic trade-off.

Recommendation: Add a `#[must_use]` lint or a named constructor
`EvidenceRecord::for_source(kind, path)` that requires the two critical fields and
defaults the rest. This would preserve ergonomics while making the contract
compiler-enforced. Remediation owner: **coder**.

---

### `libs/umrs-platform/src/detect/substrate/rpm_db.rs` — Hex Decode

**A-8: Hex validation rejects all invalid inputs**

`hex_decode()` rejects odd-length input on line 453. The `while` loop uses
`.get(i)` for bounds-safe indexing on every nibble access (lines 461-462).
`hex_nibble()` returns `None` for any byte outside `0-9`, `a-f`, `A-F` —
the match is exhaustive with a catch-all `_ => None`.

**A-9: Multi-byte UTF-8 cannot bypass validation**

The function operates on `hex.as_bytes()`. A multi-byte UTF-8 character (e.g.,
`\xC3\xA9`) would produce individual bytes that each fail the `hex_nibble()` match,
returning `Err(HexDecode)`. The odd-length check also catches any UTF-8 sequence
that produces an odd byte count. There is no path where a non-ASCII byte produces
a valid nibble value.

**A-10: Bounds checks preserved via `.get()` and `checked_add()`**

Line 461: `raw.get(i)` — bounds-checked.
Line 462: `raw.get(i + 1)` — could theoretically panic on `usize::MAX + 1`, but
`i` is bounded by `raw.len()` which is bounded by `str` length (max `isize::MAX`),
so `i + 1` cannot overflow. The `checked_add(2)` on line 466 is belt-and-suspenders
for the cursor advance. Correct.

**A-11: Digest comparison unchanged**

`hex_decode()` is called only from `query_file_digest()` (line 339), which passes
the result to the caller as `(DigestAlgorithm, Vec<u8>)`. The comparison logic in
`integrity_check.rs` (line 398) compares `computed.as_ref() == installed.value.as_slice()`,
which is a constant-time-irrelevant byte comparison (not a cryptographic MAC check).
The byte values produced by the new implementation are identical to the old one for
all valid hex inputs.

---

### `libs/umrs-platform/src/detect/integrity_check.rs` — read_bounded Pre-allocation

**A-12: `take()` size bound still enforces the read limit**

Line 538: `file.take((max_bytes as u64).saturating_add(1)).read_to_end(&mut buf)`.
The `take()` call creates a `Read` adapter that limits total bytes read to
`max_bytes + 1`. The check on line 540 (`bytes_read > max_bytes`) then rejects any
file that exceeded the limit. The `Vec::with_capacity(1024.min(max_bytes))` on line
537 is purely a pre-allocation hint — it does not constrain `read_to_end()`, which
will grow the Vec as needed up to the `take()` limit. The enforced bound is `take()`,
not the capacity.

**A-13: FIPS gate evidence record is correct**

The FIPS gate (lines 478-521) records `parse_ok: true` (line 507) and
`opened_by_fd: true` (line 505) for the `/proc/sys/crypto/fips_enabled` read.
This is correct: the read goes through `ProcfsText` + `SecureReader`, which is
fd-anchored. The evidence record accurately reflects the access method.

---

### `libs/umrs-selinux/src/context.rs` — SecurityContext FromStr splitn

**A-14: TPI cross-check gate intact**

The TPI gate lives in `xattrs.rs` (`SecureXattrReader::read_context()`), not in
`context.rs`. The `FromStr` implementation in `context.rs` is Path B only. The
`read_context()` function (verified in xattrs.rs) always runs both Path A (nom)
and Path B (FromStr), compares the results, and fails closed on disagreement
(`TpiError::Disagreement`). The optimization to Path B's internal implementation
does not affect the TPI architecture — both paths are still always called.

**A-15: Level field with internal colons parses correctly under targeted policy**

`splitn(5, ':')` splits into at most 5 tokens. For a context like
`system_u:system_r:httpd_t:s0:c0,c100`:
- Token 1: `system_u` (user)
- Token 2: `system_r` (role)
- Token 3: `httpd_t` (type)
- Token 4: `s0` (sensitivity)
- Token 5: `c0,c100` (categories — everything after the 4th colon)

This is correct for targeted policy where categories are comma-separated and
contain no colons.

**C-2: splitn(5) AXIOM citation scope is Phase 1 only**

The comment on line 214 correctly cites the selinux.md AXIOM: "Targeted policy has
exactly one sensitivity level, `s0`." It then states: "Phase 1 is targeted policy
only, so the 5th token is always a flat category list with no embedded colon
separators."

This is accurate for Phase 1 (targeted policy). However, MLS policy contexts can
contain MLS ranges with format `low_level-high_level` where each level is
`sensitivity:categories`. Example: `user_u:user_r:user_t:s0:c0-s3:c0.c1023`.
With `splitn(5, ':')`, this would produce:
- Token 4: `s0`
- Token 5: `c0-s3:c0.c1023`

The category parser would then receive `c0-s3:c0.c1023` as the remainder, which
would fail to parse as a comma-separated category list — producing a fail-closed
result (the `unwrap_or_else` on line 263 returns an empty `CategorySet`).

This is safe — it fails closed. But when MLS support is added in Phase 2, the
`splitn(5)` limit will need to be revisited. The existing comment documents this
constraint correctly.

Recommendation: No action needed now. The AXIOM citation and Phase 1 scope comment
are sufficient documentation. When MLS range parsing is implemented, the split
strategy must change to handle the `low-high` range format. Flag this for the
Phase 2 planning cycle. Remediation owner: **coder** (future Phase 2 work).

**A-16: AXIOM citation is accurate**

The comment references "Targeted policy has exactly one sensitivity level, `s0`"
from `selinux.md`. This matches the AXIOM verbatim. The Phase 1 constraint
("[CONSTRAINT] UMRS Phase 1 is targeted policy only") is correctly applied to scope
the splitn optimization.

---

### `libs/umrs-platform/src/detect/mod.rs` — Per-Phase Timing

**A-17: Debug logging does not leak security-relevant values**

The phase timing loop (lines 579-595) logs only:
- `pd.phase.name()` — static `&'static str` from a `const fn` match
- `pd.duration_ns` — numeric timing value
- `pd.record_count` — numeric count

No kernel attribute values, file paths, configuration file contents, security
labels, or context strings appear in any of these log calls. The comment on
lines 575-578 explicitly documents the SI-11 compliance rationale.

**C-3: Phase timing loop not gated behind `#[cfg(debug_assertions)]`**

The per-phase timing summary loop (lines 579-595) and the `total_duration_ns`
computation (lines 587-595) use `log::debug!()` without a `#[cfg(debug_assertions)]`
gate. They rely solely on the `release_max_level_info` feature flag in `Cargo.toml`
to compile these calls to no-ops in release builds.

This is a different pattern from the instrumentation in `validate.rs`, `context.rs`,
`level.rs`, `secure_dirent.rs`, and `textwrap.rs`, which all use
`#[cfg(debug_assertions)]` as the compilation gate.

The `release_max_level_info` feature flag is effective — it causes the `log` crate
to compile `debug!()` calls to no-ops when the maximum release log level is `info`.
All four `log` crate dependencies in the workspace (`umrs-core`, `umrs-platform`,
`umrs-selinux`, `umrs-hw`) consistently set this feature. The compiled binary will
not contain the debug format strings.

However, the two gating mechanisms have different failure modes:
- `#[cfg(debug_assertions)]` is a Rust compiler flag — cannot be changed without
  recompilation.
- `release_max_level_info` is a Cargo feature flag — could theoretically be
  overridden by a downstream crate that enables a higher `release_max_level_*`
  feature.

Since the data logged (static phase names, numeric values) contains no security-
relevant content, this is not a security concern even if the gate were bypassed.
The inconsistency is a style issue, not a vulnerability.

Recommendation: For consistency, consider wrapping the phase timing summary in
`#[cfg(debug_assertions)]` to match the pattern used in all other instrumented
files, or document the rationale for the different gating approach. This is not a
security finding. Remediation owner: **coder** (style consistency).

---

### Debug Instrumentation — All Files

**A-18: No security-relevant value leakage in any instrumented file**

Reviewed all debug instrumentation additions:

| File | What is logged | Sensitive? |
|---|---|---|
| `validate.rs` | Pattern enum variant, bool result, elapsed | No |
| `context.rs` | "[PATH B] No level field" / "Level field present" / elapsed | No |
| `level.rs` | Elapsed time only | No |
| `secure_dirent.rs` | Elapsed, `access_denied` bool, label state enum string | No |
| `textwrap.rs` | Input char count, width, elapsed | No |
| `detect/mod.rs` | Phase name, duration_ns, record_count | No |

No file logs: raw input strings, security context values, MLS level data,
configuration file contents, file paths from user input, or xattr values.

The `context.rs` Path B instrumentation (lines 239-241, 251-252, 273-274) was
carefully reviewed: it logs presence/absence of fields and a generic "resolved"
message, never the actual sensitivity or category values. This complies with
NIST SP 800-53 SI-11 (Error Information Discipline).

---

### `rust_design_rules.md` — Performance-Aware Construction Patterns

**A-19: OnceLock pattern guardrails are correct (line 182-189)**

The pattern correctly constrains applicability: "only safe when T is `Send + Sync`
and the set of keys is bounded and enumerable at compile time." The escape hatch
("Do not apply it to dynamically keyed caches") is explicitly stated.

**A-20: Vec pre-allocation pattern guardrails are correct (lines 191-199)**

The pattern correctly states the pre-allocated size is "a hint, not a hard limit"
and warns against capacity that "could silently truncate security-relevant data."
This matches the implementation in `read_bounded()` and `EvidenceBundle::new()`.

**A-21: Byte-slice indexing pattern guardrails are correct (lines 201-209)**

The pattern contains the critical security guardrail: "any code that takes the
byte-slice fast path must have already validated (or must validate inline) that all
bytes are in the expected ASCII range." The closing sentence — "A fast-path that
skips validation is a vulnerability, not a pattern" — is the correct framing.

**A-22: splitn pattern guardrails are correct (lines 211-219)**

The pattern correctly notes that the last element preserves remaining delimiters,
which is the exact property exploited in the MLS level parsing. The escape hatch
for unknown field counts is stated.

**C-4: Fail-closed Default pattern has internal tension (lines 221-231)**

The pattern states (line 228): "security-critical fields that have no safe default
... must not appear in Default at all." But `EvidenceRecord::Default` does include
`source_kind` (which has no safe default — `RegularFile` is arbitrary) and
`path_requested` (empty string is not a valid path).

This creates a documented tension between the pattern guidance and the actual
implementation. The pattern is correct as aspirational guidance; the implementation
is a pragmatic trade-off documented in the Default impl's doc comment (lines 243-244).

Recommendation: Add a sentence to the pattern acknowledging the pragmatic exception:
"When a type has many Option fields and only one or two non-Optional security-critical
fields, a Default impl with documented sentinel values and a named constructor that
requires the critical fields is an acceptable compromise." This aligns the pattern
with the actual implementation and prevents future reviewers from flagging a false
inconsistency. Remediation owner: **coder** (design rules update).

---

## Remediation Owner Summary

| ID | Title | Owner | Priority |
|---|---|---|---|
| C-1 | `source_kind`/`path_requested` not compiler-enforced in Default | coder | Low |
| C-2 | splitn(5) will need revision for MLS Phase 2 | coder | Low (future) |
| C-3 | Inconsistent debug gating style (cfg vs feature flag) | coder | Low |
| C-4 | Pattern doc tension with EvidenceRecord Default impl | coder | Low |

---

## Strengths Worth Preserving

1. **Fail-closed Default**: The `parse_ok: false` default is a textbook fail-closed
   pattern. Every construction site that represents a successful operation explicitly
   sets `parse_ok: true`. This was verified across all ~35 sites. Excellent.

2. **TPI architecture untouched**: The splitn optimization in Path B does not affect
   the TPI cross-check gate in `xattrs.rs`. Both parsers are still always called,
   and disagreements still fail closed. The optimization is entirely internal to
   Path B's field extraction.

3. **Debug instrumentation discipline**: Across all six instrumented files, no
   security-relevant values are logged. The team consistently logs structural
   metadata (enum variants, booleans, elapsed times, character counts) rather than
   content. The `context.rs` instrumentation is particularly well-done — logging
   field presence without field values.

4. **Bounds safety in hex_decode**: The byte-slice optimization in `rpm_db.rs`
   maintains `.get()` bounds checking on every access and `checked_add()` on the
   cursor advance. The validation contract (reject non-hex bytes) is preserved
   exactly.

5. **AXIOM-scoped optimization**: The `splitn(5)` optimization in `context.rs` is
   explicitly scoped to the Phase 1 targeted-policy constraint with a direct AXIOM
   citation. This is the correct way to document a performance assumption that has
   a bounded validity window.

6. **Design rules as construction requirements**: The new "Performance-Aware
   Construction Patterns" section frames each pattern with explicit security
   guardrails. The byte-slice pattern's closing line ("A fast-path that skips
   validation is a vulnerability, not a pattern") sets the right tone — performance
   is permitted only when validation is preserved.

---

## Gap Analysis Summary

```
Files reviewed: 14
Total findings: 22 (0 ERROR, 4 CONCERN, 18 ACCURATE)
Policy artifacts written: none required
Policy artifacts needed: none
Documentation gaps: C-4 (pattern doc vs implementation tension)
Code-vs-policy inconsistencies: none — all optimizations preserve the MAC/DAC
  posture, fail-closed behavior, non-bypassability, and audit trail integrity
  established by the original implementations.
```
