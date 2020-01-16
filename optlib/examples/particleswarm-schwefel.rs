use std::io;

use optlib::{
    tools::{logging, stopchecker},
    GoalFromFunction, Optimizer,
    particleswarm::{
        self,
        initializing,
        postmove,
        velocitycalc,
        PostMove,
        postvelocitycalc,
        PostVelocityCalc,
    },
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
    let coord_initializer = initializing::RandomCoordinatesInitializer::new(intervals.clone(), particles_count);
    let velocity_initializer = initializing::ZeroVelocityInitializer::new(dimension, particles_count);

    // PostMove
    let post_moves: Vec<Box<dyn PostMove<Coordinate>>> = vec![Box::new(postmove::MoveToBoundary::new(intervals.clone()))];

    // Velocity calculator
    let velocity_calculator = velocitycalc::CanonicalVelocityCalculator::new(phi_personal, phi_global, k);

    let max_velocity = 700.0;
    let post_velocity_calc: Vec<Box<dyn PostVelocityCalc<Coordinate>>> =
        vec![Box::new(postvelocitycalc::MaxVelocityAbs::new(max_velocity))];

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
        Box::new(velocity_initializer),
        Box::new(velocity_calculator),
        );
    optimizer.set_loggers(loggers);
    optimizer.set_post_moves(post_moves);
    optimizer.set_post_velocity_calc(post_velocity_calc);

    optimizer.find_min();
}
