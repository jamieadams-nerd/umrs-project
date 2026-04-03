// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! Console output subsystem for UMRS tool binaries.
//!
//! This module is the public façade for all terminal-facing output infrastructure.
//! It aggregates ANSI escape sequence constants ([`ansi`]), box-drawing primitives
//! ([`boxmsg`], [`symbols`]), a terminal spinner ([`spinner`]), typography stylization
//! ([`typography`]), and the runtime state and macros that govern formatted operator
//! output ([`macros`]).
//!
//! Key exported items:
//! - [`AnsiColor`] and ANSI escape sequence constants — via `ansi::*` re-export
//! - [`VERBOSE`], [`stderr_is_tty`], [`stdout_is_tty`] — runtime terminal detection
//! - `console_info!`, `console_warn!`, `console_error!`, `console_status!`,
//!   `verbose!` — structured operator-facing output macros (defined at crate root
//!   via `#[macro_export]` and re-exported here for ergonomic access)
//!
//! ## NO_COLOR Compliance
//!
//! All color output is gated on TTY detection. The `macros` sub-module respects
//! the `NO_COLOR` environment variable, ensuring that color escape sequences are
//! never emitted to non-interactive pipes or log-capture pipelines.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — operator-facing output must be
//!   clear and unambiguous. This module enforces consistent message formatting and
//!   prevents ANSI escape code contamination of audit log pipelines.
//! - **NSA RTB**: Security state presentation must be unambiguous; TTY-gated color
//!   output ensures log captures remain clean and machine-parseable.

pub mod ansi;
pub mod boxmsg;
pub mod macros;
pub mod spinner;
pub mod symbols;
pub mod typography;

// ------------------------------
// Console public facade
// ------------------------------

// Re-export console-facing types
pub use ansi::*;

// Re-export console runtime state & helpers
pub use macros::{VERBOSE, stderr_is_tty, stdout_is_tty};

// Re-export console macros (DEFINED at crate root via #[macro_export])
pub use crate::{console_error, console_info, console_status, console_warn, verbose};
