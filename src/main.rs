#![feature(drain_filter)]

use std::error::Error;
use csv::Writer;

mod world;
mod settlement;
mod household;

use crate::world::{World, Settings};

// TODO
const BIRTH_RATE: f64 = 0.1;
const DEATH_RATE: f64 = 0.000001;
const years_per_move: u32 = 100;
const beta: f64 = 1.5;
const m: f64 = 0.005;

const ITERATIONS: u32 = 1_000;

fn main() {
    let settings = Settings {
        size: 50,
        initial_settlements: 10,
        initial_households: 100,
    };

    if let Err(error) = run(settings) {
        eprintln!("Error: {}", error);
    }
}

fn run(settings: Settings) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path("results/results.csv")?;
    writer.write_record(&["Iteration", "Settlements", "Population"])?;

    let mut world = World::new(settings);
    for i in 1..=ITERATIONS {
        let iteration = i.to_string();
        let settlements = world.count_settlements().to_string();
        let pop = world.count_population();
        let coop = world.average_cooperation();
        let population = pop.to_string();

        writer.write_record(&[iteration, settlements, population])?;

        world.iterate();

        if i % 2 == 1 {
            println!("Iteration {} complete - population {} - cooperation {}", i, pop, coop);
        }
    }

    // TODO: Why is this important?
    writer.flush()?;
    Ok(())
}
