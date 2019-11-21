use num::abs;

use optlib::genetic::{self, creation, cross, mutation, pairing, pre_birth, selection};
use optlib::tools::logging;
use optlib::tools::stopchecker;
use optlib::{GoalFromFunction, Optimizer};
use optlib_testfunc;

type Gene = f32;
type Chromosomes = Vec<Gene>;

#[test]
fn genetic_paraboloid() {
    // General parameters
    let minval: Gene = -100.0;
    let maxval: Gene = 100.0;
    let population_size = 800;
    let chromo_count = 5;
    let intervals = vec![(minval, maxval); chromo_count];

    // Goal function
    let goal = GoalFromFunction::new(optlib_testfunc::paraboloid);

    // Creator
    let creator = creation::vec_float::RandomCreator::new(population_size, intervals.clone());

    // Pairing
    // let pairing = pairing::RandomPairing::new();
    let families_count = population_size / 2;
    let partners_count = 2;
    let rounds_count = 2;
    let pairing = pairing::Tournament::new(families_count)
        .partners_count(partners_count)
        .rounds_count(rounds_count);

    // Cross
    let single_cross = cross::FloatCrossExp::new();
    // let single_cross = cross::CrossBitwise::new();
    let cross = cross::VecCrossAllGenes::new(Box::new(single_cross));

    // Mutation
    let mutation_probability = 15.0;
    let mutation_gene_count = 3;
    let single_mutation = mutation::BitwiseMutation::new(mutation_gene_count);
    let mutation = mutation::VecMutation::new(mutation_probability, Box::new(single_mutation));

    // Pre birth
    let pre_births: Vec<Box<dyn genetic::PreBirth<Chromosomes>>> = vec![Box::new(
        pre_birth::vec_float::CheckChromoInterval::new(intervals.clone()),
    )];

    // Selection
    let selections: Vec<Box<dyn genetic::Selection<Chromosomes>>> = vec![
        Box::new(selection::KillFitnessNaN::new()),
        Box::new(selection::LimitPopulation::new(population_size)),
    ];

    // Stop checker
    let change_max_iterations = 150;
    let change_delta = 1e-7;
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-6)),
        Box::new(stopchecker::GoalNotChange::new(
            change_max_iterations,
            change_delta,
        )),
        Box::new(stopchecker::MaxIterations::new(5000)),
    ]);

    // Logger
    let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![];

    let mut optimizer = genetic::GeneticOptimizer::new(
        Box::new(goal),
        Box::new(stop_checker),
        Box::new(creator),
        Box::new(pairing),
        Box::new(cross),
        Box::new(mutation),
        selections,
        pre_births,
        loggers,
    );

    // Run genetic algorithm
    match optimizer.find_min() {
        None => assert!(false),
        Some((solution, goal_value)) => {
            for i in 0..chromo_count {
                assert!(abs(solution[i] - (i as f32 + 1.0)) < 0.1);
            }

            assert!(abs(goal_value) < 1e-3);
        }
    }
}
