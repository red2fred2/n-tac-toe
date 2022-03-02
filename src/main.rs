/// How big of a board to make. Makes an NxN tic tac toe board
/// Doesn't work when N = 0
const N: usize = 3;

/// A board is a row major array of characters
type Board = [[char; N]; N];

/// Returns whether an array of length N is all X or Os
/// 1 if X wins, -1 if O wins, 0 if neither
///
/// * `array` - the array to be checked
fn array_scores(array: &[char; N]) -> i8 {
	let first = array[0];
	match first {
		'X' => {
			for c in array {
				if c != &'X' {return 0}
			}
			1
		},
		'O' => {
			for c in array {
				if c != &'O' {return 0}
			}
			-1
		},
		_ => 0
	}
}

/// Returns the number of blank spaces left on this board
///
/// * `board` - the board to count from
fn count_blanks(board: &Board) -> usize {
	board.into_iter()
		.fold(0, |acc, row|
			acc + row.into_iter()
				.fold(0, |acc, space|
					if space == &' ' {acc + 1} else {acc}
				)
		)
}

/// Displays a board in the terminal
///
/// * `board` - the board to display
fn display_board(board: &Board) {
	// Row by row
	for i in 0..N {
		// Print the dividing line
		if i != 0 {
			let line = "-".repeat(N*4-2);
			println!("{line}");
		}

		// Print left column character
		let c = board[i][0];
		print!("{c}");

		// Run through the rest of the row
		for j in 1..N {
			let c = board[i][j];
			print!(" | {c}");
		}
		println!();
	}
}

/// Returns a column from a board
///
/// * `board` - the board to get a column from
/// * `col` - column number to get 0 <= col < N
fn get_col(board: &Board, col: usize) -> [char; N] {
	let mut array = ['*'; N];

	// Fill it with correct data
	for i in 0..N {
		array[i] = board[i][col];
	}

	array
}

/// Returns the ascending diagonal from a board
///
/// |   |   |   | X |
/// |   |   | X |   |
/// |   | X |   |   |
/// | X |   |   |   |
///
/// * `board` - the board to get this diagonal from
fn get_diag_ascending(board: &Board) -> [char; N] {
	let mut array = ['*'; N];

	// Fill it with correct data
	for i in 0..N {
		array[i] = board[N-i-1][i];
	}

	array
}

/// Returns the descending diagonal from a board
///
/// | X |   |   |   |
/// |   | X |   |   |
/// |   |   | X |   |
/// |   |   |   | X |
///
/// * `board` - the board to get this diagonal from
fn get_diag_descending(board: &Board) -> [char; N] {
	let mut array = ['*'; N];

	// Fill it with correct data
	for i in 0..N {
		array[i] = board[i][i];
	}

	array
}

/// Returns a vector of tuples like (row, col) for possible moves
///
/// * `board` - the board to check
fn get_possible_moves(board: &Board) -> Vec<(usize, usize)> {
	let mut moves = Vec::new();

	// Check for blank spots
	for r in 0..N {
		for c in 0..N {
			if board[r][c] == ' ' {
				moves.push((r, c));
			}
		}
	}

	moves
}

/// Checks if a board is in a game ending state,
///
/// returns (false, _) if not the end.
/// (true, 1) if X wins,
/// (true, -1) if O wins,
/// (true, 0) if it was a draw
///
/// * `board` - the board to check
fn utility(board: &Board) -> (bool, i8) {
	let num_blanks = count_blanks(board);

	// Check if enough moves have even been made to be terminal
	let blank_threshold = N * N - 2 * N + 1;
	if num_blanks > blank_threshold {
		return (false, 0)
	}

	// Check if there are no blank spaces
	if num_blanks == 0 {
		return (true, 0)
	}

	// Check rows
	for row in board {
		let score = array_scores(row);
		if score != 0 {
			return (true, score)
		}
	}

	// Check columns
	for col in 0..N {
		let score = array_scores(&get_col(board, col));
		if score != 0 {
			return (true, score)
		}
	}

	// Check diagonals
	let score = array_scores(&get_diag_descending(board));
	if score != 0 {
		return (true, score)
	}
	let score = array_scores(&get_diag_ascending(board));
	if score != 0 {
		return (true, score)
	}

	(false, 0)
}

fn main() {
	// Hit the eject button if N is 0
	// a 0x0 game of tic tac toe makes no sense
	if N == 0 {
		return
	}

	// Set blank board
	let mut board = [[' '; N]; N];
	board[2][0] = 'X';
	board[1][1] = 'X';
	board[0][2] = 'X';
	board[1][2] = 'O';
	board[2][1] = 'O';
	display_board(&board);
	let (is_terminal, score) = utility(&board);
	println!("Game end?: {is_terminal}, with a score: {score}");
}
