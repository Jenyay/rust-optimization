pub mod vec_float;

use std::mem;

use num::Float;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

pub trait FloatCross<G: Float> {
    fn cross(&mut self, gene: &Vec<G>) -> Vec<G>;
}

pub struct FloatCrossMean;

pub struct FloatCrossGeometricMean;

pub struct FloatCrossBitwise {
    random: ThreadRng,
}

impl FloatCrossMean {
    pub fn new() -> Self {
        Self {}
    }
}

impl<G: Float> FloatCross<G> for FloatCrossMean {
    fn cross(&mut self, gene: &Vec<G>) -> Vec<G> {
        assert!(gene.len() >= 2);
        let mut result = gene[0];
        for n in 1..gene.len() {
            result = result + gene[n];
        }

        result = result / G::from(gene.len()).unwrap();
        vec![result]
    }
}

impl FloatCrossGeometricMean {
    pub fn new() -> Self {
        Self {}
    }
}

impl<G: Float> FloatCross<G> for FloatCrossGeometricMean {
    fn cross(&mut self, gene: &Vec<G>) -> Vec<G> {
        assert!(gene.len() >= 2);
        let mut result = gene[0];
        for n in 1..gene.len() {
            result = result * gene[n];
        }

        result = result.powf(G::from(1.0).unwrap() / G::from(gene.len()).unwrap());
        vec![result]
    }
}

impl FloatCrossBitwise {
    pub fn new() -> Self {
        let random = rand::thread_rng();
        Self { random }
    }
}

impl<G: Float> FloatCross<G> for FloatCrossBitwise {
    fn cross(&mut self, gene: &Vec<G>) -> Vec<G> {
        assert_eq!(gene.len(), 2);
        let size = mem::size_of::<G>() * 8;
        let between = Uniform::new(1, size);

        let pos = between.sample(&mut self.random);

        match size {
            32 => {
                let parent_1 = gene[0].to_f32().unwrap().to_bits();
                let parent_2 = gene[1].to_f32().unwrap().to_bits();

                let child_bits = cross_u32(parent_1, parent_2, pos);
                vec![G::from(f32::from_bits(child_bits)).unwrap()]
            }
            64 => {
                let parent_1 = gene[0].to_f64().unwrap().to_bits();
                let parent_2 = gene[1].to_f64().unwrap().to_bits();

                let child_bits = cross_u64(parent_1, parent_2, pos);
                vec![G::from(f64::from_bits(child_bits)).unwrap()]
            }
            _ => panic!("Unknown gene type in cross"),
        }
    }
}


// pub trait CrossPrimitive {
//     fn cross<T>(parent_1: T, parent_2: T, pos: usize) -> T;
// }
//
// impl CrossPrimitive for u64 {
//     fn cross<u64>(parent_1: u64, parent_2: u64, pos: usize) -> u64 {
//         let size = mem::size_of_val(&parent_1) * 8;
//         let mask_parent_1 = !0u64 << pos;
//         let mask_parent_2 = !0u64 >> (size - pos);
//         (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
//     }
// }

pub fn cross_u64(parent_1: u64, parent_2: u64, pos: usize) -> u64 {
    let size = mem::size_of_val(&parent_1) * 8;
    let mask_parent_1 = !0u64 << pos;
    let mask_parent_2 = !0u64 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

pub fn cross_u32(parent_1: u32, parent_2: u32, pos: usize) -> u32 {
    let size = mem::size_of_val(&parent_1) * 8;
    let mask_parent_1 = !0u32 << pos;
    let mask_parent_2 = !0u32 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

pub fn cross_i16(parent_1: i16, parent_2: i16, pos: usize) -> i16 {
    let size = mem::size_of_val(&parent_1) * 8;
    let mask_parent_1 = !0i16 << pos;
    let mask_parent_2 = !0i16 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

pub fn cross_u16(parent_1: u16, parent_2: u16, pos: usize) -> u16 {
    let size = mem::size_of_val(&parent_1) * 8;
    let mask_parent_1 = !0u16 << pos;
    let mask_parent_2 = !0u16 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

pub fn cross_i8(parent_1: i8, parent_2: i8, pos: usize) -> i8 {
    let size = mem::size_of_val(&parent_1) * 8;
    let mask_parent_1 = !0i8 << pos;
    let mask_parent_2 = !0i8 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}

pub fn cross_u8(parent_1: u8, parent_2: u8, pos: usize) -> u8 {
    let size = mem::size_of_val(&parent_1) * 8;
    let mask_parent_1 = !0u8 << pos;
    let mask_parent_2 = !0u8 >> (size - pos);
    (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2)
}
