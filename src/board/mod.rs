use std::fmt;

use super::error::Error;
use super::player::Player;
use super::pool::{Pool, Bank};
use super::turn::Turn;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Board {
    pools: [Pool; super::TOTAL_POOLS]
}

impl Board {
    pub fn new() -> Board {
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

    pub fn valid_move(&self, player: &Player, pond: usize) -> Result<(), Error> {
        if pond >= super::PONDS_PER_PLAYER {
            Err(Error::InvalidIndex)
        } else if self.pools[self.pool_idx(player, pond)].count() == 0 {
            Err(Error::EmptyPool)
        } else {
            Ok(())
        }
    }

    // panics if pond is not a valid_move
    pub fn choose(&mut self, player: &Player, pond: usize) -> Turn {
        self.valid_move(player, pond).expect("Invalid move");
        let mut idx = self.pool_idx(&player, pond);
        let mut count = self.pools[idx].take();
        while count > 0 {
            idx = (idx + 1) % super::TOTAL_POOLS;
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("        Player B\n")?;
        f.write_str(&format!("   {:2} {:2} {:2} {:2} {:2} {:2}\n",
                             self.pools[12].count(),
                             self.pools[11].count(),
                             self.pools[10].count(),
                             self.pools[9].count(),
                             self.pools[8].count(),
                             self.pools[7].count(),
        ))?;
        f.write_str(&format!("{:2}                   {:2}\n",
                             self.pools[13].count(),
                             self.pools[6].count(),
        ))?;
        f.write_str(&format!("   {:2} {:2} {:2} {:2} {:2} {:2}\n",
                             self.pools[0].count(),
                             self.pools[1].count(),
                             self.pools[2].count(),
                             self.pools[3].count(),
                             self.pools[4].count(),
                             self.pools[5].count(),
        ))?;
        f.write_str("        Player A\n")
    }
}
