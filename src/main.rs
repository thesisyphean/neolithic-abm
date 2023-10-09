#![feature(extract_if)]

mod world;
mod settlement;
mod household;

use crate::world::World;
use rayon::prelude::*;
use csv::Writer;
use std::fmt::Display;

// These are constant across simulations
const SIZE: usize = 50;
const SETTLEMENTS: usize = 10;
const HOUSHOLDS: usize = 100;
const ITERATIONS: u32 = 10000;

const BIRTH_RATE: f64 = 0.015;
const DEATH_RATE: f64 = 0.01;

// TODO: These aren't used
const years_per_move: u32 = 100;
const beta: f64 = 1.5;
const m: f64 = 0.005;

// These vary across simulations
pub struct Settings {
    pub f: f64,
    pub degradation: f64,
    pub title: String,
}

impl Settings {
    fn new(f: f64, degradation: f64, title: &str) -> Self {
        Settings {
            f,
            degradation,
            title: title.to_owned(),
        }
    }
}

fn main() {
    let settings = vec![
        // Settings::new(1.0, 0.0, "f1"),
        // Settings::new(2.0, 0.0, "f2"),
        Settings::new(4.0, 0.0, "f4"),
        // Settings::new(8.0, 0.0, "f8"),
        // Settings::new(16.0, 0.0, "f16"),
    ];

    let results: Vec<_> = settings.into_par_iter()
        .map(|s| run(s))
        .collect();

    for result in results {
        if let Err(RunError::CSVError(e)) = &result {
            eprintln!("CSV Error: {e}");
        }

        if let Err(RunError::FlushError(e)) = result {
            eprintln!("Flush Error: {e}");
        }
    }
}

fn run(settings: Settings) -> Result<(), RunError> {
    let title = settings.title.clone();
    let mut path = String::from("results/");
    path.push_str(&title);
    path.push_str(".csv");

    let mut writer = Writer::from_path(path)
        .map_err(RunError::CSVError)?;

    writer.write_record(&["Iteration", "Settlements", "Population",
        "AveResources", "MaxLoad", "PeerTransfer", "SubTransfer"])
        .map_err(RunError::CSVError)?;

    let mut world = World::new(settings);
    for i in 0..ITERATIONS {
        let (peer, subordinate) = world.cooperation();

        let fields: Vec<Box<dyn Display>> = vec![
            Box::new(world.iteration()),
            Box::new(world.count_settlements()),
            Box::new(world.count_population()),
            Box::new(world.average_resources()),
            Box::new(world.max_load()),
            Box::new(peer),
            Box::new(subordinate),
        ];

        writer.write_record(fields.iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>())
            .map_err(RunError::CSVError)?;

        world.iterate();

        if i % 100 == 0 {
            println!("Iteration {i} of {} completed!", title);
        }
    }

    writer.flush()
        .map_err(RunError::FlushError)?;
    Ok(())
}

enum RunError {
    CSVError(csv::Error),
    FlushError(std::io::Error),
}
