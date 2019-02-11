use super::super::*;


use num::Float;

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
