use optlib::genetic::{
    self, creation, cross, mutation, pairing, pre_birth, selection, GeneticOptimizer,
};
use optlib::tools::{statistics, logging, stopchecker};
use optlib::{GoalFromFunction, Optimizer};
use optlib_testfunc;

type Gene = f32;
type Chromosomes = Vec<Gene>;

fn _create_optimizer<'a>(chromo_count: usize) -> GeneticOptimizer<'a, Chromosomes> {
    // General parameters
    let minval: Gene = -100.0;
    let maxval: Gene = 100.0;
    let population_size = 800;
    let intervals = vec![(minval, maxval); chromo_count];

    // Goal function
    let goal = GoalFromFunction::new(optlib_testfunc::paraboloid);

    // Creator
    let creator = creation::vec_float::RandomCreator::new(population_size, intervals.clone());

    // Pairing
    let families_count = population_size / 2;
    let partners_count = 2;
    let rounds_count = 2;
    let pairing = pairing::Tournament::new(families_count)
        .partners_count(partners_count)
        .rounds_count(rounds_count);

    // Cross
    let single_cross = cross::FloatCrossExp::new();
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

    let optimizer = genetic::GeneticOptimizer::new(
        Box::new(goal),
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
fn genetic_paraboloid_single() {
    let run_count = 1;
    let chromo_count = 5;
    let result;

    // Logging
    let mut statistics = statistics::Statistics::new();
    {
        let mut optimizer = _create_optimizer(chromo_count);

        let stat_logger = Box::new(statistics::StatisticsLogger::new(&mut statistics));
        let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![stat_logger];

        optimizer.set_loggers(loggers);

        result = optimizer.find_min().unwrap();
    }

    assert_eq!(statistics.get_run_count(), run_count);
    assert_eq!(statistics.get_results().len(), run_count);
    assert_eq!(statistics.get_convergence().len(), run_count);

    let stat_results = statistics.get_results();
    let (stat_solution, stat_goal) = stat_results[0].as_ref().unwrap();

    assert_eq!(result.0, *stat_solution);
    assert_eq!(result.1, *stat_goal);
}

#[test]
fn genetic_paraboloid_two() {
    let run_count = 2;
    let chromo_count = 5;
    let result_1;
    let result_2;

    // Logging
    let mut statistics = statistics::Statistics::new();
    {
        let mut optimizer = _create_optimizer(chromo_count);

        let stat_logger = Box::new(statistics::StatisticsLogger::new(&mut statistics));
        let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![stat_logger];

        optimizer.set_loggers(loggers);

        result_1 = optimizer.find_min().unwrap();
        result_2 = optimizer.find_min().unwrap();
    }

    assert_eq!(statistics.get_run_count(), run_count);
    assert_eq!(statistics.get_results().len(), run_count);
    assert_eq!(statistics.get_convergence().len(), run_count);

    let stat_results = statistics.get_results();
    let (stat_solution_1, stat_goal_1) = stat_results[0].as_ref().unwrap();
    let (stat_solution_2, stat_goal_2) = stat_results[1].as_ref().unwrap();

    assert_eq!(result_1.0, *stat_solution_1);
    assert_eq!(result_1.1, *stat_goal_1);

    assert_eq!(result_2.0, *stat_solution_2);
    assert_eq!(result_2.1, *stat_goal_2);
}
