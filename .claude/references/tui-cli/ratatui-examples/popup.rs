// Ratatui popup example — main.rs
// Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/examples/apps/popup/src/main.rs
// Version: ratatui v0.30.0
// Fetched: 2026-03-15
//
// Tags: popup, overlay, Clear, Flex::Center, centered-area, Layout::vertical,
//       Layout::horizontal, Constraint::Percentage, simple-loop
//
// Demonstrates: popup pattern using Clear + Block, centering with Flex::Center,
//               ratatui::run() convenience function (v0.30+), toggle state with bool flag

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut show_popup = false;

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, show_popup))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => show_popup = !show_popup,
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame, show_popup: bool) {
    let area = frame.area();

    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let [instructions, content] = area.layout(&layout);

    frame.render_widget(
        Line::from("Press 'p' to toggle popup, 'q' to quit").centered(),
        instructions,
    );

    frame.render_widget(Block::bordered().title("Content").on_blue(), content);

    if show_popup {
        let popup = Block::bordered().title("Popup");
        let popup_area = centered_area(area, 60, 20);
        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup, popup_area);
    }
}

/// Helper: create a centered rectangle using the given percentages of the full area.
/// Uses Flex::Center on both axes to position a single constraint in the middle.
fn centered_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = area.layout(&vertical);
    let [area] = area.layout(&horizontal);
    area
}
