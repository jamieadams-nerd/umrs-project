//use crate::console::typography::{stylize, TypographyStyle};
// let header = stylize("UMRS Core 1.0", TypographyStyle::DoubleStruck);
// println!("{}", header); // 𝕌𝕄ℝ𝕊 ℂ𝕠𝕣𝕖 𝟙.𝟘

/// Unified Typography and Stylization for the UMRS Design System.
///
/// This module provides functions to map standard ASCII alphanumeric
/// characters to various Unicode stylistic sets (Bold, Double-Struck, etc.)
/// while preserving punctuation and spacing.
pub enum TypographyStyle {
    /// 𝐀-𝐙 (0x1D400), 𝐚-𝐳 (0x1D41A), 𝟎-𝟗 (0x1D7CE)
    Bold,

    /// 𝔸-ℤ (0x1D538), 𝕒-𝕫 (0x1D552), 𝟘-𝟡 (0x1D7D8)
    DoubleStruck,

    /// Ⓐ-Ⓩ (0x24B6), ⓐ-ⓩ (0x24D0), ①-⑨ (0x2460)
    Circled,

    /// 𝔄-ℨ (0x1D504), 𝔞-𝔷 (0x1D51E) [No Digits]
    Gothic,

    /// 𝒜-𝒵 (0x1D49C), 𝒶-𝓏 (0x1D4B6) [No Digits]
    Script,

    Segmented,
}

/// Stylize a string by mapping ASCII letters and digits to Unicode equivalents.
/// Punctuation, spaces, and existing Unicode characters are left untouched.
///
/// the mapping is Uppercase, lowercase, and numbers.
///
pub fn stylize(input: &str, style: TypographyStyle) -> String {
    input
        .chars()
        .map(|c| {
            match style {
                TypographyStyle::Bold => map_char(c, 0x1D400, 0x1D41A, 0x1D7CE),
                TypographyStyle::DoubleStruck => {
                    map_char(c, 0x1D538, 0x1D552, 0x1D7D8)
                }
                TypographyStyle::Circled => map_char_circled(c),
                TypographyStyle::Gothic => map_char(c, 0x1D504, 0x1D51E, 0),
                TypographyStyle::Script => map_char(c, 0x1D49C, 0x1D4B6, 0),
                // Pass zeros to leave set untouched (regular ASCII).
                TypographyStyle::Segmented => map_char(c, 0, 0, 0x1FBF0),
            }
        })
        .collect()
}

/// Internal helper for contiguous Unicode blocks
fn map_char(
    c: char,
    upper_start: u32,
    lower_start: u32,
    digit_start: u32,
) -> char {
    if c.is_ascii_uppercase() && upper_start != 0 {
        std::char::from_u32(upper_start + (c as u32 - 'A' as u32)).unwrap_or(c)
    } else if c.is_ascii_lowercase() && lower_start != 0 {
        std::char::from_u32(lower_start + (c as u32 - 'a' as u32)).unwrap_or(c)
    } else if c.is_ascii_digit() && digit_start != 0 {
        let val = c.to_digit(10).unwrap();
        std::char::from_u32(digit_start + val).unwrap_or(c)
    } else {
        c
    }
}

/// Specialized mapper for Circled characters (which are not perfectly contiguous)
fn map_char_circled(c: char) -> char {
    if c.is_ascii_uppercase() {
        std::char::from_u32(0x24B6 + (c as u32 - 'A' as u32)).unwrap_or(c)
    } else if c.is_ascii_lowercase() {
        std::char::from_u32(0x24D0 + (c as u32 - 'a' as u32)).unwrap_or(c)
    } else if c.is_ascii_digit() {
        let val = c.to_digit(10).unwrap();
        if val == 0 {
            return '⓪';
        } // Circled zero is 0x24EA
        std::char::from_u32(0x2460 + (val - 1)).unwrap_or(c)
    } else {
        c
    }
}
