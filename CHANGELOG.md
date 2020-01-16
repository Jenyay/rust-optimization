# Optlib changelog

## 0.3.0

### API
1. Some parameters of optimizer constructor moved from constructor parameters to separate setters.
1. Add logger for statistics gathering.
1. The next_iterations method move to base trait IterativeOptimizer.

### Particle swarm optimization
1. Add the structures for calculate particle velocity considering inertia.
1. Add the possibility for correcting velocity after calculating (used for limit the velocity)
1. Add the items for limit velocity by modulus and directions.

### Statistics
1. Add the items for gathering algorithms statistics.
1. Add structures for calculate average convergence (average value of goal function on the iteration number and after the algorithm is complete).
1. Add the items for calculate standard deviation value of goal function after the algorithm is complete
1. Add the items for calculate average solution if solution is of the type Vec&lt;Float&gt;.
1. Add the items for calculate standard deviation of solution if solution is of the type Vec&lt;Float&gt;.
1. Add the items for calculate a success rate by value of goal function.
1. Add the items for calculate a success rate by found solution if solution is of the type Vec&lt;Float&gt;.
1. Add the items for calculate goal function call count.

### Examples
1. Add new example with statistics gathering for genetic algorithm.
1. Add new examples with statistics gathering for particle swarm optimization.


---

## 0.2.0

### API

1. The loggers can save content with Write trait.
1. The Goal trait moved to optlib module.
1. The test functions moved to optlib-testfunc crate.
1. Add new integration tests with optimization.

### Particle swarm optimization
1. Add the structures for particle swarm optimization.

### Examples
1. Add new example for optimization of Schwefel function with particle swarm optimization.
1. Add new example for optimization of Rastrigin function with particle swarm optimization.

---

## 0.1.0

1. The first version
1. Add genetic algorithm implementation.
1. Add test Schwefel function.

