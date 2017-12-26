use super::*;

#[test]
fn banks_start_empty() {
    let board = Board::new();
    assert_eq!(board.pools[board.bank_idx(&Player::A)].count(), 0);
    assert_eq!(board.pools[board.bank_idx(&Player::B)].count(), 0);
}

#[test]
fn pools_start_full() {
    let board = Board::new();
    for ref pool in board.pools[0..6].iter() {
        assert_eq!(pool.count(), ::INIT_COUNT);
    }
    for ref pool in board.pools[7..13].iter() {
        assert_eq!(pool.count(), ::INIT_COUNT);
    }
}
