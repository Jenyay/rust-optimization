
use super::super::*;

use num::Float;
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

impl<G: Float> Mutation<Vec<G>> for RandomChromosomesMutation {
    fn mutation(&mut self, chromosomes: &mut Vec<G>) {
        let mutate = Uniform::new(0.0, 100.0);

        for n in 0..chromosomes.len() {
            if mutate.sample(&mut self.random) < self.probability {
                chromosomes[n] = super::mutation_bitwise_float(chromosomes[n], self.gene_count);
            }
        }
    }
}
