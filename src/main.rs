use std::collections::HashMap;

/// How big of a board to make. Makes an NxN tic tac toe board
/// Doesn't work when N < 2
const N: usize = 6;

// Make player selection easier
#[allow(dead_code)]
const O: bool = false;
#[allow(dead_code)]
const X: bool = true;

/// A board is a row major array of characters
///
/// 'X' is an X move
/// 'O' is an O move
/// ' ' is a blank space
/// Anything else is undefined, the program is liable to freak out
type Board = [[char; N]; N];

/// A cache is a map of board layouts to score and best moves
type Cache = HashMap<Board, (Moves, i8)>;

/// Describes a list of moves
type Moves = Vec<(usize, usize)>;
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
#[allow(dead_code)]
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
fn get_possible_moves(board: &Board) -> Moves {
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

/// Does the best move at this point
///
/// * `board` - the board to do a min step at
/// * `cache` - the cached of boards
/// * `terminals` - the number of terminals checked
/// * `player` - true if X, false if O
/// Returns the best moves and score of that best move
fn minimax(
	board: &mut Board,
	cache: &mut Cache,
	terminals: &mut usize,
	player: bool,
	val_to_beat: Option<i8>
) -> (Moves, i8, Option<i8>) {
	// See if this is cached
	match cache.get(board) {
		Some((m, s)) => return (m.clone(), *s, val_to_beat),
		_ => {
			match utility(board) {
				// If terminal, pass along the score
				(true, s) => {
					let moves = Vec::new();

					*terminals += 1;

					// Add to cache as terminal
					// cache.insert(board.clone(), (Vec::new(), s));
					(moves, s, Some(s))
				},
				// Otherwise check another layer down
				_ => {
					let possible_moves = get_possible_moves(board);

					// Find the highest score of those moves
					// Start with an impossibly bad score
					let mut best_score = if player {i8::MIN} else {i8::MAX};
					let mut best_moves = Vec::new();
					let mut best_ab_val = val_to_beat;

					// Go through all possible moves
					for (r, c) in possible_moves {
						board[r][c] = if player {'X'} else {'O'};
						let (mut moves, score, ab_val) = minimax(board, cache, terminals, !player, val_to_beat);

						if best_ab_val == None {
							best_ab_val = ab_val;
						}

						// Set the max score if this is better
						if (player && score > best_score) ||
						   (!player && score < best_score) {
							best_score = score;
							moves.push((r,c));
							best_moves = moves;

							// Check if other moves should be pruned
							match (ab_val, best_ab_val) {
								(Some(a), Some(b)) => {
									if (player && a > b) ||
									   (!player && a < b) {
										return (best_moves, best_score, Some(best_score))
									}
								},
								_ => ()
							}
						}

						// Undo the move for next attempt
						board[r][c] = ' ';
					}

					// Cache this value
					cache.insert(board.clone(), (best_moves.clone(), best_score));

					(best_moves, best_score, Some(best_score))
				}
			}
		}
	}
}

/// Finds the board created by a list of moves
///
/// * `moves` - the moves to replay
#[allow(dead_code)]
fn replay_moves(moves: &Moves) -> Board {
	let mut board = [[' '; N]; N];
	let mut player_x = true;

	// Replay
	for (r, c) in moves {
		board[*r][*c] = if player_x {'X'} else {'O'};
		player_x = !player_x;
	}

	board
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

	let mut cache: Cache = HashMap::new();
	let mut terminals = 0;

	// Set blank board
	let mut board = [[' '; N]; N];

	// board[0][0] = 'X';
	// board[0][1] = 'X';
	// board[0][2] = 'X';
	// board[0][3] = 'O';

	// board[1][0] = 'O';
	// board[1][1] = 'X';
	// board[1][2] = 'X';
	// board[1][3] = 'O';

	// board[2][0] = 'O';
	// board[2][1] = 'X';
	// board[2][2] = 'X';
	// board[2][3] = 'O';

	// board[3][0] = 'O';
	// board[3][1] = 'O';
	// board[3][2] = 'O';
	// board[3][3] = 'O';

	display_board(&board);

	let (mut moves, score, _) = minimax(&mut board, &mut cache, &mut terminals, X, None);
	println!("Best score for max: {score}");

	moves.reverse();
	println!("Moves: {:?}", moves);
	let unique = cache.len();
	println!("Unique states: {unique}");
	println!("Terminals examined: {terminals}");
}
