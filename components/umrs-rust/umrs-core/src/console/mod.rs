// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

pub mod ansi;
pub mod boxmsg;
pub mod events;
pub mod macros;
pub mod spinner;

// ------------------------------
// Console public facade
// ------------------------------

// Re-export console-facing types
pub use ansi::*;
pub use events::*;

// Re-export console runtime state & helpers
pub use macros::{VERBOSE, stderr_is_tty, stdout_is_tty};

// Re-export console macros (DEFINED at crate root via #[macro_export])
pub use crate::{
    console_error, console_event, console_info, console_status, console_warn,
    verbose,
};
