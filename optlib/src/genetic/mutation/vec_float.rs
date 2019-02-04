use super::super::*;

use rand::distributions::{Distribution, Uniform};
use rand::rngs;

pub struct RandomChromosomesMutation {
    probability: f64,
    gene_count: usize,
    random: rngs::ThreadRng,
}

impl RandomChromosomesMutation {
    pub fn new(probability: f64, gene_count: usize) -> Self {
        let random = rand::thread_rng();
        Self {
            probability,
            gene_count,
            random,
        }
    }
}

impl Mutation<Vec<f64>> for RandomChromosomesMutation {
    fn mutation(&mut self, chromosomes: &mut Vec<f64>) {
        let mutate = Uniform::new(0.0, 100.0);

        for n in 0..chromosomes.len() {
            if mutate.sample(&mut self.random) < self.probability {
                chromosomes[n] = super::mutation_f64(chromosomes[n], self.gene_count);
            }
        }
    }
}
