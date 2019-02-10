use super::super::*;

use std::mem;

use num::Float;
use rand::distributions::{Distribution, Uniform};

// VecCrossAllGenes
pub struct VecCrossAllGenes<G: Float> {
    gene_cross_function: Box<dyn FnMut(&Vec<G>) -> Vec<G>>,
}

impl<G: Float> VecCrossAllGenes<G> {
    pub fn new(gene_cross_function: Box<FnMut(&Vec<G>) -> Vec<G>>) -> Self {
        Self {
            gene_cross_function,
        }
    }
}

impl<G: Float> Cross<Vec<G>> for VecCrossAllGenes<G> {
    fn cross(&mut self, parents: &Vec<&Vec<G>>) -> Vec<Vec<G>> {
        assert!(parents.len() == 2);

        let parent_1 = parents[0];
        let parent_2 = parents[1];

        let gene_count = parent_1.len();
        let mut child = vec![];

        for n in 0..gene_count {
            let mut new_gene = (self.gene_cross_function)(&vec![parent_1[n], parent_2[n]]);
            child.append(&mut new_gene);
        }
        vec![child]
    }
}

pub fn cross_middle<G: Float>(gene: &Vec<G>) -> Vec<G> {
    assert!(gene.len() >= 2);
    let mut result = gene[0].clone();
    for n in 1..gene.len() {
        result = result + gene[n].clone();
    }

    result = result / G::from(gene.len()).unwrap();
    vec![result]
}

pub fn cross_bitwise(gene: &Vec<f64>) -> Vec<f64> {
    assert_eq!(gene.len(), 2);
    let mut random = rand::thread_rng();
    let size = mem::size_of_val(&gene) * 8;
    let between = Uniform::new(1, size);

    let pos = between.sample(&mut random);

    let parent_1 = gene[0].to_bits();
    let parent_2 = gene[1].to_bits();

    let mask_parent_1 = (!(0 as u64)) << pos;
    let mask_parent_2 = (!(0 as u64)) >> (size - pos);

    let child_bits = (parent_1 & mask_parent_1) | (parent_2 & mask_parent_2);

    vec![f64::from_bits(child_bits)]
}
