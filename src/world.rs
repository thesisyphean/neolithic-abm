struct World {
    settlements: Vec<_>
}

impl World {
    fn new(n: usize) -> Self {
        grid
    }
}

enum Cell {
    Empty,
    Claimed,
    Settled(Settlement),
}

struct Settings {
    // initial setup
    iterations: u32,
    initial_households: u32,
    initial_settlements: u32,

    L: f64,

    // migration, reproduction, and death
    years_per_move: u32,
    birth_rate: f64,
    death_rate: f64,

    beta: f64,
    m: f64,
    mutation_rate: f64,

    // cultural algorithm
    influence_rate: f64,
    influence_frequency: u32,
    conformity_base: f64,
    conformity_limit: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            iterations: 10_000,
        }
    }
}
