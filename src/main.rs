extern crate kalaha;

use kalaha::Kalaha;
use kalaha::ai;

fn main() {
    let mut game = Kalaha::new();

    let ai_a = ai::FirstValid {};
    let ai_b = ai::LastValid {};

    game.play(&ai_a, &ai_b, true);
}
