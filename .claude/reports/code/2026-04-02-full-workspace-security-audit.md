# Full Workspace Security Audit

```
Audit date: 2026-04-02
Depth: in-depth
Scope: All 10 crates — umrs-core, umrs-hw, umrs-selinux, umrs-platform,
       umrs-ui, umrs-label, umrs-ls, umrs-stat, umrs-uname, umrs-c2pa
Auditor: Knox (security-engineer)
```

## Summary Table

| Category | Count |
|---|---|
| ERROR | 5 |
| CONCERN | 14 |
| ACCURATE | 12 |

---

## Findings by Crate

---

### umrs-c2pa

#### E-1: TOCTOU in `validate.rs` — `exists()` then `read()`

**Severity:** MEDIUM

File: `umrs-c2pa/src/c2pa/validate.rs`
Location: lines 191, 228, 411, 426, 452

Multiple functions call `path.exists()` then `std::fs::read(path)` as separate operations.
Between the existence check and the read, the file can be replaced by an attacker (symlink
substitution, race-condition replacement). The check and use must be a single operation:
attempt the read directly and handle the error.

```
Finding: TOCTOU — path.exists() followed by separate fs::read()
Severity: MEDIUM
Control reference: NIST SP 800-53 SI-7, NSA RTB
Remediation owner: coder
Recommended action: Remove the exists() check. Call std::fs::read() directly and match
  on Err(e) if e.kind() == NotFound for the "missing file" path. Same applies to
  check_cert_file, check_key_file, and check_pem_file.
```

#### E-2: TOCTOU in `creds.rs` — `exists()` then `read()` for credential validation

**Severity:** MEDIUM

File: `umrs-c2pa/src/c2pa/creds.rs`
Location: lines 245, 250, 257

Same pattern as E-1. `cert_path.exists()` and `key_path.exists()` checks are followed by
separate `std::fs::read()` calls at lines 267/274.

```
Finding: TOCTOU — credential file existence checked separately from read
Severity: MEDIUM
Control reference: NIST SP 800-53 SI-7, NSA RTB
Remediation owner: coder
Recommended action: Read directly; use error kind to determine absence.
```

#### E-3: `ingest.rs` — output file created with `File::create()` using default umask

**Severity:** MEDIUM

File: `umrs-c2pa/src/c2pa/ingest.rs`
Location: line 261

`std::fs::File::create(&out_path)` creates the output file with the process umask, which is
typically 0o022 (resulting in mode 0o644 — world-readable). For files carrying CUI security
markings (line 236-244 embeds the marking assertion), the output file should not be
world-readable by default. The comment in `umrs-label/src/main.rs` line 5 acknowledges
"umask configuration is the caller's responsibility" but the ingest pipeline should
enforce restrictive permissions at creation time, not rely on external configuration.

The private key write in `main.rs:567` correctly uses `OpenOptions::new().mode(0o600)`.
The ingest output should follow the same pattern.

```
Finding: Signed output file created with default umask, potentially world-readable
Severity: MEDIUM
Control reference: NIST SP 800-53 AC-3, SC-28
Remediation owner: coder
Recommended action: On Unix, use OpenOptions::new().write(true).create(true)
  .mode(0o640).open(&out_path) to restrict the signed output at creation time.
  For CUI-marked files, consider 0o600.
```

#### C-1: `trust.rs` — trust anchor PEM files read without `O_NOFOLLOW`

File: `umrs-c2pa/src/c2pa/trust.rs`
Location: line 69 (`read_pem`)

Trust anchor files are read via `std::fs::read_to_string(path)`, which follows symlinks.
An attacker who can create a symlink at the trust anchor path could redirect trust validation
to an arbitrary CA bundle. The private key read in `signer.rs:399` correctly uses
`O_NOFOLLOW`. Trust anchors deserve the same defense-in-depth treatment — they are the
root of trust for all C2PA signature validation.

```
Finding: Trust anchor PEM read follows symlinks
Severity: MEDIUM
Control reference: NIST SP 800-53 SI-7, AC-3
Remediation owner: coder
Recommended action: Apply the same O_NOFOLLOW pattern from read_private_key() to
  read_pem(). On non-Unix, fall back to std::fs::read_to_string.
```

#### C-2: `config.rs` — TOML config file read without `O_NOFOLLOW`

File: `umrs-c2pa/src/c2pa/config.rs`
Location: line 158

`UmrsConfig::load()` reads the config file via `std::fs::read_to_string(path)`, which
follows symlinks. Configuration files drive algorithm selection, credential paths, and
trust list paths. A symlink substitution here could redirect the tool to attacker-controlled
configuration.

```
Finding: Config file read follows symlinks
Severity: LOW
Control reference: NIST SP 800-53 CM-6
Remediation owner: coder
Recommended action: Use O_NOFOLLOW when opening the config file.
```

#### C-3: `validate.rs` — world-writable trust anchor is only WARN, should be FAIL

File: `umrs-c2pa/src/c2pa/validate.rs`
Location: line 521

`check_trust_file_permissions` emits `Warn` for world-writable trust anchor files. A
world-writable trust anchor is a complete trust bypass — any local user can inject arbitrary
CA certificates. This should be `Fail`, not `Warn`.

```
Finding: World-writable trust anchor treated as warning, not failure
Severity: MEDIUM
Control reference: NIST SP 800-53 SI-7, CM-5
Remediation owner: coder
Recommended action: Change Warn to Fail for world-writable trust anchor files.
  A trust anchor that any user can modify provides no trust.
```

#### C-4: `validate.rs` — `read_euid_from_proc()` reads `/proc/self/status` without provenance verification

File: `umrs-c2pa/src/c2pa/validate.rs`
Location: line 549

This reads `/proc/self/status` via `std::fs::read_to_string` — no fstatfs magic check.
While `/proc/self/status` is a low-risk path (it's process-local), the project has an
established `ProcfsText` + `SecureReader` pattern in `umrs-platform`. The c2pa crate
should use it for consistency.

```
Finding: procfs read bypasses SecureReader provenance verification
Severity: LOW
Control reference: NSA RTB RAIN
Remediation owner: coder
Recommended action: Use ProcfsText + SecureReader from umrs-platform for /proc/self/status.
  Same finding applies to creds.rs:474.
```

#### A-1: Private key zeroization is thorough

All private key material flows through `Zeroizing<Vec<u8>>` in both `signer.rs` and
`creds.rs`. The `read_private_key` function in `signer.rs` correctly uses `O_NOFOLLOW`
and reads via the opened fd. Key write in `main.rs:567` uses `create_new` with `mode(0o600)`.

#### A-2: Algorithm allow-list is correctly enforced

`parse_algorithm()` gates all signing paths. `ALLOWED_ALGORITHMS` excludes ed25519 with
documented FIPS rationale. The allow-list is checked before any key material is accessed.
NSA RTB RAIN property is satisfied.

#### A-3: TOCTOU-free ingest design

`ingest_file()` reads source bytes once into memory. Both SHA-256/SHA-384 digests and the
signing operation consume the same buffer. No second file read occurs.

---

### umrs-core

#### E-4: `save_state()` creates temp file with default umask

File: `libs/umrs-core/src/lib.rs`
Location: line 68

`fs::File::create(&tmp_path)` creates the temporary state file with the process umask.
State files may contain security-relevant data (e.g., `fips_enabled`). The temporary file
is world-readable during the write window.

```
Finding: Temporary state file created with default umask
Severity: LOW
Control reference: NIST SP 800-53 AC-3
Remediation owner: coder
Recommended action: On Unix, use OpenOptions with .mode(0o640) for the temp file.
```

#### C-5: `load_state()` — `path.exists()` then `File::open()`

File: `libs/umrs-core/src/lib.rs`
Location: lines 50-56

TOCTOU: `path.exists()` check followed by `File::open(path)`. The file could be replaced
between the two calls.

```
Finding: TOCTOU — exists() then open()
Severity: LOW
Control reference: NIST SP 800-53 SI-7
Remediation owner: coder
Recommended action: Remove the exists() check. Attempt File::open() directly;
  on NotFound, return Ok(UmrsState::default()).
```

#### A-4: `fs/mod.rs` is NOT wired in

The module-level documentation correctly states the architectural requirement: raw
`std::fs::read_to_string` reads of `/proc/` and `/sys/` must be replaced with
`ProcfsText`/`SysfsText` + `SecureReader` before enabling. The code is dormant.

---

### umrs-hw

#### A-5: Unsafe isolation is correctly confined

`#![forbid(unsafe_code)]` is intentionally absent only from this crate. The single
`unsafe` block (`rdtscp_raw()`) is correctly annotated with `// REVIEW: ASM` markers.
The `asm!` instruction reads CPU registers only (`nomem, nostack, preserves_flags`).
The fallback path on aarch64 uses `rustix::time::clock_gettime` with no unsafe.

---

### umrs-selinux

#### A-6: TPI enforcement is sound

All security context reads flow through `SecureXattrReader::read_context()`, which
enforces dual-path parsing (nom + FromStr). Disagreement returns `TpiError::Disagreement`
and the read fails closed. The xattr read is fd-anchored via `rustix::fs::fgetxattr`.

#### A-7: `lget_file_context` correctly uses `O_NOFOLLOW`

File: `libs/umrs-selinux/src/utils/mod.rs`
Location: line 69

`lget_file_context` opens with `custom_flags(libc::O_NOFOLLOW)` to read the symlink's
own security context, not the target's. Correct.

#### C-6: `get_pid_context` reads procfs without provenance verification

File: `libs/umrs-selinux/src/utils/mod.rs`
Location: line 113

`std::fs::read_to_string(&path)` on `/proc/<pid>/attr/current` bypasses the
`ProcfsText` + `SecureReader` pattern. While this is a per-process attribute (not a
global procfs node), the project standard is to route all procfs reads through the
verified path.

```
Finding: /proc/<pid>/attr/current read bypasses SecureReader
Severity: LOW
Control reference: NSA RTB RAIN
Remediation owner: coder
Recommended action: Route through ProcfsText + SecureReader for consistency.
```

#### C-7: `mcs/colors.rs` and `mcs/translator.rs` — config file reads not gated behind SELinux kernel check

File: `libs/umrs-selinux/src/mcs/colors.rs` line 159
File: `libs/umrs-selinux/src/mcs/translator.rs` line 477

Both `parse_secolor_file` and `load_into` (setrans.conf) read SELinux configuration files
directly via `File::open`. Per the Trust Gate Rule, config reads must be gated behind
a kernel status check. If these functions are only called after `is_selinux_enabled()`
returns true (which appears to be the case from the call sites), the gate exists upstream.
However, the functions themselves accept arbitrary paths and do not enforce the gate
internally. A caller could bypass the gate.

```
Finding: SELinux config file reads not internally gated
Severity: LOW
Control reference: NIST SP 800-53 CM-6, Trust Gate Rule
Remediation owner: coder
Recommended action: Document the precondition in the function doc comment, or
  add an internal gate check. A debug_assert!(is_selinux_enabled()) would catch
  misuse in development.
```

---

### umrs-platform

#### A-8: SecureReader enforces fd-anchored fstatfs

File: `libs/umrs-platform/src/kattrs/traits.rs`

`execute_read` and `execute_read_text` both open the file first, then call `fstatfs`
on the open fd to verify filesystem magic before parsing. The TOCTOU window between a
path-based statfs and open is correctly eliminated. This is the gold standard pattern.

#### A-9: integrity_check.rs performs TOCTOU re-verification via fstat

File: `libs/umrs-platform/src/detect/integrity_check.rs`
Location: lines 193-244

After opening the candidate file, the code calls `fstat` on the open fd and compares
`(dev, ino)` against the `release_candidate` statx record. The dev_t encoding mismatch
bug (lines 208-222) is correctly documented and fixed via `rustix::fs::major`/`minor`
decomposition. Sound.

#### C-8: `posture/configured.rs` and `posture/modprobe.rs` — `read_dir` follows symlinks

File: `libs/umrs-platform/src/posture/configured.rs` line 185
File: `libs/umrs-platform/src/posture/modprobe.rs` line 341

`std::fs::read_dir` follows symlinks in directory entries. A symlink in `/etc/sysctl.d/`
or `/etc/modprobe.d/` pointing to an attacker-controlled file could inject configuration.
SELinux type enforcement is the primary protection here, but the code should at minimum
check `is_file()` on the entry (which it does at lines 190/346), and ideally use
`symlink_metadata` to distinguish symlinks.

```
Finding: read_dir traversal follows symlinks in config directories
Severity: LOW
Control reference: NIST SP 800-53 CM-5
Remediation owner: coder
Recommended action: Use symlink_metadata() instead of is_file() to detect and
  log symlinks in config directories. The posture module already documents the
  SELinux symlink-following constraints in modprobe.rs:658-677.
```

#### C-9: `detect/substrate/rpm.rs` — `Path::exists()` for DB presence checks

File: `libs/umrs-platform/src/detect/substrate/rpm.rs`
Location: lines 133, 182-183

`Path::new(RPM_DB_ROOT).exists()` and similar calls. The code comments at line 132
acknowledge "Cache result once to avoid micro-TOCTOU from double `.exists()`", showing
awareness. For substrate detection (not a trust decision), this is acceptable risk.

```
Finding: Path::exists() for RPM DB detection
Severity: LOW (accepted risk — documented)
Control reference: NIST SP 800-53 SI-7
Remediation owner: N/A (documented accepted risk)
Recommended action: None required. The comment at line 132 documents the rationale.
```

---

### umrs-ui

#### A-10: No direct file I/O

`umrs-ui` is a TUI rendering library. It consumes structured data from other crates and
does not perform any direct file reads. The indicators module (line 43) explicitly documents
that no raw `File::open` is used.

---

### umrs-label

#### E-5: `main.rs` missing `#![forbid(unsafe_code)]`

File: `umrs-label/src/main.rs`
Location: crate root

The binary entry point has no `#![forbid(unsafe_code)]` declaration. While `lib.rs` has it,
`main.rs` is a separate compilation root and is not covered by the library's `forbid`.

```
Finding: Binary crate root missing #![forbid(unsafe_code)]
Severity: LOW
Control reference: NIST SP 800-218 SSDF PW.4, NSA RTB
Remediation owner: coder
Recommended action: Add #![forbid(unsafe_code)] to umrs-label/src/main.rs.
```

#### C-10: `catalog.rs` — JSON catalog loaded via `File::open` without symlink protection

File: `umrs-label/src/cui/catalog.rs`
Location: lines 105, 326

`load_catalog` and `load_levels` open JSON catalog files via `File::open(path_ref)`.
These catalog files define the CUI marking taxonomy — they are security-relevant
configuration. A symlink substitution could redirect the catalog to an
attacker-controlled JSON file, altering CUI category definitions.

```
Finding: CUI catalog file reads follow symlinks
Severity: LOW
Control reference: NIST SP 800-53 AC-16, CM-5
Remediation owner: coder
Recommended action: Use O_NOFOLLOW when opening catalog files.
```

---

### umrs-ls

#### A-11: `#![forbid(unsafe_code)]` present on both lib.rs and main.rs

Both `umrs-ls/src/lib.rs` (line 15) and `umrs-ls/src/main.rs` (line 35) carry the
forbid declaration. Correct.

---

### umrs-stat

#### C-11: `read_elf_info` opens file with `File::open` — no provenance verification

File: `umrs-stat/src/main.rs`
Location: line 206

`read_elf_info` opens an arbitrary path to read ELF headers. The function comment (line 202)
correctly states "not a trust-relevant assertion" and "display-only". This is acceptable
for its stated purpose, but the function should not be used in any trust decision path.

```
Finding: Display-only ELF read with no provenance check
Severity: LOW (documented limitation)
Control reference: N/A
Remediation owner: N/A
Recommended action: None required. Comment at line 202-203 documents the boundary.
```

#### C-12: `read_link` for symlink target display

File: `umrs-stat/src/main.rs`
Location: line 277

`std::fs::read_link(path)` is used for display-only symlink target resolution. The result
is never used in a trust decision. Acceptable.

---

### umrs-uname

#### A-12: Comprehensive posture assessment with provenance-verified reads

`umrs-uname` delegates all kernel attribute reads to `umrs-platform`'s `SecureReader`
and `ProcfsText` infrastructure. No raw file opens for security-relevant data.

---

### Dependency Findings

#### C-13: RUSTSEC-2023-0071 — `rsa` 0.9.10 Marvin Attack (Severity: 5.9 MEDIUM)

Dependency path: `rsa 0.9.10 <- c2pa 0.78.7 <- umrs-c2pa`

The `rsa` crate has a known timing side-channel vulnerability that could enable key
recovery. No fixed version is available. UMRS uses ECDSA (not RSA) for all signing paths,
so the `rsa` crate is pulled in transitively by the `c2pa` SDK but is not exercised by
UMRS signing operations. The risk is limited to C2PA manifest reading/verification that
encounters RSA-signed manifests from third parties.

```
Finding: Transitive dependency on rsa crate with Marvin Attack advisory
Severity: MEDIUM
Control reference: NIST SP 800-53 SC-13
Remediation owner: coder
Recommended action: Monitor c2pa crate releases for an upgrade that drops or
  patches the rsa dependency. Consider whether RSA-signed manifest verification
  can be feature-gated. Document the accepted risk.
```

#### C-14: RUSTSEC-2026-0002 — `lru` 0.12.5 Stacked Borrows violation (unsound)

Dependency path: `lru 0.12.5 <- ratatui 0.29.0 <- ratatui-garnish 0.1.0 <- umrs-ui`

The `lru` crate's `IterMut` implementation invalidates an internal pointer, violating
Stacked Borrows rules. This is an unsoundness bug (potential undefined behavior).
The vulnerable version is pulled in via `ratatui-garnish`, which depends on the older
`ratatui 0.29.0`. The main `umrs-ui` crate already depends on `ratatui 0.30.0`, which
uses `lru 0.16.3` (not affected). The issue is the `ratatui-garnish` transitive path.

**Impact assessment:** UMRS does not directly call `lru::LruCache::iter_mut()` in any
user-facing code path. The `lru` cache is internal to ratatui's layout computation.
Exploitation would require triggering specific iteration patterns in the TUI layout engine.
Practical exploitation risk is low, but the unsoundness is a compliance concern for
government review.

```
Finding: Unsound lru crate via ratatui-garnish transitive dependency
Severity: MEDIUM (unsoundness — compliance concern)
Control reference: NIST SP 800-218 SSDF PW.4
Remediation owner: coder
Recommended action: Update ratatui-garnish to a version that depends on ratatui
  0.30+ (which uses lru 0.16.3). If no such version exists, evaluate whether
  ratatui-garnish can be replaced or patched. The dual ratatui version (0.29 + 0.30)
  in the dependency graph is itself a concern — version unification would resolve both.
```

---

## Strengths Worth Preserving

1. **SecureReader pattern** (`umrs-platform/kattrs/traits.rs`) — fd-anchored fstatfs before
   any parse. This is the correct TOCTOU-safe kernel attribute read pattern. All kernel
   reads in the platform crate route through it.

2. **TPI enforcement** (`umrs-selinux/xattrs.rs`) — dual-path parsing with fail-closed
   disagreement handling. Integrity event distinction (Disagreement vs PathFailed) is
   well-designed.

3. **Key material zeroization** (`umrs-c2pa/signer.rs`, `creds.rs`) — consistent use of
   `Zeroizing<Vec<u8>>` for all private key material. The `read_private_key` function
   with `O_NOFOLLOW` + fd-anchored read is exemplary.

4. **FIPS algorithm gating** — `parse_algorithm()` as the single non-bypassable gate for
   all signing paths. ed25519 exclusion with documented FIPS rationale.

5. **Error information discipline** — `modprobe.rs` and `configured.rs` correctly suppress
   parameter values from debug logs on CUI systems. The pattern is explicitly documented
   with compliance references.

6. **`#![forbid(unsafe_code)]`** — present on 9 of 10 crates (umrs-hw is the documented
   exception). The provable safe-code guarantee is mechanically verifiable.

7. **Atomic key file creation** (`main.rs:567`) — `create_new` + `mode(0o600)` with no
   window of world-readable permissions.

---

## Gap Analysis Summary

```
Files reviewed: 85+ source files across 10 crates
Total findings: 31 (5 ERROR, 14 CONCERN, 12 ACCURATE)
Policy artifacts written: none (this is a code audit, not a policy authoring session)
Policy artifacts needed: none at this time

Documentation gaps:
- umrs-c2pa/src/main.rs: no crate-level lint attributes (no #![forbid(unsafe_code)])
- umrs-label/src/main.rs: no #![forbid(unsafe_code)]

Code-vs-policy inconsistencies:
- Trust anchor files (trust.rs) lack O_NOFOLLOW despite private key reads having it
- World-writable trust anchors classified as WARN instead of FAIL
- Ingest output files created with default umask despite carrying CUI markings
- Multiple TOCTOU patterns (exists() then read()) in validation paths

Dependency concerns:
- RUSTSEC-2023-0071: rsa 0.9.10 (Marvin Attack) — transitive via c2pa, ECDSA not affected
- RUSTSEC-2026-0002: lru 0.12.5 (unsound) — transitive via ratatui-garnish -> ratatui 0.29
- RUSTSEC-2024-0436: paste 1.0.15 (unmaintained) — transitive via ratatui 0.29
- RUSTSEC-2024-0370: proc-macro-error 1.0.4 (unmaintained) — transitive via c2pa

Findings requiring coder action (priority order):
1. [MEDIUM] E-3: Ingest output file permissions
2. [MEDIUM] C-3: World-writable trust anchor should be FAIL
3. [MEDIUM] C-1: Trust anchor reads need O_NOFOLLOW
4. [MEDIUM] E-1/E-2: TOCTOU in validation paths
5. [MEDIUM] C-13/C-14: Dependency advisories
6. [LOW] E-4: Temp file permissions in save_state
7. [LOW] E-5: Missing #![forbid(unsafe_code)] on binary roots
8. [LOW] C-2/C-4/C-5/C-6/C-7/C-8/C-10: Symlink/provenance consistency
```
