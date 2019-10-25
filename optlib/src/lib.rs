//! The crate for global optimization algorithms.
//! The crate uses common traits for easy switch between algorithms.
extern crate num;

pub mod genetic;
pub mod particleswarm;

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
    fn find_min(&mut self) -> Option<(T, f64)>;
}

/// The trait for algotithms where use agents (genetic algorithm, partical swarm algorithm etc).
///
/// `T` - type of a point in search space for goal function.
pub trait AlgorithmWithAgents<T> {
    type Agent: Agent<T>;

    /// Returns vector with references to all agents
    fn get_agents(&self) -> Vec<&Self::Agent>;

    /// Returns best agent At this point in time
    fn get_best_agent(&self) -> Option<&dyn Agent<T>>;
}

pub trait IterativeAlgorithm {
    fn get_iteration(&self) -> usize;
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
