use optlib::genetic;
use optlib::genetic::cross;
use optlib::genetic::mutation;
use optlib::Optimizer;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

type Chromosomes = Vec<f64>;
type Population<'a> = genetic::Population<'a, Chromosomes>;

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
    fn cross(&self, parents: &Vec<Chromosomes>) -> Vec<Chromosomes> {
        assert!(parents.len() == 2);

        let chromo_count = parents[0].len();
        let mut children: Vec<Chromosomes> = Vec::with_capacity(chromo_count);
        children.push(vec![]);

        for n in 0..chromo_count {
            let new_chromo = cross::cross_middle(&vec![parents[0][n], parents[1][n]]);
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
        for individual in population.iter_mut() {
            if !individual.get_fitness().is_finite() {
                individual.kill();
                continue;
            }

            for chromo in individual.get_chromosomes() {
                if !chromo.is_finite() || chromo < self.xmin || chromo > self.xmax {
                    individual.kill();
                    kill_count += 1;
                    break;
                }
            }
        }

        // 2. Keep alive only population_size best individuals
        if population.len() > self.population_size + kill_count {
            let to_kill = population.len() - self.population_size - kill_count;
            self.kill_count(population, to_kill);
        }
    }
}

impl Selection {
    fn kill_count(&self, population: &mut Population, count: usize) {
        // List of indexes of individuals in population to be kill
        let mut kill_list: Vec<usize> = Vec::with_capacity(count);
        kill_list.push(0);

        // Index of the items in kill_list with best fitness
        let mut best_index = 0;
        let mut best_fitness = population[kill_list[best_index]].get_fitness();

        for n in 1..population.len() {
            if !population[n].is_alive() {
                continue;
            }

            if kill_list.len() < count {
                kill_list.push(n);
                if population[n].get_fitness() < best_fitness {
                    best_index = kill_list.len() - 1;
                }
            } else {
                if population[n].get_fitness() > best_fitness {
                    kill_list[best_index] = n;

                    // Find new best item
                    best_index = 0;
                    best_fitness = population[kill_list[best_index]].get_fitness();
                    for m in 1..kill_list.len() {
                        if population[kill_list[m]].get_fitness() < best_fitness {
                            best_index = m;
                            best_fitness = population[kill_list[best_index]].get_fitness();
                        }
                    }
                }
            }
        }

        for n in kill_list {
            population[n].kill();
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

// Stop checker

struct StopChecker {
    max_iter: usize,
}

impl StopChecker {
    pub fn new(max_iter: usize) -> StopChecker {
        StopChecker { max_iter }
    }
}

impl genetic::StopChecker<Chromosomes> for StopChecker {
    fn can_stop(&mut self, population: &Population) -> bool {
        population.get_iteration() >= self.max_iter
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

    optimizer.find_min();
    let mut new_stop_checker = StopChecker::new(max_iterations);
    optimizer.replace_stop_checker(&mut new_stop_checker);
    let result = optimizer.next_iterations();

    match result {
        None => println!("Решение не найдено"),
        Some((chromosomes, fitness)) => println!("Значение хромосом лучшей особи: {:?}\nЗначение целевой функции: {}",
                                     chromosomes, fitness),
    }
}
