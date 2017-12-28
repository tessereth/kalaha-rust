extern crate kalaha;

use kalaha::Kalaha;
use kalaha::ai;

fn main() {
    let ai_one = ai::MinMax::new(6);
    let ai_two = ai::AlphaBeta::new(9);

    println!("Player A {:?} vs Player B {:?}", ai_one, ai_two);
    let mut game = Kalaha::new();
    game.play(&ai_one, &ai_two, false);
    println!("{}", game.game_result());

    println!("Player A {:?} vs Player B {:?}", ai_two, ai_one);
    let mut game = Kalaha::new();
    game.play(&ai_two, &ai_one, false);
    println!("{}", game.game_result());
}
