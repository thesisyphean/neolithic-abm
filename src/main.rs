#![feature(extract_if)]

const VISUAL: bool = false;

mod household;
mod settlement;
mod visualiser;
mod world;

use crate::visualiser::Visualiser;
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
    pub genes: GeneSettings,
}

impl Settings {
    fn new(f: f64, degradation: f64, title: String, path: String, genes: GeneSettings) -> Self {
        Settings {
            f,
            degradation,
            title,
            path,
            genes: genes,
        }
    }
}

#[derive(Clone, Copy)]
enum GeneSettings {
    Altruistic,
    Defective,
    Split,
}

impl Display for GeneSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeneSettings::Altruistic => 'A',
                GeneSettings::Defective => 'D',
                GeneSettings::Split => 'S',
            }
        )
    }
}

fn main() {
    if VISUAL {
        visualise(Settings::new(
            256.0,
            0.25,
            String::new(),
            String::new(),
            GeneSettings::Altruistic,
        ));
    } else {
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
}

fn generate_settings() -> Vec<Settings> {
    let mut settings = vec![];
    /* let mut settings = vec![Settings::new(
        4.0,
        0.25,
        String::from("S_f_2_d_5"),
        String::from("results/S_f_2_d_5.csv"),
        GeneSettings::Split,
    )]; */

    // purge any previous results
    // fs::remove_dir_all("results").unwrap();
    // fs::create_dir("results").unwrap();

    for genes in [
        GeneSettings::Split,
        GeneSettings::Altruistic,
        GeneSettings::Defective,
    ] {
        let folder = format!("results/{}", genes);
        fs::create_dir(&folder).unwrap();

        // Debug resolution
        for f in 1..=5 {
            for d in 0..5 {
                let f_final = 2.0f64.powi(f);
                let degradation = 0.2 * d as f64;

                let title = format!("{}_f_{}_d_{}", genes, f, d);
                let path = format!("{}/{}.csv", folder, title);

                settings.push(Settings::new(f_final, degradation, title, path, genes));
            }
        }

        /* Release resolution
        for f in 1..=12 {
            for d in 0..20 {
                let f_final = 1.587f64.powi(f);
                let degradation = 0.05 * d as f64;

                let title = format!("{}_f_{}_d_{}", genes, f, d);
                let path = format!("{}/{}.csv", folder, title);

                settings.push(Settings::new(f_final, degradation, title, path, genes));
            }
        } */
    }

    settings
}

fn visualise(settings: Settings) {
    let mut world = World::new(settings);
    let mut visualiser = Visualiser::new();
    visualiser.initialise();

    for i in 0..ITERATIONS / 2 {
        world.iterate();

        if i % 10 == 0 {
            visualiser.update_agents(world.count_population() as u32);
            visualiser.update_patches(world.count_patches() as u32);
            visualiser.save();
        }

        if i % 10 == 0 {
            println!("{i} iterations visualised!");
        }
    }
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
            "MaxResources",
            "MaxLoad",
            "PeerTransfer",
            "SubTransfer",
            "Egalitarianism",
        ])
        .map_err(RunError::CSVError)?;

    // TODO: take averages of three different worlds
    let mut world = World::new(settings);
    for i in 0..ITERATIONS {
        let (peer, subordinate) = world.cooperation();

        let fields: Vec<Box<dyn Display>> = vec![
            Box::new(world.iteration()),
            Box::new(world.count_settlements()),
            Box::new(world.count_population()),
            Box::new(world.average_resources()),
            Box::new(world.max_resources()),
            Box::new(world.max_load()),
            Box::new(peer),
            Box::new(subordinate),
            Box::new(world.egalitarianism()),
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
