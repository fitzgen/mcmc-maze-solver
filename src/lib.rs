mod maze;
pub use maze::*;

mod path;
pub use path::*;

use rand::prelude::*;

pub struct McmcMinimizer<T> {
    current: T,
    current_cost: f64,
}

impl<T> McmcMinimizer<T>
where
    T: ErgodicAndSymmetric + Clone,
{
    pub fn new(initial: T, cost: impl Fn(&T) -> f64) -> Self {
        let current = initial;
        let current_cost = cost(&current);
        McmcMinimizer {
            current,
            current_cost,
        }
    }

    pub fn current(&self) -> &T {
        &self.current
    }

    pub fn tick(&mut self, rng: &mut impl Rng, cost: impl Fn(&T) -> f64) -> Option<(&T, f64)> {
        let candidate = self.current.candidate(rng);
        let candidate_cost = cost(&candidate);

        const B: f64 = 0.5;
        let accept_probability = partial_min_max::min(
            1.0,
            std::f64::consts::E.powf(-B * candidate_cost / self.current_cost),
        );

        if rng.gen::<f64>() <= accept_probability {
            self.current = candidate;
            self.current_cost = candidate_cost;
            Some((&self.current, self.current_cost))
        } else {
            None
        }
    }
}

pub trait ErgodicAndSymmetric {
    fn candidate(&self, rng: &mut impl Rng) -> Self;
}
