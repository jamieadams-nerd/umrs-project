# Security Engineering Review: umrs-c2pa

```
Audit date: 2026-04-01
Depth: in-depth
Scope: All source files in components/rusty-gadgets/umrs-c2pa/src/ (13 files)
Reviewer: Knox (security-engineer agent)
```

---

## Executive Summary

The `umrs-c2pa` crate implements C2PA manifest inspection and signing with a
well-structured architecture. FIPS algorithm gating, fail-closed error design,
and trust anchor non-bypassability are sound. However, several security
engineering gaps require remediation before deployment in a DoD/CUI environment:

- **Private key material is never zeroized** after use (HIGH)
- **No file permission checks** on private key or trust anchor files (HIGH)
- **All file I/O is path-based** with no symlink or TOCTOU protection (MEDIUM)
- **Generated private key written to disk before permission restriction** (MEDIUM)
- **Debug logs expose filesystem paths** that may be sensitive in CUI environments (LOW)

---

## Findings

### File: `src/c2pa/signer.rs`

#### K-1: Private key material in `Vec<u8>` is never zeroized

- **Severity:** HIGH
- **Location:** Lines 110-113 (`SignerMode::Credentials`), lines 311-315 (`generate_ephemeral_cert`)
- **Control reference:** NIST SP 800-53 SC-12 (Cryptographic Key Establishment and Management), NIST SP 800-53 SC-28 (Protection of Information at Rest)
- **Remediation owner:** coder

**Description:**
`SignerMode::Credentials` holds `key_pem: Vec<u8>` containing the raw PEM-encoded
private key. When this struct is dropped, the memory is deallocated but not zeroed.
The key material persists in freed heap pages until overwritten by a subsequent
allocation, which may be minutes or hours later. An attacker with read access to
process memory (core dump, `/proc/pid/mem`, cold boot attack) can recover the key.

Similarly, `generate_ephemeral_cert` at line 312 returns `key_pem` as `Vec<u8>`
which is passed through `build_signer` and eventually dropped without zeroization.

**Recommended fix:**
1. Add `zeroize` crate as a dependency (it is pure Rust, no unsafe, widely audited).
2. Replace `Vec<u8>` for key material with `zeroize::Zeroizing<Vec<u8>>` which
   implements `Drop` to zero the buffer.
3. Apply the same treatment to the `cert_pem` and `key_pem` return values from
   `generate_ephemeral_cert`.
4. The `SignerMode` enum should derive or implement `zeroize::ZeroizeOnDrop` for
   the `Credentials` variant's `key_pem` field.

```rust
// Example:
use zeroize::Zeroizing;

pub enum SignerMode {
    Ephemeral { alg: SigningAlg, organization: String },
    Credentials {
        alg: SigningAlg,
        cert_pem: Vec<u8>,
        key_pem: Zeroizing<Vec<u8>>,  // zeroized on drop
        tsa_url: Option<String>,
    },
}
```

---

#### K-2: `read_pem` in signer.rs reads private key via path-based I/O without permission check

- **Severity:** HIGH
- **Location:** Lines 324-326
- **Control reference:** NIST SP 800-53 SC-12, NIST SP 800-53 AC-3 (Access Enforcement)
- **Remediation owner:** coder

**Description:**
`read_pem` calls `std::fs::read(path)` which follows symlinks. A symlink placed at
the configured `private_key` path could redirect the read to an attacker-controlled
file (key substitution attack), or an attacker could replace the key file between
the time the operator verified it and the time the binary reads it (TOCTOU).

More critically, there is no check that the private key file has restrictive
permissions (mode 0600 or 0400, owned by the process uid). A world-readable
private key file is a deployment error that should be caught at the earliest
possible point.

**Recommended fix:**
1. Open with `std::fs::File::open()`, then `fstat` the fd to check:
   - File is a regular file (not a symlink, device, or FIFO)
   - Mode is 0600 or 0400 (no group/world bits set)
   - Owner matches the effective uid of the process
2. Read from the fd, not the path.
3. Return a descriptive error if permission checks fail.
4. Optionally use `O_NOFOLLOW` via `std::os::unix::fs::OpenOptionsExt` to refuse
   symlinks at the open site.

```rust
#[cfg(unix)]
fn read_private_key(path: &Path) -> Result<Zeroizing<Vec<u8>>, InspectError> {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::OpenOptionsExt;

    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(InspectError::Io)?;

    let meta = file.metadata().map_err(InspectError::Io)?;
    let mode = meta.mode() & 0o777;
    if mode & 0o077 != 0 {
        return Err(InspectError::Config(format!(
            "Private key file has unsafe permissions {:04o} — expected 0600 or 0400: {}",
            mode, path.display()
        )));
    }
    // ... read from file fd ...
}
```

---

#### K-3: Ephemeral certificate serial number is always 1

- **Severity:** LOW
- **Location:** Lines 265-268
- **Control reference:** None (informational)
- **Remediation owner:** coder

**Description:**
`BigNum::from_u32(1)` produces a fixed serial number for every ephemeral
certificate. While these certs are short-lived test artifacts, colliding serial
numbers from the same pseudo-issuer can confuse certificate stores and some
validators. RFC 5280 Section 4.1.2.2 recommends unique serial numbers.

**Recommended fix:**
Use `openssl::bn::BigNum::pseudo_rand(128, ...)` to generate a random serial,
or derive one from the current timestamp.

---

### File: `src/c2pa/creds.rs`

#### K-4: Generated private key material in `GeneratedCredentials` is never zeroized

- **Severity:** HIGH
- **Location:** Lines 26-38 (`GeneratedCredentials` struct), lines 98-100
- **Control reference:** NIST SP 800-53 SC-12, NIST SP 800-53 SC-28
- **Remediation owner:** coder

**Description:**
`GeneratedCredentials.key_pem: Vec<u8>` holds a freshly generated private key
in PEM format. This struct is returned to `main.rs` where the key is written to
disk, then the struct is dropped without zeroization. The key bytes persist in
freed memory.

**Recommended fix:**
Change `key_pem` to `Zeroizing<Vec<u8>>`. The `std::fs::write` call in `main.rs`
accepts `AsRef<[u8]>`, which `Zeroizing<Vec<u8>>` implements.

---

#### K-5: `validate()` reads private key into `Vec<u8>` without zeroization

- **Severity:** MEDIUM
- **Location:** Lines 204-209
- **Control reference:** NIST SP 800-53 SC-12
- **Remediation owner:** coder

**Description:**
The `validate` function reads the private key file into `key_bytes: Vec<u8>` to
check PEM format and parse it with OpenSSL. The `PKey` object is dropped at
function end, but the raw PEM bytes in `key_bytes` are not zeroized.

**Recommended fix:**
Use `Zeroizing<Vec<u8>>` for `key_bytes`. OpenSSL's `PKey::private_key_from_pem`
accepts `&[u8]`, and `Zeroizing<Vec<u8>>` derefs to `&[u8]`.

---

#### K-6: No permission check on key file during validation

- **Severity:** MEDIUM
- **Location:** Lines 185-189
- **Control reference:** NIST SP 800-53 AC-3, NIST SP 800-53 CM-6
- **Remediation owner:** coder

**Description:**
`validate()` checks that the key file exists and is readable PEM, but does not
verify that the file permissions are restrictive. A key file that is
world-readable (mode 0644) passes all validation checks. This is a missed
opportunity to catch a critical deployment error.

**Recommended fix:**
Add a `CredCheck` entry that reads the file metadata and verifies:
- Mode is 0600 or 0400 on Unix
- Owner uid matches the effective uid
- File is not a symlink

---

### File: `src/c2pa/trust.rs`

#### K-7: Trust anchor PEM files read via path-based I/O without integrity or permission checks

- **Severity:** MEDIUM
- **Location:** Lines 68-69 (`read_pem`), lines 149-168
- **Control reference:** NIST SP 800-53 SI-7 (Integrity), NIST SP 800-53 CM-5 (Access Restrictions for Change)
- **Remediation owner:** coder

**Description:**
Trust anchor PEM files are the root of trust for all C2PA signature validation.
`read_pem` uses `std::fs::read_to_string(path)` which follows symlinks and
performs no integrity or ownership verification. An attacker who can create a
symlink at the configured `trust_anchors` path, or replace the file between
config load and trust settings build, can inject arbitrary CA certificates.

Trust anchor files should not be world-writable, and ideally should be verified
against a known hash or IMA measurement.

**Recommended fix:**
1. Open with `O_NOFOLLOW` and verify the file is a regular file.
2. Check that the file is not world-writable (mode bit `o+w` is clear).
3. Log a warning if the file is group-writable.
4. Consider adding optional SHA-256 hash pinning in `TrustConfig` for
   high-assurance deployments.

---

#### K-8: Debug log exposes trust anchor filesystem paths

- **Severity:** LOW
- **Location:** Lines 154, 163, 184, 194
- **Control reference:** NIST SP 800-53 SI-11 (Error Handling)
- **Remediation owner:** coder

**Description:**
`log::debug!` calls include full filesystem paths for trust anchor files:
```rust
log::debug!(target: "umrs", "trust_anchors path={} certs={}", path.display(), cert_count);
```
In CUI/DoD environments, debug logging may be enabled during troubleshooting.
Filesystem paths can reveal directory structure, mount points, and operational
configuration to syslog consumers.

This is documented in the project's Debug Log Information Discipline Rule. The
paths should be logged at `trace` level or suppressed, with only a hash or
filename (not full path) logged at `debug`.

**Recommended fix:**
Log the filename only (not the full path) at debug level. Log the full path
at trace level. Example:
```rust
log::debug!(target: "umrs", "trust_anchors file={} certs={}",
    path.file_name().unwrap_or_default().to_string_lossy(), cert_count);
log::trace!(target: "umrs", "trust_anchors full_path={}", path.display());
```

---

### File: `src/c2pa/config.rs`

#### K-9: TOML config loaded via path-based I/O without integrity check

- **Severity:** LOW
- **Location:** Lines 155-157 (`UmrsConfig::load`)
- **Control reference:** NIST SP 800-53 CM-6 (Configuration Settings)
- **Remediation owner:** coder

**Description:**
`UmrsConfig::load` reads the TOML configuration file with `std::fs::read_to_string`.
The config file controls signing algorithm selection, credential paths, trust
anchor paths, and policy labels. A modified config file could direct the tool to
use attacker-controlled keys or trust anchors.

The risk is lower than for key material (the config is not itself secret), but in
a high-assurance deployment the configuration file integrity should be verifiable.

**Recommended fix:**
1. Add an optional `config_hash` CLI argument or environment variable that, when
   provided, verifies the SHA-256 of the config file before parsing.
2. For SELinux deployment: assign the config file an appropriate type
   (e.g., `umrs_c2pa_conf_t`) that restricts write access.
3. Check that the config file is not world-writable.

---

#### K-10: `PathBuf` fields in config accept arbitrary paths without sanitization

- **Severity:** LOW
- **Location:** Lines 67-71 (`cert_chain`, `private_key`), Lines 114-125 (trust paths)
- **Control reference:** NIST SP 800-53 SI-10 (Information Input Validation)
- **Remediation owner:** coder

**Description:**
`PathBuf` fields deserialized from TOML accept any string value, including:
- Relative paths (which resolve differently based on CWD)
- Paths containing `..` components (directory traversal)
- Paths to device files (`/dev/null`, `/dev/random`)

While the downstream `read_pem` calls will fail on invalid content, the error
messages may be confusing and the tool may attempt to open unexpected files.

**Recommended fix:**
In a validation pass (either at load time or in `validate_config`), canonicalize
paths and reject:
- Non-absolute paths (require absolute paths for production mode)
- Paths containing `..` components
- Paths that resolve to non-regular files

---

### File: `src/main.rs`

#### K-11: Private key written to disk before permissions are set (race window)

- **Severity:** MEDIUM
- **Location:** Lines 455-464
- **Control reference:** NIST SP 800-53 SC-12, NIST SP 800-53 AC-3
- **Remediation owner:** coder

**Description:**
In `cmd_creds_generate`, the private key is written with `std::fs::write`
(which creates the file with the process umask, typically 0644) and then
`set_permissions` is called to restrict it to 0600. Between the `write` and
`set_permissions` calls, the key file is world-readable.

The `set_permissions` call is also wrapped in `let _ =`, silently discarding
any error. If the permission change fails (e.g., on a filesystem that does not
support Unix permissions), the key file remains world-readable with no warning.

**Recommended fix:**
1. Create the file with restricted permissions from the start using
   `OpenOptions` with mode 0600:
   ```rust
   use std::os::unix::fs::OpenOptionsExt;
   let mut f = std::fs::OpenOptions::new()
       .write(true)
       .create_new(true)
       .mode(0o600)
       .open(&key_path)?;
   f.write_all(&result.key_pem)?;
   ```
2. Remove the separate `set_permissions` call.
3. If `set_permissions` is retained as a fallback for non-Unix, do not ignore
   the error — log a warning.

---

#### K-12: `expect()` calls on journald logger initialization can panic

- **Severity:** LOW
- **Location:** Lines 155-158
- **Control reference:** NIST SP 800-53 SI-10
- **Remediation owner:** coder

**Description:**
```rust
systemd_journal_logger::JournalLog::new()
    .expect("Failed to connect to journald")
```
This panics if journald is not running. On systems where the binary is invoked
in a container, chroot, or minimal environment without systemd, this causes an
unrecoverable crash instead of a graceful fallback to stderr logging.

**Recommended fix:**
Handle the error gracefully. If journald is not available, fall back to
`env_logger` or `eprintln`-based logging and emit a warning.

---

### File: `src/c2pa/validate.rs`

#### K-13: No permission checks on trust anchor or key files during validation

- **Severity:** MEDIUM
- **Location:** Lines 124-190 (`check_cert_file`, `check_key_file`), Lines 249-299 (`check_trust_config`)
- **Control reference:** NIST SP 800-53 AC-3, NIST SP 800-53 CM-6
- **Remediation owner:** coder

**Description:**
The `validate_config` function checks file existence and PEM format validity, but
never checks file permissions. This is the natural place to surface deployment
errors like:
- Private key file with mode 0644 (world-readable)
- Trust anchor file with mode 0666 (world-writable)
- Files owned by a different user than the process uid
- Files that are symlinks to unexpected locations

Operators run `umrs-c2pa config validate` as a preflight check. If it reports
"all checks passed" but the key file is world-readable, the operator has false
confidence in the deployment posture.

**Recommended fix:**
Add permission checks to `check_key_file` and `check_trust_config`:
- Key file: FAIL if mode has any group or world bits set
- Trust files: WARN if world-writable, INFO if group-writable
- All files: INFO if the file is a symlink (note the target)

---

### File: `src/c2pa/ingest.rs`

#### K-14: Source file read twice (SHA-256 + signing) without fd anchoring

- **Severity:** MEDIUM
- **Location:** Lines 99, 194
- **Control reference:** NSA RTB (TOCTOU), NIST SP 800-53 SI-7
- **Remediation owner:** coder

**Description:**
`ingest_file` computes the SHA-256 of the source file at line 99 by reading the
entire file, then reads the file again at line 194 for signing. Between these two
reads, the file could be modified by another process, causing the recorded hash
to not match the actual signed content. This is a classic TOCTOU pattern.

The severity is mitigated because the C2PA manifest includes its own internal
hash, but the external SHA-256 recorded in the audit log (line 207) would be
inconsistent with the actual signed content.

**Recommended fix:**
Read the file once into a buffer, compute SHA-256 from the buffer, and pass the
buffer to the signing operation. This eliminates the double-read TOCTOU window.

```rust
let source_bytes = std::fs::read(source_path).map_err(InspectError::Io)?;
let digest = Sha256::digest(&source_bytes);
let sha256 = hex::encode(digest);
// ... later ...
builder.sign(signer.as_ref(), &format, &mut std::io::Cursor::new(source_bytes), &mut out_file)
```

---

#### K-15: `has_manifest` probes file without trust settings

- **Severity:** LOW
- **Location:** Line 169 in `manifest.rs` (called from `ingest.rs` line 103)
- **Control reference:** NSA RTB RAIN (Non-Bypassability)
- **Remediation owner:** coder

**Description:**
`has_manifest` uses `c2pa::Reader::from_file` without constructing a `Context`
with trust settings. This is a read-only probe and does not affect signing
correctness, but it means this code path bypasses the trust settings gate that
all other reader paths go through. A malformed manifest could cause different
behavior in `has_manifest` vs. `read_chain`.

**Recommended fix:**
Use the same `Context`-based reader path as `read_chain`, or document the
intentional bypass with a safety comment explaining why trust settings are not
needed for an existence probe.

---

### File: `src/c2pa/error.rs`

#### K-16: `InspectError::Io` wraps `std::io::Error` which may contain sensitive paths

- **Severity:** LOW
- **Location:** Lines 28-29
- **Control reference:** NIST SP 800-53 SI-11
- **Remediation owner:** coder

**Description:**
`std::io::Error` messages from the OS often include the full filesystem path
(e.g., "No such file or directory: /opt/umrs/secrets/signing.key"). When these
errors propagate to CLI output or logs, they may leak directory structure.

**Recommended fix:**
Consider wrapping `Io` errors to strip or redact the path before display. The
path context can be preserved in debug-level logging but should not appear in
user-facing error messages.

---

### File: `src/c2pa/manifest.rs`

#### K-17: `manifest_json` with `detailed_json = true` exposes certificate chains in stdout

- **Severity:** LOW
- **Location:** Lines 185-206
- **Control reference:** NIST SP 800-53 SI-11
- **Remediation owner:** coder (documentation)

**Description:**
The `--detailed-json` flag causes `reader.detailed_json()` to be emitted to
stdout, which includes full PEM-encoded certificate chains from the manifest.
While these are public certificates (not private keys), in a CUI environment
the certificate chain itself may reveal organizational PKI structure.

This is not a vulnerability but should be documented with a CAUTION in the
usage documentation.

**Recommended fix:**
Add a doc comment and CLI help text noting that `--detailed-json` includes
full certificate data and should be used with care in sensitive environments.

---

### File: `src/c2pa/verbose.rs`

#### K-18: Verbose output includes filesystem paths and security markings on stderr

- **Severity:** LOW
- **Location:** Throughout the crate (all `verbose!()` calls)
- **Control reference:** NIST SP 800-53 SI-11
- **Remediation owner:** coder

**Description:**
Verbose mode prints trust anchor paths, private key paths, security marking
values, and signing details to stderr. While this is gated behind `--verbose`,
stderr output may be captured in shell history, CI logs, or redirected to files
that have broader access than intended.

Example from `signer.rs:131`:
```rust
verbose!("Loading private key from: {}", key_path.display());
```

**Recommended fix:**
1. Do not print private key paths in verbose mode. Print "Loading private key..."
   without the path.
2. Security marking values in verbose output (ingest.rs:171) are acceptable
   since the operator explicitly specified them.

---

## Dependency Assessment

### Supply Chain Risk

| Dependency | Version | License | Assessment |
|---|---|---|---|
| `c2pa` | 0.78.6 | MIT/Apache-2.0 | Adobe reference SDK. Acceptable. Large dependency tree. |
| `openssl` | 0.10 | MIT/Apache-2.0 | Widely audited FFI binding. Required for FIPS. |
| `openssl-sys` | 0.9 | MIT | Build-time OpenSSL binding. |
| `thiserror` | 1 | MIT/Apache-2.0 | Minimal proc-macro. Low risk. |
| `anyhow` | 1 | MIT/Apache-2.0 | Binary-only error handling. Acceptable. |
| `clap` | 4 | MIT/Apache-2.0 | Standard CLI framework. |
| `toml` | 0.8 | MIT/Apache-2.0 | Standard TOML parser. |
| `serde` | 1 | MIT/Apache-2.0 | Standard serialization. |
| `sha2` | 0.10 | MIT/Apache-2.0 | RustCrypto SHA-2. Pure Rust. |
| `chrono` | 0.4 | MIT/Apache-2.0 | Time handling. Has had past CVEs. Monitor. |
| `ureq` | 2 | MIT/Apache-2.0 | HTTP client (optional, gated). Acceptable for TSA. |
| `systemd-journal-logger` | 2 | MIT/Apache-2.0 | journald integration. |

**Note:** The `c2pa` crate pulls in a significant transitive dependency tree.
A full `cargo audit` should be run before any deployment.

**Missing dependency:** The `zeroize` crate is not present but is required for
findings K-1, K-4, and K-5.

---

## SELinux Deployment Considerations

The following SELinux types should be defined for a production deployment of
`umrs-c2pa` on RHEL 10. These are recommendations for future policy authoring,
not findings against the current code.

| File/Resource | Proposed SELinux Type | Purpose |
|---|---|---|
| `/usr/bin/umrs-c2pa` | `umrs_c2pa_exec_t` | Binary executable |
| `/etc/umrs/c2pa.toml` | `umrs_c2pa_conf_t` | Configuration (read-only by process) |
| Trust anchor PEM files | `umrs_c2pa_trust_t` | CA certificates (read-only by process) |
| Private key files | `umrs_c2pa_key_t` | Signing keys (read-only by process, no group/world) |
| Ingest dropbox directory | `umrs_c2pa_input_t` | Input media files |
| Signed output directory | `umrs_c2pa_output_t` | Signed media files |
| Runtime process | `umrs_c2pa_t` | Process domain |

Policy rules would restrict `umrs_c2pa_t` to:
- Read `umrs_c2pa_conf_t`, `umrs_c2pa_trust_t`, `umrs_c2pa_key_t`, `umrs_c2pa_input_t`
- Write `umrs_c2pa_output_t`
- Connect to journald socket
- Optionally connect to TSA endpoint (when `internet` feature is enabled)
- Deny all other network access

This policy work is deferred until the binary is ready for packaging.

---

## Gap Analysis Summary

```
Files reviewed: 13
Total findings: 18 (3 HIGH, 5 MEDIUM, 10 LOW)
Policy artifacts written: none (code-level findings only; policy deferred to packaging phase)
Policy artifacts needed: SELinux policy module for umrs-c2pa (see SELinux Deployment Considerations)
```

### Documentation Gaps

- `--detailed-json` flag should carry a CAUTION about certificate chain exposure (K-17)
- Trust anchor file permission requirements are not documented in the config template
- Private key permission requirements (0600) should be documented in the config template comments

### Code-vs-Policy Inconsistencies

- None detected. The code does not yet have an associated SELinux policy module, which is expected at this project phase.

### Findings by Remediation Owner

| Finding | Severity | Owner | Summary |
|---|---|---|---|
| K-1 | HIGH | coder | Zeroize `SignerMode::Credentials.key_pem` |
| K-2 | HIGH | coder | Permission check + `O_NOFOLLOW` for private key reads |
| K-4 | HIGH | coder | Zeroize `GeneratedCredentials.key_pem` |
| K-11 | MEDIUM | coder | Create key file with mode 0600 from the start |
| K-14 | MEDIUM | coder | Single-read for SHA-256 + signing (TOCTOU) |
| K-13 | MEDIUM | coder | Permission checks in `validate_config` |
| K-5 | MEDIUM | coder | Zeroize `key_bytes` in `creds::validate` |
| K-6 | MEDIUM | coder | Permission check on key file during validation |
| K-7 | MEDIUM | coder | Integrity/permission checks on trust anchor files |
| K-3 | LOW | coder | Random serial number for ephemeral certs |
| K-8 | LOW | coder | Suppress full paths in debug logs |
| K-9 | LOW | coder | Config file integrity verification option |
| K-10 | LOW | coder | Path sanitization for config `PathBuf` fields |
| K-12 | LOW | coder | Graceful fallback when journald unavailable |
| K-15 | LOW | coder | Document or fix `has_manifest` trust bypass |
| K-16 | LOW | coder | Redact paths from user-facing `Io` errors |
| K-17 | LOW | coder + tech-writer | Document `--detailed-json` certificate exposure |
| K-18 | LOW | coder | Remove private key paths from verbose output |

### Priority Remediation Order

1. **K-1, K-4, K-5** (zeroization) — add `zeroize` dependency and wrap all key material. Single PR.
2. **K-2, K-11** (key file permissions) — create files with correct mode, check on read. Single PR.
3. **K-14** (TOCTOU double-read) — refactor `ingest_file` to single-read. Single PR.
4. **K-6, K-7, K-13** (permission checks in validation) — add `CredCheck` entries. Single PR.
5. Remaining LOW findings can be addressed incrementally.

---

*Report generated by Knox (security-engineer agent) on 2026-04-01.*
*This report is for human review. It does not constitute approval for deployment.*
