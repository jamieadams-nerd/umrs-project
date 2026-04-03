// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//

//! Human-readable formatting utilities for operator-facing output.
//!
//! Aggregates three presentation helpers that convert raw numeric or textual
//! values into forms suitable for display in CLI output, diagnostic reports,
//! and TUI panels:
//!
//! - [`metricfmt`] — SI-prefix scaling for engineering and diagnostic magnitudes
//!   (`SIPrefix`, `PrefixStyle`, `FormatOptions`, `auto_format`, `format_in_prefix`)
//! - [`sizefmt`] — byte-size formatting with decimal SI and binary IEC bases
//!   (`SizeBase`, `SizeUnit`, `auto_format`, `format_in_unit`)
//! - [`textwrap`] — fixed-width word-wrapping for console paragraphs and help text
//!   (`text_wrap`)
//!
//! All three sub-modules are purely presentational: they perform no I/O, make no
//! security decisions, and carry no trust-boundary responsibility.
//!
//! ## Compliance
//!
//! This module provides internal formatting utility infrastructure with no direct
//! security surface. No NIST SP 800-53, CMMC, or NSA RTB controls apply directly;
//! correct output formatting indirectly supports **NIST SP 800-53 AU-3** (audit
//! record content) by ensuring operator-facing values are legible and unambiguous.

pub mod metricfmt;
pub mod sizefmt;
pub mod textwrap;
