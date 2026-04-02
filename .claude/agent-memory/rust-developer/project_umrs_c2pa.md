---
name: umrs-c2pa crate structure and trust integration
description: Architecture, module map, and trust list wiring for the umrs-c2pa crate
type: project
---

# umrs-c2pa crate

**Path:** `components/rusty-gadgets/umrs-c2pa/`

The crate has its own `Cargo.lock` and `Cargo.toml` (independent lockfile from the workspace). It is included in the workspace `Cargo.toml`.

## Source modules

- `src/lib.rs` — crate root, declares `pub mod c2pa`
- `src/main.rs` — CLI binary, all commands dispatch through here
- `src/c2pa/mod.rs` — re-exports all public API
- `src/c2pa/config.rs` — `UmrsConfig`, `TrustConfig`, `IdentityConfig` etc.
- `src/c2pa/manifest.rs` — `read_chain`, `manifest_json`, `chain_json`, `last_signer`, `has_manifest`
- `src/c2pa/trust.rs` — `build_c2pa_settings()` (Phase 2.2/2.3 implementation)
- `src/c2pa/ingest.rs` — `ingest_file`, `sha256_hex`
- `src/c2pa/error.rs` — `InspectError` enum
- `src/c2pa/signer.rs` — signing key management, FIPS algorithm allow-list
- `src/c2pa/creds.rs` — cert/key generation
- `src/c2pa/validate.rs` — config validation checks
- `src/c2pa/report.rs` — human-readable chain of custody output
- `src/c2pa/verbose.rs` — verbose flag + `verbose!` macro

## Trust list integration (Phase 2.2/2.3)

`trust::build_c2pa_settings(config: &UmrsConfig) -> Result<c2pa::Settings, InspectError>`

- If `verify_trust = false` → returns `Settings::default()` immediately
- If `!has_trust_config()` → returns `Settings::default()` with a `warn!`
- Reads PEM files from `TrustConfig.trust_anchors`, `user_anchors`, `allowed_list`, `trust_config`
- Concatenates trust_anchors + user_anchors into a single PEM bundle for `trust.trust_anchors`
- Uses `Settings::new().with_value("trust.trust_anchors", pem)` builder pattern
- Sets `verify.verify_trust = true` when anchors are loaded

All reader functions (`read_chain`, `manifest_json`, `chain_json`, `last_signer`) now accept `&UmrsConfig` and construct a `c2pa::Context` via `build_c2pa_settings` before opening the file.

## Test fixtures

- `tests/fixtures/test.jpg` — unsigned JPEG
- `tests/fixtures/test_umrs_signed.jpg` — UMRS-signed JPEG
- `tests/fixtures/c2pa-rs/` — CA.jpg, C.jpg, XCA.jpg, CACA.jpg, CA_ct.jpg, no_manifest.jpg, ocsp.jpg, sample1.png
- `config/C2PA-TRUST-LIST.pem` — bundled C2PA official trust anchors
- `config/C2PA-TSA-TRUST-LIST.pem` — bundled TSA trust anchors

## c2pa SDK version

`c2pa = "0.78.6"` (resolves to 0.78.7). The SDK's `Settings::new().with_value(path, value)` builder pattern is the correct approach for trust configuration. Settings take **PEM content strings**, not file paths.

**Why:** `Reader::from_file()` uses thread-local settings. `Reader::from_context(context).with_file(path)` uses the provided context, enabling per-call trust settings without mutating thread-local state.
