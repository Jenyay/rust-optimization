//! The module with Creators for the case when chromosomes are Vec<Num> (vector of the genes).
//! Gene - single value in chromosome.
//! The Creators used to create the first generation of individuals.

use num::NumCast;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use crate::genetic::Creator;

/// Creator to initialize population by individuals with random genes in the preset
/// intervals.
/// `G` - type of genes. Chromosome is vector of the genes.
pub struct RandomCreator<G: NumCast + PartialOrd> {
    population_size: usize,
    intervals: Vec<(G, G)>,
    random: ThreadRng,
}

impl<G: NumCast + PartialOrd> RandomCreator<G> {
    /// Constructor.
    ///
    /// `G` - type of genes. Chromosome is vector of the genes.
    ///
    /// # Parameters
    /// * `population_size` - individuals count in the first generation.
    /// * `intervals` - vector of the tuples (minval, maxval). Length of the `intervals` must
    /// equal genes count in the chromosome. The values of `minval` and `maxval` will be included
    /// in random interval.
    pub fn new(population_size: usize, intervals: Vec<(G, G)>) -> Self {
        assert!(population_size > 0);
        assert!(!intervals.is_empty());
        for interval in &intervals {
            assert!(interval.0 < interval.1);
        }

        let random = rand::thread_rng();
        Self {
            population_size,
            intervals,
            random,
        }
    }
}

impl<G: NumCast + PartialOrd> Creator<Vec<G>> for RandomCreator<G> {
    fn create(&mut self) -> Vec<Vec<G>> {
        let mut population = Vec::with_capacity(self.population_size * 2);
        let chromo_count = self.intervals.len();

        for _ in 0..self.population_size {
            let mut chromo = Vec::with_capacity(chromo_count);
            for interval in &self.intervals {
                let between = Uniform::new_inclusive(
                    interval.0.to_f64().unwrap(),
                    interval.1.to_f64().unwrap(),
                );
                chromo.push(G::from(between.sample(&mut self.random)).unwrap());
            }

            population.push(chromo);
        }

        population
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_size() {
        let population_size = 10;
        let intervals = vec![(0.0, 1.0)];
        let mut creator = RandomCreator::new(population_size, intervals);

        let chromosomes = creator.create();
        assert_eq!(chromosomes.len(), population_size);
    }

    #[test]
    fn test_intervals() {
        let population_size = 1000;
        let intervals = vec![(0.0, 1.0), (-1.0, 1.0), (100.0, 110.0)];
        let mut creator = RandomCreator::new(population_size, intervals);

        let chromosomes: Vec<Vec<f64>> = creator.create();
        for chromosome in chromosomes {
            assert!(chromosome[0] >= 0.0);
            assert!(chromosome[0] <= 1.0);

            assert!(chromosome[1] >= -1.0);
            assert!(chromosome[1] <= 1.0);

            assert!(chromosome[2] >= 100.0);
            assert!(chromosome[2] <= 110.0);
        }
    }

    #[test]
    #[should_panic]
    fn empty_population() {
        let population_size = 0;
        let intervals = vec![(0.0, 1.0)];
        RandomCreator::new(population_size, intervals);
    }

    #[test]
    #[should_panic]
    fn empty_intervals() {
        let population_size = 10;
        let intervals: Vec<(f64, f64)> = Vec::new();
        RandomCreator::new(population_size, intervals);
    }

    #[test]
    #[should_panic]
    fn invalid_intervals_01() {
        let population_size = 10;
        let intervals = vec![(1.0, 0.0)];
        RandomCreator::new(population_size, intervals);
    }

    #[test]
    #[should_panic]
    fn invalid_intervals_02() {
        let population_size = 10;
        let intervals = vec![(0.0, 0.0)];
        RandomCreator::new(population_size, intervals);
    }

    #[test]
    #[should_panic]
    fn invalid_intervals_03() {
        let population_size = 10;
        let intervals = vec![(0.0, 1.0), (10.0, 0.0)];
        RandomCreator::new(population_size, intervals);
    }
}
