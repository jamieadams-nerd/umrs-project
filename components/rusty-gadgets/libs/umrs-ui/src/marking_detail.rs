// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Marking Detail — Reusable CUI/Protected Label Detail Panel
//!
//! Renders a scrollable detail panel for a single security marking entry.
//!
//! The primary export is [`render_marking_detail`], which accepts a
//! [`MarkingDetailData`] value and renders it into a ratatui [`Frame`].
//! The data struct carries pre-formatted, fully owned strings — callers
//! transform their domain types (catalog `Marking`, `setrans` context, etc.)
//! into `MarkingDetailData` before calling the renderer. The renderer
//! performs layout and styling only; it performs no data transformation.
//!
//! This module lives in `umrs-ui` so that both `umrs-label` (the security
//! label registry TUI) and `umrs-ls` (the directory browser) can show the
//! same detail panel without any circular dependency between those crates.
//!
//! ## Language Display
//!
//! Only English-language fields are rendered in the current implementation.
//! French-language rows are suppressed pending locale-aware display support.
//! The data struct retains bilingual fields so that locale switching can be
//! added without a schema change.
//!
//! ## Section Order
//!
//! 1. Marking key (prominent, `theme.header_name` style)
//! 2. Name (en) / Nom (fr)
//! 3. Abbreviation
//! 4. Designation — yellow for "specified", green for "basic"
//! 5. Index group
//! 6. Level
//! 7. Banner marking (en / fr)
//! 8. Description (en) / Description (fr)
//! 9. Handling
//! 10. Injury examples (en / fr)
//! 11. Required warning
//! 12. Required dissemination control
//! 13. Additional fields (caller-supplied key-value pairs)
//!
//! ## Key Exports
//!
//! - [`MarkingDetailData`] — pre-formatted input struct
//! - [`render_marking_detail`] — top-level render function
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Label display fidelity — every field present
//!   in the catalog entry is rendered without omission or truncation.
//! - **NIST SP 800-53 SI-12**: Information output handling — the renderer
//!   separates data preparation (caller's responsibility) from display
//!   (this module's responsibility), preventing raw catalog internals from
//!   reaching the presentation layer unfiltered.
//! - **NIST SP 800-53 AU-3**: Every rendered panel is self-identifying via
//!   the marking key header; no field is rendered without a label.
//! - **NSA RTB**: No I/O, no allocation beyond ratatui `Line` construction;
//!   the renderer is a pure layout function.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::palette::{palette_bg, palette_fg};
use crate::text_fit::{display_width, wrap_indented};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Separator string placed between every key label and its value.
const KEY_SEP: &str = " : ";

// ---------------------------------------------------------------------------
// Data struct
// ---------------------------------------------------------------------------

/// Pre-formatted data for the marking detail panel.
///
/// Callers construct this from their domain types (catalog `Marking`,
/// `setrans` context, etc.) and pass it to [`render_marking_detail`]. The
/// renderer performs layout and styling only — no data transformation.
///
/// All string fields are owned. Empty strings signal "not present" and
/// are silently skipped by the renderer so no blank rows appear.
///
/// The `additional` field carries caller-supplied key-value pairs that do
/// not fit the fixed schema (e.g., nation-specific fields). They are
/// rendered last, in the order provided.
///
/// ## Fields:
///
/// - `key` — marking key; the full banner string (e.g., `"CUI//ADJ"`, `"PROTECTED-A"`).
/// - `name_en` — English name.
/// - `name_fr` — French name; empty if not bilingual.
/// - `abbreviation` — banner abbreviation (e.g., `"ADJ"`, `"PA"`).
/// - `designation` — `"basic"`, `"specified"`, or empty.
/// - `index_group` — NARA index group (e.g., `"Immigration"`); empty for non-US catalogs.
/// - `level` — MLS sensitivity level (e.g., `"s1"`).
/// - `description_en` — English description; may be multi-line; the renderer word-wraps it.
/// - `description_fr` — French description; may be multi-line; the renderer word-wraps it.
/// - `handling` — handling information; pre-formatted multi-line string.
/// - `required_warning` — required warning statement; rendered in a caution colour when present.
/// - `required_dissemination` — required dissemination control description.
/// - `marking_banner_en` — English banner marking (e.g., `"PROTECTED A"`).
/// - `marking_banner_fr` — French banner marking (e.g., `"PROTÉGÉ A"`).
/// - `injury_examples_en` — English injury examples; Canadian catalog field.
/// - `injury_examples_fr` — French injury examples; Canadian catalog field.
/// - `additional` — caller-supplied key-value pairs rendered in section order.
/// - `country_flag` — country flag emoji (e.g., `"🇺🇸"`, `"🇨🇦"`); rendered flush-right on the
///   marking key line; empty when the catalog has no country code.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: label display fidelity — the struct captures every field the
///   catalog exposes so no information is silently dropped before reaching the display layer.
/// - **NIST SP 800-53 SI-12**: information output handling — owned strings decouple the catalog
///   data model from the display model.
#[derive(Debug, Clone, Default)]
#[must_use = "MarkingDetailData is display-ready label data; construct it and pass it to render_marking_detail"]
pub struct MarkingDetailData {
    pub key: String,
    pub name_en: String,
    pub name_fr: String,
    pub abbreviation: String,
    pub designation: String,
    pub index_group: String,
    pub level: String,
    pub description_en: String,
    pub description_fr: String,
    pub handling: String,
    pub required_warning: String,
    pub required_dissemination: String,
    pub marking_banner_en: String,
    pub marking_banner_fr: String,
    pub injury_examples_en: String,
    pub injury_examples_fr: String,
    pub additional: Vec<(String, String)>,
    pub country_flag: String,
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Render the marking detail panel into `area`.
///
/// When `data` is `None` (no marking selected), a placeholder line is
/// shown. When `data` is `Some`, all non-empty fields are rendered in
/// section order with bilingual rows where both variants are present.
///
/// `scroll_offset` scrolls the entire panel content — pass `0` for a
/// non-scrolling view. Multi-line text fields are manually word-wrapped
/// to the available column budget so continuation lines remain indented
/// with their originating label.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Every field present in `data` is rendered
///   without omission.
/// - **NIST SP 800-53 AU-3**: Panel is titled "Details" so operators can
///   always identify what they are looking at.
pub fn render_marking_detail(
    frame: &mut Frame,
    area: Rect,
    data: Option<&MarkingDetailData>,
    scroll_offset: u16,
    theme: &Theme,
) {
    let block = Block::default()
        .title(" Details ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    // The inner area subtracts the 2-column border (left + right).
    let inner_width = (area.width as usize).saturating_sub(2);
    let lines = match data {
        None => placeholder_lines(theme),
        Some(d) => build_detail_lines(d, theme, inner_width),
    };

    let paragraph = Paragraph::new(lines).block(block).scroll((scroll_offset, 0));

    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------------
// Line builders
// ---------------------------------------------------------------------------

/// Build the placeholder content shown when no marking is selected.
fn placeholder_lines(theme: &Theme) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Select a marking to view details.",
            theme.data_value,
        )),
    ]
}

/// Build all content lines for a populated `MarkingDetailData`.
///
/// Empty fields are silently skipped. Sections that consist entirely of
/// empty fields produce no output at all (no trailing blank lines).
///
/// The key column width is computed dynamically from the labels that will
/// actually appear, so short-label entries (e.g., "Name", "Level") do not
/// waste horizontal space with excessive padding.  Multi-line text fields
/// (Description, Handling, Injury Examples, Required Warning, Required
/// Dissemination) are rendered as a label on its own line followed by the
/// text with a 4-space indent — matching the layout used in the `umrs-label`
/// registry browser so operators see consistent presentation across tools.
///
/// French-language rows are suppressed in this rendering path. Locale-aware
/// display is a planned future feature; for now all output is English only.
///
/// This function is `pub` so that callers that already own a border block
/// (e.g., `umrs-ls`'s label detail popup) can render content directly inside
/// their own block rather than accepting the nested `Details` border that
/// [`render_marking_detail`] would otherwise produce.
#[must_use = "build_detail_lines returns the rendered content lines; pass them to a Paragraph widget"]
#[expect(
    clippy::too_many_lines,
    reason = "each section corresponds to a distinct CUI label field; \
              splitting into sub-functions would scatter the field ordering and break \
              the structural documentation embedded in the section comments"
)]
pub fn build_detail_lines<'a>(
    data: &'a MarkingDetailData,
    theme: &'a Theme,
    max_width: usize,
) -> Vec<Line<'a>> {
    // ── Dynamic key width — measure only key-value (non-multi-line) labels ────
    // Multi-line fields (Description, Handling, Injury Examples, Required Warning,
    // Dissemination) are rendered as label-then-block rather than key : value
    // pairs, so they do not contribute to the key column width calculation.
    let mut labels: Vec<&str> = Vec::new();
    if !data.name_en.is_empty() {
        labels.push("Name");
    }
    if !data.abbreviation.is_empty() {
        labels.push("Abbreviation");
    }
    if !data.designation.is_empty() {
        labels.push("Designation");
    }
    if !data.index_group.is_empty() {
        labels.push("Index Group");
    }
    if !data.level.is_empty() {
        labels.push("Level");
    }
    if !data.marking_banner_en.is_empty() {
        labels.push("Banner");
    }
    for (key, value) in &data.additional {
        if !value.is_empty() {
            labels.push(key.as_str());
        }
    }
    let key_width = labels.iter().map(|l| display_width(l)).max().unwrap_or(12);

    let mut lines: Vec<Line<'a>> = Vec::new();

    // -----------------------------------------------------------------------
    // Section 1 — Marking key (palette chip), country flag flush-right
    //
    // System-level markings (SystemLow, SystemHigh, etc.) carry
    // designation == "system" and no index group — they use the theme
    // default style.  CUI and Protected markings use the CUI palette chip
    // (NIST SP 800-53 AC-16: palette colors reinforce index-group identity).
    // -----------------------------------------------------------------------
    lines.push(Line::from(""));
    if !data.key.is_empty() {
        let key_style = if data.designation == "system" || data.index_group.is_empty() {
            theme.header_name
        } else {
            Style::default()
                .fg(palette_fg(&data.index_group))
                .bg(palette_bg(&data.index_group))
                .add_modifier(Modifier::BOLD)
        };
        // Pad the key with a leading and trailing space so the palette chip
        // background has breathing room around the text.
        let padded_key = format!(" {} ", data.key);
        let key_display_width = 2 + display_width(&data.key); // "  " prefix + key
        let mut spans = vec![Span::raw("  "), Span::styled(padded_key, key_style)];
        if !data.country_flag.is_empty() {
            let flag_width = display_width(&data.country_flag);
            // key_display_width includes the "  " indent plus the padded key
            // (" key " = 2 + key_len). Calculate gap to right-align the flag.
            let used = key_display_width + 2; // +2 for the " " padding on both sides
            let gap = max_width.saturating_sub(used + flag_width + 1); // +1 right padding
            if gap > 0 {
                spans.push(Span::raw(" ".repeat(gap)));
            }
            spans.push(Span::styled(data.country_flag.clone(), theme.data_value));
        }
        lines.push(Line::from(spans));
        lines.push(Line::from(""));
    }

    // -----------------------------------------------------------------------
    // Section 2 — Identity fields (English only)
    // -----------------------------------------------------------------------
    push_kv(&mut lines, "Name", &data.name_en, key_width, theme);
    push_kv(
        &mut lines,
        "Abbreviation",
        &data.abbreviation,
        key_width,
        theme,
    );
    push_designation(&mut lines, &data.designation, key_width, theme);
    push_kv(
        &mut lines,
        "Index Group",
        &data.index_group,
        key_width,
        theme,
    );
    push_kv(&mut lines, "Level", &data.level, key_width, theme);

    // -----------------------------------------------------------------------
    // Section 3 — Banner marking (English only)
    // -----------------------------------------------------------------------
    if !data.marking_banner_en.is_empty() {
        lines.push(Line::from(""));
        push_kv(
            &mut lines,
            "Banner",
            &data.marking_banner_en,
            key_width,
            theme,
        );
    }

    // -----------------------------------------------------------------------
    // Section 4 — Description (English only)
    // Multi-line: label on its own line, text manually word-wrapped below
    // with a 4-space indent.  Manual wrapping ensures continuation lines
    // remain aligned with the first line rather than resetting to column 0.
    // -----------------------------------------------------------------------
    if !data.description_en.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  Description", theme.data_key)));
        lines.extend(wrap_indented(
            &data.description_en,
            "    ",
            max_width,
            theme.data_value,
        ));
    }

    // -----------------------------------------------------------------------
    // Section 5 — Handling
    // Multi-line: label on its own line, text manually word-wrapped below.
    // -----------------------------------------------------------------------
    if !data.handling.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  Handling", theme.data_key)));
        lines.extend(wrap_indented(
            &data.handling,
            "    ",
            max_width,
            theme.data_value,
        ));
    }

    // -----------------------------------------------------------------------
    // Section 6 — Injury examples (Canadian catalog, English only)
    // Multi-line: label on its own line, text manually word-wrapped below.
    // -----------------------------------------------------------------------
    if !data.injury_examples_en.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Injury Examples",
            theme.data_key,
        )));
        lines.extend(wrap_indented(
            &data.injury_examples_en,
            "    ",
            max_width,
            theme.data_value,
        ));
    }

    // -----------------------------------------------------------------------
    // Section 7 — Required warning (rendered in yellow/caution style)
    // Multi-line: label on its own line, text manually word-wrapped below.
    // -----------------------------------------------------------------------
    if !data.required_warning.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Required Warning",
            theme.data_key,
        )));
        let warn_style = Style::default().fg(Color::Yellow);
        lines.extend(wrap_indented(
            &data.required_warning,
            "    ",
            max_width,
            warn_style,
        ));
    }

    // -----------------------------------------------------------------------
    // Section 8 — Required dissemination control
    // Multi-line: label on its own line, text manually word-wrapped below.
    // -----------------------------------------------------------------------
    if !data.required_dissemination.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  Dissemination", theme.data_key)));
        lines.extend(wrap_indented(
            &data.required_dissemination,
            "    ",
            max_width,
            theme.data_value,
        ));
    }

    // -----------------------------------------------------------------------
    // Section 9 — Additional caller-supplied fields
    // -----------------------------------------------------------------------
    if !data.additional.is_empty() {
        lines.push(Line::from(""));
        for (key, value) in &data.additional {
            if !value.is_empty() {
                push_kv_owned(&mut lines, key, value, key_width, theme);
            }
        }
    }

    // Trailing blank line for visual padding before the border.
    lines.push(Line::from(""));

    lines
}

// ---------------------------------------------------------------------------
// Row helpers
// ---------------------------------------------------------------------------

/// Push a single key-value line if `value` is non-empty.
///
/// The key label is right-padded to `key_width` characters and separated
/// from the value by `KEY_SEP`. The width is passed dynamically so the
/// column is sized to the widest label that actually appears on screen,
/// avoiding wasted horizontal space.
fn push_kv<'a>(
    lines: &mut Vec<Line<'a>>,
    key: &str,
    value: &'a str,
    key_width: usize,
    theme: &'a Theme,
) {
    if value.is_empty() {
        return;
    }
    let key_str = format!("  {key:<key_width$}{KEY_SEP}");
    lines.push(Line::from(vec![
        Span::styled(key_str, theme.data_key),
        Span::styled(value, theme.data_value),
    ]));
}

/// Push a single key-value line where both key and value are owned.
///
/// Used for `additional` fields where the key is a runtime `String`, not
/// a `'static` reference.
fn push_kv_owned<'a>(
    lines: &mut Vec<Line<'a>>,
    key: &str,
    value: &'a str,
    key_width: usize,
    theme: &'a Theme,
) {
    if value.is_empty() {
        return;
    }
    let key_str = format!("  {key:<key_width$}{KEY_SEP}");
    lines.push(Line::from(vec![
        Span::styled(key_str, theme.data_key),
        Span::styled(value, theme.data_value),
    ]));
}

/// Push the designation row with contextual colour coding.
///
/// - `"specified"` → yellow (draws attention; specified categories carry
///   additional statutory obligations beyond the CUI Basic baseline).
/// - `"basic"` → green (standard handling; no additional obligations).
/// - Any other non-empty value → default value style (white).
///
/// The colour distinction helps operators immediately recognise specified
/// categories that require heightened handling.
///
/// NIST SP 800-53 AC-3 — visual differentiation between Basic and Specified
/// handling requirements reduces operator error during access decisions.
fn push_designation<'a>(
    lines: &mut Vec<Line<'a>>,
    designation: &'a str,
    key_width: usize,
    theme: &'a Theme,
) {
    if designation.is_empty() {
        return;
    }
    let key_str = format!("  {:<key_width$}{KEY_SEP}", "Designation");
    let value_style = match designation {
        "specified" => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        "basic" => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        _ => theme.data_value,
    };
    lines.push(Line::from(vec![
        Span::styled(key_str, theme.data_key),
        Span::styled(designation, value_style),
    ]));
}
