//! The module with pairing algorithm traits. The pairing algorithm selects individuals for crossing.

use super::*;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// Pairing algorithm which select random individuals for crossing.
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

/// Algorithm of tournament.
///
/// Every individual to cross must be best of N others random individuals.
pub struct Tournament {
    partners_count: usize,
    families_count: usize,
    rounds_count: usize,
    random: ThreadRng,
}

impl Tournament {
    /// Constructor.
    ///
    /// # Parameters
    /// * `partners_count` - partners count for crossing.
    /// * `families_count` - families count for crossing.
    /// * `rounds_count` - How many times should win an individual to cross.
    pub fn new(partners_count: usize, families_count: usize, rounds_count: usize) -> Self {
        let random = rand::thread_rng();
        Self {
            partners_count,
            families_count,
            rounds_count,
            random,
        }
    }
}

impl<T: Clone> Pairing<T> for Tournament {
    fn get_pairs(&mut self, population: &Population<T>) -> Vec<Vec<usize>> {
        let mut pairs: Vec<Vec<usize>> = Vec::with_capacity(self.families_count);
        let between = Uniform::new(0, population.len());

        // Loop for families count
        for _ in 0..self.families_count {
            let mut family: Vec<usize> = Vec::with_capacity(self.partners_count);

            // Loop for partner
            for _ in 0..self.partners_count {
                let mut best_ind_index = between.sample(&mut self.random);

                // Loop for tournaments rounds
                for _ in 0..self.rounds_count {
                    let challenger = between.sample(&mut self.random);
                    if population[challenger].get_goal() < population[best_ind_index].get_goal() {
                        best_ind_index = challenger;
                    }
                }
                family.push(best_ind_index);
            }

            pairs.push(family);
        }

        pairs
    }
}
