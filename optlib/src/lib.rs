extern crate num;

pub mod genetic;
pub mod testfunctions;

/// Common Optimizer trait.
pub trait Optimizer<T> {
    fn find_min(&mut self) -> Option<(T, f64)>;
}

// pub trait AlgorithmWithAgents<T, A: Agent<T>> {
//     fn get_agents(&self) -> Vec<Box<A>>;
// }

pub trait Agent<T> {
    fn get_goal(&self) -> f64;
    fn get_parameter(&self) -> T;
}
