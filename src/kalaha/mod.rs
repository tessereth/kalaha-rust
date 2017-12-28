use std::fmt;

use super::error::Error;
use super::player::Player;
use super::turn::{Turn, GameResult};
use super::board::Board;
use super::ai;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Kalaha {
    board: Board,
    turn: Turn,
}

impl Kalaha {
    pub fn new() -> Kalaha {
        Kalaha { board: Board::new(), turn: Turn::Player(Player::A) }
    }

    pub fn valid_move(&self, pond: usize) -> Result<(), Error> {
        match self.turn {
            Turn::Finished(_) => Err(Error::GameFinished),
            Turn::Player(ref player) => self.board.valid_move(player, pond)
        }
    }

    pub fn bank(&self, player: &Player) -> u32 {
        self.board.bank(player).count
    }

    pub fn ponds(&self, player: &Player) -> [u32; 6] {
        self.board.pond_counts(player)
    }

    pub fn current_player(&self) -> &Player {
        self.turn.player()
    }

    pub fn is_finished(&self) -> bool {
        self.turn.is_finished()
    }

    pub fn game_result(&self) -> &GameResult {
        self.turn.game_result()
    }

    // panics if pond is not a valid_move
    pub fn choose(&mut self, pond: usize) {
        self.valid_move(pond).expect("Invalid move");
        self.turn = self.board.choose(self.turn.player(), pond);
    }

    pub fn play<A, B>(&mut self, ai_player_a: &A, ai_player_b: &B, verbose: bool) -> &GameResult
        where A: ai::AI, B: ai::AI
    {
        if verbose {
            println!("{}", self);
        }
        while !self.turn.is_finished() {
            let choice = match *self.turn.player() {
                Player::A => ai_player_a.choose(&self),
                Player::B => ai_player_b.choose(&self),
            };
            if verbose {
                println!("Player chose pond {}\n", choice);
            }
            self.choose(choice);
            if verbose {
                println!("{}", self);
            }
        }
        self.turn.game_result()
    }
}

impl fmt::Display for Kalaha {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.board.to_string())?;
        f.write_str(&format!("{}\n", self.turn))
    }
}
