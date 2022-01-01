use crate::events::{Event, Events, TICK_RATE_MS};
use crate::puzzle::{Difficulty, Puzzle, SudokuPuzzle, EMPTY_SPACE};
use crate::themes;

use std::io::{self, Stdout};
use termion::{
    event::Key,
    input::MouseTerminal,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

const BOARD_LENGTH: usize = 9;
const PUZZLE_WIDTH: u16 = 54;
const PUZZLE_HEIGHT: u16 = 27;

const CONTROLS: &str =
    "Select cell: hjkl | ← ↓ ↑ →\nErase cell: space\nStart new puzzle(Easy, Hard): z, x\nGive up: g\nQuit: q";

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
        if self.x != BOARD_LENGTH - 1 {
            self.x += 1;
        }
    }

    pub fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.y != BOARD_LENGTH - 1 {
            self.y += 1;
        }
    }

    pub fn as_board_cords(&self) -> usize {
        self.x + BOARD_LENGTH * self.y
    }
}

const BOARD_THEME: themes::Theme = themes::TRANQUIL;

type SudokuFrame<'a> =
    Frame<'a, TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;

pub struct UI {
    puzzle: Puzzle,
    displayed_puzzle: SudokuPuzzle,
    highlighted_cell: Point,
    cell_counts: [u8; BOARD_LENGTH],
    time_in_ms: u64,
}

impl UI {
    pub fn new() -> UI {
        let new_puzzle = Puzzle::new_puzzle(Difficulty::Easy);
        let displayed_puzzle = new_puzzle.puzzle;
        UI {
            puzzle: new_puzzle,
            displayed_puzzle,
            highlighted_cell: Point { x: 0, y: 0 },
            cell_counts: [0; BOARD_LENGTH],
            time_in_ms: 0,
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
            terminal
                .draw(|frame| {
                    if draw_puzzle_window(frame, self) {
                        draw_info_window(frame, self);
                        draw_controls_window(frame);
                    }
                })
                .unwrap();

            match events.next().unwrap() {
                Event::Input(key) => {
                    match key {
                        // movement using arrow keys or vim movement keys
                        Key::Up | Key::Char('k') => self.highlighted_cell.up(),
                        Key::Down | Key::Char('j') => self.highlighted_cell.down(),
                        Key::Left | Key::Char('h') => self.highlighted_cell.left(),
                        Key::Right | Key::Char('l') => self.highlighted_cell.right(),
                        Key::Char('1') => self.update_displayed_board('1'),
                        Key::Char('2') => self.update_displayed_board('2'),
                        Key::Char('3') => self.update_displayed_board('3'),
                        Key::Char('4') => self.update_displayed_board('4'),
                        Key::Char('5') => self.update_displayed_board('5'),
                        Key::Char('6') => self.update_displayed_board('6'),
                        Key::Char('7') => self.update_displayed_board('7'),
                        Key::Char('8') => self.update_displayed_board('8'),
                        Key::Char('9') => self.update_displayed_board('9'),
                        Key::Char(' ') => self.update_displayed_board(EMPTY_SPACE),
                        Key::Char('z') => self.new_game(Difficulty::Easy),
                        Key::Char('x') => self.new_game(Difficulty::Hard),
                        Key::Char('q') | Key::Ctrl('c') => break,
                        _ => {}
                    }
                }
                Event::Tick => self.time_in_ms += TICK_RATE_MS,
            }
        }
    }

    fn new_game(&mut self, difficulty: Difficulty) {
        self.time_in_ms = 0;
        self.puzzle = Puzzle::new_puzzle(difficulty);
        self.displayed_puzzle = self.puzzle.puzzle;

        // cell counts will be updated automatically on the next frame render
    }

    fn update_displayed_board(&mut self, val: char) {
        if self.puzzle.puzzle[self.highlighted_cell.as_board_cords()] == EMPTY_SPACE {
            self.displayed_puzzle[self.highlighted_cell.as_board_cords()] = val;
        }
    }
}

/*
    Draw the puzzle window, return if the window could be drawn
*/
fn draw_puzzle_window(frame: &mut SudokuFrame, ui: &mut UI) -> bool {
    let terminal_rect = frame.size();
    ui.cell_counts = [0; BOARD_LENGTH];

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "sudoku-rs",
            Style::default()
                .fg(BOARD_THEME.title_color)
                .add_modifier(Modifier::BOLD),
        ))
        .border_type(BorderType::Rounded);
    frame.render_widget(outer_block, terminal_rect);

    // the window is too small to even show the warning
    if terminal_rect.height < 2 || terminal_rect.width < 25 {
        return false;
    }

    // if the window is too small, show warning
    if terminal_rect.height < PUZZLE_HEIGHT + 2 || terminal_rect.width < PUZZLE_WIDTH + 2 {
        let text = Paragraph::new("Window is too small\nPlease expand window")
            .alignment(Alignment::Center);
        frame.render_widget(
            text,
            Rect {
                y: 1,
                x: 3,
                width: 20,
                height: 2,
            },
        );
        return false;
    }

    // draw the sudoku table
    let rect = Rect {
        x: (frame.size().width - PUZZLE_WIDTH) / 2,
        y: frame.size().y + 2,
        width: PUZZLE_WIDTH,
        height: PUZZLE_HEIGHT,
    };
    let large_table_cells = split_rect_into_three_by_three_square(rect);
    for (current_square, square) in large_table_cells.into_iter().enumerate() {
        let cells = split_rect_into_three_by_three_square(square);
        for (square_cell_counter, cell) in cells.into_iter().enumerate() {
            let point_cords = square_to_point_cords(current_square, square_cell_counter);

            // update cell counts
            if let Some(num) = ui.displayed_puzzle[point_cords.as_board_cords()].to_digit(10) {
                ui.cell_counts[num as usize - 1] += 1
            }

            let (mut bg_color, text_color, locked_square_color) = match current_square % 2 {
                0 => (
                    BOARD_THEME.light_square_color,
                    BOARD_THEME.dark_number_color,
                    BOARD_THEME.dark_square_color,
                ),
                _ => (
                    BOARD_THEME.dark_square_color,
                    BOARD_THEME.light_number_color,
                    BOARD_THEME.light_square_color,
                ),
            };

            if point_cords == ui.highlighted_cell {
                bg_color = BOARD_THEME.highlighted_color;
            } else if cell_error(&point_cords, current_square, ui) {
                bg_color = BOARD_THEME.error_color;
            }

            let char = ui.displayed_puzzle[point_cords.as_board_cords()];
            let fg_color = if ui.puzzle.puzzle[point_cords.as_board_cords()] != EMPTY_SPACE {
                locked_square_color
            } else {
                bg_color
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().bg(bg_color).fg(fg_color));

            let text = Paragraph::new(format!(
                " {}  ",
                if char == EMPTY_SPACE { ' ' } else { char }
            ))
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
        }
    }

    true
}

fn draw_info_window(frame: &mut SudokuFrame, ui: &UI) {
    // don't render frame if there isn't enough room
    if frame.size().height <= PUZZLE_HEIGHT + 4 {
        return;
    }

    // draw the score window
    let score_window = Rect {
        x: (frame.size().width - PUZZLE_WIDTH) / 2,
        y: PUZZLE_HEIGHT + 2,
        width: PUZZLE_WIDTH,
        height: 4,
    };

    let score_block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Info",
        Style::default()
            .fg(BOARD_THEME.title_color)
            .add_modifier(Modifier::BOLD),
    ));

    let mut info_str = "".to_string();
    for (i, val) in ui.cell_counts.iter().enumerate() {
        info_str = format!("{} {}:{}", info_str, i + 1, val);
    }

    info_str = format!(
        "{}\nDifficulty: {}               Time: {}s",
        info_str,
        ui.puzzle.difficulty,
        ui.time_in_ms / 1000
    );

    let text = Paragraph::new(info_str).alignment(Alignment::Center);
    frame.render_widget(
        text,
        Rect {
            y: score_window.y + 1,
            ..score_window
        },
    );

    frame.render_widget(score_block, score_window);
}

fn draw_controls_window(frame: &mut SudokuFrame) {
    // don't render frame if there isn't enough room
    if frame.size().height <= PUZZLE_HEIGHT + 12 {
        return;
    }

    //draw the controls window
    let controls_rect = Rect {
        x: (frame.size().width - PUZZLE_WIDTH) / 2,
        y: PUZZLE_HEIGHT + 5,
        width: PUZZLE_WIDTH,
        height: 7,
    };

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Controls",
        Style::default()
            .fg(BOARD_THEME.title_color)
            .add_modifier(Modifier::BOLD),
    ));

    let text = Paragraph::new(CONTROLS).alignment(Alignment::Center);
    frame.render_widget(block, controls_rect);
    frame.render_widget(
        text,
        Rect {
            y: controls_rect.y + 1,
            ..controls_rect
        },
    );
}

/*
    Determine if the given cell should display as an error
*/
fn cell_error(point_cords: &Point, current_square: usize, ui: &UI) -> bool {
    if ui.displayed_puzzle[point_cords.as_board_cords()] == EMPTY_SPACE {
        return false;
    }
    let mut counter = 0;
    // check for any duplicates in the same square
    let row = current_square - (current_square % BOARD_LENGTH);
    for i in row..row + BOARD_LENGTH {
        let current_cord = square_to_point_cords(current_square, i);
        if ui.displayed_puzzle[point_cords.as_board_cords()]
            == ui.displayed_puzzle[current_cord.as_board_cords()]
        {
            counter += 1;
        }
    }

    let mut duplicate_found = counter > 1;

    counter = 0;
    //check for any duplicates in the same col
    for i in 0..BOARD_LENGTH {
        let p = Point {
            x: i,
            y: point_cords.y,
        };
        if ui.displayed_puzzle[p.as_board_cords()]
            == ui.displayed_puzzle[point_cords.as_board_cords()]
        {
            counter += 1;
        }
    }

    duplicate_found |= counter > 1;

    counter = 0;
    //check for any duplicates in the same row
    for i in 0..BOARD_LENGTH {
        let p = Point {
            x: point_cords.x,
            y: i,
        };
        if ui.displayed_puzzle[p.as_board_cords()]
            == ui.displayed_puzzle[point_cords.as_board_cords()]
        {
            counter += 1;
        }
    }

    duplicate_found |= counter > 1;

    return duplicate_found;
}

/*
    Converts the puzzles strange coordinate system into more familiar / easier to work with x and y cords
*/
fn square_to_point_cords(square_number: usize, cell_number: usize) -> Point {
    let col = (square_number % 3) * 3 + cell_number % 3;
    let row = (square_number / 3) * 3 + cell_number / 3;

    Point { x: col, y: row }
}

/*
    Helper function to take a rect and split it equally into a 3x3 rect
*/
fn split_rect_into_three_by_three_square(area: Rect) -> Vec<Rect> {
    let mut rets = vec![];
    let rows = split_rect_into_three(area, Direction::Vertical);
    for row in rows {
        rets.extend(split_rect_into_three(row, Direction::Horizontal));
    }
    rets
}

/*
    Helper function to split a rect into three equally sized Rects
*/
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
