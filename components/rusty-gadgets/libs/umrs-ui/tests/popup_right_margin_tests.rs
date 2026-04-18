// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Regression tests for the popup right-margin invariant.
//!
//! Invariant: the rightmost content column of every row inside a popup must
//! be blank (space or empty). The border is drawn in the column AFTER the
//! content area, so "rightmost content column" means
//! `popup_rect.x + popup_rect.width - 2` — one cell to the left of the
//! right border character.
//!
//! These tests deliberately use long values (wider than the popup) for every
//! supported `DataRow` variant so that, if a new variant — or a future change
//! to an existing variant — regresses the margin, the test fails.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;

use umrs_ui::app::DataRow;
use umrs_ui::popup::{PopupCardData, render_audit_card_popup};
use umrs_ui::theme::Theme;

struct FixtureApp {
    tabs: Vec<&'static str>,
    rows: Vec<DataRow>,
}

impl PopupCardData for FixtureApp {
    fn tab_names(&self) -> &[&'static str] {
        &self.tabs
    }
    fn rows_for_tab(&self, _tab: usize) -> &[DataRow] {
        &self.rows
    }
}

/// Locate the popup border and return the inclusive x-range of the inner
/// content area (border_left + 1 .. border_right).
fn find_popup_inner_x_range(
    term: &Terminal<TestBackend>,
    width: u16,
    height: u16,
) -> Option<(u16, u16, u16, u16)> {
    let buf = term.backend().buffer();
    let mut top_y: Option<u16> = None;
    let mut bottom_y: Option<u16> = None;
    let mut left_x: Option<u16> = None;
    let mut right_x: Option<u16> = None;

    for y in 0..height {
        for x in 0..width {
            let sym = buf[(x, y)].symbol();
            // Match any of the box-drawing characters used by the popup border.
            if sym == "╔" || sym == "╗" || sym == "╚" || sym == "╝" {
                if top_y.is_none() || y < top_y? {
                    top_y = Some(y);
                }
                if bottom_y.is_none() || y > bottom_y? {
                    bottom_y = Some(y);
                }
                if left_x.is_none() || x < left_x? {
                    left_x = Some(x);
                }
                if right_x.is_none() || x > right_x? {
                    right_x = Some(x);
                }
            }
        }
    }

    Some((left_x?, right_x?, top_y?, bottom_y?))
}

/// Assert the right-margin invariant for a rendered popup: for every row
/// strictly between the top and bottom borders, the cell at column
/// `right_border_x - 1` (the rightmost content cell inside the popup) must
/// be a space.
fn assert_right_margin(term: &Terminal<TestBackend>, width: u16, height: u16, label: &str) {
    let (left, right, top, bottom) =
        find_popup_inner_x_range(term, width, height).expect("popup border not found");

    let margin_x = right - 1;
    let buf = term.backend().buffer();

    // Skip the row just above the hint line (which is the hint itself),
    // but scan every other interior row including tab bar and content.
    for y in (top + 1)..bottom {
        let margin_sym = buf[(margin_x, y)].symbol();
        assert!(
            margin_sym == " " || margin_sym.is_empty(),
            "[{label}] right-margin invariant violated at y={y}: \
             cell at x={margin_x} is {margin_sym:?}, expected blank. \
             Popup left={left} right={right} top={top} bottom={bottom}."
        );
    }
}

#[test]
fn key_value_and_empty_key_rows_respect_right_margin() {
    let data = FixtureApp {
        tabs: vec!["Identity"],
        rows: vec![
            DataRow::group_title("Computed Hashes"),
            DataRow::normal(" SHA-256", ""),
            DataRow::normal("", format!("  {} ", "a".repeat(64))),
            DataRow::normal(" SHA-384 half 1", ""),
            DataRow::normal("", format!("  {} ", "b".repeat(48))),
            DataRow::normal(" SHA-384 half 2", ""),
            DataRow::normal("", format!("  {} ", "c".repeat(48))),
            DataRow::normal(
                "Path",
                "/some/incredibly/long/path/that/will/be/truncated/at/popup/width/boundary/with/ellipsis/marker",
            ),
            DataRow::normal("Filename", "ordinary.txt"),
        ],
    };

    let theme = Theme::dark();
    for &width in &[80u16, 100u16, 120u16, 140u16, 160u16] {
        let height: u16 = 30;
        let backend = TestBackend::new(width, height);
        let mut term = Terminal::new(backend).expect("terminal init");
        term.draw(|f| {
            render_audit_card_popup(f, Rect::new(0, 0, width, height), &data, 0, 0, &theme);
        })
        .expect("draw");
        assert_right_margin(&term, width, height, &format!("width={width}"));
    }
}

#[test]
fn indicator_and_table_rows_respect_right_margin() {
    let long_rec: &'static str = "apply the hardening knob exactly as specified by the reference policy for this class of system";

    let data = FixtureApp {
        tabs: vec!["Observations"],
        rows: vec![
            DataRow::indicator_row_recommended(
                "very_long_indicator_name",
                "off",
                "description that will be wrapped across several lines because the popup is narrower than the description text and every wrapped line must respect the right-margin invariant just like every other row",
                Some(long_rec),
                umrs_ui::app::StyleHint::TrustRed,
            ),
            DataRow::table_header("Evidence Type", "Source", "Outcome"),
            DataRow::table_row(
                "Kernel attributes (/sys)",
                "/sys/kernel/very/long/source/path/that/would/overrun",
                "verified",
                umrs_ui::app::StyleHint::TrustGreen,
            ),
        ],
    };

    let theme = Theme::dark();
    for &width in &[80u16, 100u16, 120u16, 140u16, 160u16] {
        let height: u16 = 35;
        let backend = TestBackend::new(width, height);
        let mut term = Terminal::new(backend).expect("terminal init");
        term.draw(|f| {
            render_audit_card_popup(f, Rect::new(0, 0, width, height), &data, 0, 0, &theme);
        })
        .expect("draw");
        assert_right_margin(&term, width, height, &format!("indicator width={width}"));
    }
}

#[test]
fn two_column_rows_respect_right_margin() {
    let data = FixtureApp {
        tabs: vec!["Security"],
        rows: vec![DataRow::two_column(
            "LeftKey",
            "left_value_that_is_moderately_long",
            umrs_ui::app::StyleHint::Normal,
            "RightKey",
            "right_value_that_is_also_moderately_long_to_force_overrun",
            umrs_ui::app::StyleHint::Normal,
        )],
    };

    let theme = Theme::dark();
    for &width in &[80u16, 100u16, 120u16] {
        let height: u16 = 25;
        let backend = TestBackend::new(width, height);
        let mut term = Terminal::new(backend).expect("terminal init");
        term.draw(|f| {
            render_audit_card_popup(f, Rect::new(0, 0, width, height), &data, 0, 0, &theme);
        })
        .expect("draw");
        assert_right_margin(&term, width, height, &format!("two-col width={width}"));
    }
}
