// Ratatui scrollbar example — main.rs
// Source: https://raw.githubusercontent.com/ratatui/ratatui/ratatui-v0.30.0/examples/apps/scrollbar/src/main.rs
// Version: ratatui v0.30.0
// Fetched: 2026-03-15
//
// Tags: Scrollbar, ScrollbarState, ScrollbarOrientation, Paragraph, scroll, Margin,
//       tick-rate, Duration, Instant, Masked, vertical-scroll, horizontal-scroll
//
// Demonstrates: vertical and horizontal scrollbars with different styles (arrows, symbols,
//               thumb-only, mirrored), scroll state management, tick-based event loop,
//               Masked widget for password-style display, multiple layout regions

use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::scrollbar;
use ratatui::text::{Line, Masked, Span};
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{DefaultTerminal, Frame};

#[derive(Default)]
struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.render(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                last_tick = Instant::now();
                continue;
            }
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
                    KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
                    KeyCode::Char('h') | KeyCode::Left => self.scroll_left(),
                    KeyCode::Char('l') | KeyCode::Right => self.scroll_right(),
                    _ => {}
                }
            }
        }
    }

    const fn scroll_down(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    const fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    const fn scroll_left(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
        self.horizontal_scroll_state = self
            .horizontal_scroll_state
            .position(self.horizontal_scroll);
    }

    const fn scroll_right(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
        self.horizontal_scroll_state = self
            .horizontal_scroll_state
            .position(self.horizontal_scroll);
    }

    #[expect(clippy::too_many_lines, clippy::cast_possible_truncation)]
    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Words made "loooong" to demonstrate line breaking.
        let s =
            "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
        let mut long_line = s.repeat(usize::from(area.width) / s.len() + 4);
        long_line.push('\n');

        let chunks = Layout::vertical([
            Constraint::Min(1),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

        let text = vec![
            Line::from("This is a line "),
            Line::from("This is a line   ".red()),
            Line::from("This is a line".on_dark_gray()),
            Line::from("This is a longer line".crossed_out()),
            Line::from(long_line.clone()),
            Line::from("This is a line".reset()),
            Line::from(vec![
                Span::raw("Masked text: "),
                Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),
            ]),
            Line::from("This is a line "),
            Line::from("This is a line   ".red()),
            Line::from("This is a line".on_dark_gray()),
            Line::from("This is a longer line".crossed_out()),
            Line::from(long_line.clone()),
            Line::from("This is a line".reset()),
            Line::from(vec![
                Span::raw("Masked text: "),
                Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),
            ]),
        ];
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(text.len());
        self.horizontal_scroll_state = self.horizontal_scroll_state.content_length(long_line.len());

        let create_block = |title: &'static str| Block::bordered().gray().title(title.bold());

        let title = Block::new()
            .title_alignment(Alignment::Center)
            .title("Use h j k l or ◄ ▲ ▼ ► to scroll ".bold());
        frame.render_widget(title, chunks[0]);

        // Vertical scrollbar with arrows
        let paragraph = Paragraph::new(text.clone())
            .gray()
            .block(create_block("Vertical scrollbar with arrows"))
            .scroll((self.vertical_scroll as u16, 0));
        frame.render_widget(paragraph, chunks[1]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            chunks[1],
            &mut self.vertical_scroll_state,
        );

        // Vertical scrollbar without arrows, mirrored to left
        let paragraph = Paragraph::new(text.clone())
            .gray()
            .block(create_block(
                "Vertical scrollbar without arrows, without track symbol and mirrored",
            ))
            .scroll((self.vertical_scroll as u16, 0));
        frame.render_widget(paragraph, chunks[2]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalLeft)
                .symbols(scrollbar::VERTICAL)
                .begin_symbol(None)
                .track_symbol(None)
                .end_symbol(None),
            chunks[2].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.vertical_scroll_state,
        );

        // Horizontal scrollbar with begin arrow and custom thumb
        let paragraph = Paragraph::new(text.clone())
            .gray()
            .block(create_block(
                "Horizontal scrollbar with only begin arrow & custom thumb symbol",
            ))
            .scroll((0, self.horizontal_scroll as u16));
        frame.render_widget(paragraph, chunks[3]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
                .thumb_symbol("🬋")
                .end_symbol(None),
            chunks[3].inner(Margin {
                vertical: 0,
                horizontal: 1,
            }),
            &mut self.horizontal_scroll_state,
        );

        // Horizontal scrollbar without arrows, custom thumb and track
        let paragraph = Paragraph::new(text.clone())
            .gray()
            .block(create_block(
                "Horizontal scrollbar without arrows & custom thumb and track symbol",
            ))
            .scroll((0, self.horizontal_scroll as u16));
        frame.render_widget(paragraph, chunks[4]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
                .thumb_symbol("░")
                .track_symbol(Some("─")),
            chunks[4].inner(Margin {
                vertical: 0,
                horizontal: 1,
            }),
            &mut self.horizontal_scroll_state,
        );
    }
}
