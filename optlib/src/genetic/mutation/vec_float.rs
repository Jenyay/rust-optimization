use super::super::*;
use super::*;

use num::Float;
use rand::distributions::{Distribution, Uniform};
use rand::rngs;

pub struct RandomChromosomesMutation<G: Float> {
    probability: f64,
    random: rngs::ThreadRng,
    single_mutation: Box<dyn NumMutation<G>>,
}

impl<G: Float> RandomChromosomesMutation<G> {
    pub fn new(probability: f64, single_mutation: Box<dyn NumMutation<G>>) -> Self {
        let random = rand::thread_rng();
        Self {
            probability,
            random,
            single_mutation,
        }
    }
}

impl<G: Float> Mutation<Vec<G>> for RandomChromosomesMutation<G> {
    fn mutation(&mut self, chromosomes: &mut Vec<G>) {
        let mutate = Uniform::new(0.0, 100.0);

        for n in 0..chromosomes.len() {
            if mutate.sample(&mut self.random) < self.probability {
                chromosomes[n] = self.single_mutation.mutation(chromosomes[n]);
            }
        }
    }
}
