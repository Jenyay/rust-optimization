use std::io;

use optlib::{
    tools::{logging, stopchecker},
    GoalFromFunction, Optimizer,
    particleswarm::{
        self,
        initializing,
        postmove,
        speedcalc,
        PostMove,
    },
};

use optlib_testfunc;

type Coordinate = f32;

fn main() {
    // General parameters
    let minval: Coordinate = -5.12;
    let maxval: Coordinate = 5.12;
    let particles_count = 500;
    let dimension = 3;
    let intervals = vec![(minval, maxval); dimension];
    let phi_personal = 2.0;
    let phi_global = 5.0;
    let k = 0.2;

    // Goal function
    let goal = GoalFromFunction::new(optlib_testfunc::rastrigin);

    // Particles initializers
    let coord_initializer = initializing::RandomCoordinatesInitializer::new(intervals.clone(), particles_count);
    let speed_initializer = initializing::ZeroSpeedInitializer::new(dimension, particles_count);

    // PostMove
    let post_moves: Vec<Box<dyn PostMove<Coordinate>>> = vec![Box::new(postmove::MoveToBoundary::new(intervals.clone()))];

    // Speed calculator
    let speed_calculator = speedcalc::CanonicalSpeedCalculator::new(phi_personal, phi_global, k);

    // Stop checker
    let change_max_iterations = 150;
    let change_delta = 1e-8;
    let stop_checker = stopchecker::CompositeAny::new(vec![
        Box::new(stopchecker::Threshold::new(1e-6)),
        Box::new(stopchecker::GoalNotChange::new(
            change_max_iterations,
            change_delta,
        )),
        Box::new(stopchecker::MaxIterations::new(3000)),
    ]);

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
        Box::new(stop_checker),
        Box::new(coord_initializer),
        Box::new(speed_initializer),
        Box::new(speed_calculator),
        post_moves,
        );
    optimizer.set_loggers(loggers);

    optimizer.find_min();
}
