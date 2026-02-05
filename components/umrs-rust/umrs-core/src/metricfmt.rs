// SPDX-License-Identifier: MIT
// Author: Jamie Adams
//
// Purpose:
//   Generic engineering/diagnostic magnitude formatting using practical SI prefixes.
//   This module formats an arbitrary scalar (f64) into a scaled value plus a prefix
//   string only (e.g., "1.23 u" or "1.23 micro").
//   The calling code is responsible for appending the actual measured unit (e.g. "s", "Hz").
//
//   Features:
//     - Auto-select the "best" prefix so the scaled value is in the range [1, 1000), when possible.
//     - Force a specific prefix.
//     - Prefix rendering styles:
//         * AbbrevAscii: "E", "P", "T", "G", "M", "k", "", "m", "u", "n", "p", "f"
//         * FullText:    "exa", "peta", "tera", "giga", "mega", "kilo", "", "milli", "micro", "nano", "pico", "femto"
//     - Precision warning:
//         * If forced-prefix formatting yields an effectively-zero value at the configured display precision,
//           a PrecisionLoss warning is returned (caller decides how to surface it).
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SIPrefix {
    Exa,   // 10^18
    Peta,  // 10^15
    Tera,  // 10^12
    Giga,  // 10^9
    Mega,  // 10^6
    Kilo,  // 10^3
    Base,  // 10^0
    Milli, // 10^-3
    Micro, // 10^-6
    Nano,  // 10^-9
    Pico,  // 10^-12
    Femto, // 10^-15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixStyle {
    // Abbreviations with ASCII-safe micro: "u" (not "µ")
    AbbrevAscii,
    // Full text prefix names: "micro", "nano", "kilo", etc.
    FullText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatWarning {
    // The chosen/forced scale and display precision cause the formatted value to round to 0.00 (or equivalent),
    // even though the raw value is non-zero.
    PrecisionLoss,
}

#[derive(Debug, Clone, Copy)]
pub struct FormatOptions {
    // Number of decimal places used when formatting the scaled value.
    // Typical values: 2 for diagnostics, 3 for more precision.
    pub decimals: u8,

    // Auto-scale target interval lower bound (inclusive). Default: 1.0
    pub auto_target_min: f64,

    // Auto-scale target interval upper bound (exclusive). Default: 1000.0
    pub auto_target_max: f64,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            decimals: 2,
            auto_target_min: 1.0,
            auto_target_max: 1000.0,
        }
    }
}

fn prefix_exponent(prefix: SIPrefix) -> i32 {
    match prefix {
        SIPrefix::Exa => 18,
        SIPrefix::Peta => 15,
        SIPrefix::Tera => 12,
        SIPrefix::Giga => 9,
        SIPrefix::Mega => 6,
        SIPrefix::Kilo => 3,
        SIPrefix::Base => 0,
        SIPrefix::Milli => -3,
        SIPrefix::Micro => -6,
        SIPrefix::Nano => -9,
        SIPrefix::Pico => -12,
        SIPrefix::Femto => -15,
    }
}

fn prefix_label(prefix: SIPrefix, style: PrefixStyle) -> &'static str {
    match style {
        PrefixStyle::AbbrevAscii => match prefix {
            SIPrefix::Exa => "E",
            SIPrefix::Peta => "P",
            SIPrefix::Tera => "T",
            SIPrefix::Giga => "G",
            SIPrefix::Mega => "M",
            SIPrefix::Kilo => "k",
            SIPrefix::Base => "",
            SIPrefix::Milli => "m",
            SIPrefix::Micro => "u",
            SIPrefix::Nano => "n",
            SIPrefix::Pico => "p",
            SIPrefix::Femto => "f",
        },
        PrefixStyle::FullText => match prefix {
            SIPrefix::Exa => "exa",
            SIPrefix::Peta => "peta",
            SIPrefix::Tera => "tera",
            SIPrefix::Giga => "giga",
            SIPrefix::Mega => "mega",
            SIPrefix::Kilo => "kilo",
            SIPrefix::Base => "",
            SIPrefix::Milli => "milli",
            SIPrefix::Micro => "micro",
            SIPrefix::Nano => "nano",
            SIPrefix::Pico => "pico",
            SIPrefix::Femto => "femto",
        },
    }
}

fn prefixes_descending() -> &'static [SIPrefix] {
    &[
        SIPrefix::Exa,
        SIPrefix::Peta,
        SIPrefix::Tera,
        SIPrefix::Giga,
        SIPrefix::Mega,
        SIPrefix::Kilo,
        SIPrefix::Base,
        SIPrefix::Milli,
        SIPrefix::Micro,
        SIPrefix::Nano,
        SIPrefix::Pico,
        SIPrefix::Femto,
    ]
}

fn scale_value(value: f64, prefix: SIPrefix) -> f64 {
    let exp = prefix_exponent(prefix);
    value / 10f64.powi(exp)
}

fn choose_best_prefix(value: f64, opts: &FormatOptions) -> SIPrefix {
    let abs = value.abs();

    // Zero can be represented at base without any special handling.
    if abs == 0.0 {
        return SIPrefix::Base;
    }

    for &p in prefixes_descending() {
        let scaled = scale_value(abs, p);
        if scaled >= opts.auto_target_min && scaled < opts.auto_target_max {
            return p;
        }
    }

    // If nothing hits the target interval, clamp to nearest end of our practical range.
    // Large values => Exa; tiny values => Femto.
    if abs >= 10f64.powi(prefix_exponent(SIPrefix::Exa)) {
        SIPrefix::Exa
    } else {
        SIPrefix::Femto
    }
}

fn format_scaled_number(scaled: f64, decimals: u8) -> String {
    // Deterministic formatting: fixed decimal places as configured.
    format!("{:.*}", decimals as usize, scaled)
}

fn rounds_to_zero(scaled: f64, decimals: u8) -> bool {
    // If rounding at the configured precision yields 0.0, treat as precision loss.
    let factor = 10f64.powi(decimals as i32);
    ((scaled * factor).round() / factor) == 0.0
}

// Auto-select the best prefix and return "<number> <prefix>".
// Caller appends the actual measured unit (e.g., "s", "Hz", etc.).
pub fn auto_format(
    value: f64,
    style: PrefixStyle,
) -> (String, SIPrefix, Option<FormatWarning>) {
    auto_format_with_options(value, style, &FormatOptions::default())
}

// Same as auto_format, but allows customization of decimals and auto-target range.
pub fn auto_format_with_options(
    value: f64,
    style: PrefixStyle,
    opts: &FormatOptions,
) -> (String, SIPrefix, Option<FormatWarning>) {
    let prefix = choose_best_prefix(value, opts);
    let (s, w) = format_in_prefix_with_options(value, prefix, style, opts);
    (s, prefix, w)
}

// Force a specific prefix and return "<number> <prefix>".
// Caller appends the actual measured unit (e.g., "s", "Hz", etc.).
pub fn format_in_prefix(
    value: f64,
    prefix: SIPrefix,
    style: PrefixStyle,
) -> (String, Option<FormatWarning>) {
    format_in_prefix_with_options(
        value,
        prefix,
        style,
        &FormatOptions::default(),
    )
}

// Same as format_in_prefix, but allows customization of decimals and auto-target range.
// Note: auto-target range does not affect forced-prefix formatting; it is included for API symmetry.
pub fn format_in_prefix_with_options(
    value: f64,
    prefix: SIPrefix,
    style: PrefixStyle,
    opts: &FormatOptions,
) -> (String, Option<FormatWarning>) {
    let scaled = scale_value(value, prefix);
    let number = format_scaled_number(scaled, opts.decimals);
    let pfx = prefix_label(prefix, style);

    let warning = if value != 0.0 && rounds_to_zero(scaled, opts.decimals) {
        Some(FormatWarning::PrecisionLoss)
    } else {
        None
    };

    // Always return a space-delimited "<number> <prefix>" form.
    // Prefix may be empty for Base; caller can trim or handle unit concatenation.
    let out = format!("{} {}", number, pfx);
    (out, warning)
}
