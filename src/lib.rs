pub mod ai;
#[cfg(feature = "wasm")]
pub mod ai_wasm;
pub mod board;
pub mod error;
pub mod game;
pub mod megaboard;
pub mod monte_carlo;
pub mod square;

#[cfg(feature = "wasm")]
pub mod wasm;

extern crate cfg_if;

pub use board::*;
pub use game::*;
pub use megaboard::*;
pub use square::*;

pub trait Winner {
    fn winner(&self) -> square::Square;

    fn has_winner(&self) -> bool {
        self.winner() != square::Square::None
    }
}

pub trait PossibleMoves {
    /// Returns true if there are possible moves remaining.
    fn playable(&self) -> bool;

    /// Choose a random possible move, or return a None.
    fn choose<R: rand::RngCore>(&self, rng: &mut R) -> Option<usize>;
}
