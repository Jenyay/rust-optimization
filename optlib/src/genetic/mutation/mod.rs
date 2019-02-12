pub mod vec_float;

use num::Float;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::mem;

pub trait FloatMutation<G: Float> {
    fn mutation(&mut self, gene: G) -> G;
}

pub struct BitwiseFloatMutation {
    random: ThreadRng,
    change_gene_count: usize,
}

impl BitwiseFloatMutation {
    pub fn new(change_gene_count: usize) -> Self {
        let random = rand::thread_rng();
        Self {
            random,
            change_gene_count,
        }
    }
}

impl<G: Float> FloatMutation<G> for BitwiseFloatMutation {
    fn mutation(&mut self, gene: G) -> G {
        let size = mem::size_of_val(&gene) * 8;
        let between = Uniform::new(0, size);

        match size {
            64 => {
                let mut bit_value = gene.to_f64().unwrap().to_bits();
                for _ in 0..self.change_gene_count {
                    let pos = between.sample(&mut self.random);
                    bit_value ^= 1 << pos;
                }
                G::from(f64::from_bits(bit_value)).unwrap()
            }
            32 => {
                let mut bit_value = gene.to_f32().unwrap().to_bits();
                for _ in 0..self.change_gene_count {
                    let pos = between.sample(&mut self.random);
                    bit_value ^= 1 << pos;
                }
                G::from(f32::from_bits(bit_value)).unwrap()
            }

            _ => panic!("Unkwnown float type in mutation"),
        }
    }
}
