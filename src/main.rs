#![feature(drain_filter)]

use std::error::Error;
use csv::Writer;

mod world;
mod settlement;
mod household;

use crate::world::{World, Settings};

// TODO
const birth_rate: f64 = 0.01; // wrong
const death_rate: f64 = 0.001;
const years_per_move: u32 = 100;
const beta: f64 = 1.5;
const m: f64 = 0.005;

fn main() {
    let settings = Settings {
        size: 1000,
        initial_settlements: 3,
        initial_households: 100,
    };

    if let Err(error) = run(settings) {
        eprintln!("Error: {}", error);
    }
}

fn run(settings: Settings) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path("results.csv")?;
    writer.write_record(&["Iteration", "Settlements", "Population"])?;

    let mut world = World::new(settings);
    for i in 0..10_000 {
        let iteration = i.to_string();
        let settlements = world.count_settlements().to_string();
        let population = world.count_population().to_string();

        writer.write_record(&[iteration, settlements, population])?;

        world.iterate();
    }

    // TODO: Why is this important?
    writer.flush()?;
    Ok(())
}
