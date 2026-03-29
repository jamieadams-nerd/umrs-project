// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! UMRS Core ANSI Terminal Utilities
//!
//! ANSI / DEC VT terminal control sequences.
//!
//! This module centralizes all escape sequences used by UMRS console output.
//! It intentionally avoids terminal size queries or async responses.
//!
//! ANSI color definitions and escape sequence helpers for terminal output.
//!
//! References:
//! - ECMA-48 / ISO 6429
//! - DEC VT100 / VT220 Programmer Reference
//! - Xterm Control Sequences
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

/// ESC character
pub const ESC: &str = "\x1b";

/// Control Sequence Introducer
pub const CSI: &str = "\x1b[";

// ===================================================================
//   Text attributes (SGR)
// ===================================================================

/// Reset all attributes
pub const RESET: &str = "\x1b[0m";

pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const BLINK: &str = "\x1b[5m";
pub const REVERSE: &str = "\x1b[7m";
pub const HIDDEN: &str = "\x1b[8m";
pub const STRIKETHROUGH: &str = "\x1b[9m";

// ===================================================================
//   Standard colors (8 / 16)
// ===================================================================

/// Color basics
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
    /// Return the ANSI SGR sequence that starts this color.
    #[inline]
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

    /// Return the ANSI reset sequence.
    #[inline]
    pub fn reset() -> &'static str {
        RESET
    }
}

// -------------------------------------------------------------------
// Raw foreground color constants (for callers that want them)
// -------------------------------------------------------------------

pub const FG_BLACK: &str = "\x1b[30m";
pub const FG_RED: &str = "\x1b[31m";
pub const FG_GREEN: &str = "\x1b[32m";
pub const FG_YELLOW: &str = "\x1b[33m";
pub const FG_BLUE: &str = "\x1b[34m";
pub const FG_MAGENTA: &str = "\x1b[35m";
pub const FG_CYAN: &str = "\x1b[36m";
pub const FG_WHITE: &str = "\x1b[37m";

pub const FG_BRIGHT_BLACK: &str = "\x1b[90m";
pub const FG_BRIGHT_RED: &str = "\x1b[91m";
pub const FG_BRIGHT_GREEN: &str = "\x1b[92m";
pub const FG_BRIGHT_YELLOW: &str = "\x1b[93m";
pub const FG_BRIGHT_BLUE: &str = "\x1b[94m";
pub const FG_BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const FG_BRIGHT_CYAN: &str = "\x1b[96m";
pub const FG_BRIGHT_WHITE: &str = "\x1b[97m";

// ===================================================================
//   Cursor movement & position
// ===================================================================

/// Move cursor to home position (row 1, column 1)
pub const CURSOR_HOME: &str = "\x1b[H";

/// Move cursor to specific row and column (1-based)
#[inline]
pub fn cursor_to(row: usize, col: usize) -> String {
    format!("{CSI}{row};{col}H")
}

/// Move cursor up by n rows
#[inline]
pub fn cursor_up(n: usize) -> String {
    format!("{CSI}{n}A")
}

/// Move cursor down by n rows
#[inline]
pub fn cursor_down(n: usize) -> String {
    format!("{CSI}{n}B")
}

/// Move cursor forward (right) by n columns
#[inline]
pub fn cursor_forward(n: usize) -> String {
    format!("{CSI}{n}C")
}

/// Move cursor backward (left) by n columns
#[inline]
pub fn cursor_back(n: usize) -> String {
    format!("{CSI}{n}D")
}

/// Save cursor position (DEC)
pub const CURSOR_SAVE: &str = "\x1b7";

/// Restore cursor position (DEC)
pub const CURSOR_RESTORE: &str = "\x1b8";

// ===================================================================
//   Screen & line clearing
// ===================================================================

/// Clear entire screen
pub const CLEAR_SCREEN: &str = "\x1b[2J";

/// Clear from cursor to end of screen
pub const CLEAR_SCREEN_DOWN: &str = "\x1b[J";

/// Clear from cursor to beginning of screen
pub const CLEAR_SCREEN_UP: &str = "\x1b[1J";

/// Clear entire current line
pub const CLEAR_LINE: &str = "\x1b[2K";

/// Clear from cursor to end of line
pub const CLEAR_LINE_RIGHT: &str = "\x1b[K";

/// Clear from cursor to beginning of line
pub const CLEAR_LINE_LEFT: &str = "\x1b[1K";

// ===================================================================
//   Cursor visibility & shape (xterm / DEC private modes)
// ===================================================================

/// Hide cursor
pub const CURSOR_HIDE: &str = "\x1b[?25l";

/// Show cursor
pub const CURSOR_SHOW: &str = "\x1b[?25h";

/// Cursor shapes (xterm)
pub const CURSOR_DEFAULT: &str = "\x1b[0 q";
pub const CURSOR_BLINKING_BLOCK: &str = "\x1b[1 q";
pub const CURSOR_STEADY_BLOCK: &str = "\x1b[2 q";
pub const CURSOR_BLINKING_UNDERLINE: &str = "\x1b[3 q";
pub const CURSOR_STEADY_UNDERLINE: &str = "\x1b[4 q";
pub const CURSOR_BLINKING_BAR: &str = "\x1b[5 q";
pub const CURSOR_STEADY_BAR: &str = "\x1b[6 q";

// ===================================================================
//   Truecolor (24-bit RGB)
// ===================================================================

/// Set truecolor foreground
#[inline]
pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("{CSI}38;2;{r};{g};{b}m")
}

/// Set truecolor background
#[inline]
pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("{CSI}48;2;{r};{g};{b}m")
}

// ===================================================================
//   Queries (legacy / optional)
// ===================================================================

/// Device Status Report (cursor position query)
/// Terminal responds with ESC[row;colR
pub const QUERY_CURSOR_POSITION: &str = "\x1b[6n";

/// Device Attributes (DECID)
pub const QUERY_DEVICE_ATTRS: &str = "\x1b[c";

/// Request terminal size in characters (xterm; unreliable)
pub const QUERY_TERM_SIZE_CHARS: &str = "\x1b[18t";

/// Request terminal size in pixels (xterm; unreliable)
pub const QUERY_TERM_SIZE_PIXELS: &str = "\x1b[14t";
