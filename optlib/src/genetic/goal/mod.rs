//! The module with auxiliary tools to construct `Goal` trait objects.

/// Struct to convert (wrap) function to `Goal` trait.
pub struct GoalFromFunction<T> {
    function: fn(&T) -> f64,
}

impl<T> GoalFromFunction<T> {
    /// Constructor.
    pub fn new(function: fn(&T) -> f64) -> Self {
        Self { function, }
    }
}

impl<T> super::Goal<T> for GoalFromFunction<T> {
    fn get(&self, chromosomes: &T) -> f64 {
        (self.function)(chromosomes)
    }
}
