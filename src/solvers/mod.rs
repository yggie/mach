mod impulse_solver;
mod mach_constraint_solver;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod constraint_solver;

pub use self::constraint_solver::ConstraintSolver;
pub use self::impulse_solver::ImpulseSolver;
pub use self::mach_constraint_solver::MachConstraintSolver;
