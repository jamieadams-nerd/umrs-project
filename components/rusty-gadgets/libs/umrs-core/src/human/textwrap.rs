//! Text wrapping utilities for fixed-width console output.
//!
//! Provides [`text_wrap`], which wraps an input string to a specified column
//! width with configurable left and right padding. Wrapping never breaks words
//! and uses ASCII-space word boundaries.
//!
//! ## Compliance
//!
//! This module provides internal formatting utility infrastructure with no
//! direct security surface.

use ::textwrap::{Options, wrap};

pub fn text_wrap(input: &str, width: usize, left_pad: usize, right_pad: usize) -> String {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    // Character count only — input content is never logged (SI-11).
    #[cfg(debug_assertions)]
    let input_chars = input.chars().count();

    let indent = " ".repeat(left_pad);

    let options = Options::new(width)
        .initial_indent(&indent)
        .subsequent_indent(&indent)
        .break_words(false)
        .word_separator(::textwrap::WordSeparator::AsciiSpace);

    let mut lines = wrap(input, &options);

    // Right-pad each line if requested
    if right_pad > 0 {
        for line in &mut lines {
            let visible_len = line.chars().count();
            let target_len = left_pad + width + right_pad;

            if visible_len < target_len {
                let pad = target_len - visible_len;
                line.to_mut().push_str(&" ".repeat(pad));
            }
        }
    }

    let output = lines.join("\n");

    #[cfg(debug_assertions)]
    log::debug!(
        "text_wrap input_chars={input_chars} width={width} elapsed={}µs",
        t0.elapsed().as_micros()
    );

    output
}

// fn main() {
// let text = r#"
// This is a long paragraph that should be wrapped nicely to a fixed width
// without breaking words or hyphenating them. It should preserve paragraph
// breaks exactly as they appear.
//
// This is a second paragraph that should also be wrapped independently.
// "#;
//
// let wrapped = text_wrap(text, 40, 4, 0);
//
// println!("{}", wrapped);
//  }
//
