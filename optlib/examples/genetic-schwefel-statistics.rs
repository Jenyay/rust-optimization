//! Example of optimizing the Schwefel function with genetic algorithm.
//!
//! y = f(x), where x = (x0, x1, ..., xi,... xn).
//! Global minimum is x' = (420.9687, 420.9687, ...) for any xi lying in [-500.0; 500.0].
//! f(x') = 0
//!
//! # Terms
//! * `Goal function` - the function for optimization. y = f(x).
//! * `Gene` - a single value of xi.
//! * `Chromosome` - a point in the search space. x = (x0, x1, x2, ..., xn).
//! * `Individual` - union of x and value of goal function.
//! * `Population` - set of the individuals.
//! * `Generation` - a number of iteration of genetic algorithm.
use core::cell::{Ref, RefCell};
use std::io;
use std::fs::File;

use optlib::genetic::{
    self, creation, cross, mutation, pairing, pre_birth, selection, GeneticOptimizer,
};
use optlib::tools::{logging, statistics, stopchecker};
use optlib::{GoalFromFunction, Optimizer};
use optlib_testfunc;

/// Gene type
type Gene = f32;

/// Chromosomes type
type Chromosomes = Vec<Gene>;

fn create_optimizer<'a>(chromo_count: usize) -> GeneticOptimizer<'a, Chromosomes> {
    // General parameters

    // Search space. Any xi lies in [-500.0; 500.0]
    let minval: Gene = -500.0;
    let maxval: Gene = 500.0;

    // Count individuals in initial population
    let population_size = 1500;

    let intervals = vec![(minval, maxval); chromo_count];

    // Make a trait object for goal function (Schwefel function)
    let goal = GoalFromFunction::new(optlib_testfunc::schwefel);

    // Make the creator to create initial population.
    // RandomCreator will fill initial population with individuals with random chromosomes in a
    // given interval,
    let creator = creation::vec_float::RandomCreator::new(population_size, intervals.clone());

    // Make a trait object for the pairing.
    // Pairing is algorithm of selection individuals for crossbreeding.

    // Tournament method.
    let families_count = population_size / 2;
    let rounds_count = 5;
    let pairing = pairing::Tournament::new(families_count).rounds_count(rounds_count);

    // Crossbreeding algorithm.
    // Make a Cross trait object. The bitwise crossing for float genes.
    let single_cross = cross::FloatCrossExp::new();
    let cross = cross::VecCrossAllGenes::new(Box::new(single_cross));

    // Make a Mutation trait object.
    // Use bitwise mutation (change random bits with given probability).
    let mutation_probability = 10.0;
    let mutation_gene_count = 3;
    let single_mutation = mutation::BitwiseMutation::new(mutation_gene_count);
    let mutation = mutation::VecMutation::new(mutation_probability, Box::new(single_mutation));

    // Pre birth. Throw away new chlld chromosomes if their genes do not lies if given intervals.
    let pre_births: Vec<Box<dyn genetic::PreBirth<Chromosomes>>> = vec![Box::new(
        pre_birth::vec_float::CheckChromoInterval::new(intervals.clone()),
    )];

    // Stop checker. Stop criterion for genetic algorithm.
    // Stop algorithm after 3000 generation (iteration).
    // let stop_checker =
    //     stopchecker::CompositeAny::new(vec![Box::new(stopchecker::MaxIterations::new(3000))]);

    // let change_max_iterations = 150;
    // let change_delta = 1e-7;
    let stop_checker = stopchecker::CompositeAny::new(vec![
        // Box::new(stopchecker::Threshold::new(1e-6)),
        // Box::new(stopchecker::GoalNotChange::new(
        //     change_max_iterations,
        //     change_delta,
        // )),
        Box::new(stopchecker::MaxIterations::new(1000)),
    ]);

    // Make a trait object for selection. Selection is killing the worst individuals.
    // Kill all individuals if the value of goal function is NaN or Inf.
    // Kill the worst individuals to population size remained unchanged.
    let selections: Vec<Box<dyn genetic::Selection<Chromosomes>>> = vec![
        Box::new(selection::KillFitnessNaN::new()),
        Box::new(selection::LimitPopulation::new(population_size)),
    ];

    // Construct main optimizer struct
    let optimizer = genetic::GeneticOptimizer::new(
        Box::new(goal),
        Box::new(stop_checker),
        Box::new(creator),
        Box::new(pairing),
        Box::new(cross),
        Box::new(mutation),
        selections,
        pre_births,
    );

    optimizer
}

fn print_convergence_statistics(mut writer: &mut dyn io::Write, stat: &Ref<statistics::Statistics<Chromosomes>>) {
    let average_convergence = statistics::get_average_convergence(stat.get_convergence());
    for n in 0..average_convergence.len() {
        if let Some(goal_value) = average_convergence[n] {
            writeln!(&mut writer, "{n:<8}{value:15.10e}", n = n, value = goal_value).unwrap();
        }
    }
}

fn print_result_statistics(mut writer: &mut dyn io::Write, stat: &Ref<statistics::Statistics<Chromosomes>>) {
    let run_count = stat.get_run_count();

    // Print solutions for every running
    let results = stat.get_results();
    for n in 0..run_count {
        if let Some((solution, goal)) = &results[n] {
            let mut result_str = String::new();
            result_str = result_str + &format!("{:<8}", n);

            for x in solution {
                result_str = result_str + &format!("  {:<20.10}", x);
            }
            result_str = result_str + &format!("  {:20.10}", goal);

            writeln!(&mut writer, "{}", result_str).unwrap();
        } else {
            writeln!(&mut writer, "{n:<8}  Failed", n = n).unwrap();
        }
    }
}

fn main() {
    // Count of xi in the chromosomes
    let chromo_count = 15;

    let run_count = 300;

    // Logging
    let statistics_data = RefCell::new(statistics::Statistics::new());
    {
        let mut optimizer = create_optimizer(chromo_count);

        let stat_logger = Box::new(statistics::StatisticsLogger::new(
            statistics_data.borrow_mut(),
        ));
        let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![stat_logger];
        optimizer.set_loggers(loggers);

        for n in 0..run_count {
            println!("{:} / {:}", n + 1, run_count);
            optimizer.find_min().unwrap();
        }
    }

    // Print out statistics
    let new_stat = statistics_data.borrow();

    let result_stat_fname = "result_stat.txt";
    let mut result_stat_file = File::create(result_stat_fname).unwrap();

    let convergence_stat_fname = "convergence_stat.txt";
    let mut convergence_stat_file = File::create(convergence_stat_fname).unwrap();

    print_result_statistics(&mut result_stat_file, &new_stat);
    print_convergence_statistics(&mut convergence_stat_file, &new_stat);
}
