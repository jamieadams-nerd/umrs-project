# clap — Command Line Argument Parser

Source: https://docs.rs/clap/latest/clap/
Source: https://docs.rs/clap/latest/clap/_derive/index.html
Source: https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
Retrieved: 2026-03-15

## Overview

`clap` is a full-featured, fast command line argument parser for Rust. The Derive API is the
recommended approach: annotate structs and enums with procedural macros and clap generates the
builder code at compile time. Provides compile-time validation, `--help` generation, shell
completion, and error messages with suggestions.

## Adding to Cargo.toml

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Feature flags of note:
- `derive` — enables `#[derive(Parser)]`, `#[derive(Args)]`, etc.
- `env` — allows `#[arg(env = "VAR")]` to read from environment variables
- `wrap_help` — wraps help text at terminal width
- `unicode` — Unicode-aware help text wrapping
- `cargo` — reads `Cargo.toml` metadata for version/author

## Core Derive Macros

| Macro          | Annotates  | Purpose                                                |
|----------------|------------|--------------------------------------------------------|
| `Parser`       | struct     | Top-level CLI definition; has `parse()` method         |
| `Args`         | struct     | Reusable group of arguments; used with `flatten`       |
| `Subcommand`   | enum       | Enum where each variant is a subcommand                |
| `ValueEnum`    | enum       | Enum restricting an argument to a fixed set of values  |

## Basic Parser

```rust
use clap::Parser;

/// A security posture checker
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target path to inspect
    path: std::path::PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output format
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    let cli = Cli::parse();
    println!("path: {:?}, verbose: {}", cli.path, cli.verbose);
}
```

Doc comments become `--help` text. `#[command(version)]` reads from `Cargo.toml`.

## Subcommand Pattern

```rust
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspect a file's security context
    Inspect(InspectArgs),
    /// Show system SELinux status
    Status,
    /// Run a posture check
    Check {
        /// Check category
        #[arg(value_enum)]
        category: CheckCategory,
    },
}

#[derive(Args)]
struct InspectArgs {
    /// File to inspect
    path: std::path::PathBuf,

    /// Show extended attributes
    #[arg(short = 'x', long)]
    xattrs: bool,
}
```

### Flatten — Reusable Arg Groups

```rust
#[derive(Args)]
struct CommonArgs {
    #[arg(short, long)]
    verbose: bool,

    #[arg(long, default_value = "text")]
    format: String,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    common: CommonArgs,

    #[command(subcommand)]
    command: Commands,
}
```

`#[command(flatten)]` inlines the flattened struct's arguments as peers of the parent's args.
Only `next_help_heading` is supported as an attribute alongside `flatten`.

### Nested Subcommands

```rust
#[derive(Subcommand)]
enum Commands {
    /// Policy management
    Policy {
        #[command(subcommand)]
        action: PolicyAction,
    },
}

#[derive(Subcommand)]
enum PolicyAction {
    Load { path: std::path::PathBuf },
    Show,
}
```

## `ValueEnum` — Restricted Values

```rust
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Human-readable text
    Text,
    /// JSON for machine consumption
    Json,
    /// Compact CSV
    Csv,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}
```

Invalid values produce a graceful error listing valid options. Doc comments become per-value
help text.

## Argument Validation

### Built-in Type Parsing

clap automatically validates and parses into the field's Rust type:
- `u16`, `i32`, `f64` — numeric types with parse error on invalid input
- `PathBuf` — no validation; path existence must be checked manually
- `String` — always valid

### Range Validation with `value_parser`

```rust
#[arg(value_parser = clap::value_parser!(u16).range(1..=65535))]
port: u16,
```

### Custom Validation Function

```rust
fn parse_port(s: &str) -> Result<u16, String> {
    let port: u16 = s.parse().map_err(|_| format!("`{s}` is not a valid port"))?;
    if port < 1024 {
        return Err(format!("port {port} is reserved; use 1024 or higher"));
    }
    Ok(port)
}

#[arg(value_parser = parse_port)]
port: u16,
```

### Environment Variable Fallback

```rust
#[arg(long, env = "UMRS_FORMAT")]
format: String,
```

Precedence: CLI flag > environment variable > default value.

## Help Text Formatting

- Doc comments (`///`) on structs become the command `about` text
- Doc comments on fields become per-argument help text
- Doc comments on enum variants become per-subcommand or per-value help text
- First line of a doc comment is the short description; full comment is `long_about`
- `#[command(long_about = "...")]` for multi-paragraph long descriptions
- `#[arg(help = "...")]` overrides the doc comment for a specific argument

### Help Headings

```rust
#[derive(Args)]
struct Cli {
    #[arg(long, help_heading = "Output Options")]
    json: bool,

    #[arg(long, help_heading = "Output Options")]
    color: bool,
}
```

## JSON Output Flag Pattern (UMRS convention)

All UMRS commands that return structured data must provide `--json`:

```rust
#[derive(Parser)]
struct Cli {
    /// Output as JSON for machine consumption
    #[arg(long)]
    json: bool,
}
```

## `arg_required_else_help`

Print help when a subcommand or required argument is missing, rather than a bare error:

```rust
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli { ... }

// Or per-subcommand:
#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Clone { remote: String },
}
```

## Cookbook Examples (docs.rs)

| Example           | Pattern                                              |
|-------------------|------------------------------------------------------|
| `git_derive`      | git-like multi-subcommand CLI, Derive API            |
| `multicall_busybox` | single binary, multiple tool names via argv[0]     |
| `repl_derive`     | REPL loop with clap-parsed commands                  |
| `pacman`          | combined flags and subcommands (pacman-style)        |

Links:
- https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
- https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
