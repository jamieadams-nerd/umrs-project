//! Crate-level prelude for `umrs-core`.
//!
//! Re-exports the most commonly used console primitives so downstream crates
//! can write `use umrs_core::prelude::*` to pull in [`AnsiColor`], [`BOLD`],
//! [`DIM`], [`RESET`], and the console formatting macros in one import.
//!
//! ## Compliance
//!
//! This module provides internal re-export infrastructure with no direct
//! security surface.

pub use crate::console::ansi::*;
pub use crate::console::ansi::{AnsiColor, BOLD, DIM, RESET};
pub use crate::console::macros::*;
