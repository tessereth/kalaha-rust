use super::Kalaha;
use super::player::Player;

pub trait AI {
    fn choose(&self, game: &Kalaha) -> usize;
}

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

pub struct MinMax {
    depth: u32,
}

#[derive(Debug, Copy, Clone)]
struct ScoredMove {
    pond: usize,
    score: i32,
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
                if depth == 0 || game2.is_finished() {
                    scores.push(ScoredMove::new(i, self.score(&game2, player)));
                } else {
                    scores.push(ScoredMove::new(i, self.choose_depth(&game2, player, depth - 1).score));
                }
            }
        }
        if player == game.current_player() {
            *scores.iter().max_by_key(|x| x.score).expect("No valid moves")
        } else {
            *scores.iter().min_by_key(|x| x.score).expect("No valid moves")
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
