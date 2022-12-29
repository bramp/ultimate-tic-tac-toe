use crate::monte_carlo::Stats;
use std::time::Duration;
use crate::ai::MonteCarloAI as WrappedMonteCarloAI;
use crate::ai::RandomAI as WrappedRandomAI;
use crate::ai::AI;
use crate::Game;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Pos {
    pub board_pos: usize,
    pub square_pos: usize,
}

impl From<(usize, usize)> for Pos {
    fn from(p: (usize, usize)) -> Self {
        Pos {
            board_pos: p.0,
            square_pos: p.1,
        }
    }
}

/// Implementation of the Random AI for wasm.
#[wasm_bindgen]
pub struct RandomAI {
    rng: ChaCha8Rng,
    ai: WrappedRandomAI,

    pub blah: usize,
}

#[wasm_bindgen]
impl RandomAI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RandomAI {
        RandomAI {
            rng: ChaCha8Rng::from_rng(rand::thread_rng()).unwrap(),
            ai: Default::default(),
            blah: 0,
        }
    }

    pub fn choose(&mut self, g: &Game) -> Pos {
        Pos::from(self.ai.choose(&mut self.rng, g))
    }
}

/// Implementation of the MonteCarlo AI for wasm.
#[wasm_bindgen]
pub struct MonteCarloAI {
    rng: ChaCha8Rng,
    ai: WrappedMonteCarloAI,
}

#[wasm_bindgen]
impl MonteCarloAI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MonteCarloAI {
        MonteCarloAI {
            rng: ChaCha8Rng::from_rng(rand::thread_rng()).unwrap(),
            ai: WrappedMonteCarloAI::new(Duration::new(1, 0)),
        }
    }

    pub fn choose(&mut self, g: &Game) -> Pos {
        self.ai.choose(&mut self.rng, g).into()
    }

    pub fn runs(&self) -> usize {
        self.ai.last_results.runs
    }

    pub fn stats(&self, board_pos: usize, square_pos: usize) -> Stats {
        self.ai.last_results.board[board_pos][square_pos]
    }

    pub fn best(&self) -> Pos{
        self.ai.last_results.best().into()
    }

    pub fn totals(&self) -> Stats {
        self.ai.last_results.totals()
    }
}
