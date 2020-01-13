//! The module with genetic algorithm implementation.
//!
//! # Terms
//! * "chromosomes" are points in the search space.
//! Usually chromosome is single value or vector of values.
//! * "Fitness" is value of goal function value in genetic algorithm.
//! * "Generation" is iteration number of genetic algorithm.
//! * "Individual" is agent in genetic algorithm (point in the search space and value of goal
//! function).

pub mod creation;
pub mod cross;
pub mod mutation;
pub mod pairing;
pub mod pre_birth;
pub mod selection;

use std::cmp::Ordering;
use std::f64;
use std::ops;
use std::slice;

use crate::tools::logging::Logger;
use crate::tools::stopchecker::StopChecker;
use crate::{Agent, AgentsState, AlgorithmState, Goal, IterativeOptimizer, Optimizer, Solution};

/// Struct for single point (agent) in the search space
///
/// `T` - type of a point in the search space for goal function (chromosomes).
#[derive(Debug)]
pub struct Individual<T> {
    /// Point in the search space.
    chromosomes: T,

    /// Value of goal function for the point in the search space.
    fitness: f64,

    /// True if individual will pass to text generation.
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

impl<T> Agent<T> for Individual<T> {
    fn get_goal(&self) -> f64 {
        self.fitness
    }

    fn get_parameter(&self) -> &T {
        &self.chromosomes
    }
}

impl<T> Individual<T> {
    /// Return reference to chromosomes.
    pub fn get_chromosomes(&self) -> &T {
        &self.chromosomes
    }

    /// Return value of the goal function.
    pub fn get_fitness(&self) -> f64 {
        self.fitness
    }

    /// Returns true if the individual go into the next generation and false otherwise.
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    /// Kill individual. The individual do not go into next generation.
    pub fn kill(&mut self) {
        self.alive = false;
    }
}

/// Stores all individuals for current generation.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub struct Population<'a, T> {
    // Trait object for goal function.
    goal: Box<dyn Goal<T> + 'a>,

    individuals: Vec<Individual<T>>,

    // The best individual for current generation.
    best_individual: Option<Individual<T>>,

    // The worst individual for current generation.
    worst_individual: Option<Individual<T>>,

    // Generation number.
    iteration: usize,
}

impl<'a, T: Clone> Population<'a, T> {
    /// Find new the best and the worst individuals
    fn update_best_worst_individuals(&mut self) {
        // Update the best individual
        let best = self
            .individuals
            .iter()
            .min_by(|ind_1, ind_2| self.individuals_min_cmp(ind_1, ind_2));

        if let Some(ref individual) = best {
            self.best_individual = Some((*individual).clone());
        }

        // Update the worst individual
        let worst = self
            .individuals
            .iter()
            .max_by(|ind_1, ind_2| self.individuals_max_cmp(ind_1, ind_2));

        if let Some(ref individual) = worst {
            self.worst_individual = Some((*individual).clone());
        }
    }
}

impl<'a, T: Clone> AgentsState<T> for Population<'a, T> {
    type Agent = Individual<T>;

    /// Returns vector with references to all agents
    fn get_agents(&self) -> Vec<&Self::Agent> {
        let mut agents: Vec<&Self::Agent> = Vec::with_capacity(self.len());
        for individual in self.individuals.iter() {
            agents.push(individual);
        }

        agents
    }
}

impl<'a, T> Population<'a, T> {
    /// Create new `Population` struct
    /// # Parameters
    /// * `goal` - trait object for goal function
    fn new(goal: Box<dyn Goal<T> + 'a>) -> Self {
        Population {
            goal,
            individuals: vec![],
            best_individual: None,
            worst_individual: None,
            iteration: 0,
        }
    }

    /// Remove all individuals and go to generation 0.
    fn reset(&mut self) {
        self.individuals.clear();
        self.best_individual = None;
        self.worst_individual = None;
        self.iteration = 0;
    }

    /// Create new `Individual` struct with `chromosomes` and add it to population.
    fn push(&mut self, chromosomes: T) {
        let fitness = self.goal.get(&chromosomes);
        let new_individual = Individual {
            chromosomes,
            fitness,
            alive: true,
        };

        self.individuals.push(new_individual);
    }

    /// Create new individuals (`Individual` struct) for all items in `chromosomes_list` and add
    /// them to population.
    fn append(&mut self, chromosomes_list: Vec<T>) {
        for chromosome in chromosomes_list {
            self.push(chromosome);
        }
    }

    /// Returns iterator for all individuals (`Individual` struct) in population.
    pub fn iter(&self) -> slice::Iter<Individual<T>> {
        self.individuals.iter()
    }

    /// Returns mut iterator for all individuals (`Individual` struct) in population.
    pub fn iter_mut(&mut self) -> slice::IterMut<Individual<T>> {
        self.individuals.iter_mut()
    }

    /// Returns iteration number (generation number).
    pub fn get_iteration(&self) -> usize {
        self.iteration
    }

    /// Return count of live individuals
    pub fn len_alive(&self) -> usize {
        self.individuals
            .iter()
            .filter(|individual| individual.is_alive())
            .count()
    }

    /// Returns the best individual in the population if exists or None otherwise.
    pub fn get_best(&self) -> &Option<Individual<T>> {
        &self.best_individual
    }

    /// Returns the worst individual in the population if exists or None otherwise.
    pub fn get_worst(&self) -> &Option<Individual<T>> {
        &self.worst_individual
    }

    /// Returns count of the individuals in the population.
    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    /// Function to find individual with minimal fitness.
    ///
    /// NaN fitness greater others.
    fn individuals_min_cmp(
        &self,
        individual_1: &Individual<T>,
        individual_2: &Individual<T>,
    ) -> Ordering {
        let goal_1 = individual_1.get_goal();
        let goal_2 = individual_2.get_goal();

        if goal_1.is_nan() && goal_2.is_nan() {
            Ordering::Greater
        } else if goal_1.is_nan() {
            Ordering::Greater
        } else if goal_2.is_nan() {
            Ordering::Less
        } else {
            goal_1.partial_cmp(&goal_2).unwrap()
        }
    }

    /// Function to find individual with maximal fitness.
    ///
    /// NaN fitness less others.
    fn individuals_max_cmp(
        &self,
        individual_1: &Individual<T>,
        individual_2: &Individual<T>,
    ) -> Ordering {
        let goal_1 = individual_1.get_goal();
        let goal_2 = individual_2.get_goal();

        if goal_1.is_nan() && goal_2.is_nan() {
            Ordering::Less
        } else if goal_1.is_nan() {
            Ordering::Less
        } else if goal_2.is_nan() {
            Ordering::Greater
        } else {
            goal_1.partial_cmp(&goal_2).unwrap()
        }
    }

    /// Switch to next iteration (generation)
    fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    fn remove_dead(&mut self) {
        self.individuals.retain(|individual| individual.is_alive());
    }
}

/// Index trait implementation for Population
impl<'a, T> ops::Index<usize> for Population<'a, T> {
    type Output = Individual<T>;

    fn index(&self, index: usize) -> &Individual<T> {
        &self.individuals[index]
    }
}

/// IndexMut trait implementation for Population
impl<'a, T> ops::IndexMut<usize> for Population<'a, T> {
    fn index_mut<'b>(&'b mut self, index: usize) -> &'b mut Individual<T> {
        &mut self.individuals[index]
    }
}

impl<'a, T: Clone> AlgorithmState<T> for Population<'a, T> {
    fn get_best_solution(&self) -> Option<(T, f64)> {
        match &self.best_individual {
            None => None,
            Some(individual) => Some((individual.chromosomes.clone(), individual.fitness)),
        }
    }

    fn get_iteration(&self) -> usize {
        self.iteration
    }
}

/// The trait to create initial individuals for population.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub trait Creator<T> {
    /// Must return vector of the chromosomes of a new individuals for population
    fn create(&mut self) -> Vec<T>;
}

/// The trait with cross algorithm.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub trait Cross<T> {
    /// The method accepts slice of references to parent chromosomes (`parents`),
    /// must return vector of chromosomes of children. The children will be added to population
    /// after mutation.
    fn cross(&mut self, parents: &[&T]) -> Vec<T>;
}

/// The trait with mutation algorithm.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub trait Mutation<T> {
    /// The method accepts reference to chromosomes of single individual and must return new
    /// chromosomes (possibly modified). New individuals will be created with the chromosomes after
    /// mutation.
    fn mutation(&mut self, chromosomes: &T) -> T;
}

/// The trait may be used after mutation but before birth of the individuals.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub trait PreBirth<T> {
    /// The method may modify chromosomes list before birth of the individuals.
    fn pre_birth(&mut self, population: &Population<T>, new_chromosomes: &mut Vec<T>);
}

/// The trait with selection algorithm.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub trait Selection<T> {
    /// The method kills bad individuals. The method must call `Individual::kill()` method for
    /// individuals which will not go to next generation.
    fn kill(&mut self, population: &mut Population<T>);
}

/// The trait to select individuals to pairing.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub trait Pairing<T> {
    /// The method must select individuals to cross. Returns vector of vector with individuals
    /// numbers in `population`. Selected individuals will parents for new children.
    fn get_pairs(&mut self, population: &Population<T>) -> Vec<Vec<usize>>;
}

/// The main struct for an user. `GeneticOptimizer` implements `Optimizer` trait and keep all parts
/// of genetic algorithm as trait objects: `Creator`, `Pairing`, `Cross`, `Mutation`, `Selection`,
/// `StopChecker` and, if needed, `Logger`.
/// The trait run genetic algorithm.
///
/// `T` - type of a point in the search space for goal function (chromosomes).
pub struct GeneticOptimizer<'a, T> {
    stop_checker: Box<dyn StopChecker<T> + 'a>,
    creator: Box<dyn Creator<T> + 'a>,
    pairing: Box<dyn Pairing<T> + 'a>,
    cross: Box<dyn Cross<T> + 'a>,
    mutation: Box<dyn Mutation<T> + 'a>,
    selections: Vec<Box<dyn Selection<T> + 'a>>,
    pre_births: Vec<Box<dyn PreBirth<T> + 'a>>,
    loggers: Vec<Box<dyn Logger<T> + 'a>>,
    population: Population<'a, T>,
}

impl<'a, T: Clone> GeneticOptimizer<'a, T> {
    /// Create a new `GeneticOptimizer`.
    pub fn new(
        goal: Box<dyn Goal<T> + 'a>,
        stop_checker: Box<dyn StopChecker<T> + 'a>,
        creator: Box<dyn Creator<T> + 'a>,
        pairing: Box<dyn Pairing<T> + 'a>,
        cross: Box<dyn Cross<T> + 'a>,
        mutation: Box<dyn Mutation<T> + 'a>,
        selections: Vec<Box<dyn Selection<T> + 'a>>,
        pre_births: Vec<Box<dyn PreBirth<T> + 'a>>,
    ) -> GeneticOptimizer<'a, T> {
        GeneticOptimizer {
            creator,
            stop_checker,
            pairing,
            cross,
            mutation,
            selections,
            pre_births,
            loggers: vec![],
            population: Population::new(goal),
        }
    }

    pub fn set_loggers(&mut self, loggers: Vec<Box<dyn Logger<T> + 'a>>) {
        self.loggers = loggers;
    }

    /// Replace the trait object of pairing algorithm.
    pub fn set_pairing(&mut self, pairing: Box<dyn Pairing<T>>) {
        self.pairing = pairing;
    }

    /// Replace the trait object of cross algorithm.
    pub fn set_cross(&mut self, cross: Box<dyn Cross<T>>) {
        self.cross = cross;
    }

    /// Replace the trait object of mutation algorithm.
    pub fn set_mutation(&mut self, mutation: Box<dyn Mutation<T>>) {
        self.mutation = mutation;
    }

    /// Replace the trait object of selection algorithm.
    pub fn set_selection(&mut self, selections: Vec<Box<dyn Selection<T>>>) {
        self.selections = selections;
    }

    /// Replace the trait object of selection algorithm.
    pub fn set_pre_birth(&mut self, pre_births: Vec<Box<dyn PreBirth<T>>>) {
        self.pre_births = pre_births;
    }

    /// Replace the trait object of stop checker algorithm.
    pub fn set_stop_checker(&mut self, stop_checker: Box<dyn StopChecker<T>>) {
        self.stop_checker = stop_checker;
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

impl<'a, T: Clone> IterativeOptimizer<T> for GeneticOptimizer<'a, T> {
    /// Do new iterations of genetic algorithm.
    fn next_iterations(&mut self) -> Option<Solution<T>> {
        for logger in &mut self.loggers {
            logger.resume(&self.population);
        }

        while !self.stop_checker.can_stop(&self.population) {
            // Pairing
            let mut children_chromo_list = self.run_pairing();

            // Mutation
            let mut children_mutants: Vec<T> = children_chromo_list
                .iter_mut()
                .map(|chromo| self.mutation.mutation(chromo))
                .collect();

            // May be change new chromosomes vector before birth
            for pre_birth in &mut self.pre_births {
                pre_birth.pre_birth(&self.population, &mut children_mutants);
            }

            // Create new individuals by new chromosomes and add new individuals to population
            self.population.append(children_mutants);

            // Selection
            for selection in &mut self.selections {
                selection.kill(&mut self.population);
            }

            self.population.remove_dead();

            self.population.update_best_worst_individuals();

            self.population.next_iteration();

            for logger in &mut self.loggers {
                logger.next_iteration(&self.population);
            }
        }

        for logger in &mut self.loggers {
            logger.finish(&self.population);
        }

        match &self.population.best_individual {
            None => None,
            Some(individual) => Some((individual.chromosomes.clone(), individual.fitness)),
        }
    }
}

impl<'a, T: Clone> Optimizer<T> for GeneticOptimizer<'a, T> {
    /// Run genetic algorithm
    fn find_min(&mut self) -> Option<(T, f64)> {
        self.population.reset();
        let start_chromo_list = self.creator.create();

        // Create individuals from chromosomes
        self.population.append(start_chromo_list);

        for logger in &mut self.loggers {
            logger.start(&self.population);
        }

        self.next_iterations()
    }
}
