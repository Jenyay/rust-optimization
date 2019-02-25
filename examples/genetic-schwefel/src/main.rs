use optlib::genetic;
use optlib::genetic::creation;
use optlib::genetic::cross;
use optlib::genetic::goal;
use optlib::genetic::logging;
use optlib::genetic::mutation;
use optlib::genetic::pairing;
use optlib::genetic::pre_birth;
use optlib::genetic::selection;
use optlib::genetic::stopchecker;
use optlib::testfunctions;
use optlib::Optimizer;

type Gene = f32;
type Chromosomes = Vec<Gene>;

fn main() {
    // General parameters
    let minval: Gene = -500.0;
    let maxval: Gene = 500.0;
    let population_size = 500;
    let chromo_count = 15;
    let intervals = vec![(minval, maxval); chromo_count];

    // Goal function
    let goal = goal::GoalFromFunction::new(testfunctions::schwefel);

    // Creator
    let creator = creation::vec_float::RandomCreator::new(population_size, intervals.clone());

    // Pairing
    // let pairing = pairing::RandomPairing::new();
    let partners_count = 2;
    let families_count = population_size / 2;
    let rounds_count = 5;
    let pairing = pairing::Tournament::new(partners_count, families_count, rounds_count);

    // Cross
    // let single_cross = cross::CrossMean::new();
    let single_cross = cross::FloatCrossExp::new();
    // let single_cross = cross::CrossBitwise::new();
    let cross = cross::VecCrossAllGenes::new(Box::new(single_cross));

    // Mutation
    let mutation_probability = 15.0;
    let mutation_gene_count = 3;
    let single_mutation = mutation::BitwiseMutation::new(mutation_gene_count);
    // let single_cross = cross::CrossMean::new();
    // let single_cross = cross::FloatCrossGeometricMean::new();
    let mutation = mutation::VecMutation::new(mutation_probability, Box::new(single_mutation));

    // Pre birth
    let pre_births: Vec<Box<genetic::PreBirth<Chromosomes>>> = vec![Box::new(
        pre_birth::vec_float::CheckChromoInterval::new(intervals.clone()),
    )];

    // Stop checker
    // let change_max_iterations = 200;
    // let change_delta = 1e-7;
    // let stop_checker = stopchecker::GoalNotChange::new(change_max_iterations, change_delta);
    // let stop_checker = stopchecker::MaxIterations::new(500);
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-4)),
        // Box::new(stopchecker::GoalNotChange::new(
        //     change_max_iterations,
        //     change_delta,
        // )),
        Box::new(stopchecker::MaxIterations::new(3000)),
    ]);

    // Selection
    let selections: Vec<Box<dyn genetic::Selection<Chromosomes>>> = vec![
        Box::new(selection::KillFitnessNaN::new()),
        Box::new(selection::LimitPopulation::new(population_size)),
    ];

    // Logger
    let loggers: Vec<Box<genetic::Logger<Chromosomes>>> = vec![
        // Box::new(logging::VerboseStdoutLogger::new(15)),
        Box::new(logging::StdoutResultOnlyLogger::new(15)),
        Box::new(logging::TimeStdoutLogger::new()),
    ];

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

    optimizer.find_min();
}
