// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # CUI Marking Color Palette
//!
//! Maps NARA CUI index groups to terminal true-color (24-bit RGB) background
//! and foreground colors for marking chips and group headers.
//!
//! Colors are sourced from `config/us/US-CUI-PALETTE.json`, one entry per
//! index group. Canadian markings (no matching index group) fall through to
//! the default `cui_purple`.
//!
//! ## Color Constraints
//!
//! Red and orange are prohibited in CUI palettes per the `labeling_mcs` rules.
//! In Five Eyes classified systems, red signifies SECRET and orange signifies
//! TOP SECRET. Using these colors for unclassified CUI markings would create
//! dangerous visual confusion for operators who work across classification levels.
//!
//! ## Key Exports
//!
//! - [`palette_bg`] — background color for a marking chip keyed by index group
//! - [`palette_fg`] — foreground color for a marking chip keyed by index group
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — palette colors visually
//!   reinforce the index group classification for rapid operator identification.

use ratatui::style::Color;

/// Map an index group name to a truecolor background for the marking chip.
///
/// All backgrounds are dark enough that white foreground text is legible,
/// except Financial gold and Natural/Cultural green which require black text —
/// use [`palette_fg`] to obtain the correct foreground for each background.
///
/// Callers that do not have an index group (e.g., group headers that only carry
/// a marking string) should pass an empty string `""` to obtain the default
/// `cui_purple` fallback color.
///
/// NIST SP 800-53 AC-16 — palette colors visually reinforce the index group
/// classification of each marking for rapid operator identification.
#[must_use = "background color is computed but not applied; pass to Style::bg()"]
pub fn palette_bg(index_group: &str) -> Color {
    match index_group {
        "Critical Infrastructure" => Color::Rgb(0x4A, 0x6A, 0x20), // olive_brass
        "Defense" | "Export Control" => Color::Rgb(0x46, 0x82, 0xB4), // cti_steel
        "Financial" => Color::Rgb(0xD6, 0xA3, 0x00),               // finance_gold
        "Immigration" | "International Agreements" | "Legal" => {
            Color::Rgb(0x5B, 0x7C, 0x99) // govt_blue_gray
        }
        "Intelligence" => Color::Rgb(0x4B, 0x2E, 0x83), // intel_purple
        "Law Enforcement" => Color::Rgb(0x1F, 0x4E, 0x79), // police_blue
        "Natural and Cultural Resources" => Color::Rgb(0x70, 0xAD, 0x47), // agriculture_green
        "Nuclear" => Color::Rgb(0x1B, 0x3A, 0x5C),      // nnpi_navy
        "Patent" | "Statistical" => Color::Rgb(0x6E, 0x6E, 0x6E), // research_gray
        "Privacy" => Color::Rgb(0x8B, 0x3A, 0x62),      // privacy_rose
        "Procurement and Acquisition" => Color::Rgb(0x4A, 0x55, 0x68), // procure_slate
        "Proprietary Business Information" => Color::Rgb(0x7A, 0x6B, 0x5A), // warm_taupe
        "Tax" => Color::Rgb(0x55, 0x6B, 0x2F),          // tax_olive
        "Transportation" => Color::Rgb(0x2D, 0x2D, 0x2D), // opsec_charcoal
        _ => Color::Rgb(0x6A, 0x3D, 0x9A),              // cui_purple (default / CA)
    }
}

/// Map an index group name to a truecolor foreground for the marking chip.
///
/// Most palette backgrounds are dark enough to pair with white. Financial gold
/// and Natural/Cultural green are lighter backgrounds that require black text
/// for legible contrast.
///
/// NIST SP 800-53 AC-16 — foreground contrast ensures marking text remains
/// readable in all terminal environments.
#[must_use = "foreground color is computed but not applied; pass to Style::fg()"]
pub fn palette_fg(index_group: &str) -> Color {
    match index_group {
        "Financial" | "Natural and Cultural Resources" => {
            Color::Rgb(0x00, 0x00, 0x00) // black fg for light backgrounds
        }
        _ => Color::Rgb(0xFF, 0xFF, 0xFF), // white fg
    }
}
