---
name: umrs-c2pa public API surface
description: Key types, functions, and signatures in umrs-c2pa after the SHA-384 / Algorithm enrichment changes
type: project
---

## IngestResult fields (ingest.rs)

- `source_path: PathBuf`
- `output_path: PathBuf`
- `sha256: String` — 64 hex chars, computed via system OpenSSL
- `sha384: String` — 96 hex chars, computed via system OpenSSL (CNSA 2.0)
- `had_manifest: bool`
- `action: String`
- `previous_signer: Option<String>`
- `previous_signed_at: Option<String>`
- `is_ephemeral: bool`

## Public hash functions (ingest.rs)

- `sha256_hex(path: &Path) -> Result<String, InspectError>` — system OpenSSL SHA-256
- `sha384_hex(path: &Path) -> Result<String, InspectError>` — system OpenSSL SHA-384

## Report functions (report.rs)

- `print_chain(path, sha256, sha384, chain, ingest)` — full chain report to stdout
- `print_chain_readonly(path, sha256, sha384, chain)` — read-only chain report

## Manifest JSON functions (manifest.rs)

- `chain_json(path, config)` — returns JSON array of ChainEntry (no file hashes)
- `chain_report_json(path, sha256, sha384, config)` — returns JSON object with sha256, sha384, chain fields

## Algorithm description (signer.rs)

- `describe_algorithm(alg: &str) -> &'static str` — expects lowercase alg string ("es256", not "Es256")
- Returns rich description e.g. "ES256  ECDSA / P-256 (prime256v1) / SHA-256 / 256-bit / FIPS-safe"
- Returns "(unknown algorithm)" for unknown input

## Dependency notes

- `sha2` crate removed; SHA digests now computed via `openssl::hash::{hash, MessageDigest}`
- `openssl` was already a direct dep (ephemeral cert generation)

**Why:** FIPS 140-2/3 compliance on RHEL 10 requires system OpenSSL, not a pure-Rust sha2 implementation.
