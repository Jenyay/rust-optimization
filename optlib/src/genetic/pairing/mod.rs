//! The module with pairing algorithm traits. The pairing algorithm selects individuals for crossing.

use super::*;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// The struct with pairing algorithm which select random individuals for crossing.
pub struct RandomPairing {
    random: ThreadRng,
}

impl<T: Clone> Pairing<T> for RandomPairing {
    fn get_pairs(&mut self, population: &Population<T>) -> Vec<Vec<usize>> {
        let mut pairs: Vec<Vec<usize>> = vec![];

        let between = Uniform::new(0, population.len());
        let count = population.len() / 2;
        for _ in 0..count {
            let first = between.sample(&mut self.random);
            let second = between.sample(&mut self.random);
            let pair = vec![first, second];
            pairs.push(pair);
        }

        pairs
    }
}

impl RandomPairing {
    /// Constructor.
    pub fn new() -> Self {
        let random = rand::thread_rng();
        Self { random }
    }
}
