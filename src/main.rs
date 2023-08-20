mod pets;
mod food;
mod states;

use pets::Pet;
use states::{State, simulate_battle};

enum Food {
    Apple,
}



fn main() {
    let game_result = simulate_battle(State);

    println!("Hello, world!");
}
