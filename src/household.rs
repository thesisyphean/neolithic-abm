use crate::world::Index;

// TODO: move other constants here
const CONSUMPTION: f64 = 0.5;
const L: f64 = 0.6;
const RESOURCE_DEGREDATION: f64 = 0.0;
const MUTATION_FREQ: f64 = 0.33;
const MUTATION_AMPL: f64 = 0.25;

pub struct Household {
    pub id: u32,
    pub resources: f64,
    pub load: f64,
    pub hunger: f64,
    pub resource_patch: Option<Index>,
    pub genes: Genes,
    pub years_since_move: u32,
    pub satisfaction: f64,
}

impl Household {
    // TODO: once-over these
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

    pub fn required(&self) -> f64 {
        f64::max(CONSUMPTION - self.resources, 0.0)
    }

    pub fn provide(&mut self, resources: f64) {
        self.resources += resources;
    }

    pub fn consume(&mut self) {
        self.resources = f64::max(self.resources - CONSUMPTION, 0.0);

        // TODO: update hunger and satisfaction
        self.hunger = f64::min(self.resources / 0.5, 1.0);
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
            self.load += required;
        }

        donating
    }

    pub fn birth_new(&mut self, genes: Genes, id: u32) -> Self {
        // TODO: check if other attributes need to be changed
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

    // these three methods were pulled from cnc
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
        chance < self.hunger * crate::BIRTH_RATE
    }

    pub fn death(&self, chance: f64) -> bool {
        chance * self.hunger < crate::DEATH_RATE
    }

    // TODO: obviously you need to get migration working...
    /*pub fn update_satisfaction(&mut self, consumed: f64) {
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
        // TODO: 12 is obviously not the right value
        if self.years_since_move >= 12 {
            //
        }
    }*/

    fn degrade_resources(&mut self, chance: f64) {
        self.resources *= 1.0 - RESOURCE_DEGREDATION * chance;
    }
}

#[derive(Clone, Copy)]
pub struct Genes {
    peer_transfer: f64, // likelihood of contributing to peers
    subordinate_transfer: f64, // ditto for subordinates
    attachment: f64, // likelihood of remaining in a settlement
}

impl Genes {
    fn new(peer_transfer: f64, subordinate_transfer: f64,
           attachment: f64) -> Self {
        Genes {
            peer_transfer,
            subordinate_transfer,
            attachment,
        }
    }

    fn altruistic(attachment: f64) -> Self {
        Genes::new(1.0, 1.0, attachment)
    }

    fn defective(attachment: f64) -> Self {
        Genes::new(0.0, 0.0, attachment)
    }

    fn combine(&self, other: Self) -> Self {
        Genes::new(
            Self::random_choice(self.peer_transfer, other.peer_transfer),
            Self::random_choice(self.subordinate_transfer, other.subordinate_transfer),
            Self::random_choice(self.attachment, other.attachment),
        )
    }

    fn random_choice(first: f64, second: f64) -> f64 {
        let new_gene = if rand::random() {
            first
        } else {
            second
        };

        if rand::random::<f64>() < MUTATION_FREQ {
            new_gene + MUTATION_AMPL * rand::random::<f64>()
        } else {
            new_gene
        }
    }

    pub fn cooperation(&self) -> f64 {
        (self.peer_transfer + self.subordinate_transfer) / 2.0
    }
}

impl Default for Genes {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.5)
    }
}

pub enum QueryType {
    Superior,
    Peer,
    Subordinate,
}
