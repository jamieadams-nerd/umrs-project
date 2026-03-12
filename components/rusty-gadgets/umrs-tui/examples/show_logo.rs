// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// NIST 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — this is a
// compile-time proof, not a policy.
#![forbid(unsafe_code)]

//! UMRS TUI — wizard art viewer.
//!
//! Renders the built-in braille wizard art with configurable terminal
//! justification. Uses `COLUMNS` environment variable for terminal width
//! detection; falls back to 80 columns when the variable is absent or
//! unparseable.
//!
//! Usage:
//!   umrs-tui [--justify left|right] [-j left|right]
//!
//! Options:
//!   -j, --justify <left|right>   Justify art to the left or right (default: left)

use umrs_core::robots::{WIZARD_MEDIUM, WIZARD_SMALL};

// ---------------------------------------------------------------------------
// Justification
// ---------------------------------------------------------------------------

/// Output justification for the wizard art display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Justify {
    Left,
    Right,
}

// ---------------------------------------------------------------------------
// Argument parsing
// ---------------------------------------------------------------------------

/// Parse `--justify` / `-j` from the argument list.
///
/// Returns `Err(String)` with a usage message on unrecognized arguments or
/// invalid values. Unrecognized flags are rejected — the binary has a single
/// purpose and a minimal contract.
fn parse_args() -> Result<Justify, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut justify = Justify::Left;
    let mut i = 1usize;

    while i < args.len() {
        let arg = args.get(i).map(String::as_str).unwrap_or("");
        match arg {
            "--justify" | "-j" => {
                i = i.saturating_add(1);
                let value = args.get(i).map(String::as_str).unwrap_or("");
                match value {
                    "left" => justify = Justify::Left,
                    "right" => justify = Justify::Right,
                    other => {
                        return Err(format!(
                            "unknown justification '{}'; expected 'left' or 'right'",
                            other
                        ));
                    }
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => {
                return Err(format!(
                    "unrecognized argument '{}'\nRun with --help for usage.",
                    other
                ));
            }
        }
        i = i.saturating_add(1);
    }

    Ok(justify)
}

fn print_usage() {
    println!("umrs-tui — UMRS wizard art viewer");
    println!();
    println!("USAGE:");
    println!("    umrs-tui [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -j, --justify <left|right>   Justify output (default: left)");
    println!("    -h, --help                   Print this help");
}

// ---------------------------------------------------------------------------
// Terminal width
// ---------------------------------------------------------------------------

/// Determine terminal width.
///
/// Reads the `COLUMNS` environment variable and parses it as a `usize`.
/// Falls back to 80 if the variable is absent, empty, or non-numeric.
fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(80)
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Compute the left-padding string for right-justification.
///
/// Returns a `String` of spaces sized so the art ends at `term_width`.
/// Saturating subtraction prevents underflow when `art_width >= term_width`.
fn right_pad(term_width: usize, art_width: usize) -> String {
    let spaces = term_width.saturating_sub(art_width);
    " ".repeat(spaces)
}

/// Print one piece of wizard art with the requested justification.
fn print_art(art: &umrs_core::robots::AsciiArtStatic, justify: Justify, term_width: usize) {
    let pad = if justify == Justify::Right {
        right_pad(term_width, art.width)
    } else {
        String::new()
    };

    for line in art.lines {
        println!("{}{}", pad, line);
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let justify = match parse_args() {
        Ok(j) => j,
        Err(msg) => {
            eprintln!("error: {msg}");
            std::process::exit(1);
        }
    };

    let term_width = terminal_width();

    print_art(&WIZARD_MEDIUM, justify, term_width);
    println!();
    print_art(&WIZARD_SMALL, justify, term_width);
}
