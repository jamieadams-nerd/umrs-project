# color-eyre — Error Reporting

Source: https://docs.rs/color-eyre/latest/color_eyre/
Retrieved: 2026-03-15

## Overview

`color-eyre` is an error report handler for `eyre` that provides colorized, human-readable
error reports with optional backtraces and span traces. It is the standard error handling
crate for UMRS TUI binaries.

## Adding to Cargo.toml

```toml
[dependencies]
color-eyre = "0.6"
eyre = "0.6"
```

## Feature Flags

| Feature      | Default | Description                                      |
|--------------|---------|--------------------------------------------------|
| `track-caller` | yes   | Captures call-site location for better reports   |
| `issue-url`  | no      | Adds a URL to file bug reports in error output   |
| `url`        | yes     | Enables URL display in reports                   |

## Core Types

### `color_eyre::Report`

The `Report` type is `color_eyre`'s implementation of `eyre::Report` — an opaque error container
that wraps any `std::error::Error + Send + Sync + 'static`. Used as the `Err` variant in
`eyre::Result<T>`.

### `eyre::Result<T>`

A type alias: `type Result<T, E = Report> = std::result::Result<T, E>`.

Import it as:
```rust
use color_eyre::eyre::Result;
// or equivalently:
use color_eyre::Result;
```

## Installation — `color_eyre::install()`

Must be called once, at the very start of `main()`, before any `?`-propagated errors can occur.
Sets up the global panic and error report handlers.

```rust
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    // rest of application ...
    Ok(())
}
```

Placement rule: `install()` must come before any `?` usage that could return a `Report`. If
called after the first error is constructed, some report context may be missing.

`install()` sets:
- The `eyre` hook: formats `Report` values with colors, causes, and optional backtraces
- The panic hook: formats panics with colors and backtraces

## Adding Context to Errors

`eyre::WrapErr` / `color_eyre::eyre::WrapErr` adds human-readable context strings:

```rust
use color_eyre::eyre::{Result, WrapErr};
use std::fs;

fn load_config(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .wrap_err_with(|| format!("failed to read config file: {path}"))?;
    Ok(String::new())
}
```

`wrap_err(msg)` — static message string
`wrap_err_with(|| ...)` — closure for lazy formatting

## Section Helpers

`color_eyre` provides section helpers to attach additional structured context to reports,
displayed as labeled sections in the error output:

```rust
use color_eyre::eyre::{Result, eyre};
use color_eyre::Section;

fn check_env() -> Result<()> {
    let val = std::env::var("REQUIRED_VAR")
        .map_err(|_| eyre!("missing required environment variable"))
        .with_suggestion(|| "set REQUIRED_VAR before running this command")
        .with_note(|| "this variable controls X behavior")?;
    Ok(())
}
```

Section methods (from `color_eyre::Section` trait):
- `.with_note(|| ...)` — appends a note section to the report
- `.with_warning(|| ...)` — appends a warning section
- `.with_suggestion(|| ...)` — appends an actionable suggestion
- `.with_error(|| ...)` — appends a related error
- `.section(header, body)` — appends a custom labeled section

## Backtrace and SpanTrace Control

Backtraces are controlled via `RUST_BACKTRACE` / `RUST_LIB_BACKTRACE` environment variables:
- `RUST_BACKTRACE=1` — enable backtraces
- `RUST_BACKTRACE=full` — enable full (unfiltered) backtraces

SpanTrace (tracing-error integration) is captured automatically if `tracing-error` is enabled
and a subscriber is active.

## `eyre::bail!` and `eyre::ensure!`

```rust
use color_eyre::eyre::{bail, ensure, Result};

fn validate(x: i32) -> Result<()> {
    ensure!(x > 0, "x must be positive, got {x}");
    if x > 1000 {
        bail!("x exceeds maximum allowed value");
    }
    Ok(())
}
```

## Typical TUI Binary Pattern

```rust
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame))?;
        if matches!(crossterm::event::read()?, crossterm::event::Event::Key(_)) {
            break;
        }
    }
    Ok(())
}
```
