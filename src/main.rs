extern crate kalaha;

use kalaha::Kalaha;
use kalaha::ai;

fn main() {
    let mut game = Kalaha::new();
    println!("{}", game);
    game.choose(0);
    println!("{}", game);
    println!("Valid move 0: {:?}", game.valid_move(0));
    println!("Valid move 1: {:?}", game.valid_move(1));
    game.choose(1);
    println!("{}", game);
    game.choose(1);
    println!("{}", game);

    let ai_a = ai::FirstValid {};
    let ai_b = ai::LastValid {};

    game.play(&ai_a, &ai_b, true);
}
