use rand::Rng;
use std::fmt::{self};

use crate::puzzle_transformer::transform_puzzle;

pub const EMPTY_SPACE: char = '_';

pub type SudokuPuzzle = [char; 81];

#[derive(PartialEq)]
pub enum Difficulty {
    Beginner,
    Easy,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

pub struct Puzzle {
    pub puzzle: SudokuPuzzle,
    pub solution: SudokuPuzzle,
    pub difficulty: Difficulty,
}

impl Puzzle {
    pub fn new_puzzle(difficulty: Difficulty) -> Puzzle {
        let mut puzzle;

        if difficulty == Difficulty::Easy {
            let index = rand::thread_rng().gen_range(0..EASY_PUZZLES);
            puzzle = Puzzle {
                puzzle: PUZZLES_EASY[index].0,
                solution: PUZZLES_EASY[index].1,
                difficulty,
            };
        } else if difficulty == Difficulty::Hard {
            let index = rand::thread_rng().gen_range(0..HARD_PUZZLES);
            puzzle = Puzzle {
                puzzle: PUZZLES_HARD[index].0,
                solution: PUZZLES_HARD[index].1,
                difficulty,
            }
        } else {
            let index = rand::thread_rng().gen_range(0..BEGINNER_PUZZLES);
            puzzle = Puzzle {
                puzzle: PUZZLES_BEGINNER[index].0,
                solution: PUZZLES_BEGINNER[index].1,
                difficulty,
            }
        }
        transform_puzzle((&mut puzzle.puzzle, &mut puzzle.solution));
        puzzle
    }
}

const BEGINNER_PUZZLES: usize = 10;
const PUZZLES_BEGINNER: [(SudokuPuzzle, SudokuPuzzle); EASY_PUZZLES] = [
    (
        [
            '_', '_', '2', '7', '_', '4', '_', '_', '6', '_', '_', '_', '_', '_', '_', '5', '_',
            '_', '_', '_', '1', '_', '6', '_', '_', '_', '3', '5', '_', '_', '_', '_', '_', '9',
            '_', '_', '_', '4', '_', '_', '_', '_', '_', '1', '5', '_', '1', '3', '_', '_', '_',
            '6', '_', '_', '3', '7', '4', '_', '_', '_', '2', '_', '_', '_', '_', '6', '_', '2',
            '5', '4', '9', '_', '_', '_', '_', '_', '_', '_', '_', '_', '1',
        ],
        [
            '9', '3', '2', '7', '5', '4', '1', '8', '6', '7', '6', '8', '1', '3', '2', '5', '4',
            '9', '4', '5', '1', '9', '6', '8', '7', '2', '3', '5', '2', '7', '8', '1', '6', '9',
            '3', '4', '6', '4', '9', '2', '7', '3', '8', '1', '5', '8', '1', '3', '5', '4', '9',
            '6', '7', '2', '3', '7', '4', '6', '9', '1', '2', '5', '8', '1', '8', '6', '3', '2',
            '5', '4', '9', '7', '2', '9', '5', '4', '8', '7', '3', '6', '1',
        ],
    ),
    (
        [
            '_', '_', '_', '_', '_', '4', '7', '1', '5', '_', '_', '_', '3', '_', '_', '_', '_',
            '2', '6', '_', '_', '_', '_', '7', '9', '_', '_', '_', '_', '_', '7', '_', '_', '2',
            '_', '_', '_', '7', '_', '_', '2', '6', '4', '_', '3', '2', '5', '_', '_', '_', '3',
            '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '4', '5', '9', '_', '_', '_',
            '2', '3', '_', '_', '_', '_', '_', '5', '1', '_', '_', '_', '_',
        ],
        [
            '9', '3', '2', '8', '6', '4', '7', '1', '5', '7', '8', '5', '3', '9', '1', '6', '4',
            '2', '6', '4', '1', '2', '5', '7', '9', '3', '8', '3', '1', '6', '7', '4', '5', '2',
            '8', '9', '8', '7', '9', '1', '2', '6', '4', '5', '3', '2', '5', '4', '9', '8', '3',
            '1', '7', '6', '1', '2', '7', '6', '3', '8', '5', '9', '4', '5', '9', '8', '4', '7',
            '2', '3', '6', '1', '4', '6', '3', '5', '1', '9', '8', '2', '7',
        ],
    ),
    (
        [
            '_', '_', '_', '2', '1', '_', '_', '_', '_', '_', '_', '_', '_', '_', '6', '_', '4',
            '_', '_', '_', '3', '_', '_', '4', '6', '9', '7', '_', '_', '_', '7', '_', '_', '_',
            '_', '6', '_', '_', '_', '_', '_', '_', '2', '_', '4', '_', '5', '_', '_', '_', '1',
            '_', '_', '_', '_', '_', '_', '_', '5', '_', '_', '_', '3', '_', '9', '_', '_', '6',
            '_', '7', '_', '_', '2', '_', '_', '3', '_', '_', '4', '_', '9',
        ],
        [
            '9', '6', '4', '2', '1', '7', '3', '8', '5', '5', '8', '7', '9', '3', '6', '1', '4',
            '2', '1', '2', '3', '5', '8', '4', '6', '9', '7', '8', '4', '9', '7', '2', '3', '5',
            '1', '6', '6', '3', '1', '8', '9', '5', '2', '7', '4', '7', '5', '2', '6', '4', '1',
            '9', '3', '8', '4', '7', '6', '1', '5', '9', '8', '2', '3', '3', '9', '8', '4', '6',
            '2', '7', '5', '1', '2', '1', '5', '3', '7', '8', '4', '6', '9',
        ],
    ),
    (
        [
            '_', '2', '_', '6', '_', '_', '8', '4', '_', '_', '4', '6', '_', '_', '_', '_', '7',
            '_', '_', '_', '8', '7', '_', '3', '_', '_', '_', '_', '8', '_', '_', '_', '_', '_',
            '_', '1', '_', '_', '_', '3', '2', '_', '7', '_', '_', '1', '_', '_', '4', '_', '5',
            '_', '_', '_', '_', '_', '_', '_', '6', '_', '_', '5', '_', '_', '1', '_', '_', '_',
            '_', '4', '_', '_', '_', '5', '_', '8', '_', '_', '_', '_', '2',
        ],
        [
            '7', '2', '1', '6', '5', '9', '8', '4', '3', '3', '4', '6', '2', '1', '8', '9', '7',
            '5', '5', '9', '8', '7', '4', '3', '2', '1', '6', '4', '8', '2', '9', '7', '6', '5',
            '3', '1', '9', '6', '5', '3', '2', '1', '7', '8', '4', '1', '3', '7', '4', '8', '5',
            '6', '2', '9', '2', '7', '9', '1', '6', '4', '3', '5', '8', '8', '1', '3', '5', '9',
            '2', '4', '6', '7', '6', '5', '4', '8', '3', '7', '1', '9', '2',
        ],
    ),
    (
        [
            '_', '1', '_', '_', '_', '5', '_', '8', '2', '_', '_', '_', '_', '_', '3', '_', '7',
            '_', '3', '8', '_', '_', '7', '_', '6', '_', '_', '_', '_', '_', '_', '_', '4', '_',
            '6', '_', '2', '_', '_', '_', '_', '_', '_', '5', '7', '_', '_', '_', '_', '8', '2',
            '_', '_', '_', '_', '4', '_', '5', '_', '_', '_', '2', '3', '8', '_', '_', '6', '_',
            '_', '_', '4', '_', '_', '_', '_', '_', '_', '_', '1', '_', '_',
        ],
        [
            '9', '1', '7', '4', '6', '5', '3', '8', '2', '4', '2', '6', '8', '1', '3', '5', '7',
            '9', '3', '8', '5', '2', '7', '9', '6', '1', '4', '1', '7', '3', '9', '5', '4', '2',
            '6', '8', '2', '9', '8', '1', '3', '6', '4', '5', '7', '5', '6', '4', '7', '8', '2',
            '9', '3', '1', '6', '4', '1', '5', '9', '7', '8', '2', '3', '8', '3', '9', '6', '2',
            '1', '7', '4', '5', '7', '5', '2', '3', '4', '8', '1', '9', '6',
        ],
    ),
    (
        [
            '2', '6', '_', '3', '_', '_', '7', '_', '8', '_', '_', '_', '_', '_', '4', '_', '_',
            '2', '_', '_', '3', '_', '8', '_', '4', '_', '5', '8', '_', '_', '_', '_', '7', '_',
            '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '5', '_', '_', '1', '_', '8',
            '_', '_', '_', '7', '_', '_', '_', '_', '_', '_', '2', '_', '_', '_', '2', '_', '_',
            '3', '_', '_', '6', '_', '_', '8', '7', '5', '_', '_', '1', '_',
        ],
        [
            '2', '6', '4', '3', '1', '5', '7', '9', '8', '9', '8', '5', '6', '7', '4', '1', '3',
            '2', '1', '7', '3', '2', '8', '9', '4', '6', '5', '8', '4', '6', '9', '2', '7', '3',
            '5', '1', '3', '9', '1', '5', '4', '6', '2', '8', '7', '5', '2', '7', '1', '3', '8',
            '6', '4', '9', '7', '5', '9', '4', '6', '1', '8', '2', '3', '4', '1', '2', '8', '9',
            '3', '5', '7', '6', '6', '3', '8', '7', '5', '2', '9', '1', '4',
        ],
    ),
    (
        [
            '_', '_', '_', '5', '_', '_', '_', '_', '_', '_', '9', '_', '3', '2', '1', '7', '_',
            '_', '_', '_', '_', '_', '_', '_', '1', '5', '_', '_', '5', '3', '_', '_', '_', '8',
            '9', '_', '6', '_', '_', '2', '9', '_', '3', '_', '_', '_', '1', '_', '_', '_', '_',
            '_', '6', '_', '_', '7', '_', '_', '6', '_', '9', '1', '2', '_', '_', '_', '8', '_',
            '_', '_', '7', '_', '_', '_', '_', '_', '_', '_', '6', '_', '_',
        ],
        [
            '7', '6', '1', '5', '8', '9', '4', '2', '3', '5', '9', '4', '3', '2', '1', '7', '8',
            '6', '3', '2', '8', '6', '7', '4', '1', '5', '9', '2', '5', '3', '1', '4', '6', '8',
            '9', '7', '6', '8', '7', '2', '9', '5', '3', '4', '1', '4', '1', '9', '7', '3', '8',
            '2', '6', '5', '8', '7', '5', '4', '6', '3', '9', '1', '2', '9', '3', '6', '8', '1',
            '2', '5', '7', '4', '1', '4', '2', '9', '5', '7', '6', '3', '8',
        ],
    ),
    (
        [
            '_', '_', '_', '_', '7', '_', '2', '_', '_', '9', '_', '4', '_', '5', '_', '_', '6',
            '_', '_', '_', '_', '6', '9', '_', '_', '7', '_', '_', '_', '_', '1', '_', '_', '_',
            '9', '_', '_', '1', '_', '4', '_', '2', '_', '_', '_', '6', '_', '_', '9', '_', '_',
            '8', '_', '5', '_', '_', '_', '_', '_', '_', '_', '_', '_', '2', '_', '_', '_', '_',
            '6', '_', '_', '8', '_', '_', '_', '_', '_', '_', '1', '5', '6',
        ],
        [
            '1', '5', '6', '3', '7', '4', '2', '8', '9', '9', '7', '4', '2', '5', '8', '3', '6',
            '1', '3', '2', '8', '6', '9', '1', '5', '7', '4', '7', '8', '3', '1', '6', '5', '4',
            '9', '2', '5', '1', '9', '4', '8', '2', '6', '3', '7', '6', '4', '2', '9', '3', '7',
            '8', '1', '5', '8', '6', '1', '5', '4', '9', '7', '2', '3', '2', '3', '5', '7', '1',
            '6', '9', '4', '8', '4', '9', '7', '8', '2', '3', '1', '5', '6',
        ],
    ),
    (
        [
            '1', '9', '_', '8', '_', '_', '_', '_', '_', '_', '_', '6', '5', '_', '7', '_', '_',
            '_', '8', '_', '7', '_', '2', '_', '_', '_', '6', '_', '_', '_', '_', '6', '1', '_',
            '8', '_', '_', '_', '_', '_', '5', '_', '_', '_', '_', '_', '_', '_', '9', '_', '2',
            '_', '_', '_', '_', '6', '8', '_', '3', '_', '7', '_', '1', '7', '_', '1', '_', '_',
            '_', '_', '_', '8', '5', '_', '3', '_', '_', '_', '_', '_', '2',
        ],
        [
            '1', '9', '2', '8', '4', '6', '3', '7', '5', '4', '3', '6', '5', '1', '7', '8', '2',
            '9', '8', '5', '7', '3', '2', '9', '1', '4', '6', '2', '7', '5', '4', '6', '1', '9',
            '8', '3', '6', '8', '9', '7', '5', '3', '2', '1', '4', '3', '1', '4', '9', '8', '2',
            '5', '6', '7', '9', '6', '8', '2', '3', '4', '7', '5', '1', '7', '2', '1', '6', '9',
            '5', '4', '3', '8', '5', '4', '3', '1', '7', '8', '6', '9', '2',
        ],
    ),
    (
        [
            '_', '_', '8', '_', '_', '7', '9', '_', '1', '_', '_', '_', '1', '_', '_', '4', '8',
            '_', '_', '_', '_', '9', '_', '3', '_', '_', '_', '_', '_', '_', '_', '7', '5', '_',
            '_', '_', '_', '_', '_', '_', '_', '_', '1', '_', '_', '_', '5', '7', '6', '_', '8',
            '_', '_', '_', '_', '_', '_', '3', '5', '6', '_', '7', '_', '_', '_', '_', '_', '_',
            '_', '8', '_', '6', '9', '_', '_', '_', '_', '_', '_', '4', '_',
        ],
        [
            '6', '2', '8', '5', '4', '7', '9', '3', '1', '7', '9', '3', '1', '6', '2', '4', '8',
            '5', '1', '4', '5', '9', '8', '3', '7', '6', '2', '3', '1', '9', '4', '7', '5', '6',
            '2', '8', '8', '6', '4', '2', '3', '9', '1', '5', '7', '2', '5', '7', '6', '1', '8',
            '3', '9', '4', '4', '8', '1', '3', '5', '6', '2', '7', '9', '5', '3', '2', '7', '9',
            '4', '8', '1', '6', '9', '7', '6', '8', '2', '1', '5', '4', '3',
        ],
    ),
];

const EASY_PUZZLES: usize = 10;
const PUZZLES_EASY: [(SudokuPuzzle, SudokuPuzzle); EASY_PUZZLES] = [
    (
        [
            '7', '2', '3', '_', '_', '_', '1', '5', '9', '6', '_', '_', '3', '_', '2', '_', '_',
            '8', '8', '_', '_', '_', '1', '_', '_', '_', '2', '_', '7', '_', '6', '5', '4', '_',
            '2', '_', '_', '_', '4', '2', '_', '7', '3', '_', '_', '_', '5', '_', '9', '3', '1',
            '_', '4', '_', '5', '_', '_', '_', '7', '_', '_', '_', '3', '4', '_', '_', '1', '_',
            '3', '_', '_', '6', '9', '3', '2', '_', '_', '_', '7', '1', '4',
        ],
        [
            '7', '2', '3', '8', '4', '6', '1', '5', '9', '6', '1', '5', '3', '9', '2', '4', '7',
            '8', '8', '4', '9', '7', '1', '5', '6', '3', '2', '3', '7', '8', '6', '5', '4', '9',
            '2', '1', '1', '9', '4', '2', '8', '7', '3', '6', '5', '2', '5', '6', '9', '3', '1',
            '8', '4', '7', '5', '6', '1', '4', '7', '9', '2', '8', '3', '4', '8', '7', '1', '2',
            '3', '5', '9', '6', '9', '3', '2', '5', '6', '8', '7', '1', '4',
        ],
    ),
    (
        [
            '_', '7', '_', '_', '_', '_', '_', '2', '6', '5', '6', '_', '_', '3', '_', '_', '_',
            '_', '_', '_', '_', '7', '9', '_', '_', '5', '3', '_', '_', '8', '_', '_', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '1', '9', '_', '_',
            '_', '7', '_', '6', '8', '_', '_', '1', '3', '9', '_', '_', '_', '9', '_', '6', '_',
            '8', '_', '_', '7', '2', '_', '_', '_', '_', '_', '_', '1', '_',
        ],
        [
            '3', '7', '9', '8', '5', '4', '1', '2', '6', '5', '6', '4', '1', '3', '2', '7', '8',
            '9', '8', '1', '2', '7', '9', '6', '4', '5', '3', '9', '5', '8', '4', '2', '7', '3',
            '6', '1', '7', '2', '6', '3', '8', '1', '5', '9', '4', '4', '3', '1', '9', '6', '5',
            '8', '7', '2', '6', '8', '7', '2', '1', '3', '9', '4', '5', '1', '9', '5', '6', '4',
            '8', '2', '3', '7', '2', '4', '3', '5', '7', '9', '6', '1', '8',
        ],
    ),
    (
        [
            '_', '6', '_', '1', '_', '_', '_', '_', '_', '_', '_', '9', '_', '_', '6', '_', '_',
            '_', '_', '_', '_', '_', '3', '_', '8', '_', '_', '1', '4', '7', '_', '9', '_', '_',
            '8', '_', '6', '_', '_', '8', '_', '3', '_', '_', '_', '_', '9', '_', '_', '5', '_',
            '_', '_', '1', '9', '_', '_', '_', '6', '_', '7', '_', '_', '_', '5', '_', '7', '_',
            '_', '_', '4', '_', '_', '_', '8', '_', '_', '4', '_', '_', '_',
        ],
        [
            '5', '6', '2', '1', '7', '8', '9', '3', '4', '3', '8', '9', '5', '4', '6', '1', '7',
            '2', '4', '7', '1', '2', '3', '9', '8', '5', '6', '1', '4', '7', '6', '9', '2', '5',
            '8', '3', '6', '2', '5', '8', '1', '3', '4', '9', '7', '8', '9', '3', '4', '5', '7',
            '2', '6', '1', '9', '1', '4', '3', '6', '5', '7', '2', '8', '2', '5', '6', '7', '8',
            '1', '3', '4', '9', '7', '3', '8', '9', '2', '4', '6', '1', '5',
        ],
    ),
    (
        [
            '1', '_', '_', '3', '_', '7', '_', '_', '_', '_', '_', '8', '_', '_', '4', '1', '_',
            '_', '4', '6', '_', '8', '_', '5', '_', '_', '9', '_', '9', '_', '_', '_', '6', '8',
            '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
            '5', '1', '_', '5', '4', '_', '_', '_', '_', '_', '_', '3', '8', '_', '_', '_', '5',
            '_', '9', '_', '_', '_', '_', '7', '_', '_', '_', '6', '4', '_',
        ],
        [
            '1', '5', '9', '3', '2', '7', '4', '6', '8', '3', '7', '8', '6', '9', '4', '1', '5',
            '2', '4', '6', '2', '8', '1', '5', '7', '3', '9', '7', '9', '5', '1', '3', '6', '8',
            '2', '4', '2', '1', '4', '5', '7', '8', '3', '9', '6', '6', '8', '3', '9', '4', '2',
            '5', '1', '7', '5', '4', '1', '7', '6', '9', '2', '8', '3', '8', '2', '6', '4', '5',
            '3', '9', '7', '1', '9', '3', '7', '2', '8', '1', '6', '4', '5',
        ],
    ),
    (
        [
            '_', '7', '_', '_', '_', '_', '2', '_', '_', '_', '_', '_', '_', '9', '_', '_', '_',
            '_', '_', '9', '_', '_', '_', '3', '_', '6', '8', '_', '4', '8', '9', '_', '_', '6',
            '_', '_', '_', '_', '_', '_', '_', '_', '_', '5', '2', '_', '6', '_', '4', '2', '_',
            '_', '_', '_', '4', '_', '9', '_', '_', '_', '5', '_', '_', '_', '3', '_', '_', '_',
            '_', '9', '_', '_', '5', '8', '_', '_', '_', '_', '_', '4', '_',
        ],
        [
            '3', '7', '4', '1', '8', '6', '2', '9', '5', '8', '5', '6', '7', '9', '2', '3', '1',
            '4', '1', '9', '2', '5', '4', '3', '7', '6', '8', '2', '4', '8', '9', '3', '5', '6',
            '7', '1', '9', '1', '3', '6', '7', '8', '4', '5', '2', '7', '6', '5', '4', '2', '1',
            '8', '3', '9', '4', '2', '9', '3', '1', '7', '5', '8', '6', '6', '3', '1', '8', '5',
            '4', '9', '2', '7', '5', '8', '7', '2', '6', '9', '1', '4', '3',
        ],
    ),
    (
        [
            '_', '7', '_', '_', '_', '_', '5', '_', '9', '4', '_', '5', '7', '_', '_', '2', '_',
            '_', '_', '9', '_', '_', '_', '_', '_', '7', '_', '_', '_', '_', '_', '6', '_', '4',
            '_', '2', '_', '_', '7', '_', '_', '_', '6', '_', '_', '6', '8', '_', '_', '_', '_',
            '_', '9', '5', '_', '_', '4', '9', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
            '_', '8', '3', '_', '8', '3', '_', '_', '_', '4', '_', '_', '7',
        ],
        [
            '3', '7', '8', '2', '1', '6', '5', '4', '9', '4', '1', '5', '7', '9', '3', '2', '6',
            '8', '2', '9', '6', '4', '8', '5', '3', '7', '1', '9', '5', '3', '8', '6', '7', '4',
            '1', '2', '1', '4', '7', '5', '2', '9', '6', '8', '3', '6', '8', '2', '3', '4', '1',
            '7', '9', '5', '7', '2', '4', '9', '3', '8', '1', '5', '6', '5', '6', '9', '1', '7',
            '2', '8', '3', '4', '8', '3', '1', '6', '5', '4', '9', '2', '7',
        ],
    ),
    (
        [
            '_', '_', '_', '_', '_', '5', '9', '1', '_', '_', '_', '_', '_', '_', '4', '_', '_',
            '_', '_', '7', '_', '8', '1', '_', '_', '5', '6', '_', '_', '_', '9', '4', '_', '_',
            '_', '_', '_', '_', '3', '7', '_', '_', '_', '_', '8', '_', '_', '_', '_', '_', '_',
            '3', '9', '_', '8', '_', '_', '_', '6', '_', '_', '7', '3', '_', '_', '_', '3', '_',
            '_', '1', '_', '_', '_', '_', '6', '_', '_', '_', '5', '_', '_',
        ],
        [
            '4', '6', '8', '2', '3', '5', '9', '1', '7', '5', '9', '1', '6', '7', '4', '8', '3',
            '2', '3', '7', '2', '8', '1', '9', '4', '5', '6', '6', '8', '5', '9', '4', '3', '7',
            '2', '1', '9', '1', '3', '7', '5', '2', '6', '4', '8', '7', '2', '4', '1', '8', '6',
            '3', '9', '5', '8', '4', '9', '5', '6', '1', '2', '7', '3', '2', '5', '7', '3', '9',
            '8', '1', '6', '4', '1', '3', '6', '4', '2', '7', '5', '8', '9',
        ],
    ),
    (
        [
            '_', '_', '_', '4', '9', '6', '8', '_', '2', '_', '_', '_', '_', '_', '_', '_', '4',
            '9', '_', '_', '1', '_', '_', '8', '5', '_', '_', '_', '_', '_', '_', '_', '4', '2',
            '9', '_', '_', '_', '_', '_', '_', '_', '_', '_', '1', '5', '_', '_', '2', '_', '1',
            '_', '3', '_', '1', '_', '_', '_', '_', '_', '3', '_', '6', '_', '_', '_', '_', '4',
            '9', '_', '5', '_', '_', '_', '5', '_', '_', '3', '_', '_', '_',
        ],
        [
            '3', '5', '7', '4', '9', '6', '8', '1', '2', '6', '2', '8', '3', '1', '5', '7', '4',
            '9', '9', '4', '1', '7', '2', '8', '5', '6', '3', '7', '1', '6', '8', '3', '4', '2',
            '9', '5', '4', '3', '2', '9', '5', '7', '6', '8', '1', '5', '8', '9', '2', '6', '1',
            '4', '3', '7', '1', '9', '4', '5', '8', '2', '3', '7', '6', '2', '7', '3', '6', '4',
            '9', '1', '5', '8', '8', '6', '5', '1', '7', '3', '9', '2', '4',
        ],
    ),
    (
        [
            '_', '_', '_', '7', '_', '_', '_', '4', '_', '_', '7', '8', '_', '_', '6', '_', '5',
            '_', '2', '_', '_', '_', '5', '1', '_', '_', '8', '_', '_', '_', '_', '6', '_', '_',
            '_', '_', '_', '_', '_', '1', '_', '_', '_', '_', '5', '_', '3', '_', '_', '_', '_',
            '7', '_', '_', '4', '_', '_', '_', '1', '2', '_', '_', '_', '6', '_', '7', '_', '_',
            '_', '2', '3', '_', '_', '_', '3', '_', '_', '_', '4', '_', '_',
        ],
        [
            '3', '5', '1', '7', '2', '8', '9', '4', '6', '9', '7', '8', '4', '3', '6', '1', '5',
            '2', '2', '6', '4', '9', '5', '1', '3', '7', '8', '7', '9', '5', '2', '6', '4', '8',
            '1', '3', '8', '4', '2', '1', '7', '3', '6', '9', '5', '1', '3', '6', '5', '8', '9',
            '7', '2', '4', '4', '8', '9', '3', '1', '2', '5', '6', '7', '6', '1', '7', '8', '4',
            '5', '2', '3', '9', '5', '2', '3', '6', '9', '7', '4', '8', '1',
        ],
    ),
    (
        [
            '9', '7', '_', '_', '_', '_', '_', '_', '_', '3', '_', '2', '_', '_', '9', '_', '4',
            '_', '_', '4', '_', '_', '_', '_', '2', '_', '_', '1', '8', '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '9', '_', '_', '_', '8', '_', '_', '_', '_', '_', '_', '3', '7',
            '_', '_', '2', '5', '_', '_', '4', '_', '2', '1', '7', '_', '8', '3', '_', '9', '_',
            '_', '5', '_', '4', '_', '_', '_', '_', '_', '_', '_', '_', '_',
        ],
        [
            '9', '7', '8', '5', '2', '4', '3', '6', '1', '3', '5', '2', '6', '1', '9', '7', '4',
            '8', '6', '4', '1', '3', '7', '8', '2', '9', '5', '1', '8', '3', '2', '9', '6', '4',
            '5', '7', '7', '2', '9', '1', '4', '5', '8', '3', '6', '4', '6', '5', '8', '3', '7',
            '9', '1', '2', '5', '9', '6', '4', '8', '2', '1', '7', '3', '8', '3', '7', '9', '6',
            '1', '5', '2', '4', '2', '1', '4', '7', '5', '3', '6', '8', '9',
        ],
    ),
];

const HARD_PUZZLES: usize = 10;
const PUZZLES_HARD: [(SudokuPuzzle, SudokuPuzzle); HARD_PUZZLES] = [
    (
        [
            '_', '_', '_', '8', '4', '7', '_', '9', '_', '_', '_', '_', '_', '_', '_', '_', '3',
            '_', '_', '_', '4', '9', '2', '_', '_', '_', '1', '_', '4', '_', '_', '_', '5', '_',
            '_', '_', '_', '_', '_', '1', '_', '_', '5', '_', '8', '_', '8', '_', '_', '_', '4',
            '7', '_', '9', '_', '2', '5', '_', '9', '_', '_', '_', '3', '9', '_', '_', '_', '_',
            '_', '_', '2', '_', '_', '1', '_', '_', '_', '_', '_', '_', '7',
        ],
        [
            '1', '3', '6', '8', '4', '7', '2', '9', '5', '2', '9', '7', '5', '1', '6', '8', '3',
            '4', '8', '5', '4', '9', '2', '3', '6', '7', '1', '7', '4', '9', '6', '8', '5', '3',
            '1', '2', '3', '6', '2', '1', '7', '9', '5', '4', '8', '5', '8', '1', '2', '3', '4',
            '7', '6', '9', '6', '2', '5', '7', '9', '1', '4', '8', '3', '9', '7', '3', '4', '5',
            '8', '1', '2', '6', '4', '1', '8', '3', '6', '2', '9', '5', '7',
        ],
    ),
    (
        [
            '_', '4', '2', '_', '3', '7', '_', '_', '8', '_', '_', '_', '4', '_', '_', '_', '_',
            '9', '8', '_', '3', '_', '_', '6', '2', '_', '_', '3', '_', '_', '_', '_', '2', '_',
            '7', '5', '_', '2', '_', '_', '7', '_', '6', '_', '_', '_', '_', '9', '5', '_', '_',
            '4', '_', '_', '7', '_', '6', '_', '_', '_', '_', '_', '_', '4', '_', '_', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '7', '_', '_',
        ],
        [
            '9', '4', '2', '1', '3', '7', '5', '6', '8', '5', '6', '7', '4', '2', '8', '3', '1',
            '9', '8', '1', '3', '9', '5', '6', '2', '4', '7', '3', '8', '4', '6', '9', '2', '1',
            '7', '5', '1', '2', '5', '8', '7', '4', '6', '9', '3', '6', '7', '9', '5', '1', '3',
            '4', '8', '2', '7', '5', '6', '2', '4', '9', '8', '3', '1', '4', '3', '1', '7', '8',
            '5', '9', '2', '6', '2', '9', '8', '3', '6', '1', '7', '5', '4',
        ],
    ),
    (
        [
            '2', '_', '6', '_', '9', '_', '_', '_', '1', '_', '1', '9', '_', '_', '_', '3', '_',
            '_', '3', '_', '_', '_', '2', '1', '9', '_', '_', '_', '_', '7', '6', '_', '4', '1',
            '9', '_', '_', '_', '1', '7', '_', '_', '_', '_', '6', '_', '_', '_', '8', '_', '_',
            '_', '4', '7', '_', '_', '3', '_', '_', '_', '_', '_', '_', '_', '9', '_', '_', '_',
            '_', '_', '_', '2', '7', '_', '_', '_', '_', '_', '_', '_', '_',
        ],
        [
            '2', '5', '6', '3', '9', '8', '4', '7', '1', '4', '1', '9', '5', '7', '6', '3', '2',
            '8', '3', '7', '8', '4', '2', '1', '9', '6', '5', '8', '2', '7', '6', '5', '4', '1',
            '9', '3', '9', '4', '1', '7', '3', '2', '8', '5', '6', '6', '3', '5', '8', '1', '9',
            '2', '4', '7', '1', '6', '3', '2', '4', '5', '7', '8', '9', '5', '9', '4', '1', '8',
            '7', '6', '3', '2', '7', '8', '2', '9', '6', '3', '5', '1', '4',
        ],
    ),
    (
        [
            '_', '_', '1', '_', '_', '_', '_', '8', '_', '_', '_', '_', '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '2', '7', '8', '_', '_', '_', '_', '_', '8', '_', '_', '3', '_',
            '5', '7', '_', '2', '_', '8', '_', '_', '_', '_', '9', '_', '3', '5', '_', '_', '_',
            '2', '1', '_', '_', '7', '_', '_', '1', '_', '_', '2', '_', '_', '_', '_', '3', '_',
            '5', '6', '_', '_', '6', '5', '_', '_', '_', '_', '1', '_', '_',
        ],
        [
            '2', '9', '1', '5', '3', '4', '7', '8', '6', '7', '8', '3', '1', '6', '9', '5', '4',
            '2', '5', '4', '6', '2', '7', '8', '9', '3', '1', '1', '6', '8', '9', '2', '3', '4',
            '5', '7', '4', '2', '7', '8', '5', '1', '3', '6', '9', '9', '3', '5', '6', '4', '7',
            '2', '1', '8', '3', '7', '9', '4', '1', '6', '8', '2', '5', '8', '1', '2', '3', '9',
            '5', '6', '7', '4', '6', '5', '4', '7', '8', '2', '1', '9', '3',
        ],
    ),
    (
        [
            '_', '_', '_', '5', '_', '1', '_', '_', '_', '5', '_', '_', '_', '7', '_', '_', '_',
            '6', '6', '9', '_', '_', '_', '_', '_', '_', '7', '_', '_', '_', '_', '_', '7', '_',
            '_', '_', '_', '8', '2', '6', '_', '_', '_', '_', '5', '_', '_', '_', '_', '_', '_',
            '_', '1', '_', '_', '_', '_', '_', '5', '_', '_', '8', '_', '3', '_', '_', '2', '9',
            '_', '_', '_', '_', '_', '_', '8', '_', '_', '_', '6', '2', '3',
        ],
        [
            '8', '7', '4', '5', '6', '1', '3', '9', '2', '5', '2', '1', '9', '7', '3', '8', '4',
            '6', '6', '9', '3', '4', '8', '2', '1', '5', '7', '4', '3', '5', '1', '2', '7', '9',
            '6', '8', '1', '8', '2', '6', '4', '9', '7', '3', '5', '7', '6', '9', '8', '3', '5',
            '2', '1', '4', '2', '1', '7', '3', '5', '6', '4', '8', '9', '3', '4', '6', '2', '9',
            '8', '5', '7', '1', '9', '5', '8', '7', '1', '4', '6', '2', '3',
        ],
    ),
    (
        [
            '_', '_', '9', '8', '_', '5', '2', '_', '_', '_', '4', '_', '_', '_', '9', '_', '8',
            '_', '_', '2', '_', '1', '_', '_', '_', '9', '6', '_', '_', '4', '5', '_', '_', '_',
            '_', '_', '8', '_', '_', '_', '_', '_', '1', '6', '2', '_', '_', '6', '_', '9', '_',
            '4', '_', '_', '_', '_', '_', '_', '1', '7', '5', '2', '_', '_', '5', '7', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '1', '_',
        ],
        [
            '6', '7', '9', '8', '3', '5', '2', '4', '1', '3', '4', '1', '6', '2', '9', '7', '8',
            '5', '5', '2', '8', '1', '7', '4', '3', '9', '6', '2', '3', '4', '5', '6', '1', '8',
            '7', '9', '8', '9', '5', '7', '4', '3', '1', '6', '2', '7', '1', '6', '2', '9', '8',
            '4', '5', '3', '9', '6', '3', '4', '1', '7', '5', '2', '8', '1', '5', '7', '9', '8',
            '2', '6', '3', '4', '4', '8', '2', '3', '5', '6', '9', '1', '7',
        ],
    ),
    (
        [
            '6', '1', '3', '_', '4', '_', '_', '7', '_', '_', '5', '_', '2', '7', '_', '6', '_',
            '3', '2', '_', '_', '_', '_', '_', '_', '4', '_', '_', '_', '_', '5', '_', '_', '4',
            '_', '_', '_', '_', '_', '8', '3', '_', '_', '_', '2', '_', '_', '2', '_', '_', '_',
            '7', '3', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '3', '_', '7', '5',
            '4', '_', '_', '_', '_', '_', '8', '_', '1', '_', '3', '_', '_',
        ],
        [
            '6', '1', '3', '9', '4', '5', '2', '7', '8', '4', '5', '9', '2', '7', '8', '6', '1',
            '3', '2', '8', '7', '1', '6', '3', '5', '4', '9', '3', '9', '1', '5', '2', '7', '4',
            '8', '6', '5', '7', '4', '8', '3', '6', '1', '9', '2', '8', '6', '2', '4', '9', '1',
            '7', '3', '5', '1', '4', '5', '3', '8', '2', '9', '6', '7', '9', '3', '6', '7', '5',
            '4', '8', '2', '1', '7', '2', '8', '6', '1', '9', '3', '5', '4',
        ],
    ),
    (
        [
            '_', '2', '_', '_', '_', '_', '_', '6', '_', '_', '7', '_', '_', '_', '5', '9', '2',
            '_', '_', '6', '_', '3', '_', '4', '7', '_', '_', '_', '_', '2', '_', '_', '_', '_',
            '_', '_', '9', '_', '1', '_', '4', '3', '_', '_', '_', '_', '5', '_', '_', '_', '6',
            '_', '_', '_', '_', '_', '5', '4', '_', '7', '_', '_', '_', '_', '_', '_', '5', '_',
            '_', '3', '_', '4', '_', '_', '_', '_', '6', '9', '_', '_', '1',
        ],
        [
            '3', '2', '8', '9', '7', '1', '4', '6', '5', '1', '7', '4', '6', '8', '5', '9', '2',
            '3', '5', '6', '9', '3', '2', '4', '7', '1', '8', '6', '3', '2', '7', '5', '8', '1',
            '4', '9', '9', '8', '1', '2', '4', '3', '6', '5', '7', '4', '5', '7', '1', '9', '6',
            '8', '3', '2', '8', '1', '5', '4', '3', '7', '2', '9', '6', '7', '9', '6', '5', '1',
            '2', '3', '8', '4', '2', '4', '3', '8', '6', '9', '5', '7', '1',
        ],
    ),
    (
        [
            '_', '6', '7', '4', '_', '_', '_', '_', '_', '_', '8', '_', '_', '5', '_', '2', '_',
            '_', '2', '_', '9', '_', '7', '_', '_', '_', '_', '5', '_', '_', '_', '_', '_', '1',
            '_', '_', '6', '9', '2', '_', '_', '4', '_', '_', '_', '_', '_', '4', '_', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '2', '7', '9', '_', '_', '_', '_', '_', '8', '1',
            '_', '_', '5', '_', '_', '7', '_', '_', '_', '_', '8', '_', '2',
        ],
        [
            '1', '6', '7', '4', '9', '2', '5', '3', '8', '4', '8', '3', '6', '5', '1', '2', '7',
            '9', '2', '5', '9', '3', '7', '8', '6', '4', '1', '5', '3', '8', '7', '6', '9', '1',
            '2', '4', '6', '9', '2', '1', '3', '4', '7', '8', '5', '7', '1', '4', '2', '8', '5',
            '3', '9', '6', '8', '4', '1', '5', '2', '7', '9', '6', '3', '9', '2', '6', '8', '1',
            '3', '4', '5', '7', '3', '7', '5', '9', '4', '6', '8', '1', '2',
        ],
    ),
    (
        [
            '_', '_', '_', '_', '_', '_', '_', '1', '_', '7', '8', '_', '9', '_', '_', '_', '3',
            '_', '_', '_', '_', '_', '_', '8', '9', '_', '4', '_', '_', '_', '3', '_', '_', '_',
            '_', '1', '_', '_', '4', '_', '6', '_', '_', '7', '_', '_', '_', '_', '_', '_', '7',
            '8', '6', '3', '_', '1', '_', '6', '_', '4', '_', '_', '_', '_', '7', '_', '_', '_',
            '_', '_', '_', '9', '_', '_', '3', '_', '9', '2', '_', '_', '6',
        ],
        [
            '4', '9', '2', '5', '3', '6', '7', '1', '8', '7', '8', '5', '9', '4', '1', '6', '3',
            '2', '3', '6', '1', '2', '7', '8', '9', '5', '4', '6', '2', '7', '3', '8', '5', '4',
            '9', '1', '8', '3', '4', '1', '6', '9', '2', '7', '5', '1', '5', '9', '4', '2', '7',
            '8', '6', '3', '9', '1', '8', '6', '5', '4', '3', '2', '7', '2', '7', '6', '8', '1',
            '3', '5', '4', '9', '5', '4', '3', '7', '9', '2', '1', '8', '6',
        ],
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    /*
        Quick sanity check to make sure theres nothing obviously wrong with the puzzles
    */

    #[test]
    fn solution_sanity_check() {
        for (puzzle, solution) in PUZZLES_BEGINNER {
            for (a, b) in puzzle.iter().zip(solution.iter()) {
                if *a != '_' {
                    assert_eq!(*a, *b);
                }
            }
        }

        for (puzzle, solution) in PUZZLES_EASY {
            for (a, b) in puzzle.iter().zip(solution.iter()) {
                if *a != '_' {
                    assert_eq!(*a, *b);
                }
            }
        }

        for (puzzle, solution) in PUZZLES_HARD {
            for (a, b) in puzzle.iter().zip(solution.iter()) {
                if *a != '_' {
                    assert_eq!(*a, *b);
                }
            }
        }
    }
}
