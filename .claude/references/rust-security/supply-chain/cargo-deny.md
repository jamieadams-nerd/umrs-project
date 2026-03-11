# cargo-deny

Source: https://github.com/EmbarkStudios/cargo-deny (README)
Retrieved: 2026-03-10

---

A Cargo plugin from Embark Studios for dependency linting. Performs security and compliance checks on Rust project dependencies.

## Key Features

Four main verification categories:

1. **Licenses** — Validates that every dependency has acceptable license terms
2. **Bans** — Allows denying or allowing specific crates; detects duplicate versions
3. **Advisories** — Identifies known security issues by querying an advisory database
4. **Sources** — Ensures crates originate only from trusted repositories

## Quick Setup

```bash
cargo install --locked cargo-deny && cargo deny init && cargo deny check
```

The tool can also run standalone (without Cargo installed) via the `standalone` feature, useful for containerized environments.

## Integration Options

- **Pre-commit hooks** — For automated checking before commits
- **GitHub Actions** — Via the `cargo-deny-action` repository

## Important Disclaimer

"No functionality in - or information provided by - cargo-deny constitutes legal advice."

## Licensing

Dual-licensed under Apache 2.0 and MIT.
