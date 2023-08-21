#![feature(drain_filter)]

// TODO -
// Environmental Stress

mod world;
mod settlement;
mod household;

use crate::world::{World, Settings};

// TODO
const birth_rate: f64 = 0.1;
const death_rate: f64 = 0.1;
const L: f64 = 0.6;
const years_per_move: u32 = 100;
const beta: f64 = 1.5;
const m: f64 = 0.005;

fn main() {
    let settings = Settings {
        size: 100,
        initial_settlements: 3,
        initial_households: 3,
    };

    let mut world = World::new(settings);
    world.iterate();

    println!("Settlements: {}", world.count_settlements());
    println!("Population: {}", world.count_population());
}
