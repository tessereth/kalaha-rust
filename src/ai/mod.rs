pub trait AI {
    fn choose(&self, game: &super::Kalaha) -> usize;
}

pub struct FirstValid {}

impl AI for FirstValid {
    fn choose(&self, game: &super::Kalaha) -> usize {
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
    fn choose(&self, game: &super::Kalaha) -> usize {
        for i in (0..super::PONDS_PER_PLAYER).rev() {
            if game.valid_move(i).is_ok() {
                return i;
            }
        }
        panic!("No valid moves");
    }
}
