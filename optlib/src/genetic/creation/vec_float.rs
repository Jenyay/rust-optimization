use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use super::super::*;

/// RandomCreator
pub struct RandomCreator {
    population_size: usize,
    intervals: Vec<(f64, f64)>,
    random: ThreadRng,
}

impl RandomCreator {
    pub fn new(population_size: usize, intervals: Vec<(f64, f64)>) -> Self {
        let random = rand::thread_rng();
        Self {
            population_size,
            intervals,
            random,
        }
    }
}

impl Creator<Vec<f64>> for RandomCreator {
    fn create(&mut self) -> Vec<Vec<f64>> {
        let mut population = Vec::with_capacity(self.population_size * 2);
        let chromo_count = self.intervals.len();

        for _ in 0..self.population_size {
            let mut chromo = Vec::with_capacity(chromo_count);
            for interval in &self.intervals {
                let between = Uniform::new(interval.0, interval.1);
                chromo.push(between.sample(&mut self.random));
            }

            population.push(chromo);
        }

        population
    }
}
