use super::*;
use pool::{Pond, Bank};
use turn::GameResult;

fn board_from_counts(counts: &[u32]) -> Board {
    Board {
        pools: [
            Pool::Pond(Pond { player: Player::A, count: counts[0] }),
            Pool::Pond(Pond { player: Player::A, count: counts[1] }),
            Pool::Pond(Pond { player: Player::A, count: counts[2] }),
            Pool::Pond(Pond { player: Player::A, count: counts[3] }),
            Pool::Pond(Pond { player: Player::A, count: counts[4] }),
            Pool::Pond(Pond { player: Player::A, count: counts[5] }),
            Pool::Bank(Bank { player: Player::A, count: counts[6] }),
            Pool::Pond(Pond { player: Player::B, count: counts[7] }),
            Pool::Pond(Pond { player: Player::B, count: counts[8] }),
            Pool::Pond(Pond { player: Player::B, count: counts[9] }),
            Pool::Pond(Pond { player: Player::B, count: counts[10] }),
            Pool::Pond(Pond { player: Player::B, count: counts[11] }),
            Pool::Pond(Pond { player: Player::B, count: counts[12] }),
            Pool::Bank(Bank { player: Player::B, count: counts[13] }),
        ]
    }
}

#[test]
fn new_banks_start_empty() {
    let board = Board::new();
    assert_eq!(board.pools[board.bank_idx(&Player::A)].count(), 0);
    assert_eq!(board.pools[board.bank_idx(&Player::B)].count(), 0);
}

#[test]
fn new_pools_start_full() {
    let board = Board::new();
    for ref pool in board.pools[0..6].iter() {
        assert_eq!(pool.count(), ::INIT_COUNT);
    }
    for ref pool in board.pools[7..13].iter() {
        assert_eq!(pool.count(), ::INIT_COUNT);
    }
}

#[test]
fn valid_move_too_big() {
    assert_eq!(Board::new().valid_move(&Player::A, 6), Err(Error::InvalidIndex));
}

#[test]
fn valid_move_pond_empty() {
    let board = board_from_counts(&[0,0,0,0,0,3,18,6,2,0,1,0,0,42]);
    assert_eq!(board.valid_move(&Player::A, 0), Err(Error::EmptyPool));
}

#[test]
fn choose_normal() {
    let mut board = Board::new();
    assert_eq!(board.choose(&Player::A, 1), Turn::Player(Player::B));
    assert_eq!(
        board,
        board_from_counts(&[6,0,7,7,7,7,1,7,6,6,6,6,6,0])
    );
}

#[test]
fn choose_go_again() {
    let mut board = Board::new();
    assert_eq!(board.choose(&Player::A, 0), Turn::Player(Player::A));
    assert_eq!(
        board,
        board_from_counts(&[0,7,7,7,7,7,1,6,6,6,6,6,6,0])
    );
}

#[test]
fn choose_capture() {
    let mut board = board_from_counts(&[0,0,0,0,1,8,0,1,1,1,1,1,1,0]);
    assert_eq!(board.choose(&Player::A, 5), Turn::Player(Player::B));
    assert_eq!(
        board,
        board_from_counts(&[0,0,0,0,1,0,4,2,2,2,2,2,0,0])
    );
}

#[test]
fn choose_finish_a_cleared() {
    let mut board = board_from_counts(&[0,0,0,0,0,3,18,6,2,0,1,0,0,42]);
    assert_eq!(
        board.choose(&Player::A, 5),
        Turn::Finished( GameResult::Winner { player: Player::B, score_a: 19, score_b: 53 })
    );
    assert_eq!(
        board,
        board_from_counts(&[0,0,0,0,0,0,19,0,0,0,0,0,0,53])
    );
}

#[test]
fn choose_finish_b_cleared() {
    let mut board = board_from_counts(&[6,2,0,1,0,0,18,0,0,0,0,0,3,42]);
    assert_eq!(
        board.choose(&Player::B, 5),
        Turn::Finished( GameResult::Winner { player: Player::B, score_a: 29, score_b: 43 })
    );
    assert_eq!(
        board,
        board_from_counts(&[0,0,0,0,0,0,29,0,0,0,0,0,0,43])
    );
}

#[test]
#[should_panic(expected = "Invalid move")]
fn choose_invalid() {
    let mut board = board_from_counts(&[0,0,0,0,0,3,18,6,2,0,1,0,0,42]);
    board.choose(&Player::A, 0);
}
