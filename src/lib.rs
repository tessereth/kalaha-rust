const INIT_COUNT: u32 = 6;
const NUM_POOLS: usize = 14;

#[derive(Debug)]
pub enum Error {
    EmptyPool,
    NotImplemented,
}

#[derive(Debug, PartialEq)]
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

impl Turn {
    pub fn is_finished(&self) -> bool {
        match *self {
            Turn::Player(_) => false,
            Turn::Finished(_) => true,
        }
    }
}

impl Pool {
    fn new_pond(player: Player) -> Pool {
        Pool::Pond(Pond { player, count: INIT_COUNT })
    }

    fn new_bank(player: Player) -> Pool {
        Pool::Bank(Bank { player, count: INIT_COUNT })
    }

    fn count(&self) -> u32 {
        match *self {
            Pool::Pond(ref x) => x.count,
            Pool::Bank(ref x) => x.count,
        }
    }

    fn increment(&mut self) {
        match *self {
            Pool::Pond(ref mut x) => x.count += 1,
            Pool::Bank(ref mut x) => x.count += 1,
        };
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

//    fn valid_move(&self, player: Player, pond: usize) -> Result<(), Error> {
//        Err(Error::NotImplemented)
//    }

    fn play(&mut self, player: Player, pond: usize) -> Result<Turn, Error> {
        let mut idx = self.pool_idx(&player, pond);
        let mut count = self.pools[idx].take();
        if count == 0 {
            return Err(Error::EmptyPool);
        }
        while count > 0 {
            idx += 1 % NUM_POOLS;
            match self.pools[idx] {
                Pool::Pond(ref mut pond) => pond.count += 1,
                Pool::Bank(ref mut bank) => if bank.player == player {
                    bank.count += 1;
                } else {
                    continue;
                }
            };
            count -= 1;
        }
        Ok(Turn::Player(player))
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

    pub fn play(&mut self, player: Player, pond: usize) -> Result<(), Error> {
        self.turn = self.board.play(player, pond)?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        self.board.to_string()
    }
}
