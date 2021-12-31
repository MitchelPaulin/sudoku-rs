use crate::events::{Event, Events};

use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

const ROWS: usize = 9;
const COLS: usize = 9;
const PUZZLE_WIDTH: u16 = 54;
const PUZZLE_HEIGHT: u16 = 27;

#[derive(PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.x != COLS - 1 {
            self.x += 1;
        }
    }

    pub fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.y != ROWS - 1 {
            self.y += 1;
        }
    }
}

pub struct UI {
    puzzle: [[Option<u8>; ROWS]; COLS],
    highlighted_cell: Point,
}

impl UI {
    pub fn new() -> UI {
        UI {
            puzzle: [[None; ROWS]; COLS],
            highlighted_cell: Point { x: 0, y: 0 },
        }
    }

    pub fn run(&mut self) {
        // Terminal initialization
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        //Setup event handlers
        let events = Events::new();

        loop {
            let mut current_square = 0;
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
                        x: (frame.size().width - PUZZLE_WIDTH) / 2,
                        y: frame.size().y + 2,
                        width: PUZZLE_WIDTH,
                        height: PUZZLE_HEIGHT,
                    };
                    let large_table_cells = split_rect_into_three_by_three_square(rect);
                    for square in large_table_cells {
                        let mut square_cell_counter = 0;
                        let cells = split_rect_into_three_by_three_square(square);
                        for cell in cells {
                            let (mut bg_color, text_color) = match current_square % 2 {
                                0 => (Color::White, Color::Black),
                                _ => (Color::Gray, Color::Black),
                            };

                            if square_to_point_cords(current_square, square_cell_counter)
                                == self.highlighted_cell
                            {
                                bg_color = Color::Red;
                            }

                            let block = Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().bg(bg_color).fg(bg_color));
                            let text = Paragraph::new(format!(" {}  ", square_cell_counter))
                                .alignment(Alignment::Center)
                                .style(
                                    Style::default()
                                        .bg(bg_color)
                                        .fg(text_color)
                                        .add_modifier(Modifier::BOLD),
                                );
                            let text_rect = Rect {
                                x: cell.x + 1,
                                y: cell.y + 1,
                                width: 4,
                                height: 1,
                            };
                            frame.render_widget(block, cell);
                            frame.render_widget(text, text_rect);
                            square_cell_counter += 1;
                        }
                        current_square += 1;
                    }
                })
                .unwrap();

            if let Event::Input(key) = events.next().unwrap() {
                match key {
                    // movement using arrow keys or vim movement keys
                    Key::Up | Key::Char('k') => self.highlighted_cell.up(),
                    Key::Down | Key::Char('j') => self.highlighted_cell.down(),
                    Key::Left | Key::Char('h') => self.highlighted_cell.left(),
                    Key::Right | Key::Char('l') => self.highlighted_cell.right(),
                    Key::Char('q') => break,
                    _ => {}
                }
            }
        }
    }
}

/*
    Converts the puzzles strange coordinate system into more familiar x and y coords
*/
fn square_to_point_cords(square_number: usize, cell_number: usize) -> Point {
    let col = (square_number % 3) * 3 + cell_number % 3;
    let row = (square_number / 3) * 3 + cell_number / 3;

    Point { x: col, y: row }
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
