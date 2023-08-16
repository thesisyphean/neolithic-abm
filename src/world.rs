use crate::settlement::{Settlement};
use rand::{rngs::ThreadRng, RngCore};

pub struct World {
    settings: Settings,
    matrix: Vec<Vec<Cell>>,
    rng: ThreadRng,
}

impl World {
    pub fn new(settings: Settings) -> Self {
        let cells = settings.size * settings.size;

        // create the matrix
        let mut matrix: Vec<Vec<_>> = (0..settings.size)
            .map(|_| (0..settings.size)
                .map(|_| Cell::Unclaimed)
                .collect())
            .collect();

        let mut rng = rand::thread_rng();

        // assert initial_settlements <= cells?
        // spawn the initial settlements
        for n in 0..settings.initial_settlements {
            let mut new_index = rng.next_u32() as usize % (cells - n);

            'outer: for i in 0..settings.size {
                for j in 0..settings.size {
                    if let Cell::Unclaimed = matrix[i][j] {
                        if new_index == 0 {
                            // create and place the settlement
                            let settlement = Settlement::new(settings.initial_households);
                            matrix[i][j] = Cell::Settled(settlement);

                            break 'outer;
                        } else {
                            new_index -= 1;
                        }
                    }
                }
            }
        }

        World {
            settings,
            matrix,
            rng,
        }
    }

    pub fn iterate(&mut self) {
        for i in 0..self.settings.size {
            for j in 0..self.settings.size {
                if let Cell::Settled(settlement) = &mut self.matrix[i][j] {
                    // we've found a settlement
                    println!("Found a settlement at i: {}, j: {}", i, j);

                    for household in &mut settlement.households {
                        println!("There is actually a household here...");
                    }
                }
            }
        }
    }
}

enum Cell {
    Settled(Settlement),
    Claimed,
    Unclaimed,
}

pub struct Index(usize, usize);

// TODO
pub struct Settings {
    pub size: usize,
    pub initial_settlements: usize,
    pub initial_households: usize,
}
