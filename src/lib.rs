use core::fmt;
use std::collections::HashSet;
use std::fmt::Formatter;

pub struct Sudoku {
    board: Vec<u8>,
}

impl Sudoku {
    pub fn new(s: &str) -> Self {
        let line = String::from(s);
        let board = line
            .chars()
            .take(81)
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        Sudoku { board }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            &self
                .board
                .chunks(9)
                .map(|row| row
                    .to_vec()
                    .chunks(3)
                    .map(|block| block
                        .to_vec()
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<_>>()
                        .join(" "))
                    .collect::<Vec<_>>()
                    .join(" | "))
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|group_of_rows| group_of_rows.to_vec().join("\n"))
                .collect::<Vec<_>>()
                .join("\n---------------------\n")
        )
    }
}

pub fn solve_sudoku(game: &mut Sudoku) -> bool {
    match find_first_empty_spot(game) {
        None => true,
        Some((row, col)) => {
            for option in options_for(game, row, col) {
                let idx = (row * 9 + col) as usize;
                game.board[idx] = option;
                if solve_sudoku(game) {
                    return true;
                }
                game.board[idx] = 0;
            }
            false
        }
    }
}

fn find_first_empty_spot(game: &mut Sudoku) -> Option<(u8, u8)> {
    let (idx, _) = game
        .board
        .iter()
        .enumerate()
        .find(|(_, i)| **i == 0 as u8)?;
    Some(((idx as u8 / 9), (idx as u8 % 9)))
}

fn options_for(game: &mut Sudoku, row: u8, col: u8) -> Vec<u8> {
    let taken_nums_in_row: HashSet<u8> = game
        .board
        .iter()
        .skip(row as usize * 9)
        .take(9)
        .filter(|i| **i != 0)
        .copied()
        .collect();
    let taken_nums_in_col: HashSet<u8> = game
        .board
        .iter()
        .skip(col as usize)
        .step_by(9)
        .filter(|i| **i != 0)
        .copied()
        .collect();

    let start = (row / 3 * 27 + col / 3 * 3) as usize;
    let taken_nums_in_square: HashSet<u8> = [
        &game.board[start..start + 3],
        &game.board[start + 9..start + 12],
        &game.board[start + 18..start + 21],
    ]
    .iter()
    .map(|slice| slice.iter())
    .flatten()
    .filter(|i| **i != 0)
    .copied()
    .collect();

    let free_num_in_row = (1..=9)
        .collect::<HashSet<u8>>()
        .difference(&taken_nums_in_row)
        .copied()
        .collect::<HashSet<u8>>()
        .difference(&taken_nums_in_col)
        .copied()
        .collect::<HashSet<u8>>()
        .difference(&taken_nums_in_square)
        .copied()
        .collect();

    // println!("Row {:?}", taken_nums_in_row);
    // println!("Col {:?}", taken_nums_in_col);
    // println!("Squ {:?}", taken_nums_in_square);
    // println!("Free {:?}", free_num_in_row);
    // panic!("Stop");

    free_num_in_row
}
