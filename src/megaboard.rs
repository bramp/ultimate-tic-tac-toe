use crate::error::MyError;
use crate::error::MyError::*;
use crate::Board;
use crate::PossibleMoves;
use crate::Square;
use crate::Winner;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Index;
use core::ops::IndexMut;
use rand::prelude::SliceRandom;
use rand::RngCore;
use substring::Substring;

#[derive(Debug, Clone)]
pub struct MegaBoard {
    board: [Board; 9],

    moves: Vec<usize>,

    /// Cache if there is a winner
    winner: Square,
}

impl Default for MegaBoard {
    fn default() -> Self {
        MegaBoard {
            board: Default::default(),
            moves: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            winner: Square::None,
        }
    }
}

impl MegaBoard {
    pub fn len(&self) -> usize {
        self.board.len()
    }

    pub fn play(&mut self, mega_pos: usize, pos: usize, player: Square) -> Result<bool, MyError> {
        if mega_pos >= self.board.len() {
            return Err(InvalidBoard);
        }

        let b = self.board.index_mut(mega_pos);
        if b.play(pos, player)? {
            // Sub-grid has finished,
            // Check if the larger one has been won.
            self.winner = self.check_winner();

            // Remove
            let x = self.moves.iter().position(|&x| x == mega_pos).unwrap();
            self.moves.swap_remove(x);
        }

        return Ok(self.winner != Square::None || self.moves.is_empty());
    }
}

impl PossibleMoves for MegaBoard {
    fn choose<R>(&self, r: &mut R) -> Option<usize>
    where
        R: RngCore,
    {
        self.moves.choose(r).copied()
    }

    fn playable(&self) -> bool {
        !self.moves.is_empty() && !self.has_winner()
    }
}

impl Index<(usize, usize)> for MegaBoard {
    type Output = Board;

    fn index(&self, p: (usize, usize)) -> &Self::Output {
        self.board.index(p.1 * 3 + p.0)
    }
}

impl Index<usize> for MegaBoard {
    type Output = Board;

    fn index(&self, pos: usize) -> &Self::Output {
        self.board.index(pos)
    }
}

macro_rules! check {
    ($grid:expr, $a:literal, $b:literal, $c:literal) => {
        let a = $grid.index($a).winner();
        if a != Square::None {
            if a == $grid.index($b).winner() && a == $grid.index($c).winner() {
                return a;
            }
        }
    };
}

impl MegaBoard {
    fn check_winner(&self) -> Square {
        // Rows
        check!(self.board, 0, 1, 2);
        check!(self.board, 3, 4, 5);
        check!(self.board, 6, 7, 8);

        // Cols
        check!(self.board, 0, 3, 6);
        check!(self.board, 1, 4, 7);
        check!(self.board, 2, 5, 8);

        // Diagonals
        check!(self.board, 0, 4, 8);
        check!(self.board, 2, 4, 6);

        Square::None
    }
}

impl Winner for MegaBoard {
    fn winner(&self) -> Square {
        self.winner
    }
}

impl Display for MegaBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // ┌───────────┐
        // │ O │ X │   │
        // │───┼───┼───│
        // │   │ O │ X │
        // │───┼───┼───│
        // │ X │ X │ O │
        // └───────────┘

        // Dimensions of a single grid
        const WIDTH: usize = 14;
        const HEIGHT: usize = 7;

        let mut strings = Vec::with_capacity(3 * 3);
        for y in 0..3 {
            for x in 0..3 {
                let s = format!("{}", self.board[y * 3 + x]);
                assert_eq!(
                    s.chars().count(),
                    WIDTH * HEIGHT,
                    "The size of the grids has changed."
                );
                strings.push(s);
            }
        }

        // Now do a bitblt / 2d print
        for y in 0..3 {
            for row in 0..HEIGHT {
                for x in 0..3 {
                    let g = &strings[3 * y + x];

                    // Get a single row of text from this grid (minus the new line)
                    let s = g.substring(row * WIDTH, (row + 1) * WIDTH - 1);

                    // and print it.
                    write!(f, "{}", s)?;
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::MyError;
    use crate::MegaBoard;
    use crate::PossibleMoves;
    use crate::Square;
    use crate::Winner;

    /// Win the specific board.
    fn win_board(b: &mut MegaBoard, board_pos: usize, player: Square) {
        //  We assume the board is empty.
        assert!(b.board[board_pos].grid.iter().all(|x| x == &Square::None));

        // ┌───────────┐
        // │ O │ O │ O │
        // │───┼───┼───│
        // │   │   │   │
        // │───┼───┼───│
        // │   │   │   │
        // └───────────┘
        b.play(board_pos, 0, player).unwrap();

        assert_eq!(b.play(board_pos, 0, player), Err(MyError::AlreadyPlayed));

        b.play(board_pos, 1, player).unwrap();
        b.play(board_pos, 2, player).unwrap();

        assert_eq!(b.board[board_pos].winner(), player);
        assert!(!b.board[board_pos].playable());

        // Both are valid errors
        assert!(
            b.play(board_pos, 0, player) == Err(MyError::AlreadyPlayed)
                || b.play(board_pos, 0, player) == Err(MyError::AlreadyWon)
        );

        assert_eq!(b.play(board_pos, 3, player), Err(MyError::AlreadyWon));
    }

    #[test]
    fn win() {
        // ┌───────────┐
        // │ O │   │   │
        // │───┼───┼───│
        // │ O │   │   │
        // │───┼───┼───│
        // │ O │   │   │
        // └───────────┘
        let b = &mut MegaBoard::default();
        assert_eq!(b.winner(), Square::None);

        win_board(b, 0, Square::O);
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        win_board(b, 4, Square::O);
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        win_board(b, 8, Square::O);
        assert_eq!(b.winner(), Square::O);
        assert!(!b.playable());
    }
}
