# Security Audit Report: umrs-c2pa Crate

```
Audit date: 2026-04-01
Depth: in-depth
Scope: All .rs source files in components/rusty-gadgets/umrs-c2pa/src/
       (13 files: lib.rs, main.rs, c2pa/mod.rs, c2pa/config.rs, c2pa/creds.rs,
        c2pa/error.rs, c2pa/ingest.rs, c2pa/manifest.rs, c2pa/report.rs,
        c2pa/signer.rs, c2pa/trust.rs, c2pa/validate.rs, c2pa/verbose.rs)
```

---

## Summary Table

| Category | Count |
|---|---|
| ACCURATE | 14 |
| CONCERN | 9 |
| ERROR | 3 |

---

## Findings by File

---

### `src/c2pa/validate.rs`

---

```
File: src/c2pa/validate.rs
Location: module level
Finding: The file has no //! module-level doc block. It is missing all three
  required elements: purpose statement, key exported types, and a ## Compliance
  section. This module exposes CheckStatus, ValidationResult, and validate_config,
  which perform FIPS algorithm enforcement checks and trust list preflight
  validation. The absence of a compliance block leaves these security-critical
  checks without any control citation.
Severity: HIGH
Recommended citation: NIST SP 800-53 CM-6 (Configuration Settings), NIST SP 800-53 SC-13
  (Cryptographic Protection — FIPS algorithm check), NSA RTB Fail Secure
  (fail-closed reporting on invalid algorithm)
Remediation owner: coder
```

---

```
File: src/c2pa/validate.rs
Location: validate_config function (line 67)
Finding: #[must_use] is present but has no message string. The project rules
  require #[must_use] to include a message explaining why the return value matters.
  Bare annotations are explicitly non-compliant per the Must-Use Contract Rule
  and clippy::must_use_candidate does not enforce message strings.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SI-10 (Information Input Validation)
Remediation owner: coder
```

---

```
File: src/c2pa/validate.rs
Location: CheckStatus enum (line 9) and ValidationResult struct (line 19)
Finding: Both types are security-relevant (they carry FIPS algorithm pass/fail
  status and trust list validation results) and are exported from the module,
  but neither has any doc comment with a control citation. CheckStatus::Fail
  drives the process exit code that determines whether a misconfigured system
  is allowed to continue — this is a direct security enforcement decision.
Severity: LOW
Recommended citation: NIST SP 800-53 SI-10, NIST SP 800-53 SC-13
Remediation owner: coder
```

---

### `src/c2pa/creds.rs`

---

```
File: src/c2pa/creds.rs
Location: module level (line 1-11)
Finding: The //! doc block exists but contains no ## Compliance section. This
  module performs PKI key generation, certificate construction, and credential
  validation — all directly security-critical operations. The comment says
  the goal is "simple, simple, simple," which is a design rationale but not a
  compliance annotation. The module must have a Compliance section per the
  Module Documentation Checklist Rule.
Severity: HIGH
Recommended citation: NIST SP 800-53 SC-13 (Cryptographic Protection), NIST SP 800-218
  SSDF PW.4.1 (validate at construction — algorithm gated through parse_algorithm),
  CMMC SC.L2-3.13.10 (FIPS-validated cryptography)
Remediation owner: coder
```

---

```
File: src/c2pa/creds.rs
Location: generate function (line 75)
Finding: The generate function is security-critical (it produces signing key
  material and certificates) but has no control citations in its doc comment.
  It delegates algorithm validation to signer::parse_algorithm, which is the
  correct non-bypassability pattern, but this chain is not documented at the
  function level.
Severity: LOW
Recommended citation: NIST SP 800-53 SC-13, NSA RTB RAIN (Non-Bypassability —
  algorithm validation flows through parse_algorithm unconditionally)
Remediation owner: coder
```

---

```
File: src/c2pa/creds.rs
Location: validate function (line 149)
Finding: #[must_use] is present but has no message string. The return value
  of this function (Vec<CredCheck>) carries pass/fail results for certificate
  validity and key-cert pair matching. Discarding it silently would mean a
  caller never sees that credentials are expired or mismatched.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SI-10
Remediation owner: coder
```

---

```
File: src/c2pa/creds.rs
Location: GeneratedCredentials struct (line 26) and CredCheck struct (line 42)
Finding: GeneratedCredentials contains raw PEM key material (key_pem field)
  and cert_or_csr_pem. It is a security-relevant type (it holds cryptographic
  signing material) but is not marked #[must_use]. If a caller discards the
  return value of generate(), the key material is silently lost. The type
  should carry #[must_use] at the type definition per the Must-Use Contract Rule.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SC-13, NSA RTB Fail Secure
Remediation owner: coder
```

---

### `src/c2pa/verbose.rs`

---

```
File: src/c2pa/verbose.rs
Location: module level (line 1-13)
Finding: The //! doc block exists with a purpose statement but has no
  ## Compliance section. While verbose output is not a direct security control,
  the Module Documentation Checklist Rule requires every file to have a
  Compliance section, even if the note is that no direct security surface exists.
  The doc comment should explicitly state this.
Severity: LOW
Recommended citation: No direct control. The Compliance section should state:
  "This module provides console progress output with no direct security surface.
  NIST SP 800-53 SI-11 (Error Handling) governs the separation of verbose
  progress output (stderr) from structured audit logging (journald)."
Remediation owner: coder
```

---

### `src/main.rs`

---

```
File: src/main.rs
Location: module level
Finding: No //! module-level doc block is present. The Module Documentation
  Checklist Rule applies to every .rs source file under src/ with no exception
  for binary entry points. This file initializes journald logging, dispatches
  to security-sensitive subcommands, and sets the log level filter — all
  security-relevant behaviors that should be annotated at the module level.
Severity: LOW
Recommended citation: NIST SP 800-53 AU-3 (Audit Record Content — journald
  initialization), NIST SP 800-53 CM-6 (Configuration Settings — config load
  and dispatch), NIST SP 800-53 SC-13 (Cryptographic Protection — delegation
  to signing subcommands)
Remediation owner: coder
```

---

```
File: src/main.rs
Location: main function (line 135), journald initialization (line 154-159)
Finding: The journald logger is initialized with .expect() on two calls. The
  project prohibits unwrap_used and treat expect() as equivalent in spirit.
  While #[warn(clippy::unwrap_used)] is active at the crate level, it covers
  unwrap() not expect(). More significantly, a journald initialization failure
  silently panics with a fixed message rather than falling back gracefully or
  writing a structured error. This is not fail-closed — it is fail-loud with
  no audit trail.

  The two .expect() calls are:
    .expect("Failed to connect to journald")
    .expect("Failed to initialize journald logger")

  If journald is unavailable (e.g., non-systemd environment), the binary
  panics before any operation is performed. Whether this constitutes a
  security concern depends on deployment context, but a high-assurance tool
  should handle journald unavailability gracefully (fallback to stderr logging)
  rather than aborting.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 AU-5 (Response to Audit Processing Failures)
Remediation owner: coder
```

---

### `src/c2pa/manifest.rs`

---

```
File: src/c2pa/manifest.rs
Location: read_chain function (line 138)
Finding: read_chain is a security-critical function (it performs trust
  validation on chain-of-custody data) but is missing #[must_use]. A caller
  that invokes read_chain and discards the result without inspecting trust
  status receives no compiler warning. The function returns a Vec<ChainEntry>
  where each entry carries a TrustStatus that may be Invalid or Revoked.
  Silent discard of this result would bypass a trust enforcement decision.
Severity: HIGH
Recommended citation: NIST SP 800-53 SI-10 (Information Input Validation),
  NSA RTB RAIN (Non-Bypassability)
Remediation owner: coder
```

---

```
File: src/c2pa/manifest.rs
Location: manifest_json function (line 185) and chain_json function (line 238)
Finding: Neither manifest_json nor chain_json carries #[must_use]. Both
  return Results that carry manifest content or serialized chain data used
  for audit and forensic output. While these are less directly load-bearing
  than read_chain, they still return Results per the Must-Use Contract Rule,
  which requires #[must_use] on all public functions returning Result.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SI-10
Remediation owner: coder
```

---

```
File: src/c2pa/manifest.rs
Location: derive_trust function (line 477), catch-all branch (line 502-503)
Finding: The catch-all branch of derive_trust returns TrustStatus::Untrusted
  when validation_status codes are non-empty but no recognized code matches.
  This means an unknown or future SDK status code (one that is neither
  "revoked", nor contains "mismatch"/"failed", nor is "signingCredential.trusted",
  nor contains "untrusted") silently resolves to Untrusted rather than signaling
  an unrecognized state.

  This is a documentation gap, not a code error — Untrusted is a conservative
  and defensible fallback. However, the behavior is undocumented in the function.
  An inline comment explaining the rationale for the fallback is required so
  future maintainers do not change it without understanding the security implication.
Severity: LOW
Recommended citation: NSA RTB Fail Secure (conservative fallback to Untrusted
  when status codes are unrecognized is the correct fail-closed behavior)
Remediation owner: coder
```

---

```
File: src/c2pa/manifest.rs
Location: has_manifest function (line 167)
Finding: #[must_use] is present but has no message string. The function
  determines whether trust validation and chain walking should proceed. The
  annotation should explain what happens if the result is discarded.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SI-10
Remediation owner: coder
```

---

```
File: src/c2pa/manifest.rs
Location: TrustStatus enum (line 54)
Finding: TrustStatus is a security-relevant type (it carries an access-decision-
  adjacent trust evaluation result that is displayed in chain-of-custody reports
  and serialized to JSON). The type is not marked #[must_use] at the type
  definition. Per the Must-Use Contract Rule, security-relevant types that carry
  pending or evaluated security decisions should be annotated at the type level.
Severity: LOW
Recommended citation: NIST SP 800-53 AU-3 (Audit Record Content), NSA RTB RAIN
Remediation owner: coder
```

---

### `src/c2pa/ingest.rs`

---

```
File: src/c2pa/ingest.rs
Location: ingest_file function (line 85)
Finding: ingest_file is the primary signing entry point — a security-critical
  function — but is not marked #[must_use]. It returns Result<IngestResult, InspectError>.
  If a caller discards the return value, the signing outcome (including the
  output path and ephemeral mode status) is silently lost. No compiler warning
  is produced. This is directly load-bearing: a discarded result means no
  caller verification that signing succeeded.
Severity: HIGH
Recommended citation: NIST SP 800-53 AU-10 (Non-repudiation), NIST SP 800-53 SI-10
  (Information Input Validation), NSA RTB RAIN
Remediation owner: coder
```

---

```
File: src/c2pa/ingest.rs
Location: IngestResult struct (line 46)
Finding: IngestResult is a security-relevant type (it is a chain-of-custody
  evidence record that includes signing outcome, SHA-256 hash, action label,
  and ephemeral mode status). It is not marked #[must_use] at the type definition.
  Per the Must-Use Contract Rule, security-relevant types must carry #[must_use].
Severity: MEDIUM
Recommended citation: NIST SP 800-53 AU-10, NIST SP 800-53 AU-3
Remediation owner: coder
```

---

```
File: src/c2pa/ingest.rs
Location: sha256_hex function (line 240)
Finding: sha256_hex returns Result<String, InspectError> but has no #[must_use].
  The SHA-256 digest is used as an integrity reference in audit log entries and
  chain-of-custody reports. If a caller discards it, the integrity reference is
  lost without warning. This function is also exported as part of the public API.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SI-7 (Software, Firmware, and Information Integrity)
Remediation owner: coder
```

---

### `src/c2pa/signer.rs`

---

```
File: src/c2pa/signer.rs
Location: describe_algorithm function (line 66)
Finding: #[must_use] is present but has no message string. While describe_algorithm
  is not directly security-critical (it returns a human-readable string), it
  is annotated with #[must_use] and must comply with the message requirement.
Severity: LOW
Recommended citation: No additional citation needed — the message string
  should read: "Algorithm description is used in operator-facing validation
  output; discarding it silently produces no report."
Remediation owner: coder
```

---

```
File: src/c2pa/signer.rs
Location: is_ephemeral function (line 319)
Finding: #[must_use] is present but has no message string. is_ephemeral drives
  the decision of whether to flag signing output as test-mode in reports and
  log entries. Discarding it would suppress the ephemeral mode warning.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 AU-3 (Audit Record Content — ephemeral
  status is recorded in ingest log entries)
Remediation owner: coder
```

---

```
File: src/c2pa/signer.rs
Location: SignerMode enum (line 101)
Finding: SignerMode is a security-relevant type (it determines whether signing
  uses production credentials or an ephemeral test cert, and carries the FIPS-
  validated algorithm choice). It is not marked #[must_use] at the type definition.
  A caller that discards the return value of resolve_signer_mode silently loses
  the resolution outcome.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SC-13, NSA RTB Fail Secure
Remediation owner: coder
```

---

```
File: src/c2pa/signer.rs
Location: resolve_signer_mode function (line 123) and build_signer function (line 174)
Finding: Neither resolve_signer_mode nor build_signer carries #[must_use].
  Both return Results from security-critical operations — resolve_signer_mode
  gates FIPS algorithm validation; build_signer produces the cryptographic
  signing object. Per the Must-Use Contract Rule, all public functions returning
  Result must carry #[must_use] with a message.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 SC-13, NSA RTB RAIN
Remediation owner: coder
```

---

### `src/c2pa/config.rs`

---

```
File: src/c2pa/config.rs
Location: module-level doc comment, Compliance section (line 27)
Finding: The Compliance section cites "NSA RTB" without specifying a principle.
  The project Citation Format Rule requires "NSA RTB" followed by the specific
  principle (e.g., NSA RTB RAIN, NSA RTB Fail Secure). The cited behavior —
  credential paths not accessed until requested, no I/O at load time — maps
  to the Trust Gate pattern, which cites NIST SP 800-53 CM-6. If the intent
  is to cite non-bypassability of the trust gate, the correct form is NSA RTB RAIN.
Severity: MEDIUM
Recommended citation: Replace "NSA RTB" with "NSA RTB RAIN" if the intent is
  non-bypassability of the trust gate, or remove the NSA RTB citation if the
  intent is solely the Trust Gate pattern (NIST SP 800-53 CM-6 suffices).
Remediation owner: coder
```

---

```
File: src/c2pa/config.rs
Location: UmrsConfig methods: has_credentials (line 178), has_trust_config (line 184),
  log_level_filter (line 162)
Finding: All three methods carry bare #[must_use] without message strings.
  - has_credentials drives ephemeral-vs-production mode selection
  - has_trust_config gates trust validation and warning emission
  - log_level_filter determines audit log verbosity
  All three are security-adjacent decisions. The #[must_use] message strings
  are required per the Must-Use Contract Rule.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 CM-6 (for log_level_filter and has_trust_config),
  NIST SP 800-53 SC-13 (for has_credentials)
Remediation owner: coder
```

---

### `src/lib.rs`

---

*No findings. Module-level doc block is complete with purpose, key exported types,
and a well-cited Compliance section. #[forbid(unsafe_code)] is present. Clippy
pedantic and nursery are enabled. All approved suppressions are present.*

---

### `src/c2pa/mod.rs`

---

*No findings. Module-level doc block is complete with purpose, key re-exports,
and a well-cited Compliance section including NSA RTB RAIN with specific principle.
NSA RTB citations are properly formed.*

---

### `src/c2pa/trust.rs`

---

*No findings. Module-level doc block is complete and well-structured. The trust
model is accurately described and consistent with the implementation. build_c2pa_settings
carries #[must_use] with a descriptive message. Pattern execution measurement in
debug mode is implemented correctly. Fail-closed behavior (returning Err on unreadable
PEM) is correctly implemented and documented. NSA RTB RAIN citation is properly formed.*

---

### `src/c2pa/error.rs`

---

*No findings. Module-level doc block is complete with purpose, key types, and
a well-cited Compliance section. InspectError variants carry structured context
consistent with the AU-3 and SI-10 claims.*

---

### `src/c2pa/report.rs`

---

*No findings. Module-level doc block is complete. SI-11 claim ("does not emit
key material, credential paths, or classified data") is accurate: the report
functions emit signer names, issuers, timestamps, algorithms, and security
markings — none of which constitute key material or credential paths. The
output path printed for ingest results is the destination file path, not a
credential file path.*

---

## Detailed Finding List

### ERROR Items

| ID | File | Location | Description |
|---|---|---|---|
| E-1 | validate.rs | module level | No //! block — security enforcement module is entirely uncited |
| E-2 | creds.rs | module level | Missing ## Compliance section — PKI key generation module uncited |
| E-3 | ingest.rs | ingest_file | Primary signing function lacks #[must_use] — silent discard of chain-of-custody outcome |

**E-1: validate.rs — No Module Doc Block**

Severity: HIGH

The validate module performs FIPS algorithm enforcement and trust list preflight
validation. It is entirely uncited. This is the most severe gap in the crate
because the module-level compliance block is the primary documentation of what
security controls are satisfied by the validation workflow.

Recommended replacement:

```rust
//! # UMRS C2PA Configuration Preflight Validation
//!
//! Runs preflight checks against a loaded [`UmrsConfig`] to verify that
//! signing credentials, algorithms, trust lists, and operational parameters
//! are correctly configured before any signing or inspection operation.
//!
//! ## Key Exported Types
//!
//! - [`CheckStatus`] — pass/fail/warn/info/skip status for a single check
//! - [`ValidationResult`] — result of a single preflight check
//! - [`validate_config`] — run all preflight checks; returns one result per check
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-6**: Configuration Settings — all operational
//!   parameters are verified against expected values before use.
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — the algorithm check
//!   verifies that the configured algorithm is in the FIPS-safe allow-list.
//! - **NSA RTB Fail Secure**: a CheckStatus::Fail on any required check
//!   causes the caller to exit non-zero, preventing operation with an invalid
//!   configuration.
```

Remediation owner: coder

---

**E-2: creds.rs — Missing Compliance Section**

Severity: HIGH

The credential generation module produces RSA/ECDSA key material and
certificates for C2PA signing. No compliance section exists. This is a
PKI-critical module — FIPS algorithm gating via parse_algorithm is the
primary control, and it is completely undocumented at the module level.

Recommended replacement — add after the existing doc block:

```rust
//! ## Compliance
//!
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — key generation
//!   is gated through [`signer::parse_algorithm`], which enforces the
//!   FIPS-safe algorithm allow-list before any key material is produced.
//! - **NIST SP 800-218 SSDF PW.4.1**: Validate at construction — algorithm
//!   validation occurs at the start of [`generate`], before any I/O.
//! - **CMMC SC.L2-3.13.10**: Employ FIPS-validated cryptography — all
//!   generated keys use ECDSA P-256/384/521 curves.
//! - **NSA RTB RAIN**: Non-Bypassability — every code path through
//!   [`generate`] calls `parse_algorithm` unconditionally.
```

Remediation owner: coder

---

**E-3: ingest_file — Missing #[must_use]**

Severity: HIGH

ingest_file is the primary chain-of-custody signing function. A caller that
discards its return value has no way to know whether signing succeeded, which
output path was written, or whether an ephemeral cert was used. This is a
load-bearing non-repudiation function.

Recommended replacement:

```rust
#[must_use = "Discarding IngestResult loses the chain-of-custody record, \
              output path, and ephemeral mode status; call sites must \
              inspect or log the result"]
pub fn ingest_file(...) -> Result<IngestResult, InspectError> {
```

Remediation owner: coder

---

### CONCERN Items

| ID | File | Location | Description |
|---|---|---|---|
| C-1 | validate.rs | validate_config | Bare #[must_use] — no message string |
| C-2 | creds.rs | validate fn | Bare #[must_use] — no message string |
| C-3 | creds.rs | GeneratedCredentials | Security-relevant type not marked #[must_use] |
| C-4 | manifest.rs | read_chain | Missing #[must_use] on trust-validating function |
| C-5 | manifest.rs | manifest_json, chain_json, has_manifest | Missing or bare #[must_use] |
| C-6 | ingest.rs | IngestResult, sha256_hex | Missing #[must_use] on type and Result-returning function |
| C-7 | signer.rs | is_ephemeral, SignerMode, resolve_signer_mode, build_signer | Missing/bare #[must_use] |
| C-8 | config.rs | has_credentials, has_trust_config, log_level_filter | Bare #[must_use] — no message strings |
| C-9 | config.rs | NSA RTB citation | Imprecise — missing specific principle |

**C-1: validate_config — Bare #[must_use]**

Recommendation: Add a message string:

```rust
#[must_use = "Preflight results must be inspected; discarding them means \
              a misconfigured FIPS algorithm or missing trust anchors \
              would go unreported"]
```

**C-2: creds::validate — Bare #[must_use]**

Recommendation:

```rust
#[must_use = "Credential check results must be inspected; discarding them \
              silently ignores expired certificates or key mismatches"]
```

**C-3: GeneratedCredentials — Not #[must_use]**

Recommendation: Mark the type:

```rust
#[must_use = "Generated key material must be written to disk; discarding \
              this value means the private key is permanently lost"]
pub struct GeneratedCredentials {
```

**C-4: read_chain — Missing #[must_use]**

Recommendation:

```rust
#[must_use = "Trust validation results are returned in ChainEntry::trust_status; \
              discarding the chain means trust decisions are silently bypassed"]
pub fn read_chain(...) -> Result<Vec<ChainEntry>, InspectError> {
```

**C-5: manifest_json, chain_json, has_manifest**

- manifest_json and chain_json: add `#[must_use = "..."]` per the Must-Use Contract Rule.
- has_manifest: replace bare `#[must_use]` with:

```rust
#[must_use = "The presence/absence of a manifest determines whether trust \
              validation and chain walking should proceed"]
```

**C-6: IngestResult, sha256_hex**

- IngestResult: add `#[must_use = "..."]` at the type definition.
- sha256_hex: add `#[must_use = "SHA-256 digest is the integrity reference for the \
  audit log entry; discarding it means no integrity record exists"]`.

**C-7: signer.rs #[must_use] gaps**

- is_ephemeral: replace bare annotation with message.
- SignerMode: add `#[must_use]` at type definition.
- resolve_signer_mode: add `#[must_use = "..."]`.
- build_signer: add `#[must_use = "..."]`.

**C-8: config.rs bare #[must_use] annotations**

Add message strings to has_credentials, has_trust_config, and log_level_filter.

**C-9: NSA RTB citation without principle**

Replace `**NSA RTB**: Trust Gate...` with `**NSA RTB RAIN**: Non-Bypassability —
credential paths are not accessed until explicitly requested...` per Citation Format Rule.

---

### ACCURATE Items

| ID | Description |
|---|---|
| A-1 | lib.rs — complete module doc, #[forbid(unsafe_code)], correct clippy config |
| A-2 | c2pa/mod.rs — complete module doc, NSA RTB RAIN correctly cited with principle |
| A-3 | trust.rs — complete and accurate module doc; build_c2pa_settings has full #[must_use] message; pattern execution measurement in debug mode is correct |
| A-4 | trust.rs — fail-closed I/O behavior: InspectError::Io is returned on unreadable PEM, caller cannot proceed with degraded trust |
| A-5 | error.rs — complete module doc; all error variants carry structured context consistent with AU-3 and SI-10 claims |
| A-6 | signer.rs — FIPS allow-list enforcement via parse_algorithm; ed25519 exclusion is documented with correct technical rationale |
| A-7 | signer.rs — ephemeral cert generation correctly uses FIPS-safe curves; CN clearly marks it as test-only |
| A-8 | ingest.rs — is_umrs_signed guard prevents double-signing (AlreadySigned variant); documented as best-effort |
| A-9 | ingest.rs — SHA-256 computed before signing; pre-signature integrity reference is documented |
| A-10 | ingest.rs — log::info! audit entries do not log key material, credential paths, or classified content; consistent with SI-11 claim |
| A-11 | manifest.rs — TrustStatus enum variants are matchable data; callers can filter by Trusted/Invalid/Revoked programmatically (Security Findings as Data pattern) |
| A-12 | manifest.rs — cycle guard in walk_manifest prevents infinite recursion in malformed manifest stores |
| A-13 | report.rs — reports do not emit key material or credential paths; SI-11 claim is accurate |
| A-14 | manifest.rs — store-level trust propagation is correctly prioritized: tamper detection (mismatch/failed) takes unconditional precedence over trust evaluation |

---

## Remediation Owner Summary

| Owner | Priority | Items |
|---|---|---|
| coder | Immediate | E-1 (validate.rs module doc), E-2 (creds.rs Compliance section), E-3 (ingest_file #[must_use]) |
| coder | High | C-4 (read_chain #[must_use]), C-7 (signer.rs #[must_use] gaps), C-9 (NSA RTB citation) |
| coder | Medium | C-1, C-2, C-3, C-5, C-6, C-8 (remaining bare/missing #[must_use] annotations) |
| coder | Low | All LOW findings (validate.rs types, creds.rs generate fn, verbose.rs compliance note, main.rs module doc, manifest.rs TrustStatus, manifest.rs derive_trust fallback comment) |

---

## Strengths Worth Preserving

**Trust model architecture.** `trust.rs` is the best-annotated module in the crate.
The two-gate design (verify_trust flag, then trust file presence check) is correctly
documented, correctly implemented, and correctly cited. The NSA RTB RAIN citation at
both the module level and function level is properly formed with the principle named.
This module is the template other modules should follow.

**FIPS algorithm enforcement.** The `parse_algorithm` function is the single gate
through which all algorithm strings pass. It is called unconditionally from both
`resolve_signer_mode` and `creds::generate`. The allow-list constant `ALLOWED_ALGORITHMS`
is public and consistently referenced across modules. The FIPS rationale for excluding
ed25519 is documented correctly.

**Security findings as data.** `TrustStatus` is an enum with matchable, serializable,
displayable variants. Chain entries carry trust status as structured data, not log
strings. The `--chain-json` output enables programmatic consumption by other tools.
This is the correct implementation of the Security Findings as Data pattern.

**Fail-closed I/O errors.** When a configured trust anchor PEM file cannot be read,
`build_c2pa_settings` returns `InspectError::Io` immediately. The caller cannot
construct a trust context with degraded or missing anchors. This is correctly documented
as fail-closed behavior.

**Ephemeral mode transparency.** The `is_ephemeral` flag propagates to report output,
log entries, and the IngestResult struct. Operators cannot receive a signing report
without knowing whether an ephemeral cert was used. The CN on ephemeral certs
("UMRS ephemeral — self-signed") is visually unambiguous in validator output.

**Pattern execution measurement.** `trust.rs` implements `#[cfg(debug_assertions)]`
timing measurement for the TrustSettingsBuilder pattern. This is the correct
implementation of the Pattern Execution Measurement Rule.

---

## Gap Analysis Summary

```
Files reviewed: 13
Total findings: 25 (4 HIGH, 12 MEDIUM, 9 LOW)

Uncited security claims:
  - validate.rs: entire module — FIPS algorithm check and trust list preflight
    have no compliance annotations anywhere
  - creds.rs: PKI key generation module has no ## Compliance section

Inconsistencies (code vs. docs):
  - config.rs: "NSA RTB" cited without principle; code implements a trust gate
    pattern (CM-6) and non-bypassability (RAIN) — the citation should specify RAIN
    or be replaced with the appropriate Trust Gate citation

Missing #[must_use] on security-critical items (count by type):
  - Type definitions missing #[must_use]: IngestResult, SignerMode, TrustStatus,
    GeneratedCredentials (4 types)
  - Functions returning Result missing #[must_use]: ingest_file, read_chain,
    manifest_json, chain_json, sha256_hex, resolve_signer_mode, build_signer (7 functions)
  - Bare #[must_use] without message string: validate_config, creds::validate,
    has_manifest, has_credentials, has_trust_config, log_level_filter,
    describe_algorithm, is_ephemeral (8 items)
```
