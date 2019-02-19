//! The module with Creators for the case when chromosomes are Vec<Num>.
//! The Creators used to create the first generation of individuals.

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use num::NumCast;

use super::super::*;

/// Creator to initialize population by individuals with random chromosomes in the preset
/// intervals.
pub struct RandomCreator<G: Clone + NumCast> {
    population_size: usize,
    intervals: Vec<(G, G)>,
    random: ThreadRng,
}

impl<G: Clone + NumCast> RandomCreator<G> {
    pub fn new(population_size: usize, intervals: Vec<(G, G)>) -> Self {
        let random = rand::thread_rng();
        Self {
            population_size,
            intervals,
            random,
        }
    }
}

impl<G: Clone + NumCast> Creator<Vec<G>> for RandomCreator<G> {
    fn create(&mut self) -> Vec<Vec<G>> {
        let mut population = Vec::with_capacity(self.population_size * 2);
        let chromo_count = self.intervals.len();

        for _ in 0..self.population_size {
            let mut chromo = Vec::with_capacity(chromo_count);
            for interval in &self.intervals {
                let between = Uniform::new_inclusive(interval.0.to_f64().unwrap(), interval.1.to_f64().unwrap());
                chromo.push(G::from(between.sample(&mut self.random)).unwrap());
            }

            population.push(chromo);
        }

        population
    }
}
