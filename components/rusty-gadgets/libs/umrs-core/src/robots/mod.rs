// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! Robot ASCII art support.
//!
//! This module contains normalized ASCII art used for console rendering.
//! The generator tool produces instances of these structures.
//!
//! ## Compliance
//!
//! No security controls apply — this module renders decorative ASCII art
//! mascots for console output. It contains no security logic, no credential
//! handling, and no audit-relevant state.

mod data;

pub use data::{AsciiArt, AsciiArtStatic, build_robot_art};

// Re-export ALL robots automatically;
//

mod builtins;
pub use builtins::*;
