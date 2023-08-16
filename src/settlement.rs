use crate::household::Household;

pub struct Settlement {
    // TODO - implement this as an iterator
    pub households: Vec<Household>,
}

impl Settlement {
    pub fn new(initial_households: usize) -> Self {
        let households = (0..initial_households)
            .map(|_| Household::default())
            .collect();

        Settlement {
            households,
        }
    }
}
