mod world;
mod settlement;
mod household;

use crate::world::{World, Settings};

// TODO
const birth_rate: f64 = 0.1;
const death_rate: f64 = 0.1;
const L: f64 = 0.6;

fn main() {
    println!("Hello, Neo!");

    let settings = Settings {
        size: 100,
        initial_settlements: 3,
        initial_households: 3,
    };

    let mut world = World::new(settings);
    world.iterate();
}
