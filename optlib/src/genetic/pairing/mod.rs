//! The module with pairing algorithm traits. The pairing algorithm selects individuals for crossing.

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use crate::Agent;
use crate::genetic:: {Pairing, Population};

/// Pairing algorithm which select random individuals for crossing.
pub struct RandomPairing {
    random: ThreadRng,
}

impl<T> Pairing<T> for RandomPairing {
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
    families_count: usize,
    partners_count: usize,
    rounds_count: usize,
    random: ThreadRng,
}

impl Tournament {
    /// Constructor.
    ///
    /// # Parameters
    /// * `families_count` - families count for crossing.
    pub fn new(families_count: usize) -> Self {
        let random = rand::thread_rng();
        Self {
            families_count,
            partners_count: 2,
            rounds_count: 1,
            random,
        }
    }

    /// Set partners count for every family. Tthe default is 2.
    pub fn partners_count<'a>(mut self, count: usize) -> Self {
        self.partners_count = count;
        self
    }

    /// How many competitors should an individual win? The default is 1
    pub fn rounds_count<'a>(mut self, count: usize) -> Self {
        self.rounds_count = count;
        self
    }
}

impl<T> Pairing<T> for Tournament {
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
