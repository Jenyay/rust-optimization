use std::mem;

use num::{Float, Num, NumCast};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use super::Cross;


// VecCrossAllGenes
pub struct VecCrossAllGenes<G: Clone> {
    single_cross: Box<dyn Cross<G>>,
}

pub struct CrossMean;

pub struct FloatCrossGeometricMean;

pub struct CrossBitwise {
    random: ThreadRng,
}

pub struct FloatCrossExp {
    random: ThreadRng,
}

impl CrossMean {
    pub fn new() -> Self {
        Self {}
    }
}

impl<G: NumCast + Num + Clone> Cross<G> for CrossMean {
    fn cross(&mut self, parents_genes: &[&G]) -> Vec<G> {
        assert!(parents_genes.len() >= 2);
        let mut result = parents_genes[0].clone();
        for n in 1..parents_genes.len() {
            result = result + parents_genes[n].clone();
        }

        result = result / G::from(parents_genes.len()).unwrap();
        vec![result]
    }
}

impl FloatCrossGeometricMean {
    pub fn new() -> Self {
        Self {}
    }
}

impl<G: Float> Cross<G> for FloatCrossGeometricMean {
    fn cross(&mut self, parents_genes: &[&G]) -> Vec<G> {
        assert!(parents_genes.len() >= 2);
        let mut result = *parents_genes[0];
        for n in 1..parents_genes.len() {
            result = result * (*parents_genes[n]);
        }

        result = result.powf(G::one() / G::from(parents_genes.len()).unwrap());
        vec![result]
    }
}

impl CrossBitwise {
    pub fn new() -> Self {
        let random = rand::thread_rng();
        Self { random }
    }
}

impl Cross<f64> for CrossBitwise {
    fn cross(&mut self, parents_genes: &[&f64]) -> Vec<f64> {
        assert_eq!(parents_genes.len(), 2);
        let size = mem::size_of_val(parents_genes[0]) * 8;
        let between = Uniform::new(1, size);
        let pos = between.sample(&mut self.random);

        vec![cross_f64(*parents_genes[0], *parents_genes[1], pos)]
    }
}

impl Cross<f32> for CrossBitwise {
    fn cross(&mut self, parents_genes: &[&f32]) -> Vec<f32> {
        assert_eq!(parents_genes.len(), 2);
        let size = mem::size_of_val(parents_genes[0]) * 8;
        let between = Uniform::new(1, size);
        let pos = between.sample(&mut self.random);

        vec![cross_f32(*parents_genes[0], *parents_genes[1], pos)]
    }
}

impl<G: Clone> VecCrossAllGenes<G> {
    pub fn new(single_cross: Box<dyn Cross<G>>) -> Self {
        Self { single_cross }
    }
}

impl<G: Clone> Cross<Vec<G>> for VecCrossAllGenes<G> {
    fn cross(&mut self, parents: &[&Vec<G>]) -> Vec<Vec<G>> {
        assert!(parents.len() == 2);

        let parent_1 = parents[0];
        let parent_2 = parents[1];

        let gene_count = parent_1.len();
        let mut child = vec![];

        for n in 0..gene_count {
            let mut new_gene = self.single_cross.cross(vec![&parent_1[n], &parent_2[n]].as_slice());
            child.append(&mut new_gene);
        }
        vec![child]
    }
}

impl FloatCrossExp {
    pub fn new() -> Self {
        let random = rand::thread_rng();
        Self { random }
    }
}

impl<G: Float> Cross<G> for FloatCrossExp {
    fn cross(&mut self, parents_genes: &[&G]) -> Vec<G> {
        assert_eq!(parents_genes.len(), 2);
        // mantissa: u64, exponent: i16, sign: i8
        let (mantissa_1, exponent_1, sign_1) = parents_genes[0].integer_decode();
        let (mantissa_2, exponent_2, sign_2) = parents_genes[1].integer_decode();

        let mantissa_size = mem::size_of_val(&mantissa_1) * 8;
        let exponent_size = mem::size_of_val(&exponent_1) * 8;

        let mantissa_between = Uniform::new(1, mantissa_size);
        let exponent_between = Uniform::new(1, exponent_size);

        let mantissa_pos = mantissa_between.sample(&mut self.random);
        let exponent_pos = exponent_between.sample(&mut self.random);

        let mantissa_child = cross_u64(mantissa_1, mantissa_2, mantissa_pos);
        let exponent_child = cross_i16(exponent_1, exponent_2, exponent_pos);

        let sign_child = match Uniform::new_inclusive(0i8, 1i8).sample(&mut self.random) {
            0 => sign_1,
            1 => sign_2,
            _ => panic!("Invalid random value in FloatCrossExp"),
        };

        vec![G::from(sign_child).unwrap() * G::from(mantissa_child).unwrap() * G::from(exponent_child).unwrap().exp2()]
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

pub fn cross_f32(parent_1: f32, parent_2: f32, pos: usize) -> f32 {
    let parent_1_bits = parent_1.to_bits();
    let parent_2_bits = parent_2.to_bits();

    let child_bits = cross_u32(parent_1_bits, parent_2_bits, pos);
    f32::from_bits(child_bits)
}

pub fn cross_f64(parent_1: f64, parent_2: f64, pos: usize) -> f64 {
    let parent_1_bits = parent_1.to_bits();
    let parent_2_bits = parent_2.to_bits();

    let child_bits = cross_u64(parent_1_bits, parent_2_bits, pos);
    f64::from_bits(child_bits)
}
