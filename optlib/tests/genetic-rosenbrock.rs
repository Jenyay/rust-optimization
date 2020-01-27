//! Example of optimizing the Rosenbrock function.
//!
//! y = f(x), where x = (x0, x1, ..., xi,... xn).
//! Global minimum is x' = (1.0, 1.0, ...) for any xi.
//! f(x') = 0
//!
//! # Terms
//! * `Goal function` - the function for optimization. y = f(x).
//! * `Gene` - a single value of xi.
//! * `Chromosome` - a point in the search space. x = (x0, x1, x2, ..., xn).
//! * `Individual` - union of x and value of goal function.
//! * `Population` - set of the individuals.
//! * `Generation` - a number of iteration of genetic algorithm.
use std::sync::mpsc;
use std::thread;

use optlib::genetic::{
    self, creation, cross, mutation, pairing, pre_birth, selection, GeneticOptimizer,
};
use optlib::tools::statistics::{get_predicate_success_vec_solution, StatFunctionsSolution};
use optlib::tools::{logging, statistics, stopchecker};
use optlib::{Goal, GoalFromFunction, Optimizer};
use optlib_testfunc;

/// Gene type
type Gene = f32;

/// Chromosomes type
type Chromosomes = Vec<Gene>;

fn create_optimizer<'a>(
    chromo_count: usize,
    goal: Box<dyn Goal<Chromosomes> + 'a>,
) -> GeneticOptimizer<'a, Chromosomes> {
    // General parameters

    // Search space. Any xi lies in [-500.0; 500.0]
    let minval: Gene = -2.0;
    let maxval: Gene = 2.0;

    // Count individuals in initial population
    let population_size = 700;

    let intervals = vec![(minval, maxval); chromo_count];

    // Make the creator to create initial population.
    // RandomCreator will fill initial population with individuals with random chromosomes in a
    // given interval,
    let creator = creation::vec_float::RandomCreator::new(population_size, intervals.clone());

    // Make a trait object for the pairing.
    // Pairing is algorithm of selection individuals for crossbreeding.

    // Tournament method.
    let pairing = pairing::RandomPairing::new();

    // Crossbreeding algorithm.
    // Make a Cross trait object. The bitwise crossing for float genes.
    let single_cross = cross::FloatCrossExp::new();
    let cross = cross::VecCrossAllGenes::new(Box::new(single_cross));

    // Make a Mutation trait object.
    // Use bitwise mutation (change random bits with given probability).
    let mutation_probability = 80.0;
    let mutation_gene_count = 3;
    let single_mutation = mutation::BitwiseMutation::new(mutation_gene_count);
    let mutation = mutation::VecMutation::new(mutation_probability, Box::new(single_mutation));

    // Pre birth. Throw away new chlld chromosomes if their genes do not lies if given intervals.
    let pre_births: Vec<Box<dyn genetic::PreBirth<Chromosomes>>> = vec![Box::new(
        pre_birth::vec_float::CheckChromoInterval::new(intervals.clone()),
    )];

    // Stop checker. Stop criterion for genetic algorithm.
    // Stop algorithm after 3000 generation (iteration).
    let change_max_iterations = 2000;
    let change_delta = 1e-7;
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-6)),
        Box::new(stopchecker::GoalNotChange::new(
            change_max_iterations,
            change_delta,
        )),
    ]);

    // Make a trait object for selection. Selection is killing the worst individuals.
    // Kill all individuals if the value of goal function is NaN or Inf.
    // Kill the worst individuals to population size remained unchanged.
    let selections: Vec<Box<dyn genetic::Selection<Chromosomes>>> = vec![
        Box::new(selection::KillFitnessNaN::new()),
        Box::new(selection::LimitPopulation::new(population_size)),
    ];

    // Construct main optimizer struct
    let optimizer = genetic::GeneticOptimizer::new(
        goal,
        Box::new(stop_checker),
        Box::new(creator),
        Box::new(pairing),
        Box::new(cross),
        Box::new(mutation),
        selections,
        pre_births,
    );

    optimizer
}

#[test]
fn genetic_rosenbrock() {
    let cpu = num_cpus::get();
    let dimension = 3;

    // Running count per CPU
    let run_count = 100 / cpu;

    // Statistics from all runnings
    let mut full_stat = statistics::Statistics::new();

    let (tx, rx) = mpsc::channel();

    for _ in 0..cpu {
        let current_tx = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let mut local_full_stat = statistics::Statistics::new();

            for _ in 0..run_count {
                // Statistics from single run
                let mut statistics_data = statistics::Statistics::new();
                {
                    // Make a trait object for goal function
                    let goal = GoalFromFunction::new(optlib_testfunc::rosenbrock);

                    let mut optimizer = create_optimizer(dimension, Box::new(goal));

                    // Add logger to collect statistics
                    let stat_logger =
                        Box::new(statistics::StatisticsLogger::new(&mut statistics_data));
                    let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![stat_logger];
                    optimizer.set_loggers(loggers);

                    // Run optimization
                    optimizer.find_min();
                }

                // Add current running statistics to full statistics
                local_full_stat.unite(statistics_data);
            }
            current_tx.send(local_full_stat).unwrap();
        });
    }

    // Collect data from threads
    for _ in 0..cpu {
        let statistics_data = rx.recv().unwrap();
        full_stat.unite(statistics_data);
    }

    let valid_answer = vec![1.0; dimension];
    let delta = vec![1e-2; dimension];

    let success_rate = full_stat
        .get_results()
        .get_success_rate(get_predicate_success_vec_solution(valid_answer, delta))
        .unwrap();

    assert!(success_rate >= 0.75);
}
