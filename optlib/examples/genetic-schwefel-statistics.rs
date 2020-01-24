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
use std::fs::File;
use std::io;
use std::sync::mpsc;
use std::thread;

use optlib::genetic::{
    self, creation, cross, mutation, pairing, pre_birth, selection, GeneticOptimizer,
};
use optlib::tools::statistics::{
    get_predicate_success_vec_solution, CallCountData, GoalCalcStatistics,
    StatFunctionsConvergence, StatFunctionsGoal, StatFunctionsSolution,
};
use optlib::tools::{logging, statistics, stopchecker};
use optlib::{Goal, GoalFromFunction, Optimizer};
use optlib_testfunc;

/// Gene type
type Gene = f32;

/// Chromosomes type
type Chromosomes = Vec<Gene>;

fn create_optimizer<'a>(
    chromo_count: usize,
    goal: Box<dyn Goal<Chromosomes> + 'a>,
) -> GeneticOptimizer<'a, Chromosomes> {
    // General parameters

    // Search space. Any xi lies in [-500.0; 500.0]
    let minval: Gene = -500.0;
    let maxval: Gene = 500.0;

    // Count individuals in initial population
    let population_size = 100;

    let intervals = vec![(minval, maxval); chromo_count];

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
        Box::new(stopchecker::MaxIterations::new(3000)),
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
        goal,
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

fn print_convergence_statistics(
    mut writer: &mut dyn io::Write,
    stat: &statistics::Statistics<Chromosomes>,
) {
    let average_convergence = stat.get_convergence().get_average_convergence();
    for n in 0..average_convergence.len() {
        if let Some(goal_value) = average_convergence[n] {
            writeln!(
                &mut writer,
                "{n:<8}{value:15.10e}",
                n = n,
                value = goal_value
            )
            .unwrap();
        }
    }
}

fn print_solution(mut writer: &mut dyn io::Write, stat: &statistics::Statistics<Chromosomes>) {
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

fn print_statistics(
    stat: &statistics::Statistics<Chromosomes>,
    call_count: &CallCountData,
    chromo_count: usize,
) {
    let valid_answer = vec![420.9687; chromo_count];
    let delta = vec![1.0; chromo_count];

    let success_rate_answer = stat
        .get_results()
        .get_success_rate(get_predicate_success_vec_solution(valid_answer, delta))
        .unwrap();
    let average_goal = stat.get_results().get_average_goal().unwrap();
    let standard_deviation_goal = stat.get_results().get_standard_deviation_goal().unwrap();

    println!("Run count{:15}", stat.get_run_count());
    println!("Success rate:{:15.5}", success_rate_answer);
    println!("Average goal:{:15.5}", average_goal);
    println!(
        "Standard deviation for goal:{:15.5}",
        standard_deviation_goal
    );
    println!(
        "Average goal function call count:{:15.5}",
        call_count.get_average_call_count().unwrap()
    );
}

fn main() {
    let cpu = num_cpus::get();
    let dimension = 3;

    // Running count per CPU
    let run_count = 1000 / cpu;

    println!("CPUs:{:15}", cpu);
    println!("Run count per CPU:{:8}", run_count);
    print!("Run optimizations... ");

    // Statistics from all runnings
    let mut full_stat = statistics::Statistics::new();
    let mut full_call_count = CallCountData::new();

    let (tx, rx) = mpsc::channel();

    for _ in 0..cpu {
        let current_tx = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let mut local_full_stat = statistics::Statistics::new();
            let mut local_full_call_count = CallCountData::new();

            for _ in 0..run_count {
                // Statistics from single run
                let mut statistics_data = statistics::Statistics::new();
                let mut call_count = CallCountData::new();
                {
                    // Make a trait object for goal function
                    let mut goal_object = GoalFromFunction::new(optlib_testfunc::schwefel);
                    let goal = GoalCalcStatistics::new(&mut goal_object, &mut call_count);

                    let mut optimizer = create_optimizer(dimension, Box::new(goal));

                    // Add logger to collect statistics
                    let stat_logger =
                        Box::new(statistics::StatisticsLogger::new(&mut statistics_data));
                    let loggers: Vec<Box<dyn logging::Logger<Chromosomes>>> = vec![stat_logger];
                    optimizer.set_loggers(loggers);

                    // Run optimization
                    optimizer.find_min();
                }

                // Add current running statistics to full statistics
                local_full_stat.unite(statistics_data);
                local_full_call_count.unite(call_count);
            }
            current_tx
                .send((local_full_stat, local_full_call_count))
                .unwrap();
        });
    }

    // Collect data from threads
    for _ in 0..cpu {
        let (statistics_data, call_count) = rx.recv().unwrap();
        full_stat.unite(statistics_data);
        full_call_count.unite(call_count);
    }

    println!("OK");

    // Print out statistics
    let result_stat_fname = "result_stat.txt";
    let mut result_stat_file = File::create(result_stat_fname).unwrap();

    let convergence_stat_fname = "convergence_stat.txt";
    let mut convergence_stat_file = File::create(convergence_stat_fname).unwrap();
    print_solution(&mut result_stat_file, &full_stat);
    print_convergence_statistics(&mut convergence_stat_file, &full_stat);
    print_statistics(&full_stat, &full_call_count, dimension);
}
