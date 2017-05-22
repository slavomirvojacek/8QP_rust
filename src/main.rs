#[derive(Copy, Clone, Debug)]
enum Square {
	Queen {r: u8, c: u8},
	Empty,
	Unattainable
}

type Board = Vec<Vec<Square>>;

fn main() {
	let size: u8 = 5;
	let queen_position: (u8, u8) = (2, 2);
	let diagonals_to_fill: Vec<(u8, u8)> = get_diagonals_to_fill(size, queen_position);

	let mut board: Board = vec![vec![Square::Empty; size as usize]; size as usize];

	board = place_queen(board, queen_position);
	board = fill_row(board, queen_position);
	board = fill_col(board, queen_position);

	for position in diagonals_to_fill {
		board = fill_pos(board, position);
	}

	for r in board {
		println!("{:?}", r);
	}
}

fn place_queen(mut board: Board, pos: (u8, u8)) -> Board {
	let (r, c) = pos;
	board[r as usize][c as usize] = Square::Queen { r: r, c: c };
	board
}

fn fill_pos(mut board: Board, pos: (u8, u8)) -> Board {
	let (r, c) = pos;
	match board[r as usize][c as usize] {
		Square::Empty => board[r as usize][c as usize] = Square::Unattainable,
		_ => panic!("Unexpected Queen"),
	}
	board
}

fn fill_row(mut board: Board, pos: (u8, u8)) -> Board {
	let (r, c) = pos;
	for row_index in 0..board.len() {
		if row_index == r as usize {
			for col_index in 0..board[row_index].len() {
				match board[row_index][col_index] {
					Square::Empty => board[row_index][col_index] = Square::Unattainable,
					_ => match col_index == c as usize {
						true => {},
						_ => panic!("Unexpected Queen"),
					},
				}
			}
		}
	}
	board
}

fn fill_col(mut board: Board, pos: (u8, u8)) -> Board {
	let (r, c) = pos;
	for row_index in 0..board.len() {
		for col_index in 0..board[row_index].len() {
			if col_index == c as usize {
				match board[row_index][col_index] {
					Square::Empty => board[row_index][col_index] = Square::Unattainable,
					_ => match col_index == c as usize {
						true => {},
						_ => panic!("Unexpected Queen"),
					},
				}
			}
		}
	}
	board
}

fn get_diagonals_to_fill(size: u8, pos: (u8, u8)) -> Vec<(u8, u8)> {
	let (r, c) = pos;
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

	// Top: iterate through each row from 0 to queen start position, in reverse order; queen pos will be ignored
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
	col <= size as i8 && col >= 0
}
