# cargo-audit

Source: https://github.com/RustSec/rustsec (cargo-audit/README.md)
Retrieved: 2026-03-10

---

`cargo audit` is a Cargo subcommand that scans project dependencies against the RustSec Advisory Database to identify security vulnerabilities in Rust crates.

## Key Requirements & Installation

Requires Rust 1.74 or later. Installation:
```
cargo install cargo-audit --locked
```

Packages also available for Alpine Linux, Arch Linux, macOS, and OpenBSD.

## Main Features

**Three Primary Subcommands**:
1. **`cargo audit`** — Standard vulnerability scanning
2. **`cargo audit fix`** — Experimental; automatically updates `Cargo.toml`; use `--dry-run` for preview
3. **`cargo audit bin`** — Audits compiled binaries; enhanced accuracy when built with `cargo auditable`

## Advisory Management

Users can ignore specific advisories using the `--ignore` flag when upgrades are unavailable and vulnerabilities don't affect their application. Configuration can be managed through an `audit.toml` file.

## CI/CD Integration

Includes guidance for Travis CI and the `audit-check` action for GitHub Actions.

## Licensing

Licensed under Apache 2.0 or MIT.
