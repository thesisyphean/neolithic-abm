use crate::{L, years_per_move};
use crate::world::Index;

pub struct Household {
    pub id: u32,
    pub resources: f64,
    pub hunger: f64,
    pub resource_patch: Option<Index>,
    pub load: f64,
    pub genes: Genes,
    years_since_move: u32,
    satisfaction: f64,
}

impl Household {
    pub fn new(id: u32, genes: Genes) -> Self {
        Household {
            id,
            resources: 0.0,
            hunger: 0.0,
            resource_patch: None,
            load: 0.0,
            genes,
            years_since_move: 0,
            satisfaction: 0.0,
        }
    }

    pub fn consume(&mut self, resources: f64) -> Option<f64> {
        self.resources += resources;

        if self.resources > 0.5 {
            self.resources -= 0.5;
            None
        } else {
            let result = Some(self.resources);
            self.resources = 0.0;
            result
        }
    }

    pub fn query_donation(&mut self, required: f64, query_type: QueryType,
        chance: f64) -> bool {
        // don't have the resources to donate
        if required > self.resources {
            return false;
        }

        let donating = match query_type {
            QueryType::Superior => true,
            QueryType::Peer => chance < self.genes.peer_transfer,
            QueryType::Subordinate => chance < self.genes.subordinate_transfer,
        };

        if donating {
            self.resources -= required;
        }
        donating
    }

    pub fn birth_new(&mut self, genes: Genes, id: u32) -> Self {
        // resources are split between parent and child
        self.resources /= 2.0;

        Household {
            id,
            resources: self.resources,
            hunger: 0.0,
            resource_patch: None,
            load: 0.0,
            genes: self.genes.combine(genes),
            years_since_move: 0,
            satisfaction: 0.0,
        }
    }

    pub fn status(&self) -> f64 {
        self.resources + self.load
    }

    pub fn is_peer(&self, other_status: f64) -> bool {
        (other_status - self.status()).abs() /
            f64::max(self.status(), other_status)
            <= L
    }

    pub fn is_auth(&self, other_status: f64) -> bool {
        (other_status - self.status()) /
            f64::max(self.status(), other_status)
            > L
    }

    pub fn is_sub(&self, other_status: f64) -> bool {
        (self.status() - other_status) /
            f64::max(other_status, self.status())
            > L
    }

    pub fn birth(&self, chance: f64) -> bool {
        chance < self.hunger * crate::birth_rate
    }

    pub fn death(&self, chance: f64) -> bool {
        chance * crate::death_rate < self.hunger 
    }

    pub fn update_satisfaction(&mut self, consumed: f64) {
        self.satisfaction *= self.years_since_move as f64;
        self.satisfaction += consumed;

        self.years_since_move += 1;
        self.satisfaction /= self.years_since_move as f64;
    }

    pub fn movement(&self, chance: f64) -> bool {
        2.0 * self.genes.attachment * self.satisfaction
            < chance
    }

    fn migrate(&mut self) {
        if self.years_since_move >= years_per_move {
            //
        }
    }

    fn degrade_resources(&mut self, delta: f64, degradation: f64) {
        self.resources *= 1.0 - delta * degradation;
    }
}

#[derive(Clone, Copy)]
pub struct Genes {
    peer_transfer: f64,
    subordinate_transfer: f64,
    conformity: f64,
    attachment: f64,
}

impl Genes {
    fn new(peer_transfer: f64, subordinate_transfer: f64,
           conformity: f64, attachment: f64) -> Self {
        Genes {
            peer_transfer,
            subordinate_transfer,
            conformity,
            attachment,
        }
    }

    fn altruistic(conformity: f64, attachment: f64) -> Self {
        Genes::new(1.0, 1.0, conformity, attachment)
    }

    fn defective(conformity: f64, attachment: f64) -> Self {
        Genes::new(0.0, 0.0, conformity, attachment)
    }

    fn combine(&self, other: Self) -> Self {
        // TODO - uniform crossover and random mutation
        Self::default()
    }
}

impl Default for Genes {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5)
    }
}

pub enum QueryType {
    Superior,
    Peer,
    Subordinate,
}
