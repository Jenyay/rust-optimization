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
    pub probability: usize,
}


impl Mutation {
    pub fn new(probability: usize) -> Mutation {
        Mutation { probability }
    }
}

impl genetic::Mutation<Chromosomes> for Mutation {
    fn mutation(&self, individual: &mut genetic::Individual<Chromosomes>) {
        let mut rng = rand::thread_rng();
        let mutate = Uniform::new(0, self.probability + 1);
        let mutation_count = 1;

        for n in 0..individual.chromosomes.len() {
            if mutate.sample(&mut rng) < self.probability {
                let new_chromo = mutation::mutation_f64(individual.chromosomes[n], mutation_count);
                individual.chromosomes[n] = new_chromo;
            }
        }
    }
}


fn main() {
    let size = 50;
    let goal = Goal{};
    let cross = Cross {};
    let mutation = Mutation::new(5);
    let optimizer = genetic::GeneticOptimizer::new(size,
                                                   &goal,
                                                   &cross,
                                                   &mutation);

    optimizer.run();
}
