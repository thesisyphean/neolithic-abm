use std::cmp;

struct Settlement {
    households: Vec<Household>,
}

struct Household {
    storage: f64,
    resource_patch: bool,
    load: f64,

    peer_transfer: f64,
    subordinate_transfer: f64,
    conformity: f64,
    attachment: f64,
}

impl Household {
    fn new() -> Self {
    }

    fn status(&self) -> f64 {
        self.resources + self.load
    }

    fn is_peer(&self, other: Self) -> bool {
        (other.status() - self.status()).abs() /
            cmp::max(self.status(), other.status()) <= L
    }

    fn is_auth(&self, other: Self) -> bool {
        (other.status() - self.status()) /
            cmp::max(self.status(), other.status()) > L
    }

    // TODO: These require random numbers...
    fn birth(&self) -> bool { false }
    fn death(&self) -> bool { false }
}
