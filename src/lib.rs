use std::fmt;
use std::cmp::Ordering;

pub mod ai;

const INIT_COUNT: u32 = 6;
const PONDS_PER_PLAYER: usize = 6;
const TOTAL_POOLS: usize = 14;

#[derive(Debug)]
pub enum Error {
    EmptyPool,
    InvalidIndex,
    GameFinished,
    NotImplemented,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Player {
    A,
    B,
}

#[derive(Debug)]
struct Pond {
    player: Player,
    count: u32,
}

#[derive(Debug)]
struct Bank {
    player: Player,
    count: u32,
}

#[derive(Debug)]
enum Pool {
    Pond(Pond),
    Bank(Bank),
}

#[derive(Debug)]
struct Board {
    pools: [Pool; TOTAL_POOLS]
}

#[derive(Debug)]
pub enum GameResult {
    Winner { player: Player, score_a: u32, score_b: u32 },
    Draw { score: u32 },
}

#[derive(Debug)]
pub enum Turn {
    Player(Player),
    Finished(GameResult),
}

#[derive(Debug)]
pub struct Kalaha {
    board: Board,
    turn: Turn,
}

impl Player {
    fn next(&self) -> Player {
        match *self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }
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

impl Pond {
    fn take(&mut self) -> u32 {
        let count = self.count;
        self.count = 0;
        count
    }
}

impl Pool {
    fn new_pond(player: Player) -> Pool {
        Pool::Pond(Pond { player, count: INIT_COUNT })
    }

    fn new_bank(player: Player) -> Pool {
        Pool::Bank(Bank { player, count: 0 })
    }

    fn count(&self) -> u32 {
        match *self {
            Pool::Pond(ref x) => x.count,
            Pool::Bank(ref x) => x.count,
        }
    }

    fn take(&mut self) -> u32 {
        match *self {
            Pool::Pond(ref mut pond) => {
                pond.take()
            },
            Pool::Bank(_) => {
                panic!("Cannot take from a bank")
            },
        }
    }
}

impl Board {
    fn new() -> Board {
        let pools = [
            Pool::new_pond(Player::A),
            Pool::new_pond(Player::A),
            Pool::new_pond(Player::A),
            Pool::new_pond(Player::A),
            Pool::new_pond(Player::A),
            Pool::new_pond(Player::A),
            Pool::new_bank(Player::A),
            Pool::new_pond(Player::B),
            Pool::new_pond(Player::B),
            Pool::new_pond(Player::B),
            Pool::new_pond(Player::B),
            Pool::new_pond(Player::B),
            Pool::new_pond(Player::B),
            Pool::new_bank(Player::B),
        ];
        Board { pools }
    }

    fn pool_idx(&self, player: &Player, pond: usize) -> usize {
        match *player {
            Player::A => pond,
            Player::B => pond + 7,
        }
    }

    fn opposite_idx(&self, pond: usize) -> usize {
        // trust me, it is
        12 - pond
    }

    fn bank_idx(&self, player: &Player) -> usize {
        match *player {
            Player::A => 6,
            Player::B => 13,
        }
    }

    fn bank(&self, player: &Player) -> &Bank {
        match self.pools[self.bank_idx(player)] {
            Pool::Bank(ref bank) => bank,
            _ => panic!("Not a bank")
        }
    }

    fn bank_mut(&mut self, player: &Player) -> &mut Bank {
        match self.pools[self.bank_idx(player)] {
            Pool::Bank(ref mut bank) => bank,
            _ => panic!("Not a bank")
        }
    }

    fn valid_move(&self, player: &Player, pond: usize) -> Result<(), Error> {
        if pond >= PONDS_PER_PLAYER {
            Err(Error::InvalidIndex)
        } else if self.pools[self.pool_idx(player, pond)].count() == 0 {
            Err(Error::EmptyPool)
        } else {
            Ok(())
        }
    }

    // panics if pond is not a valid_move
    fn choose(&mut self, player: &Player, pond: usize) -> Turn {
        self.valid_move(player, pond).expect("Invalid move");
        let mut idx = self.pool_idx(&player, pond);
        let mut count = self.pools[idx].take();
        while count > 0 {
            idx = (idx + 1) % TOTAL_POOLS;
            match self.pools[idx] {
                Pool::Pond(ref mut pond) => pond.count += 1,
                Pool::Bank(ref mut bank) => if bank.player == *player {
                    bank.count += 1;
                } else {
                    continue;
                }
            };
            count -= 1;
        }
        let capture = match self.pools[idx] {
            Pool::Pond(ref mut pond) => {
                // if we ended on our side if the board, in an empty space
                if pond.player == *player && pond.count == 1 {
                    pond.take()
                } else {
                    0
                }
            },
            Pool::Bank(_) => 0
        };
        if capture > 0 {
            self.bank_mut(player).count += capture + self.pools[self.opposite_idx(idx)].take();
        }
        let finished = self.handle_finish();
        self.next_turn(player, idx, finished)
    }

    fn handle_finish(&mut self) -> bool {
        if self.pools[0..6].iter().fold(0, |count, pool| count + pool.count()) == 0 {
            let mut add_to_bank = 0;
            for pool in &mut self.pools[7..13] {
                add_to_bank += pool.take();
            };
            if add_to_bank > 0 {
                self.bank_mut(&Player::B).count += add_to_bank;
            }
            true
        } else if self.pools[7..13].iter().fold(0, |count, pool| count + pool.count()) == 0 {
            let mut add_to_bank = 0;
            for pool in &mut self.pools[0..6] {
                add_to_bank += pool.take();
            };
            if add_to_bank > 0 {
                self.bank_mut(&Player::A).count += add_to_bank;
            }
            true
        } else {
            false
        }
    }

    fn next_turn(&self, current_player: &Player, last_idx: usize, finished: bool) -> Turn {
        if finished {
            let a_count = self.bank(&Player::A).count;
            let b_count = self.bank(&Player::B).count;
            Turn::new_finished(a_count, b_count)
        } else {
            let next_player = match self.pools[last_idx] {
                Pool::Pond(_) => current_player.next(),
                Pool::Bank(_) => current_player.clone(),
            };
            Turn::Player(next_player)
        }
    }
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
            self.choose(choice);
            if verbose {
                println!("{}", self);
            }
        }
        self.turn.game_result()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut s = String::new();
        s.push_str("        Player B\n");
        s.push_str(&format!("   {:2} {:2} {:2} {:2} {:2} {:2}\n",
                            self.pools[12].count(),
                            self.pools[11].count(),
                            self.pools[10].count(),
                            self.pools[9].count(),
                            self.pools[8].count(),
                            self.pools[7].count(),
        ));
        s.push_str(&format!("{:2}                   {:2}\n",
                            self.pools[13].count(),
                            self.pools[6].count(),
        ));
        s.push_str(&format!("   {:2} {:2} {:2} {:2} {:2} {:2}\n",
                            self.pools[0].count(),
                            self.pools[1].count(),
                            self.pools[2].count(),
                            self.pools[3].count(),
                            self.pools[4].count(),
                            self.pools[5].count(),
        ));
        s.push_str("        Player A\n");
        f.write_str(&s)
    }
}

impl fmt::Display for Kalaha {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut s = self.board.to_string();
        s.push_str(&format!("Next turn: {:?}\n", self.turn));
        f.write_str(&s)
    }
}
