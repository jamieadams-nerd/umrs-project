# Rust — Reproducible Builds

Source: https://reproducible-builds.org/docs/rust/
Retrieved: 2026-03-10

---

Rust is a general-purpose programming language with emphasis on performance, type safety, concurrency, and memory safety. The language references build dependencies through cryptographic hashes in lockfiles, making programs often reproducible by default when the original build toolchain version matches and build paths are normalized.

## Dependency Lockfiles

Distributions like Arch Linux, Alpine, and Homebrew depend on upstream projects committing the `Cargo.lock` dependency lockfile to their repositories. While `cargo build` and `cargo build --release` typically respect this file, cargo may re-resolve dependencies using latest semver-compatible versions if the lockfile cannot satisfy requirements.

To enforce strict lockfile compliance, use:

```
cargo build --locked
```

Debian uses its own dependency resolver rather than relying on upstream lockfiles, and records build information in .buildinfo files.

## Diffing the Build Directory

When encountering unreproducible binaries, run diffoscope on the `target/` directory to identify specific problematic binaries:

```
cargo build --release && mv target target.1
cargo build --release && mv target target.2
diffoscope --html diff.html --exclude-directory-metadata=yes target.1/ target.2/
```

Ignore `.rustc_info.json` and `.fingerprint/` directory files during analysis. Differences in `.rlib` files indicate issues within specific crates.

## Build Scripts (`build.rs`)

Build scripts can introduce non-determinism. Refer to the stable outputs page for build script guidance.

## Embedded Build Time

Rust programs rarely record compilation timestamps. When current time is needed, check for the `SOURCE_DATE_EPOCH` environment variable before accessing system time.

## rust-embed

The `rust-embed` crate embeds files into binaries, often recording filesystem metadata including modification times. Disable this using the `deterministic-timestamps` feature:

```toml
[dependencies]
rust-embed = { version = "8.5.0", features = ["deterministic-timestamps"] }
```
