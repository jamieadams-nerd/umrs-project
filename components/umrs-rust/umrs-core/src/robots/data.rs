// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

use std::error::Error;

/// Normalized ASCII art.
///
/// The generator guarantees:
/// - no empty top/bottom rows
/// - no empty left/right columns
/// - rectangular shape (right-padded)

/// Normalized ASCII art (owned; used by tools like umrs-robotgen).
#[derive(Debug, Clone)]
pub struct AsciiArt {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub lines: Vec<String>,
}

/// Embedded ASCII art (static; used by umrs-core).
#[derive(Debug, Clone, Copy)]
pub struct AsciiArtStatic {
    pub name: &'static str,
    pub width: usize,
    pub height: usize,
    pub lines: &'static [&'static str],
}


/// Build normalized ASCII art from raw input.
///
/// This is intended to be called by the robot generator tool.
pub fn build_robot_art(
    name: &str,
    input: &str,
) -> Result<AsciiArt, Box<dyn Error>> {
    let raw_lines: Vec<&str> = input.lines().collect();

    // Trim empty top/bottom
    let first = raw_lines.iter().position(|l| !l.trim().is_empty());
    let last = raw_lines.iter().rposition(|l| !l.trim().is_empty());

    let (first, last) = match (first, last) {
        (Some(f), Some(l)) => (f, l),
        _ => {
            return Err("ASCII art is empty".into());
        }
    };

    let mut lines: Vec<String> =
        raw_lines[first..=last].iter().map(|s| s.to_string()).collect();

    // Determine left/right trim
    let left = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().take_while(|c| *c == ' ').count())
        .min()
        .unwrap_or(0);

    let right = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().rev().take_while(|c| *c == ' ').count())
        .min()
        .unwrap_or(0);

    // Apply trim
    lines = lines
        .into_iter()
        .map(|l| {
            let len = l.len();
            l[left..len.saturating_sub(right)].to_string()
        })
        .collect();

    // Normalize width
    let width = lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);

    for l in &mut lines {
        let pad = width.saturating_sub(l.chars().count());
        l.extend(std::iter::repeat(' ').take(pad));
    }

    let height = lines.len();

    Ok(AsciiArt {
        name: name.to_string(),
        width,
        height,
        lines,
    })
}
