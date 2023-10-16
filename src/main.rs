#![feature(extract_if)]

mod household;
mod settlement;
mod world;

use crate::household::Genes;
use crate::world::World;
use csv::Writer;
use rayon::prelude::*;
use std::fmt::Display;
use std::fs;

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
#[derive(Clone)]
pub struct Settings {
    pub f: f64,
    pub degradation: f64,
    pub title: String,
    pub path: String,
    pub genes: Genes,
}

impl Settings {
    fn new(f: f64, degradation: f64, title: String, path: String, genes: Genes) -> Self {
        Settings {
            f,
            degradation,
            title,
            path,
            genes: genes,
        }
    }
}

fn main() {
    let settings = generate_settings();
    let results: Vec<_> = settings.into_par_iter().map(run).collect();

    for result in results {
        if let Err(RunError::CSVError(e)) = &result {
            eprintln!("CSV Error: {e}");
        }

        if let Err(RunError::FlushError(e)) = result {
            eprintln!("Flush Error: {e}");
        }
    }
}

fn generate_settings() -> Vec<Settings> {
    let mut settings = vec![];

    // purge any previous results
    fs::remove_dir_all("results").unwrap();
    fs::create_dir("results").unwrap();

    for genes in [Genes::default(), Genes::altruistic(), Genes::defective()] {
        let folder = format!("results/{}", genes);
        fs::create_dir(folder).unwrap();

        for f in 0..8 {
            for d in 0..10 {
                let f_final = 2.0f64.powi(f);
                let degradation = 0.1 * d as f64;

                let title = format!("{}_f_{}_d_{}", genes, f, d);
                let path = format!("{}/{}.csv", folder, title);

                settings.push(Settings::new(f_final, degradation, title, path, genes));
            }
        }
    }

    settings
}

fn run(settings: Settings) -> Result<(), RunError> {
    let title = settings.title.clone();
    let mut writer = Writer::from_path(&settings.path).map_err(RunError::CSVError)?;

    writer
        .write_record(&[
            "Iteration",
            "Settlements",
            "Population",
            "AveResources",
            "MaxLoad",
            "PeerTransfer",
            "SubTransfer",
            "Gini",
        ])
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
            Box::new(world.gini_coefficient()),
        ];

        writer
            .write_record(fields.iter().map(|f| f.to_string()).collect::<Vec<_>>())
            .map_err(RunError::CSVError)?;

        world.iterate();

        if i % 100 == 0 {
            println!("Iteration {i} of {} completed!", title);
        }
    }

    writer.flush().map_err(RunError::FlushError)?;
    Ok(())
}

enum RunError {
    CSVError(csv::Error),
    FlushError(std::io::Error),
}
