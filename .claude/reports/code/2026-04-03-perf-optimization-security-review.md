# Security Review: Performance Optimizations

Audit date: 2026-04-03
Depth: in-depth
Scope: 6 optimizations across 3 crates — `umrs-core`, `umrs-platform`, `umrs-selinux`

Files examined:

- `libs/umrs-core/src/validate.rs`
- `libs/umrs-platform/src/evidence.rs`
- `libs/umrs-platform/src/detect/substrate/rpm_db.rs`
- `libs/umrs-platform/src/detect/integrity_check.rs`
- `libs/umrs-selinux/src/context.rs`
- `libs/umrs-selinux/src/xattrs.rs` (TPI gate verification)
- `libs/umrs-platform/src/detect/release_candidate.rs` (Default audit)
- `libs/umrs-platform/src/detect/release_parse.rs` (Default audit)
- `libs/umrs-platform/src/detect/file_ownership.rs` (Default audit)
- `libs/umrs-platform/src/detect/kernel_anchor.rs` (Default audit)
- `libs/umrs-platform/src/detect/pkg_substrate.rs` (Default audit)
- `libs/umrs-platform/src/detect/mount_topology.rs` (Default audit)
- `libs/umrs-platform/src/detect/substrate/rpm.rs` (Default audit)
- `libs/umrs-platform/src/detect/substrate/dpkg.rs` (Default audit)

---

## Summary Table

| Category | Count |
|---|---|
| ACCURATE | 14 |
| CONCERN | 2 |
| ERROR | 0 |

---

## Findings by File

---

### `libs/umrs-platform/src/detect/integrity_check.rs`

**C-1 — FIPS gate evidence record uses misleading `parse_ok: false` default**

File: `libs/umrs-platform/src/detect/integrity_check.rs`
Location: lines 503–510 (`fips_mode_active()`)

The `EvidenceRecord` pushed when FIPS mode is confirmed active omits `parse_ok`, so it defaults to `false`. The notes correctly state "FIPS mode active: sha2 is not FIPS 140-3 validated; T4 not earned," but the procfs read of `/proc/sys/crypto/fips_enabled` succeeded and the content was parsed as `"1"`. `parse_ok: false` on this record will mislead an audit reviewer into thinking the read itself failed, rather than that T4 was blocked by policy.

This does not affect enforcement — the FIPS gate correctly returns `true` and T4 is blocked regardless. The issue is audit record fidelity under NIST SP 800-53 AU-3.

Severity: CONCERN

Recommendation: Add `parse_ok: true` to the FIPS gate evidence record. The note text already accurately explains why T4 was not earned; `parse_ok` should reflect whether the I/O and parse operation itself succeeded.

```rust
evidence.push(EvidenceRecord {
    source_kind: SourceKind::Procfs,
    opened_by_fd: true,
    path_requested: FIPS_PATH.to_owned(),
    parse_ok: true,  // read succeeded; T4 blocked by policy, not by I/O failure
    notes: vec![
        "FIPS mode active: sha2 is not FIPS 140-3 validated; T4 not earned".to_owned(),
    ],
    ..Default::default()
});
```

Remediation owner: coder

---

### `libs/umrs-selinux/src/context.rs`

**C-2 — `splitn(5, ':')` comment hedges with "in practice" where a guarantee is required**

File: `libs/umrs-selinux/src/context.rs`
Location: lines 210–213 (Path B inline comment)

The comment reads:

> "category remainder that may itself contain commas but no colons in practice"

The phrase "in practice" is an informal hedge. The correct invariant is structural: under targeted policy (the only policy UMRS Phase 1 supports), MCS category strings use comma-separated category identifiers with no colons. Category strings do not contain colons. This is a constraint of the targeted-policy MCS format, not merely an observed behaviour.

The hedge matters because the `splitn(5, ':')` approach is correct for targeted-policy contexts but would mis-parse MLS range expressions (e.g., `s0:c0,c1-s3:c0`) if MLS range notation ever appears. The existing AXIOM in `selinux.md` — "Targeted policy has exactly one sensitivity level: s0" — rules out range expressions in Phase 1. The comment should cite this constraint explicitly rather than relying on "in practice," so that any future developer adding MLS range support sees immediately that this path requires revision.

This finding affects documentation quality and forward-safety. It does not affect the current targeted-policy parse path, which is correct as implemented.

Severity: CONCERN

Recommendation: Replace the hedge with a precise statement:

```rust
// `splitn(5, ':')` splits into at most 5 tokens. Under targeted policy
// (UMRS Phase 1 constraint — see selinux.md AXIOM), all MCS labels are at
// s0 and category strings use comma-separated identifiers only; no colons
// appear within a category string. The 5th token therefore captures the
// entire category remainder without further splitting. If MLS range notation
// (e.g., `s0:c0,c1-s3:c0`) is ever needed, this path requires revision.
```

Remediation owner: coder

---

## Accurate Findings

The following items were examined and found to be correct. They are documented here for completeness.

---

**A-1 — Rank 1 (`validate.rs`): OnceLock refactor preserves validation non-bypassability**

The three `static OnceLock<Regex>` cells (`RE_EMAIL`, `RE_RGB_HEX`, `RE_SAFE_STRING`) are private statics. The only public entry point is `is_valid()`, which is decorated with `#[must_use]` and a message. `get_regex()` is private and cannot be called from outside the module. The regex patterns are the same literals as before, embedded as `const fn` return values on `UmrsPattern::regex()`. Warm-path behaviour is now a single atomic load with no Mutex acquisition and no Regex clone. The NSA RTB RAIN non-bypassability claim holds: callers cannot reach the OnceLock cells directly, and ignoring the `is_valid()` return value produces a compiler warning via `#[must_use]`.

---

**A-2 — Rank 2 (`evidence.rs`): AU-10 append-only invariant survives `Vec::with_capacity(32)`**

`EvidenceBundle::records` remains a private field. The only mutation method is `push()`, which delegates to `Vec::push`. There is no `pop()`, `clear()`, `drain()`, `truncate()`, `remove()`, `retain()`, or `sort()` exposed at any visibility level. `Vec::with_capacity(32)` allocates backing storage upfront; it does not alter the empty initial length or provide any new access path. The `const fn` removal is safe: `Vec::with_capacity` cannot be `const` because heap allocation is not const-evaluable, and the `Default` derive on `EvidenceBundle` generates `records: Vec::new()` independently. No regression on the AU-10 invariant.

---

**A-3 — Rank 3 (`rpm_db.rs`): `hex_decode()` byte-slice path correctly rejects all invalid inputs including multi-byte UTF-8**

The security concern about multi-byte UTF-8 sequences is not a real vulnerability here. Rust `str::as_bytes()` returns the raw UTF-8 byte sequence. All valid hex characters (`0`–`9`, `a`–`f`, `A`–`F`) are single-byte ASCII with values 0x30–0x39 and 0x41–0x46 and 0x61–0x66. Any multi-byte UTF-8 continuation byte has the high bit set (value ≥ 0x80), and any multi-byte lead byte has value ≥ 0xC0. The `hex_nibble()` match arm `_ => None` covers every byte value outside the three valid ranges, so multi-byte sequences are rejected. The odd-length guard (`!hex.len().is_multiple_of(2)`) is preserved. The checked arithmetic on cursor advancement (`i.checked_add(2).ok_or(RpmDbError::HexDecode)`) is preserved. Bounds-safe indexing via `.get(i)` and `.get(i + 1)` with `ok_or` is preserved. No regression.

---

**A-4 — Rank 4 (`integrity_check.rs`): `Vec::with_capacity(1024.min(max_bytes))` is not the enforced read limit**

The hard limit is imposed by `file.take((max_bytes as u64).saturating_add(1))`, which is the kernel's `read(2)` boundary. `take()` wraps the `File` in an adapter that stops reading after `max_bytes + 1` bytes regardless of how much buffer capacity was pre-allocated. `read_to_end()` will reallocate the `Vec` as needed up to the `take()` bound — the pre-allocation of `1024.min(max_bytes)` only avoids the first few realloc cycles for small files. The size check `bytes_read > max_bytes` then detects that the sentinel +1 byte was consumed and returns `Err`. The pre-allocation does not become and cannot become the enforced limit. NSA RTB bounded-read invariant holds.

---

**A-5 — Rank 5 (`context.rs`): `splitn(5, ':')` correctly handles the targeted-policy level field**

For targeted-policy contexts (the only supported format in Phase 1), the level field has the form `sN` or `sN:cA,cB,...`. With `splitn(5, ':')` applied to a full context `user:role:type:sN:cA,cB`:

- Token 1: `user`
- Token 2: `role`
- Token 3: `type`
- Token 4 (sensitivity): `sN`
- Token 5 (cats_remainder): `cA,cB`

The code then reconstructs `raw_level` as `format!("{sens_str}:{cats_remainder}")` when `cats_remainder` is non-empty, which produces the canonical `sN:cA,cB` string. This is identical to the previous `parts[3..].join(":")` approach. For a context with no level field (`user:role:type`), token 4 is `None` and the code returns `level: None`. Both cases are handled correctly. The allocation is one `format!()` per `MlsLevel` construction, same as before.

---

**A-6 — Rank 5 (`xattrs.rs`): TPI gate is intact — both paths always called**

The TPI gate in `xattrs.rs` at lines 261–326 has not changed. Both Path A (`parse_context_nom()`) and Path B (`context_str.parse()`) are called unconditionally, in sequence, with results collected before the gate evaluates them. There is no short-circuit evaluation. The comment "do NOT short-circuit. Path B is always attempted" remains present and the code implements it. A disagreement between Path A and Path B still produces `TpiError::Disagreement` and fails closed. The `splitn` change to Path B's implementation does not weaken the gate — the gate's property is that both paths are always called and compared, not that they use any particular internal algorithm.

---

**A-7 — Rank 7 (`evidence.rs`): `Default` impl is correctly fail-closed**

`EvidenceRecord::default()` sets `parse_ok: false`. This is the correct fail-closed default: a record that neglects to explicitly set `parse_ok: true` is conservatively treated as a failed parse rather than a silent success.

---

**A-8 — Rank 7: All `..Default::default()` construction sites for failure records correctly omit `parse_ok` (defaulting to `false`)**

Surveyed 30 construction sites across 8 files. Every failure, rejection, error, and anomaly record correctly omits `parse_ok` or explicitly sets it to `false`. This includes:

- TOCTOU-detected file substitution (`integrity_check.rs` line 244–253) — `parse_ok` defaults `false` ✓
- MD5 algorithm rejection (`integrity_check.rs` line 351–363) — `parse_ok` defaults `false` ✓
- Unknown algorithm rejection (`integrity_check.rs` line 363–376) — `parse_ok` defaults `false` ✓
- No digest in package DB (`integrity_check.rs` line 457–463) — `parse_ok` defaults `false` ✓
- SHA-512 cross-algorithm mismatch (`integrity_check.rs` line 293–308) — `parse_ok` defaults `false` ✓
- Mount topology failures (`mount_topology.rs` lines 120, 164, 182, 238) — `parse_ok` defaults `false` ✓
- Kernel anchor read failures (`kernel_anchor.rs` lines 232, 286, 355) — `parse_ok` defaults `false` ✓
- World-writable candidate rejection (`release_candidate.rs` line 152) — `parse_ok` defaults `false` ✓
- TPI disagreement record (`release_parse.rs` line 166) — `parse_ok` defaults `false` ✓
- os-release open failure (`release_parse.rs` line 240) — `parse_ok` defaults `false` ✓
- File unowned by package (`file_ownership.rs` line 166) — `parse_ok` defaults `false` ✓
- RPM DB not found (`substrate/rpm.rs` line 454) — `parse_ok` defaults `false` ✓
- dpkg DB root absent (`substrate/dpkg.rs` line 79) — `parse_ok` defaults `false` ✓

---

**A-9 — Rank 7: All success-path construction sites explicitly set `parse_ok: true`**

Every record representing a successful I/O operation and parse explicitly sets `parse_ok: true`. No success path was found that relies on the default. Confirmed across:

- `release_candidate.rs` line 209 — candidate accepted ✓
- `release_parse.rs` line 192 — TPI parse succeeded ✓
- `file_ownership.rs` line 157 — ownership confirmed ✓
- `kernel_anchor.rs` lines 158, 255, 313 — procfs reads succeeded ✓
- `pkg_substrate.rs` line 219 — SELinux enforce read succeeded ✓
- `mount_topology.rs` lines 199, 230 — mountinfo and statfs succeeded ✓
- `substrate/rpm.rs` line 214 — RPM DB opened and inventoried ✓
- `substrate/dpkg.rs` line 150 — dpkg DB opened and inventoried ✓
- `substrate/rpm_db.rs` line 490 — explicit via `ok` parameter in `evidence_record()` helper ✓

---

**A-10 — Rank 7: `source_kind` and `path_requested` always explicitly set at all construction sites**

Every `..Default::default()` site for `EvidenceRecord` explicitly provides both `source_kind` and `path_requested`. No site relies on the `SourceKind::RegularFile` / empty-string defaults for these fields. The doc comment's warning that these fields have no meaningful sentinel value and must always be supplied is consistently followed.

---

## Gap Analysis Summary

```
Files reviewed: 14
Total findings: 2 (0 HIGH, 2 CONCERN, 0 ERROR)

Uncited security claims: none

Inconsistencies (code vs. docs): none

Security posture assessment: No optimization weakened any security invariant.
All six optimizations are structurally equivalent to their predecessors for
security purposes. Two CONCERN findings are identified: one audit record
fidelity gap (C-1) and one documentation precision gap (C-2). Neither affects
enforcement, authorization decisions, or control flow. Both are safe to fix
independently without blocking any downstream work.
```
