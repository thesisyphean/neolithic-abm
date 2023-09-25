use crate::settlement::Settlement;
use std::collections::VecDeque;
use rand::{rngs::ThreadRng, Rng, RngCore};

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

        // create the matrix with all unclaimed cells
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
            let mut new_index = rng.next_u32() as usize % (cells - n);

            'outer: for i in 0..settings.size {
                for j in 0..settings.size {
                    if let Cell::Unclaimed = matrix[i][j] {
                        if new_index == 0 {
                            // create and place the settlement
                            let settlement = Settlement::new(n as u32,
                                Index(i, j), settings.initial_households);
                            settlements.push(settlement);
                            matrix[i][j] = Cell::Settled(n as u32);

                            break 'outer;
                        }

                        new_index -= 1;
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
        if self.count_population() < self.settings.size.pow(2) {
            self.iterate_settlement();
        }

        // agents consume and request resources
        self.iterate_consumption();

        // agents reproduce based on their hunger
        self.iterate_birth();

        // agents die based on their hunger
        self.iterate_death();

        // agents migrate based on their satisfaction
        // self.iterate_migration();

        self.iteration += 1;
    }

    fn iterate_settlement(&mut self) {
        // each vector holds the indices of the households in that settlement
        //   that don't have a resource patch
        let mut to_search: Vec<_> = (0..self.settlements.len())
            .map(|_| Vec::new())
            .collect();

        for (n, settlement) in self.settlements.iter().enumerate() {
            for (i, household) in settlement.households.iter().enumerate() {
                // record that the household is searching for a resource patch
                if household.resource_patch.is_none() {
                    to_search[n].push(i);
                }
            }
        }

        for (n, searches) in to_search.iter().enumerate() {
            for &i in searches {
                let id = self.settlements[n].id;

                // the household searches for an available patch
                let unclaimed_patch = self.find_unclaimed_patch(self.settlements[n].position, id);

                // if they fail, there are no neighbouring patches,
                //   so no other household in the same settlement will find one
                if unclaimed_patch.is_none() {
                    break;
                }

                // update the household and matrix
                self.settlements[n].households[i].resource_patch = unclaimed_patch;
                let pos = unclaimed_patch.unwrap();
                self.matrix[pos.0][pos.1] = Cell::Claimed(id);
            }
        }
    }

    fn iterate_consumption(&mut self) {
        for settlement in &mut self.settlements {
            let mut requests = Vec::new();

            for (i, household) in settlement.households.iter_mut().enumerate() {
                // if a houshold has a resource patch, they gather resources from it
                household.provide(if household.resource_patch.is_some() {
                    World::resources() } else { 0.0 });

                // the household returns how much they need
                let required = household.required();
                if required > 0.0 {
                    requests.push((i, required));
                }
            }

            // perform the requests
            for (i, required) in requests {
                if settlement.query_donations(i, required, &mut self.rng) {
                    settlement.households[i].provide(required);
                }
            }

            // having gathered and requested resources, agents consume them
            for household in &mut settlement.households {
                household.consume();
            }
        }
    }

    // we know that an agent's hunger has been set
    fn iterate_birth(&mut self) {
        let mut births: Vec<_> = (0..self.settlements.len())
            .map(|_| Vec::new())
            .collect();

        for (n, settlement) in self.settlements.iter().enumerate() {
            let total_statuses = self.settlements.iter()
                .filter(|s| settlement.influence(s) > 0.0)
                .map(|s| s.status())
                .sum::<f64>() as u32;

            for household in &settlement.households {
                if household.birth(self.rng.gen()) {
                    // we choose another partner from the possible options
                    if total_statuses == 0 {
                        births[n].push((household.id, household.genes));
                    } else {
                        let mut chosen = (self.rng.next_u32() % total_statuses) as f64;
                        let mut genes = household.genes;

                        for s in &self.settlements {
                            if settlement.influence(s) <= 0.0 { continue; }

                            if chosen <= settlement.status() {
                                genes = settlement.find_genes(chosen);
                                break;
                            }

                            chosen -= settlement.status();
                        }

                        births[n].push((household.id, genes));
                    }
                }
            }
        }

        for (n, settlement_births) in births.into_iter().enumerate() {
            for (id, genes) in settlement_births {
                self.settlements[n].add(id, genes);
            }
        }
    }

    fn iterate_death(&mut self) {
        let mut settlements_to_remove = Vec::new();

        for settlement in &mut self.settlements {
            let to_remove: Vec<_> = settlement.households.iter()
                .filter(|h| h.death(self.rng.gen()))
                .map(|h| h.id)
                .collect();

            let removed = settlement.households.drain_filter(|h|
                to_remove.contains(&h.id));
            
            for household in removed {
                if let Some(pos) = household.resource_patch {
                    self.matrix[pos.0][pos.1] = Cell::Unclaimed;
                }
            }

            if settlement.households.len() == 0 {
                settlements_to_remove.push(settlement.id);
            }
        }

        for settlement_id in settlements_to_remove {
            let position = self.settlements.iter()
                .position(|s| s.id == settlement_id)
                .unwrap();

            self.settlements.swap_remove(position);
        }
    }

    fn iterate_migration(&mut self) {
        // TODO -
    }

    pub fn resources() -> f64 {
        // TODO
        0.8
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

    pub fn count_settlements(&self) -> usize {
        self.settlements.len()
    }

    pub fn count_population(&self) -> usize {
        self.settlements.iter()
            .map(|s| s.population())
            .sum()
    }

    pub fn average_cooperation(&self) -> f64 {
        self.settlements.iter()
            .map(|s| s.cooperation())
            .sum::<f64>() / self.count_settlements() as f64
    }

    // TODO: this is gonna be weird...
    pub fn gini_coefficient(&self) -> f64 {
        0.0
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
        let o = other.as_isize();

        let x = (s.0 - o.0).pow(2) as f64;
        let y = (s.1 - o.1).pow(2) as f64;

        (x + y).sqrt()
    }

    fn as_isize(&self) -> (isize, isize) {
        (self.0 as isize, self.1 as isize)
    }
}

pub struct Settings {
    pub size: usize,
    pub initial_settlements: usize,
    pub initial_households: usize,
}
