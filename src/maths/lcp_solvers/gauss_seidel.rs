use {Scalar, TOLERANCE};
use maths::{LCP, LCPSolver};

pub struct GaussSeidel;

impl LCPSolver for GaussSeidel {
    fn solve_in_place(&self, problem: &mut LCP) {
        let size = problem.size();
        let convergence_tolerance = 10.0 * TOLERANCE * size as Scalar;
        for iter in 0..50 {
            let mut total_change = 0.0;

            for i in 0..size {
                if i != 0 && problem.solution(i).abs() < TOLERANCE {
                    *problem.unknown_mut(i) = 0.0;
                    continue;
                }

                let mut delta = 0.0 as Scalar;
                if i > 0 {
                    for j in 0..(i - 1) {
                        delta = delta + problem.matrix(i, j) * problem.solution(j);
                    }
                }

                for j in (i + 1)..size {
                    delta = delta + problem.matrix(i, j) * problem.solution(j);
                }

                let value_before_constraint = (problem.bias(i) - delta)
                    / problem.matrix(i, i);

                let value = problem.apply_constraints(i, value_before_constraint);
                // println!("UNKNOWN[{}] = {} -> {}", i, value_before_constraint, value);

                debug_assert!(value.is_finite(), format!("Non-finite solution {} when calculating the solution for index {} at iteration {}. LCP = {:?}", value, i, iter, problem));
                total_change = total_change + (problem.solution(i) - value).abs();
                *problem.unknown_mut(i) = value;
            }

            if total_change < convergence_tolerance {
                // println!("CONVERGENCE, EXIT SOLVER");
                break;
            }
        }
    }
}
