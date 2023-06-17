use std::io::{self, Write};
use rand::Rng;

/// The size of the Sudoku board.
const SIZE: usize = 9;
/// The value representing an empty cell in the Sudoku board.
const EMPTY_CELL: u8 = 0;

/// Checks if placing a number in a specific position is valid.
///
/// It checks if the number is already present in the same row, column, or 3x3 grid.
///
/// # Arguments
///
/// * `board` - The Sudoku board.
/// * `row` - The row index.
/// * `col` - The column index.
/// * `num` - The number to be placed.
///
/// # Returns
///
/// * `true` if the number is valid in the given position, otherwise `false`.
fn is_valid(board: &[[u8; SIZE]; SIZE], row: usize, col: usize, num: u8) -> bool {
    // Check if the number is already present in the row
    for i in 0..SIZE {
        if board[row][i] == num {
            return false;
        }
    }

    // Check if the number is already present in the column
    for i in 0..SIZE {
        if board[i][col] == num {
            return false;
        }
    }

    // Check if the number is already present in the 3x3 grid
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }

    true
}

/// Solves the Sudoku puzzle using a backtracking algorithm.
///
/// The function modifies the input `board` to store the solved puzzle.
///
/// # Arguments
///
/// * `board` - The Sudoku board.
///
/// # Returns
///
/// * `true` if a solution is found, otherwise `false`.
fn solve_sudoku(board: &mut [[u8; SIZE]; SIZE]) -> bool {
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board[row][col] == EMPTY_CELL {
                for num in 1..=9 {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;

                        if solve_sudoku(board) {
                            return true;
                        }

                        board[row][col] = EMPTY_CELL;
                    }
                }

                return false;
            }
        }
    }

    true
}

/// Generates a Sudoku board with random numbers.
///
/// Approximately 60% of the board is filled with random valid numbers.
///
/// # Returns
///
/// The generated Sudoku board.
fn generate_board() -> [[u8; SIZE]; SIZE] {
    let mut rng = rand::thread_rng();
    let mut board = [[0; SIZE]; SIZE];

    for i in 0..SIZE {
        for j in 0..SIZE {
            if rng.gen::<f64>() <= 0.6 { // Fill approximately 60% of the board
                let num = rng.gen_range(1..=9);
                if is_valid(&board, i, j, num) {
                    board[i][j] = num;
                }
            }
        }
    }

    board
}

/// Prints the Sudoku board.
///
/// # Arguments
///
/// * `board` - The Sudoku board to print.
fn print_board(board: &[[u8; SIZE]; SIZE]) {
    for i in 0..SIZE {
        if i % 3 == 0 && i != 0 {
            println!("---------------------");
        }
        for j in 0..SIZE {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            print!("{} ", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let mut board = generate_board();

    println!("Sudoku Board:");
    print_board(&board);

    if solve_sudoku(&mut board) {
        println!("Solution:");
        print_board(&board);
    } else {
        println!("No solution found!");
    }
}
