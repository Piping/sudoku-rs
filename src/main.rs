pub mod lib;

use sudoku_rs::{Sudoku, solve_sudoku};

fn main() {
    let mut instance = Sudoku::new(
        "050000000300420000006000984607100000000200070000080090400600030700000005030008410",
    );
    println!("{}", instance);
    if solve_sudoku(&mut instance) {
        println!("Solved:\n{}", instance);
    }
}
