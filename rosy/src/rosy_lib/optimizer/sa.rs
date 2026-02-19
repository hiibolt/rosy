/// Simulated Annealing optimizer (Algorithm 3 in COSY).
///
/// This is a stochastic global optimization method that attempts to find the
/// global minimum by randomly exploring the search space while gradually
/// reducing the "temperature" parameter. At high temperatures, the algorithm
/// accepts worse solutions with higher probability, allowing it to escape
/// local minima. As the temperature cools, it becomes more selective and
/// converges toward the best solution found.
///
/// This implementation uses a deterministic pseudo-random number generator
/// (linear congruential generator) for reproducible results, matching COSY's
/// behavior of producing identical results across runs.

/// Simple deterministic pseudo-random number generator (LCG).
/// Uses the same constants as the classic Numerical Recipes LCG for
/// reproducibility.
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generate next pseudo-random u64
    fn next_u64(&mut self) -> u64 {
        // LCG parameters (Knuth MMIX)
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    /// Generate a random f64 in [0, 1)
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Generate a random f64 in [-1, 1)
    fn next_f64_symmetric(&mut self) -> f64 {
        2.0 * self.next_f64() - 1.0
    }
}

/// Evaluate the merit function (sum of squares of objectives)
fn merit(objectives: &[f64]) -> f64 {
    objectives.iter().map(|o| o * o).sum()
}

/// Run Simulated Annealing optimization.
///
/// Minimizes the sum of squares of the objective values returned by `body`.
///
/// # Parameters
/// - `variables`: Initial variable values (modified in place)
/// - `eps`: Convergence tolerance
/// - `max_iter`: Maximum number of iterations
/// - `_num_objectives`: Number of objectives (unused, kept for API consistency)
/// - `body`: Closure that evaluates objectives given variable values
pub fn simulated_annealing<F>(
    variables: &mut [f64],
    eps: f64,
    max_iter: usize,
    _num_objectives: usize,
    body: &mut F,
) -> anyhow::Result<()>
where
    F: FnMut(&mut [f64]) -> anyhow::Result<Vec<f64>>,
{
    let nv = variables.len();
    let mut rng = Rng::new(12345); // Deterministic seed for reproducibility

    // Initial evaluation
    let mut current = variables.to_vec();
    let objs = body(current.as_mut_slice())?;
    let mut current_cost = merit(&objs);

    // Track best solution found
    let mut best = current.clone();
    let mut best_cost = current_cost;

    // Determine initial step sizes based on variable values
    let mut step_sizes: Vec<f64> = current.iter().map(|v| {
        if v.abs() > 1e-10 {
            v.abs() * 0.5
        } else {
            1.0
        }
    }).collect();

    // SA parameters
    // Initial temperature: set relative to the initial cost
    let t_initial = if current_cost > 1e-10 {
        current_cost * 2.0
    } else {
        1.0
    };

    // Cooling schedule: exponential decay
    // We want T to decrease smoothly over max_iter iterations
    let cooling_rate = (1e-10_f64 / t_initial).powf(1.0 / max_iter as f64);

    let mut temperature = t_initial;

    // Acceptance tracking for adaptive step sizes
    let mut accept_count = 0u64;
    let mut total_count = 0u64;
    let adapt_interval = (20 * nv).max(50);

    for iteration in 0..max_iter {
        // Check convergence
        if best_cost < eps * eps {
            break;
        }

        // Generate a random neighbor by perturbing one variable at a time
        let mut candidate = current.clone();
        for j in 0..nv {
            candidate[j] += step_sizes[j] * rng.next_f64_symmetric();
        }

        // Evaluate candidate
        let candidate_objs = body(candidate.as_mut_slice())?;
        let candidate_cost = merit(&candidate_objs);

        // Metropolis acceptance criterion
        let accept = if candidate_cost < current_cost {
            true
        } else {
            let delta = candidate_cost - current_cost;
            let acceptance_prob = (-delta / temperature).exp();
            rng.next_f64() < acceptance_prob
        };

        if accept {
            current = candidate;
            current_cost = candidate_cost;
            accept_count += 1;

            // Update best if this is the best we've seen
            if current_cost < best_cost {
                best = current.clone();
                best_cost = current_cost;
            }
        }

        total_count += 1;

        // Adaptive step size adjustment
        if total_count % adapt_interval as u64 == 0 {
            let accept_ratio = accept_count as f64 / total_count as f64;

            // Target acceptance ratio around 0.3-0.5
            if accept_ratio > 0.5 {
                // Too many acceptances — increase step sizes
                for s in step_sizes.iter_mut() {
                    *s *= 1.2;
                }
            } else if accept_ratio < 0.2 {
                // Too few acceptances — decrease step sizes
                for s in step_sizes.iter_mut() {
                    *s *= 0.8;
                }
            }

            accept_count = 0;
            total_count = 0;
        }

        // Cool down
        temperature *= cooling_rate;

        // Periodically restart from best (reheating)
        if iteration > 0 && iteration % (max_iter / 4).max(1) == 0 {
            current = best.clone();
            current_cost = best_cost;
            // Slight reheat to allow further exploration
            temperature = temperature.max(t_initial * 0.01);
        }
    }

    // Restore best variables found
    variables.copy_from_slice(&best);

    // Run body one final time with best values so objectives are set
    body(variables)?;

    Ok(())
}
