use algorithms::IterativeAlgorithm;

pub struct PanicOnIteration<I: IterativeAlgorithm> {
    limit: u32,
    message: &'static str,
    algorithm: I,
    iterations: u32,
}

impl<I: IterativeAlgorithm> PanicOnIteration<I> {
    pub fn new(algorithm: I, limit: u32, message: &'static str) -> PanicOnIteration<I> {
        PanicOnIteration {
            limit: limit,
            message: message,
            algorithm: algorithm,
            iterations: 0,
        }
    }
}

impl<I: IterativeAlgorithm> IterativeAlgorithm for PanicOnIteration<I> {
    type Result = I::Result;

    #[inline]
    fn result(self) -> Self::Result {
        self.algorithm.result()
    }

    #[inline]
    fn has_converged(&self) -> bool {
        self.algorithm.has_converged()
    }

    fn next_iteration(&mut self) {
        self.algorithm.next_iteration();
        self.iterations = self.iterations + 1;

        if self.iterations > self.limit {
            panic!("Took over {} iterations to complete the process: \"{}\"", self.limit, self.message);
        }
    }
}
