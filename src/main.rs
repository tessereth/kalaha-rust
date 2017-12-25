extern crate kalaha;

fn main() {
    let mut board = kalaha::Kalaha::new();
    println!("{}", board.to_string());
    board.play(0).expect("Failed to play");
    println!("{}", board.to_string());
    board.play(1).expect("Failed to play");
    println!("{}", board.to_string());
}
