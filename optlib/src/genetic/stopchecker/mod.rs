use super::*;


pub struct MaxIterations {
    max_iter: usize,
}

impl MaxIterations {
    pub fn new(max_iter: usize) -> Self {
        MaxIterations { max_iter }
    }
}

impl<T: Clone> StopChecker<T> for MaxIterations {
    fn can_stop(&mut self, population: &Population<T>) -> bool {
        population.get_iteration() >= self.max_iter
    }
}
