use std::io;

use optlib::{
    particleswarm::{
        self, initializing, postmove, postspeedcalc, speedcalc, PostMove, PostSpeedCalc,
    },
    tools::{logging, stopchecker},
    GoalFromFunction, IterativeOptimizer, Optimizer,
};

use optlib_testfunc;

type Coordinate = f32;

fn main() {
    // General parameters
    let minval: Coordinate = -500.0;
    let maxval: Coordinate = 500.0;
    let particles_count = 100;
    let dimension = 3;
    let intervals = vec![(minval, maxval); dimension];
    let phi_personal = 3.2;
    let phi_global = 1.0;
    let k = 0.9;

    // Goal function
    let goal = GoalFromFunction::new(optlib_testfunc::schwefel);

    // Particles initializers
    let coord_initializer =
        initializing::RandomCoordinatesInitializer::new(intervals.clone(), particles_count);
    let speed_initializer = initializing::ZeroSpeedInitializer::new(dimension, particles_count);

    // PostMove
    let post_moves: Vec<Box<dyn PostMove<Coordinate>>> =
        vec![Box::new(postmove::MoveToBoundary::new(intervals.clone()))];

    // Speed calculator
    let speed_calculator = speedcalc::CanonicalSpeedCalculator::new(phi_personal, phi_global, k);

    let max_speed = 700.0;
    let post_speed_calc: Vec<Box<dyn PostSpeedCalc<Coordinate>>> =
        vec![Box::new(postspeedcalc::MaxSpeedAbs::new(max_speed))];

    // Stop checker
    let stop_checker_1 = Box::new(stopchecker::MaxIterations::new(50));
    let stop_checker_2 = Box::new(stopchecker::MaxIterations::new(150));

    // Logger
    let mut stdout_verbose = io::stdout();
    let mut stdout_result = io::stdout();
    let mut stdout_time = io::stdout();

    let loggers: Vec<Box<dyn logging::Logger<Vec<Coordinate>>>> = vec![
        Box::new(logging::VerboseLogger::new(&mut stdout_verbose, 15)),
        Box::new(logging::ResultOnlyLogger::new(&mut stdout_result, 15)),
        Box::new(logging::TimeLogger::new(&mut stdout_time)),
    ];

    let mut optimizer = particleswarm::ParticleSwarmOptimizer::new(
        Box::new(goal),
        stop_checker_1,
        Box::new(coord_initializer),
        Box::new(speed_initializer),
        Box::new(speed_calculator),
    );
    optimizer.set_loggers(loggers);
    optimizer.set_post_moves(post_moves);
    optimizer.set_post_speed_calc(post_speed_calc);

    optimizer.find_min();

    optimizer.set_stop_checker(stop_checker_2);
    optimizer.next_iterations();
}
