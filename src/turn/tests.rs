use super::*;

#[test]
fn new_finished_a() {
    assert_eq!(
        Turn::new_finished(20, 10),
        Turn::Finished(GameResult::Winner { player: Player::A, score_a: 20, score_b: 10 })
    );
}

#[test]
fn new_finished_b() {
    assert_eq!(
        Turn::new_finished(20, 30),
        Turn::Finished(GameResult::Winner { player: Player::B, score_a: 20, score_b: 30 })
    );
}

#[test]
fn new_finished_draw() {
    assert_eq!(
        Turn::new_finished(20, 20),
        Turn::Finished(GameResult::Draw { score: 20 })
    );
}

#[test]
fn is_finished() {
    assert!(Turn::Finished(GameResult::Draw { score: 20 }).is_finished());
    assert!(!Turn::Player(Player::A).is_finished());
}

#[test]
fn player() {
    assert_eq!(*Turn::Player(Player::A).player(), Player::A);
}

#[test]
#[should_panic(expected = "Game has finished")]
fn player_panic() {
    Turn::Finished(GameResult::Draw { score: 20 }).player();
}

#[test]
fn game_result() {
    assert_eq!(
        *Turn::Finished(GameResult::Draw { score: 20 }).game_result(),
        GameResult::Draw { score: 20 }
    );
}

#[test]
#[should_panic(expected = "Game has not finished")]
fn game_result_panic() {
    Turn::Player(Player::A).game_result();
}
