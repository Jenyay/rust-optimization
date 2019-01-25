use optlib::genetic;
use optlib::genetic::cross;
use optlib::genetic::mutation;
use optlib::Optimizer;

use rand::distributions::{Distribution, Uniform};

type Chromosomes = Vec<f64>;

// Goal function
struct Goal;

impl genetic::Goal<Chromosomes> for Goal {
    fn get(&mut self, chromosomes: &Chromosomes) -> f64 {
        let mut result = 0.0;
        for val in chromosomes {
            result += val * val;
        }

        result
    }
}


// Creator
struct Creator {
    size: usize,
    chromo_count: usize,
    xmin: f64,
    xmax: f64,
}


impl Creator {
    pub fn new(size: usize, chromo_count: usize,
               xmin: f64, xmax: f64) -> Creator {
        Creator { size, chromo_count, xmin, xmax }
    }
}


impl genetic::Creator<Chromosomes> for Creator {
    fn create(&mut self) -> Vec<genetic::Individual<Chromosomes>> {
        let mut population = Vec::with_capacity(self.size * 2);

        population
    }
}


// Cross
struct Cross;

impl genetic::Cross<Chromosomes> for Cross {
    fn cross(&mut self, individuals: &Vec<genetic::Individual<Chromosomes>>) -> Vec<genetic::Individual<Chromosomes>> {
        assert!(individuals.len() == 2);

        let chromo_count = individuals[0].chromosomes.len();
        let mut new_chromosomes = Chromosomes::with_capacity(chromo_count);

        for n in 0..chromo_count {
            let new_chromo = cross::cross_middle(&vec![individuals[0].chromosomes[n], individuals[1].chromosomes[n]]);
            new_chromosomes.push(new_chromo);
        }

        let new_individual = genetic::Individual::new(new_chromosomes);
        vec![new_individual]
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
    fn mutation(&mut self, individual: &mut genetic::Individual<Chromosomes>) {
        let mut rng = rand::thread_rng();
        let mutate = Uniform::new(0.0, 100.0);
        let mutation_count = 1;

        for n in 0..individual.chromosomes.len() {
            if mutate.sample(&mut rng) < self.probability {
                let new_chromo = mutation::mutation_f64(individual.chromosomes[n], mutation_count);
                individual.chromosomes[n] = new_chromo;
            }
        }
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
    fn get_dead(&mut self, population: &Vec<genetic::Individual<Chromosomes>>) -> Vec<usize> {
        let dead_indexes: Vec<usize> = vec![];

        dead_indexes
    }
}


// Pairing

struct Pairing;

impl genetic::Pairing<Chromosomes> for Pairing {
    fn get_pairs(&mut self, population: &Vec<genetic::Individual<Chromosomes>>) -> Vec<Vec<usize>> {
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
    fn finish(&mut self, population: &Vec<genetic::Individual<Chromosomes>>) -> bool {
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

    let mut goal = Goal{};
    let mut creator = Creator::new(size, chromo_count, xmin, xmax);
    let mut cross = Cross{};
    let mut mutation = Mutation::new(mutation_probability);
    let mut selection = Selection::new(size);
    let mut pairing = Pairing{};
    let mut stop_checker = StopChecker::new(max_iterations);

    let mut optimizer = genetic::GeneticOptimizer::new(&mut goal,
                                                       &mut creator,
                                                       &mut pairing,
                                                       &mut cross,
                                                       &mut mutation,
                                                       &mut selection,
                                                       &mut stop_checker);

    optimizer.find_min();
}
