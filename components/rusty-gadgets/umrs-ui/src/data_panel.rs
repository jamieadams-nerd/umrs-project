// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Data Panel — Scrollable Key-Value Data Area
//!
//! Renders a bordered, scrollable list of [`DataRow`] entries. Keys are
//! displayed in dim-cyan; values are colored per their [`StyleHint`].
//!
//! A ratatui `Scrollbar` is rendered on the right edge when the content
//! exceeds the visible area height.
//!
//! When a tab provides pinned rows via [`AuditCardApp::pinned_rows`], the
//! data panel splits vertically: a fixed-height pinned section at the top
//! (no scrolling) and a scrollable evidence section below with its own
//! border. This keeps summary information always visible while the operator
//! reviews detailed evidence.
//!
//! When the scrollable section contains a [`DataRow::TableHeader`] as its
//! first row, that row is rendered as a sticky (non-scrolling) column label
//! bar at the top of the evidence pane using bold + reverse video styling.
//! The header remains visible at all scroll positions so the evidence table
//! is always self-labelling without requiring the operator to scroll back up.
//!
//! `IndicatorRow` entries (used by the Kernel Security tab) render as a
//! multi-line block: indicator name + value on the first line, followed by
//! a dim italic description wrapped to the available width and indented to
//! align under the value text, and a trailing blank line for visual
//! separation. The key column width is computed dynamically from all
//! `IndicatorRow` entries in the row list so no indicator name is ever
//! truncated regardless of catalog growth.
//!
//! Long values in `KeyValue` rows with an empty key (description rows) are
//! word-wrapped to the available panel width with 3 characters of right
//! padding so text never extends beyond the visible area.
//!
//! When a tab returns [`ColumnLayout::TwoColumn`] from
//! [`AuditCardApp::column_layout`], the data area is split horizontally 50/50.
//! The left half renders [`AuditCardApp::data_rows_left`] and the right half
//! renders [`AuditCardApp::data_rows_right`]. Both columns share the same
//! scroll offset from [`AuditCardState`] and each has its own scrollbar when
//! content overflows. Pinned rows (if any) remain full-width above both columns.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Structured key-value rows ensure every field
//!   is labelled; there is no ambiguous free-form data blob.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState,
};

use umrs_core::i18n;
use umrs_platform::posture::ContradictionKind;

use crate::app::{AuditCardApp, ColumnLayout, DataRow, StyleHint};
use crate::theme::{Theme, style_hint_color};

// ---------------------------------------------------------------------------
// Key column width
// ---------------------------------------------------------------------------

/// Fixed width of the key column (characters), including trailing padding.
const KEY_COL_WIDTH: usize = 20;

/// Minimum width of the key column for `IndicatorRow` entries (characters).
///
/// The dynamic scan in [`TableWidths::from_rows`] finds the actual maximum
/// key length across all `IndicatorRow` entries and uses this as the lower
/// bound. 20 characters is a conservative floor — the actual maximum is
/// computed from the data so future indicators with longer names are never
/// truncated.
const INDICATOR_KEY_MIN: usize = 20;

// ---------------------------------------------------------------------------
// Right padding for word-wrapped description rows
// ---------------------------------------------------------------------------

/// Number of characters reserved as right margin when wrapping long descriptions.
///
/// Prevents wrapped text from touching the panel border. Three characters gives
/// a comfortable visual margin without wasting significant display width.
const WRAP_RIGHT_PADDING: usize = 3;

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Render the data panel, optionally with a fixed pinned summary section above
/// a scrollable content area.
///
/// When `pinned_rows` is non-empty, the panel is split:
/// - A fixed-height bordered pane at the top displays the pinned rows.
/// - A scrollable bordered pane below displays `rows` with the scroll offset.
///
/// When `pinned_rows` is empty, the full area is used for the scrollable
/// content (the original single-pane layout).
///
/// When [`ColumnLayout::TwoColumn`] is active for the tab, the scrollable
/// area is split 50/50 horizontally. Each column has its own border and
/// scrollbar. Both columns share `scroll_offset`. Pinned rows (if any) remain
/// full-width above both columns.
///
/// `scroll_offset` applies only to the scrollable section. The pinned section
/// always shows from its beginning.
///
/// NIST SP 800-53 AU-3 — every data field is labelled; no ambiguous blobs.
pub fn render_data_panel(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    active_tab: usize,
    scroll_offset: usize,
    theme: &Theme,
) {
    let pinned = app.pinned_rows(active_tab);
    let layout = app.column_layout(active_tab);

    // Compute the scrollable area, splitting off a pinned section when present.
    let scroll_area = if pinned.is_empty() {
        area
    } else {
        // Compute pinned section height from content, then carve it off the top.
        let inner_width = (area.width as usize).saturating_sub(4);
        let pinned_line_count: usize = pinned
            .iter()
            .map(|r| expanded_row_line_count(r, inner_width))
            .sum();
        let pinned_height_raw = pinned_line_count.saturating_add(2); // +2 borders
        // Clamp: never use more than 40% of area for pinned content.
        let max_pinned =
            (area.height as usize).saturating_mul(2).saturating_div(5);
        let pinned_height = pinned_height_raw.min(max_pinned).max(4);

        #[allow(clippy::cast_possible_truncation)]
        let pinned_height_u16 = pinned_height as u16;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(pinned_height_u16),
                Constraint::Min(0),
            ])
            .split(area);

        let [pinned_area, remaining] = *chunks else {
            return;
        };

        render_pinned_pane(frame, pinned_area, &pinned, inner_width, theme);
        remaining
    };

    // Dispatch on layout mode for the scrollable content area.
    match layout {
        ColumnLayout::Full => {
            let rows = app.data_rows(active_tab);
            let has_pinned = !pinned.is_empty();
            render_scrollable_pane(
                frame,
                scroll_area,
                &rows,
                scroll_offset,
                theme,
                has_pinned,
            );
        }
        ColumnLayout::TwoColumn => {
            let left_rows = app.data_rows_left(active_tab);
            let right_rows = app.data_rows_right(active_tab);
            render_two_column_pane(
                frame,
                scroll_area,
                &left_rows,
                &right_rows,
                scroll_offset,
                theme,
            );
        }
    }
}

/// Render two independent side-by-side scrollable column panes.
///
/// The `area` is split 50/50 horizontally. Each half is rendered as a
/// separate bordered pane with its own scrollbar (when content overflows).
/// Both columns scroll by the same `scroll_offset` — independent scrolling
/// is deferred to a future enhancement.
///
/// Group titles, key-value rows, and separators are supported in both columns.
/// `IndicatorRow` and `TableRow` types are full-width by design and should not
/// be placed in two-column tabs; they will render correctly but may appear
/// cramped at half width.
///
/// NIST SP 800-53 AU-3 — column layout does not affect data completeness;
/// every labelled field is rendered in both columns at all scroll positions.
fn render_two_column_pane(
    frame: &mut Frame,
    area: Rect,
    left_rows: &[DataRow],
    right_rows: &[DataRow],
    scroll_offset: usize,
    theme: &Theme,
) {
    // Split the area 50/50 horizontally.
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let [left_area, right_area] = *cols else {
        return;
    };

    render_scrollable_pane(
        frame,
        left_area,
        left_rows,
        scroll_offset,
        theme,
        false,
    );
    render_scrollable_pane(
        frame,
        right_area,
        right_rows,
        scroll_offset,
        theme,
        false,
    );
}

/// Render a non-scrollable pinned summary pane.
///
/// The pinned pane has a rounded border with a `" Summary "` title. Rows are
/// expanded (with word-wrap for description rows) and rendered from top to
/// bottom. Overflow is clipped at the pane height — pinned content must be
/// concise.
fn render_pinned_pane(
    frame: &mut Frame,
    area: Rect,
    pinned: &[DataRow],
    inner_width: usize,
    theme: &Theme,
) {
    let inner_height = (area.height as usize).saturating_sub(2);
    // Pinned panes rarely contain table rows, but compute widths defensively
    // so that if a table row is ever pinned, alignment is correct.
    let widths = TableWidths::from_rows(pinned);

    let mut lines: Vec<Line<'_>> = Vec::new();
    for row in pinned {
        let expanded = expand_row_lines(row, inner_width, theme, widths);
        for line in expanded {
            if lines.len() >= inner_height {
                break;
            }
            lines.push(line);
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(" Summary ", theme.group_title));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

/// Render the scrollable main content pane.
///
/// `has_pinned` controls the border title: when the panel is split, the
/// scrollable section gets an `" Evidence Chain "` title to distinguish it
/// from the pinned summary pane above.
///
/// If the `rows` slice begins with a `DataRow::TableHeader`, that row is
/// extracted and rendered as a sticky (non-scrolling) row at the top of the
/// content area. The table header is always visible regardless of scroll
/// position — it serves as the column label row for the evidence table.
/// The sticky header is rendered with bold + reverse video to distinguish it
/// clearly from scrollable data rows.
///
/// All other rows are scrollable. The scrollbar gutter is removed from the
/// inner width so that word-wrap accounts for the scrollbar column.
///
/// NIST SP 800-53 AU-3 — column headers remain visible throughout scroll so
/// the evidence table is always self-labelling.
fn render_scrollable_pane(
    frame: &mut Frame,
    area: Rect,
    rows: &[DataRow],
    scroll_offset: usize,
    theme: &Theme,
    has_pinned: bool,
) {
    // Inner width available for content (subtract 2 for borders, 1 for
    // scrollbar gutter when applicable).
    let inner_width_base = (area.width as usize).saturating_sub(3); // borders + gutter

    // Compute dynamic table column widths from all rows (including header).
    // Scanning the full slice ensures the header and data rows use the same
    // column widths, preventing misalignment between the sticky header bar
    // and the scrollable data rows below it.
    let widths = TableWidths::from_rows(rows);

    // Check if the first row is a TableHeader — if so, extract it as the
    // sticky column-label row and exclude it from the scrollable content.
    let (sticky_header, scrollable_rows) = if let Some(DataRow::TableHeader {
        ..
    }) = rows.first()
    {
        (Some(&rows[0]), &rows[1..])
    } else {
        (None, rows)
    };

    // Reserve one inner line for the sticky header when present.
    let sticky_height: usize = usize::from(sticky_header.is_some());
    let inner_height = (area.height as usize)
        .saturating_sub(2) // borders
        .saturating_sub(sticky_height);

    // Expand scrollable rows to lines (applying word-wrap).
    let expanded: Vec<Vec<Line<'_>>> = scrollable_rows
        .iter()
        .map(|r| expand_row_lines(r, inner_width_base, theme, widths))
        .collect();

    let total_lines: usize = expanded.iter().map(Vec::len).sum();

    // Clamp scroll offset.
    let max_offset = total_lines.saturating_sub(inner_height);
    let offset = scroll_offset.min(max_offset);

    // Flatten and take the visible window of scrollable content.
    let scrollable_visible: Vec<Line<'_>> = expanded
        .into_iter()
        .flatten()
        .skip(offset)
        .take(inner_height)
        .collect();

    // Build the full visible line list: sticky header (if any) + scrollable body.
    let mut visible: Vec<Line<'_>> = Vec::new();
    if let Some(header_row) = sticky_header {
        // Render the header with bold + reverse video so it stands out as the
        // column label row. Uses header_field (bright cyan) rather than
        // data_key (dim cyan) so the reversed text is readable.
        let header_style = theme
            .header_field
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED);
        visible.push(build_table_header_line_styled(
            header_row,
            header_style,
            widths,
        ));
    }
    visible.extend(scrollable_visible);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    // No title on the scrollable pane — the sticky TableHeader row at the top
    // of the evidence section is self-labelling; a box title would be redundant.
    let _ = has_pinned;

    let paragraph = Paragraph::new(visible).block(block);

    // Render with scrollbar when scrollable content overflows.
    if total_lines > inner_height {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        frame.render_widget(paragraph, chunks[0]);

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(total_lines)
            .viewport_content_length(inner_height)
            .position(offset);

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight);

        frame.render_stateful_widget(
            scrollbar,
            chunks[1],
            &mut scrollbar_state,
        );
    } else {
        frame.render_widget(paragraph, area);
    }
}

/// Build a single styled line from a `TableHeader` row, applying `header_style`
/// to all three columns.
///
/// Used exclusively by the sticky header path in `render_scrollable_pane`.
/// All three columns share the same style (bold + reverse video) so the
/// header bar reads as a solid, clearly-distinguishable visual boundary.
///
/// Leading and trailing spaces are added inside each reversed header span so
/// the text does not butt against adjacent column content. The Evidence Type
/// column uses left-aligned padding (content flush-left) to align visually
/// with the group title rows rendered below it.
fn build_table_header_line_styled(
    row: &DataRow,
    header_style: ratatui::style::Style,
    widths: TableWidths,
) -> Line<'_> {
    let DataRow::TableHeader {
        col1,
        col2,
        col3,
    } = row
    else {
        // Defensive: caller guarantees this is a TableHeader; return empty line.
        return Line::from("");
    };
    // Add leading and trailing space inside each reversed span so the
    // highlighted labels do not butt against adjacent content. The col1
    // width is reduced by 2 to account for the added spaces; clip_pad
    // then right-pads to fill the column to keep alignment intact.
    let col1_inner = widths.col1.saturating_sub(2);
    let col2_inner = widths.col2.saturating_sub(2);
    let col1_str = format!(" {}", clip_pad(col1, col1_inner));
    let col2_str = format!(" {}", clip_pad(col2, col2_inner));
    let col3_str = format!(" {col3} ");
    Line::from(vec![
        Span::raw(" "),
        Span::styled(col1_str, header_style),
        Span::raw(TABLE_COL_GAP),
        Span::styled(col2_str, header_style),
        Span::raw(TABLE_COL_GAP),
        Span::styled(col3_str, header_style),
    ])
}

// ---------------------------------------------------------------------------
// Row expansion — single row → one or more lines
// ---------------------------------------------------------------------------

/// Compute how many display lines a row will expand to at `inner_width`.
///
/// Used to calculate pinned pane height before rendering. Must match the
/// expansion logic in `expand_row_lines` exactly.
fn expanded_row_line_count(row: &DataRow, inner_width: usize) -> usize {
    match row {
        DataRow::KeyValue {
            key,
            value,
            ..
        } if key.is_empty() => {
            // Description row — may wrap.
            word_wrap_count(value, inner_width)
        }
        DataRow::IndicatorRow {
            description,
            recommendation,
            contradiction,
            configured_line,
            ..
        } => {
            // 1 line for the key+value pair, N lines for wrapped description
            // (only when description is non-empty), 1 line for contradiction
            // marker (when present), 1 line for configured value (when present),
            // 1 line for recommendation (when present), and 1 trailing blank line.
            // This mirrors expand_row_lines exactly so pinned height is correct.
            let desc_lines = if description.is_empty() {
                0
            } else {
                word_wrap_count(description, inner_width)
            };
            let contradiction_lines = usize::from(contradiction.is_some());
            let configured_lines = usize::from(configured_line.is_some());
            let rec_lines = usize::from(recommendation.is_some());
            1usize
                .saturating_add(desc_lines)
                .saturating_add(contradiction_lines)
                .saturating_add(configured_lines)
                .saturating_add(rec_lines)
                .saturating_add(1)
        }
        DataRow::Separator
        | DataRow::GroupTitle(_)
        | DataRow::KeyValue {
            ..
        }
        | DataRow::TwoColumn {
            ..
        }
        | DataRow::TableRow {
            ..
        }
        | DataRow::TableHeader {
            ..
        } => 1,
    }
}

/// Expand a single [`DataRow`] into one or more styled [`Line`]s.
///
/// Description rows (key is empty) are word-wrapped to `inner_width` minus
/// `WRAP_RIGHT_PADDING`. `IndicatorRow` entries are delegated to
/// [`expand_indicator_row`]. All other rows produce exactly one line.
///
/// Description rows use `Modifier::ITALIC` to visually distinguish them from
/// data rows. This helps the operator quickly identify purpose text vs. values.
fn expand_row_lines<'a>(
    row: &'a DataRow,
    inner_width: usize,
    theme: &'a Theme,
    widths: TableWidths,
) -> Vec<Line<'a>> {
    match row {
        DataRow::KeyValue {
            key,
            value,
            style_hint,
            ..
        } if key.is_empty() => {
            // Description row: word-wrap the value and render in italic.
            let wrap_width = inner_width.saturating_sub(WRAP_RIGHT_PADDING);
            let wrap_width = wrap_width.max(20); // never wrap at < 20 chars
            let wrapped = word_wrap(value, wrap_width);
            let color = style_hint_color(*style_hint);
            let style =
                theme.data_value.fg(color).add_modifier(Modifier::ITALIC);
            wrapped
                .into_iter()
                .map(|chunk| {
                    Line::from(vec![
                        Span::raw("  "), // description indent
                        Span::styled(chunk, style),
                    ])
                })
                .collect()
        }
        DataRow::IndicatorRow {
            key,
            value,
            description,
            recommendation,
            contradiction,
            configured_line,
            style_hint,
        } => expand_indicator_row(
            key,
            value,
            description,
            *recommendation,
            *contradiction,
            configured_line.as_ref(),
            *style_hint,
            inner_width,
            theme,
            widths,
        ),
        other => vec![build_row_line(other, theme, widths)],
    }
}

/// Expand an `IndicatorRow` into its multi-line rendered form.
///
/// Rendering order:
/// 1. Key + live value (line 1 — always present)
/// 2. Description wrapped at `inner_width` (dim italic, indented)
/// 3. Contradiction marker with `⚠` symbol (when present — more urgent than recommendation)
/// 4. Configured-value line (when present — source attribution for the config file)
/// 5. `[ Recommended: <value> ]` (when present — remediation guidance)
/// 6. Trailing blank line (visual separation without explicit Separator rows)
///
/// The `⚠` symbol ensures the contradiction marker is visible without relying
/// on color alone (WCAG 1.4.1 / NO_COLOR compliance).
///
/// NIST SP 800-53 CA-7 — contradictions surface configuration drift in-line
/// so the assessor can see a finding without consulting a separate list.
/// NIST SP 800-53 CM-6 — remediation guidance and configured-value source
/// attribution are co-located with the failing indicator.
#[allow(clippy::too_many_arguments)]
fn expand_indicator_row<'a>(
    key: &str,
    value: &str,
    description: &'static str,
    recommendation: Option<&'static str>,
    contradiction: Option<ContradictionKind>,
    configured_line: Option<&String>,
    style_hint: StyleHint,
    inner_width: usize,
    theme: &'a Theme,
    widths: TableWidths,
) -> Vec<Line<'a>> {
    // Line 1: " " + key padded to indicator_key_col + ": " + value.
    let key_col = widths.indicator_key_col;
    let key_padded = pad_key(key, key_col);
    let value_color = style_hint_color(style_hint);
    let kv_line = Line::from(vec![
        Span::raw(" "),
        Span::styled(key_padded, theme.data_key),
        Span::styled(value.to_owned(), theme.data_value.fg(value_color)),
    ]);

    let mut lines = vec![kv_line];

    // Indent shared by description, contradiction, configured, and recommendation.
    // Indent = 1 (leading space) + key_col + 2 (": ").
    let indent_len = 1usize.saturating_add(key_col).saturating_add(2);
    let desc_indent = " ".repeat(indent_len);

    // Description lines: dim italic, wrapped to available width.
    if !description.is_empty() {
        let available = inner_width
            .saturating_sub(indent_len)
            .saturating_sub(WRAP_RIGHT_PADDING);
        let wrap_width = available.max(20);
        let wrapped = word_wrap(description, wrap_width);
        let desc_style = theme
            .data_value
            .fg(style_hint_color(StyleHint::Dim))
            .add_modifier(Modifier::ITALIC);
        for chunk in wrapped {
            lines.push(Line::from(vec![
                Span::raw(desc_indent.clone()),
                Span::styled(chunk, desc_style),
            ]));
        }
    }

    // Contradiction marker: shown before the recommendation because a
    // live/configured disagreement is more urgent than a hardening gap.
    //
    // Style by kind:
    //   BootDrift         → TrustRed    (config says hardened; kernel is not)
    //   EphemeralHotfix   → TrustYellow (hardened now; lost after reboot)
    //   SourceUnavailable → Dim         (cannot verify; not an active failure)
    if let Some(kind) = contradiction {
        let (marker_text, marker_hint) = match kind {
            ContradictionKind::BootDrift => (
                "\u{26A0} DRIFT: config says hardened, kernel is not",
                StyleHint::TrustRed,
            ),
            ContradictionKind::EphemeralHotfix => (
                "\u{26A0} NOT PERSISTED: hardened now, lost after reboot",
                StyleHint::TrustYellow,
            ),
            ContradictionKind::SourceUnavailable => (
                "\u{26A0} UNVERIFIABLE: config exists but kernel node unreadable",
                StyleHint::Dim,
            ),
        };
        let marker_style = theme
            .data_value
            .fg(style_hint_color(marker_hint))
            .add_modifier(Modifier::ITALIC);
        lines.push(Line::from(vec![
            Span::raw(desc_indent.clone()),
            Span::styled(marker_text, marker_style),
        ]));
    }

    // Configured-value line: shows what the persisted config says and which
    // file it came from. Rendered dim so it is clearly subordinate to the
    // live value on line 1.
    if let Some(cfg_line) = configured_line {
        let cfg_style = theme
            .data_value
            .fg(style_hint_color(StyleHint::Dim))
            .add_modifier(Modifier::ITALIC);
        lines.push(Line::from(vec![
            Span::raw(desc_indent.clone()),
            Span::styled(cfg_line.clone(), cfg_style),
        ]));
    }

    // Recommendation line: "[ Recommended: <value> ]" for unhardened indicators.
    if let Some(rec) = recommendation {
        let rec_style = theme
            .data_value
            .fg(style_hint_color(StyleHint::Dim))
            .add_modifier(Modifier::ITALIC);
        lines.push(Line::from(vec![
            Span::raw(desc_indent),
            Span::styled(format!("[ {}: {rec} ]", i18n::tr("Recommended")), rec_style),
        ]));
    }

    // Trailing blank line for visual separation between indicators.
    lines.push(Line::from(""));

    lines
}

// ---------------------------------------------------------------------------
// Word-wrap helpers
// ---------------------------------------------------------------------------

/// Split `text` into chunks of at most `width` characters, breaking on
/// whitespace where possible.
///
/// Words longer than `width` are emitted as-is on their own line (no
/// mid-word break). Empty text produces a single empty chunk so the row
/// is not silently omitted.
fn word_wrap(text: &str, width: usize) -> Vec<String> {
    if width == 0 || text.is_empty() {
        return vec![text.to_owned()];
    }

    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        let current_len = current.chars().count();

        if current_len > 0 {
            if current_len.saturating_add(1).saturating_add(word_len) <= width {
                // Word fits on the current line with a space separator.
                current.push(' ');
            } else {
                // Flush the current line and start a new one.
                lines.push(current.clone());
                current.clear();
            }
        }
        // Append word (first word on a new line, or after space/flush).
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Count how many lines `text` will produce when wrapped at `width`, applying
/// `WRAP_RIGHT_PADDING`. Mirrors `expand_row_lines` wrapping logic.
fn word_wrap_count(text: &str, inner_width: usize) -> usize {
    let wrap_width = inner_width.saturating_sub(WRAP_RIGHT_PADDING).max(20);
    word_wrap(text, wrap_width).len()
}

// ---------------------------------------------------------------------------
// Row builders
// ---------------------------------------------------------------------------

/// Half the standard key column width, used for each side of a `TwoColumn` row.
const HALF_KEY_COL_WIDTH: usize = KEY_COL_WIDTH / 2;

// ---------------------------------------------------------------------------
// Evidence table column widths
// ---------------------------------------------------------------------------

/// Minimum width of the `Evidence Type` column in a `TableRow` / `TableHeader`.
///
/// The longest plain-English evidence type label is `"Kernel attributes (/sys)"` at
/// 24 characters. 28 characters gives breathing room and prevents truncation of any
/// current or near-future label. The actual column width is computed dynamically
/// from the row data — this constant is the lower bound.
const TABLE_COL1_MIN: usize = 28;

/// Minimum width of the `Source` column in a `TableRow` / `TableHeader`.
///
/// Source paths vary in length. 36 characters fits the longest current paths
/// (e.g., `/proc/sys/kernel/random/boot_id` = 32 chars) with margin.
/// The actual column width is computed dynamically — this is the lower bound.
const TABLE_COL2_MIN: usize = 36;

/// Minimum space inserted between adjacent evidence table columns.
///
/// Prevents columns from visually merging when rendered in a terminal.
/// Two characters of padding give operators a clear visual break between
/// the Source and Verification columns without wasting display width.
const TABLE_COL_GAP: &str = "  ";

// ---------------------------------------------------------------------------
// Dynamic table column widths
// ---------------------------------------------------------------------------

/// Computed column widths for the evidence table and indicator key column.
///
/// Derived once per render pass by scanning all rows in the list. Using the
/// actual data prevents truncation when source paths, evidence type labels,
/// or indicator names are longer than the minimum constants.
///
/// `col1` and `col2` are for the three-column evidence table (`TableRow` and
/// `TableHeader`). `indicator_key_col` is the key column width for
/// `IndicatorRow` entries — computed from the longest indicator name so that
/// indicator keys are never truncated regardless of catalog size.
#[derive(Debug, Clone, Copy)]
struct TableWidths {
    col1: usize,
    col2: usize,
    /// Width of the key column for `IndicatorRow` entries.
    ///
    /// Equal to the length of the longest `IndicatorRow::key` string found in
    /// the row list, clamped to `INDICATOR_KEY_MIN`. Used to align all
    /// indicator values at a consistent column regardless of key length.
    indicator_key_col: usize,
}

impl TableWidths {
    /// Compute the minimum sufficient column widths from a row slice.
    ///
    /// Scans all `TableRow`, `TableHeader`, and `IndicatorRow` rows, finds the
    /// maximum character count for each column, then clamps to the minimum
    /// constants.
    fn from_rows(rows: &[DataRow]) -> Self {
        let mut col1 = TABLE_COL1_MIN;
        let mut col2 = TABLE_COL2_MIN;
        let mut indicator_key_col = INDICATOR_KEY_MIN;
        for row in rows {
            match row {
                DataRow::TableRow {
                    col1: c1,
                    col2: c2,
                    ..
                }
                | DataRow::TableHeader {
                    col1: c1,
                    col2: c2,
                    ..
                } => {
                    let c1_len = c1.chars().count();
                    let c2_len = c2.chars().count();
                    if c1_len > col1 {
                        col1 = c1_len;
                    }
                    if c2_len > col2 {
                        col2 = c2_len;
                    }
                }
                DataRow::IndicatorRow {
                    key,
                    ..
                } => {
                    let key_len = key.chars().count();
                    if key_len > indicator_key_col {
                        indicator_key_col = key_len;
                    }
                }
                _ => {}
            }
        }
        Self {
            col1,
            col2,
            indicator_key_col,
        }
    }
}

/// Build the [`Line`] for a `DataRow::KeyValue` row.
///
/// When `highlight_key` is true, the key span uses `theme.header_field`
/// (bright cyan) instead of `theme.data_key` (dim cyan). This makes summary
/// labels like `"Kernel Version"` and `"Indicators"` visually match the
/// header area label styling for visual consistency.
fn build_key_value_line<'a>(
    key: &str,
    value: &str,
    style_hint: StyleHint,
    highlight_key: bool,
    theme: &'a Theme,
) -> Line<'a> {
    let key_padded = pad_key(key, KEY_COL_WIDTH);
    let value_color = style_hint_color(style_hint);
    let key_style = if highlight_key {
        theme.header_field
    } else {
        theme.data_key
    };
    Line::from(vec![
        Span::raw(" "),
        Span::styled(key_padded, key_style),
        Span::styled(value.to_owned(), theme.data_value.fg(value_color)),
    ])
}

/// Build a single [`Line`] from a [`DataRow`].
///
/// Each variant produces a line that fits within the panel's text area:
/// - `KeyValue` — key padded to `KEY_COL_WIDTH`, then value.
/// - `TwoColumn` — left half (key+value) padded to fill half the panel,
///   right half (key+value) following. Each key uses `HALF_KEY_COL_WIDTH`.
/// - `GroupTitle` — title string rendered flush-left using `theme.group_title`
///   (bold white). Takes one display row. No border or ASCII decoration.
/// - `Separator` — blank line.
///
/// Description rows (empty-key `KeyValue`) are handled separately by
/// `expand_row_lines` which may return multiple wrapped lines. This function
/// will receive them only when they have a non-empty key.
fn build_row_line<'a>(
    row: &'a DataRow,
    theme: &'a Theme,
    widths: TableWidths,
) -> Line<'a> {
    match row {
        DataRow::KeyValue {
            key,
            value,
            style_hint,
            highlight_key,
        } => {
            build_key_value_line(key, value, *style_hint, *highlight_key, theme)
        }

        DataRow::TwoColumn {
            left_key,
            left_value,
            left_hint,
            right_key,
            right_value,
            right_hint,
        } => {
            // Left half: key (half width) + value padded to fill the left side.
            // Right half: key (half width) + value.
            //
            // The total width of each half is KEY_COL_WIDTH (left key + left
            // value) and KEY_COL_WIDTH (right key + right value). We pad the
            // left value so the right column starts consistently.
            let left_key_str = pad_key(left_key, HALF_KEY_COL_WIDTH);
            let left_val_color = style_hint_color(*left_hint);
            let right_key_str = pad_key(right_key, HALF_KEY_COL_WIDTH);
            let right_val_color = style_hint_color(*right_hint);

            // Pad the left value to fill the remaining left-column budget so
            // the right column is aligned. Total left-column chars = KEY_COL_WIDTH.
            let left_budget = KEY_COL_WIDTH.saturating_sub(HALF_KEY_COL_WIDTH);
            let left_val_padded = pad_value(left_value, left_budget);

            Line::from(vec![
                Span::raw(" "),
                Span::styled(left_key_str, theme.data_key),
                Span::styled(
                    left_val_padded,
                    theme.data_value.fg(left_val_color),
                ),
                Span::styled(right_key_str, theme.data_key),
                Span::styled(
                    right_value.clone(),
                    theme.data_value.fg(right_val_color),
                ),
            ])
        }

        DataRow::GroupTitle(title) => {
            // Flush-left, single styled span. No padding — the group title
            // fills the full line width naturally. Indentation of rows under
            // this title is the caller's responsibility (see DataRow docs).
            Line::from(vec![
                Span::raw(" "),
                Span::styled(title.clone(), theme.group_title),
            ])
        }

        DataRow::Separator => Line::from(""),

        DataRow::TableRow {
            col1,
            col2,
            col3,
            style_hint,
        } => {
            // col1: evidence type — clipped/padded to dynamic width.
            let col1_str = clip_pad(col1, widths.col1);
            // col2: source path — clipped/padded to dynamic width.
            let col2_str = clip_pad(col2, widths.col2);
            // col3: verification outcome — rendered with the style hint color;
            // remainder of the line so no fixed width needed.
            let col3_color = style_hint_color(*style_hint);
            Line::from(vec![
                Span::raw(" "),
                Span::styled(col1_str, theme.data_key),
                Span::raw(TABLE_COL_GAP),
                Span::styled(col2_str, theme.data_value),
                Span::raw(TABLE_COL_GAP),
                Span::styled(col3.clone(), theme.data_value.fg(col3_color)),
            ])
        }

        DataRow::TableHeader {
            col1,
            col2,
            col3,
        } => {
            // All three columns rendered with the bold key style to signal
            // that this is a header, not a data row. One-space left padding
            // matches GroupTitle so the header aligns with group labels above it.
            let col1_str = clip_pad(col1, widths.col1);
            let col2_str = clip_pad(col2, widths.col2);
            Line::from(vec![
                Span::raw(" "),
                Span::styled(col1_str, theme.data_key),
                Span::raw(TABLE_COL_GAP),
                Span::styled(col2_str, theme.data_key),
                Span::raw(TABLE_COL_GAP),
                Span::styled(col3.clone(), theme.data_key),
            ])
        }

        // IndicatorRow is handled by expand_row_lines before reaching
        // build_row_line. This arm exists only as a defensive fallback to
        // keep the match exhaustive if the dispatch path ever changes.
        // The recommendation field is intentionally ignored here — the full
        // multi-line rendering (including recommendation) is handled by
        // expand_row_lines. This fallback emits only the key-value line.
        DataRow::IndicatorRow {
            key,
            value,
            style_hint,
            ..
        } => {
            let key_padded = pad_key(key, widths.indicator_key_col);
            let value_color = style_hint_color(*style_hint);
            Line::from(vec![
                Span::raw(" "),
                Span::styled(key_padded, theme.data_key),
                Span::styled(value.clone(), theme.data_value.fg(value_color)),
            ])
        }
    }
}

/// Right-pad a key string to `width` characters (truncates if too long).
fn pad_key(key: &str, width: usize) -> String {
    let char_count = key.chars().count();
    if char_count >= width {
        // Truncate and add separator
        let truncated: String =
            key.chars().take(width.saturating_sub(2)).collect();
        format!("{truncated}: ")
    } else {
        let pad = width.saturating_sub(char_count).saturating_sub(2);
        format!("{key}: {}", " ".repeat(pad))
    }
}

/// Pad (or truncate) a value string to exactly `width` characters so that
/// the right column of a `TwoColumn` row starts at a consistent position.
fn pad_value(value: &str, width: usize) -> String {
    let char_count = value.chars().count();
    if char_count >= width {
        value.chars().take(width).collect()
    } else {
        let pad = width.saturating_sub(char_count);
        format!("{value}{}", " ".repeat(pad))
    }
}

/// Clip a string to `width` characters and right-pad to exactly `width`.
///
/// Used for `TableRow` and `TableHeader` columns where overflow would push
/// subsequent columns out of alignment. The string is clipped to `width`
/// characters (not bytes) and then space-padded to exactly `width` chars.
///
/// A string already shorter than `width` is padded; a string that equals
/// or exceeds `width` is truncated to exactly `width` characters. This
/// guarantees a fixed-width field for column alignment without overflow.
fn clip_pad(value: &str, width: usize) -> String {
    let char_count = value.chars().count();
    if char_count >= width {
        value.chars().take(width).collect()
    } else {
        let pad = width.saturating_sub(char_count);
        format!("{value}{}", " ".repeat(pad))
    }
}
