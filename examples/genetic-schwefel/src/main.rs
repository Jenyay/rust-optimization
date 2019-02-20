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
type Population = genetic::Population<Chromosomes>;

// Selection
struct Selection {
    population_size: usize,
    minval: Gene,
    maxval: Gene,
}

impl Selection {
    pub fn new(population_size: usize, minval: Gene, maxval: Gene) -> Selection {
        Selection {
            population_size,
            minval,
            maxval,
        }
    }
}

impl genetic::Selection<Chromosomes> for Selection {
    fn kill(&mut self, population: &mut Population) {
        // 1. Kill all individuals with chromosomes outside the interval [minval; maxval]
        let mut kill_count = 0;
        kill_count += selection::kill_fitness_nan(population);

        kill_count +=
            selection::vec_float::kill_chromo_interval(population, self.minval, self.maxval);

        // 2. Keep alive only population_size best individuals
        if population.len() - kill_count > self.population_size {
            let to_kill = population.len() - self.population_size - kill_count;
            selection::kill_worst(population, to_kill);
        }
    }
}

fn main() {
    // General parameters
    let minval: Gene = -500.0;
    let maxval: Gene = 500.0;
    let size = 500;
    let chromo_count = 15;

    // Mutation
    let mutation_probability = 15.0;
    let mutation_gene_count = 3;
    let single_mutation = mutation::BitwiseMutation::new(mutation_gene_count);
    // let single_cross = cross::CrossMean::new();
    // let single_cross = cross::FloatCrossGeometricMean::new();
    let mutation = mutation::VecMutation::new(
        mutation_probability,
        Box::new(single_mutation),
    );

    // Cross
    // let single_cross = cross::CrossMean::new();
    let single_cross = cross::FloatCrossExp::new();
    // let single_cross = cross::CrossBitwise::new();
    let cross = cross::VecCrossAllGenes::new(Box::new(single_cross));

    // Stop checker
    let change_max_iterations = 150;
    let change_delta = 1e-7;
    let stop_checker = stopchecker::GoalNotChange::new(change_max_iterations, change_delta);
    // let stop_checker = stopchecker::MaxIterations::new(500);

    let goal = goal::GoalFromFunction::new(testfunctions::schwefel);
    let intervals: Vec<(Gene, Gene)> = (0..chromo_count).map(|_| (minval, maxval)).collect();
    let creator = creation::vec_float::RandomCreator::new(size, intervals);
    let selection = Selection::new(size, minval, maxval);
    let pairing = pairing::RandomPairing::new();

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
