extern crate num;

pub mod genetic;


/// Common Optimizer trait.
pub trait Optimizer {
    fn run(&mut self);
}
