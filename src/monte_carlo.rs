use core::fmt::Display;
use core::fmt::Formatter;
use std::fmt::Write;
use substring::Substring;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Stats {
    pub wins: usize,
    pub loses: usize,
    pub totals: usize,
}

#[derive(Debug, Default)]
pub struct MegaBoardStats {
    pub board: [[Stats; 9]; 9],
    pub runs: usize,
}

impl Stats {
    pub fn win_ratio(&self) -> f64 {
        if self.totals == 0 {
            return 0.0;
        }
        self.wins as f64 / self.totals as f64
    }

    pub fn lose_ratio(&self) -> f64 {
        if self.totals == 0 {
            return 0.0;
        }
        self.loses as f64 / self.totals as f64
    }

    pub fn draw_ratio(&self) -> f64 {
        if self.totals == 0 {
            return 0.0;
        }
        1.0 - (self.wins + self.loses) as f64 / self.totals as f64
    }
}

impl MegaBoardStats {
    pub fn best(&self) -> (usize, usize) {
        let ((board_pos, square_pos), _s) = self
            .board
            .iter()
            .enumerate()
            .flat_map(|(x, s)| s.iter().enumerate().map(move |(y, s)| ((x, y), s)))
            .map(|(p, a)| (p, a.win_ratio()))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            //.max_by(|(_, a), (_, b)| a.wins.cmp(&b.wins))
            .unwrap();

        (board_pos, square_pos)
    }

    pub fn totals(&self) -> Stats {
        self
            .board
            .iter()
            .flat_map(|s| s.iter())
            .copied()
            .reduce(|a, b| Stats{
                wins: a.wins + b.wins,
                loses: a.loses + b.loses,
                totals: a.totals + b.totals,
            })
            .unwrap()
    }

    fn fmt_board(&self, f: &mut String, board: usize) -> Result<(), std::fmt::Error> {
        // ┌───────────┐
        // │ O │ X │   │
        // │───┼───┼───│
        // │   │ O │ X │
        // │───┼───┼───│
        // │ X │ X │ O │
        // └───────────┘
        let board = &self.board[board];
        let runs = if self.runs == 0 {
            1f64
        } else {
            self.runs as f64
        };

        // Find best square
        let best = board.iter().max_by(|a, b| a.wins.cmp(&b.wins)).unwrap();

        // Ensure the value is only 3 wide
        let num_fmt = |stat: &Stats| {
            // Win Percentage (for the board)
            let i = stat.wins as f64 / runs * 100.0;

            // Win ratio (for that square)
            //let i = stat.wins as f64 / stat.loses as f64;

            let mut s = format!("{:0}", i);

            // Trim 0 from the front (e.g 0.1 becomes .1)
            s = s.trim_start_matches('0').to_string();

            // Now trim to 3 digits
            s = s.chars().take(3).collect();

            //if stat.wins == best.wins {
            //    s = "\x1b[7m".to_owned() + &s + "\x1b[0m"
            //}
            s
        };

        writeln!(f, "┌───────────┐")?;
        for row in 0..3 {
            writeln!(
                f,
                "│{:>3}│{:>3}│{:>3}│",
                num_fmt(&board[3 * row + 0]),
                num_fmt(&board[3 * row + 1]),
                num_fmt(&board[3 * row + 2]),
            )?;
            if row < 2 {
                writeln!(f, "│───┼───┼───│")?;
            }
        }
        writeln!(f, "└───────────┘")?;

        Ok(())
    }
}

impl Display for MegaBoardStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Dimensions of a single grid
        const WIDTH: usize = 14;
        const HEIGHT: usize = 7;

        let mut strings = Vec::with_capacity(3 * 3);
        for y in 0..3 {
            for x in 0..3 {
                let mut s = String::new();
                self.fmt_board(&mut s, y * 3 + x)?;

                assert_eq!(
                    s.chars().count(),
                    WIDTH * HEIGHT,
                    "The size of the grids has changed.\n{}",
                    s
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
    use crate::monte_carlo::Stats;

    #[test]
    fn test_stats() {
        let s = Stats::default();

        s.win_ratio();
        s.lose_ratio();
        s.draw_ratio();
    }
}
