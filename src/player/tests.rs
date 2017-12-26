use super::*;

#[test]
fn test_next() {
    assert_eq!(Player::A.next(), Player::B);
    assert_eq!(Player::B.next(), Player::A);
}
