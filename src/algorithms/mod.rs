mod panic_on_iteration;
mod iterative_algorithm_executor;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod iterative_algorithm;

pub use self::panic_on_iteration::PanicOnIteration;
pub use self::iterative_algorithm::IterativeAlgorithm;
pub use self::iterative_algorithm_executor::IterativeAlgorithmExecutor;
