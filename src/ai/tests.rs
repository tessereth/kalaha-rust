use super::*;

fn assert_depth_eq(depth: u32) {
    let game = Kalaha::new();
    // Do comparison based on score rather than pond to allow the algorithms to choose different
    // ponds with the same score
    assert_eq!(
        MinMax { depth }.choose_depth(&game,game.current_player(), depth).score,
        AlphaBeta { depth }.alpha_beta(
            &game, game.current_player(), depth, i32::min_value(), i32::max_value()
        ).score
    );
}

#[test]
fn minmax_eq_alphabeta_1() {
    assert_depth_eq(1);
}

#[test]
fn minmax_eq_alphabeta_2() {
    assert_depth_eq(2);
}

#[test]
fn minmax_eq_alphabeta_3() {
    assert_depth_eq(3);
}

#[test]
fn minmax_eq_alphabeta_4() {
    assert_depth_eq(4);
}

#[test]
fn minmax_eq_alphabeta_5() {
    assert_depth_eq(5);
}
