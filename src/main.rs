use crate::monte_carlo::MegaBoardStats;
use core::time::Duration;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::time::Instant;
use ultimate_tic_tac_toe::error::MyError;
use ultimate_tic_tac_toe::monte_carlo;
use ultimate_tic_tac_toe::Game;
use ultimate_tic_tac_toe::PossibleMoves;
use ultimate_tic_tac_toe::Square;
use ultimate_tic_tac_toe::Winner;

fn many_games() -> Result<(), MyError> {
    let r = &mut ChaCha8Rng::from_rng(rand::thread_rng()).unwrap();

    // TODO Consider not using a hashmap, or using fixed variables.
    let mut winners = HashMap::<Square, u32>::new();

    let duration = Duration::new(10, 0);
    let mut runs = 0;
    let mut now = Instant::now();
    let start = now;

    loop {
        let mut g = Game::default();

        while g.playable() {
            let mega_move = g.choose(r).unwrap();
            let board_move = g[mega_move].choose(r).unwrap();

            g.play(mega_move, board_move)?;

            //println!("{}", g);
        }

        //println!("Winner: {}", g.winner());
        *winners.entry(g.winner()).or_default() += 1;
        runs += 1;

        now = Instant::now();
        if (now - start) > duration {
            break;
        }
    }

    println!(
        "Games: {:?} ({}/s)",
        runs,
        runs as f64 / (now - start).as_secs() as f64
    );

    let o = *winners.entry(Square::O).or_default();
    let x = *winners.entry(Square::X).or_default();
    let draws = *winners.entry(Square::None).or_default();
    let total = (o + x + draws) as f64 / 100.0;

    println!("Winners: ");
    println!("    O: {} {:.1}% (goes first)", o, o as f64 / total);
    println!("    X: {} {:.1}% (goes second)", x, x as f64 / total);
    println!("Draws: {} {:.1}%", draws, draws as f64 / total);

    Ok(())
}

fn monte_carlo() -> Result<(), MyError> {
    let r = &mut ChaCha8Rng::from_rng(rand::thread_rng()).unwrap();

    let duration = Duration::new(10, 0);
    let mut now = Instant::now();
    let start = now;

    let mut stats = MegaBoardStats::default();

    let me = Square::O;
    let mut g = Game::default();
    g.play(4, 4)?;
    g.play(4, 0)?;
    g.play(0, 8)?;
    g.play(8, 1)?;
    g.play(1, 1)?;
    g.play(1, 0)?;

    loop {
        let mut g = g.clone(); // Reset
        assert_eq!(me, g.current_turn());
        assert!(g.playable());

        let mega_move = g.choose(r).unwrap();
        let board_move = g[mega_move].choose(r).unwrap();

        // Play this first move
        g.play(mega_move, board_move)?;

        // Now keep playing randomly
        while g.playable() {
            //println!("{}", g);
            let mega_move = g.choose(r).unwrap();
            let board_move = g[mega_move].choose(r).unwrap();

            g.play(mega_move, board_move)?;
        }

        let mut stat = &mut stats.board[mega_move][board_move];
        if g.winner() == me {
            stat.wins += 1;
        } else if g.winner() != Square::None {
            stat.loses += 1;
        }

        stat.totals += 1;
        stats.runs += 1;

        now = Instant::now();
        if (now - start) > duration {
            break;
        }
    }

    println!(
        "Games: {:?} ({}/s)",
        stats.runs,
        stats.runs as f64 / (now - start).as_secs() as f64
    );
    println!("{}", stats);

    Ok(())
}

fn main() {
    many_games().unwrap();
    //monte_carlo().unwrap();
}
