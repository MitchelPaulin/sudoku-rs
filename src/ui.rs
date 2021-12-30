use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Row},
    Terminal,
};

const ROWS: usize = 9;
const COLS: usize = 9;

pub struct UI {
    puzzle: [[Option<u8>; ROWS]; COLS]
}

impl UI {

    pub fn new() -> UI {
        UI {
            puzzle: [[None; ROWS]; COLS]
        }
    }

    pub fn run(&mut self) {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        loop {
            terminal.draw(|frame| {
                let terminal_rect = frame.size();

                let outer_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Sudoku",
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ))
                .border_type(BorderType::Rounded);
                frame.render_widget(outer_block, terminal_rect);

            }).unwrap();
        }

    }
}