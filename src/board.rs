use crate::error::MyError;
use crate::error::MyError::*;
use crate::PossibleMoves;
use crate::Square;
use crate::Winner;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Index;
use rand::prelude::SliceRandom;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Board {
    #[wasm_bindgen(skip)]
    pub grid: [Square; 9], // TODO rename square

    /// Keep track of possible moves
    moves: Vec<usize>,

    /// Cache if there is a winner
    winner: Square,
}

impl Default for Board {
    fn default() -> Self {
        use Square::None;

        Board {
            grid: [
                None, None, None, //
                None, None, None, //
                None, None, None, //
            ],

            moves: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],

            winner: None,
        }
    }
}

impl From<[Square; 9]> for Board {
    fn from(grid: [Square; 9]) -> Self {
        Board {
            grid,

            moves: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],

            winner: Square::None,
        }
    }
}

impl Board {
    /// Plays at the specific position. Returns true if this board is over (won or drawn).
    pub fn play(&mut self, pos: usize, player: Square) -> Result<bool, MyError> {
        if pos >= self.grid.len() {
            return Err(InvalidSquare);
        }
        if self.grid[pos] != Square::None {
            return Err(AlreadyPlayed);
        }
        if self.has_winner() {
            return Err(AlreadyWon);
        }

        // Play
        self.grid[pos] = player;
        self.winner = self.check_winner();

        // Remove this possible move
        let x = self.moves.iter().position(|&x| x == pos).unwrap();
        self.moves.swap_remove(x);

        Ok(self.winner != Square::None || self.moves.is_empty())
    }

    /*
        pub fn undo(&mut self, pos: usize) {
            assert_ne!(self.grid[pos], Square::None);

            self.grid[pos] = Square::None;

            // We should have no winner now. We can skip the check_winner
            assert_eq!(self.check_winner(), Square::None);
            self.winner = Square::None;

            self.moves.push(pos);
        }
    */
}

impl Board {
    pub fn square(&self, square_pos: usize) -> Result<Square, MyError> {
        if square_pos >= self.grid.len() {
            return Err(InvalidSquare);
        }

        Ok(self.grid[square_pos])
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Board {
    pub fn len(&self) -> usize {
        self.grid.len()
    }
}

/// Extra methods only for the wasm version.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Board {
    // This is here to map Traits to non Traits due to lack of
    // support yet: https://github.com/rustwasm/wasm-bindgen/issues/2073
    #[wasm_bindgen(js_name = playable)]
    pub fn playable_js(&self) -> bool {
        self.playable()
    }

    /// Wrapper around [square] to return JsError.
    #[wasm_bindgen(js_name = square)]
    pub fn square_js(&self, square_pos: usize) -> Result<Square, JsError> {
        Ok(self.square(square_pos)?)
    }

    // This is here to map Traits to non Traits due to lack of
    // support yet: https://github.com/rustwasm/wasm-bindgen/issues/2073
    #[wasm_bindgen(js_name = winner)]
    pub fn winner_js(&self) -> Square {
        self.winner()
    }
}

impl PossibleMoves for Board {
    fn choose<R: rand::RngCore>(&self, rng: &mut R) -> Option<usize> {
        self.moves.choose(rng).copied()
    }

    fn playable(&self) -> bool {
        !self.moves.is_empty() && !self.has_winner()
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Square;

    fn index(&self, p: (usize, usize)) -> &Self::Output {
        self.grid.index(p.1 * 3 + p.0)
    }
}

impl Index<usize> for Board {
    type Output = Square;

    fn index(&self, pos: usize) -> &Self::Output {
        self.grid.index(pos)
    }
}

macro_rules! check {
    ($grid:expr, $a:literal, $b:literal, $c:literal) => {
        if $grid[$a] != Square::None && $grid[$a] == $grid[$b] && $grid[$a] == $grid[$c] {
            return $grid[$a];
        }
    };
}

impl Board {
    fn check_winner(&self) -> Square {
        // Rows
        check!(self.grid, 0, 1, 2);
        check!(self.grid, 3, 4, 5);
        check!(self.grid, 6, 7, 8);

        // Cols
        check!(self.grid, 0, 3, 6);
        check!(self.grid, 1, 4, 7);
        check!(self.grid, 2, 5, 8);

        // Diagonals
        check!(self.grid, 0, 4, 8);
        check!(self.grid, 2, 4, 6);

        Square::None
    }
}

impl Winner for Board {
    fn winner(&self) -> Square {
        self.winner
    }
}

impl Board {
    const DISPLAY_O: &str = "┌── ██████╗ ┐\n\
                             │ ██╔═══██╗ │\n\
                             │ ██║   ██║ │\n\
                             │ ██║   ██║ │\n\
                             │ ╚██████╔╝ │\n\
                             │  ╚═════╝  │\n\
                             └───────────┘\n";

    const DISPLAY_X: &str = "┌─ ██╗  ██╗ ┐\n\
                             │  ╚██╗██╔╝ │\n\
                             │   ╚███╔╝  │\n\
                             │   ██╔██╗  │\n\
                             │  ██╔╝ ██╗ │\n\
                             │  ╚═╝  ╚═╝ │\n\
                             └───────────┘\n";
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // ┌───────────┐
        // │ O │ X │   │
        // │───┼───┼───│
        // │   │ O │ X │
        // │───┼───┼───│
        // │ X │ X │ O │
        // └───────────┘

        match self.winner() {
            Square::O => write!(f, "{}", Board::DISPLAY_O)?,
            Square::X => write!(f, "{}", Board::DISPLAY_X)?,
            Square::None => {
                writeln!(f, "┌───────────┐")?;
                for row in 0..3 {
                    writeln!(
                        f,
                        "│ {} │ {} │ {} │",
                        self.grid[3 * row + 0],
                        self.grid[3 * row + 1],
                        self.grid[3 * row + 2]
                    )?;
                    if row < 2 {
                        writeln!(f, "│───┼───┼───│")?;
                    }
                }
                writeln!(f, "└───────────┘")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;
    use crate::PossibleMoves;
    use crate::Square;
    use crate::Square::None;
    use crate::Square::O;
    use crate::Square::X;
    use crate::Winner;

    #[test]
    fn win() {
        // ┌───────────┐
        // │ X │   │   │
        // │───┼───┼───│
        // │ X │   │   │
        // │───┼───┼───│
        // │ X │   │   │
        // └───────────┘
        let mut b = Board::default();
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        b.play(0, Square::X).unwrap();
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        b.play(4, Square::X).unwrap();
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        b.play(8, Square::X).unwrap();
        assert_eq!(b.winner(), Square::X);
        assert!(!b.playable());
    }

    #[test]
    fn draw() {
        // ┌───────────┐
        // │ X │ O │ X │
        // │───┼───┼───│
        // │ O │ X │ O │
        // │───┼───┼───│
        // │ O │ X │ O │
        // └───────────┘
        let mut b = Board::default();
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        for i in (0..6).step_by(2) {
            b.play(i, Square::X).unwrap();
            assert_eq!(b.winner(), Square::None);
            assert!(b.playable());

            b.play(i + 1, Square::O).unwrap();
            assert_eq!(b.winner(), Square::None);
            assert!(b.playable());
        }

        b.play(6, Square::O).unwrap();
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        b.play(7, Square::X).unwrap();
        assert_eq!(b.winner(), Square::None);
        assert!(b.playable());

        b.play(8, Square::O).unwrap();
        assert_eq!(b.winner(), Square::None);
        assert!(!b.playable());
    }

    #[test]
    fn display() {
        let b = Board {
            grid: [
                O, X, None, //
                None, O, X, //
                X, X, None, //
            ],
            ..Default::default()
        };

        assert_eq!(
            format!("{}", b),
            "┌───────────┐\n\
             │ O │ X │   │\n\
             │───┼───┼───│\n\
             │   │ O │ X │\n\
             │───┼───┼───│\n\
             │ X │ X │   │\n\
             └───────────┘\n\
"
            .trim_start()
        );
    }
}
