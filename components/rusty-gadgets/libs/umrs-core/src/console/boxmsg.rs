//! Box-drawing utilities for framed console output.
//!
//! Provides [`BoxStyle`], which holds the Unicode or ASCII border characters for a
//! frame, and [`box_lines`], which renders a slice of strings inside a padded
//! bordered box. Supports both Unicode line-drawing glyphs and ASCII fallback
//! characters for environments that cannot render Unicode.
//!
//! ## Compliance
//!
//! This module provides internal formatting utility infrastructure with no
//! direct security surface.

pub struct BoxStyle {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub horizontal: &'static str,
    pub vertical: &'static str,
}

impl BoxStyle {
    pub const UNICODE: Self = Self {
        top_left: "┌",
        top_right: "┐",
        bottom_left: "└",
        bottom_right: "┘",
        horizontal: "─",
        vertical: "│",
    };

    pub const ASCII: Self = Self {
        top_left: "+",
        top_right: "+",
        bottom_left: "+",
        bottom_right: "+",
        horizontal: "-",
        vertical: "|",
    };
}

pub fn box_lines(lines: &[String], padding: usize, style: &BoxStyle) -> String {
    let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    let inner_width = max_width + padding * 2;
    let top = format!(
        "{}{}{}",
        style.top_left,
        style.horizontal.repeat(inner_width),
        style.top_right
    );
    let bottom = format!(
        "{}{}{}",
        style.bottom_left,
        style.horizontal.repeat(inner_width),
        style.bottom_right
    );

    let mut out = Vec::new();
    out.push(top);

    for line in lines {
        let visible_len = line.chars().count();
        let right_pad = max_width - visible_len;

        let content = format!(
            "{}{}{}{}{}{}",
            style.vertical,
            " ".repeat(padding),
            line,
            " ".repeat(right_pad),
            " ".repeat(padding),
            style.vertical,
        );

        out.push(content);
    }

    out.push(bottom);

    out.join("\n")
}
