use optlib::genetic;
use optlib::genetic::cross;
use optlib::genetic::mutation;
use optlib::Optimizer;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

type Chromosomes = Vec<f64>;
type Individual = genetic::Individual<Chromosomes>;
type Population = Vec<Individual>;

// Goal function
struct Goal;

impl genetic::Goal<Chromosomes> for Goal {
    fn get(&self, chromosomes: &Chromosomes) -> f64 {
        let mut result = 0.0;
        for val in chromosomes {
            result += val * val;
        }

        result
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
    fn cross(&self, individuals: &Population) -> Vec<Chromosomes> {
        assert!(individuals.len() == 2);

        let chromo_count = individuals[0].get_chromosomes().len();
        let mut new_chromosomes: Vec<Chromosomes> = Vec::with_capacity(chromo_count);

        for n in 0..chromo_count {
            let new_chromo = cross::cross_middle(&vec![
                individuals[0].get_chromosomes()[n],
                individuals[1].get_chromosomes()[n],
            ]);
            new_chromosomes.push(vec![new_chromo]);
        }

        new_chromosomes
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
    fn mutation(&mut self, chromosomes: &Chromosomes) -> Chromosomes {
        let mut rng = rand::thread_rng();
        let mutate = Uniform::new(0.0, 100.0);
        let mutation_count = 1;
        let mut mutant: Chromosomes = Vec::with_capacity(chromosomes.len());

        for n in 0..chromosomes.len() {
            if mutate.sample(&mut rng) < self.probability {
                let new_chromo = mutation::mutation_f64(chromosomes[n], mutation_count);
                mutant.push(new_chromo);
            }
        }

        mutant
    }
}

// Selection
struct Selection {
    population_size: usize,
}

impl Selection {
    pub fn new(population_size: usize) -> Selection {
        Selection { population_size }
    }
}

impl genetic::Selection<Chromosomes> for Selection {
    fn get_dead(&mut self, population: &Population) -> Vec<usize> {
        let dead_indexes: Vec<usize> = vec![];

        dead_indexes
    }
}

// Pairing

struct Pairing;

impl genetic::Pairing<Chromosomes> for Pairing {
    fn get_pairs(&mut self, population: &Population) -> Vec<Vec<usize>> {
        let pairs: Vec<Vec<usize>> = vec![];

        pairs
    }
}

// Stop checker

struct StopChecker {
    max_iter: usize,
    iteration: usize,
}

impl StopChecker {
    pub fn new(max_iter: usize) -> StopChecker {
        StopChecker {
            max_iter,
            iteration: 0,
        }
    }
}

impl genetic::StopChecker<Chromosomes> for StopChecker {
    fn finish(&mut self, population: &Population) -> bool {
        self.iteration += 1;
        self.iteration == self.max_iter
    }
}

fn main() {
    let xmin = -100.0;
    let xmax = 100.0;
    let size = 50;
    let chromo_count = 5;
    let mutation_probability = 5.0;
    let max_iterations = 250;

    let mut goal = Goal {};
    let mut creator = Creator::new(size, chromo_count, xmin, xmax);
    let mut cross = Cross {};
    let mut mutation = Mutation::new(mutation_probability);
    let mut selection = Selection::new(size);
    let mut pairing = Pairing {};
    let mut stop_checker = StopChecker::new(max_iterations);

    let mut optimizer = genetic::GeneticOptimizer::new(
        &mut goal,
        &mut creator,
        &mut pairing,
        &mut cross,
        &mut mutation,
        &mut selection,
        &mut stop_checker,
    );

    match optimizer.find_min() {
        None => println!("Решение не найдено"),
        Some((chromosomes, fitness)) => println!("Значение хромосом лучшей особи: {:?}\nЗначение целевой функции: {}",
                                     chromosomes, fitness),
    }
}
