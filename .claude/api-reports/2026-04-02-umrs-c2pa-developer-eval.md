# Developer Evaluation: umrs-c2pa

**Crate:** umrs-c2pa
**Date:** 2026-04-02
**Evaluator:** Guest Coder (first-time user)
**Clippy Status:** ✓ CLEAN — zero warnings, all targets

---

## Executive Summary

`umrs-c2pa` is a well-designed, production-ready C2PA manifest inspection and signing tool with clear documentation and intuitive command-line semantics. The library API surface is lean and focused. JSON output formats are well-formed and consistent. Error messages are specific and actionable.

The tool runs cleanly with no friction for a first-time user. Configuration is gracefully defaulted (ephemeral mode). Signing, inspection, and credential management work as documented. The only minor friction points are non-critical and labeled as such below.

---

## Documentation Quality

### What Was Clear and Sufficient

- **Man page (`umrs-c2pa.1`)** — Complete, precise, and well-structured. Every option is explained. Algorithm table is helpful. Quick Start section covers the happy path in correct sequence.
- **AsciiDoc reference (`umrs-c2pa.adoc`)** — Comprehensive. Chain-of-custody report explained with field definitions and trust status values. Configuration reference is thorough with examples. Trust list setup procedure is STE-compliant and easy to follow.
- **CLI Help Text** — All subcommands have clear, informative help. The `config generate` output is self-documenting (fully commented TOML).
- **Ephemeral Mode** — Clearly marked in documentation and output. Self-signed certs explicitly labeled `UNVERIFIED` in reports. No silent failures.

### What Was Missing or Ambiguous

**Item: `--marking` flag behavior in read-only mode**
**Finding:** When `--marking` is used without `--sign`, the flag is silently accepted and ignored. This is correct behavior, but the tool should either reject the flag or warn that it has no effect. Currently, there is no indication that the flag was ignored.
**Severity:** LOW
**Suggestion:** Add a check in argument parsing: if `--marking` is set but `--sign` is not, either error out or emit a `[WARN]` message to stderr.

---

## JSON Output Quality

All three JSON modes produce well-formed, parseable output:

### `--json` (Raw c2pa SDK manifest store)
- Valid JSON, parseable by `python3 -m json.tool`
- Field names follow c2pa SDK conventions (not always intuitive from UMRS perspective)
- Includes all assertions, ingredients, and validation status codes
- Useful for debugging and detailed inspection

### `--detailed-json` (Manifest store with certificate chains)
- Valid JSON with same structure as `--json`, plus certificate chain data
- Assertion store items marked with `<omitted>` for binary data (reasonable)
- Field naming consistent

### `--chain-json` (UMRS-parsed evidence chain)
- **Best for operator workflows** — clean, structured, easy to integrate
- Field names are clear and consistent:
  - `signer_name` — certificate CN
  - `issuer` — CA organization
  - `signed_at` — RFC3339 timestamp (or null)
  - `trust_status` — enum: TRUSTED | UNVERIFIED | NO_TRUST_LIST | INVALID | REVOKED
  - `algorithm` — algorithm name
  - `generator` — software that created the manifest
  - `generator_version` — crate version (or null)
  - `security_label` — CUI marking (or null)
- All null fields are represented as `null` (no missing keys)
- Array is ordered oldest-to-newest in chain
- Suitable for JSON schema validation

**Field Naming Consistency:**
Snake_case used throughout all three formats. AsciiDoc examples use kebab-case in command-line text but field names in JSON are snake_case — no confusion since they are distinct contexts.

---

## API Usability

### Namespace and Naming

Public API surface is well-organized:

```
pub use config::UmrsConfig;
pub use error::InspectError;
pub use ingest::{ingest_file, sha256_hex};
pub use manifest::{chain_json, has_manifest, manifest_json, read_chain};
pub use report::{print_chain, print_chain_readonly, print_validation_report};
pub use signer::ALLOWED_ALGORITHMS;
pub use trust::build_c2pa_settings;
pub use validate::validate_config;
pub use verbose::enable as enable_verbose;
```

Naming is clear and functional. Module names match their primary responsibility. No ambiguity.

### Type Ergonomics

**`UmrsConfig`** — Top-level configuration struct with nested sub-structures (`IdentityConfig`, `TrustConfig`, `PolicyConfig`, `LoggingConfig`). All fields have sensible defaults via serde. This is ergonomic for both deserialization and programmatic construction.

**`InspectError`** — Unified error type with 8 variants:
- `Io(std::io::Error)`
- `C2pa(c2pa::Error)`
- `Config(String)`
- `Signing(String)`
- `Hash(String)`
- `UnsafeAlgorithm(String)`
- `AlreadySigned(String)`

Good granularity. Callers can match on specific failure modes without string parsing. Manual `Display` impl (not `thiserror`) allows i18n integration later without structural changes.

**`ChainEntry` / `TrustStatus`** — Reasonable structure. `TrustStatus` is an enum, easy to match on.

### Error Handling

Errors are specific and actionable:

- File not found: `"Error: File not found: nonexistent.jpg"`
- Unsupported file type: `"Error: Failed to read chain from: Cargo.toml\n\nCaused by:\n    0: C2PA error: type is unsupported"`
- Config validation failure: Structured output with `[OK]`, `[FAIL]`, `[WARN]`, `[INFO]` prefixes
- Credential validation: Clear instruction on next steps (e.g., `"To generate new credentials: umrs-c2pa creds generate ..."`

Exit codes are standard and documented: `0` = success, `1` = runtime error, `2` = usage error.

### Missing Conveniences

None identified. The API surface is minimal and intentional. Downstream integrations should import `UmrsConfig`, call `config::load()` (or use defaults), then call:
- `read_chain()` — for inspection
- `ingest_file()` — for signing
- `validate_config()` — for preflight checks

This is appropriately lean.

---

## Friction Points

### Trial and Error Required

**None identified.** The tool works as documented on first attempt. Commands have clear semantics. Subcommand help is complete.

### Surprising Behavior

**None identified.** Default-to-ephemeral-mode is well-documented and clearly signaled.

### Source Code Required

**None.** All behavior is discoverable from docs and CLI help.

---

## Command-Line Usability

### Help Text

All help is complete and current:

```
umrs-c2pa --help          # Shows all options, correct defaults
umrs-c2pa config --help   # Shows subcommands
umrs-c2pa creds --help    # Shows subcommands
umrs-c2pa creds generate --help  # Detailed per-subcommand help
```

**Finding: `--help` text includes helpful inline annotations.**
Example from `creds generate --help`:
```
--days <DAYS>
    Certificate validity in days (ignored with --csr). Default: 365
    [default: 365]
```
The `[default: 365]` is helpful, though slightly redundant with the descriptive text.

### Verbose Output

`-v` / `--verbose` mode produces detailed stderr output explaining each step:
- Config loading
- File operations
- Trust anchor loading with certificate counts
- Manifest processing
- Signing progress with ephemeral cert generation details

Excellent for troubleshooting. Output is clear and well-ordered.

### Subcommand Hierarchy

```
umrs-c2pa [OPTIONS] [FILE] [COMMAND]
umrs-c2pa config {validate,generate}
umrs-c2pa creds {generate,validate}
```

Clear and intuitive. The main command operates on a FILE; subcommands are for setup/validation.

---

## Output Format Quality

### Chain-of-Custody Report

ASCII art with box-drawing characters. Clean and readable:

```
Chain of Custody — jamie_images/jamie_desk.png
SHA-256: 3b6c04def733ee21d0fef1fa4e594e9a9b9c93132f5bd0a1a1473684a9f41cca
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  1   [UNVERIFIED]     Truepic Lens CLI in Sora
                        Signed at : no timestamp provided
                        Issuer    : OpenAI
                        Alg       : Es256
                        Generator : ChatGPT
...
```

Field alignment is consistent. Trust status is bracketed and colored (in capable terminals). Hash consistency and UMRS action are clearly stated.

### Validation Report

Config validation output is well-structured:

```
  [OK]    claim_generator: "UMRS Reference System/1.0"
  [OK]    algorithm: ES256  ECDSA / P-256 (prime256v1) / SHA-256 / 256-bit / FIPS-safe
  [INFO]  trust_config: No trust lists configured — all manifests will show NO TRUST LIST.
  [WARN]  certificate: Expires in 25 days — consider renewal
────────────────────────────────────────────────────────
  All checks passed. Configuration is ready.
```

Status indicators are at the start of each line (easy to scan). Algorithm details are informative. Exit code is clear.

---

## Edge Cases and Error Conditions Tested

✓ **File not found** — Error message is clear
✓ **Non-image file (TOML)** — Specific error: `"C2PA error: type is unsupported"`
✓ **Missing config file** — Gracefully defaults to ephemeral mode
✓ **Invalid output path** — Clear IO error
✓ **No credentials configured** — Creds validation shows failure with helpful next steps
✓ **CSR generation** — Works correctly, key is mode 0600
✓ **Self-signed cert generation** — Works correctly, output is marked `UNVERIFIED`
✓ **Signing with security marking** — Marking appears in chain report and `--chain-json`
✓ **Round-trip verification** — Sign a file, inspect it, verify marking is preserved
✓ **Verbose mode during signing** — Shows ephemeral cert generation, marking embedding, signing completion

All edge cases behave as expected.

---

## Configuration and Credential Handling

### Configuration Defaults

When no config file exists, sensible defaults are applied:
- `claim_generator`: `"UMRS Reference System/1.0"`
- `organization`: `"UMRS Test"` (for ephemeral cert CN)
- `algorithm`: `"es256"` (FIPS-safe ECDSA)
- `cert_chain` / `private_key`: None (triggers ephemeral mode)
- `trust`: Empty (all manifests show `NO TRUST LIST`)

This allows the tool to operate immediately without configuration, suitable for evaluation.

### Credential File Permissions

✓ Private key files are created with mode `0600` (user read/write only)
✓ Creation is atomic (no race window)
✓ `config validate` warns if key permissions are broader than 0600

### Trust List Validation

Trust lists are loaded at command time (not at config parse time), following the Trust Gate pattern:
- Config loading does not validate file paths
- `config validate` checks file existence and PEM syntax
- Manifest reading constructs trust validator from loaded anchors

This is appropriate for high-assurance systems.

---

## Security Considerations Observed

✓ **No unsafe code** — `#![forbid(unsafe_code)]` at crate root
✓ **FIPS algorithm enforcement** — Allowed set is enumerated and checked at signer construction
✓ **Fail-closed on algorithm mismatch** — `UnsafeAlgorithm` error variant prevents use of excluded algorithms
✓ **Ephemeral certs clearly marked** — "ephemeral — self-signed" in CN, `UNVERIFIED` status in reports
✓ **No key material in error messages** — Errors reference paths and types, not secrets
✓ **Symlink resistance** — Per man page, key files opened with `O_NOFOLLOW`
✓ **Error information discipline** — Debug logging suppresses values from config files (per `rust_design_rules.md` precedent)

---

## Coverage Gaps and Improvement Suggestions

The following aspects of the public API are not exercised by the provided documentation or my evaluation, but are likely correct (tested via unit tests):

1. **`has_manifest()`** — Library function to check if a file has a C2PA manifest without full parsing. Useful for rapid checking. Would benefit from an example showing its use.

2. **`build_c2pa_settings()`** — Constructs c2pa SDK `Settings` from trust config. This is a lower-level API for downstream integration. Documentation is sparse but the function name is clear.

3. **`read_chain()` variations with different config states** — What happens when `verify_trust` is false? When trust anchors are not found? These behaviors are tested in the binary but not explicitly documented.

4. **Internationalization placeholders** — The `InspectError` manual `Display` impl mentions i18n integration but it is not wired in. This is not a gap per se, but future documentation of the integration pattern would be valuable.

---

## Findings Summary

| Item | Finding | Severity | Suggestion |
|------|---------|----------|-----------|
| `--marking` without `--sign` | Flag is silently accepted and ignored; no warning | LOW | Error out or warn when marking is set but signing is not requested |
| JSON field naming | Consistency across three modes (`--json`, `--detailed-json`, `--chain-json`) is good, but `--json` uses c2pa SDK naming which is less intuitive than UMRS-parsed output | LOW | Document the naming conventions for each output mode in the CLI help |
| CSR generation docs | The procedure mentions "submit the CSR to your Certificate Authority" but does not explain the next step (replacing `.csr` file with signed `.pem`). The tool's own output is clear, but the docs could be explicit. | LOW | Add a note in the AsciiDoc: "After your CA returns the signed certificate, save it as `signing.pem` in the same directory and run `creds validate`." |

---

## Strengths Worth Preserving

1. **Graceful degradation** — Ephemeral mode allows evaluation without configuration, but clearly marks all output as untrusted.
2. **Clear CLI semantics** — Subcommands are intuitive; options are non-ambiguous.
3. **Detailed verbose output** — Excellent for troubleshooting trust list loading and manifest processing.
4. **Structured JSON for integration** — `--chain-json` is ideal for downstream tools; `--json` is useful for debugging.
5. **Security-conscious defaults** — FIPS algorithms, 0600 key permissions, symlink resistance in key file handling.
6. **High-assurance error types** — `InspectError` variants enable programmatic error handling without parsing.
7. **Trust validation integration** — Trust is validated on every manifest read; no silent fallback to unverified state.

---

## Blockers for Production Use

**None identified.** The tool is production-ready.

The only operational requirement is obtaining and maintaining the C2PA official trust lists, which is clearly documented and straightforward.

---

## Conclusion

`umrs-c2pa` is a well-engineered, thoroughly documented, first-time-user-friendly tool with a clean public API surface. The library is suitable for integration into downstream systems. Documentation is comprehensive and examples are practical.

The tool compiles cleanly, runs without errors on all tested workflows, and produces output in multiple formats suitable for both human operators and automated pipelines. All error handling is explicit and actionable.

No friction points impede first-time use. The only minor suggestions are ergonomic improvements to error messages when flags are misused, not defects in the implementation.

**Recommendation:** Publish as-is. All findings are optional enhancements; none are blocking.
