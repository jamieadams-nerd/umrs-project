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
//! - verbose()          Only visibile if -v 
//! - console_info()
//! - console_status()   Pass success/fail type messages
//! - console_warn()
//! - console_error()
//! - console_fatal()    This will result in program halt
//! - console_event()    structured, pre-defined event types (-v enables)
//!
//! Goals:
//! - Internationalization of console_event() predefined events
//! - Terminal capability detection or feature negotiation.
//!
//! Non-goals:
//! - Full logging or audit event handling or persist to disk.
//!
//!
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
    let _ = stdout_is_tty();
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
static STDOUT_IS_TTY: OnceLock<bool> = OnceLock::new();

#[allow(unused)]
pub fn stdout_is_tty() -> bool {
    *STDOUT_IS_TTY.get_or_init(|| std::io::stdout().is_terminal())
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
                   concat!("", $fmt)
                   $(, $arg)*
               );
            } else {
                eprintln!(concat!("", $fmt) $(, $arg)*);
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
/// - `message`: Human-readable informational text to display.
///
/// # Behavior
/// - Formats the message using the standard informational style.
/// - Emits the message to standard output.
///
/// # Side Effects
/// - Writes to stdout.
///
/// This function does not intentionally panic.
#[macro_export]
#[allow(unused)]
macro_rules! console_info {
    ($fmt:expr $(, $arg:expr)*) => {{
        // Options information 'i' is \u{2139} but it would to overwhelming
        // if several information messages appeared in sequence. But they 
        // could use verbose most of the time and then an info occassionally.
        //let prefix = "\u{2139} Info: ";
        //let prefix = "\u{202F} Info: "; // Halfspace
        //
        let prefix = "INFO:";

        if $crate::console::stdout_is_tty() {
            // ANSI on tty
            use ::colored::Colorize;

            // Left align and padd to 8
            let msg = format!(
                "{:>8} {}",
                prefix,
                format!($fmt $(, $arg)*)
            );
            println!("{}", msg.cyan().bold());

            // Colorize just the PREFIX
            // println!(
            //     "{} {}", 
            //     prefix.cyan().bold(),
            //     format!($fmt $(, $arg)*)
            // );
            //
        } else {
            // plain text off-tty
            println!(
                concat!("{} ", $fmt),
                prefix,
                $( $arg ),*
            );
        }
    }};
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
    ($fmt:expr $(, $arg:expr)*) => {{
        //let prefix = "\u{26A0} Warning: ";
        // let prefix = "\u{202F} Warning: "; // Halfspace
        // Leave trailing space off, format will do it below.
        let prefix = "WARN:"; 

        if $crate::console::stdout_is_tty() {
            // ANSI on tty
            use ::colored::Colorize;

            let msg = format!(
                "{:>8} {}",
                prefix,
                format!($fmt $(, $arg)*)
            );
            println!("{}", msg.yellow().bold());

            // Colorize just the PREFIX
            // println!(
            //     "{} {}", 
            //     prefix.cyan().bold(),
            //     format!($fmt $(, $arg)*)
            // );
            //
        } else {
            // plain text off-tty
            println!(
                concat!("{} ", $fmt),
                prefix,
                $( $arg ),*
            );
        }
    }};
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
    ($fmt:expr $(, $arg:expr)*) => {{
        // \u{1F6D1} is a giant, red, filled octagon -- it's awkward. 
        // \u{270B} is a raised hand as if to stay "stop" 
        // \u{13020} Egyptian hieroglyph - Man raising hands
        //let prefix = "\u{13020} Error: ";
        // Don't put trailing space.. format will do it below
        let prefix = "ERROR:";

        if $crate::console::stdout_is_tty() {
            // ANSI on tty
            use ::colored::Colorize;

            let msg = format!(
                " {:>7} {}",
                prefix,
                format!($fmt $(, $arg)*)
            );
            println!("{}", msg.red().bold());

            // Colorize just the PREFIX
            // println!(
            //     "{} {}", 
            //     prefix.cyan().bold(),
            //     format!($fmt $(, $arg)*)
            // );
            //
        } else {
            // plain text off-tty
            println!(
                concat!("{} ", $fmt),
                prefix,
                $( $arg ),*
            );
        }
    }};
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
#[allow(unused)]
macro_rules! console_status {
    ($ok:expr, $fmt:expr $(, $arg:expr)*) => {{
        // \u{2705} and \u{274E) are BIG green boxes with checkmark and X
        // \u{2714} and \u{2715) are small, plain checkmark and X
        let prefix = if $ok {
            "\u{2714}"
        } else {
            "\u{2715}"
        };

        if $crate::console::stdout_is_tty() {
            // ANSI on tty
            use ::colored::Colorize;

            // The ^ will center the symbol inside of 7 spaces
            // the < > will left and right align
            let msg = format!(
                "{:>4} {}",
                prefix,
                format!($fmt $(, $arg)*)
            );
            println!("{}", msg);

            // Colorize just the PREFIX
            // println!(
            //     "{} {}", 
            //     prefix.cyan().bold(),
            //     format!($fmt $(, $arg)*)
            // );
            //
        } else {
            // plain text off-tty
            println!(
                concat!("{} ", $fmt),
                prefix,
                $( $arg ),*
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

    FileOpen { path: &'a str },
    FileClose { path: &'a str },

    DataReading { path: &'a str },
    DataRead { path: &'a str },
    DataWrite { path: &'a str },
    DataWrote { path: &'a str },

    // Single Events
    FileNotFound { path: &'a str },
}

impl<'a> ConsoleEvent<'a> {
    /// Render into a human-facing message.
    ///
    /// Private: this is presentation logic, not API.
    fn render(&self) -> String {
        match self {
            // Common Paired Events
            ConsoleEvent::BeginTask { name } => format!("\u{27E6}  Begin. {}", name),
            ConsoleEvent::EndTask { name } => format!("\u{27E7}  End. {}", name),

            ConsoleEvent::FileOpen { path } => format!("\u{1F5C0}  Opening {}", path),
            ConsoleEvent::FileClose { path } => format!("\u{2394}  Closing {}", path),

            ConsoleEvent::DataRead { path } => format!("\u{26C1}  Read {}", path),
            ConsoleEvent::DataReading { path } => format!("\u{26C1}  Reading {}", path),

            ConsoleEvent::DataWrite { path } => format!("\u{26C3}  Writing {}", path),
            ConsoleEvent::DataWrote { path } => format!("\u{26C3}  Wrote {}", path),

            // Common Single events
            ConsoleEvent::FileNotFound { path } => format!("\u{2715}  Not found: {}", path),


            // Common Single events

        }
    }
}

// ==================================================================
// PRIVATE Private emission helper. -- Never part of the public API.
// ==================================================================
#[allow(unused)]
pub fn __console_emit(event: ConsoleEvent<'_>) {
    let message = event.render();
    verbose!("  {}", message);
}
