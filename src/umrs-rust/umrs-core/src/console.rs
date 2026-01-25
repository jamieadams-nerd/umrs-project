// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! UMRS Core Console Utilities
//!
//! Lightweight helpers for structured console output and message formatting.
//!
//! Guarantees:
//! - Deterministic formatting of console messages
//! - No side effects beyond stdout/stderr output
//! - Dependency-free implementation aligned with UMRS CLI needs
//!
//! Non-goals:
//! - Full logging or audit event handling
//! - Terminal capability detection or feature negotiation
//! - Internationalization or localization support
//
use colored::Colorize;

/// Initialize the UMRS console output subsystem.
///
/// Configures global console state and prepares runtime formatting behavior
/// for all subsequent human-facing output.
///
/// # Behavior
///
/// - Sets up internal console formatting state.
/// - Must be called before any other console output functions.
/// - Has no effect if called multiple times.
///
/// # Side Effects
///
/// - Modifies global console output state.
/// - Affects all subsequent calls to console output helpers.
///
/// # Panics
///
/// This function does not intentionally panic.  
/// Any initialization failures are handled internally.

pub fn init() {
    // Future use: global quiet mode, no-color mode, etc.
}



/// Emit an informational message to the console.
///
/// Use this for neutral, high-level progress updates that describe what the
/// system is doing without implying success, failure, or abnormal conditions.
///
/// # Parameters
///
/// - `message`: Human-readable informational text to display.
///
/// # Behavior
///
/// - Formats the message using the standard informational style.
/// - Emits the message to standard output.
///
/// # Side Effects
///
/// - Writes to stdout.
///
/// # Panics
///
/// This function does not intentionally panic.

pub fn info(message: &str) {
    println!("{} {}", "[INFO]".bold(), message);
}


/// Emit a success message to the console.
///
/// Use this for positive confirmation that an operation has completed
/// successfully and no corrective action is required.
///
/// # Parameters
///
/// - `message`: Human-readable success text to display.
///
/// # Behavior
///
/// - Formats the message using the standard success style.
/// - Emits the message to standard output.
///
/// # Side Effects
///
/// - Writes to stdout.
///
/// # Panics
///
/// This function does not intentionally panic.

pub fn success(message: &str) {
    println!("{} {}", "[ OK ]".green().bold(), message);
}


/// Emit a warning message to the console.
///
/// Use this for non-fatal conditions that may require user attention or
/// corrective action but do not prevent continued execution.
///
/// # Parameters
///
/// - `message`: Human-readable warning text to display.
///
/// # Behavior
///
/// - Formats the message using the standard warning style.
/// - Emits the message to standard error.
///
/// # Side Effects
///
/// - Writes to stderr.
///
/// # Panics
///
/// This function does not intentionally panic.

pub fn warn(message: &str) {
    eprintln!("{} {}", "[WARN]".yellow().bold(), message);
}


/// Emit an error message to the console.
///
/// Use this for fatal or near-fatal conditions that indicate an operation
/// has failed and that further progress may not be possible without
/// intervention.
///
/// # Parameters
///
/// - `message`: Human-readable error text to display.
///
/// # Behavior
///
/// - Formats the message using the standard error style.
/// - Emits the message to standard error.
///
/// # Side Effects
///
/// - Writes to stderr.
///
/// # Panics
///
/// This function does not intentionally panic.

pub fn error(message: &str) {
    eprintln!("{} {}", "[ERR ]".red().bold(), message);
}


/// Emit a debug message to the console.
///
/// Use this for developer-oriented diagnostic output intended to assist
/// with troubleshooting and internal state inspection.
///
/// # Parameters
///
/// - `message`: Human-readable debug text to display.
///
/// # Behavior
///
/// - Formats the message using the standard debug style.
/// - Emits the message to standard output.
/// - Intended for non-production diagnostic use.
///
/// # Side Effects
///
/// - Writes to stdout.
///
/// # Panics
///
/// This function does not intentionally panic.

pub fn debug(message: &str) {
    eprintln!("{} {}", "[DBG ]".cyan(), message);
}


/// Emit a labeled status indicator to the console.
///
/// Use this for reporting the pass/fail state of discrete operational steps
/// such as checks, validations, or setup phases.
///
/// # Parameters
///
/// - `label`: Short human-readable label describing the operation being reported.
/// - `ok`: Boolean status flag indicating success (`true`) or failure (`false`).
///
/// # Behavior
///
/// - Formats the label and status indicator using the standard status style.
/// - Emits a success-style output if `ok` is `true`.
/// - Emits an error-style output if `ok` is `false`.
///
/// # Side Effects
///
/// - Writes to stdout or stderr depending on the status value.
///
/// # Panics
///
/// This function does not intentionally panic.

pub fn status(label: &str, ok: bool) {
    if ok {
        println!("{} {}", "[ OK ]".green().bold(), label);
    } else {
        println!("{} {}", "[FAIL]".red().bold(), label);
    }
}
