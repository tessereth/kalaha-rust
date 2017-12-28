extern crate kalaha;

use kalaha::Kalaha;
use kalaha::ai;

fn main() {
    let mut game = Kalaha::new();

    let ai_a = ai::LastValid {};
    let ai_b = ai::MinMax::new(7);

    game.play(&ai_a, &ai_b, true);

    println!("{}", game.game_result());
}
