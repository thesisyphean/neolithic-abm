use crate::L;
use crate::world::Index;
use std::cmp;

pub struct Household {
    resources: f64,
    resource_patch: Option<Index>,
    load: f64,
    genes: Genes,
}

impl Household {
    pub fn new(genes: Genes) -> Self {
        Household {
            resources: 0.0,
            resource_patch: None,
            load: 0.0,
            genes,
        }
    }

    pub fn birth_new(&mut self, other: Self) -> Self {
        // resources is split between parent and child
        self.resources /= 2.0;

        Household {
            resources: self.resources,
            resource_patch: None,
            load: 0.0,
            genes: self.genes.combine(&other.genes),
        }
    }

    fn status(&self) -> f64 {
        self.resources + self.load
    }

    fn is_peer(&self, other: &Self) -> bool {
        (other.status() - self.status()).abs() /
            cmp::max_by(self.status(), other.status(), f64::total_cmp)
            <= L
    }

    fn is_auth(&self, other: &Self) -> bool {
        (other.status() - self.status()) /
            cmp::max_by(self.status(), other.status(), f64::total_cmp)
            > L
    }

    fn is_sub(&self, other: &Self) -> bool {
        other.is_auth(self)
    }

    fn hunger(&self) -> f64 {
        cmp::min_by(self.resources / 0.5, 1.0, f64::total_cmp)
    }

    fn birth(&self, chance: f64) -> bool {
        chance < self.hunger() * crate::birth_rate
    }

    fn death(&self, chance: f64) -> bool {
        chance * crate::death_rate < self.hunger()
    }
}

impl Default for Household {
    fn default() -> Self {
        Household::new(Genes::default())
    }
}

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

    fn combine(&self, other: &Self) -> Self {
        // TODO - uniform crossover and random mutation
        Self::default()
    }
}

impl Default for Genes {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5)
    }
}
