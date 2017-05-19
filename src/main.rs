#[derive(Copy, Clone, Debug)]
enum Square {
	Queen {r: u8, c: u8},
	Empty,
	Unattainable
}

type Board = Vec<Vec<Square>>;

fn main() {
	let size: u8 = 8;
	let pos: (u8, u8) = (2, 2);

	let mut board: Board = vec![vec![Square::Empty; size as usize]; size as usize];

	board = place_queen(board, pos);
	board = fill_row(board, pos);
	board = fill_col(board, pos);

	get_positions_to_fill(size, pos);

	for r in board {
		println!("{:?}", r);
	}	
}

fn place_queen(mut board: Board, pos: (u8, u8)) -> Board {
	let (r, c) = pos;
	board[r as usize][c as usize] = Square::Queen { r: r, c: c };
	board
}

// fn fill_square(mut board: Board, pos: (u8, u8)) -> Board {
// 	let (r, c) = pos;
// 	board[r as usize][c as usize] = Square::Unattainable;
// 	board
// }


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
					} 
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
					}
				}
			}
		}
	}
	board
}

fn get_positions_to_fill(size: u8, pos: (u8, u8)) {
	let (r, c) = pos;
	let mut cols: (i8, i8) = (c as i8, c as i8);

	// Bottom: iterate through each row from queen start position to the end of the board
	for i in r..size {
		// ignore the starting row, for queen is already placed here
		if i != r {
			println!("row index {}; col pair {:?}", i, cols);
		}

		// with each row, calculate the next pair of columns to be filled
		cols.0 -= 1;
		cols.1 += 1;
	}

	// reset starting columns pair
	cols = (c as i8, c as i8);

	// Top: iterate through each row from 0 to queen start position, in reverse order; queen pos will be ignored
	for i in (0..r).rev() {
		cols.0 -= 1;
		cols.1 += 1;

		println!("row index {}; col pair {:?}", i, cols);
	}
}

// .iter_mut()
		// .map(|s| match s {
		// 	&mut Square::Empty => Square::Unattainable,
		// 	_ => *s,
		// })
		// .collect();

// fn fill_col_at(mut board: Vec<Vec<Square>>, r: usize, i: usize) -> Vec<Vec<Square>> {
// 	for index in 0..board[r].len() {
// 		if (index == i) && (board[r][index] == Square::Empty) {
// 			board[r][index] = Square::Unattainable;
// 		};
// 	};

// 	board
// }

// fn fill_row(mut row: Vec<Square>) -> Vec<Square> {
// 	row
// 		.iter_mut()
// 		.map(|s| match s {
// 			&mut Square::Empty => Square::Unattainable,
// 			_ => *s,
// 		})
// 		.collect()
// }

// fn can_place_queen(board: &Board, row: u8, column: u8) -> Result<(usize, usize), &str> {
// 	match board[row as usize][column as usize] {
// 		Square::Empty => Ok((row as usize, column as usize)),
// 		Square::Unattainable => Err("R{} C{} is unattainable"),
// 		_ => Err("R{} C{} is already occupied by a Queen"),
// 	}
// }

// fn place_queen(mut board: Board, r: u8, c: u8) {
// 	if let Ok((row, col)) = can_place_queen(&board, r, c) {
// 		board[row][col] = Square::Queen { r: r, c: c };
// 		board = fill_unattainable_row(board, row);
// 		// board = fill_unattainable_column(board, col);
// 		// board = fill_unattainable_diagonal_bottom_top(board, row, col);
// 		// board = fill_unattainable_diagonal_top_bottom(board, row, col);
// 		for r in board.iter() {
// 			println!("{:?}", r);
// 		}
// 	} else {

// 	}
// }

// fn fill_unattainable_row(board: Board, row: usize) -> Board {
// 	let mut i: usize = 0;
	
// 	board
// 		.iter()
// 		.map(|r| match i {
// 			row => {
// 				i += 1;
				
// 				r
// 					.iter()
// 					.map(|c| match c {
// 						&Square::Empty => Square::Unattainable,
// 						_ => *c,
// 					})
// 			},
// 			_ => r,
// 		}).collect();


// 	// board[r].iter_mut().map(|x| match x {
// 	// 		&mut Square::Empty => Square::Unattainable,
// 	// 		_ => *x,
// 	// 	});

// 	board
// }

// fn fill_unattainable_column(board: [[Square; 8]; 8], c: usize) -> [[Square; 8]; 8] {
// 	let mut board = board.clone();
// 	for i in 0..board.len() {
// 		match board[i][c] {
// 			Square::Empty => board[i][c] = Square::Unattainable,
// 			_ => {}
// 		}
// 	}

// 	board
// }

// fn fill_unattainable_diagonal_bottom_top(board: [[Square; 8]; 8], r: usize, c: usize) -> [[Square; 8]; 8] {
// 	let mut board = board.clone();
	
// 	let start_row = r.clone();
// 	let start_column = c.clone();
// 	let board_len = board.len();

// 	let mut current_column = &start_column - 0;

// 	// for i in start_row..board_len {
// 	// 	if (i == start_row) {
// 	// 		// ignore
// 	// 	} else {
// 	// 		if (current_column >= 0) {
// 	// 			match board[i][current_column] {
// 	// 				Square::Empty => board[i][current_column] = Square::Unattainable,
// 	// 				_ => {}
// 	// 			}
// 	// 		}
// 	// 	}

// 	// 	current_column -= 1;
// 	// }

// 	current_column = &start_column + 0;

// 	for i in (0..start_row).rev() {
// 		if (i == start_row) {
// 			// ignore
// 		} else {
// 			if (current_column <= board_len) {
// 				match board[i][current_column] {
// 					Square::Empty => board[i][current_column] = Square::Unattainable,
// 					_ => {}
// 				}
// 			}
// 		}

// 		current_column += 1;
// 	}

// 	board
// }

// fn fill_unattainable_diagonal_top_bottom(board: [[Square; 8]; 8], r: usize, c: usize) -> [[Square; 8]; 8] {
// 	let mut board = board.clone();
	
// 	let start_row = r.clone();
// 	let start_column = c.clone();
// 	let board_len = board.len();
	
// 	let mut current_column = &start_column + 1;

// 	for i in start_row..board_len {
// 		if (i == start_row || current_column > board_len) {
// 			// ignore
// 		} else {
// 			// match board[i][current_column] {
// 			// 	Square::Empty => board[i][current_column] = Square::Unattainable,
// 			// 	_ => {}
// 			// };

// 			// current_column += 1;
// 		}
// 	}

// 	current_column = &start_column - 1;

// 	for i in (0..start_row).rev() {
// 		if (i == start_row || current_column < 0) {
// 			// ignore
// 		} else {
// 			// match board[i][current_column] {
// 			// 	Square::Empty => board[i][current_column] = Square::Unattainable,
// 			// 	_ => {}
// 			// };

// 			// if (current_column != 0) {
// 			// 	current_column -= 1;
// 			// }
// 		}
// 	}

// 	board
// }
