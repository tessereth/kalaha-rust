extern crate clap;
extern crate kalaha;

use clap::{Arg, App};
use kalaha::Kalaha;
use kalaha::ai;

fn ai_from_arg(name: &str, depth: Option<&str>) -> Box<ai::AI> {
    let depth = depth.map_or(6, |s| s.parse().unwrap());
    match name {
        "first" => return Box::new(ai::FirstValid {}),
        "last" => return Box::new(ai::LastValid {}),
        "minmax" => return Box::new(ai::MinMax::new(depth)),
        "alphabeta" => return Box::new(ai::AlphaBeta::new(depth)),
        _ => panic!("Unknown AI name"),
    }
}

fn main() {
    let matches = App::new("Kalaha simulator")
        .arg(Arg::with_name("ai_one")
            .short("a")
            .long("ai_one")
            .value_name("AI")
            .possible_values(&["first", "last", "minmax", "alphabeta"])
            .help("AI for player one"))
        .arg(Arg::with_name("ai_two")
            .short("b")
            .long("ai_two")
            .value_name("AI")
            .possible_values(&["first", "last", "minmax", "alphabeta"])
            .help("AI for player two"))
        .arg(Arg::with_name("ai_one_depth")
            .short("d")
            .long("ai_one_depth")
            .value_name("depth")
            .help("depth for AI one (if required)")
            .required_ifs(&[("ai_one", "minmax"), ("ai_one", "alphabeta")]))
        .arg(Arg::with_name("ai_two_depth")
            .short("e")
            .long("ai_two_depth")
            .value_name("depth")
            .help("depth for AI two (if required)")
            .required_ifs(&[("ai_two", "minmax"), ("ai_two", "alphabeta")]))
        .arg(Arg::with_name("verbose")
            .short("v")
            .help("Use verbose mode"))
        .get_matches();

    let ai_one = ai_from_arg(
        matches.value_of("ai_one").unwrap_or("minmax"),
        matches.value_of("ai_one_depth"),
    );
    let ai_two = ai_from_arg(
        matches.value_of("ai_two").unwrap_or("alphabeta"),
        matches.value_of("ai_two_depth"),
    );
    let verbose = matches.is_present("verbose");

    println!("Player A {:?} vs Player B {:?}", ai_one, ai_two);
    let mut game = Kalaha::new();
    game.play(&ai_one, &ai_two, verbose);
    println!("{}", game.game_result());

    println!("Player A {:?} vs Player B {:?}", ai_two, ai_one);
    let mut game = Kalaha::new();
    game.play(&ai_two, &ai_one, verbose);
    println!("{}", game.game_result());
}
