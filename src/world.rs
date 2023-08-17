use crate::settlement::{Settlement};
use rand::{rngs::ThreadRng, Rng, RngCore};
use std::collections::VecDeque;

pub struct World {
    settings: Settings,
    matrix: Vec<Vec<Cell>>,
    settlements: Vec<Settlement>,
    iteration: u32,
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

        let mut settlements = Vec::new();

        let mut rng = rand::thread_rng();

        // assert initial_settlements <= cells?
        // spawn the initial settlements
        for n in 0..settings.initial_settlements {
            let mut new_index = rng.next_u32() as usize % (cells - n) + 1;

            'outer: for i in 0..settings.size {
                for j in 0..settings.size {
                    if let Cell::Unclaimed = matrix[i][j] {
                        new_index -= 1;

                        if new_index == 0 {
                            // create and place the settlement
                            let settlement = Settlement::new(n as u32,
                                Index(i, j), settings.initial_households);
                            settlements.push(settlement);
                            matrix[i][j] = Cell::Settled(n as u32);

                            break 'outer;
                        }
                    }
                }
            }
        }

        World {
            settings,
            matrix,
            settlements,
            iteration: 0,
            rng,
        }
    }

    pub fn iterate(&mut self) {
        // agents without a resource patch try to claim one
        self.iterate_settlement();

        // agents consume and request resources
        self.iterate_consumption();

        // agents reproduce based on their hunger
        self.iterate_birth();

        // agents die based on their hunger
        self.iterate_death();

        // agents migrate based on their satisfaction
        self.iterate_migration();

        self.iteration += 1;
    }

    fn iterate_settlement(&mut self) {
        for settlement in &mut self.settlements {
            for household in &mut settlement.households {
                if household.resource_patch.is_none() {
                    let unclaimed_patch = self.find_unclaimed_patch(settlement.position,
                        settlement.id);

                    if unclaimed_patch.is_none() {
                        // if an agent fails to find a new patch, there isn't one
                        break;
                    } else {
                        // give the agent the resource patch and mark it as claimed
                        household.resource_patch = unclaimed_patch;
                        let pos = unclaimed_patch.unwrap();
                        // this is necessary so agents don't leave their settlement's territory
                        self.matrix[pos.0][pos.1] = Cell::Claimed(settlement.id);
                    }
                }
            }
        }
    }

    fn iterate_consumption(&mut self) {
        for settlement in &mut self.settlements {
            let mut requests = Vec::new();

            for (i, household) in settlement.households.iter_mut().enumerate() {
                let resources: f64 = if household.resource_patch.is_some() {
                    self.rng.gen() } else { 0.0 };

                if let Some(required) = household.consume(resources) {
                    requests.push((i, required));
                }

                household.hunger = 1.0;
            }

            for (i, required) in requests {
                settlement.query_donations(i, required, &mut self.rng);
            }
        }
    }

    // we know that an agent's hunger has been set
    fn iterate_birth(&mut self) {
        for settlement in &mut self.settlements {
            let mut to_add = Vec::new();

            for household in &mut settlement.households {
                if household.birth(self.rng.gen()) {
                    // the household has reproduced
                    // we need to find the other household

                    

                    // let new_household = household.birth_new();

                    to_add.push(new_household);
                }
            }

            for new_household in to_add {
                settlement.households.push(new_household);
            }
        }
    }

    fn iterate_death(&mut self) {
        for settlement in self.settlements {
            let mut to_remove = Vec::new();

            for household in &mut settlement.households {
                if household.death(self.rng.gen()) {
                    // the household has died
                    to_remove.push(household.id);
                }
            }

            let removed = settlement.households.drain_filter(|h|
                to_remove.contains(&h.id));
            
            for household in removed {
                if let Some(pos) = household.resource_patch {
                    self.matrix[pos.0][pos.1] = Cell::Unclaimed;
                }
            }
        }
    }

    fn iterate_migration(&mut self) {
        // TODO -
    }

    pub fn resources(iteration: u32) -> f64 {
        // TODO
        0.5
    }

    // this is a simple grid traversal algorithm
    pub fn find_unclaimed_patch(&self, pos: Index, id: u32) -> Option<Index> {
        let mut searched = vec![pos];
        let mut to_search = VecDeque::from(pos.surroundings(self.settings.size as isize));

        while !to_search.is_empty() {
            let current_pos = to_search.pop_front().unwrap();

            // avoid repeating searches
            if searched.contains(&current_pos) {
                continue;
            }

            let cell = &self.matrix[current_pos.0][current_pos.1];

            // we've found an empty cell so we return
            if let Cell::Unclaimed = cell {
                return Some(current_pos);
            }

            searched.push(current_pos);

            // we can only continue searching on cells that belong to our settlement
            // otherwise we are trespassing
            if let Cell::Claimed(cid) = cell {
                if *cid == id {
                    let mut surroundings = VecDeque::from(current_pos.surroundings(self.settings.size as isize));
                    to_search.append(&mut surroundings);
                }
            }
        }

        None
    }
}

enum Cell {
    // the id of the settlement
    Settled(u32),
    // the id of the settlement that claimed it
    Claimed(u32),
    Unclaimed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Index(usize, usize);

impl Index {
    fn surroundings(&self, size: isize) -> Vec<Index> {
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        dirs.into_iter()
            .map(|d| ((self.0 as isize) + d.0, (self.1 as isize) + d.1))
            .filter(|s| s.0 >= 0 && s.0 < size &&
                s.1 >= 0 && s.1 < size)
            .map(|s| Index(s.0 as usize, s.1 as usize))
            .collect()
    }

    pub fn dist(&self, other: Self) -> f64 {
        let s = self.as_isize();
        let o = self.as_isize();

        let x = (s.0 - o.0).pow(2) as f64;
        let y = (s.1 - o.1).pow(2) as f64;

        (x + y).sqrt()
    }

    fn as_isize(&self) -> (isize, isize) {
        (self.0 as isize, self.1 as isize)
    }
}

// TODO
pub struct Settings {
    pub size: usize,
    pub initial_settlements: usize,
    pub initial_households: usize,
}
