// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

use umrs_core::console::ansi::*;

// =====================================================================
//   Constants & basic invariants
// =====================================================================

#[test]
fn esc_and_csi_constants() {
    assert_eq!(ESC, "\x1b");
    assert_eq!(CSI, "\x1b[");
}

#[test]
fn reset_constant() {
    assert_eq!(RESET, "\x1b[0m");
}

// =====================================================================
//   AnsiColor mapping
// =====================================================================

#[test]
fn ansi_color_start_codes() {
    assert_eq!(AnsiColor::Red.start(), "\x1b[31m");
    assert_eq!(AnsiColor::Green.start(), "\x1b[32m");
    assert_eq!(AnsiColor::Yellow.start(), "\x1b[33m");
    assert_eq!(AnsiColor::Blue.start(), "\x1b[34m");
    assert_eq!(AnsiColor::Magenta.start(), "\x1b[35m");
    assert_eq!(AnsiColor::Cyan.start(), "\x1b[36m");
    assert_eq!(AnsiColor::Gray.start(), "\x1b[90m");

    assert_eq!(AnsiColor::BrightRed.start(), "\x1b[91m");
    assert_eq!(AnsiColor::BrightGreen.start(), "\x1b[92m");
    assert_eq!(AnsiColor::BrightYellow.start(), "\x1b[93m");
    assert_eq!(AnsiColor::BrightBlue.start(), "\x1b[94m");
    assert_eq!(AnsiColor::BrightMagenta.start(), "\x1b[95m");
    assert_eq!(AnsiColor::BrightCyan.start(), "\x1b[96m");
    assert_eq!(AnsiColor::BrightWhite.start(), "\x1b[97m");
}

#[test]
fn ansi_color_reset_is_global_reset() {
    assert_eq!(AnsiColor::reset(), RESET);
}

// =====================================================================
//   Text attributes (SGR)
// =====================================================================

#[test]
fn text_attribute_constants() {
    assert_eq!(BOLD, "\x1b[1m");
    assert_eq!(DIM, "\x1b[2m");
    assert_eq!(ITALIC, "\x1b[3m");
    assert_eq!(UNDERLINE, "\x1b[4m");
    assert_eq!(BLINK, "\x1b[5m");
    assert_eq!(REVERSE, "\x1b[7m");
    assert_eq!(HIDDEN, "\x1b[8m");
    assert_eq!(STRIKETHROUGH, "\x1b[9m");
}

// =====================================================================
//   Cursor movement
// =====================================================================

#[test]
fn cursor_positioning_sequences() {
    assert_eq!(CURSOR_HOME, "\x1b[H");
    assert_eq!(cursor_to(1, 1), "\x1b[1;1H");
    assert_eq!(cursor_to(10, 20), "\x1b[10;20H");
}

#[test]
fn cursor_movement_sequences() {
    assert_eq!(cursor_up(3), "\x1b[3A");
    assert_eq!(cursor_down(5), "\x1b[5B");
    assert_eq!(cursor_forward(7), "\x1b[7C");
    assert_eq!(cursor_back(9), "\x1b[9D");
}

#[test]
fn cursor_save_restore() {
    assert_eq!(CURSOR_SAVE, "\x1b7");
    assert_eq!(CURSOR_RESTORE, "\x1b8");
}

// =====================================================================
//   Screen & line clearing
// =====================================================================

#[test]
fn screen_clear_sequences() {
    assert_eq!(CLEAR_SCREEN, "\x1b[2J");
    assert_eq!(CLEAR_SCREEN_DOWN, "\x1b[J");
    assert_eq!(CLEAR_SCREEN_UP, "\x1b[1J");
}

#[test]
fn line_clear_sequences() {
    assert_eq!(CLEAR_LINE, "\x1b[2K");
    assert_eq!(CLEAR_LINE_RIGHT, "\x1b[K");
    assert_eq!(CLEAR_LINE_LEFT, "\x1b[1K");
}

// =====================================================================
//   Cursor visibility & shape
// =====================================================================

#[test]
fn cursor_visibility_sequences() {
    assert_eq!(CURSOR_HIDE, "\x1b[?25l");
    assert_eq!(CURSOR_SHOW, "\x1b[?25h");
}

#[test]
fn cursor_shape_sequences() {
    assert_eq!(CURSOR_DEFAULT, "\x1b[0 q");
    assert_eq!(CURSOR_BLINKING_BLOCK, "\x1b[1 q");
    assert_eq!(CURSOR_STEADY_BLOCK, "\x1b[2 q");
    assert_eq!(CURSOR_BLINKING_UNDERLINE, "\x1b[3 q");
    assert_eq!(CURSOR_STEADY_UNDERLINE, "\x1b[4 q");
    assert_eq!(CURSOR_BLINKING_BAR, "\x1b[5 q");
    assert_eq!(CURSOR_STEADY_BAR, "\x1b[6 q");
}

// =====================================================================
//   Truecolor (24-bit RGB)
// =====================================================================

#[test]
fn truecolor_foreground_sequence() {
    assert_eq!(fg_rgb(255, 0, 0), "\x1b[38;2;255;0;0m");
    assert_eq!(fg_rgb(0, 255, 0), "\x1b[38;2;0;255;0m");
    assert_eq!(fg_rgb(0, 0, 255), "\x1b[38;2;0;0;255m");
}

#[test]
fn truecolor_background_sequence() {
    assert_eq!(bg_rgb(10, 20, 30), "\x1b[48;2;10;20;30m");
}

// =====================================================================
//   Queries (legacy / optional)
// =====================================================================

#[test]
fn query_sequences() {
    assert_eq!(QUERY_CURSOR_POSITION, "\x1b[6n");
    assert_eq!(QUERY_DEVICE_ATTRS, "\x1b[c");
    assert_eq!(QUERY_TERM_SIZE_CHARS, "\x1b[18t");
    assert_eq!(QUERY_TERM_SIZE_PIXELS, "\x1b[14t");
}
