mod error;
mod player;
mod pool;
mod turn;
mod board;
mod kalaha;

pub mod ai;
pub use kalaha::Kalaha;

const INIT_COUNT: u32 = 6;
const PONDS_PER_PLAYER: usize = 6;
const TOTAL_POOLS: usize = 14;

