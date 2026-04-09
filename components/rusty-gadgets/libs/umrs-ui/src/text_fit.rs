// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # text_fit — Width-Aware Text Truncation Helpers
//!
//! Small utilities for fitting text into a column of a known terminal
//! width.  Every UMRS TUI tool eventually needs to display a value
//! (typically a filesystem path) that may exceed its allocated column;
//! this module provides the single, shared implementation.
//!
//! ## Key Exports
//!
//! - [`truncate_left`] — truncate a string to fit a given display width,
//!   dropping characters from the **front** and prepending an ellipsis.
//!   Preserves the tail of the input (usually the important part of a
//!   path — the filename and its immediate parents).
//! - [`display_width`] — compute the terminal display width of a string
//!   in monospace cells (`unicode-width` crate).  Most single-glyph
//!   emoji occupy two cells; most ASCII occupies one.
//! - [`wrap_text_lines`] — word-wrap a plain string into a `Vec<String>`,
//!   one element per line, each fitting within a given column budget.
//! - [`wrap_indented`] — word-wrap into styled ratatui [`Line`] values with
//!   a fixed indent prefix on every continuation line.  Unlike ratatui's
//!   built-in `Wrap`, this preserves the indent on every wrapped line so
//!   multi-line fields in detail panels stay aligned.
//!
//! ## Why left-truncation for paths?
//!
//! Operators scanning a file browser care about *where they are right
//! now* (the leaf directory and its immediate parent), not the prefix
//! that anchors them to the root.  A right-truncated path
//! (`/DEVELOPMENT/umrs-project/components/rusty…`) loses the leaf and
//! is almost useless; a left-truncated path
//! (`…components/rusty-gadgets/umrs-ls/src/main.rs`) keeps the signal
//! and discards the anchor the operator already knows.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit-relevant display fields (paths,
//!   security contexts, filenames) must remain legible at any terminal
//!   width; truncation must be deterministic and never silently hide
//!   the trailing identifier.

use ratatui::style::Style;
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthStr;

use crate::icons::ELLIPSIS;

/// Compute the monospace display width of `s` in terminal cells.
///
/// Wraps `unicode_width::UnicodeWidthStr::width` so callers need not
/// depend on the crate directly.
#[must_use = "width is the only output; discarding it has no effect"]
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Truncate `input` from the left so its display width fits in
/// `max_width` cells, prepending [`ELLIPSIS`] (`…`) to signal the cut.
///
/// Behaviour:
///
/// - If `input`'s display width already fits in `max_width`, it is
///   returned unchanged (no ellipsis).
/// - If `max_width` is `0`, returns an empty string.
/// - If `max_width` is `1`, returns just the ellipsis (`…`).
/// - Otherwise, scans the string from the right, keeping characters
///   until adding one more would push the display width (including the
///   ellipsis cell) over `max_width`, then returns `"…<kept_tail>"`.
///
/// Unicode-correct: every character boundary check goes through
/// `char_indices` so multi-byte sequences are never split, and display
/// width is measured per-character via `unicode_width` so wide glyphs
/// (emoji, CJK) are counted accurately.
///
/// # Examples
///
/// ```
/// use umrs_ui::text_fit::truncate_left;
/// assert_eq!(truncate_left("hello world", 20), "hello world");
/// assert_eq!(truncate_left("hello world", 6),  "\u{2026}world");
/// assert_eq!(truncate_left("hello",       5),  "hello");
/// assert_eq!(truncate_left("hello",       1),  "\u{2026}");
/// assert_eq!(truncate_left("hello",       0),  "");
/// ```
#[must_use = "truncated string is the only output; discarding it has no effect"]
pub fn truncate_left(input: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let input_width = display_width(input);
    if input_width <= max_width {
        return input.to_owned();
    }

    // Always reserve one cell for the ellipsis.  If the budget is only
    // one cell, that's all we can show.
    let ellipsis_width = display_width(ELLIPSIS); // always 1 in practice
    if max_width <= ellipsis_width {
        return ELLIPSIS.to_owned();
    }
    let tail_budget = max_width - ellipsis_width;

    // Walk characters from the right, accumulating width until adding
    // the next char would exceed the tail budget.  `char_indices` gives
    // us byte offsets we can slice safely.
    let mut kept_width: usize = 0;
    let mut keep_from: usize = input.len();
    for (byte_idx, ch) in input.char_indices().rev() {
        let ch_width = UnicodeWidthStr::width(ch.encode_utf8(&mut [0u8; 4]));
        if kept_width + ch_width > tail_budget {
            break;
        }
        kept_width += ch_width;
        keep_from = byte_idx;
    }

    let mut out = String::with_capacity(ELLIPSIS.len() + (input.len() - keep_from));
    out.push_str(ELLIPSIS);
    out.push_str(&input[keep_from..]);
    out
}

/// Truncate `input` from the **right** so its display width fits in
/// `max_width` cells, appending [`ELLIPSIS`] to signal the cut.
///
/// Mirror of [`truncate_left`] — use this when the *start* of the string
/// is the important part (hostnames, usernames, contexts, OS names)
/// rather than the tail.  Paths typically want `truncate_left`; labeled
/// fields typically want `truncate_right`.
///
/// Behaviour:
///
/// - If `input`'s display width already fits in `max_width`, it is
///   returned unchanged (no ellipsis).
/// - If `max_width` is `0`, returns an empty string.
/// - If `max_width` is `1`, returns just the ellipsis.
/// - Otherwise, scans the string from the left, keeping characters
///   until adding one more would push the display width (including the
///   ellipsis cell) over `max_width`, then returns `"<kept_head>…"`.
///
/// UTF-8 and wide-glyph safe, same contract as [`truncate_left`].
///
/// # Examples
///
/// ```
/// use umrs_ui::text_fit::truncate_right;
/// assert_eq!(truncate_right("hello world", 20), "hello world");
/// assert_eq!(truncate_right("hello world", 6),  "hello\u{2026}");
/// assert_eq!(truncate_right("hello",       5),  "hello");
/// assert_eq!(truncate_right("hello",       1),  "\u{2026}");
/// assert_eq!(truncate_right("hello",       0),  "");
/// ```
#[must_use = "truncated string is the only output; discarding it has no effect"]
pub fn truncate_right(input: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let input_width = display_width(input);
    if input_width <= max_width {
        return input.to_owned();
    }

    let ellipsis_width = display_width(ELLIPSIS);
    if max_width <= ellipsis_width {
        return ELLIPSIS.to_owned();
    }
    let head_budget = max_width - ellipsis_width;

    // Walk characters from the left until one more would exceed budget.
    let mut kept_width: usize = 0;
    let mut keep_until: usize = 0;
    for (byte_idx, ch) in input.char_indices() {
        let ch_width = UnicodeWidthStr::width(ch.encode_utf8(&mut [0u8; 4]));
        if kept_width + ch_width > head_budget {
            break;
        }
        kept_width += ch_width;
        keep_until = byte_idx + ch.len_utf8();
    }

    let mut out = String::with_capacity(keep_until + ELLIPSIS.len());
    out.push_str(&input[..keep_until]);
    out.push_str(ELLIPSIS);
    out
}

/// Word-wrap `text` into owned strings that each fit within `max_width`
/// display columns.
///
/// Words are split on ASCII whitespace. A word wider than `max_width` is
/// placed on its own line without mid-word splitting. Returns an empty
/// `Vec` when `text` is empty; returns the full text as a single element
/// when `max_width` is zero (no wrapping performed).
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Operator-visible text fields must remain
///   legible at any terminal width; manual wrapping preserves indentation
///   that ratatui's built-in `Wrap` cannot maintain.
#[must_use = "wrapped lines are the only output; discarding them has no effect"]
pub fn wrap_text_lines(text: &str, max_width: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }
    if max_width == 0 {
        return vec![text.to_owned()];
    }
    let mut result: Vec<String> = Vec::new();

    // Respect embedded newlines: split into paragraphs first, then
    // word-wrap each paragraph independently.  A bare `\n` in the
    // input produces a blank line in the output (paragraph break).
    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            result.push(String::new());
            continue;
        }
        let mut current = String::new();
        let mut current_width: usize = 0;

        for word in paragraph.split_whitespace() {
            let ww = display_width(word);
            if current_width == 0 {
                word.clone_into(&mut current);
                current_width = ww;
            } else if current_width.saturating_add(1).saturating_add(ww) <= max_width {
                current.push(' ');
                current.push_str(word);
                current_width = current_width.saturating_add(1).saturating_add(ww);
            } else {
                let mut next = String::new();
                word.clone_into(&mut next);
                result.push(std::mem::replace(&mut current, next));
                current_width = ww;
            }
        }
        if !current.is_empty() {
            result.push(current);
        }
    }
    result
}

/// Word-wrap `text` into styled [`Line`] values, each prefixed with `indent`.
///
/// The available text budget per line is
/// `max_width.saturating_sub(display_width(indent))`. A word wider than
/// the budget is placed on its own line without splitting. Returns an empty
/// `Vec` when `text` is empty or the budget is zero.
///
/// Unlike ratatui's built-in `Wrap`, this function preserves the indentation
/// prefix on every continuation line, so multi-line fields (Description,
/// Handling, etc.) in detail panels remain visually aligned regardless of
/// terminal width.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Multi-line content in audit-relevant panels
///   must remain legible and correctly indented at any terminal width.
/// - **NIST SP 800-53 AC-3**: Label display fidelity — no continuation line
///   may be rendered without its indent prefix, which would break visual
///   grouping with the originating label.
#[must_use = "wrapped lines are the only output; pass them to a Paragraph widget"]
pub fn wrap_indented(
    text: &str,
    indent: &str,
    max_width: usize,
    style: Style,
) -> Vec<Line<'static>> {
    let indent_width = display_width(indent);
    let text_budget = max_width.saturating_sub(indent_width);
    if text_budget == 0 || text.is_empty() {
        return Vec::new();
    }
    wrap_text_lines(text, text_budget)
        .into_iter()
        .map(|segment| {
            let full_line = format!("{indent}{segment}");
            Line::from(Span::styled(full_line, style))
        })
        .collect()
}
