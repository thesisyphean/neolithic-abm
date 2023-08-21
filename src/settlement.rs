use crate::household::{Household, Genes, QueryType};
use crate::world::Index;
use rand::{rngs::ThreadRng, Rng};

pub struct Settlement {
    pub id: u32,
    pub position: Index,
    // TODO - implement this as an iterator
    pub households: Vec<Household>,
}

impl Settlement {
    pub fn new(id: u32, position: Index, initial_households: usize) -> Self {
        let households = (0..initial_households)
            .map(|n| Household::new(n as u32, Genes::default()))
            .collect();

        Settlement {
            id,
            position,
            households,
        }
    }

    pub fn query_donations(&mut self, i: usize, required: f64,
        rng: &mut ThreadRng) -> bool {
        let status = self.households[i].status();

        // check with superiors first
        for (j, other_household) in self.households.iter_mut().enumerate() {
            if i != j && other_household.is_auth(status) {
                let chance: f64 = rng.gen();

                if other_household.query_donation(required, QueryType::Subordinate, chance) {
                    return true;
                }
            }
        }

        // check with peers second
        for (j, other_household) in self.households.iter_mut().enumerate() {
            if i != j && other_household.is_peer(status) {
                let chance: f64 = rng.gen();

                if other_household.query_donation(required, QueryType::Peer, chance) {
                    return true;
                }
            }
        }

        // get from subordinates last
        for (j, other_household) in self.households.iter_mut().enumerate() {
            if i != j && other_household.is_sub(status) {
                if other_household.query_donation(required, QueryType::Superior, 0.0) {
                    return true;
                }
            }
        }

        self.households[i].hunger = 2.0 * (0.5 - required);
        false
    }

    pub fn influence(&self, other: &Self) -> f64 {
        other.status().powf(crate::beta) -
            crate::m * self.position.dist(other.position)
    }

    pub fn status(&self) -> f64 {
        self.households.iter()
            .map(|h| h.load)
            .sum()
    }

    pub fn average_resources(&self) -> f64 {
        self.households.iter()
            .map(|h| h.resources)
            .sum::<f64>() / self.households.len() as f64
    }

    pub fn remove(&mut self, id: u32) -> Household {
        let i = self.households.iter()
            .position(|h| h.id == id)
            .unwrap();

        self.households.swap_remove(i)
    }

    pub fn add(&mut self, id: u32, genes: Genes) {
        let i = self.households.iter()
            .position(|h| h.id == id)
            .unwrap();

        let new_id = self.households.len() as u32;
        let new_household = self.households[i].birth_new(genes, new_id);

        self.households.push(new_household);
    }

    pub fn find_genes(&self, mut status: f64) -> Genes {
        for household in &self.households {
            if status <= household.load {
                return household.genes;
            }

            status -= household.load;
        }

        self.households[0].genes
    }

    pub fn population(&self) -> usize {
        self.households.len()
    }
}
