use crate::GeneSettings;
use crate::household::{Household, Genes, QueryType};
use crate::world::Index;
use rand::{rngs::ThreadRng, Rng};

pub struct Settlement {
    pub id: u32, // used for marking land in the matrix
    pub position: Index, // position in the matrix
    pub households: Vec<Household>,
}

impl Settlement {
    pub fn new(id: u32, position: Index, initial_households: usize, initial_genes: GeneSettings) -> Self {
        let households = (0..initial_households)
            .map(|n| {
                Household::new(n as u32, match initial_genes {
                    GeneSettings::Altruistic => Genes::altruistic(),
                    GeneSettings::Defective => Genes::defective(),
                    GeneSettings::Split => if n % 2 == 0 {
                        Genes::altruistic()
                    } else {
                        Genes::defective()
                    }
                })
            }).collect();

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
                if other_household.query_donation(required, QueryType::Subordinate, rng.gen()) {
                    return true;
                }
            }
        }

        // check with peers second
        for (j, other_household) in self.households.iter_mut().enumerate() {
            if i != j && other_household.is_peer(status) {
                if other_household.query_donation(required, QueryType::Peer, rng.gen()) {
                    return true;
                }
            }
        }

        // check with subordinates last
        for (j, other_household) in self.households.iter_mut().enumerate() {
            if i != j && other_household.is_sub(status) {
                if other_household.query_donation(required, QueryType::Superior, 0.0) {
                    return true;
                }
            }
        }

        false
    }

    pub fn influence(&self, other: &Self) -> f64 {
        other.status().powf(crate::beta) -
            crate::m * self.position.dist(other.position)
    }

    pub fn status(&self) -> f64 {
        self.households.iter()
            // TODO: shouldn't this be resources as well?
            .map(|h| h.load)
            .sum()
    }

    // one call of this method invalidates positions, so we need to use ids
    pub fn remove(&mut self, id: u32) -> Household {
        let i = self.households.iter()
            .position(|h| h.id == id)
            .unwrap();

        self.households.swap_remove(i)
    }

    // the household with id pairs with another household with genes
    pub fn add(&mut self, id: u32, genes: Genes) {
        let new_id = self.max_id() + 1;
        let pos = self.pos(id);

        let new_household = self.households[pos].birth_new(genes, new_id);
        self.households.push(new_household);
    }

    fn max_id(&self) -> u32 {
        self.households.iter()
            .map(|h| h.id)
            .max().unwrap_or(0)
    }

    fn pos(&self, id: u32) -> usize {
        self.households.iter()
            .position(|h| h.id == id)
            .unwrap()
    }

    pub fn find_genes(&self, mut status: f64) -> Genes {
        for household in &self.households {
            if status <= household.load {
                return household.genes;
            }

            status -= household.load;
        }

        panic!("status was > the sum of all statuses")
    }

    pub fn population(&self) -> usize {
        self.households.len()
    }

    pub fn average_cooperation(&self) -> f64 {
        self.households.iter()
            .map(|h| h.genes.cooperation())
            .sum::<f64>() / self.population() as f64
    }

    pub fn cooperation(&self) -> (f64, f64) {
        let coop_sum = self.households.iter()
            .fold((0.0, 0.0), |a, h|
                (a.0 + h.genes.peer_transfer, a.1 + h.genes.subordinate_transfer));

        let pop = self.population() as f64;
        (coop_sum.0 / pop, coop_sum.1 / pop)
    }

    pub fn statuses(&self) -> Vec<f64> {
        self.households.iter()
            .map(|h| h.status())
            .collect()
    }

    pub fn average_resources(&self) -> f64 {
        self.households.iter()
            .map(|h| h.resources)
            .sum::<f64>() / self.population() as f64
    }

    pub fn max_load(&self) -> f64 {
        self.households.iter()
            .map(|h| h.load)
            .fold(0.0 / 0.0, f64::max)
    }
}
