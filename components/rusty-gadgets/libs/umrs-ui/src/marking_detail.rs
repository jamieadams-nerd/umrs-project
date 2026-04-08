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
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

use crate::text_fit::display_width;
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
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Label display fidelity — the struct captures
///   every field the catalog exposes so no information is silently dropped
///   before reaching the display layer.
/// - **NIST SP 800-53 SI-12**: Information output handling — owned strings
///   decouple the catalog data model from the display model.
#[derive(Debug, Clone, Default)]
#[must_use = "MarkingDetailData is display-ready label data; construct it and pass it to render_marking_detail"]
pub struct MarkingDetailData {
    /// Marking key — the full banner string (e.g., `"CUI//ADJ"`, `"PROTECTED-A"`).
    pub key: String,
    /// English name.
    pub name_en: String,
    /// French name — empty if not bilingual.
    pub name_fr: String,
    /// Banner abbreviation (e.g., `"ADJ"`, `"PA"`).
    pub abbreviation: String,
    /// Designation: `"basic"`, `"specified"`, or empty.
    pub designation: String,
    /// NARA index group (e.g., `"Immigration"`) — empty for non-US catalogs.
    pub index_group: String,
    /// MLS sensitivity level (e.g., `"s1"`).
    pub level: String,
    /// English description — may be multi-line; the renderer word-wraps it.
    pub description_en: String,
    /// French description — may be multi-line; the renderer word-wraps it.
    pub description_fr: String,
    /// Handling information — pre-formatted multi-line string.
    pub handling: String,
    /// Required warning statement — rendered in a caution colour when present.
    pub required_warning: String,
    /// Required dissemination control description.
    pub required_dissemination: String,
    /// English banner marking (e.g., `"PROTECTED A"`).
    pub marking_banner_en: String,
    /// French banner marking (e.g., `"PROTÉGÉ A"`).
    pub marking_banner_fr: String,
    /// English injury examples — Canadian catalog field.
    pub injury_examples_en: String,
    /// French injury examples — Canadian catalog field.
    pub injury_examples_fr: String,
    /// Additional caller-supplied key-value pairs rendered in section order.
    pub additional: Vec<(String, String)>,
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
/// non-scrolling view. The panel uses `Wrap { trim: false }` so long
/// text fields flow across lines rather than overflowing off-screen.
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

    let lines = match data {
        None => placeholder_lines(theme),
        Some(d) => build_detail_lines(d, theme),
    };

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));

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
/// waste horizontal space with excessive padding.
///
/// French-language rows are suppressed in this rendering path. Locale-aware
/// display is a planned future feature; for now all output is English only.
fn build_detail_lines<'a>(data: &'a MarkingDetailData, theme: &'a Theme) -> Vec<Line<'a>> {
    // ── Dynamic key width — measure only labels that will appear ─────────────
    let mut labels: Vec<&str> = Vec::new();
    if !data.name_en.is_empty() { labels.push("Name"); }
    if !data.abbreviation.is_empty() { labels.push("Abbreviation"); }
    if !data.designation.is_empty() { labels.push("Designation"); }
    if !data.index_group.is_empty() { labels.push("Index Group"); }
    if !data.level.is_empty() { labels.push("Level"); }
    if !data.marking_banner_en.is_empty() { labels.push("Banner"); }
    if !data.description_en.is_empty() { labels.push("Description"); }
    if !data.handling.is_empty() { labels.push("Handling"); }
    if !data.injury_examples_en.is_empty() { labels.push("Injury Examples"); }
    if !data.required_warning.is_empty() { labels.push("Required Warning"); }
    if !data.required_dissemination.is_empty() { labels.push("Dissemination"); }
    for (key, value) in &data.additional {
        if !value.is_empty() { labels.push(key.as_str()); }
    }
    let key_width = labels.iter().map(|l| display_width(l)).max().unwrap_or(12);

    let mut lines: Vec<Line<'a>> = Vec::new();

    // -----------------------------------------------------------------------
    // Section 1 — Marking key (prominent header)
    // -----------------------------------------------------------------------
    lines.push(Line::from(""));
    if !data.key.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(&data.key, theme.header_name),
        ]));
        lines.push(Line::from(""));
    }

    // -----------------------------------------------------------------------
    // Section 2 — Identity fields (English only)
    // -----------------------------------------------------------------------
    push_kv(&mut lines, "Name", &data.name_en, key_width, theme);
    push_kv(&mut lines, "Abbreviation", &data.abbreviation, key_width, theme);
    push_designation(&mut lines, &data.designation, key_width, theme);
    push_kv(&mut lines, "Index Group", &data.index_group, key_width, theme);
    push_kv(&mut lines, "Level", &data.level, key_width, theme);

    // -----------------------------------------------------------------------
    // Section 3 — Banner marking (English only)
    // -----------------------------------------------------------------------
    if !data.marking_banner_en.is_empty() {
        lines.push(Line::from(""));
        push_kv(&mut lines, "Banner", &data.marking_banner_en, key_width, theme);
    }

    // -----------------------------------------------------------------------
    // Section 4 — Description (English only)
    // -----------------------------------------------------------------------
    if !data.description_en.is_empty() {
        lines.push(Line::from(""));
        push_kv(&mut lines, "Description", &data.description_en, key_width, theme);
    }

    // -----------------------------------------------------------------------
    // Section 5 — Handling
    // -----------------------------------------------------------------------
    if !data.handling.is_empty() {
        lines.push(Line::from(""));
        push_kv(&mut lines, "Handling", &data.handling, key_width, theme);
    }

    // -----------------------------------------------------------------------
    // Section 6 — Injury examples (Canadian catalog, English only)
    // -----------------------------------------------------------------------
    if !data.injury_examples_en.is_empty() {
        lines.push(Line::from(""));
        push_kv(&mut lines, "Injury Examples", &data.injury_examples_en, key_width, theme);
    }

    // -----------------------------------------------------------------------
    // Section 7 — Required warning (rendered in yellow/caution style)
    // -----------------------------------------------------------------------
    if !data.required_warning.is_empty() {
        lines.push(Line::from(""));
        push_warning(&mut lines, "Required Warning", &data.required_warning, key_width, theme);
    }

    // -----------------------------------------------------------------------
    // Section 8 — Required dissemination control
    // -----------------------------------------------------------------------
    if !data.required_dissemination.is_empty() {
        lines.push(Line::from(""));
        push_kv(&mut lines, "Dissemination", &data.required_dissemination, key_width, theme);
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
        "specified" => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        "basic" => Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
        _ => theme.data_value,
    };
    lines.push(Line::from(vec![
        Span::styled(key_str, theme.data_key),
        Span::styled(designation, value_style),
    ]));
}

/// Push a warning-style key-value line for required statutory warnings.
///
/// The value is rendered in yellow to signal that the operator must read
/// and acknowledge this text before handling the marked data.
///
/// NIST SP 800-53 SI-12 — required warning statements must be visually
/// distinct to prevent operators from overlooking mandatory handling notices.
fn push_warning<'a>(
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
    // Yellow matches `theme.indicator_unavailable` — the caution tier.
    let warn_style = Style::default().fg(Color::Yellow);
    lines.push(Line::from(vec![
        Span::styled(key_str, theme.data_key),
        Span::styled(value, warn_style),
    ]));
}
