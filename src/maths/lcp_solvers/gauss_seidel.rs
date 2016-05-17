use {INFINITY, Scalar, TOLERANCE};
use maths::{LCP, LCPSolver};
use algorithms::{Execute, IterativeAlgorithm, LimitIterationsTo};

pub struct GaussSeidel;

impl LCPSolver for GaussSeidel {
    fn solve_in_place(&self, problem: &mut LCP) {
        GaussSeidelAlgorithm::new(problem)
            .limit_iterations_to(50)
            .execute();
    }
}

struct GaussSeidelAlgorithm<'a> {
    problem: &'a mut LCP,
    convergence_threshold: Scalar,
    total_change: Scalar,
}

impl<'a> GaussSeidelAlgorithm<'a> {
    fn new(problem: &'a mut LCP) -> GaussSeidelAlgorithm<'a> {
        let size = problem.size();
        GaussSeidelAlgorithm {
            problem: problem,
            total_change: INFINITY,
            convergence_threshold: 10.0 * TOLERANCE * size as Scalar,
        }
    }
}

impl<'a> IterativeAlgorithm for GaussSeidelAlgorithm<'a> {
    type Result = &'a mut LCP;

    fn result(self) -> Self::Result {
        self.problem
    }

    fn has_converged(&self) -> bool {
        self.total_change < self.convergence_threshold
    }

    fn next_iteration(&mut self) {
        let size = self.problem.size();
        self.total_change = 0.0;

        for i in 0..size {
            if i != 0 && self.problem.solution(i).abs() < TOLERANCE {
                *self.problem.unknown_mut(i) = 0.0;
                continue;
            }

            let mut delta = 0.0 as Scalar;
            if i > 0 {
                for j in 0..(i - 1) {
                    delta = delta + self.problem.matrix(i, j) * self.problem.solution(j);
                }
            }

            for j in (i + 1)..size {
                delta = delta + self.problem.matrix(i, j) * self.problem.solution(j);
            }

            let value_before_constraint = (self.problem.bias(i) - delta)
                / self.problem.matrix(i, i);

            let value = self.problem.apply_constraints(i, value_before_constraint);
            // println!("UNKNOWN[{}] = {} -> {}", i, value_before_constraint, value);

            debug_assert!(value.is_finite(), format!("Non-finite solution {} when calculating the solution for index {}. LCP = {:?}", value, i, self.problem));
            self.total_change += (self.problem.solution(i) - value).abs();
            *self.problem.unknown_mut(i) = value;
        }
    }
}
