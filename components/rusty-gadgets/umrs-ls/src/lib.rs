// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # umrs-ls — Library Target
//!
//! Exposes the `grouping`, `tree_adapter`, and `viewer_app` modules for
//! integration testing and TUI integration.  All display and CLI logic
//! lives in `main.rs`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: File cuddling preserves complete audit record
//!   content — sibling entries retain all security metadata.
//! - **NSA RTB**: Deterministic Execution — O(n) grouping with no hidden
//!   side effects.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]

pub mod grouping;
pub mod identity;
pub mod tree_adapter;
pub mod tui_render;
pub mod viewer_app;
