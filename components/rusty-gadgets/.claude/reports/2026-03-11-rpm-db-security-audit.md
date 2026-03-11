# Security Audit Report — RPM DB Implementation & OS Detection Pipeline

```
Audit date: 2026-03-11
Depth: in-depth
Scope: umrs-platform src/detect/ (all phase modules), src/detect/substrate/
       (rpm.rs, rpm_db.rs, rpm_header.rs, mod.rs), src/evidence.rs,
       src/confidence.rs, tests/rpm_header_tests.rs, tests/rpm_db_tests.rs,
       examples/os_detect.rs, Cargo.toml
```

---

## Findings

---

### `src/detect/substrate/rpm_db.rs`

---

**RPM-01**

```
File: src/detect/substrate/rpm_db.rs
Location: line 398
Finding: `hex_decode` uses `hex.len().is_multiple_of(2)` — `is_multiple_of` is a
  nightly/unstable API added in Rust 1.86 (stabilised under a different flag path).
  The standard idiom for stable Rust is `hex.len() % 2 == 0`. If the build toolchain
  is older than 1.86 this will fail to compile; if it silently compiles under a
  different semantic, the odd-length check may not fire as intended. The FIPS-posture
  requirement that digest comparisons be deterministic and correct is undermined if
  this guard misbehaves.
Severity: MEDIUM
Recommended citation: NIST SP 800-218 SSDF PW.4.1 — secure arithmetic; use
  portable stable-Rust arithmetic (`% 2 == 0`) for security-relevant input gates.
Remediation owner: coder
```

---

**RPM-02**

```
File: src/detect/substrate/rpm_db.rs
Location: lines 75-84 (Display impl for RpmDbError)
Finding: `Self::Sqlite(e) => write!(f, "rpm db sqlite error: {e}")` — the `e`
  in the Sqlite variant is `rusqlite::Error`. The Display of rusqlite errors can
  include the SQL query text, table names, or portions of the path used to open the
  DB. While none of those are security labels or credentials, path disclosure in
  error messages creates a supply-chain information leak (an attacker learning the
  exact DB path from a log is a minor but real information exposure). The SI-12
  claim in the doc comment ("Variant payloads carry only structural or type
  information — never file content, security labels, or user data") is not fully
  accurate because rusqlite's Display is outside the codebase's control.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SI-12 — information management; error strings
  must not expose internal paths. Wrap sqlite errors and emit only a category label
  (e.g., "sqlite error (code N)") rather than forwarding rusqlite's Display verbatim.
Remediation owner: coder
```

---

**RPM-03**

```
File: src/detect/substrate/rpm_db.rs
Location: lines 104-186 (RpmDb::open)
Finding: The structural check at line 153 queries `sqlite_master` which is the
  legacy alias; on SQLite 3.37+ the authoritative table is `sqlite_schema`.
  Both names are accepted for backward compatibility, so this is not currently
  broken. However, the comment says "basic structural check: the `Packages` table
  must exist" — this is the correct intent, and the query itself is correct.
  The more material issue: the check only verifies table existence, not that the
  table has the expected `(hnum INTEGER PRIMARY KEY, blob BLOB)` schema. A file
  that is a valid SQLite database with a `Packages` table but different columns
  will pass this check and then produce opaque errors at query time rather than
  failing at open. This is a trust model gap: the open-time structural gate is
  weaker than documented.
Severity: LOW
Recommended citation: NIST SP 800-53 CM-8 — the component inventory source must
  be validated before it is trusted; schema verification at open time strengthens
  the gate.
Remediation owner: coder
```

---

**RPM-04**

```
File: src/detect/substrate/rpm_db.rs
Location: lines 198-262 (query_file_owner), lines 270-334 (query_file_digest)
Finding: Both methods contain nearly identical logic — locate hnum via
  Basenames, fetch blob, parse header, confirm full path in file list. The
  duplication means any bug fix or security improvement applied to one method
  must be manually replicated in the other. This is not a direct security
  vulnerability, but it is a maintenance risk that increases the probability of
  a divergence creating a security defect. The audit scope rule "code wins over
  docs" makes this a process risk rather than a current defect, but it warrants
  a finding.
Severity: LOW
Recommended citation: NIST SP 800-218 SSDF PW.4 — reduce attack surface through
  code reuse; extract a shared `find_header_for_path` helper to eliminate the
  duplicated traversal.
Remediation owner: coder
```

---

**RPM-05 (POSITIVE)**

```
File: src/detect/substrate/rpm_db.rs
Location: lines 135-138 (RpmDb::open)
Finding: POSITIVE — the database is opened with
  `SQLITE_OPEN_READ_ONLY | SQLITE_OPEN_NO_MUTEX` and the connection is opened
  once and reused for all subsequent queries. This correctly implements the
  TOCTOU and read-only properties documented in the module-level comment.
  All queries use parameterized `?1` placeholders — SQL injection is not possible.
  This is well-executed.
Severity: INFO
```

---

### `src/detect/substrate/rpm_header.rs`

---

**RPM-06**

```
File: src/detect/substrate/rpm_header.rs
Location: lines 320-347 (parse_index_manual — try_into().unwrap_or([0u8; 4]))
Finding: In the manual parse path (Path B), all four `try_into().unwrap_or([0u8; 4])`
  calls silently substitute `[0u8; 4]` (which decodes as u32=0) on slice-to-array
  conversion failure. Because `get()` is called first and returns `Err(TooShort)`
  on an out-of-bounds slice, a conversion failure on a correctly-bounded 4-byte
  slice is structurally impossible — `try_into` on a `&[u8; 4]` never fails. The
  silent fallback therefore never activates and cannot introduce a bug. However,
  the pattern looks like it could silently hide an error, which is a code-clarity
  concern under SSDF PW.4. A reviewer seeing `unwrap_or([0u8; 4])` without the
  context that the slice length is already guaranteed may not understand why this
  is safe. Recommend replacing with `.map_err(|_| RpmHeaderError::TooShort)?` to
  make the invariant explicit and match the error-handling style used everywhere
  else in the file.
Severity: LOW
Recommended citation: NIST SP 800-218 SSDF PW.4.1 — fail-closed; silent fallbacks
  on security-relevant parsing operations should be replaced with explicit errors
  even when the fallback path is provably unreachable.
Remediation owner: coder
```

---

**RPM-07**

```
File: src/detect/substrate/rpm_header.rs
Location: lines 786-789 (extract_file_list — consistency check)
Finding: The array-length consistency check at lines 786-789 compares
  `dirindexes.len() != file_count` and `filedigests.len() != file_count` but
  silently returns an empty file list rather than returning an error. The comment
  says "All per-file arrays must have the same count." A mismatch between array
  lengths indicates a malformed or tampered header blob — this is structurally a
  corruption condition and should return an error rather than silently dropping all
  file entries. A missing file in the returned list means the ownership and digest
  phases will not find the expected file, causing T4 not to be reached without any
  error trace in the evidence record pointing to the root cause.
Severity: MEDIUM
Recommended citation: NSA RTB — fail-closed; NIST SP 800-53 SI-10 — information
  accuracy; a length mismatch on security-relevant arrays is an integrity signal
  that should propagate as an error, not a silent empty result.
Remediation owner: coder
```

---

**RPM-08 (POSITIVE)**

```
File: src/detect/substrate/rpm_header.rs
Location: lines 597-665 (parse_rpm_header)
Finding: POSITIVE — the TPI implementation is correct and complete. Path A (nom)
  and Path B (manual byte-slicing) operate on the same immutable slice, produce
  independent `Vec<IndexEntry>` values, and are compared element-by-element before
  any extraction occurs. The blob size (MAX_BLOB_BYTES = 16 MiB) and index count
  (MAX_INDEX_ENTRIES = 4096) limits are enforced before either parse path runs.
  All arithmetic uses `checked_*` operations. All store access uses `.get()`.
  This is a high-quality TPI implementation.
Severity: INFO
```

---

**RPM-09 (POSITIVE)**

```
File: src/detect/substrate/rpm_header.rs
Location: lines 96-130 (RpmHeaderError enum)
Finding: POSITIVE — the error type is well-designed for SI-12 compliance. All
  variants expose only structural metadata (tag numbers, offsets, lengths) —
  never file content or string data. The Display impl is clean.
Severity: INFO
```

---

### `src/detect/substrate/rpm.rs`

---

**RPM-10**

```
File: src/detect/substrate/rpm.rs
Location: lines 129-143 (probe_inner — path-based existence checks)
Finding: `Path::new(RPM_DB_ROOT).exists()` at line 130, and the
  `RPM_PACKAGES_SQLITE` / `RPM_PACKAGES_BDB` checks at lines 179-180 are
  path-based calls. These are not TOCTOU-safe — the filesystem state can change
  between the `.exists()` call and the later `RpmDb::open()` call. In practice
  the risk is low because these are presence-check calls that decide whether
  to attempt a DB open (not security-critical decisions), and the DB open itself
  uses read-only flags. However, the module-level doc comment claims "NSA RTB
  TOCTOU: ownership queries re-verify (dev, ino)" without noting that the probe
  phase itself uses path-based existence checks. The documentation claim is
  therefore broader than what is implemented. Code wins: the doc should be
  narrowed to say that TOCTOU protection applies to ownership queries, not to
  the probe phase presence checks.
Severity: LOW
Recommended citation: NSA RTB TOCTOU — documentation must accurately reflect the
  scope of the TOCTOU protection; the probe-phase path checks are not TOCTOU-safe.
Remediation owner: tech-writer
```

---

**RPM-11**

```
File: src/detect/substrate/rpm.rs
Location: lines 307-323 (query_ownership_inner — TOCTOU re-verification)
Finding: The TOCTOU re-check uses `nix::sys::stat::stat(path)` — a path-based
  stat call. After `db.query_file_owner(path)` returns `Some((pkg_name, ...))`,
  the code re-stats the path to compare `(st_dev, st_ino)` against the `(dev, ino)`
  pair passed in by the caller. This check is correctly placed and guards against
  substitution between the caller's open and the DB query. However, a determined
  attacker who can manipulate the filesystem could create a hard link with the same
  `(dev, ino)` pointing to a different file, then replace the original before the
  re-stat. This is a theoretically possible double-swap attack. The check
  significantly raises the bar and the design note documents this correctly, so
  this is a LOW-severity observation rather than a defect. The existing check is
  the correct mitigation available without fd-anchored stat on an already-opened fd.
Severity: LOW
Recommended citation: NSA RTB TOCTOU — the re-stat is noted as a best-effort
  mitigation; for full TOCTOU safety, the file must remain open from the initial
  statx through ownership confirmation (fd-anchored). Consider passing the open
  file descriptor from release_candidate through to query_ownership in a future
  revision.
Remediation owner: coder
```

---

**RPM-12**

```
File: src/detect/substrate/rpm.rs
Location: lines 396-411 (is_installed public function)
Finding: `is_installed` opens a new `RpmDb` connection every time it is called,
  independent of any existing `RpmProbe` state. If called multiple times from
  the same execution context (e.g., the example's three-call package query demo)
  it opens and closes three separate SQLite connections. This is not a security
  defect, but it is a resource management concern. More importantly: the function
  creates a fresh `EvidenceBundle` that is immediately discarded — audit evidence
  from `is_installed` calls is silently dropped. This means callers who use
  `is_installed` have no audit trail for those queries, contradicting the AU-3
  compliance claim in its doc comment.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 AU-3 — audit record content; `is_installed`
  discards its evidence bundle, leaving no audit trail for the query. The function
  should accept an `Option<&mut EvidenceBundle>` or document that it is not an
  audited operation.
Remediation owner: coder
```

---

### `src/detect/integrity_check.rs`

---

**RPM-13**

```
File: src/detect/integrity_check.rs
Location: lines 251-276 (SHA-512 reference digest handling)
Finding: When the package DB contains a SHA-512 reference digest but the code
  has computed SHA-256, the code logs a warning, records the computed SHA-256 in
  the evidence record (sha256 field), and returns false. This is correct behavior.
  However, the code does NOT compute SHA-512 to attempt a proper comparison.
  The module comment at lines 27-30 says "Only `DigestAlgorithm::Sha256` and
  `DigestAlgorithm::Sha512` digests are accepted" — this implies both are handled.
  The reality is that SHA-512 is accepted (passes the algorithm check at line 348)
  but then correctly rejected at lines 251-276 as a cross-algorithm mismatch.
  The documentation at lines 27-30 says SHA-512 is "accepted" without clarifying
  that it is accepted for recording purposes only and T4 cannot be reached via
  SHA-512. This is a doc-vs-code inconsistency: "accepted" reads as "will succeed"
  when the correct reading is "will be recorded but cannot earn T4."
Severity: LOW
Recommended citation: NIST SP 800-53 SI-7 — documentation must accurately reflect
  what integrity algorithms can earn T4; docs say SHA-512 is "accepted," code
  blocks T4 on SHA-512 references.
Remediation owner: tech-writer
```

---

**RPM-14 (POSITIVE)**

```
File: src/detect/integrity_check.rs
Location: lines 153-157 (FIPS gate) and lines 467-521 (fips_mode_active)
Finding: POSITIVE — the FIPS posture statement in the module doc (lines 11-22)
  is unusually thorough and accurate. The FIPS gate is implemented correctly:
  reads `/proc/sys/kernel/fips_enabled` via `ProcfsText` + `SecureReader`
  (non-bypassable, provenance-verified), blocks T4 when FIPS=1, records the
  decision in evidence, and fails open (missing fips_enabled is treated as
  FIPS disabled, which is correct for non-FIPS systems). This is a well-executed
  implementation of a genuinely difficult compliance boundary.
Severity: INFO
```

---

**RPM-15**

```
File: src/detect/integrity_check.rs
Location: lines 159-170 (File::open call)
Finding: `File::open(candidate)` is a path-based open. The code immediately
  follows with `rustix::fs::fstat` and a `(dev, ino)` comparison against the
  release_candidate evidence record (lines 174-228). When fstat verification
  succeeds and confirms identity, `opened_by_fd` is set to `true` in the evidence
  record (line 389). This is technically incorrect: `opened_by_fd` should reflect
  how the file was opened, not whether a post-open fstat verification succeeded.
  The file was opened via a path-based `File::open`. Marking it as `opened_by_fd=true`
  will mislead an auditor reviewing the evidence record. The TOCTOU mitigation is
  real and valuable, but the field name in the evidence record must not misrepresent
  the open method.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 AU-3 — audit record content must accurately
  reflect the actual operation; `opened_by_fd` should be `false` for path-based
  opens even when post-open fstat verification is performed.
Remediation owner: coder
```

---

### `src/detect/release_candidate.rs`

---

**RPM-16**

```
File: src/detect/release_candidate.rs
Location: lines 59-63 (OS_RELEASE_PATHS) and pipeline behavior
Finding: On RHEL 10, `/etc/os-release` is a symlink to `/usr/lib/os-release`.
  `statx` with `AtFlags::empty()` follows symlinks (line 141), so the recorded
  `(dev, ino)` is that of `/usr/lib/os-release` (the symlink target). However,
  the `candidate` path returned to the orchestrator is `/etc/os-release` (the
  symlink path, line 238). The ownership and digest phases then query the RPM
  database with `/etc/os-release`. The RPM database records `/usr/lib/os-release`
  — not `/etc/os-release` — because `/etc/os-release` is a symlink that is not
  tracked as a separate package file. This creates a systematic pipeline failure:
  on any RHEL 10+ system where `/etc/os-release` is a symlink, `query_file_owner`
  and `query_file_digest` will return `None` (the symlink path is unowned), and
  T4 can never be reached regardless of how the RPM database queries are otherwise
  implemented. The `path_resolved` field is recorded in evidence (line 285-298)
  but is not used to redirect the ownership/digest queries. This is a
  HIGH-severity functional security gap because it systematically prevents the
  pipeline from reaching its highest-assurance trust tier on the primary target
  platform.
Severity: HIGH
Recommended citation: NIST SP 800-53 SI-7 — software integrity verification
  must be able to complete on the target platform. The pipeline should query
  ownership and digest using the resolved (real) path when the candidate is a
  symlink, not the symlink path itself.
Remediation owner: coder
```

---

**RPM-17 (POSITIVE)**

```
File: src/detect/release_candidate.rs
Location: lines 140-173 (probe_candidate — world-writable check)
Finding: POSITIVE — the world-writable check is correct, complete, and properly
  records the rejection in evidence with a confidence downgrade. The statx call
  uses `AtFlags::empty()` (follows symlinks) which is explicitly commented.
  The comment at line 140 is accurate: "returns target (dev,ino), not symlink's."
  The behavior is correctly documented for the symlink case.
Severity: INFO
```

---

### `src/detect/pkg_substrate.rs`

---

**RPM-18**

```
File: src/detect/pkg_substrate.rs
Location: lines 174-198 (check_selinux_enforce gate)
Finding: When `SecureReader::<SelinuxEnforce>::new().read()` fails (the SELinux
  enforce file is unreadable), the function returns `false` and the confidence
  is downgraded. This is correct fail-closed behavior. However, on a system
  where SELinux is absent (non-SELinux kernel, or SELinuxfs not mounted), this
  will always downgrade, and T3 can never be reached. The module doc says
  "T3 cannot be fully asserted on a permissive-mode system" but the actual
  behavior is that T3 cannot be reached on any non-SELinux system either.
  The doc comment should distinguish between "SELinux is present but permissive"
  vs "SELinux is absent/not mounted." These are different conditions with different
  security implications, and both produce the same downgrade.
Severity: LOW
Recommended citation: NIST SP 800-53 SI-3, SI-7 — Biba integrity pre-check;
  documentation should distinguish absent SELinux from permissive SELinux,
  as they have different policy implications.
Remediation owner: tech-writer
```

---

### `src/detect/release_parse.rs`

---

**RPM-19**

```
File: src/detect/release_parse.rs
Location: lines 456-463 (parse_with_split — quote stripping)
Finding: Path B (`split_once` parser) uses `val.trim_matches('"')` to strip
  quotes. Path A (nom) uses `delimited(char('"'), opt(is_not("\"")), char('"'))`
  which correctly handles `KEY=""` (empty quoted value) and `KEY="value"` with
  inner content. `trim_matches('"')` strips ALL leading and trailing double-quotes,
  including multiple consecutive quotes. For a pathological value like
  `KEY="""triple"""`, Path A would likely fail to parse (nom would see `""` as
  empty, then leftover `"triple"""`), while Path B would produce `triple`. This
  creates a value-level disagreement that the TPI agreement check — which only
  compares KEY SETS, not values (line 473, "Value comparison is not performed
  here") — would not catch. The TPI gate protects against key-set tampering but
  not against value-level forgery in pathological inputs. The comment at line
  473 explicitly acknowledges this limitation. This is architecturally sound
  (key-set agreement is the documented gate) but worth flagging for future
  hardening — the TPI check could be extended to compare values for known-critical
  fields (ID, VERSION_ID).
Severity: LOW
Recommended citation: NIST SP 800-53 SI-7 — TPI coverage; consider extending
  value comparison to security-critical fields (ID=, VERSION_ID=) as an optional
  stricter gate.
Remediation owner: coder
```

---

**RPM-20**

```
File: src/detect/release_parse.rs
Location: lines 109-140 (run_inner — per-line length enforcement)
Finding: The per-line length check runs on the raw `content` string, which has
  already been read via `std::fs::File` (a path-based open, not bounded by
  `max_read_bytes`). The `read_to_string` call at line 335 has no byte cap —
  it will read the entire file into memory before any line-length enforcement
  occurs. On a legitimate system this is harmless (os-release files are small),
  but on a system where `/etc/os-release` has been replaced with a large file,
  this phase will allocate the full file content before rejecting it. The
  `integrity_check` phase has a bounded read via `read_bounded`, but `release_parse`
  does not. The `OsDetector::max_read_bytes` limit is passed to `integrity_check`
  but not to `release_parse::read_candidate`.
Severity: MEDIUM
Recommended citation: NSA RTB — bounded reads prevent unbounded allocation;
  NIST SP 800-218 SSDF PW.4.1 — resource limits on security-relevant input reads.
  The `release_parse` read should be capped by `max_read_bytes` (passed through
  from `OsDetector`).
Remediation owner: coder
```

---

### `src/detect/mount_topology.rs`

---

**RPM-21**

```
File: src/detect/mount_topology.rs
Location: lines 254-295 (read_etc_statfs)
Finding: `statfs("/etc")` is called via the `nix` crate. The module-level doc
  cites "NSA RTB RAIN: All procfs reads use ProcfsText + SecureReader" — this
  claim is correct for the procfs reads. However, `statfs("/etc")` is a path-based
  call. The comment in the code says "This cross-checks that the path where
  os-release will be sought is on a real, identifiable filesystem — not a tmpfs
  substitution." This is a legitimate security observation. The use of `nix::statfs`
  (not `rustix::fs::fstatfs` on an open fd) is a limitation of the available
  API surface — `statfs` must take a path. This is correctly not claimed to be
  fd-anchored. No annotation gap here; the limitation is inherent to the syscall
  and not misrepresented in the code. This is an INFO observation.
Severity: INFO
Recommended citation: Already correctly scoped in the module doc (NSA RTB RAIN
  covers procfs reads; statfs is a separate operation).
```

---

### `src/evidence.rs`

---

**RPM-22**

```
File: src/evidence.rs
Location: lines 223-228 (EvidenceBundle — pub records field)
Finding: `EvidenceBundle::records` is `pub`. The module doc and `push()` doc
  both assert "append-only" and "Records are never reordered or removed after
  being pushed." However, because `records` is pub, any caller in the crate can
  do `bundle.records.clear()`, `bundle.records.pop()`, or
  `bundle.records.sort()`, defeating the non-repudiation property. The AU-10
  citation ("callers cannot remove or reorder entries after push") is not
  enforced by the type system. This is a load-bearing security claim (AU-10,
  non-repudiation) that is contradicted by the actual implementation.
Severity: HIGH
Recommended citation: NIST SP 800-53 AU-10 — non-repudiation requires that the
  audit record be append-only and not modifiable by the caller. Make `records`
  private (`records: Vec<EvidenceRecord>`) and expose only `push()`, `len()`,
  `is_empty()`, and an immutable iterator (`iter()` / `as_slice()`). Code wins
  over the documentation claim.
Remediation owner: coder
```

---

### `src/confidence.rs`

---

**RPM-23 (POSITIVE)**

```
File: src/confidence.rs
Location: lines 118-182 (ConfidenceModel)
Finding: POSITIVE — the private `level` field with `upgrade()`/`downgrade()`
  accessor methods correctly enforces the invariant that level can only decrease
  after a downgrade call, and upgrade only applies when the new level is strictly
  higher. The NSA RTB RAIN / non-bypassability claim is well-implemented. The
  `record_contradiction` method correctly records contradictions before downg-
  rading, ensuring the audit trail captures both pieces of information.
Severity: INFO
```

---

### `Cargo.toml`

---

**RPM-24**

```
File: Cargo.toml
Location: line 46
Finding: `rusqlite = { version = "0.31", features = ["bundled"], optional = true }` —
  the "bundled" feature links against a vendored copy of SQLite embedded in the
  rusqlite crate. The comment at lines 40-46 justifies this as keeping the binary
  "hermetic on RHEL 10." However, the bundled SQLite is compiled from source
  at build time without RHEL's hardening flags, FIPS validation, or system
  patching pipeline. The system-provided `/usr/lib64/libsqlite3.so` on RHEL 10
  is compiled with `-fstack-protector-strong`, `-D_FORTIFY_SOURCE=2`, and is
  covered by Red Hat CVE response. A CVE in SQLite (e.g., a parser vulnerability
  in the blob-reading path) would require rebuilding the entire binary to address,
  whereas the system library would be patched by a `dnf update`. The supply chain
  comment (lines 40-46) acknowledges the tradeoff but does not evaluate it from
  the CMVP / RHEL security policy perspective. For a FIPS-mode RHEL 10 deployment,
  using the bundled SQLite is at minimum a policy question that should be
  explicitly resolved. The SA-12 citation in the comment is present but the
  risk analysis is incomplete.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SA-12 — supply chain risk: bundled SQLite
  is not covered by RHEL's security patching pipeline and is compiled without
  RHEL hardening flags. Consider using the system library (`features = []`) with
  an explicit runtime check for the minimum required SQLite version.
Remediation owner: coder
```

---

**RPM-25 (POSITIVE)**

```
File: Cargo.toml
Location: lines 39-46 (rusqlite supply chain commentary)
Finding: POSITIVE — the supply chain justification for `rusqlite` is more thorough
  than required, addressing maintenance status, transitive deps, and the lack
  of unsafe surface exposure to callers. The feature-gating (`optional = true`)
  is correctly implemented and tested. The SHA-2 crate selection rationale
  (lines 24-31) correctly notes it is not used for key derivation or
  authentication — an important FIPS boundary disclosure.
Severity: INFO
```

---

### `tests/rpm_header_tests.rs`

---

**RPM-26**

```
File: tests/rpm_header_tests.rs
Location: lines 251, 302 (unwrap() calls)
Finding: Two test-only `unwrap()` calls on `try_into()` for extracting 4-byte
  slices from a known-good byte vector. These are in test code and the
  surrounding context guarantees the slices are valid. `unwrap()` in tests is
  acceptable and normal — this is not a production code concern. Noted for
  completeness; no remediation required.
Severity: INFO
```

---

**RPM-27**

```
File: tests/rpm_header_tests.rs
Location: entire file
Finding: The test suite covers: minimal parse, file list reconstruction, digest
  algorithm variants (SHA256, SHA512, MD5 default), error cases (TooShort,
  InvalidIndexCount, TooLarge, OffsetOutOfBounds, MissingNulTerminator), TPI
  agreement and corruption, meta-package with no files, RELEASE tag, and digest
  algorithm mapping. Missing coverage:
  - No test for `InvalidDirindex` error (dirindex pointing to out-of-bounds
    dirname slot).
  - No test for `Utf8Error` (non-UTF-8 bytes in a string field).
  - No test for `hsize` = 0 with nonzero nindex (valid but unusual).
  - No test for maximum-allowed blob (near MAX_BLOB_BYTES) to verify the 16 MiB
    limit does not produce an off-by-one.
  These gaps are LOW severity — the core paths are well tested.
Severity: LOW
Recommended citation: NIST SP 800-53 SI-10 — input validation; test coverage for
  all defined error variants ensures the fail-closed claims are verified.
Remediation owner: coder
```

---

### `tests/rpm_db_tests.rs`

---

**RPM-28**

```
File: tests/rpm_db_tests.rs
Location: lines 42-65 (query_os_release_ownership test)
Finding: The test queries `/usr/lib/os-release` for ownership, which is correct
  (the real path in the RPM DB), but the pipeline under test queries with the
  symlink path `/etc/os-release` (see RPM-16). The test validates behavior of
  `RpmDb::query_file_owner` directly with the correct path — this is good for
  unit-testing the DB layer. However, there is no end-to-end integration test
  that exercises the full pipeline path through `OsDetector::detect()` and verifies
  that T4 is reached on an RHEL 10 system. The gap between what the DB layer can
  do (correctly resolves `/usr/lib/os-release`) and what the pipeline sends it
  (`/etc/os-release`) is not tested. This is the test-side manifestation of
  RPM-16.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SI-7 — integrity verification test coverage;
  an end-to-end pipeline test that asserts `label_trust == TrustedLabel` on RHEL
  10 would catch the symlink-path mismatch that prevents T4 from being reached.
Remediation owner: coder
```

---

### `examples/os_detect.rs`

---

**RPM-29 (POSITIVE)**

```
File: examples/os_detect.rs
Location: lines 311-318 (contradiction truncation), lines 440-443 (description truncation)
Finding: POSITIVE — both locations where `LabelTrust::IntegrityVerifiedButContradictory`
  and `Contradiction::description` are displayed apply `.chars().take(64)` before
  printing. This correctly implements the SI-12 note-length guidance from the
  `EvidenceRecord::notes` doc comment and the `LabelTrust` variant doc. The
  terminal detection (`std::io::IsTerminal`) for color selection is also correct.
Severity: INFO
```

---

## Gap Analysis Summary

```
Files reviewed: 16
  src/detect/substrate/rpm_header.rs
  src/detect/substrate/rpm_db.rs
  src/detect/substrate/rpm.rs
  src/detect/substrate/mod.rs
  src/detect/mod.rs
  src/detect/integrity_check.rs
  src/detect/file_ownership.rs
  src/detect/pkg_substrate.rs
  src/detect/release_candidate.rs
  src/detect/kernel_anchor.rs
  src/detect/mount_topology.rs
  src/detect/release_parse.rs
  src/detect/label_trust.rs
  src/evidence.rs
  src/confidence.rs
  Cargo.toml
  tests/rpm_header_tests.rs
  tests/rpm_db_tests.rs
  examples/os_detect.rs

Total findings: 29 (2 HIGH, 6 MEDIUM, 9 LOW, 12 INFO)
```

### Uncited Security Claims

- `rpm_db.rs` line 76: `Self::Sqlite(e) => write!(f, "rpm db sqlite error: {e}")` — the SI-12
  claim in the error type doc comment is not fully enforced because rusqlite's Display output
  is outside this codebase's control. (RPM-02)

- `evidence.rs` lines 243-246: `push()` doc claims "Records are never reordered or removed"
  (AU-10) but the `records` field is `pub`, permitting callers to mutate the vec directly.
  (RPM-22)

### Inconsistencies (code vs. docs)

1. **RPM-10** — `rpm.rs` module doc claims NSA RTB TOCTOU coverage for the probe. Code only
   implements TOCTOU protection for ownership queries; presence checks are path-based.
   Doc must be narrowed. Tech-writer remediation.

2. **RPM-13** — `integrity_check.rs` module doc says SHA-512 digests are "accepted." Code
   correctly blocks T4 on SHA-512 (cross-algorithm comparison unsupported). Doc says "accepted"
   when "recorded but unable to earn T4" is the accurate description. Tech-writer remediation.

3. **RPM-22** — `evidence.rs` module doc and `push()` doc both assert AU-10 append-only
   non-repudiation. `records` field is `pub`. Code contradicts the documentation claim.
   Coder remediation (make field private).

### Critical Path Issue (RPM-16)

RPM-16 is the highest-priority remediation. On any RHEL 10 system where `/etc/os-release`
is a symlink to `/usr/lib/os-release` (which is the standard RHEL 10 configuration),
the pipeline cannot reach T4 (`IntegrityAnchored`) because:

1. `release_candidate.rs` returns `/etc/os-release` as the candidate path.
2. `file_ownership.rs` queries the RPM DB with `/etc/os-release`.
3. The RPM DB owns `/usr/lib/os-release`, not `/etc/os-release`.
4. `query_file_owner` returns `None` → no ownership → T4 blocked.

The `path_resolved` field in the evidence record contains the correct real path, but
nothing in the pipeline uses it to redirect the queries. The fix is to use `path_resolved`
(the symlink target) for ownership and digest queries when it is available.
```
