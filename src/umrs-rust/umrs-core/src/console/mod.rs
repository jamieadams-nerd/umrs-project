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
//! Console messages:
//! - verbose()
//! - console_info()
//! - console_status()   // Pass success/fail type messages
//! - console_warn()
//! - console_error()
//! - console_fatal()    // This will result in program halt
//! - console_event()    // structured, pre-defined event ty pes
//!
//! Non-goals:
//! - Full logging or audit event handling
//! - Terminal capability detection or feature negotiation
//! - Internationalization or localization support
//
use colored::Colorize;
use std::io::IsTerminal;
use std::sync::atomic::AtomicBool;
use std::sync::OnceLock;

pub mod ansi;
pub mod boxmsg;
pub mod spinner;

// ==================================================================
//   Initialize Console Messaging subsystem
// ==================================================================
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
    let _ = stderr_is_tty();
    let _ = stdin_is_tty();
}

// ==================================================================
//   TTY OR TERMINAL? -- Only print CTL characters on terminal.
// ==================================================================
#[allow(unused)]
static STDERR_IS_TTY: OnceLock<bool> = OnceLock::new();

#[allow(unused)]
pub fn stderr_is_tty() -> bool {
    *STDERR_IS_TTY.get_or_init(|| std::io::stderr().is_terminal())
}

#[allow(unused)]
static STDIN_IS_TTY: OnceLock<bool> = OnceLock::new();

#[allow(unused)]
pub fn stdin_is_tty() -> bool {
    *STDIN_IS_TTY.get_or_init(|| std::io::stdin().is_terminal())
}

// ==================================================================
//   VERBOSE Messages
// ==================================================================
#[allow(unused)]
pub static VERBOSE: AtomicBool = AtomicBool::new(false);

#[macro_export]
#[allow(unused)]
macro_rules! verbose {
    ($fmt:expr $(, $arg:expr)*) => {
        if $crate::console::VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
            if $crate::console::stderr_is_tty() {
               eprintln!(
                   concat!("\x1b[36m\u{21E2}\x1b[0m ", $fmt)
                   $(, $arg)*
               );
            } else {
                eprintln!(concat!("\u{21E2} ", $fmt) $(, $arg)*);
            }
        }
    };
}

// ==================================================================
//   INFORMATIONAL Messages
// ==================================================================
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
#[macro_export]
#[allow(unused)]
macro_rules! console_info {
    ($fmt:expr $(, $arg:expr)*) => {
            if $crate::console::stdin_is_tty() {
               println!(
                   concat!("\x1b[36m\u{21E2}\x1b[0m ", $fmt)
                   $(, $arg)*
               );
            } else {
                println!(concat!("\u{21E2} ", $fmt) $(, $arg)*);
            }
    };
}

// ==================================================================
//   WARNING Messages
// ==================================================================
/// Emit a warning message to the console.
///
/// Use this for non-fatal conditions that may require user attention or
/// corrective action but do not prevent continued execution.
///
#[macro_export]
#[allow(unused)]
macro_rules! console_warn {
    ($fmt:expr $(, $arg:tt)*) => {
        if $crate::console::stderr_is_tty() {
            eprintln!(
                concat!("\x1b[31mWarning:\x1b[0m ", $fmt)
                $(, $arg)*
            );
         } else {
            eprintln!(
                concat!("Warning: ", $fmt)
                $(, $arg)*
            );
         }
    };
}

// ==================================================================
//   ERROR Messages
// ==================================================================
/// Emit an error message to the console.
///
/// Use this for fatal or near-fatal conditions that indicate an operation
/// has failed and that further progress may not be possible without
/// intervention.
///
#[macro_export]
#[allow(unused)]
macro_rules! console_error {
    ($fmt:expr $(, $arg:tt)*) => {
        if $crate::console::stderr_is_tty() {
            eprintln!(
                concat!("\x1b[31m[ERROR]\x1b[0m ", $fmt)
                $(, $arg)*
            );
         } else {
            eprintln!(
                concat!("[ERROR] ", $fmt)
                $(, $arg)*
            );
         }
    };
}

// ==================================================================
//   STATUS Messages - Send True or False = OK or Fail message
// ==================================================================
/// Emit a labeled status indicator to the console.
///
/// Use this for reporting the pass/fail state of discrete operational steps
/// such as checks, validations, or setup phases.
///
/// # Parameters
///
/// - `ok`: Boolean status flag indicating success (`true`) or failure (`false`).
/// - `label`: Short human-readable label describing the operation being reported.
///
#[macro_export]
#[allow(unused)]
macro_rules! console_status {
    ($ok:expr, $fmt:expr $(, $arg:expr)*) => {{
        let prefix = if $ok {
            "[Success] "
        } else {
            "[Failure] "
        };

        if $crate::console::stdin_is_tty() {
            println!(
                concat!("\x1b[36m\u{21E2}\x1b[0m ", "{}"),
                format!(concat!("{}", $fmt), prefix $(, $arg)*)
            );
        } else {
            println!(
                "{}",
                format!(concat!("{}", $fmt), prefix $(, $arg)*)
            );
        }
    }};
}

// ==================================================================
//   FATAL Event - Will display message and end the program
//     - Using this will result in program halt!
// ==================================================================
#[macro_export]
#[allow(unused)]
macro_rules! console_fatal {
    ($code:expr, $fmt:expr $(, $arg:expr)*) => {{
        eprintln!(
            concat!("\x1b[31m[ Fatal ]\x1b[0m ", $fmt)
            $(, $arg)*
        );
        std::process::exit($code);
    }};
}

// ==================================================================
//   PREDEINFED EVENTS (e.h., BeginTask or EndTask
// ==================================================================
//
// The console_event! is not for ad-hoc messages.
// It is for predefined, semantic events that:
//   - have a known shape
//   - are reusable
//   - narrate what the tool is doing
//   - can later be rendered differently without touching call sites
//
// Think of it as a typed console narration layer.
//
// -------------------------------------------------------------------
// Example using console_event!:
//
//   use umrs_core::prelude::*;
//   use umrs_core::console::ConsoleEvent;
//
//   fn main() {
//       console_event!(ConsoleEvent::BeginTask {
//          name: "Extract Unicode Hieroglyphs",
//       });
//   }
// -------------------------------------------------------------------
// Public macro — THIS is the API.
// (be sure to add to prelude.rs
#[macro_export]
macro_rules! console_event {
    ($event:expr) => {{
        $crate::console::__console_emit($event)
    }};
}

//
// EVENT Definitions - Predefined events with template messages
//
// NOTES:
// • Lifetimes keep it lightweight (no allocations forced)
// • Variants define argument shape, not semantics
// • Adding a new action is deliberate and visible
//
// Public because other crates construct these.
//
pub enum ConsoleEvent<'a> {
    // Common pairs of events
    BeginTask { name: &'a str },
    EndTask { name: &'a str },

    OpenFile { path: &'a str },
    CloseFile { path: &'a str },

    ReadFile { path: &'a str },
    WriteFile { path: &'a str },

    // Misc. Stuff
    UsingCache { key: &'a str },
    Skipped { reason: &'a str },
}

impl<'a> ConsoleEvent<'a> {
    /// Render into a human-facing message.
    ///
    /// Private: this is presentation logic, not API.
    fn render(&self) -> String {
        match self {
            ConsoleEvent::BeginTask { name } => format!("BEGIN - {}", name),

            ConsoleEvent::EndTask { name } => format!("END   - {}", name),

            ConsoleEvent::OpenFile { path } => format!("Opening file {}", path),
            ConsoleEvent::CloseFile { path } => format!("Closing file {}", path),

            ConsoleEvent::ReadFile { path } => format!("Reading file {}", path),
            ConsoleEvent::WriteFile { path } => format!("Writing file {}", path),

            ConsoleEvent::UsingCache { key } => format!("Using cached result ({})", key),
            ConsoleEvent::Skipped { reason } => format!("Skipped: {}", reason),
        }
    }
}

// ==================================================================
// PRIVATE Private emission helper. -- Never part of the public API.
// ==================================================================
#[allow(unused)]
pub fn __console_emit(event: ConsoleEvent<'_>) {
    let message = event.render();
    verbose!("{}", message);
}
