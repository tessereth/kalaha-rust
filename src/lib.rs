const INIT_COUNT: u32 = 6;
const NUM_POOLS: usize = 14;

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
    pools: [Pool; NUM_POOLS]
}

#[derive(Debug)]
pub enum GameResult {
    Winner(Player),
    Draw,
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
    pub fn is_finished(&self) -> bool {
        match *self {
            Turn::Player(_) => false,
            Turn::Finished(_) => true,
        }
    }
}

impl Pool {
    fn new_pond() -> Pool {
        Pool::Pond(Pond { count: INIT_COUNT })
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
                let count = pond.count;
                pond.count = 0;
                count
            },
            Pool::Bank(ref mut bank) => {
                let count = bank.count;
                bank.count = 0;
                count
            },
        }
    }
}

impl Board {
    fn new() -> Board {
        let pools = [
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_bank(Player::A),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
            Pool::new_pond(),
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

    fn valid_move(&self, player: &Player, pond: usize) -> Result<(), Error> {
        if pond >= NUM_POOLS {
            Err(Error::InvalidIndex)
        } else if self.pools[self.pool_idx(player, pond)].count() == 0 {
            Err(Error::EmptyPool)
        } else {
            Ok(())
        }
    }

    fn choose(&mut self, player: &Player, pond: usize) -> Result<Turn, Error> {
        let mut idx = self.pool_idx(&player, pond);
        let mut count = self.pools[idx].take();
        if count == 0 {
            return Err(Error::EmptyPool);
        }
        while count > 0 {
            idx = (idx + 1) % NUM_POOLS;
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
        // TODO: Handle capture
        // TODO: Handle game finish
        let next_player = match self.pools[idx] {
            Pool::Pond(_) => player.next(),
            Pool::Bank(_) => player.clone(),
        };
        Ok(Turn::Player(next_player))
    }

    fn to_string(&self) -> String {
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
        s
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

    pub fn choose(&mut self, pond: usize) -> Result<(), Error> {
        self.valid_move(pond)?;
        self.turn = {
            let player = match self.turn {
                Turn::Finished(_) => return Err(Error::GameFinished),
                Turn::Player(ref player) => player,
            };
            self.board.choose(player, pond)?
        };
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut s = self.board.to_string();
        s.push_str(&format!("Next turn: {:?}\n", self.turn));
        s
    }
}
