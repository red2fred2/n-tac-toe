/// How big of a board to make. Makes an NxN tic tac toe board
/// Does weird stuff when N = 0
const N: usize = 3;

/// A board is a row major array of characters
type Board = [[char; N]; N];

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

fn main() {
	// Set blank board
	let mut board = [[' '; N]; N];
	board[1][2] = 'X';
	// display_board(&board);
	let blanks = count_blanks(&board);

	println!("Blanks: {blanks}");
}
