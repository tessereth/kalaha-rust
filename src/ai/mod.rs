use std::fmt;
use super::Kalaha;
use super::player::Player;

#[cfg(test)]
mod tests;

pub trait AI: fmt::Debug {
    fn choose(&self, game: &Kalaha) -> usize;
}

#[derive(Debug)]
pub struct FirstValid {}

impl AI for FirstValid {
    fn choose(&self, game: &Kalaha) -> usize {
        for i in 0..super::PONDS_PER_PLAYER {
            if game.valid_move(i).is_ok() {
                return i;
            }
        }
        panic!("No valid moves");
    }
}

#[derive(Debug)]
pub struct LastValid {}

impl AI for LastValid {
    fn choose(&self, game: &Kalaha) -> usize {
        for i in (0..super::PONDS_PER_PLAYER).rev() {
            if game.valid_move(i).is_ok() {
                return i;
            }
        }
        panic!("No valid moves");
    }
}

#[derive(Debug)]
pub struct MinMax {
    depth: u32,
}

// sorts by score, then by pond
// note that choosing higher pond values will generally give a better result
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ScoredMove {
    score: i32,
    pond: usize,
}

impl ScoredMove {
    fn new(pond: usize, score: i32) -> ScoredMove {
        ScoredMove { pond, score }
    }
}

impl MinMax {
    pub fn new(depth: u32) -> MinMax {
        MinMax { depth }
    }

    fn choose_depth(&self, game: &Kalaha, player: &Player, depth: u32) -> ScoredMove {
        let mut scores = Vec::new();
        for i in 0..super::PONDS_PER_PLAYER {
            if game.valid_move(i).is_ok() {
                let mut game2 = game.clone();
                game2.choose(i);
                if depth <= 1 || game2.is_finished() {
                    scores.push(ScoredMove::new(i, self.score(&game2, player)));
                } else {
                    scores.push(ScoredMove::new(i, self.choose_depth(&game2, player, depth - 1).score));
                }
            }
        }
        if player == game.current_player() {
            *scores.iter().max().expect("No valid moves")
        } else {
            *scores.iter().min().expect("No valid moves")
        }
    }

    fn score(&self, game: &Kalaha, player: &Player) -> i32 {
        game.bank(player) as i32 - game.bank(&player.next()) as i32
    }
}

impl AI for MinMax {
    fn choose(&self, game: &Kalaha) -> usize {
        self.choose_depth(game, game.current_player(), self.depth).pond
    }
}

#[derive(Debug)]
pub struct AlphaBeta {
    depth: u32,
}

impl AlphaBeta {
    pub fn new(depth: u32) -> AlphaBeta {
        AlphaBeta { depth }
    }

    // See https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
    fn alpha_beta(&self, game: &Kalaha, player: &Player, depth: u32, alpha: i32, beta: i32) -> ScoredMove {
        let mut best_guess;
        let mut alpha = alpha;
        let mut beta = beta;
        if game.current_player() == player {
            best_guess = ScoredMove::new(6, i32::min_value());
            for (pond, game2) in self.all_children(game) {
                best_guess = best_guess.max(
                    ScoredMove::new(
                        pond,
                        self.score_for(&game2, player, pond, depth, alpha, beta).score
                    )
                );
                alpha = best_guess.score.max(alpha);
                if beta <= alpha {
                    break
                }
            }
        } else {
            best_guess = ScoredMove::new(6, i32::max_value());
            for (pond, game2) in self.all_children(game) {
                best_guess = best_guess.min(
                    ScoredMove::new(
                        pond,
                        self.score_for(&game2, player, pond, depth, alpha, beta).score
                    )
                );
                beta = best_guess.score.min(beta);
                if beta <= alpha {
                    break
                }
            }
        }
        best_guess
    }

    fn score_for(&self, game: &Kalaha, player: &Player, pond: usize, depth: u32, alpha: i32, beta: i32) -> ScoredMove {
        if depth <= 1 || game.is_finished() {
            ScoredMove::new(pond, self.score(game, player))
        } else {
            self.alpha_beta(game, player, depth - 1, alpha, beta)
        }
    }

    fn score(&self, game: &Kalaha, player: &Player) -> i32 {
        game.bank(player) as i32 - game.bank(&player.next()) as i32
    }

    fn all_children(&self, game: &Kalaha) -> Vec<(usize, Kalaha)> {
        let mut ret = Vec::new();
        // Reverse the order because generally the later ponds are a better choice
        // and we want to prune asap
        for i in (0..super::PONDS_PER_PLAYER).rev() {
            if game.valid_move(i).is_ok() {
                let mut game2 = game.clone();
                game2.choose(i);
                ret.push((i, game2));
            }
        }
        ret
    }
}

impl AI for AlphaBeta {
    fn choose(&self, game: &Kalaha) -> usize {
        self.alpha_beta(
            game, game.current_player(), self.depth, i32::min_value(), i32::max_value()
        ).pond
    }
}
