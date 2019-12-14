//! The crate for global optimization algorithms.
//! The crate uses common traits for easy switch between algorithms.
extern crate num;

pub mod genetic;
pub mod particleswarm;
pub mod tools;

/// First item is current solution in search space,
/// second item is current goal value
type Solution<T> = (T, f64);

/// Common Optimizer trait.
///
/// `T` - type of a point in search space for goal function.
pub trait Optimizer<T> {
    /// Run an algorithm.
    ///
    /// Returns `Some(x: &T, goal: f64)`, where `x` - result of optimization,
    /// `goal` - value of goal function. Returns `None` if an algoritm can't find minimum of a goal function.
    ///
    /// # Remarks
    /// All algorithms with `Optimizer` must search minimum of a goal function.
    fn find_min(&mut self) -> Option<Solution<T>>;
}

pub trait AlgorithmState<T> {
    fn get_best_solution(&self) -> Option<Solution<T>>;
    fn get_iteration(&self) -> usize;
}

/// The trait for algotithms where use agents (genetic algorithm, partical swarm algorithm etc).
///
/// `T` - type of a point in search space for goal function.
pub trait AgentsState<T>: AlgorithmState<T> {
    type Agent: Agent<T>;

    /// Returns vector with references to all agents
    fn get_agents(&self) -> Vec<&Self::Agent>;
}

/// The trait for single point in search space. The trait used with `AlgorithmWithAgents`.
///
/// `T` - type of a point in search space for goal function.
pub trait Agent<T> {
    /// Returns parameter (point in search space) of the agent.
    fn get_parameter(&self) -> &T;

    /// Returns value of a goal function for current agent.
    fn get_goal(&self) -> f64;
}

/// The trait for the goal function.
pub trait Goal<T> {
    /// Must return value of goal function for the point in the search space (x).
    fn get(&self, x: &T) -> f64;
}

/// Struct to convert (wrap) function to `Goal` trait.
pub struct GoalFromFunction<T> {
    function: fn(&T) -> f64,
}

impl<T> GoalFromFunction<T> {
    /// Constructor.
    pub fn new(function: fn(&T) -> f64) -> Self {
        Self { function }
    }
}

impl<T> Goal<T> for GoalFromFunction<T> {
    fn get(&self, x: &T) -> f64 {
        (self.function)(x)
    }
}
