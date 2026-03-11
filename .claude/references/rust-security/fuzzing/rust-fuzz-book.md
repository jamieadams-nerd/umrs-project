# Rust Fuzz Book

Source: https://rust-fuzz.github.io/book/print.html
GitHub: https://github.com/rust-fuzz/book
Retrieved: 2026-03-10

---

Fuzz testing is a software testing technique that identifies security and stability issues by supplying pseudo-random data as input to software.

Two primary tools covered: **cargo-fuzz** (libFuzzer) and **afl.rs** (AFL/AFL++).

---

## 1. Fuzzing with cargo-fuzz

cargo-fuzz invokes libFuzzer through the libfuzzer-sys crate.

### Requirements

- x86-64 Linux, x86-64 macOS, Apple Silicon macOS, or Windows (MSVC AddressSanitizer)
- C++ compiler with C++11 support
- **Nightly Rust required** (due to `-Z` compiler flags for address sanitization)

```bash
rustup install nightly && rustup default nightly
cargo install cargo-fuzz
```

### Basic Fuzz Target

```rust
#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate url;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = url::Url::parse(s);
    }
});
```

Run:
```bash
cargo fuzz run <fuzz_target_name>
```

### Cargo Features

```bash
cargo fuzz run <target> --features=feature_name
cargo fuzz run <target> --no-default-features
```

### `#[cfg(fuzzing)]`

All instrumented crates compile with `--cfg fuzzing`, enabling code path management:

```rust
#[cfg(not(fuzzing))]
fn verify_signature() { /* ... */ }
```

### libFuzzer Configuration

```bash
cargo fuzz run <target> -- -help=1
cargo fuzz run <target> -- -only_ascii=1
```

---

## Structure-Aware Fuzzing

Use any type implementing the `Arbitrary` trait instead of raw bytes:

```rust
libfuzzer_sys::fuzz_target!(|input: MyType| {
    // Use input here
});
```

Enable `#[derive(Arbitrary)]`:

```toml
[dependencies]
arbitrary = { version = "1", optional = true, features = ["derive"] }
```

```rust
#[derive(Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

---

## Writing Oracles That Access Data

**Bad harness** (misses invalid memory):
```rust
fuzz_target!(|data: &[u8]| {
    api_with_callback(data, |lib_data| {});  // Doesn't access lib_data
});
```

**Good harness** (validates all references):
```rust
fuzz_target!(|data: &[u8]| {
    api_with_callback(data, |lib_data| {
        lib_data.iter().for_each(|byte_ref| {
            core::hint::black_box(*byte_ref);  // Forces memory access
        });
    });
});
```

`black_box` prevents optimization from removing the access, allowing sanitizers to catch invalid memory.

---

## Code Coverage

```bash
cargo fuzz coverage <target> [corpus dirs]
```

Visualize as HTML:
```bash
cargo cov -- show fuzz/target/<triple>/release/<target> \
    --format=html \
    -instr-profile=fuzz/coverage/<target>/coverage.profdata \
    > index.html
```

---

## CI Integration (GitHub Actions)

```yaml
name: Smoke-Test Fuzz Targets
on:
  push:
    branches: ["main"]
  pull_request:
    branches: ["main"]

jobs:
  fuzz:
    runs-on: ubuntu-latest
    env:
      CARGO_FUZZ_VERSION: 0.12.0
      FUZZ_TIME: 300
    strategy:
      matrix:
        include:
          - fuzz_target: my_first_fuzz_target
          - fuzz_target: my_second_fuzz_target
    steps:
    - uses: actions/checkout@v4
    - run: rustup toolchain install nightly && rustup default nightly
    - uses: actions/cache@v4
      with:
        path: ${{ runner.tool_cache }}/cargo-fuzz
        key: cargo-fuzz-bin-${{ env.CARGO_FUZZ_VERSION }}
    - run: cargo install --root "${{ runner.tool_cache }}/cargo-fuzz" --version ${{ env.CARGO_FUZZ_VERSION }} cargo-fuzz --locked
    - run: cargo fuzz build ${{ matrix.fuzz_target }}
    - run: cargo fuzz run ${{ matrix.fuzz_target }} -- -max_total_time=${{ env.FUZZ_TIME }}
    - uses: actions/upload-artifact@v4
      if: failure()
      with:
        name: fuzzing-artifacts-${{ matrix.fuzz_target }}-${{ github.sha }}
        path: fuzz/artifacts
```

---

## 2. Fuzzing with afl.rs

**American fuzzy lop** (AFL) — coverage-guided fuzzer.

### Setup

```bash
cargo install cargo-afl
```

### Fuzz Target

```rust
#[macro_use]
extern crate afl;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = url::Url::parse(&s);
        }
    });
}
```

### Build and Run

```bash
cargo afl build
mkdir in && echo "http://example.com/" > in/url
cargo afl fuzz -i in -o out target/debug/url-fuzz-target
```

### Reproduce Crashes

```bash
cargo afl run url-fuzz-target < out/default/crashes/crash_file
```

---

## Trophy Case

Community-maintained list of bugs found via fuzzing: https://github.com/rust-fuzz/trophy-case
