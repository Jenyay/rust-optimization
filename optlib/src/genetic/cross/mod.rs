pub mod vec_float;

use std::mem;

use num::Float;
use rand::distributions::{Distribution, Uniform};


pub fn cross_mean_float<G: Float>(gene: &Vec<G>) -> Vec<G> {
    assert!(gene.len() >= 2);
    let mut result = gene[0];
    for n in 1..gene.len() {
        result = result + gene[n];
    }

    result = result / G::from(gene.len()).unwrap();
    vec![result]
}

pub fn cross_geometric_mean_float<G: Float>(gene: &Vec<G>) -> Vec<G> {
    assert!(gene.len() >= 2);
    let mut result = gene[0];
    for n in 1..gene.len() {
        result = result * gene[n];
    }

    result = result.powf(G::from(1.0).unwrap() / G::from(gene.len()).unwrap());
    vec![result]
}

pub fn cross_bitwise_float<G: Float>(gene: &Vec<G>) -> Vec<G> {
    assert_eq!(gene.len(), 2);
    let mut random = rand::thread_rng();
    let size = mem::size_of_val(&gene[0]) * 8;
    let between = Uniform::new(1, size);

    let pos = between.sample(&mut random);

    match size {
        32 => {
            let parent_1 = gene[0].to_f32().unwrap().to_bits();
            let parent_2 = gene[1].to_f32().unwrap().to_bits();

            let mask_parent_1 = (!(0 as u32)) << pos;
            let mask_parent_2 = (!(0 as u32)) >> (size - pos);
            let child_bits = (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2);
            vec![G::from(f32::from_bits(child_bits)).unwrap()]
        }
        64 => {
            let parent_1 = gene[0].to_f64().unwrap().to_bits();
            let parent_2 = gene[1].to_f64().unwrap().to_bits();

            let mask_parent_1 = (!(0 as u64)) << pos;
            let mask_parent_2 = (!(0 as u64)) >> (size - pos);
            let child_bits = (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2);
            vec![G::from(f64::from_bits(child_bits)).unwrap()]
        }
         _ => panic!("Unknown gene type in cross")
    }


}
