---
name: umrs-c2pa evaluation patterns
description: Notes on API clarity, JSON output, error handling, and first-time UX from umrs-c2pa developer evaluation
type: reference
---

## API Surface Quality

`umrs-c2pa` is a model for lean, focused library design:

- Public API exports 10 functions/types, re-exported from `c2pa::mod`
- Config struct uses nested serde-deserializable substruct pattern (IdentityConfig, TrustConfig, etc.)
- Error type is a hand-written enum, not thiserror macro — allows i18n integration without structural change
- Trust validation happens on manifest read, not config load (Trust Gate pattern)

## JSON Output Patterns

Three output modes, all valid:

1. **`--json`** — Raw c2pa SDK manifest store; use `--detailed-json` for certificate chains
2. **`--chain-json`** — UMRS-parsed, operator-friendly; field names: `signer_name`, `trust_status`, `security_label`
3. **Text report** — ASCII art with box-drawing, readable chain with status badges `[TRUSTED]`, `[UNVERIFIED]`

Field naming is consistent (snake_case in all JSON modes). Null values represented as `null`, not missing keys.

## Graceful Defaults / Ephemeral Mode

Tool runs without any config file present:
- Defaults to ephemeral self-signed cert
- All manifests marked `UNVERIFIED` in output
- Config validation passes with `[INFO]` flags, not errors
- Allows immediate evaluation without setup

This is a UX pattern worth preserving in other tools.

## Error Handling

Manual error type with specific variants (`UnsafeAlgorithm`, `AlreadySigned`, `Config`, `Signing`, `Hash`, `Io`, `C2pa`). Enables:

- Programmatic error matching (not string parsing)
- Specific messages without exposing key material
- Integration with i18n layer (gettext insertion in Display impl)

Exit codes: 0 = success, 1 = runtime error, 2 = usage error (per bash rules).

## Command-Line Usability

- Subcommand hierarchy is intuitive: main command takes FILE; `config` and `creds` subcommands for setup
- All help text is current and complete
- Verbose mode (`-v`) shows step-by-step stderr (config loading, trust anchor counts, manifest processing)
- No hidden defaults; ephemeral mode is explicitly marked in output

## First-Time Friction Points

None significant. Tool works as documented on first attempt.

Minor (LOW severity) findings:
- `--marking` flag silently ignored if `--sign` is not used (should warn or error)
- CSR → signed cert handoff process could be more explicit in docs
- `--json` vs `--chain-json` purpose could be clearer in help text

## Security Patterns

- `#![forbid(unsafe_code)]` at crate root
- FIPS algorithm enforcement via allow-list enum
- Private key files created mode 0600 atomically (no race)
- Key paths validated at validation time, not config load (Trust Gate)
- Error messages do not leak key material or classified content
- Symlink resistance on key file operations (`O_NOFOLLOW`)

## Tests Worth Noting

Tool was evaluated with:
- Image files with existing C2PA manifests (Adobe, ChatGPT/Sora)
- Files without manifests
- Non-image files (proper error: `type is unsupported`)
- Missing files (`File not found` error)
- Signing with security markings (appears in --chain-json)
- Round-trip: sign → inspect → verify marking preserved
- Verbose mode during signing (shows ephemeral cert generation)
- Credential generation (--csr mode works, keys are 0600)
- Config validation with missing files (graceful defaults)

All workflows produced correct output without trial-and-error.

## Code Quality

Clippy: zero warnings, all targets. No unsafe code. Well-structured module hierarchy. Module-level doc comments with compliance citations (NIST SP 800-53, SC-13, etc.).
