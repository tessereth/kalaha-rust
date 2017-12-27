use super::*;

#[test]
fn valid_move_valid() {
    assert_eq!(Kalaha::new().valid_move(0), Ok(()));
}

#[test]
fn valid_move_too_big() {
    assert_eq!(Kalaha::new().valid_move(6), Err(Error::InvalidIndex));
}

#[test]
fn valid_move_pond_empty() {
    let mut kalaha = Kalaha::new();
    kalaha.choose(0);
    assert_eq!(kalaha.valid_move(0), Err(Error::EmptyPool));
}

#[test]
fn valid_move_finished() {
    let kalaha = Kalaha {
        board: Board::new(),
        turn: Turn::Finished(GameResult::Draw { score: 36 })
    };
    assert_eq!(kalaha.valid_move(0), Err(Error::GameFinished));
}
