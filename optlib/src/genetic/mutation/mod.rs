pub mod vec_float;

use std::mem;
use num::Float;
use rand::distributions::{Distribution, Uniform};


pub fn mutation_bitwise_float<G: Float>(gene: G, count: usize) -> G {
    let size = mem::size_of_val(&gene) * 8;
    let between = Uniform::new(0, size);
    let mut rng = rand::thread_rng();

    match size {
        64 => {
            let mut bit_value = gene.to_f64().unwrap().to_bits();
            for _ in 0..count {
                let pos = between.sample(&mut rng);
                bit_value ^= 1 << pos;
            }
            G::from(f64::from_bits(bit_value)).unwrap()
        }
        32 => {
            let mut bit_value = gene.to_f32().unwrap().to_bits();
            for _ in 0..count {
                let pos = between.sample(&mut rng);
                bit_value ^= 1 << pos;
            }
            G::from(f32::from_bits(bit_value)).unwrap()
        }

        _ => panic!("Unkwnown float type in mutation"),
    }
}
