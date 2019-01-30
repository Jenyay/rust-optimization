extern crate num;

pub mod genetic;
pub mod testfunctions;


/// Common Optimizer trait.
pub trait Optimizer<T> {
    fn find_min(&mut self) -> Option<(T, f64)>;
}
