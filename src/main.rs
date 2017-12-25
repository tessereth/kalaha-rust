extern crate kalaha;

fn main() {
    let mut board = kalaha::Kalaha::new();
    println!("{}", board.to_string());
    board.choose(0).expect("Failed to play");
    println!("{}", board.to_string());
    println!("Valid move 0: {:?}", board.valid_move(0));
    println!("Valid move 1: {:?}", board.valid_move(1));
    board.choose(1).expect("Failed to play");
    println!("{}", board.to_string());
    board.choose(1).expect("Failed to play");
    println!("{}", board.to_string());
}
