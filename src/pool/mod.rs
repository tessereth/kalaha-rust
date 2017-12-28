use super::player::Player;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct Pond {
    pub player: Player,
    pub count: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bank {
    pub player: Player,
    pub count: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pool {
    Pond(Pond),
    Bank(Bank),
}

impl Pond {
    pub fn take(&mut self) -> u32 {
        let count = self.count;
        self.count = 0;
        count
    }
}

impl Pool {
    pub fn new_pond(player: Player) -> Pool {
        Pool::Pond(Pond { player, count: super::INIT_COUNT })
    }

    pub fn new_bank(player: Player) -> Pool {
        Pool::Bank(Bank { player, count: 0 })
    }

    pub fn count(&self) -> u32 {
        match *self {
            Pool::Pond(ref x) => x.count,
            Pool::Bank(ref x) => x.count,
        }
    }

    pub fn take(&mut self) -> u32 {
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

