// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
use std::io::IsTerminal;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;

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
        let prefix = "[INFO]";

        if ! $crate::console::stdout_is_tty() {
            println!( concat!("{} ", $fmt), prefix, $( $arg ),*);
        } else {
            use ::colored::Colorize;
            let msg = format!( "{prefix} {}", format!($fmt $(, $arg)*));
            println!("{msg}");
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
        let prefix = "[WARN]";

        if ! $crate::console::stdout_is_tty() {
            println!( concat!("{} ", $fmt), prefix, $( $arg ),*);
        } else {
            use ::colored::Colorize;
            let msg = format!( "{} {}", prefix.yellow().bold(), format!($fmt $(, $arg)*));
            println!("{msg}");
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
        let prefix = "[ERROR]";

        if ! $crate::console::stdout_is_tty() {
            println!( concat!("{} ", $fmt), prefix, $( $arg ),*);
         } else {
            use ::colored::Colorize;
            let msg = format!( "{} {}", prefix.red().bold(), format!($fmt $(, $arg)*));
            println!("{msg}");
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
macro_rules! console_status {
    ($ok:expr, $fmt:expr $(, $arg:expr)*) => {{
        // \u{2705} and \u{274E) are BIG green boxes with checkmark and X
        // \u{2714} and \u{2715) are small, plain checkmark and X
        let prefix = if $ok {
            "\u{2714}"
        } else {
            "\u{2715}"
        };

        if ! $crate::console::stdout_is_tty() {
            let msg = format!( "{:>4} {}", prefix, format!($fmt $(, $arg)*));
            println!("{msg}");
        } else {
            use ::colored::Colorize;
            let msg = format!( "{:>4} {}", prefix, format!($fmt $(, $arg)*));
            println!("{msg}");
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
