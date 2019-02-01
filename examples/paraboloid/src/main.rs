use optlib::genetic;
use optlib::genetic::creation;
use optlib::genetic::cross;
use optlib::genetic::mutation;
use optlib::genetic::pairing;
use optlib::genetic::selection;
use optlib::genetic::stopchecker;
use optlib::testfunctions;
use optlib::Optimizer;

use rand::distributions::{Distribution, Uniform};

type Chromosomes = Vec<f64>;
type Population<'a> = genetic::Population<'a, Chromosomes>;

// Goal function
struct Goal;

impl genetic::Goal<Chromosomes> for Goal {
    fn get(&self, chromosomes: &Chromosomes) -> f64 {
        testfunctions::paraboloid(chromosomes)
    }
}

// Mutation
struct Mutation {
    pub probability: f64,
}

impl Mutation {
    pub fn new(probability: f64) -> Mutation {
        Mutation { probability }
    }
}

impl genetic::Mutation<Chromosomes> for Mutation {
    fn mutation(&mut self, chromosomes: &mut Chromosomes) {
        let mut rng = rand::thread_rng();
        let mutate = Uniform::new(0.0, 100.0);
        let mutation_count = 1;

        for n in 0..chromosomes.len() {
            if mutate.sample(&mut rng) < self.probability {
                chromosomes[n] = mutation::mutation_f64(chromosomes[n], mutation_count);
            }
        }
    }
}

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
    let size = 50;
    let chromo_count = 8;
    let mutation_probability = 5.0;
    let intervals = (0..chromo_count).map(|_| (minval, maxval)).collect();
    let cross_function = cross::vec_float::cross_middle;

    // For stop checkers
    let change_max_iterations = 50;
    let change_delta = 1e-5;
    // let max_iterations = 100;

    let mut goal = Goal {};
    let mut creator = creation::vec_float::RandomCreator::new(size, intervals);
    let mut cross = cross::vec_float::FuncCross::new(cross_function);
    let mut mutation = Mutation::new(mutation_probability);
    let mut selection = Selection::new(size, minval, maxval);
    let mut pairing = pairing::RandomPairing::new();
    let mut stop_checker = stopchecker::GoalNotChange::new(change_max_iterations, change_delta);
    // let mut stop_checker = stopchecker::MaxIterations::new(max_iterations);

    let mut optimizer = genetic::GeneticOptimizer::new(
        &mut goal,
        &mut creator,
        &mut pairing,
        &mut cross,
        &mut mutation,
        &mut selection,
        &mut stop_checker,
    );

    let result = optimizer.find_min();
    // let mut new_stop_checker = stopchecker::MaxIterations::new(max_iterations);
    // optimizer.replace_stop_checker(&mut new_stop_checker);
    // let result = optimizer.next_iterations();

    match result {
        None => println!("Решение не найдено"),
        Some((chromosomes, fitness)) => println!("Значение хромосом лучшей особи: {:?}\nЗначение целевой функции: {}",
                                     chromosomes, fitness),
    }
}
