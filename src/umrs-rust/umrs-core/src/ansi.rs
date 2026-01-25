// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! UMRS Core ANSI Terminal Utilities
//!
//! ANSI color definitions and escape sequence helpers for terminal output.
//!
//! Guarantees:
//! - Deterministic mapping of color names to ANSI escape codes
//! - No side effects beyond returning static escape sequences
//! - Minimal, dependency-free implementation suitable for CLI formatting
//!
//! Non-goals:
//! - Terminal capability probing or feature detection
//! - Full color system abstraction beyond basic ANSI escape codes
//! - Cross-platform console API support outside ANSI terminals
//
#[derive(Clone, Copy, Debug)]
pub enum AnsiColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl AnsiColor {
    pub fn start(self) -> &'static str {
        match self {
            AnsiColor::Red => "\x1b[31m",
            AnsiColor::Green => "\x1b[32m",
            AnsiColor::Yellow => "\x1b[33m",
            AnsiColor::Blue => "\x1b[34m",
            AnsiColor::Magenta => "\x1b[35m",
            AnsiColor::Cyan => "\x1b[36m",
            AnsiColor::Gray => "\x1b[90m",
            AnsiColor::BrightRed => "\x1b[91m",
            AnsiColor::BrightGreen => "\x1b[92m",
            AnsiColor::BrightYellow => "\x1b[93m",
            AnsiColor::BrightBlue => "\x1b[94m",
            AnsiColor::BrightMagenta => "\x1b[95m",
            AnsiColor::BrightCyan => "\x1b[96m",
            AnsiColor::BrightWhite => "\x1b[97m",
        }
    }

    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
}
