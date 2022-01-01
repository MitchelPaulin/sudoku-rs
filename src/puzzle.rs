use std::fmt::{self};

const PUZZLES_EASY: [(SudokuPuzzle, SudokuPuzzle); 1] = [(
    [
        '7', '2', '3', '_', '_', '_', '1', '5', '9', '6', '_', '_', '3', '_', '2', '_', '_', '8',
        '8', '_', '_', '_', '1', '_', '_', '_', '2', '_', '7', '_', '6', '5', '4', '_', '2', '_',
        '_', '_', '4', '2', '_', '7', '3', '_', '_', '_', '5', '_', '9', '3', '1', '_', '4', '_',
        '5', '_', '_', '_', '7', '_', '_', '_', '3', '4', '_', '_', '1', '_', '3', '_', '_', '6',
        '9', '3', '2', '_', '_', '_', '7', '1', '4',
    ],
    [
        '7', '2', '3', '8', '4', '6', '1', '5', '9', '6', '1', '5', '3', '9', '2', '4', '7', '8',
        '8', '4', '9', '7', '1', '5', '6', '3', '2', '3', '7', '8', '6', '5', '4', '9', '2', '1',
        '1', '9', '4', '2', '8', '7', '3', '6', '5', '2', '5', '6', '9', '3', '1', '8', '4', '7',
        '5', '6', '1', '4', '7', '9', '2', '8', '3', '4', '8', '7', '1', '2', '3', '5', '9', '6',
        '9', '3', '2', '5', '6', '8', '7', '1', '4',
    ],
)];

pub const EMPTY_SPACE: char = '_';

pub type SudokuPuzzle = [char; 81];

pub enum PuzzleType {
    Easy,
    Hard,
}

impl fmt::Display for PuzzleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuzzleType::Easy => write!(f, "Easy"),
            PuzzleType::Hard => write!(f, "Hard"),
        }
    }
}

pub struct Puzzle {
    pub puzzle: SudokuPuzzle,
    pub solution: SudokuPuzzle,
    pub difficulty: PuzzleType,
}

impl Puzzle {
    pub fn new_puzzle(difficulty: PuzzleType) -> Puzzle {
        Puzzle {
            puzzle: PUZZLES_EASY[0].0,
            solution: PUZZLES_EASY[0].1,
            difficulty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_sanity_check() {
        for (puzzle, solution) in PUZZLES_EASY {
            for (a, b) in puzzle.iter().zip(solution.iter()) {
                if *a != '_' {
                    assert_eq!(*a, *b);
                }
            }
        }
    }
}
