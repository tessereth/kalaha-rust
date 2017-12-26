use std::fmt;

mod error;
mod player;
mod pool;
mod turn;
mod board;
use error::Error;
use player::Player;
use turn::{Turn, GameResult};
use board::Board;

pub mod ai;

const INIT_COUNT: u32 = 6;
const PONDS_PER_PLAYER: usize = 6;
const TOTAL_POOLS: usize = 14;

#[derive(Debug)]
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
