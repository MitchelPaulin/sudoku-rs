use crate::puzzle::SudokuPuzzle;
use rand::seq::SliceRandom;
use rand::thread_rng;

/*
    Apply a series of transformations to the puzzle that keeps the puzzle solveable

    This means we can use one puzzle "seed" and get plenty of puzzles out of it

    Note: All transforms that need to be done to the puzzle must also be done on the solution
*/

pub fn transform_puzzle(game: (&mut SudokuPuzzle, &mut SudokuPuzzle)) {
    randomize_numbers(game);
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
