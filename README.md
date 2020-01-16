# optlib

[![Current Version](https://img.shields.io/crates/v/optlib.svg)](https://crates.io/crates/optlib)
[![Documentation](https://docs.rs/optlib/badge.svg)](https://docs.rs/optlib)
[![License](https://img.shields.io/crates/l/optlib.svg)](https://crates.io/crates/optlib)

Optimization algorithms implemented in Rust

For now optlib provides genetic algorithm and partcile swarm algorithm.


## Example of optimization

```rust
//! Example of optimizing the Schwefel function with genetic algorithm.
//!
//! y = f(x), where x = (x0, x1, ..., xi,... xn).
//! Global minimum is x' = (420.9687, 420.9687, ...) for any xi lying in [-500.0; 500.0].
//! f(x') = 0
//!
//! # Terms
//! * `Goal function` - the function for optimization. y = f(x).
//! * `Gene` - a single value of xi.
//! * `Chromosome` - a point in the search space. x = (x0, x1, x2, ..., xn).
//! * `Individual` - union of x and value of goal function.
//! * `Population` - set of the individuals.
//! * `Generation` - a number of iteration of genetic algorithm.
use std::io;

use optlib::genetic::{self, creation, cross, mutation, pairing, pre_birth, selection};
use optlib::tools::logging;
use optlib::tools::stopchecker;
use optlib::{GoalFromFunction, Optimizer};
use optlib_testfunc;

/// Gene type
type Gene = f32;

/// Chromosomes type
type Chromosomes = Vec<Gene>;

fn main() {
    // General parameters

    // Search space. Any xi lies in [-500.0; 500.0]
    let minval: Gene = -500.0;
    let maxval: Gene = 500.0;

    // Count individuals in initial population
    let population_size = 500;

    // Count of xi in the chromosomes
    let chromo_count = 15;

    let intervals = vec![(minval, maxval); chromo_count];

    // Make a trait object for goal function (Schwefel function)
    let goal = GoalFromFunction::new(optlib_testfunc::schwefel);

    // Make the creator to create initial population.
    // RandomCreator will fill initial population with individuals with random chromosomes in a
    // given interval,
    let creator = creation::vec_float::RandomCreator::new(population_size, intervals.clone());

    // Make a trait object for the pairing.
    // Pairing is algorithm of selection individuals for crossbreeding.

    // Select random individuals from the population.
    // let pairing = pairing::RandomPairing::new();

    // Tournament method.
    let families_count = population_size / 2;
    let rounds_count = 5;
    let pairing = pairing::Tournament::new(families_count).rounds_count(rounds_count);

    // Crossbreeding algorithm.
    // Make a Cross trait object. The bitwise crossing for float genes.
    let single_cross = cross::FloatCrossExp::new();
    let cross = cross::VecCrossAllGenes::new(Box::new(single_cross));

    // Make a Mutation trait object.
    // Use bitwise mutation (change random bits with given probability).
    let mutation_probability = 15.0;
    let mutation_gene_count = 3;
    let single_mutation = mutation::BitwiseMutation::new(mutation_gene_count);
    let mutation = mutation::VecMutation::new(mutation_probability, Box::new(single_mutation));

    // Pre birth. Throw away new chlld chromosomes if their genes do not lies if given intervals.
    let pre_births: Vec<Box<dyn genetic::PreBirth<Chromosomes>>> = vec![Box::new(
        pre_birth::vec_float::CheckChromoInterval::new(intervals.clone()),
    )];

    // Stop checker. Stop criterion for genetic algorithm.
    // Stop algorithm if the value of goal function will become less of 1e-4 or
    // after 3000 generations (iterations).
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-4)),
        Box::new(stopchecker::MaxIterations::new(3000)),
    ]);

    // Make a trait object for selection. Selection is killing the worst individuals.
    // Kill all individuals if the value of goal function is NaN or Inf.
    // Kill the worst individuals to population size remained unchanged.
    let selections: Vec<Box<dyn genetic::Selection<Chromosomes>>> = vec![
        Box::new(selection::KillFitnessNaN::new()),
        Box::new(selection::LimitPopulation::new(population_size)),
    ];

    // Make a loggers trait objects
    let mut stdout_result = io::stdout();
    let mut stdout_time = io::stdout();

    let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![
        Box::new(logging::ResultOnlyLogger::new(&mut stdout_result, 15)),
        Box::new(logging::TimeLogger::new(&mut stdout_time)),
    ];

    // Construct main optimizer struct
    let mut optimizer = genetic::GeneticOptimizer::new(
        Box::new(goal),
        Box::new(stop_checker),
        Box::new(creator),
        Box::new(pairing),
        Box::new(cross),
        Box::new(mutation),
        selections,
        pre_births,
    );
    optimizer.set_loggers(loggers);

    // Run genetic algorithm
    optimizer.find_min();
}

```

Build all crates:

```
cargo build --release --all
```

Run example:

```
cargo run --example genetic-schwefel --release
```

Work result:

```
Solution:
  420.974975585937500
  420.969146728515625
  420.955078125000000
  421.004760742187500
  420.999511718750000
  421.007263183593750
  420.987487792968750
  421.001800537109375
  420.980499267578125
  420.991180419921875
  421.001068115234375
  420.942718505859375
  420.964080810546875
  420.951721191406250
  420.961029052734375


Goal: 0.000488281250000
Iterations count: 3000
Time elapsed: 2352 ms
```

Also optlib library contains other optimization examples:

* genetic-paraboloid
* genetic-rastrigin
* genetic-rosenbrock
* genetic-schwefel
* genetic-schwefel-iterative
* genetic-schwefel-statistics
* particleswarm-paraboloid
* particleswarm-rastrigin
* particleswarm-rastrigin-statistics-inertia
* particleswarm-rosenbrock
* particleswarm-schwefel
* particleswarm-schwefel-iterative
* particleswarm-schwefel-statistics
* particleswarm-schwefel-statistics-full
* particleswarm-schwefel-statistics-inertia
