use optlib::genetic;
use optlib::genetic::cross;
use optlib::genetic::mutation;
use optlib::Optimizer;

use rand::distributions::{Distribution, Uniform};

type Chromosomes = Vec<f64>;

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


// Cross
struct Cross;

impl genetic::Cross<Chromosomes> for Cross {
    fn cross(&self, individuals: &Vec<genetic::Individual<Chromosomes>>) -> Vec<genetic::Individual<Chromosomes>> {
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
    fn mutation(&self, individual: &mut genetic::Individual<Chromosomes>) {
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
    fn get_dead(&self, population: &Vec<genetic::Individual<Chromosomes>>) -> Vec<usize> {
        let dead_indexes: Vec<usize> = vec![];

        dead_indexes
    }
}


// Pairing

struct Pairing;

impl genetic::Pairing<Chromosomes> for Pairing {
    fn get_pairs(&self, population: &Vec<genetic::Individual<Chromosomes>>) -> Vec<Vec<usize>> {
        let pairs: Vec<Vec<usize>> = vec![];

        pairs
    }
}


fn main() {
    let size = 50;
    let mutation_probability = 5.0;
    let goal = Goal{};
    let cross = Cross{};
    let mutation = Mutation::new(mutation_probability);
    let selection = Selection::new(size);
    let pairing = Pairing{};
    let optimizer = genetic::GeneticOptimizer::new(size,
                                                   &goal,
                                                   &pairing,
                                                   &cross,
                                                   &mutation,
                                                   &selection);

    optimizer.run();
}
