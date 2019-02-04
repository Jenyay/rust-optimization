pub mod vec_float;

use std::mem;

use rand::distributions::{Distribution, Uniform};


pub fn mutation_f64(value: f64, count: usize) -> f64 {
    let size = mem::size_of_val(&value) * 8;
    let between = Uniform::new(0, size);
    let mut rng = rand::thread_rng();

    let mut bit_value = value.to_bits();
    for _ in 0..count {
        let pos = between.sample(&mut rng);
        bit_value ^= 1 << pos;
    }

    f64::from_bits(bit_value)
}
