use std::fmt;
use std::cmp::Ordering;

use super::player::Player;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Winner { player: Player, score_a: u32, score_b: u32 },
    Draw { score: u32 },
}

#[derive(Debug, PartialEq)]
pub enum Turn {
    Player(Player),
    Finished(GameResult),
}

impl Turn {
    pub fn new_finished(score_a: u32, score_b: u32) -> Turn {
        let game_result = match score_a.cmp(&score_b) {
            Ordering::Greater => GameResult::Winner { player: Player::A, score_a, score_b },
            Ordering::Less => GameResult::Winner { player: Player::B, score_a, score_b },
            Ordering::Equal => GameResult::Draw { score: score_a },
        };
        Turn::Finished(game_result)
    }

    pub fn is_finished(&self) -> bool {
        match *self {
            Turn::Player(_) => false,
            Turn::Finished(_) => true,
        }
    }

    pub fn player(&self) -> &Player {
        match *self {
            Turn::Player(ref player) => player,
            Turn::Finished(_) => panic!("Game has finished"),
        }
    }

    pub fn game_result(&self) -> &GameResult {
        match *self {
            Turn::Player(_) => panic!("Game has not finished"),
            Turn::Finished(ref game_result) => game_result,
        }
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            GameResult::Winner {player: Player::A, score_a, score_b} =>
                f.write_str(&format!("Player A wins {} to {}", score_a, score_b)),
            GameResult::Winner {player: Player::B, score_a, score_b} =>
                f.write_str(&format!("Player B wins {} to {}", score_b, score_a)),
            GameResult::Draw { score } =>
                f.write_str(&format!("Draw {} to {}", score, score)),
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Turn::Player(ref player) => f.write_str(&format!("Next turn: {}", player)),
            Turn::Finished(ref game_result) =>
                f.write_str(&format!("Game finished: {}", game_result)),
        }
    }
}

