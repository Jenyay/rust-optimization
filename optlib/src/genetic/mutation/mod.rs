//! The module with most usable algorithms of mutations for various types.
//! The module contains struct which implements the `Mutation` trait to mutate
//! chromosomes various types.

use crate::genetic::Mutation;
use rand::distributions::{Distribution, Uniform};
use rand::rngs;
use rand::rngs::ThreadRng;
use std::mem;

/// The struct to change random bits in the chromosomes.
pub struct BitwiseMutation {
    random: ThreadRng,
    change_gene_count: usize,
}

/// Mutation for chromosomes of Vec<G>, where G - type of single gene.
pub struct VecMutation<G> {
    probability: f64,
    random: rngs::ThreadRng,
    single_mutation: Box<dyn Mutation<G>>,
}

impl BitwiseMutation {
    /// Constructor
    ///
    /// # Parameters
    /// * `change_gene_count` - how many bits will changed by algorithm during mutation.
    pub fn new(change_gene_count: usize) -> Self {
        let random = rand::thread_rng();
        Self {
            random,
            change_gene_count,
        }
    }
}

impl Mutation<f32> for BitwiseMutation {
    fn mutation(&mut self, gene: &f32) -> f32 {
        let size = mem::size_of::<f32>() * 8;
        let between = Uniform::new(0, size);

        let mut bit_value = gene.to_bits();
        for _ in 0..self.change_gene_count {
            let pos = between.sample(&mut self.random);
            bit_value ^= 1 << pos;
        }
        f32::from_bits(bit_value)
    }
}

impl Mutation<f64> for BitwiseMutation {
    fn mutation(&mut self, gene: &f64) -> f64 {
        let size = mem::size_of::<f64>() * 8;
        let between = Uniform::new(0, size);

        let mut bit_value = gene.to_bits();
        for _ in 0..self.change_gene_count {
            let pos = between.sample(&mut self.random);
            bit_value ^= 1 << pos;
        }
        f64::from_bits(bit_value)
    }
}

impl<G> VecMutation<G> {
    /// Constructor
    ///
    /// # Parameters
    /// * `probability` - probability of mutation of single gene.
    /// * `single_mutation` - trait object with mutation algorithm for single gene.
    pub fn new(probability: f64, single_mutation: Box<dyn Mutation<G>>) -> Self {
        let random = rand::thread_rng();
        Self {
            probability,
            random,
            single_mutation,
        }
    }
}

impl<G: Clone> Mutation<Vec<G>> for VecMutation<G> {
    fn mutation(&mut self, chromosomes: &Vec<G>) -> Vec<G> {
        let mutate = Uniform::new(0.0, 100.0);
        let mut result = Vec::with_capacity(chromosomes.len());

        for chromo in chromosomes {
            if mutate.sample(&mut self.random) < self.probability {
                result.push(self.single_mutation.mutation(&chromo));
            } else {
                result.push(chromo.clone());
            }
        }

        result
    }
}
