use algorithms::IterativeAlgorithm;

pub struct IterativeAlgorithmExecutor;

impl IterativeAlgorithmExecutor {
    pub fn execute<I: IterativeAlgorithm>(mut algorithm: I) -> I::Result {
        while !algorithm.has_converged() {
            algorithm.next_iteration();
        }

        return algorithm.result();
    }
}
