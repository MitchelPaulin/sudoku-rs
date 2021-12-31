use std::fmt::format;
use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen, style::Bold};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Row, Table},
    Terminal,
};

const ROWS: usize = 9;
const COLS: usize = 9;

pub struct UI {
    puzzle: [[Option<u8>; ROWS]; COLS],
}

impl UI {
    pub fn new() -> UI {
        UI {
            puzzle: [[None; ROWS]; COLS],
        }
    }

    pub fn run(&mut self) {
        // Terminal initialization
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        // Setup event handlers - TODO
        //let events = Events::new();
        loop {
            let mut i = 0;
            terminal
                .draw(|frame| {
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

                    // draw the sudoku table
                    let rect = Rect {
                        x: frame.size().x + 2,
                        y: frame.size().y + 2,
                        width: 54,
                        height: 27,
                    };
                    let large_table_cells = split_rect_into_three_by_three_square(rect);
                    for square in large_table_cells {
                        let cells = split_rect_into_three_by_three_square(square);
                        for cell in cells {
                            let (bg_color, text_color) = match i % 2 {
                                0 => (Color::White, Color::Black),
                                _ => (Color::Gray, Color::Black),
                            };

                            let block = Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().bg(bg_color).fg(bg_color));
                            let text = Paragraph::new(format!(" {}  ", i))
                                .alignment(Alignment::Center)
                                .style(Style::default().bg(bg_color).fg(text_color).add_modifier(Modifier::BOLD));
                            let text_rect = Rect {
                                x: cell.x + 1,
                                y: cell.y + 1,
                                width: 4,
                                height: 1,
                            };
                            frame.render_widget(block, cell);
                            frame.render_widget(text, text_rect);
                        }
                        i += 1;
                    }
                })
                .unwrap();
        }
    }
}

// Helper function to take a rect and split it equally into a 3x3 rect
fn split_rect_into_three_by_three_square(area: Rect) -> Vec<Rect> {
    let mut rets = vec![];
    let rows = split_rect_into_three(area, Direction::Vertical);
    for row in rows {
        rets.extend(split_rect_into_three(row, Direction::Horizontal));
    }
    rets
}

// Helper function to split a rect into three equally sized Rects
fn split_rect_into_three(area: Rect, dir: Direction) -> Vec<Rect> {
    Layout::default()
        .direction(dir)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(area)
}
