# Kalaha in Rust

Because why not?

## Usage

To run the demo program:

```bash
cargo run
```

The main entry point to the library is [kalaha::Kalaha](src/kalaha/mod.rs). To create a kalaha game object:

```rust
use kalaha::Kalaha;

let mut game = Kalaha::new();
```

To make a single move in the game (moves are between 0 and 5 inclusive):

```rust
game.choose(3);
```

You can also create an AI to play the game for you. AI objects implement the [kalaha::ai::AI](src/ai/mod.rs) trait,
which must return a valid move for the current player.

```rust
pub trait AI {
    fn choose(&self, game: &super::Kalaha) -> usize;
}
```

The following methods are available on the `game` object to help you choose the best move.

```rust
pub fn valid_move(&self, pond: usize) -> Result<(), Error>;
pub fn bank(&self, player: &Player) -> u32;
pub fn ponds(&self, player: &Player) -> [u32; 6];
```

You can also determine what the game would look like if you made a particular move with:

```rust
game.clone().choose(3);
```

Once you have two AI objects, you can play them against each other with:

```rust
game.play(ai_player_a, ai_player_b, true);
```

The last parameter is the `verbose` flag. If true, each move chosen and the board state after each move will
be printed to stdout.
