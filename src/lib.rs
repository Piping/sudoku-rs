use core::fmt;
use std::collections::HashSet;
use std::fmt::Formatter;

pub struct Sudoku {
    board: Vec<u8>,
    rows: Vec<u16>,
    cols: Vec<u16>,
    squares: Vec<u16>,
}

impl Sudoku {
    pub fn new(s: &str) -> Self {
        let line = String::from(s);
        let board = line
            .chars()
            .take(81)
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        let mut rows = vec![0; 9];
        let mut cols = vec![0; 9];
        let mut squares = vec![0; 9];

        // bitmap spot on set means that option is available
        // for memorization, we only need 9 rows, 9 cols, and 9 squares to check available moves
        for row in 0..9 {
            let mut taken_nums_in_row = 0;
            for i in board
                .iter()
                .skip(row as usize * 9)
                .take(9)
                .filter(|i| **i != 0)
            {
                taken_nums_in_row |= 1 << (i - 1);
            }
            rows[row] = !taken_nums_in_row;
        }
        for col in 0..9 {
            let mut taken_nums_in_col = 0;
            for i in board
                .iter()
                .skip(col as usize)
                .step_by(9)
                .filter(|i| **i != 0)
            {
                taken_nums_in_col |= 1 << (i - 1);
            }
            cols[col] = !taken_nums_in_col;
        }

        for idx in 0..9 {
            let row = idx/3*3;
            let col = idx%3*3;
            let mut taken_nums_in_square = 0;
            let start = (row / 3 * 27 + col / 3 * 3) as usize;
            [
                &board[start..start + 3],
                &board[start + 9..start + 12],
                &board[start + 18..start + 21],
            ]
            .iter()
            .map(|slice| slice.iter())
            .flatten()
            .filter(|i| **i != 0)
            .for_each(|i| taken_nums_in_square |= 1 << (i - 1));
            squares[idx] = !taken_nums_in_square;
        }

        Sudoku {
            board,
            rows,
            cols,
            squares,
        }
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
            let row = row as usize;
            let col = col as usize;
            let idx = row * 9 + col;
            let square = row / 3 * 3 + col / 3;
            let bitset = game.rows[row] & game.cols[col] & game.squares[square];
            for option in 1..=9 {
                let mask = 1 << (option - 1);
                if bitset & mask != 0 {
                    // use the option by clear
                    game.rows[row] &= !mask;
                    game.cols[col] &= !mask;
                    game.squares[square] &= !mask;
                    game.board[idx] = option;
                    if solve_sudoku(game) {
                        return true;
                    }
                    game.board[idx] = 0;
                    // unuse the option by set
                    game.rows[row] |= mask;
                    game.cols[col] |= mask;
                    game.squares[square] |= mask;
                }
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

fn options_for_with_hashset(game: &mut Sudoku, row: u8, col: u8) -> Vec<u8> {
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
