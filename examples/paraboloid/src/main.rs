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

type Chromosomes = Vec<f64>;
type Population<'a> = genetic::Population<'a, Chromosomes>;

// Selection
struct Selection {
    population_size: usize,
    minval: f64,
    maxval: f64,
}

impl Selection {
    pub fn new(population_size: usize, minval: f64, maxval: f64) -> Selection {
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
    let minval = -100.0;
    let maxval = 100.0;
    let size = 100;
    let chromo_count = 5;
    let mutation_probability = 15.0;
    let mutation_gene_count = 1;
    let intervals = (0..chromo_count).map(|_| (minval, maxval)).collect();
    let cross_function = cross::vec_float::cross_middle;

    // For stop checkers
    let change_max_iterations = 50;
    let change_delta = 1e-5;

    let mut goal = goal::vec_float::GoalFromFunction::new(testfunctions::paraboloid);
    let mut creator = creation::vec_float::RandomCreator::new(size, intervals);
    let mut cross = cross::vec_float::FuncCross::new(cross_function);
    let mut mutation = mutation::vec_float::RandomChromosomesMutation::new(
        mutation_probability,
        mutation_gene_count,
    );
    let mut selection = Selection::new(size, minval, maxval);
    let mut pairing = pairing::RandomPairing::new();
    let logger = logging::vec_float::StdoutResultOnlyLogger::new(15);
    // let logger = logging::vec_float::StdoutLogger::new(15);
    let mut stop_checker = stopchecker::GoalNotChange::new(change_max_iterations, change_delta);
    // let mut stop_checker = stopchecker::MaxIterations::new(500);

    let mut optimizer = genetic::GeneticOptimizer::new(
        &mut goal,
        &mut creator,
        &mut pairing,
        &mut cross,
        &mut mutation,
        &mut selection,
        &mut stop_checker,
        Some(Box::new(logger)),
    );

    optimizer.find_min();
}
