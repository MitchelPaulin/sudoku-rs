use crate::puzzle::SudokuPuzzle;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/*
    Apply a series of transformations to the puzzle that keeps the puzzle solveable

    This means we can use one puzzle "seed" and get plenty of puzzles out of it

    Note: All transforms that need to be done to the puzzle must also be done on the solution
*/

pub fn transform_puzzle(game: (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    //randomize_numbers(game);
    rotate(game);
}

/*
    Create a bijection from [1,9] -> [1,9] and map all numbers to create a new puzzle
*/
fn randomize_numbers((puzzle, solution): (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    let mut rng = thread_rng();
    let mut mapping = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    mapping.shuffle(&mut rng);

    for i in 0..81 {
        solution[i] = match solution[i] {
            '1' => mapping[0],
            '2' => mapping[1],
            '3' => mapping[2],
            '4' => mapping[3],
            '5' => mapping[4],
            '6' => mapping[5],
            '7' => mapping[6],
            '8' => mapping[7],
            '9' => mapping[8],
            _ => '_',
        };

        // ensure that the puzzle and solution stay in sync
        if puzzle[i] != '_' {
            puzzle[i] = solution[i];
        }
    }
}

/*
    Rotate a puzzle, either 0, 90, 180, or 270 degrees
*/
fn rotate((puzzle, solution): (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    let rot_count = rand::thread_rng().gen_range(0..=3);
    for _ in 0..rot_count {
        rotate_90(puzzle);
        rotate_90(solution);
    }
}

/*
    Rotate the given puzzle 90 degrees
*/
fn rotate_90(matrix: &mut SudokuPuzzle) {
    let mut res = *matrix;

    for i in 0..9 {
        for j in 0..9 {
            // index magic was adapted from this helpful post: https://math.stackexchange.com/a/1676457/528931
            res[j * 9 + 9 - i - 1] = matrix[i * 9 + j];
        }
    }

    // copy results back to the original matrix
    for i in 0..9 {
        for j in 0..9 {
            matrix[i * 9 + j] = res[i * 9 + j];
        }
    }
}
