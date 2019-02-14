pub mod creation;
pub mod cross;
pub mod goal;
pub mod logging;
pub mod mutation;
pub mod pairing;
pub mod selection;
pub mod stopchecker;

use std::f64;
use std::ops;
use std::slice;

use super::{Agent, AlgorithmWithAgents, Optimizer};

#[derive(Debug)]
pub struct Individual<T: Clone> {
    chromosomes: T,
    fitness: f64,
    alive: bool,
}

impl<T: Clone> Clone for Individual<T> {
    fn clone(&self) -> Individual<T> {
        Individual {
            chromosomes: self.chromosomes.clone(),
            fitness: self.fitness,
            alive: self.alive,
        }
    }
}

impl<T: Clone> Agent<T> for Individual<T> {
    fn get_goal(&self) -> f64 {
        self.fitness
    }

    fn get_parameter(&self) -> &T {
        &self.chromosomes
    }
}

impl<T: Clone> Individual<T> {
    pub fn get_chromosomes(&self) -> &T {
        &self.chromosomes
    }

    pub fn get_fitness(&self) -> f64 {
        self.fitness
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
}

pub struct Population<T: Clone> {
    goal: Box<dyn Goal<T>>,
    individuals: Vec<Individual<T>>,
    best_individual: Option<Individual<T>>,
    iteration: usize,
}

impl<T: Clone> Population<T> {
    pub fn new(goal: Box<dyn Goal<T>>) -> Self {
        Population {
            goal,
            individuals: vec![],
            best_individual: None,
            iteration: 0,
        }
    }

    pub fn reset(&mut self) {
        self.individuals.clear();
        self.best_individual = None;
        self.iteration = 0;
    }

    pub fn push(&mut self, chromosomes: T) {
        let fitness = self.goal.get(&chromosomes);
        let new_individual = Individual {
            chromosomes,
            fitness,
            alive: true,
        };
        match &self.best_individual {
            None => self.best_individual = Some(new_individual.clone()),
            Some(old_best) => {
                if new_individual.get_fitness() < old_best.get_fitness() {
                    self.best_individual = Some(new_individual.clone());
                }
            }
        }
        self.individuals.push(new_individual);
    }

    pub fn append(&mut self, mut chromosomes_list: Vec<T>) {
        (0..chromosomes_list.len())
                .for_each(|_| self.push(chromosomes_list.remove(0)));
    }

    pub fn iter(&self) -> slice::Iter<Individual<T>> {
        self.individuals.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Individual<T>> {
        self.individuals.iter_mut()
    }

    pub fn get_iteration(&self) -> usize {
        self.iteration
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn get_best(&self) -> &Option<Individual<T>> {
        &self.best_individual
    }

    fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    fn remove(&mut self, index: usize) {
        self.individuals.remove(index);
    }
}

impl<T: Clone> ops::Index<usize> for Population<T> {
    type Output = Individual<T>;

    fn index(&self, index: usize) -> &Individual<T> {
        &self.individuals[index]
    }
}

impl<T: Clone> ops::IndexMut<usize> for Population<T> {
    fn index_mut<'b>(&'b mut self, index: usize) -> &'b mut Individual<T> {
        &mut self.individuals[index]
    }
}

pub trait Goal<T> {
    fn get(&self, chromosomes: &T) -> f64;
}

pub trait Creator<T: Clone> {
    fn create(&mut self) -> Vec<T>;
}

pub trait Cross<T: Clone> {
    fn cross(&mut self, parents: &[&T]) -> Vec<T>;
}

pub trait Mutation<T: Clone> {
    fn mutation(&mut self, chromosomes: &T) -> T;
}

pub trait Selection<T: Clone> {
    fn kill(&mut self, population: &mut Population<T>);
}

pub trait Pairing<T: Clone> {
    fn get_pairs(&mut self, population: &Population<T>) -> Vec<Vec<usize>>;
}

pub trait StopChecker<T: Clone> {
    fn can_stop(&mut self, population: &Population<T>) -> bool;
}

pub trait Logger<T: Clone> {
    fn start(&mut self, _population: &Population<T>) {}
    fn resume(&mut self, _population: &Population<T>) {}
    fn next_iteration(&mut self, _population: &Population<T>) {}
    fn finish(&mut self, _population: &Population<T>) {}
}

pub struct GeneticOptimizer<T: Clone> {
    creator: Box<dyn Creator<T>>,
    pairing: Box<dyn Pairing<T>>,
    cross: Box<dyn Cross<T>>,
    mutation: Box<dyn Mutation<T>>,
    selection: Box<dyn Selection<T>>,
    stop_checker: Box<dyn StopChecker<T>>,
    logger: Option<Box<dyn Logger<T>>>,
    population: Population<T>,
}

impl<T: Clone> GeneticOptimizer<T> {
    pub fn new(
        goal: Box<dyn Goal<T>>,
        creator: Box<dyn Creator<T>>,
        pairing: Box<dyn Pairing<T>>,
        cross: Box<dyn Cross<T>>,
        mutation: Box<dyn Mutation<T>>,
        selection: Box<dyn Selection<T>>,
        stop_checker: Box<dyn StopChecker<T>>,
        logger: Option<Box<dyn Logger<T>>>,
    ) -> GeneticOptimizer<T> {
        GeneticOptimizer {
            creator,
            pairing,
            cross,
            mutation,
            selection,
            stop_checker,
            logger,
            population: Population::new(goal),
        }
    }

    pub fn replace_pairing(&mut self, pairing: Box<dyn Pairing<T>>) {
        self.pairing = pairing;
    }

    pub fn replace_cross(&mut self, cross: Box<dyn Cross<T>>) {
        self.cross = cross;
    }

    pub fn replace_mutation(&mut self, mutation: Box<dyn Mutation<T>>) {
        self.mutation = mutation;
    }

    pub fn replace_selection(&mut self, selection: Box<dyn Selection<T>>) {
        self.selection = selection;
    }

    pub fn replace_stop_checker(&mut self, stop_checker: Box<dyn StopChecker<T>>) {
        self.stop_checker = stop_checker;
    }

    pub fn next_iterations(&mut self) -> Option<(&T, f64)> {
        if let Some(ref mut logger) = self.logger {
            logger.resume(&self.population);
        }

        while !self.stop_checker.can_stop(&self.population) {
            // Pairing
            let mut children_chromo_list = self.run_pairing();

            // Mutation
            let children_mutants = children_chromo_list
                .iter_mut()
                .map(|chromo| self.mutation.mutation(chromo)).collect();

            // Add new individuals to population
            self.population.append(children_mutants);

            // Selection
            self.selection.kill(&mut self.population);

            // Remove dead individuals
            let mut dead_count = 0;
            for n in 0..self.population.len() {
                if !self.population[n - dead_count].is_alive() {
                    self.population.remove(n - dead_count);
                    dead_count += 1;
                }
            }

            self.population.next_iteration();

            if let Some(ref mut logger) = self.logger {
                logger.next_iteration(&self.population);
            }
        }

        if let Some(ref mut logger) = self.logger {
            logger.finish(&self.population);
        }

        match &self.population.best_individual {
            None => None,
            Some(individual) => Some((&individual.chromosomes, individual.fitness)),
        }
    }

    fn run_pairing(&mut self) -> Vec<T> {
        let pairs: Vec<Vec<usize>> = self.pairing.get_pairs(&self.population);
        let mut new_chromosomes: Vec<T> = Vec::with_capacity(pairs.len());

        for pair in pairs {
            let mut cross_chromosomes = Vec::with_capacity(pair.len());
            for i in pair {
                cross_chromosomes.push(self.population[i].get_chromosomes());
            }

            let mut child_chromosomes = self.cross.cross(&cross_chromosomes);
            new_chromosomes.append(&mut child_chromosomes);
        }

        new_chromosomes
    }
}

impl<T: Clone> Optimizer<T> for GeneticOptimizer<T> {
    fn find_min(&mut self) -> Option<(&T, f64)> {
        self.population.reset();
        let start_chromo_list = self.creator.create();

        // Create individuals from chromosomes
        self.population.append(start_chromo_list);

        if let Some(ref mut logger) = self.logger {
            logger.start(&self.population);
        }
        self.next_iterations()
    }
}

impl<T: Clone> AlgorithmWithAgents<T> for GeneticOptimizer<T> {
    type Agent = Individual<T>;

    fn get_agents(&self) -> Vec<&Self::Agent> {
        let mut agents: Vec<&Self::Agent> = Vec::with_capacity(self.population.len());
        for individual in self.population.iter() {
            agents.push(individual);
        }

        agents
    }
}
