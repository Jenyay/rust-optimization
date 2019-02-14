pub mod vec_float;

use std::mem;

use num::{Float, Num, NumCast};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

pub trait NumCross<G: NumCast + Num + Clone> {
    fn cross(&mut self, gene: &Vec<G>) -> Vec<G>;
}

pub struct CrossMean;

pub struct FloatCrossGeometricMean;

pub struct CrossBitwise {
    random: ThreadRng,
}

impl CrossMean {
    pub fn new() -> Self {
        Self {}
    }
}

impl<G: NumCast + Num + Clone> NumCross<G> for CrossMean {
    fn cross(&mut self, gene: &Vec<G>) -> Vec<G> {
        assert!(gene.len() >= 2);
        let mut result = gene[0].clone();
        for n in 1..gene.len() {
            result = result + gene[n].clone();
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

impl<G: Float> NumCross<G> for FloatCrossGeometricMean {
    fn cross(&mut self, gene: &Vec<G>) -> Vec<G> {
        assert!(gene.len() >= 2);
        let mut result = gene[0];
        for n in 1..gene.len() {
            result = result * gene[n];
        }

        result = result.powf(G::one() / G::from(gene.len()).unwrap());
        vec![result]
    }
}

impl CrossBitwise {
    pub fn new() -> Self {
        let random = rand::thread_rng();
        Self { random }
    }
}

impl NumCross<f64> for CrossBitwise {
    fn cross(&mut self, gene: &Vec<f64>) -> Vec<f64> {
        assert_eq!(gene.len(), 2);
        let size = mem::size_of_val(&gene[0]) * 8;
        let between = Uniform::new(1, size);

        let pos = between.sample(&mut self.random);

        let parent_1 = gene[0].to_bits();
        let parent_2 = gene[1].to_bits();

        let child_bits = cross_u64(parent_1, parent_2, pos);
        vec![f64::from_bits(child_bits)]
    }
}

impl NumCross<f32> for CrossBitwise {
    fn cross(&mut self, gene: &Vec<f32>) -> Vec<f32> {
        assert_eq!(gene.len(), 2);
        let size = mem::size_of_val(&gene[0]) * 8;
        let between = Uniform::new(1, size);

        let pos = between.sample(&mut self.random);

        let parent_1 = gene[0].to_bits();
        let parent_2 = gene[1].to_bits();

        let child_bits = cross_u32(parent_1, parent_2, pos);
        vec![f32::from_bits(child_bits)]
    }
}

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
