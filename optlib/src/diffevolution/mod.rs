//! The module with Differential Evolution (DE) method.
//!
//! # Terms
//! * "Vector" is point in the search space.

use num::Float;

use crate::tools::logging::Logger;
use crate::tools::stopchecker::StopChecker;
use crate::{Agent, AgentsState, AlgorithmState, Goal, IterativeOptimizer, Optimizer, Solution};
