pub mod vec_float;

use num::{Num, NumCast};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::mem;

pub trait NumMutation<G: NumCast + Num + Clone> {
    fn mutation(&mut self, gene: G) -> G;
}

pub struct BitwiseMutation {
    random: ThreadRng,
    change_gene_count: usize,
}

impl BitwiseMutation {
    pub fn new(change_gene_count: usize) -> Self {
        let random = rand::thread_rng();
        Self {
            random,
            change_gene_count,
        }
    }
}


impl NumMutation<f32> for BitwiseMutation {
    fn mutation(&mut self, gene: f32) -> f32 {
        let size = mem::size_of_val(&gene) * 8;
        let between = Uniform::new(0, size);

        let mut bit_value = gene.to_bits();
        for _ in 0..self.change_gene_count {
            let pos = between.sample(&mut self.random);
            bit_value ^= 1 << pos;
        }
        f32::from_bits(bit_value)
    }
}

impl NumMutation<f64> for BitwiseMutation {
    fn mutation(&mut self, gene: f64) -> f64 {
        let size = mem::size_of_val(&gene) * 8;
        let between = Uniform::new(0, size);

        let mut bit_value = gene.to_bits();
        for _ in 0..self.change_gene_count {
            let pos = between.sample(&mut self.random);
            bit_value ^= 1 << pos;
        }
        f64::from_bits(bit_value)
    }
}
