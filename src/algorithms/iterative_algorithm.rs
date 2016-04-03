pub trait IterativeAlgorithm: Sized {
    type Result;

    fn result(self) -> Self::Result;

    fn has_converged(&self) -> bool;

    fn next_iteration(&mut self);
}
