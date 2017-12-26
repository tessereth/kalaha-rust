use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn next(&self) -> Player {
        match *self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Player::A => f.write_str("Player A"),
            Player::B => f.write_str("Player B"),
        }
    }
}
