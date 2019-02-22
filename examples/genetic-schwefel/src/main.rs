use optlib::genetic;
use optlib::genetic::creation;
use optlib::genetic::cross;
use optlib::genetic::goal;
use optlib::genetic::logging;
use optlib::genetic::mutation;
use optlib::genetic::pairing;
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
    let pairing = pairing::RandomPairing::new();

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

    // Stop checker
    let change_max_iterations = 150;
    let change_delta = 1e-7;
    let stop_checker = stopchecker::GoalNotChange::new(change_max_iterations, change_delta);
    // let stop_checker = stopchecker::MaxIterations::new(500);

    // Selection
    let selection_algorithms: Vec<Box<dyn genetic::Selection<Chromosomes>>> = vec![
        Box::new(selection::KillFitnessNaN::new()),
        Box::new(selection::vec_float::CheckChromoInterval::new(intervals.clone())),
        Box::new(selection::LimitPopulation::new(population_size)),
    ];
    let selection = selection::Composite::new(selection_algorithms);

    // Logger
    let logger = logging::StdoutResultOnlyLogger::new(8);
    // let logger = logging::VerboseStdoutLogger::new(8);

    let mut optimizer = genetic::GeneticOptimizer::new(
        Box::new(goal),
        Box::new(creator),
        Box::new(pairing),
        Box::new(cross),
        Box::new(mutation),
        Box::new(selection),
        Box::new(stop_checker),
        Some(Box::new(logger)),
    );

    optimizer.find_min();
}
