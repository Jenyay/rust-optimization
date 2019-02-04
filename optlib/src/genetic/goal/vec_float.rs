use num::Float;
use super::super::*;


pub struct GoalFromFunction<G: Float> {
    function: fn(&Vec<G>) -> f64,
}

impl<G: Float> GoalFromFunction<G> {
    pub fn new(function: fn(&Vec<G>) -> f64) -> Self {
        Self { function, }
    }
}

impl<G: Float> Goal<Vec<G>> for GoalFromFunction<G> {
    fn get(&self, chromosomes: &Vec<G>) -> f64 {
        (self.function)(chromosomes)
    }
}
