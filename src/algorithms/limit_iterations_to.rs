use algorithms::IterativeAlgorithm;

pub trait LimitIterationsTo: IterativeAlgorithm {
    fn limit_iterations_to(self, usize) -> IterationLimiter<Self>;
}

impl<T: IterativeAlgorithm> LimitIterationsTo for T {
    fn limit_iterations_to(self, limit: usize) -> IterationLimiter<T> {
        IterationLimiter {
            algorithm: self,
            iteration_limit: limit,
            current_iteration: 0,
        }
    }
}

pub struct IterationLimiter<T: IterativeAlgorithm> {
    algorithm: T,
    current_iteration: usize,
    iteration_limit: usize,
}

impl<T: IterativeAlgorithm> IterativeAlgorithm for IterationLimiter<T> {
    type Result = T::Result;

    #[inline(always)]
    fn result(self) -> Self::Result {
        self.algorithm.result()
    }

    fn has_converged(&self) -> bool {
        self.algorithm.has_converged() ||
            self.current_iteration >= self.iteration_limit
    }

    fn next_iteration(&mut self) {
        self.algorithm.next_iteration();
        self.current_iteration += 1;
    }
}
