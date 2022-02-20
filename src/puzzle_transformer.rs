use crate::puzzle::SudokuPuzzle;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/*
    Apply a series of transformations to the puzzle that keeps the puzzle solveable

    This means we can use one puzzle "seed" and get plenty of puzzles out of it

    Note: All transforms performed on the puzzle must also be done to the solution
*/

pub fn transform_puzzle(mut game: (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    randomize_numbers(&mut game);
    flop(&mut game);
    rotate(&mut game);
}

/*
    Create a bijection from [1,9] -> [1,9] and map all numbers to create a new puzzle
*/
fn randomize_numbers((puzzle, solution): &mut (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
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
fn rotate((puzzle, solution): &mut (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
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
    let mut res = matrix.clone();

    for i in 0..9 {
        for j in 0..9 {
            // index magic was adapted from this helpful post: https://math.stackexchange.com/a/1676457/528931
            res[j * 9 + 9 - i - 1] = matrix[i * 9 + j];
        }
    }

    // copy results back to the original matrix
    matrix[..81].clone_from_slice(&res[..81]);
}

/*
    Reflect the puzzle either vertically or horizontally with a 50 percent probability 
*/
fn flop(game: &mut (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    if rand::thread_rng().gen_bool(0.5) {
        reflect_horizontal(game);
    }

    if rand::thread_rng().gen_bool(0.5) {
        reflect_vertical(game);
    }
}

/*
    Reflect the puzzle horizontally across the center row
*/
fn reflect_horizontal((puzzle, solution): &mut (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    let mut res_puzzle = **puzzle;
    let mut res_solution = **solution;

    for i in 0..9 {
        for j in 0..9 {
            res_puzzle[i * 9 + j] = puzzle[(9 - i - 1) * 9 + j];
            res_solution[i * 9 + j] = solution[(9 - i - 1) * 9 + j];
        }
    }

    // copy results
    puzzle[..81].clone_from_slice(&res_puzzle[..81]);
    solution[..81].clone_from_slice(&res_solution[..81]);
}

/*
    Reflect the puzzle vertically across the center column

    We can be a bit clever here, rotating by 90, reflecting, then rotating another 270
    has the same effect as reflecting vertically

    Since the matrix will be rotated anyway though, we can just skip the 270 rotation
*/
fn reflect_vertical(game: &mut (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    rotate_90(game.0);
    rotate_90(game.1);
    reflect_horizontal(game);
}
