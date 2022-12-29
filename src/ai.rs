use instant::Instant; // Works on wasm instead of use std::time::Instant;
use crate::monte_carlo::MegaBoardStats;
use crate::Game;
use crate::PossibleMoves;
use crate::Square;
use crate::Winner;
use core::time::Duration;
use rand::RngCore;

pub trait AI {
    fn choose<R: RngCore>(&mut self, r: R, g: &Game) -> (usize, usize);
}

/// Picks the next move completely at random.
#[derive(Default)]
pub struct RandomAI {}

impl AI for RandomAI {
    fn choose<R: RngCore>(&mut self, mut r: R, g: &Game) -> (usize, usize) {
        let board_pos = g.choose(&mut r).unwrap();
        let square_pos = g[board_pos].choose(&mut r).unwrap();

        (board_pos, square_pos)
    }
}

// Picks the next move completely based on MonteCarlo simulation.
#[derive(Default)]
pub struct MonteCarloAI {
    time_limit: Duration,
    pub last_results: MegaBoardStats,
}

impl MonteCarloAI {
    pub fn new(time_limit: Duration) -> Self {
        MonteCarloAI {
            time_limit,

            ..Default::default()
        }
    }
}

impl AI for MonteCarloAI {
    // TODO Change return value to Result<>
    fn choose<R: RngCore>(&mut self, mut r: R, game: &Game) -> (usize, usize) {
        let mut stats = MegaBoardStats::default();

        let mut now = Instant::now();

        let start = now;
        let me = game.current_player();

        loop {
            let mut g = game.clone(); // Reset
            assert!(g.playable());

            // Pick a random next move.
            let board_pos = g.choose(&mut r).unwrap();
            let square_pos = g[board_pos].choose(&mut r).unwrap();

            // Play this first move
            g.play(board_pos, square_pos)
                .expect("valid first move play");

            // Now keep playing randomly
            while g.playable() {
                // TODO sample based on the paths that seem best
                let board_pos = g.choose(&mut r).unwrap();
                let square_pos = g[board_pos].choose(&mut r).unwrap();

                g.play(board_pos, square_pos).expect("valid play");
            }

            let mut stat = &mut stats.board[board_pos][square_pos];
            if g.winner() == me {
                stat.wins += 1;
            } else if g.winner() != Square::None {
                stat.loses += 1;
            }

            stat.totals += 1;
            stats.runs += 1;

            now = Instant::now();
            if (now - start) > self.time_limit {
                break;
            }
        }

        let best = stats.best();

        // Record the results
        self.last_results = stats;

        best
    }
}

#[cfg(test)]
mod tests {
    use crate::ai::MonteCarloAI;
    use crate::ai::AI;
    use crate::Game;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::time::Duration;

    #[test]
    fn test_ai() {
        let mut rng = ChaCha8Rng::from_rng(rand::thread_rng()).unwrap();

        let g = Game::default();
        let mut ai = MonteCarloAI::new(Duration::new(1, 0));

        ai.choose(&mut rng, &g);
    }
}
