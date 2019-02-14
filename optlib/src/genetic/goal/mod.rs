pub struct GoalFromFunction<T> {
    function: fn(&T) -> f64,
}

impl<T> GoalFromFunction<T> {
    pub fn new(function: fn(&T) -> f64) -> Self {
        Self { function, }
    }
}

impl<T> super::Goal<T> for GoalFromFunction<T> {
    fn get(&self, chromosomes: &T) -> f64 {
        (self.function)(chromosomes)
    }
}
