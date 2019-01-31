use optlib::genetic;
use optlib::genetic::cross;
use optlib::genetic::mutation;
use optlib::genetic::selection;
use optlib::genetic::stopchecker;
use optlib::testfunctions;
use optlib::Optimizer;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

type Chromosomes = Vec<f64>;
type Population<'a> = genetic::Population<'a, Chromosomes>;

// Goal function
struct Goal;

impl genetic::Goal<Chromosomes> for Goal {
    fn get(&self, chromosomes: &Chromosomes) -> f64 {
        testfunctions::paraboloid(chromosomes)
    }
}

// Creator
struct Creator {
    population_size: usize,
    chromo_count: usize,
    xmin: f64,
    xmax: f64,
    random: ThreadRng,
}

impl Creator {
    pub fn new(population_size: usize, chromo_count: usize, xmin: f64, xmax: f64) -> Creator {
        let random = rand::thread_rng();
        Creator {
            population_size,
            chromo_count,
            xmin,
            xmax,
            random,
        }
    }
}

impl genetic::Creator<Chromosomes> for Creator {
    fn create(&mut self) -> Vec<Chromosomes> {
        let between = Uniform::new(self.xmin, self.xmax);
        let mut population = Vec::with_capacity(self.population_size * 2);

        for _ in 0..self.population_size {
            let mut chromo = Vec::with_capacity(self.chromo_count);
            for _ in 0..self.chromo_count {
                chromo.push(between.sample(&mut self.random));
            }

            population.push(chromo);
        }

        population
    }
}

// Cross
struct Cross;

impl genetic::Cross<Chromosomes> for Cross {
    fn cross(&self, parents: &Vec<Chromosomes>) -> Vec<Chromosomes> {
        assert!(parents.len() == 2);

        let chromo_count = parents[0].len();
        let mut children: Vec<Chromosomes> = Vec::with_capacity(chromo_count);
        children.push(vec![]);

        for n in 0..chromo_count {
            let new_chromo = cross::vec_float::cross_middle(&vec![parents[0][n], parents[1][n]]);
            children[0].push(new_chromo);
        }

        children
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
                chromosomes[n] = mutation::mutation_f64(chromosomes[n], mutation_count);;
            }
        }
    }
}

// Selection
struct Selection {
    population_size: usize,
    xmin: f64,
    xmax: f64,
}

impl Selection {
    pub fn new(population_size: usize, xmin: f64, xmax: f64) -> Selection {
        Selection {
            population_size,
            xmin,
            xmax,
        }
    }
}

impl genetic::Selection<Chromosomes> for Selection {
    fn kill(&mut self, population: &mut Population) {
        // 1. Kill all individuals with chromosomes outside the interval [xmin; xmax]
        let mut kill_count = 0;
        kill_count += selection::kill_fitness_nan(population);
        kill_count += selection::vec_float::kill_chromo_interval(population, self.xmin, self.xmax);

        // 2. Keep alive only population_size best individuals
        if population.len() - kill_count > self.population_size {
            let to_kill = population.len() - self.population_size - kill_count;
            selection::kill_worst(population, to_kill);
        }
    }
}

// Pairing

struct Pairing {
    random: ThreadRng,
}

impl genetic::Pairing<Chromosomes> for Pairing {
    fn get_pairs(&mut self, population: &Population) -> Vec<Vec<usize>> {
        let mut pairs: Vec<Vec<usize>> = vec![];

        let between = Uniform::new(0, population.len());
        let count = population.len() / 2;
        for _ in 0..count {
            let first = between.sample(&mut self.random);
            let second = between.sample(&mut self.random);
            let pair = vec![first, second];
            pairs.push(pair);
        }

        pairs
    }
}

impl Pairing {
    fn new() -> Self {
        let random = rand::thread_rng();
        Pairing { random }
    }
}

fn main() {
    let xmin = -100.0;
    let xmax = 100.0;
    let size = 50;
    let chromo_count = 5;
    let mutation_probability = 5.0;
    let max_iterations = 500;

    let mut goal = Goal {};
    let mut creator = Creator::new(size, chromo_count, xmin, xmax);
    let mut cross = Cross {};
    let mut mutation = Mutation::new(mutation_probability);
    let mut selection = Selection::new(size, xmin, xmax);
    let mut pairing = Pairing::new();
    let mut stop_checker = stopchecker::MaxIterations::new(max_iterations);

    let mut optimizer = genetic::GeneticOptimizer::new(
        &mut goal,
        &mut creator,
        &mut pairing,
        &mut cross,
        &mut mutation,
        &mut selection,
        &mut stop_checker,
    );

    optimizer.find_min();
    let mut new_stop_checker = stopchecker::MaxIterations::new(max_iterations);
    optimizer.replace_stop_checker(&mut new_stop_checker);
    let result = optimizer.next_iterations();

    match result {
        None => println!("Решение не найдено"),
        Some((chromosomes, fitness)) => println!("Значение хромосом лучшей особи: {:?}\nЗначение целевой функции: {}",
                                     chromosomes, fitness),
    }
}
