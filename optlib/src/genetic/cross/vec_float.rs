use super::super::*;
use super::NumCross;

use num::Float;

// VecCrossAllGenes
pub struct VecCrossAllGenes<G: Float> {
    single_cross: Box<dyn NumCross<G>>,
}

impl<G: Float> VecCrossAllGenes<G> {
    pub fn new(single_cross: Box<dyn NumCross<G>>) -> Self {
        Self { single_cross }
    }
}

impl<G: Float> Cross<Vec<G>> for VecCrossAllGenes<G> {
    fn cross(&mut self, parents: &[&Vec<G>]) -> Vec<Vec<G>> {
        assert!(parents.len() == 2);

        let parent_1 = parents[0];
        let parent_2 = parents[1];

        let gene_count = parent_1.len();
        let mut child = vec![];

        for n in 0..gene_count {
            let mut new_gene = self.single_cross.cross(&vec![parent_1[n], parent_2[n]]);
            child.append(&mut new_gene);
        }
        vec![child]
    }
}
