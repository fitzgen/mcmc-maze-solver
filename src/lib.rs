mod maze;
pub use maze::*;

mod path;
pub use path::*;

use rand::prelude::*;
use std::future::Future;

pub struct Mcmc<T> {
    current: T,
    current_cost: f64,
}

impl<T> Mcmc<T>
where
    T: ErgodicAndSymmetric + Clone,
{
    pub fn new(initial: T, mut cost: impl FnMut(&T) -> f64) -> Self {
        let current = initial;
        let current_cost = cost(&current);
        Mcmc {
            current,
            current_cost,
        }
    }

    pub fn current(&self) -> &T {
        &self.current
    }

    pub fn into_current(self) -> T {
        self.current
    }

    pub fn tick(
        &mut self,
        rng: &mut impl Rng,
        mut cost: impl FnMut(&T) -> f64,
    ) -> Option<(&T, f64)> {
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

pub async fn solve_maze(
    rng: &mut impl Rng,
    maze: &Maze,
    start_cell: Cell,
    destination_cell: Cell,
    mut each_tick: impl FnMut(Option<(&Maze, &Path)>) -> Option<Box<dyn Future<Output = ()> + Unpin>>,
) -> Path {
    let mut mcmc = Mcmc::new(Path { moves: vec![] }, |path| {
        let end_of_path = maze
            .follow_path(start_cell, path)
            .last()
            .unwrap_or(start_cell);
        maze.bird_flight_distance(end_of_path, destination_cell)
    });

    loop {
        let mut end_of_path = None;

        if let Some((path, _)) = mcmc.tick(rng, |path| {
            end_of_path = Some(
                maze.follow_path(start_cell, path)
                    .last()
                    .unwrap_or(start_cell),
            );
            maze.bird_flight_distance(end_of_path.unwrap(), destination_cell)
        }) {
            if end_of_path == Some(destination_cell) {
                return mcmc.into_current();
            } else {
                if let Some(fut) = each_tick(Some((&maze, path))) {
                    fut.await;
                }
                continue;
            }
        }

        if let Some(fut) = each_tick(None) {
            fut.await;
        }
    }
}
