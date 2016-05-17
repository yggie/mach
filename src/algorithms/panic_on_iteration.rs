use algorithms::IterativeAlgorithm;

pub trait PanicOnIteration: IterativeAlgorithm {
    fn panic_on_iteration(self, usize, &str) -> IterationLimiterWithPanic<Self>;
}

impl<T: IterativeAlgorithm> PanicOnIteration for T {
    fn panic_on_iteration(self, limit: usize, message: &str) -> IterationLimiterWithPanic<Self> {
        IterationLimiterWithPanic {
            limit: limit,
            message: String::from(message),
            algorithm: self,
            current_iteration: 0,
        }
    }
}

pub struct IterationLimiterWithPanic<T: IterativeAlgorithm> {
    limit: usize,
    message: String,
    algorithm: T,
    current_iteration: usize,
}

impl<T: IterativeAlgorithm> IterativeAlgorithm for IterationLimiterWithPanic<T> {
    type Result = T::Result;

    #[inline(always)]
    fn result(self) -> Self::Result {
        self.algorithm.result()
    }

    #[inline(always)]
    fn has_converged(&self) -> bool {
        self.algorithm.has_converged()
    }

    fn next_iteration(&mut self) {
        self.algorithm.next_iteration();
        self.current_iteration += 1;

        if self.current_iteration >= self.limit {
            panic!("Took over {} iterations to complete the process: \"{}\"", self.limit, self.message);
        }
    }
}
