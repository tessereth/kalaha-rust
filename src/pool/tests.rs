use super::*;

#[test]
fn pond_take() {
    let mut pond = Pond { player: Player::A, count: 4 };
    assert_eq!(pond.count, 4);
    assert_eq!(pond.take(), 4);
    assert_eq!(pond.count, 0);
}

#[test]
fn pool_new_pond() {
    let pool = Pool::new_pond(Player::A);
    assert_eq!(pool.count(), ::INIT_COUNT);
}

#[test]
fn pool_new_bank() {
    let pool = Pool::new_bank(Player::A);
    assert_eq!(pool.count(), 0);
}

#[test]
fn pool_take_pond() {
    let mut pool = Pool::Pond(Pond { player: Player::A, count: 4 });
    assert_eq!(pool.count(), 4);
    assert_eq!(pool.take(), 4);
    assert_eq!(pool.count(), 0);
}

#[test]
#[should_panic(expected = "Cannot take from a bank")]
fn pool_take_bank() {
    let mut pool = Pool::Bank(Bank { player: Player::A, count: 4 });
    pool.take();
}
