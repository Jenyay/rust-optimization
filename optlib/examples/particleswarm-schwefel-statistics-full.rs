//! Example of optimizing the Schwefel function with particle sqwarm algorithm.
use std::fs::File;
use std::io;

use optlib::particleswarm::{
    self, initializing, postmove, postvelocitycalc, velocitycalc, ParticleSwarmOptimizer, PostMove,
    PostVelocityCalc,
};
use optlib::tools::statistics::{
    get_predicate_success_vec_solution, CallCountData, GoalCalcStatistics,
    StatFunctionsConvergence, StatFunctionsGoal, StatFunctionsSolution,
};
use optlib::tools::{logging, statistics, stopchecker};
use optlib::{Goal, GoalFromFunction, Optimizer};
use optlib_testfunc;

/// Coordinates type
type Coordinate = f32;

fn create_optimizer<'a>(
    dimension: usize,
    goal: Box<dyn Goal<Vec<Coordinate>> + 'a>,
) -> ParticleSwarmOptimizer<'a, Coordinate> {
    // General parameters
    let minval: Coordinate = -500.0;
    let maxval: Coordinate = 500.0;
    let particles_count = 50;
    let intervals = vec![(minval, maxval); dimension];

    let phi_best_personal = 3.2;
    let phi_best_current = 0.0;
    let phi_best_global = 1.0;

    let phi_worst_personal = 0.0;
    let phi_worst_current = 0.0;
    let phi_worst_global = 0.0;

    let phi_t = phi_best_personal + phi_best_current + phi_best_global + phi_worst_current;

    let alpha = 0.9;
    let xi = 2.0 * alpha / (phi_t - 2.0);

    // Particles initializers
    let coord_initializer =
        initializing::RandomCoordinatesInitializer::new(intervals.clone(), particles_count);
    let velocity_initializer = initializing::ZeroVelocityInitializer::new(dimension, particles_count);

    let max_velocity = 700.0;
    let post_velocity_calc: Vec<Box<dyn PostVelocityCalc<Coordinate>>> =
        vec![Box::new(postvelocitycalc::MaxVelocityAbs::new(max_velocity))];

    // PostMove
    let post_moves: Vec<Box<dyn PostMove<Coordinate>>> =
        vec![Box::new(postmove::MoveToBoundary::new(intervals.clone()))];

    // Velocity calculator
    let velocity_calculator = velocitycalc::NegativeReinforcement::new(
        phi_best_personal,
        phi_best_current,
        phi_best_global,
        phi_worst_personal,
        phi_worst_current,
        phi_worst_global,
        xi
    );

    // Stop checker
    let change_max_iterations = 50;
    let change_delta = 1e-8;
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-6)),
        Box::new(stopchecker::GoalNotChange::new(
            change_max_iterations,
            change_delta,
        )),
        Box::new(stopchecker::MaxIterations::new(1000)),
    ]);

    let mut optimizer = particleswarm::ParticleSwarmOptimizer::new(
        goal,
        Box::new(stop_checker),
        Box::new(coord_initializer),
        Box::new(velocity_initializer),
        Box::new(velocity_calculator),
    );
    optimizer.set_post_moves(post_moves);
    optimizer.set_post_velocity_calc(post_velocity_calc);
    optimizer
}

fn print_convergence_statistics(
    mut writer: &mut dyn io::Write,
    stat: &statistics::Statistics<Vec<Coordinate>>,
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

fn print_solution(
    mut writer: &mut dyn io::Write,
    stat: &statistics::Statistics<Vec<Coordinate>>,
) {
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
    stat: &statistics::Statistics<Vec<Coordinate>>,
    call_count: &CallCountData,
    dimension: usize,
) {
    let valid_answer = vec![420.9687; dimension];
    let delta = vec![1.0; dimension];

    let success_rate_answer = stat
        .get_results()
        .get_success_rate(get_predicate_success_vec_solution(valid_answer, delta))
        .unwrap();
    let average_goal = stat.get_results().get_average_goal().unwrap();
    let standard_deviation_goal = stat.get_results().get_standard_deviation_goal().unwrap();

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
    let dimension = 3;
    let run_count = 1000;

    let mut full_stat = statistics::Statistics::new();
    let mut full_call_count = CallCountData::new();

    for n in 0..run_count {
        let mut statistics_data = statistics::Statistics::new();
        let mut call_count = CallCountData::new();
        {
            // Make a trait object for goal function (Schwefel function)
            let mut goal_object = GoalFromFunction::new(optlib_testfunc::schwefel);
            let goal = GoalCalcStatistics::new(&mut goal_object, &mut call_count);

            let mut optimizer = create_optimizer(dimension, Box::new(goal));

            let stat_logger = Box::new(statistics::StatisticsLogger::new(&mut statistics_data));
            let loggers: Vec<Box<dyn logging::Logger<Vec<Coordinate>>>> = vec![stat_logger];
            optimizer.set_loggers(loggers);

            println!("{:} / {:}", n + 1, run_count);
            optimizer.find_min().unwrap();
        }
        full_stat.unite(statistics_data);
        full_call_count.unite(call_count);
    }

    // Print out statistics
    let result_stat_fname = "result_stat.txt";
    let mut result_stat_file = File::create(result_stat_fname).unwrap();

    let convergence_stat_fname = "convergence_stat.txt";
    let mut convergence_stat_file = File::create(convergence_stat_fname).unwrap();
    print_solution(&mut result_stat_file, &full_stat);
    print_convergence_statistics(&mut convergence_stat_file, &full_stat);
    print_statistics(&full_stat, &full_call_count, dimension);
}
