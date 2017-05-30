extern crate rand;

use rand::distributions::{IndependentSample, Range};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Square {
    Queen { r: u8, c: u8 },
    Empty,
    Unattainable,
}

type Board = Vec<Vec<Square>>;
type Position = (u8, u8);

fn main() {
    let size: u8 = 8;
    let mut board: Board = gen_empty_board(size);
    let mut complete: bool = false;
    let mut queen_position: Position = gen_random_pos(size);
    let mut empty_squares: Vec<Position> = Vec::new();
    let mut queens: Vec<Position> = Vec::new();

    while !complete {
        // Reset empty squares
        empty_squares = Vec::new();

        let op_res = place_queen(&mut board, queen_position)
			.and_then(|()| { fill_row(&mut board, queen_position) })
			.and_then(|()| { fill_col(&mut board, queen_position) })
			.and_then(|()| { fill_diagonals(&mut board, get_diagonals_to_fill(size, queen_position)) });

        match op_res {
            Ok(()) => queens.push(queen_position),
            _ => {
                board = gen_empty_board(size);
                queens = Vec::new()
            }
        }

        for row_index in 0..size {
            for col_index in 0..size {
                if is_empty_at_pos(&board, (row_index, col_index)) {
                    empty_squares.push((row_index, col_index));
                }
            }
        }

        if (empty_squares.len() == 0) && (queens.len() as u8 == size) {
            complete = true;
        } else {
            queen_position = pick_random_pos(&empty_squares).unwrap_or(gen_random_pos(size));
        }
    }

    println!("{:?}", queens);
}

fn gen_empty_board(size: u8) -> Board {
    vec![vec![Square::Empty; size as usize]; size as usize]
}

fn place_queen(board: &mut Board, (r, c): Position) -> Result<(), Position> {
    let mut error: Option<Position> = None;
    match is_empty_at_pos(&board, (r, c)) {
        true => board[r as usize][c as usize] = Square::Queen { r: r, c: c },
        _ => error = Some((r as u8, c as u8)),
    }
    match error {
        Some(pos) => Err(pos),
        _ => Ok(()),
    }
}

fn fill_pos(board: &mut Board, (r, c): Position) -> Result<(), Position> {
    let mut error: Option<Position> = None;
    match board[r as usize][c as usize] {
        Square::Empty | Square::Unattainable => {
            board[r as usize][c as usize] = Square::Unattainable
        }
        _ => error = Some((r as u8, c as u8)),
    }
    match error {
        Some(pos) => Err(pos),
        _ => Ok(()),
    }
}

fn fill_row(board: &mut Board, (r, c): Position) -> Result<(), Position> {
    let mut error: Option<Position> = None;
    for row_index in 0..board.len() {
        if row_index == r as usize {
            for col_index in 0..board[row_index].len() {
                match board[row_index][col_index] {
                    Square::Empty | Square::Unattainable => {
                        board[row_index][col_index] = Square::Unattainable
                    }
                    _ => {
                        match col_index == c as usize {
                            true => {}
                            _ => error = Some((row_index as u8, col_index as u8)),
                        }
                    }
                }
            }
        }
    }
    match error {
        Some(pos) => Err(pos),
        _ => Ok(()),
    }
}

fn fill_col(board: &mut Board, (r, c): Position) -> Result<(), Position> {
    let mut error: Option<Position> = None;
    for row_index in 0..board.len() {
        for col_index in 0..board[row_index].len() {
            if col_index == c as usize {
                match board[row_index][col_index] {
                    Square::Empty | Square::Unattainable => {
                        board[row_index][col_index] = Square::Unattainable
                    }
                    _ => {
                        match col_index == c as usize {
                            true => {}
                            _ => error = Some((row_index as u8, col_index as u8)),
                        }
                    }
                }
            }
        }
    }
    match error {
        Some(pos) => Err(pos),
        _ => Ok(()),
    }
}

fn fill_diagonals(board: &mut Board, positions: Vec<Position>) -> Result<(), Position> {
    let mut error: Option<Position> = None;
    for position in positions {
        match fill_pos(board, position) {
            Err(pos) => error = Some(pos),
            _ => {}
        }
    }
    match error {
        Some(pos) => Err(pos),
        _ => Ok(()),
    }
}

fn get_diagonals_to_fill(size: u8, (r, c): Position) -> Vec<Position> {
    let mut cols: (i8, i8) = (c as i8, c as i8);
    let mut pairs: Vec<(i8, i8)> = Vec::new();

    // Bottom: iterate through each row from queen start position to the end of the board
    for i in r..size {
        // ignore the starting row, for queen is already placed here
        if i != r {
            pairs.push((i as i8, cols.0));
            pairs.push((i as i8, cols.1));
        }

        cols.0 -= 1;
        cols.1 += 1;
    }

    // reset starting columns pair
    cols = (c as i8, c as i8);

    // Top: iterate through each row from 0 to queen start position,
    // in reverse order; queen pos will be ignored
    for i in (0..r).rev() {
        cols.0 -= 1;
        cols.1 += 1;

        pairs.push((i as i8, cols.0));
        pairs.push((i as i8, cols.1));
    }

    pairs
        .into_iter()
        .filter(|x| can_be_col_index(size, x.1))
        .map(|pair| (pair.0 as u8, pair.1 as u8))
        .collect()
}

fn can_be_col_index(size: u8, col: i8) -> bool {
    col < size as i8 && col >= 0
}

fn is_empty_at_pos(board: &Board, (r, c): Position) -> bool {
    board[r as usize][c as usize] == Square::Empty
}

fn gen_random_pos(size: u8) -> Position {
    let between = Range::new(0, size);
    let mut rng = rand::thread_rng();
    (between.ind_sample(&mut rng), between.ind_sample(&mut rng))
}

fn pick_random_pos(positions: &Vec<Position>) -> Result<Position, ()> {
    match positions.len() {
        0 => Err(()),
        len => {
            let between = Range::new(0, len);
            let mut rng = rand::thread_rng();
            // Does this move the value (at index) out of positions?
            Ok(positions[between.ind_sample(&mut rng)])
            // Ok(positions.clone()[between.ind_sample(&mut rng)])
        }
    }
}
