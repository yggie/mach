use algorithms::IterativeAlgorithm;

pub trait Execute: IterativeAlgorithm {
    fn execute(self) -> Self::Result;
}

impl<T: IterativeAlgorithm> Execute for T {
    fn execute(mut self) -> T::Result {
        while !self.has_converged() {
            self.next_iteration();
        }

        return self.result();
    }
}
