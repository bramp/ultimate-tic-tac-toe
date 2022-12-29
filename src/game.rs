use crate::error::MyError;
use crate::error::MyError::*;
use crate::Board;
use crate::MegaBoard;
use crate::PossibleMoves;
use crate::Square;
use crate::Winner;
use core::fmt::Formatter;
use core::ops::Index;
use rand::RngCore;
use std::fmt::Display;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct Game {
    board: MegaBoard,

    /// The player's who turn it is.
    current_player: Square,

    /// The current board being played on.
    current_board: Option<usize>,

    /// The current turn.
    turns: usize,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Default::default(),
            current_player: Square::O,
            current_board: None,
            turns: 0,
        }
    }
}

impl PossibleMoves for Game {
    fn playable(&self) -> bool {
        self.board.playable()
    }

    fn choose<R: RngCore>(&self, r: &mut R) -> Option<usize> {
        // If we are playing in a current board chose that
        // otherwise pick a random one.
        match self.current_board {
            Some(board) => Some(board),
            None => self.board.choose(r),
        }
    }
}

impl Index<usize> for Game {
    type Output = Board;

    fn index(&self, pos: usize) -> &Self::Output {
        self.board.index(pos)
    }
}

impl Winner for Game {
    fn winner(&self) -> Square {
        self.board.winner()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.board.fmt(f)?;
        writeln!(f, "{}'s turn", self.current_player)
    }
}

impl Game {
    /// Play a move on board at pos. Returns true if the game was ended.
    pub fn play(&mut self, board_pos: usize, pos: usize) -> Result<bool, MyError> {
        if let Some(current_board) = self.current_board {
            if current_board != board_pos {
                return Err(WrongBoard);
            }
        };

        // Play the turn
        if self.board.play(board_pos, pos, self.current_player)? {
            // and the game is won! Congrats to `self.turn`.
            return Ok(true);
        }

        // No one won, so jump to the next board
        self.current_board = if self.board[pos].playable() {
            Some(pos)
        } else {
            None
        };

        // Switch turn
        self.current_player = if self.current_player == Square::O {
            Square::X
        } else {
            Square::O
        };
        self.turns += 1;

        Ok(false)
    }

    pub fn board(&self, board_pos: usize) -> Result<&Board, MyError> {
        if board_pos >= self.board.len() {
            return Err(InvalidBoard);
        }
        Ok(&self.board[board_pos])
    }

    pub fn square(&self, board_pos: usize, square_pos: usize) -> Result<Square, MyError> {
        if board_pos >= self.board.len() {
            return Err(InvalidBoard);
        }
        if board_pos >= self.board[board_pos].len() {
            return Err(InvalidSquare);
        }

        Ok(self.board[board_pos][square_pos])
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Game {
    /// Returns the player who's turn it is.
    pub fn current_player(&self) -> Square {
        self.current_player
    }

    /// Returns the board to play in. If None is returned it means any board.
    pub fn current_board(&self) -> Option<usize> {
        self.current_board
    }

    pub fn turns(&self) -> usize {
        self.turns
    }
}

/// Extra methods only for the wasm version.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new_js() -> Game {
        Game::default()
    }

    /// Wrapper around [play] to return JsError.
    #[wasm_bindgen(js_name = play)]
    pub fn play_js(&mut self, board_pos: usize, square_pos: usize) -> Result<bool, JsError> {
        Ok(self.play(board_pos, square_pos)?)
    }

    /// Wrapper around [board] to return JsError.
    #[wasm_bindgen(js_name = board)]
    pub fn board_js(&self, board_pos: usize) -> Result<Board, JsError> {
        Ok(self.board(board_pos).cloned()?)
    }

    /// Wrapper around [square] to return JsError.
    #[wasm_bindgen(js_name = square)]
    pub fn square_js(&self, board_pos: usize, square_pos: usize) -> Result<Square, JsError> {
        Ok(self.square(board_pos, square_pos)?)
    }

    // This is here to map Traits to non Traits due to lack of
    // support yet: https://github.com/rustwasm/wasm-bindgen/issues/2073
    #[wasm_bindgen(js_name = playable)]
    pub fn playable_js(&self) -> bool {
        self.playable()
    }

    // This is here to map Traits to non Traits due to lack of
    // support yet: https://github.com/rustwasm/wasm-bindgen/issues/2073
    #[wasm_bindgen(js_name = winner)]
    pub fn winner_js(&self) -> Square {
        self.winner()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::MyError;
    use crate::Game;
    use crate::PossibleMoves;
    use crate::Square;
    use crate::Winner;

    /*
        /// Win the specific board in specific order.
        fn win_board(g: &mut Game, board_pos: usize, order: &[usize; 3]) {
            //  We assume the board is empty.
            assert!(g.board[board_pos].grid.iter().all(|x| x==&Square::None));
            let current_player = g.current_turn();

            assert_eq!(g.board[board_pos].winner(), Square::None);
            assert!(g.board[board_pos].playable());

            // ┌───────────┐
            // │ O │ X │   │
            // │───┼───┼───│
            // │ O │ X │   │
            // │───┼───┼───│
            // │ O │   │   │
            // └───────────┘
            g.play(board_pos, order[0]).unwrap(); // O
            g.play(board_pos, 1).unwrap(); // X - TODO pick random numbr not in order
            g.play(board_pos, order[1]).unwrap(); // O
            g.play(board_pos, 2).unwrap(); // X
            g.play(board_pos, order[2]).unwrap(); // O

            assert_eq!(g.board[board_pos].winner(), current_player);
            assert!(!g.board[board_pos].playable());

            assert!(
                g.play(board_pos, 0) == Err(MyError::AlreadyPlayed) ||
                g.play(board_pos, 0) == Err(MyError::AlreadyWon)
            );

            assert_eq!(
                g.play(board_pos, 3),
                Err(MyError::AlreadyWon));
        }
    */

    #[test]
    fn default() {
        let g = &mut Game::default();
        assert_eq!(g.winner(), Square::None);
        assert!(g.playable());
        assert_eq!(g.current_board(), None);
    }

    #[test]
    fn wrong_board() {
        let g = &mut Game::default();

        // First move anywhere
        g.play(0, 1).unwrap();

        // Second move should be on board 1.
        assert_eq!(g.current_board(), Some(1));
        assert_eq!(g.play(0, 0), Err(MyError::WrongBoard));
    }

    #[test]
    fn win_with_jump_to_already_won() {
        // Let's say X has won the bottom left board
        // if O wins the top left, ending in the bottom
        // left square. The next play should be anywhere else.
        let g = &mut Game::default();

        g.play(0, 0).unwrap(); // O
        g.play(0, 5).unwrap();
        g.play(5, 0).unwrap(); // O
        g.play(0, 2).unwrap();
        g.play(2, 0).unwrap(); // O
        g.play(0, 3).unwrap();
        g.play(3, 0).unwrap(); // O
        g.play(0, 4).unwrap(); // X wins 0 now jumps to 4
        g.play(4, 8).unwrap(); // O
        g.play(8, 7).unwrap();
        g.play(7, 8).unwrap(); // O
        g.play(8, 2).unwrap();
        g.play(2, 8).unwrap(); // O
        g.play(8, 4).unwrap();
        g.play(4, 4).unwrap(); // O
        g.play(4, 0).unwrap(); // X jumps to 0

        assert_eq!(g.current_board(), None);
    }
}
