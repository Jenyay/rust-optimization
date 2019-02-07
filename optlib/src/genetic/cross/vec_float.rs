use super::super::*;

use num::Float;

// FuncCross
pub struct FuncCross<G: Float> {
    cross_function: fn(&Vec<G>) -> Vec<G>,
}

impl<G: Float> FuncCross<G> {
    pub fn new(cross_function: fn(&Vec<G>) -> Vec<G>) -> Self {
        Self { cross_function }
    }
}

impl<G: Float> Cross<Vec<G>> for FuncCross<G> {
    fn cross(&self, parents: &Vec<&Vec<G>>) -> Vec<Vec<G>> {
        assert!(parents.len() == 2);

        let chromo_count = parents[0].len();
        let mut children: Vec<Vec<G>> = Vec::with_capacity(chromo_count);
        children.push(vec![]);

        for n in 0..chromo_count {
            let mut new_chromo = (self.cross_function)(&vec![parents[0][n], parents[1][n]]);
            children[0].append(&mut new_chromo);
        }

        children
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
